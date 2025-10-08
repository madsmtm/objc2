use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;
use std::iter;

use clang::{Entity, EntityKind, EntityVisitResult};

use crate::availability::Availability;
use crate::cfgs::PlatformCfg;
use crate::config::StmtData;
use crate::config::{Config, LibraryConfig, MethodData};
use crate::context::Context;
use crate::display_helper::FormatterFn;
use crate::documentation::Documentation;
use crate::expr::Expr;
use crate::fn_utils::follows_create_rule;
use crate::id::cfg_gate_ln;
use crate::id::ItemIdentifier;
use crate::id::ItemTree;
use crate::id::Location;
use crate::immediate_children;
use crate::method::{handle_reserved, Method};
use crate::name_translation::is_likely_bounds_affecting;
use crate::name_translation::{enum_prefix, split_words};
use crate::protocol::parse_direct_protocols;
use crate::protocol::ProtocolRef;
use crate::rust_type::PointeeTy;
use crate::rust_type::SafetyProperty;
use crate::rust_type::Ty;
use crate::thread_safety::ThreadSafety;
use crate::unexposed_attr::UnexposedAttr;

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Derives(Cow<'static, str>);

impl Default for Derives {
    fn default() -> Self {
        Derives("Debug, PartialEq, Eq, Hash".into())
    }
}

impl fmt::Display for Derives {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.0.is_empty() {
            write!(f, "#[derive({})]", self.0)?;
        }
        Ok(())
    }
}

/// Find all protocols, protocol's protocols and superclass' protocols.
fn parse_protocols<'tu>(
    entity: &Entity<'tu>,
    protocols: &mut BTreeMap<ItemIdentifier, Entity<'tu>>,
    context: &Context<'_>,
) {
    immediate_children(entity, |entity, _span| match entity.get_kind() {
        EntityKind::ObjCProtocolRef => {
            let entity = entity
                .get_reference()
                .expect("ObjCProtocolRef to reference entity");
            if protocols
                .insert(ItemIdentifier::new(&entity, context), entity)
                .is_none()
            {
                // Only recurse if we haven't already seen this protocol
                parse_protocols(&entity, protocols, context);
            }
        }
        EntityKind::ObjCSuperClassRef => {
            let entity = entity
                .get_reference()
                .expect("ObjCSuperClassRef to reference entity");
            parse_protocols(&entity, protocols, context);
        }
        _ => {}
    });
}

pub(crate) fn parse_superclasses<'ty>(
    entity: &Entity<'ty>,
    context: &Context<'_>,
) -> Vec<(ItemIdentifier, Vec<String>, Entity<'ty>)> {
    let mut current_entity = *entity;
    let mut superclasses = vec![];

    loop {
        let mut superclass = None;
        let mut applied_generics = Vec::new();

        immediate_children(&current_entity, |entity, _span| match entity.get_kind() {
            EntityKind::ObjCSuperClassRef => {
                superclass = Some(
                    entity
                        .get_reference()
                        .expect("ObjCSuperClassRef to reference entity"),
                );
            }
            EntityKind::TypeRef => {
                let name = entity.get_name().expect("typeref name");
                applied_generics.push(name);
            }
            _ => {}
        });

        if let Some(superclass) = superclass {
            current_entity = superclass;
            superclasses.push((
                ItemIdentifier::new(&superclass, context),
                applied_generics,
                superclass,
            ));
        } else {
            return superclasses;
        }
    }
}

pub(crate) type GenericWithBound = (String, Option<PointeeTy>);

pub(crate) fn parse_class_generics(
    entity: &Entity<'_>,
    context: &Context<'_>,
) -> Vec<GenericWithBound> {
    let mut generics = Vec::new();

    #[allow(clippy::single_match)]
    immediate_children(entity, |entity, _span| match entity.get_kind() {
        EntityKind::TemplateTypeParameter => {
            let name = entity.get_name().expect("template name");
            let mut bound = None;

            immediate_children(&entity, |entity, _span| match entity.get_kind() {
                EntityKind::ObjCClassRef | EntityKind::TypeRef => {
                    let ty = entity.get_type().expect("template type");
                    let ty = PointeeTy::parse_generic_bound(ty, context);
                    bound = Some(ty);
                }
                EntityKind::ObjCProtocolRef => {
                    let definition = entity.get_definition().expect("template definition");
                    if let Some(ty) = &mut bound {
                        ty.add_protocol(definition, context);
                    } else {
                        error!(?entity, "should have handled protocol type parameter");
                    }
                }
                _ => error!(?entity, "unknown type parameter child"),
            });

            if let Some(ty) = &bound {
                if ty.is_plain_anyobject() {
                    // A bound of just `id` or `AnyObject` is unnecessary.
                    bound = None;
                }
            }

            generics.push((name, bound));
        }
        _ => {}
    });

    generics
}

/// Deduplicate methods that are autogenerated from properties.
///
/// Guaranteed to only contain `ObjCInstanceMethodDecl`, `ObjCClassMethodDecl`
/// and `ObjCPropertyDecl`.
pub(crate) fn method_or_property_entities<'tu>(
    entity: &Entity<'tu>,
    get_data: impl Fn(&str) -> MethodData,
) -> Vec<Entity<'tu>> {
    let mut entities = Vec::new();

    // Track seen properties, so that when methods are autogenerated by the
    // compiler from them, we can skip them
    let mut properties = HashSet::new();

    immediate_children(entity, |entity, _span| match entity.get_kind() {
        EntityKind::ObjCInstanceMethodDecl | EntityKind::ObjCClassMethodDecl => {
            let is_class = entity.get_kind() == EntityKind::ObjCClassMethodDecl;
            let selector = entity.get_name().expect("method selector");

            if get_data(&selector).skipped {
                return;
            }

            if !properties.remove(&(is_class, selector)) {
                entities.push(entity);
            }
        }
        EntityKind::ObjCPropertyDecl => {
            entities.push(entity);

            let partial = Method::partial_property(entity);

            // TODO: Use `get_overridden_methods` to deduplicate property
            // getters (when declared on both immutable and mutable class).

            if !get_data(&partial.getter_sel).skipped
                && !properties.insert((partial.is_class, partial.getter_sel.clone()))
            {
                error!(?partial, ?entity, "already existing getter property");
            }

            if let Some(setter_sel) = &partial.setter_sel {
                if !get_data(setter_sel).skipped
                    && !properties.insert((partial.is_class, setter_sel.clone()))
                {
                    error!(?partial, ?entity, "already existing setter property");
                }
            }
        }
        _ => {}
    });

    if !properties.is_empty() {
        error!(
            ?entities,
            ?properties,
            "did not properly add methods to properties"
        );
    }

    entities
}

fn parse_methods(
    entity: &Entity<'_>,
    get_data: impl Fn(&str) -> MethodData,
    thread_safety: &ThreadSafety,
    is_pub: bool,
    context: &Context<'_>,
) -> (Vec<Method>, Vec<String>) {
    let mut methods = Vec::new();
    let mut designated_initializers = Vec::new();

    for entity in method_or_property_entities(entity, &get_data) {
        match entity.get_kind() {
            EntityKind::ObjCInstanceMethodDecl | EntityKind::ObjCClassMethodDecl => {
                let selector = entity.get_name().expect("method selector");

                let data = get_data(&selector);

                if let Some((designated_initializer, method)) =
                    Method::parse_method(entity, data, thread_safety, is_pub, context)
                {
                    if designated_initializer {
                        designated_initializers.push(method.selector.clone());
                    }
                    methods.push(method);
                }
            }
            EntityKind::ObjCPropertyDecl => {
                let partial = Method::partial_property(entity);

                // TODO: Use `get_overridden_methods` to deduplicate property
                // getters (when declared on both immutable and mutable class).

                let getter_data = get_data(&partial.getter_sel);
                let setter_data = partial
                    .setter_sel
                    .as_ref()
                    .map(|setter_sel| get_data(setter_sel));

                let (getter, setter) = Method::parse_property(
                    partial,
                    getter_data,
                    setter_data,
                    thread_safety,
                    is_pub,
                    context,
                );
                if let Some(getter) = getter {
                    methods.push(getter);
                }
                if let Some(setter) = setter {
                    methods.push(setter);
                }
            }
            kind => unreachable!("method/property entity {kind:?}"),
        }
    }

    (methods, designated_initializers)
}

/// Takes one of:
/// - `EntityKind::ObjCInterfaceDecl`
/// - `EntityKind::ObjCProtocolDecl`
/// - `EntityKind::ObjCCategoryDecl`
fn verify_objc_decl(entity: &Entity<'_>, _context: &Context<'_>) {
    let parent_kind = entity.get_kind();

    immediate_children(entity, |entity, _span| {
        match (entity.get_kind(), parent_kind) {
            (EntityKind::ObjCExplicitProtocolImpl, EntityKind::ObjCProtocolDecl) => {
                // TODO NS_PROTOCOL_REQUIRES_EXPLICIT_IMPLEMENTATION
            }
            (
                EntityKind::ObjCIvarDecl | EntityKind::StructDecl | EntityKind::UnionDecl,
                EntityKind::ObjCInterfaceDecl,
            ) => {
                // Explicitly ignored
            }
            (
                EntityKind::ObjCSuperClassRef | EntityKind::TypeRef,
                EntityKind::ObjCInterfaceDecl,
            ) => {
                // Parsed in parse_superclasses
            }
            (EntityKind::ObjCSubclassingRestricted, EntityKind::ObjCInterfaceDecl) => {
                // TODO: https://clang.llvm.org/docs/AttributeReference.html#objc-subclassing-restricted
            }
            (EntityKind::ObjCRootClass, EntityKind::ObjCInterfaceDecl) => {
                debug!("parsing root class");
            }
            (
                EntityKind::ObjCClassRef,
                EntityKind::ObjCInterfaceDecl | EntityKind::ObjCCategoryDecl,
            ) => {
                // debug!("ObjCClassRef: {:?}", entity.get_display_name());
            }
            (
                EntityKind::TemplateTypeParameter,
                EntityKind::ObjCInterfaceDecl | EntityKind::ObjCCategoryDecl,
            ) => {
                // Parsed in parse_class_generics
            }
            (EntityKind::ObjCProtocolRef, _) => {
                // Parsed in parse_protocols and parse_direct_protocols
            }
            (
                EntityKind::ObjCInstanceMethodDecl
                | EntityKind::ObjCClassMethodDecl
                | EntityKind::ObjCPropertyDecl,
                _,
            ) => {
                // Parsed in method_or_property_entities
            }
            (EntityKind::VisibilityAttr, _) => {
                // Already exposed as entity.get_visibility()
            }
            (EntityKind::ObjCException, EntityKind::ObjCInterfaceDecl) => {
                // Maybe useful for knowing when to implement `Error` for the type
            }
            (EntityKind::UnexposedAttr, _) => {
                // Parsed in parse_attributes
            }
            (EntityKind::AnnotateAttr, _) if entity.get_name().unwrap() == "main-thread-only" => {
                // Already parsed via. UnexposedAttr.
            }
            (_, parent_kind) => error!(?entity, ?parent_kind, "unknown in parent"),
        }
    });
}

/// Whether the entity contains a bridging modifier, and if so, what that
/// modifier bridges to.
pub(crate) fn bridged_to(entity: &Entity<'_>, context: &Context<'_>) -> Option<Option<String>> {
    let mut bridged = None;
    immediate_children(entity, |child, _span| {
        if let EntityKind::UnexposedAttr = child.get_kind() {
            if let Some(attr) = UnexposedAttr::parse(&child, context) {
                match attr {
                    UnexposedAttr::Bridged(to) | UnexposedAttr::BridgedMutable(to) => {
                        if to == "id" {
                            bridged = Some(None);
                        } else {
                            bridged = Some(Some(to));
                        }
                    }
                    UnexposedAttr::BridgedRelated
                    | UnexposedAttr::BridgedTypedef
                    | UnexposedAttr::BridgedImplicit => {
                        bridged = Some(None);
                    }
                    _ => {}
                }
            }
        }
    });
    bridged
}

pub(crate) fn anonymous_record_name(entity: &Entity<'_>, context: &Context<'_>) -> Option<String> {
    let parent = entity.get_semantic_parent()?;

    if !matches!(
        parent.get_kind(),
        EntityKind::StructDecl | EntityKind::UnionDecl
    ) {
        return None;
    }

    let parent_id = ItemIdentifier::new_optional(&parent, context)
        .map_name(|name| name.or_else(|| anonymous_record_name(&parent, context)))
        .to_option()?;

    // Find the field name for this.
    //
    // UnionDecl/StructDecl comes first, then the matching FieldDecl.
    let mut just_found_record = false;
    let mut field_name = None;
    immediate_children(&parent, |searched, _span| match searched.get_kind() {
        EntityKind::FieldDecl => {
            if just_found_record {
                field_name = Some(searched.get_name().expect("field name"));
                just_found_record = false;
            }
        }
        EntityKind::UnionDecl | EntityKind::StructDecl => {
            if searched == *entity {
                just_found_record = true;
            }
        }
        _ => {}
    });

    let field_name = field_name?;

    Some(format!("{}_{}", parent_id.name, field_name))
}

pub(crate) fn superclasses_required_items<'a, I>(
    superclasses: I,
) -> impl Iterator<Item = ItemTree> + 'a
where
    I: IntoIterator<Item = ItemIdentifier> + 'a,
    <I as IntoIterator>::IntoIter: Clone,
{
    let iter = superclasses.into_iter();
    iter.clone()
        .enumerate()
        .map(move |(i, superclass)| {
            ItemTree::new(
                superclass,
                superclasses_required_items(iter.clone().skip(i + 1).collect::<Vec<_>>()),
            )
        })
        .chain(iter::once(ItemTree::objc("__macros__")))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum Counterpart {
    #[default]
    NoCounterpart,
    ImmutableSuperclass(ItemIdentifier),
    MutableSubclass(ItemIdentifier),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Abi {
    C,
    CUnwind,
    OuterRustInnerC,
    OuterRustInnerCUnwind,
}

impl Abi {
    fn extern_inner(&self) -> &'static str {
        match self {
            Self::OuterRustInnerC | Self::C => "extern \"C\" ",
            Self::OuterRustInnerCUnwind | Self::CUnwind => "extern \"C-unwind\" ",
        }
    }

    fn extern_outer(&self) -> &'static str {
        if self.rust_outer() {
            ""
        } else {
            self.extern_inner()
        }
    }

    fn rust_outer(&self) -> bool {
        matches!(self, Self::OuterRustInnerC | Self::OuterRustInnerCUnwind)
    }

    pub(crate) fn as_rust_outer(&self) -> Self {
        match self {
            Self::C => Self::OuterRustInnerC,
            Self::CUnwind => Self::OuterRustInnerCUnwind,
            Self::OuterRustInnerC | Self::OuterRustInnerCUnwind => panic!("already Rust outer"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    /// @interface name: superclass <protocols*>
    /// ->
    /// extern_class!
    ClassDecl {
        id: ItemIdentifier,
        generics: Vec<GenericWithBound>,
        objc_name: String,
        availability: Availability,
        /// Superclass + their applied generics.
        superclasses: Vec<(ItemIdentifier, Vec<String>)>,
        designated_initializers: Vec<String>,
        derives: Derives,
        main_thread_only: bool,
        skipped: bool,
        sendable: bool,
        documentation: Documentation,
        bridged_to: Option<ItemIdentifier>,
    },
    /// @interface class_name (category_name) <protocols*>
    /// ->
    /// extern_methods!
    ExternMethods {
        location: Location,
        availability: Availability,
        cls: ItemIdentifier,
        cls_superclasses: Vec<ItemIdentifier>,
        cls_generics: Vec<GenericWithBound>,
        methods: Vec<Method>,
        documentation: Documentation,
    },
    /// @interface class_name (category_name)
    /// ->
    /// impl trait category_name {
    ///     extern_methods!(...)
    /// }
    ///
    /// impl trait category_name for class_name {}
    ExternCategory {
        id: ItemIdentifier,
        availability: Availability,
        cls: ItemIdentifier,
        cls_superclasses: Vec<ItemIdentifier>,
        cls_generics: Vec<GenericWithBound>,
        methods: Vec<Method>,
        documentation: Documentation,
    },
    /// @protocol name <protocols*>
    /// ->
    /// extern_protocol!
    ProtocolDecl {
        id: ItemIdentifier,
        objc_name: String,
        availability: Availability,
        super_protocols: Vec<ProtocolRef>,
        methods: Vec<Method>,
        required_sendable: bool,
        required_mainthreadonly: bool,
        documentation: Documentation,
    },
    /// @interface ty: _ <protocols*>
    /// @interface ty (_) <protocols*>
    ProtocolImpl {
        location: Location,
        cls: ItemIdentifier,
        cls_superclasses: Vec<ItemIdentifier>,
        cls_counterpart: Counterpart,
        protocol: ItemIdentifier,
        protocol_super_protocols: Vec<ProtocolRef>,
        generics: Vec<GenericWithBound>,
        availability: Availability,
    },
    /// struct name {
    ///     fields*
    /// };
    ///
    /// typedef struct {
    ///     fields*
    /// } name;
    ///
    /// typedef struct _name {
    ///     fields*
    /// } name;
    ///
    /// union name {
    ///     fields*
    /// };
    RecordDecl {
        id: ItemIdentifier,
        /// internal objc struct name (before typedef). shows up in encoding
        /// and is used in message verification.
        encoding_name: String,
        availability: Availability,
        boxable: bool,
        fields: Vec<(String, Documentation, Ty)>,
        sendable: Option<bool>,
        align: usize,
        natural_align: usize,
        documentation: Documentation,
        is_union: bool,
    },
    /// typedef NS_OPTIONS(type, name) {
    ///     variants*
    /// };
    ///
    /// typedef NS_ENUM(type, name) {
    ///     variants*
    /// };
    ///
    /// enum name {
    ///     variants*
    /// };
    ///
    /// enum {
    ///     variants*
    /// };
    EnumDecl {
        id: ItemIdentifier,
        availability: Availability,
        ty: Ty,
        kind: Option<UnexposedAttr>,
        variants: Vec<(String, Documentation, Availability, Expr)>,
        sendable: Option<bool>,
        documentation: Documentation,
    },
    /// Anonymous enum variants are emitted as free constants.
    ///
    /// enum {
    ///     variants*
    /// };
    ConstDecl {
        id: ItemIdentifier,
        availability: Availability,
        ty: Ty,
        value: Expr,
        // Hack to get prettier output
        is_last: bool,
        documentation: Documentation,
    },
    /// static const ty name = expr;
    /// extern const ty name;
    VarDecl {
        id: ItemIdentifier,
        link_name: String,
        availability: Availability,
        ty: Ty,
        value: Option<Expr>,
        documentation: Documentation,
    },
    /// extern ret name(args*);
    ///
    /// static inline ret name(args*) {
    ///     body
    /// }
    FnDecl {
        id: ItemIdentifier,
        c_name: String,
        link_name: String,
        availability: Availability,
        arguments: Vec<(String, Ty)>,
        first_arg_is_self: bool,
        result_type: Ty,
        // Some -> inline function.
        body: Option<()>,
        safe: bool,
        must_use: bool,
        abi: Abi,
        returns_retained: bool,
        documentation: Documentation,
        /// Passed onwards from FnData to global analysis.
        no_implementor: bool,
        /// Passed onwards from FnData to global analysis.
        custom_implementor: Option<ItemTree>,
    },
    /// CFTypeID CGColorGetTypeID(void)
    FnGetTypeId {
        id: ItemIdentifier,
        cf_item: ItemTree,
        link_name: String,
        result_type: Ty,
        availability: Availability,
        abi: Abi,
        documentation: Documentation,
    },
    /// typedef Type TypedefName;
    AliasDecl {
        id: ItemIdentifier,
        availability: Availability,
        ty: Ty,
        kind: Option<UnexposedAttr>,
        documentation: Documentation,
    },
    /// typedef struct CF_BRIDGED_TYPE(id) CGColorSpace *CGColorSpaceRef;
    OpaqueDecl {
        id: ItemIdentifier,
        generics: Vec<GenericWithBound>,
        encoding_name: String,
        availability: Availability,
        documentation: Documentation,
        is_cf: bool,
        sendable: Option<bool>,
        bridged: Option<String>,
        superclass: Option<ItemIdentifier>,
    },
    GeneralImpl {
        location: Location,
        item: ItemTree, // TODO: Make this a Ty?
        stmts: Vec<Self>,
    },
}

fn parse_fn_param_children(parent: &Entity<'_>, context: &Context<'_>) -> Option<UnexposedAttr> {
    let mut ret = None;

    immediate_children(parent, |entity, _span| match entity.get_kind() {
        EntityKind::UnexposedAttr => {
            if let Some(attr) = UnexposedAttr::parse(&entity, context) {
                if ret.is_some() {
                    error!("found multiple attributes {ret:?} and {attr:?} on fn param");
                }
                ret = Some(attr);
            }
        }
        EntityKind::ObjCClassRef
        | EntityKind::TypeRef
        | EntityKind::ObjCProtocolRef
        | EntityKind::ParmDecl => {}
        EntityKind::NSConsumed => {
            error!("found NSConsumed, which requires manual handling");
        }
        // For some reason we recurse into array types
        EntityKind::IntegerLiteral => {}
        kind => error!(?parent, ?kind, "unknown"),
    });

    ret
}

impl Stmt {
    pub fn parse(
        entity: &Entity<'_>,
        context: &Context<'_>,
        current_library: &LibraryConfig,
    ) -> Vec<Self> {
        if let EntityKind::ObjCClassRef | EntityKind::ObjCProtocolRef = entity.get_kind() {
            // These are inconsequential for us, since we resolve imports differently
            return vec![];
        }

        // The C name of the entity.
        let c_name = if !entity.is_anonymous() {
            entity.get_name()
        } else {
            None
        };

        let _span = debug_span!("stmt", kind = ?entity.get_kind(), ?c_name).entered();

        // The Rust name + the location of the entity.
        let id = ItemIdentifier::new_optional(entity, context);

        // The configuration data.
        //
        // NOTE: We look this up relative to what we're currently parsing,
        // instead of the more natural `context.library(&id).get(entity))`.
        //
        // This allows us to configure the item (like skipping it) even when
        // the item is marked as `external` and id thus points to another crate.
        let data = current_library.get(entity);

        if data.skipped.unwrap_or(false) {
            return vec![];
        }

        let availability = Availability::parse(entity, context);
        let mut documentation = Documentation::from_entity(entity, context);

        match entity.get_kind() {
            EntityKind::ObjCInterfaceDecl => {
                let id = id.require_name();
                // entity.get_mangled_objc_names()
                let objc_name = entity.get_name().unwrap();

                let thread_safety = ThreadSafety::from_decl(entity, context);

                let counterpart = data.counterpart.clone();

                verify_objc_decl(entity, context);
                let generics = parse_class_generics(entity, context);
                let (methods, designated_initializers) = parse_methods(
                    entity,
                    |name| data.method(name),
                    &thread_safety,
                    true,
                    context,
                );

                let mut protocols = Default::default();
                parse_protocols(entity, &mut protocols, context);
                let skipped_protocols = data.skipped_protocols.clone();
                protocols.retain(|protocol, _| !skipped_protocols.contains(&protocol.name));

                let superclasses_full = parse_superclasses(entity, context);

                let superclasses: Vec<_> = superclasses_full
                    .iter()
                    .map(|(id, generics, _)| (id.clone(), generics.clone()))
                    .collect();
                let cls_superclasses: Vec<_> = superclasses_full
                    .iter()
                    .map(|(id, _, _)| id.clone())
                    .collect();

                // Used for duplicate checking (sometimes the subclass
                // defines the same method that the superclass did).
                let mut seen_methods: BTreeMap<_, _> = methods
                    .iter()
                    .map(|method| (method.id(), Some(method)))
                    .collect();

                let superclass_methods: Vec<_> = superclasses_full
                    .iter()
                    .filter_map(|(superclass_id, _, entity)| {
                        let superclass_data = context
                            .library(superclass_id)
                            .class_data
                            .get(&superclass_id.name)
                            .unwrap_or_else(|| StmtData::empty());

                        // Explicitly keep going, even if the class itself is skipped
                        // if superclass_data.skipped

                        let (mut methods, _) = parse_methods(
                            entity,
                            |name| {
                                let data = data.method(name);
                                let superclass_data = superclass_data.method(name);
                                data.merge_with_superclass(superclass_data)
                            },
                            &thread_safety,
                            true,
                            context,
                        );
                        methods.retain(|superclass_method| {
                            // Extra: Validate the signature of overwritten
                            // methods. These are unsound.
                            //
                            // AppKit breaks protocol type-safety with e.g.
                            // `dataSource`, you can set the data source on an
                            // outline view, and then re-interpret the object
                            // as-if it was a table data source (without it
                            // actually implementing that protocol). Example:
                            //
                            // ```
                            // let view: &NSOutlineView = ...;
                            // let source: &ProtocolObject<dyn NSOutlineViewDataSource> = ...;
                            // view.setDataSource(source);
                            // let view: &NSTableView = &**view;
                            // let source: ProtocolObject<dyn NSTableViewDataSource> = view.dataSource();
                            // ```
                            if let Some(Some(method)) = seen_methods.get(&superclass_method.id()) {
                                if !superclass_method.valid_to_override_as(method) {
                                    if method.safe || superclass_method.safe {
                                        // Ideally, we'd just automatically
                                        // unmark these as unsafe, but that's
                                        // kinda hard with the current setup,
                                        // so give an error instead.
                                        error!(
                                            class = ?id.name,
                                            superclass = ?superclass_id.name,
                                            selector = method.selector,
                                            "incorrectly overwritten method was marked safe"
                                        );
                                    } else {
                                        warn!(
                                            class = ?id.name,
                                            superclass = ?superclass_id.name,
                                            selector = method.selector,
                                            "incorrectly overwritten method"
                                        );
                                    }
                                }
                            }

                            superclass_method.emit_on_subclasses()
                                && !seen_methods.contains_key(&superclass_method.id())
                        });
                        seen_methods.extend(methods.iter().map(|method| (method.id(), None)));
                        if methods.is_empty() {
                            None
                        } else {
                            let mut documentation = Documentation::empty();
                            documentation.set_first(format!(
                                "Methods declared on superclass `{}`.",
                                superclass_id.name
                            ));
                            Some(Self::ExternMethods {
                                location: id.location().clone(),
                                availability: Availability::default(),
                                cls: id.clone(),
                                cls_superclasses: cls_superclasses.clone(),
                                cls_generics: generics.clone(),
                                methods,
                                documentation,
                            })
                        }
                    })
                    .collect();

                let methods = Self::ExternMethods {
                    location: id.location().clone(),
                    // The class is already marked with this availability, so
                    // no need to mark the impl as well.
                    availability: Availability::default(),
                    cls: id.clone(),
                    cls_superclasses: cls_superclasses.clone(),
                    cls_generics: generics.clone(),
                    methods,
                    documentation: Documentation::empty(),
                };

                iter::once(Self::ClassDecl {
                    id: id.clone(),
                    objc_name,
                    generics: generics.clone(),
                    availability: availability.clone(),
                    superclasses,
                    designated_initializers,
                    derives: data.derives.clone(),
                    main_thread_only: thread_safety.explicit_mainthreadonly(),
                    skipped: data.definition_skipped,
                    // Ignore sendability on superclasses; since it's an auto
                    // trait, it's propagated to subclasses anyhow!
                    sendable: thread_safety.explicit_sendable(),
                    documentation,
                    bridged_to: data.bridged_to.clone(),
                })
                .chain(protocols.into_iter().map(|(p, entity)| Self::ProtocolImpl {
                    location: id.location().clone(),
                    cls: id.clone(),
                    cls_superclasses: cls_superclasses.clone(),
                    cls_counterpart: counterpart.clone(),
                    protocol: p,
                    protocol_super_protocols: ProtocolRef::super_protocols(&entity, context),
                    generics: generics.clone(),
                    availability: availability.clone(),
                }))
                .chain(iter::once(methods))
                .chain(superclass_methods)
                .collect()
            }
            EntityKind::ObjCCategoryDecl => {
                // TODO: Move this parsing to LibraryData.get?
                let category = id;

                let mut cls_entity = None;
                entity.visit_children(|entity, _parent| {
                    if entity.get_kind() == EntityKind::ObjCClassRef {
                        if cls_entity.is_some() {
                            panic!("could not find unique category class")
                        }
                        let definition = entity
                            .get_definition()
                            .expect("category class ref definition");
                        cls_entity = Some(definition);
                        EntityVisitResult::Break
                    } else {
                        EntityVisitResult::Continue
                    }
                });
                let cls_entity = cls_entity.expect("could not find category class");

                let cls = ItemIdentifier::new(&cls_entity, context);
                let cls_data = context
                    .library(&category)
                    .class_data
                    .get(&cls.name)
                    .unwrap_or_else(|| StmtData::empty());

                if cls_data.skipped.unwrap_or(false) {
                    return vec![];
                }

                let category_data = if let Some(category_name) = &category.name {
                    cls_data
                        .categories
                        .get(category_name)
                        .cloned()
                        .unwrap_or_default()
                } else {
                    Default::default()
                };

                if category_data.skipped {
                    return vec![];
                }

                let cls_thread_safety = ThreadSafety::from_decl(&cls_entity, context);
                let cls_superclasses: Vec<_> = parse_superclasses(&cls_entity, context)
                    .into_iter()
                    .map(|(id, _, _)| id)
                    .collect();

                verify_objc_decl(entity, context);
                let generics = parse_class_generics(entity, context);

                let protocols = parse_direct_protocols(entity, context);
                let protocols: BTreeMap<_, _> = protocols
                    .into_iter()
                    .map(|entity| (ItemIdentifier::new(&entity, context), entity))
                    .filter(|(protocol, _)| !cls_data.skipped_protocols.contains(&protocol.name))
                    .collect();

                let protocol_impls = protocols.into_iter().map(|(p, entity)| Self::ProtocolImpl {
                    location: category.location().clone(),
                    cls: cls.clone(),
                    cls_superclasses: cls_superclasses.clone(),
                    cls_counterpart: cls_data.counterpart.clone().clone(),
                    generics: generics.clone(),
                    availability: availability.clone(),
                    protocol: p,
                    protocol_super_protocols: ProtocolRef::super_protocols(&entity, context),
                });

                // For ease-of-use, if the category is defined in the same
                // library as the class, we just emit it as `extern_methods!`.
                if cls.library_name() == category.library_name() {
                    assert!(
                        category_data.renamed.is_none(),
                        "cannot rename categories emitted in same crate"
                    );

                    // extern_methods!

                    let (methods, designated_initializers) = parse_methods(
                        entity,
                        |name| cls_data.method(name),
                        &cls_thread_safety,
                        true,
                        context,
                    );

                    if !designated_initializers.is_empty() {
                        warn!(
                            ?designated_initializers,
                            "designated initializer in category"
                        );
                    }

                    let extra_methods = if let Counterpart::MutableSubclass(subclass) =
                        cls_data.counterpart.clone()
                    {
                        let subclass_data = context
                            .library(&subclass)
                            .class_data
                            .get(&subclass.name)
                            .unwrap_or_else(|| StmtData::empty());
                        assert!(!subclass_data.skipped.unwrap_or(false));

                        let (mut methods, _) = parse_methods(
                            entity,
                            |name| {
                                let data = cls_data.method(name);
                                let subclass_data = subclass_data.method(name);
                                subclass_data.merge_with_superclass(data)
                            },
                            &cls_thread_safety,
                            true,
                            context,
                        );
                        methods.retain(|method| method.emit_on_subclasses());
                        if methods.is_empty() {
                            None
                        } else {
                            let mut documentation = documentation.clone();
                            documentation.set_first(format!(
                                "Methods declared on superclass `{}`.",
                                cls.name
                            ));
                            if let Some(category_name) = &category.name {
                                documentation.add(format!("{category_name}."));
                            }
                            Some(Self::ExternMethods {
                                location: category.location().clone(),
                                // Assume that immutable/mutable pairs have the
                                // same availability ...
                                availability: availability.clone(),
                                cls: subclass,
                                // ... the same required items ...
                                cls_superclasses: cls_superclasses.clone(),
                                // ... and that they have the same amount of generics.
                                cls_generics: generics.clone(),
                                methods,
                                documentation,
                            })
                        }
                    } else {
                        None
                    };

                    if let Some(category_name) = &category.name {
                        documentation.set_first(format!("{category_name}."));
                    }
                    iter::once(Self::ExternMethods {
                        location: category.location().clone(),
                        availability: availability.clone(),
                        cls: cls.clone(),
                        cls_superclasses: cls_superclasses.clone(),
                        cls_generics: generics.clone(),
                        methods,
                        documentation,
                    })
                    .chain(extra_methods)
                    .chain(protocol_impls)
                    .collect()
                } else {
                    // external category

                    // Rough heuristic to determine category name.
                    //
                    // Note: There isn't really a good way to do this, as
                    // category names are not part of the public API in
                    // Objective-C.
                    let id = category.clone().map_name(|name| {
                        if let Some(name) = category_data.renamed {
                            name
                        } else if let Some(name) = name {
                            if name.contains(&cls.name)
                                || name.contains(&cls.name.replace("Mutable", ""))
                            {
                                name.clone()
                            } else {
                                format!("{}{}", cls.name, name)
                            }
                        } else {
                            error!(?cls, "missing category name for external category");
                            format!("{}Category", cls.name)
                        }
                    });

                    // Alias to the C name.
                    if let Some(c_name) = entity.get_name() {
                        if c_name != id.name {
                            documentation
                                .set_first(format!("Category \"{c_name}\" on [`{}`].", cls.name));
                            documentation.set_alias(c_name);
                        } else {
                            documentation.set_first(format!("Category on [`{}`].", cls.name));
                        }
                    } else {
                        documentation.set_first(format!("Category on [`{}`].", cls.name));
                    }

                    let (methods, designated_initializers) = parse_methods(
                        entity,
                        |name| cls_data.method(name),
                        &cls_thread_safety,
                        false,
                        context,
                    );

                    if !designated_initializers.is_empty() {
                        warn!(
                            ?designated_initializers,
                            "designated initializer in category"
                        );
                    }

                    // Categories are often used to implement protocols for a
                    // type, so as an optimization let's not emit empty
                    // external declarations.
                    //
                    // Additionally, if all methods are deprecated, then there
                    // really isn't a need for us to emit the category
                    // (especially on NSObject, as that just adds a bunch of
                    // clutter).
                    if methods
                        .iter()
                        .all(|method| method.availability.is_deprecated())
                    {
                        None
                    } else {
                        Some(Self::ExternCategory {
                            id,
                            availability: availability.clone(),
                            cls: cls.clone(),
                            cls_superclasses: cls_superclasses.clone(),
                            cls_generics: generics.clone(),
                            methods,
                            documentation,
                        })
                    }
                    .into_iter()
                    .chain(protocol_impls)
                    .collect()
                }
            }
            EntityKind::ObjCProtocolDecl => {
                let id = id.require_name();
                let objc_name = c_name.unwrap();

                let thread_safety = ThreadSafety::from_decl(entity, context);

                verify_objc_decl(entity, context);
                let (methods, designated_initializers) = parse_methods(
                    entity,
                    |name| data.method(name),
                    &thread_safety,
                    false,
                    context,
                );

                if !designated_initializers.is_empty() {
                    warn!(
                        ?designated_initializers,
                        "designated initializer in protocol"
                    )
                }

                vec![Self::ProtocolDecl {
                    id,
                    objc_name,
                    availability,
                    super_protocols: ProtocolRef::super_protocols(entity, context),
                    methods,
                    required_sendable: thread_safety.explicit_sendable(),
                    required_mainthreadonly: thread_safety.explicit_mainthreadonly(),
                    documentation,
                }]
            }
            EntityKind::TypedefDecl => {
                let id = id.require_name();
                let c_name = c_name.unwrap();

                let mut kind = None;
                let mut inner_struct = None;
                let mut sendable = None;

                immediate_children(entity, |entity, _span| match entity.get_kind() {
                    EntityKind::UnexposedAttr => {
                        if let Some(attr) = UnexposedAttr::parse(&entity, context) {
                            match attr {
                                UnexposedAttr::Sendable => sendable = Some(true),
                                UnexposedAttr::NonSendable => sendable = Some(false),
                                UnexposedAttr::UIActor => {
                                    // TODO: Track these on blocks somehow?
                                    // Maybe add an extra `mtm` argument?
                                    warn!(name = ?id.name, "main-thread-only typedef");
                                }
                                _ => {
                                    if kind.is_some() {
                                        warn!(?kind, ?attr, "got multiple unexposed attributes");
                                    }
                                    kind = Some(attr);
                                }
                            }
                        }
                    }
                    EntityKind::TypeRef => {
                        inner_struct =
                            Some(entity.get_reference().expect("typeref to have reference"));
                    }
                    EntityKind::StructDecl
                    | EntityKind::UnionDecl
                    | EntityKind::ObjCClassRef
                    | EntityKind::ObjCProtocolRef
                    | EntityKind::ParmDecl
                    | EntityKind::EnumDecl
                    | EntityKind::IntegerLiteral
                    | EntityKind::BinaryOperator
                    | EntityKind::DeclRefExpr
                    | EntityKind::ParenExpr => {}
                    EntityKind::ObjCIndependentClass => {
                        // TODO: Might be interesting?
                    }
                    _ => error!(?entity, "unknown typedef child"),
                });

                let ty = entity
                    .get_typedef_underlying_type()
                    .expect("typedef underlying type");
                let mut ty = Ty::parse_typedef(ty, context);

                if let Some(nullability) = data.nullability {
                    ty.change_nullability(nullability.into());
                }

                if ty.needs_simd() {
                    debug!("simd types are not yet possible in typedefs");
                    return vec![];
                }

                // No need to output a typedef if it'll just point to the same thing.
                //
                // TODO: We're discarding a slight bit of availability data this way.
                if ty.is_enum(&c_name) || ty.is_record(&c_name) {
                    return vec![];
                }

                let mut bridged = if let Some(inner_struct) = &inner_struct {
                    bridged_to(inner_struct, context)
                } else {
                    bridged_to(entity, context)
                };

                if let Some((is_cf, encoding_name)) =
                    ty.pointer_to_opaque_struct_or_void(&c_name, bridged.is_some())
                {
                    if kind.is_some() {
                        error!(?kind, "unknown kind on opaque type");
                    }

                    assert_eq!(
                        inner_struct
                            .as_ref()
                            .map(|entity| entity.get_name().unwrap())
                            .as_deref(),
                        encoding_name,
                        "inner struct must be the same that `pointer_to_opaque_struct_or_void` found",
                    );

                    if is_cf {
                        let id = context.replace_typedef_name(id, is_cf);

                        if id.name != c_name {
                            documentation.set_alias(c_name);
                        }

                        // If the class name contains the word "Mutable"
                        // exactly once per the usual word-boundary rules, a
                        // corresponding class name without the word "Mutable"
                        // will be used as the superclass if present.
                        // Otherwise, the CF type is taken to be a root object.
                        let is_mutable = split_words(&id.name)
                            .filter(|word| *word == "Mutable")
                            .count()
                            == 1;
                        let superclass = if is_mutable {
                            // Assume that class pairs are declared in the same file.
                            Some(id.clone().map_name(|name| name.replace("Mutable", "")))
                        } else {
                            None
                        };

                        // Mutable typedefs have the same underlying struct
                        // name as their non-mutable counterparts, so clang
                        // unifies them, and we don't get the correct bridging
                        // name. So try to fix that here.
                        if is_mutable {
                            if let Some(Some(bridged_name)) = &bridged {
                                let mut class_iter = split_words(&id.name);
                                let mut res_name = String::new();
                                for bridged_word in split_words(bridged_name) {
                                    if class_iter.next() == Some("Mutable") {
                                        res_name.push_str("Mutable");
                                    }
                                    res_name.push_str(bridged_word);
                                }
                                bridged = Some(Some(res_name));
                            }
                        }

                        if let Some(Some(bridged)) = &bridged {
                            documentation
                                .add(format!("This is toll-free bridged with `{bridged}`."));
                        }

                        return vec![Self::OpaqueDecl {
                            id,
                            encoding_name: encoding_name.unwrap().to_string(),
                            generics: data
                                .generics
                                .iter()
                                .map(|generic| (generic.clone(), None))
                                .collect(),
                            availability,
                            documentation,
                            is_cf,
                            sendable,
                            bridged: bridged.flatten(),
                            superclass,
                        }];
                    } else if let Some(entity) = inner_struct {
                        return vec![
                            Self::OpaqueDecl {
                                id: ItemIdentifier::new(&entity, context),
                                generics: data
                                    .generics
                                    .iter()
                                    .map(|generic| (generic.clone(), None))
                                    .collect(),
                                encoding_name: encoding_name.unwrap().to_string(),
                                availability: Availability::parse(&entity, context),
                                documentation: Documentation::from_entity(&entity, context),
                                is_cf,
                                sendable,
                                bridged: bridged.flatten(),
                                superclass: None,
                            },
                            Self::AliasDecl {
                                id,
                                availability,
                                ty,
                                kind,
                                documentation,
                            },
                        ];
                    }
                }

                if sendable.is_some() {
                    warn!(name = ?id.name, ?sendable, "unhandled sendability attribute on typedef");
                }

                let id = context.replace_typedef_name(id, ty.is_cf_type_ptr());

                if id.name != c_name {
                    documentation.set_alias(c_name);
                }

                vec![Self::AliasDecl {
                    id,
                    availability,
                    ty,
                    kind,
                    documentation,
                }]
            }
            EntityKind::StructDecl | EntityKind::UnionDecl => {
                let is_union = entity.get_kind() == EntityKind::UnionDecl;
                let id = id
                    .map_name(|name| name.or_else(|| anonymous_record_name(entity, context)))
                    .to_option();
                let Some(id) = id else {
                    warn!(?entity, "skipped anonymous union/struct");
                    return vec![];
                };

                // See https://github.com/rust-lang/rust-bindgen/blob/95fd17b874910184cc0fcd33b287fa4e205d9d7a/bindgen/ir/comp.rs#L1392-L1408
                if !entity.is_definition() {
                    return vec![];
                }

                let ty = entity.get_type().unwrap();
                let enc = ty.get_objc_encoding().expect("record has encoding");
                let encoding_name = enc
                    .strip_prefix('{')
                    .unwrap_or_else(|| {
                        enc.strip_prefix('(')
                            .expect("record has { or ( in encoding")
                    })
                    .split_once('=')
                    .unwrap()
                    .0
                    .to_string();

                let mut boxable = false;
                let mut fields = Vec::new();
                let mut sendable = None;
                let mut packed = false;
                let align = ty.get_alignof().expect("alignment of record type");
                let mut natural_align = 0;

                let mut res = vec![];

                immediate_children(entity, |entity, span| match entity.get_kind() {
                    EntityKind::UnexposedAttr => {
                        if let Some(attr) = UnexposedAttr::parse(&entity, context) {
                            match attr {
                                UnexposedAttr::Sendable => sendable = Some(true),
                                UnexposedAttr::NonSendable => sendable = Some(false),
                                attr => error!(?attr, "unknown attribute on struct/union"),
                            }
                        }
                    }
                    EntityKind::FieldDecl => {
                        drop(span);
                        let name = entity.get_name().unwrap_or_else(|| "__unknown__".into());
                        let _span = debug_span!("field", name).entered();

                        let ty = entity.get_type().expect("struct/union field type");
                        let field_align = ty.get_alignof().unwrap();
                        if align < field_align {
                            // Similar to what bindgen does, we cannot detect
                            // #pragma pack, but we can detect it's effect
                            // (that the type itself has lower alignment than
                            // its fields).
                            packed = true;
                        }
                        // Compute the natural alignment of the struct.
                        natural_align = natural_align.max(field_align);

                        let ty = Ty::parse_record_field(ty, context);

                        if entity.is_bit_field() {
                            error!("unsound struct/union bitfield");
                        }

                        let documentation = Documentation::from_entity(&entity, context);
                        fields.push((name, documentation, ty))
                    }
                    EntityKind::ObjCBoxable => {
                        boxable = true;
                    }
                    EntityKind::UnionDecl | EntityKind::StructDecl => {
                        // Recursively parse inner unions and structs, but
                        // emit them at the top-level.
                        res.extend(Self::parse(&entity, context, current_library));
                    }
                    EntityKind::PackedAttr => {
                        assert_eq!(align, 1, "packed records must have an alignment of 1");
                    }
                    EntityKind::VisibilityAttr => {}
                    EntityKind::AlignedAttr => {
                        // Handled with ty.get_alignof() above.
                    }
                    _ => error!(?entity, "unknown struct/union child"),
                });

                if fields.iter().any(|(_, _, field_ty)| field_ty.needs_simd()) {
                    debug!("simd types are not yet possible in struct/union");
                    return res;
                }

                res.push(Self::RecordDecl {
                    id,
                    encoding_name,
                    availability,
                    boxable,
                    fields,
                    sendable,
                    align,
                    natural_align,
                    documentation,
                    is_union,
                });

                res
            }
            EntityKind::EnumDecl => {
                // Enum declarations show up twice for some reason, but
                // luckily this flag is set on the least descriptive entity.
                if !entity.is_definition() {
                    return vec![];
                }

                let ty = entity.get_enum_underlying_type().expect("enum type");
                let mut ty = Ty::parse_enum(ty, context);
                let is_signed = ty.is_signed().unwrap_or_else(|| {
                    error!(?ty, "cannot determine signed-ness of enum");
                    false
                });
                let mut kind = None;
                let mut variants = Vec::new();
                let mut sendable = None;

                immediate_children(entity, |entity, _span| match entity.get_kind() {
                    EntityKind::EnumConstantDecl => {
                        let id = ItemIdentifier::new(&entity, context);
                        let const_data = context.library(&id).get(&entity);
                        let availability = Availability::parse(&entity, context);

                        if const_data.skipped.unwrap_or(false) {
                            return;
                        }

                        let value = entity
                            .get_enum_constant_value()
                            .expect("enum constant value");

                        let mut expr = if is_signed {
                            Expr::Signed(value.0)
                        } else {
                            Expr::Unsigned(value.1)
                        };

                        if !(const_data.use_value.or(data.use_value).unwrap_or(false)) {
                            // Some enums constants don't declare a value, but
                            // let it be inferred from the position in the
                            // enum instead; in those cases, we use the value
                            // generated above.
                            immediate_children(&entity, |entity, _span| match entity.get_kind() {
                                EntityKind::UnexposedAttr => {
                                    if let Some(attr) = UnexposedAttr::parse(&entity, context) {
                                        match attr {
                                            UnexposedAttr::Enum
                                            | UnexposedAttr::Options
                                            | UnexposedAttr::ClosedEnum
                                            | UnexposedAttr::ErrorEnum => {
                                                if kind.as_ref() != Some(&attr) {
                                                    error!(?kind, ?attr, "enum child had attribute that parent did not");
                                                }
                                            }
                                            attr => {
                                                error!(?attr, "unknown attribute on enum constant")
                                            }
                                        }
                                    }
                                }
                                EntityKind::VisibilityAttr => {}
                                _ if entity.is_expression() => {
                                    expr = Expr::parse_enum_constant(&entity, context);
                                }
                                _ => {
                                    panic!("unknown EnumConstantDecl child in {id:?}: {entity:?}")
                                }
                            });
                        };

                        let documentation = Documentation::from_entity(&entity, context);

                        if ty.is_simple_uint() {
                            ty = expr.guess_type(id.location());
                        }

                        variants.push((id.name, documentation, availability, expr));
                    }
                    EntityKind::UnexposedAttr => {
                        if let Some(attr) = UnexposedAttr::parse(&entity, context) {
                            match attr {
                                UnexposedAttr::Sendable => sendable = Some(true),
                                UnexposedAttr::NonSendable => sendable = Some(false),
                                attr => {
                                    if let Some(kind) = &kind {
                                        assert_eq!(
                                            kind, &attr,
                                            "got differing enum kinds in {id:?}"
                                        );
                                    } else {
                                        kind = Some(attr);
                                    }
                                }
                            }
                        }
                    }
                    EntityKind::FlagEnum => {
                        let macro_ = UnexposedAttr::Options;
                        if let Some(kind) = &kind {
                            assert_eq!(kind, &macro_, "got differing enum kinds in {id:?}");
                        } else {
                            kind = Some(macro_);
                        }
                    }
                    EntityKind::VisibilityAttr => {
                        // Already exposed as entity.get_visibility()
                    }
                    _ => error!("unknown"),
                });

                if id.name.is_none() {
                    // Availability propagates to the variants automatically
                    let _ = availability;
                    // TODO: Unsure how to handle error enums
                    assert!(matches!(
                        kind,
                        None | Some(UnexposedAttr::Enum) | Some(UnexposedAttr::ErrorEnum)
                    ));
                    assert_eq!(sendable, None);
                    let variants_len = variants.len();
                    variants
                        .into_iter()
                        .enumerate()
                        .map(
                            |(i, (name, documentation, availability, value))| Self::ConstDecl {
                                id: id.clone().map_name(|_| name),
                                availability,
                                ty: ty.clone(),
                                value,
                                is_last: i == variants_len - 1,
                                documentation,
                            },
                        )
                        .collect()
                } else {
                    vec![Self::EnumDecl {
                        id: id.map_name(|name| name.unwrap()),
                        availability,
                        ty,
                        kind,
                        variants,
                        sendable,
                        documentation,
                    }]
                }
            }
            EntityKind::VarDecl => {
                let ty = entity.get_type().expect("var type");
                let mut ty = Ty::parse_static(ty, context);

                if let Some(nullability) = data.nullability {
                    ty.change_nullability(nullability.into());
                }

                if ty.needs_simd() {
                    debug!("simd types are not yet possible in statics");
                    return vec![];
                }

                let mut value = None;
                immediate_children(entity, |entity, _span| match entity.get_kind() {
                    EntityKind::UnexposedAttr => {
                        if let Some(attr) = UnexposedAttr::parse(&entity, context) {
                            match attr {
                                // Swift makes some statics associated with a
                                // certain type, which means it needs this to
                                // allow accessing them from any thread. We
                                // don't generally restrict statics in this
                                // fashion, so it shouldn't matter for us.
                                UnexposedAttr::NonIsolated => {}
                                attr => error!(?attr, "unknown attribute on var"),
                            }
                        }
                    }
                    EntityKind::VisibilityAttr => {}
                    EntityKind::ObjCClassRef => {}
                    EntityKind::TypeRef => {}
                    _ if entity.is_expression() => {
                        if value.is_none() {
                            if data.use_value.unwrap_or(false) {
                                value = Some(Expr::from_evaluated(&entity));
                            } else {
                                value = Some(Expr::parse_var(&entity, context));
                            }
                        } else {
                            error!(?value, ?entity, "got variable value twice");
                        }
                    }
                    _ => error!(?id, ?entity, "unknown vardecl child"),
                });

                vec![Self::VarDecl {
                    id: id.require_name(),
                    link_name: c_name.unwrap(),
                    availability,
                    ty,
                    value,
                    documentation,
                }]
            }
            EntityKind::FunctionDecl => {
                let c_name = c_name.unwrap();

                if entity.is_variadic() {
                    debug!(?c_name, "can't handle variadic function");
                    return vec![];
                }

                let result_type = entity.get_result_type().expect("function result type");
                let mut result_type = Ty::parse_function_return(result_type, context);
                result_type.apply_override(&data.return_);
                let mut arguments = Vec::new();
                let mut must_use = false;
                // Assume by default that functions can unwind.
                let mut abi = Abi::CUnwind;
                let mut link_name = c_name.clone();

                if entity.is_static_method() {
                    warn!("unexpected static method");
                }

                let mut returns_retained = follows_create_rule(&c_name);

                immediate_children(entity, |entity, _span| match entity.get_kind() {
                    EntityKind::UnexposedAttr => {
                        if let Some(attr) = UnexposedAttr::parse(&entity, context) {
                            match attr {
                                // TODO
                                UnexposedAttr::UIActor => {
                                    warn!("unhandled UIActor on function declaration")
                                }
                                UnexposedAttr::BridgedImplicit => {}
                                UnexposedAttr::ReturnsRetained => {
                                    // Override the inferred value.
                                    returns_retained = true;
                                }
                                UnexposedAttr::ReturnsNotRetained => {
                                    // Override the inferred value.
                                    returns_retained = false;
                                }
                                UnexposedAttr::NoThrow => {
                                    abi = Abi::C;
                                }
                                _ => error!(?attr, "unknown attribute on function"),
                            }
                        }
                    }
                    EntityKind::ObjCClassRef
                    | EntityKind::TypeRef
                    | EntityKind::ObjCProtocolRef => {}
                    EntityKind::ParmDecl => {
                        let attr = parse_fn_param_children(&entity, context);
                        // Could also be retrieved via `get_arguments`
                        let name = entity.get_name().unwrap_or_else(|| "_".into());
                        let ty = entity.get_type().expect("function argument type");
                        let mut ty = Ty::parse_function_argument(ty, attr, context);
                        if let Some(override_) = data.arguments.get(&arguments.len()) {
                            ty.apply_override(override_);
                        }
                        arguments.push((name, ty))
                    }
                    EntityKind::WarnUnusedResultAttr => {
                        must_use = true;
                    }
                    EntityKind::PureAttr | EntityKind::ConstAttr => {
                        // Ignore, we currently have no way of marking
                        // external functions as pure in Rust.
                    }
                    EntityKind::AsmLabelAttr => {
                        let name = entity.get_name().expect("asm label to have name");
                        let name = if let Some(name) = name.strip_prefix('_') {
                            name.to_string()
                        } else {
                            error!(?name, "symbol did not start with _");
                            name
                        };
                        link_name = name;
                    }
                    EntityKind::VisibilityAttr => {
                        // CG_EXTERN or UIKIT_EXTERN
                    }
                    _ => error!("unknown"),
                });

                if result_type.needs_simd()
                    || arguments.iter().any(|(_, arg_ty)| arg_ty.needs_simd())
                {
                    debug!("simd types are not yet possible in functions");
                    return vec![];
                }

                if let Some((_, first_ty)) = arguments.first_mut() {
                    first_ty.fix_fn_first_argument_cf_nullability(&c_name);
                }

                // Don't map `CFRetain`, `CFRelease`, `CFAutorelease`, as well
                // as custom ones like as `CGColorRelease`, but not things
                // like `CMBufferQueueDequeueAndRetain`.
                //
                // Roughly the same as what Swift does:
                // <https://github.com/swiftlang/swift/blob/swift-6.0.3-RELEASE/lib/ClangImporter/ImportDecl.cpp#L8435-L8452>
                //
                // Users can achieve (almost) the same by using `CFRetained`
                // and `ManuallyDrop`, and this is clearer and safer.
                //
                // Besides, these do not have the necessary memory management
                // attributes (cf_consumed/cf_returns_retained), and as such
                // cannot be mapped correctly without extra hacks.
                if (c_name.ends_with("Retain")
                    || c_name.ends_with("Release")
                    || c_name.ends_with("Autorelease"))
                    && !c_name.ends_with("AndRetain")
                {
                    if let Some((_, first_arg_ty)) = arguments.first() {
                        if first_arg_ty.is_cf_type_ptr() {
                            return vec![];
                        }
                    }
                }

                let body = if entity.is_inline_function() {
                    Some(())
                } else {
                    None
                };

                let default_safety = context.library(&id).default_safety;

                let mut any_argument_bounds_affecting = false;
                let mut safety = arguments
                    .iter()
                    .fold(SafetyProperty::Safe, |mut safety, (arg_name, arg_ty)| {
                        if !default_safety.bounds_checked_internally
                            && is_likely_bounds_affecting(arg_name)
                            && arg_ty.can_affect_bounds()
                        {
                            any_argument_bounds_affecting = true;
                            safety = safety.merge(SafetyProperty::new_unknown(format!(
                                "`{arg_name}` might not be bounds-checked"
                            )));
                        }
                        safety.merge(arg_ty.safety_in_fn_argument(&crate::to_snake_case(arg_name)))
                    })
                    .merge(result_type.safety_in_fn_return());

                if !default_safety.bounds_checked_internally
                    && !any_argument_bounds_affecting
                    && is_likely_bounds_affecting(&c_name)
                    && arguments
                        .iter()
                        .any(|(_, arg_ty)| arg_ty.can_affect_bounds())
                {
                    safety =
                        safety.merge(SafetyProperty::new_unknown("Might not be bounds-checked"));
                }

                let safe = if let Some(unsafe_) = data.unsafe_ {
                    if safety.is_unsafe() && !unsafe_ {
                        // TODO(breaking): Disallow these.
                        error!(?id, "unsafe function was marked as safe");
                    }
                    !unsafe_
                } else {
                    // TODO(breaking): Remove unavailable instead of just marking them unsafe.
                    safety.is_safe()
                        && default_safety.automatically_safe
                        && availability.is_available()
                };

                if let Some(safety) = safety.to_safety_comment() {
                    if data.unsafe_ != Some(false) {
                        documentation.add("# Safety");
                        documentation.add(safety);
                    }
                }

                vec![Self::FnDecl {
                    id: id.require_name(),
                    c_name,
                    link_name,
                    availability,
                    arguments,
                    // May be changed by global analysis.
                    first_arg_is_self: false,
                    result_type,
                    body,
                    safe,
                    must_use,
                    abi,
                    returns_retained,
                    documentation,
                    no_implementor: data.no_implementor,
                    custom_implementor: data.implementor.clone().map(ItemTree::from_id),
                }]
            }
            EntityKind::UnexposedDecl => {
                // `@compatibility_alias`, can be ignored (since we don't
                // need to support older SDK versions).
                vec![]
            }
            _ => {
                error!("unknown");
                vec![]
            }
        }
    }

    /// Ideally, we'd import macros in a similar fashion as Swift does:
    /// <https://github.com/swiftlang/swift/blob/swift-6.0.3-RELEASE/docs/HowSwiftImportsCAPIs.md#macros>
    /// <https://developer.apple.com/documentation/swift/using-imported-c-macros-in-swift>
    ///
    /// I couldn't figure out a clean way to do that though, so our impl is a bit messy.
    pub fn parse_macro_definition(entity: &Entity<'_>, context: &Context<'_>) -> Option<Self> {
        let id = ItemIdentifier::new(entity, context);

        if entity.is_function_like_macro() {
            // Ignore function-like macros for now.
            return None;
        };

        let data = context.library(&id).const_data.get(&id.name);
        let skipped = data.and_then(|data| data.skipped);

        if skipped.unwrap_or(false) {
            return None;
        }

        if skipped.is_none()
            && !id.name.starts_with('k')
            && !id.name.to_lowercase().contains("version")
        {
            // By default, we want to map macros that start with "k", or
            // contain the word "version". Seems to get us most of the things
            // we want, without emitting weird things like `CG_INLINE`.
            return None;
        }

        let mut value = Expr::parse_macro_definition(entity, context)?;
        let _ = value.update_idents(&context.ident_mapping);

        let ty = value.guess_type(id.location());

        if ty.is_object_like_ptr() {
            // cf_string! and ns_string! are not supported (since they cannot
            // yet be used in `const`).
            return None;
        }

        Some(Self::ConstDecl {
            id,
            // macro definitions cannot have availability.
            availability: Availability::default(),
            ty,
            value,
            is_last: false,
            documentation: Documentation::from_entity(entity, context),
        })
    }

    pub fn get_ident_mapping(&self) -> HashMap<String, Expr> {
        let mut mapping = HashMap::new();

        match self {
            Stmt::EnumDecl {
                id,
                ty,
                kind,
                variants,
                ..
            } => {
                let mut relevant_enum_cases = variants
                    .iter()
                    .filter(|(_, _, availability, _)| availability.is_available_non_deprecated())
                    .map(|(name, _, _, _)| &**name)
                    .peekable();
                let prefix = if relevant_enum_cases.peek().is_some() {
                    enum_prefix(&id.name, relevant_enum_cases)
                } else {
                    enum_prefix(&id.name, variants.iter().map(|(name, _, _, _)| &**name))
                };

                for (name, ..) in variants {
                    mapping.insert(
                        name.clone(),
                        Expr::Enum {
                            id: id.clone(),
                            variant: name.strip_prefix(prefix).unwrap_or(name).to_string(),
                            ty: ty.clone(),
                            attrs: kind.iter().cloned().collect(),
                        },
                    );
                }
            }
            Stmt::ConstDecl { id, ty, .. } => {
                mapping.insert(
                    id.name.clone(),
                    Expr::Const {
                        id: id.clone(),
                        ty: ty.clone(),
                    },
                );
            }
            Stmt::VarDecl { id, ty, .. } => {
                mapping.insert(
                    id.name.clone(),
                    Expr::Var {
                        id: id.clone(),
                        ty: ty.clone(),
                    },
                );
            }
            _ => {}
        }

        mapping
    }

    pub(crate) fn provided_item(&self) -> Option<ItemIdentifier> {
        match self {
            Self::ClassDecl { id, skipped, .. } => {
                if *skipped {
                    None
                } else {
                    Some(id.clone())
                }
            }
            Self::ExternMethods { .. } => None,
            Self::ExternCategory { id, .. } => Some(id.clone()),
            Self::ProtocolDecl { id, .. } => Some(id.clone()),
            Self::ProtocolImpl { .. } => None,
            Self::RecordDecl { id, .. } => Some(id.clone()),
            Self::EnumDecl { id, .. } => Some(id.clone()),
            Self::ConstDecl { id, .. } => Some(id.clone()),
            Self::VarDecl { id, .. } => Some(id.clone()),
            Self::FnDecl { id, body: None, .. } => Some(id.clone()),
            // TODO
            Self::FnDecl { body: Some(_), .. } => None,
            Self::FnGetTypeId { .. } => None, // Emits a trait impl
            Self::AliasDecl { id, .. } => Some(id.clone()),
            Self::OpaqueDecl { id, .. } => Some(id.clone()),
            Self::GeneralImpl { .. } => None,
        }
    }

    /// Whether it is possible to attach `impl` stmts to the provided item.
    pub(crate) fn implementable(&self) -> bool {
        matches!(
            self,
            Self::ClassDecl { skipped: false, .. }
                | Self::RecordDecl { .. }
                | Self::EnumDecl { .. }
                | Self::OpaqueDecl { .. }
        )
    }

    pub(crate) fn location(&self) -> &Location {
        match self {
            Self::ClassDecl { id, .. } => id.location(),
            Self::ExternMethods { location, .. } => location,
            Self::ExternCategory { id, .. } => id.location(),
            Self::ProtocolDecl { id, .. } => id.location(),
            Self::ProtocolImpl { location, .. } => location,
            Self::RecordDecl { id, .. } => id.location(),
            Self::EnumDecl { id, .. } => id.location(),
            Self::ConstDecl { id, .. } => id.location(),
            Self::VarDecl { id, .. } => id.location(),
            Self::FnDecl { id, .. } => id.location(),
            Self::FnGetTypeId { id, .. } => id.location(),
            Self::AliasDecl { id, .. } => id.location(),
            Self::OpaqueDecl { id, .. } => id.location(),
            Self::GeneralImpl { location, .. } => location,
        }
    }

    /// Items required by the statement at the top-level.
    pub(crate) fn required_items(&self) -> impl Iterator<Item = ItemTree> {
        let items: Vec<ItemTree> = match self {
            Self::ClassDecl {
                superclasses,
                generics,
                ..
            } => iter::once(ItemTree::objc("__macros__"))
                .chain(superclasses_required_items(
                    superclasses.iter().map(|(s, _)| s.clone()),
                ))
                .chain(
                    generics
                        .iter()
                        .flat_map(|(_, bound)| bound.as_ref())
                        .flat_map(|bound| bound.required_items()),
                )
                .collect(),
            Self::ExternMethods {
                cls,
                cls_superclasses,
                cls_generics,
                ..
            } => vec![
                ItemTree::objc("__macros__"),
                ItemTree::new(
                    cls.clone(),
                    superclasses_required_items(cls_superclasses.iter().cloned()),
                ),
            ]
            .into_iter()
            .chain(
                cls_generics
                    .iter()
                    .flat_map(|(_, bound)| bound.as_ref())
                    .flat_map(|bound| bound.required_items()),
            )
            .collect(),
            // Intentionally doesn't require anything, the impl itself is
            // cfg-gated
            Self::ExternCategory { .. } => {
                vec![ItemTree::objc("__macros__")]
            }
            Self::ProtocolDecl {
                super_protocols, ..
            } => iter::once(ItemTree::objc("__macros__"))
                .chain(super_protocols.iter().flat_map(|p| p.required_items()))
                .collect(),
            Self::ProtocolImpl {
                cls,
                cls_superclasses,
                protocol,
                protocol_super_protocols,
                generics,
                ..
            } => vec![
                ItemTree::new(
                    cls.clone(),
                    superclasses_required_items(cls_superclasses.iter().cloned()),
                ),
                ItemTree::new(
                    protocol.clone(),
                    protocol_super_protocols
                        .iter()
                        .flat_map(|p| p.required_items()),
                ),
                ItemTree::objc("__macros__"),
            ]
            .into_iter()
            .chain(
                generics
                    .iter()
                    .flat_map(|(_, bound)| bound.as_ref())
                    .flat_map(|bound| bound.required_items()),
            )
            .collect(),
            Self::RecordDecl { fields, .. } => fields
                .iter()
                .flat_map(|(_, _, field_ty)| field_ty.required_items())
                .collect(),
            // Variants manage required items themselves
            Self::EnumDecl { ty, .. } => ty.required_items().collect(),
            Self::ConstDecl { ty, value, .. } => {
                ty.required_items().chain(value.required_items()).collect()
            }
            Self::VarDecl { ty, value, .. } => ty
                .required_items()
                .chain(value.iter().flat_map(|value| value.required_items()))
                .collect(),
            Self::FnDecl {
                arguments,
                result_type,
                body: None,
                ..
            } => {
                let mut items = Vec::new();
                for (_, arg_ty) in arguments {
                    items.extend(arg_ty.required_items());
                }
                items.extend(result_type.fn_return_required_items());
                items
            }
            // TODO
            Self::FnDecl { body: Some(_), .. } => Vec::new(),
            Self::FnGetTypeId {
                cf_item,
                result_type,
                ..
            } => {
                let mut items = vec![ItemTree::cf("ConcreteType"), cf_item.clone()];
                items.extend(result_type.fn_return_required_items());
                items
            }
            Self::AliasDecl { ty, .. } => ty.required_items().collect(),
            Self::OpaqueDecl {
                generics,
                superclass,
                ..
            } => {
                let mut items = vec![ItemTree::unsafecell(), ItemTree::phantoms()];
                if let Some(superclass) = superclass {
                    items.push(ItemTree::new(superclass.clone(), items.clone()));
                }
                if !generics.is_empty() {
                    items.push(ItemTree::core_ffi("c_void"));
                }
                items.extend(
                    generics
                        .iter()
                        .flat_map(|(_, bound)| bound.as_ref())
                        .flat_map(|bound| bound.required_items()),
                );
                items
            }
            Self::GeneralImpl { item, .. } => {
                vec![item.clone()]
            }
        };

        items.into_iter()
    }

    /// Items required for any part of the statement.
    pub(crate) fn required_items_inner(&self) -> impl Iterator<Item = ItemTree> {
        let required_by_inner: Vec<ItemTree> = match self {
            Self::ClassDecl {
                bridged_to: Some(bridged_to),
                ..
            } => vec![ItemTree::from_id(bridged_to.clone())],
            Self::ExternCategory {
                cls,
                cls_superclasses,
                cls_generics,
                methods,
                ..
            } => iter::once(ItemTree::new(
                cls.clone(),
                superclasses_required_items(cls_superclasses.iter().cloned()),
            ))
            .chain(methods.iter().flat_map(|method| method.required_items()))
            .chain(
                cls_generics
                    .iter()
                    .flat_map(|(_, bound)| bound.as_ref())
                    .flat_map(|bound| bound.required_items()),
            )
            .collect(),
            Self::ExternMethods { methods, .. } | Self::ProtocolDecl { methods, .. } => methods
                .iter()
                .flat_map(|method| method.required_items())
                .collect(),
            Self::RecordDecl { .. } => vec![ItemTree::objc("Encoding")],
            Self::EnumDecl { kind, variants, .. } => {
                let mut items: Vec<_> = variants
                    .iter()
                    .flat_map(|(_, _, _, expr)| expr.required_items())
                    .collect();
                if let Some(UnexposedAttr::Options) = kind {
                    items.push(ItemTree::bitflags());
                }
                items.push(ItemTree::objc("Encoding"));
                items
            }
            Self::OpaqueDecl { is_cf, .. } => {
                if *is_cf {
                    vec![ItemTree::cf("cf_type"), ItemTree::objc("cf_objc2_type")]
                } else {
                    vec![ItemTree::objc("Encoding")]
                }
            }
            Self::GeneralImpl { stmts, .. } => stmts
                .iter()
                .flat_map(|stmt| stmt.required_items().chain(stmt.required_items_inner()))
                .collect(),
            _ => vec![],
        };
        self.required_items().chain(required_by_inner)
    }

    fn cfg_gate_ln<'a>(&'a self, config: &'a Config) -> impl fmt::Display + 'a {
        self.cfg_gate_ln_for(self.required_items(), config)
    }

    fn cfg_gate_ln_for<'a>(
        &'a self,
        required_items: impl IntoIterator<Item = ItemTree> + 'a,
        config: &'a Config,
    ) -> impl fmt::Display + 'a {
        cfg_gate_ln(required_items, [] as [ItemTree; 0], config, self.location())
    }

    fn cfg_gate_ln_inner<'a>(
        &'a self,
        required_items: impl IntoIterator<Item = ItemTree> + 'a,
        config: &'a Config,
    ) -> impl fmt::Display + 'a {
        cfg_gate_ln(
            required_items,
            self.required_items(),
            config,
            self.location(),
        )
    }

    pub fn fmt<'a>(&'a self, config: &'a Config) -> impl fmt::Display + 'a {
        FormatterFn(move |f| {
            let _span = debug_span!("stmt", provided_item = ?self.provided_item()).entered();

            // TODO: Derive this after https://github.com/madsmtm/objc2/issues/55
            fn unsafe_impl_encode<'a>(
                ident: impl fmt::Display + 'a,
                encoding: impl fmt::Display + 'a,
            ) -> impl fmt::Display + 'a {
                FormatterFn(move |f| {
                    writeln!(f, "unsafe impl Encode for {ident} {{")?;
                    writeln!(f, "    const ENCODING: Encoding = {encoding};")?;
                    writeln!(f, "}}")?;
                    Ok(())
                })
            }

            // TODO: Derive this after https://github.com/madsmtm/objc2/issues/55
            fn unsafe_impl_refencode<'a>(ident: impl fmt::Display + 'a) -> impl fmt::Display + 'a {
                FormatterFn(move |f| {
                    writeln!(f, "unsafe impl RefEncode for {ident} {{")?;
                    writeln!(
                        f,
                        "    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);"
                    )?;
                    writeln!(f, "}}")?;
                    Ok(())
                })
            }

            match self {
                Self::ClassDecl {
                    id,
                    objc_name,
                    generics,
                    availability,
                    superclasses,
                    designated_initializers: _,
                    derives,
                    main_thread_only,
                    skipped,
                    sendable,
                    documentation,
                    bridged_to,
                } => {
                    if *skipped {
                        return Ok(());
                    }

                    let cfg = self.cfg_gate_ln_for([ItemTree::objc("extern_class")], config);
                    write!(f, "{cfg}")?;
                    writeln!(f, "extern_class!(")?;
                    write!(f, "{}", documentation.fmt(Some(id)))?;
                    write!(f, "    #[unsafe(super(")?;
                    for (i, (superclass, generics)) in superclasses.iter().enumerate() {
                        if 0 < i {
                            write!(f, ", ")?;
                        }
                        write!(
                            f,
                            "{}{}",
                            superclass.path_in_relation_to(id.location()),
                            GenericTyHelper(generics)
                        )?;
                    }
                    writeln!(f, "))]")?;
                    if *main_thread_only {
                        writeln!(f, "    #[thread_kind = MainThreadOnly]")?;
                    }
                    if *objc_name != id.name {
                        writeln!(f, "    #[name = {objc_name:?}]")?;
                    }
                    writeln!(f, "    {derives}")?;
                    write!(f, "    {}", self.cfg_gate_ln(config))?;
                    write!(f, "    {availability}")?;
                    write!(f, "    pub struct {}", id.name)?;
                    if !generics.is_empty() {
                        write!(f, "<")?;
                        for (generic, bound) in generics {
                            write!(f, "{generic}: ?Sized")?;
                            if let Some(_bound) = bound {
                                // TODO(breaking): Add the bound and avoid the
                                // default generic `AnyObject`.
                                //
                                // write!(f, " + {}, ", bound.generic_bound())?;
                            }
                            write!(f, " = AnyObject, ")?;
                        }
                        write!(f, ">")?;
                    };
                    writeln!(f, ";")?;
                    writeln!(f, ");")?;

                    if *sendable && generics.is_empty() {
                        writeln!(f)?;
                        write!(f, "{}", self.cfg_gate_ln(config))?;
                        writeln!(f, "unsafe impl Send for {} {{}}", id.name)?;

                        writeln!(f)?;
                        write!(f, "{}", self.cfg_gate_ln(config))?;
                        writeln!(f, "unsafe impl Sync for {} {{}}", id.name)?;
                    }

                    if let Some(bridged_to) = bridged_to {
                        writeln!(f)?;
                        write!(
                            f,
                            "{}",
                            self.cfg_gate_ln_for([ItemTree::from_id(bridged_to.clone())], config)
                        )?;
                        writeln!(
                            f,
                            "impl{} AsRef<{}{}> for {}{} {{",
                            GenericParamsHelper(generics, "?Sized + Message"),
                            id.path(),
                            generic_ty(generics),
                            bridged_to.name,
                            generic_ty(generics),
                        )?;
                        writeln!(f, "    #[inline]")?;
                        writeln!(
                            f,
                            "    fn as_ref(&self) -> &{}{} {{",
                            id.path(),
                            generic_ty(generics),
                        )?;
                        // SAFETY: The two types are toll-free bridged, and
                        // can hence be freely converted to/from each other.
                        //
                        // For generic types like `CFArray<T>`, this is only
                        // true when the `T` is messagable (so `CFArray<i32>`
                        // and similar are possible, but those won't fly).
                        writeln!(f, "        unsafe {{ &*((self as *const Self).cast()) }}",)?;
                        writeln!(f, "    }}")?;
                        writeln!(f, "}}")?;

                        writeln!(f)?;
                        write!(
                            f,
                            "{}",
                            self.cfg_gate_ln_for([ItemTree::from_id(bridged_to.clone())], config)
                        )?;
                        writeln!(
                            f,
                            "impl{} AsRef<{}{}> for {}{} {{",
                            GenericParamsHelper(generics, "?Sized + Message"),
                            bridged_to.name,
                            generic_ty(generics),
                            id.path(),
                            generic_ty(generics),
                        )?;
                        writeln!(f, "    #[inline]")?;
                        writeln!(
                            f,
                            "    fn as_ref(&self) -> &{}{} {{",
                            bridged_to.path(),
                            generic_ty(generics),
                        )?;
                        // SAFETY: Same as above.
                        writeln!(f, "        unsafe {{ &*((self as *const Self).cast()) }}",)?;
                        writeln!(f, "    }}")?;
                        writeln!(f, "}}")?;
                    }

                    // Add casting from `NSArray<T>` to `NSArray<U>`.
                    if !generics.is_empty() {
                        writeln!(f)?;
                        write!(f, "{}", self.cfg_gate_ln(config))?;
                        add_generic_cast_helpers(f, id, generics, false)?;
                    }
                }
                Self::ExternMethods {
                    location: _,
                    availability,
                    cls,
                    cls_superclasses: _,
                    cls_generics,
                    methods,
                    documentation,
                } => {
                    write!(f, "{}", documentation.fmt(None))?;
                    write!(f, "{availability}")?;
                    write!(f, "{}", self.cfg_gate_ln(config))?;
                    // TODO: Add ?Sized here once `extern_methods!` supports it.
                    writeln!(
                        f,
                        "impl{} {}{} {{",
                        GenericParamsHelper(cls_generics, "Message"),
                        cls.path(),
                        generic_ty(cls_generics),
                    )?;
                    writeln!(f, "    extern_methods!(")?;
                    for method in methods {
                        write!(
                            f,
                            "{}",
                            self.cfg_gate_ln_inner(method.required_items(), config)
                        )?;
                        writeln!(f, "{method}")?;
                    }
                    writeln!(f, "    );")?;
                    writeln!(f, "}}")?;

                    if let Some(method) = methods
                        .iter()
                        .find(|method| method.usable_in_default_retained())
                    {
                        writeln!(f)?;
                        // Assume `new` methods require no extra features
                        write!(f, "{}", self.cfg_gate_ln(config))?;
                        writeln!(
                            f,
                            "impl{} DefaultRetained for {}{} {{",
                            GenericParamsHelper(cls_generics, "Message"),
                            cls.path(),
                            generic_ty(cls_generics),
                        )?;
                        writeln!(f, "    #[inline]")?;
                        writeln!(f, "    fn default_retained() -> Retained<Self> {{")?;
                        writeln!(f, "        Self::{}()", method.fn_name)?;
                        writeln!(f, "    }}")?;
                        writeln!(f, "}}")?;
                    }
                }
                Self::ExternCategory {
                    id,
                    availability,
                    cls,
                    cls_superclasses,
                    cls_generics,
                    methods,
                    documentation,
                } => {
                    // Helper module to seal the trait.
                    writeln!(f, "mod private_{} {{", id.name)?;
                    writeln!(f, "    pub trait Sealed {{}}")?;
                    writeln!(f, "}}")?;

                    writeln!(f)?;

                    write!(f, "{}", documentation.fmt(None))?;

                    write!(f, "{}", self.cfg_gate_ln(config))?;
                    write!(f, "{availability}")?;
                    writeln!(
                        f,
                        "pub unsafe trait {}{}: ClassType + Sized + private_{}::Sealed {{",
                        id.name,
                        GenericParamsHelper(cls_generics, "Message"),
                        id.name,
                    )?;
                    writeln!(f, "    extern_methods!(")?;
                    for method in methods {
                        write!(
                            f,
                            "{}",
                            self.cfg_gate_ln_inner(method.required_items(), config)
                        )?;
                        writeln!(f, "{method}")?;
                    }
                    writeln!(f, "    );")?;
                    writeln!(f, "}}")?;

                    writeln!(f)?;

                    let impl_cfg = self.cfg_gate_ln_for(
                        [ItemTree::new(
                            cls.clone(),
                            superclasses_required_items(cls_superclasses.iter().cloned()),
                        )],
                        config,
                    );

                    write!(f, "{impl_cfg}")?;
                    writeln!(
                        f,
                        "impl{} private_{}::Sealed for {}{} {{}}",
                        GenericParamsHelper(cls_generics, "Message"),
                        id.name,
                        cls.path_in_relation_to(id.location()),
                        generic_ty(cls_generics),
                    )?;

                    write!(f, "{impl_cfg}")?;
                    writeln!(
                        f,
                        "unsafe impl{} {}{} for {}{} {{}}",
                        GenericParamsHelper(cls_generics, "Message"),
                        id.name,
                        generic_ty(cls_generics),
                        cls.path_in_relation_to(id.location()),
                        generic_ty(cls_generics),
                    )?;
                }
                Self::ProtocolImpl {
                    location: id,
                    cls,
                    cls_superclasses: _,
                    cls_counterpart,
                    generics,
                    protocol,
                    protocol_super_protocols: _,
                    availability: _, // Trait implementations can't be deprecated
                } => {
                    let extra_bound = if !generics.is_empty() {
                        match (protocol.library_name(), &*protocol.name) {
                            // The object inherits from `NSObject` or `NSProxy` no
                            // matter what the generic type is, so this must be
                            // safe.
                            ("ObjectiveC", "NSObjectProtocol") => "?Sized",
                            // Encoding and decoding requires that the inner types
                            // are codable as well.
                            ("Foundation", "NSCoding") => "?Sized + NSCoding",
                            ("Foundation", "NSSecureCoding") => "?Sized + NSSecureCoding",
                            // Copying collections is done as a shallow copy:
                            // <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Collections/Articles/Copying.html>
                            //
                            // E.g. it does a retain count bump on the items, and
                            // hence does not require the inner type to implement
                            // `NSCopying`.
                            ("Foundation", "NSCopying") => "?Sized",
                            ("Foundation", "NSMutableCopying") => "?Sized",
                            // TODO: Do we need further tweaks to this?
                            ("Foundation", "NSFastEnumeration") => "?Sized",
                            // AppKit/UIKit fixes. TODO: Should we add more bounds here?
                            ("AppKit", "NSCollectionViewDataSource") => "?Sized",
                            ("UIKit", "UICollectionViewDataSource") => "?Sized + Message",
                            ("AppKit", "NSTableViewDataSource") => "?Sized",
                            ("UIKit", "UITableViewDataSource") => "?Sized + Message",
                            _ => {
                                error!(
                                    ?protocol,
                                    ?cls,
                                    "unknown where bound for generic protocol impl"
                                );
                                "?Sized"
                            }
                        }
                    } else {
                        "InvalidGenericBound"
                    };

                    write!(f, "{}", self.cfg_gate_ln(config))?;
                    writeln!(
                        f,
                        "extern_conformance!(unsafe impl{} {} for {}{} {{}});",
                        GenericParamsHelper(generics, extra_bound),
                        protocol.path_in_relation_to(id),
                        cls.path_in_relation_to(id),
                        generic_ty(generics),
                    )?;

                    // To make `NSCopying` and `NSMutableCopying` work, we
                    // need to emit `CopyingHelper` impls to tell Rust which
                    // return types they have.
                    if matches!(&*protocol.name, "NSCopying" | "NSMutableCopying") {
                        let copy_helper = ItemIdentifier::copyhelper(protocol.name != "NSCopying");

                        let mut required_items: Vec<_> = self.required_items().collect();

                        // Assume counterparts have the same generics.
                        let ty = match (cls_counterpart, &*protocol.name) {
                            (Counterpart::ImmutableSuperclass(superclass), "NSCopying") => {
                                // REMARK: Already part of required items.
                                format!(
                                    "{}{}",
                                    superclass.path_in_relation_to(id),
                                    generic_ty(generics)
                                )
                            }
                            (Counterpart::MutableSubclass(subclass), "NSMutableCopying") => {
                                required_items.push(ItemTree::from_id(subclass.clone()));
                                format!(
                                    "{}{}",
                                    subclass.path_in_relation_to(id),
                                    generic_ty(generics)
                                )
                            }
                            _ => "Self".into(),
                        };

                        writeln!(f)?;
                        write!(f, "{}", self.cfg_gate_ln_for(required_items, config))?;
                        writeln!(
                            f,
                            "unsafe impl{} {} for {}{} {{",
                            GenericParamsHelper(generics, "?Sized + Message"),
                            copy_helper.path_in_relation_to(id),
                            cls.path_in_relation_to(id),
                            generic_ty(generics),
                        )?;

                        writeln!(f, "    type Result = {ty};")?;

                        writeln!(f, "}}")?;
                    }

                    if protocol.name == "NSMutableCopying"
                        && *cls_counterpart == Counterpart::NoCounterpart
                    {
                        error!(
                            ?cls,
                            "Class implements NSMutableCopying, \
                                but does not have a counterpart. You should \
                                define the counterpart for this class, or \
                                ignore the NSMutableCopying implementation"
                        );
                    }
                }
                Self::ProtocolDecl {
                    id,
                    objc_name,
                    availability,
                    super_protocols,
                    methods,
                    required_sendable,
                    required_mainthreadonly,
                    documentation,
                } => {
                    let cfg = self.cfg_gate_ln_for([ItemTree::objc("extern_protocol")], config);
                    write!(f, "{cfg}")?;
                    writeln!(f, "extern_protocol!(")?;

                    write!(f, "{}", documentation.fmt(Some(id)))?;
                    write!(f, "    {}", self.cfg_gate_ln(config))?;
                    write!(f, "    {availability}")?;
                    if *objc_name != id.name {
                        writeln!(f, "    #[name = {objc_name:?}]")?;
                    }
                    write!(f, "    pub unsafe trait {}", id.name)?;
                    if !super_protocols.is_empty() {
                        for (i, protocol) in super_protocols.iter().enumerate() {
                            if i == 0 {
                                write!(f, ": ")?;
                            } else {
                                write!(f, "+ ")?;
                            }
                            write!(f, "{}", protocol.id.path())?;
                        }
                    }
                    if *required_sendable {
                        if super_protocols.is_empty() {
                            write!(f, ": ")?;
                        } else {
                            write!(f, "+ ")?;
                        }
                        write!(f, "Send + Sync")?;
                    }
                    if *required_mainthreadonly {
                        if super_protocols.is_empty() {
                            write!(f, ": ")?;
                        } else {
                            write!(f, "+ ")?;
                        }
                        write!(f, "MainThreadOnly")?;
                    }
                    writeln!(f, " {{")?;

                    for method in methods {
                        write!(
                            f,
                            "{}",
                            self.cfg_gate_ln_inner(method.required_items(), config)
                        )?;
                        writeln!(f, "{method}")?;
                    }
                    writeln!(f, "    }}")?;
                    writeln!(f)?;
                    writeln!(f, ");")?;
                }
                Self::RecordDecl {
                    id,
                    encoding_name,
                    availability,
                    boxable: _,
                    fields,
                    sendable,
                    align,
                    natural_align,
                    documentation,
                    is_union,
                } => {
                    write!(f, "{}", documentation.fmt(Some(id)))?;
                    write!(f, "{}", self.cfg_gate_ln(config))?;
                    write!(f, "{availability}")?;

                    // See <https://doc.rust-lang.org/reference/type-layout.html#the-alignment-modifiers>
                    match align.cmp(natural_align) {
                        Ordering::Less if *align == 1 => writeln!(f, "#[repr(C, packed)]")?,
                        Ordering::Less => writeln!(f, "#[repr(C, packed({align}))]")?,
                        Ordering::Equal => writeln!(f, "#[repr(C)]")?,
                        Ordering::Greater => writeln!(f, "#[repr(C, align({align}))]")?,
                    }

                    if *is_union || fields.iter().any(|(_, _, field)| field.contains_union()) {
                        writeln!(f, "#[derive(Clone, Copy)]")?;
                    } else {
                        if fields
                            .iter()
                            .any(|(_, _, field)| field.directly_contains_fn_ptr())
                        {
                            // TODO(breaking): Maybe remove the PartialEq implementation here?
                            writeln!(f, "#[allow(unpredictable_function_pointer_comparisons)]")?;
                        }
                        writeln!(f, "#[derive(Clone, Copy, Debug, PartialEq)]")?;
                    }
                    if *is_union {
                        writeln!(f, "pub union {} {{", id.name)?;
                    } else {
                        writeln!(f, "pub struct {} {{", id.name)?;
                    }
                    for (name, documentation, ty) in fields {
                        write!(f, "{}", documentation.fmt(None))?;
                        write!(f, "    ")?;
                        if name.starts_with('_') {
                            write!(f, "pub(crate) ")?;
                        } else {
                            write!(f, "pub ")?;
                        }
                        let name = handle_reserved(name);
                        writeln!(f, "{name}: {},", ty.record())?;
                    }
                    writeln!(f, "}}")?;
                    writeln!(f)?;

                    let required_items = self
                        .required_items()
                        .chain(iter::once(ItemTree::objc("Encoding")));
                    let cfg_encoding = self.cfg_gate_ln_for(required_items, config);

                    let encoding = FormatterFn(|f| {
                        if *is_union {
                            write!(f, "Encoding::Union")?;
                        } else {
                            write!(f, "Encoding::Struct")?;
                        }
                        writeln!(f, "({encoding_name:?}, &[")?;
                        for (_, _, ty) in fields {
                            writeln!(f, "        {},", ty.record_encoding())?;
                        }
                        write!(f, "    ])")?;
                        Ok(())
                    });

                    // SAFETY: The struct/union is marked `#[repr(C)]`.
                    write!(f, "{cfg_encoding}")?;
                    writeln!(f, "{}", unsafe_impl_encode(&id.name, encoding))?;
                    write!(f, "{cfg_encoding}")?;
                    writeln!(f, "{}", unsafe_impl_refencode(&id.name))?;

                    if let Some(true) = sendable {
                        writeln!(f)?;
                        write!(f, "{}", self.cfg_gate_ln(config))?;
                        writeln!(f, "unsafe impl Send for {} {{}}", id.name)?;

                        writeln!(f)?;
                        write!(f, "{}", self.cfg_gate_ln(config))?;
                        writeln!(f, "unsafe impl Sync for {} {{}}", id.name)?;
                    }
                }
                Self::EnumDecl {
                    id,
                    availability,
                    ty,
                    kind,
                    variants,
                    sendable,
                    documentation,
                } => {
                    write!(f, "{}", documentation.fmt(Some(id)))?;

                    let mut relevant_enum_cases = variants
                        .iter()
                        .filter(|(_, _, availability, _)| {
                            availability.is_available_non_deprecated()
                        })
                        .map(|(name, _, _, _)| &**name)
                        .peekable();
                    let prefix = if relevant_enum_cases.peek().is_some() {
                        enum_prefix(&id.name, relevant_enum_cases)
                    } else {
                        enum_prefix(&id.name, variants.iter().map(|(name, _, _, _)| &**name))
                    };

                    match kind {
                    // TODO: Once Rust gains support for more precisely
                    // specifying niches, use that to convert this into a
                    // native enum with a hidden variant that contains the
                    // remaining cases.
                    None
                    // Swift emits these slightly differently, with
                    // only `NS_ENUM` being a "native" enum with
                    // exhaustiveness-checking and all.
                    //
                    // <https://developer.apple.com/documentation/swift/grouping-related-objective-c-constants#Declare-Simple-Enumerations>
                    | Some(UnexposedAttr::Enum)
                    // TODO: Handle this differently.
                    | Some(UnexposedAttr::ErrorEnum) => {
                        match kind {
                            None => {}
                            Some(UnexposedAttr::Enum) => writeln!(f, "// NS_ENUM")?,
                            Some(UnexposedAttr::ErrorEnum) => writeln!(f, "// NS_ERROR_ENUM")?,
                            _ => unreachable!(),
                        }

                        write!(f, "{}", self.cfg_gate_ln(config))?;
                        write!(f, "{availability}")?;
                        writeln!(f, "#[repr(transparent)]")?;
                        // TODO: Implement `Debug` manually
                        writeln!(
                            f,
                            "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]"
                        )?;
                        // External enums can be safely constructed from the
                        // raw value, and as such it is safe to expose it
                        // directly without causing unsoundess in external
                        // libraries (but it will likely lead to an exception
                        // or a crash, if the invalid value is used).
                        //
                        // Note: This cannot be a Rust `enum`, since that
                        // assumes that the enum is exhaustive ABI-wise, even
                        // with `#[non_exhaustive]`, see:
                        // <https://play.rust-lang.org/?version=stable&mode=release&edition=2021&gist=3a19fcb4267d6fdb0d26b0c9defd946a>
                        //
                        // What we really need here is some way to specify
                        // niches of integers, then a hacky solution to this
                        // would be doable.
                        writeln!(f, "pub struct {}(pub {});", id.name, ty.enum_())?;

                        write!(f, "{}", self.cfg_gate_ln(config))?;
                        writeln!(f, "impl {} {{", id.name)?;

                        for (name, documentation, availability, expr) in variants {
                            write!(f, "{}", documentation.fmt(None))?;
                            let pretty_name = name.strip_prefix(prefix).unwrap_or(name);
                            if pretty_name != name {
                                writeln!(f, "    #[doc(alias = \"{name}\")]")?;
                            }
                            write!(f, "    {}", self.cfg_gate_ln_inner(expr.required_items(), config))?;
                            write!(f, "    {availability}")?;
                            writeln!(f, "    pub const {pretty_name}: Self = Self({expr});")?;
                        }
                        writeln!(f, "}}")?;
                        writeln!(f)?;
                    }
                    Some(UnexposedAttr::Options) => {
                        writeln!(f, "// NS_OPTIONS")?;

                        write!(f, "{}", self.cfg_gate_ln(config))?;
                        write!(f, "{availability}")?;
                        writeln!(f, "#[repr(transparent)]")?;
                        writeln!(
                            f,
                            "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]"
                        )?;
                        // TODO(breaking): Make the inner field private
                        writeln!(f, "pub struct {}(pub {});", id.name, ty.enum_())?;

                        write!(f, "{}", self.cfg_gate_ln(config))?;
                        writeln!(f, "bitflags::bitflags! {{")?;

                        writeln!(f, "    impl {}: {} {{", id.name, ty.enum_())?;

                        for (name, documentation, availability, expr) in variants {
                            write!(f, "{}", documentation.fmt(None))?;
                            let pretty_name = name.strip_prefix(prefix).unwrap_or(name);
                            if pretty_name != name {
                                writeln!(f, "        #[doc(alias = \"{name}\")]")?;
                            }
                            write!(f, "{}", self.cfg_gate_ln_inner(expr.required_items(), config))?;
                            write!(f, "{availability}")?;
                            writeln!(f, "        const {pretty_name} = {expr};")?;
                        }
                        writeln!(f, "    }}")?;
                        writeln!(f, "}}")?;
                        writeln!(f)?;
                    }
                    Some(UnexposedAttr::ClosedEnum) => {
                        // SAFETY: `NS_CLOSED_ENUM` is guaranteed to never
                        // gain additional cases, so we are allowed to use a
                        // Rust enum (which in turn will assume that the
                        // unused patterns are valid to use as a niche).
                        writeln!(f, "// NS_CLOSED_ENUM")?;
                        write!(f, "{}", self.cfg_gate_ln(config))?;
                        write!(f, "{availability}")?;
                        writeln!(f, "{}", ty.closed_enum_repr())?;
                        writeln!(
                            f,
                            "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]"
                        )?;
                        writeln!(f, "pub enum {} {{", id.name)?;

                        for (name, documentation, availability, expr) in variants {
                            write!(f, "{}", documentation.fmt(None))?;
                            let pretty_name = name.strip_prefix(prefix).unwrap_or(name);
                            if pretty_name != name {
                                writeln!(f, "    #[doc(alias = \"{name}\")]")?;
                            }
                            write!(f, "    {}", self.cfg_gate_ln_inner(expr.required_items(), config))?;
                            write!(f, "    {availability}")?;
                            writeln!(f, "    {pretty_name} = {expr},")?;
                        }
                        writeln!(f, "}}")?;
                        writeln!(f)?;
                    }
                    _ => panic!("invalid enum kind"),
                }

                    let required_items = self
                        .required_items()
                        .chain(iter::once(ItemTree::objc("Encoding")));
                    let cfg_encoding = self.cfg_gate_ln_for(required_items, config);

                    // SAFETY: The enum is either a `#[repr(transparent)]` newtype
                    // over the type, or a `#[repr(REPR)]`, where REPR is a valid
                    // repr with the same size and alignment as the type.
                    write!(f, "{cfg_encoding}")?;
                    writeln!(f, "{}", unsafe_impl_encode(&id.name, ty.enum_encoding()))?;
                    write!(f, "{cfg_encoding}")?;
                    writeln!(f, "{}", unsafe_impl_refencode(&id.name))?;

                    if let Some(true) = sendable {
                        writeln!(f)?;
                        write!(f, "{}", self.cfg_gate_ln(config))?;
                        writeln!(f, "unsafe impl Send for {} {{}}", id.name)?;

                        writeln!(f)?;
                        write!(f, "{}", self.cfg_gate_ln(config))?;
                        writeln!(f, "unsafe impl Sync for {} {{}}", id.name)?;
                    }
                }
                Self::ConstDecl {
                    id,
                    availability,
                    ty,
                    value,
                    is_last,
                    documentation,
                } => {
                    write!(f, "{}", documentation.fmt(Some(id)))?;
                    write!(f, "{}", self.cfg_gate_ln(config))?;
                    write!(f, "{availability}")?;
                    write!(f, "pub const {}: {} = {value};", id.name, ty.const_())?;
                    if *is_last {
                        writeln!(f)?;
                    }
                }
                Self::VarDecl {
                    id,
                    link_name,
                    availability,
                    ty,
                    value: None,
                    documentation,
                } => {
                    writeln!(f, "extern \"C\" {{")?;
                    write!(f, "{}", documentation.fmt(Some(id)))?;
                    write!(f, "{}", self.cfg_gate_ln(config))?;
                    write!(f, "{availability}")?;
                    if *link_name != id.name {
                        writeln!(f, "#[link_name = {link_name:?}]")?;
                    }
                    writeln!(f, "    pub static {}: {};", id.name, ty.var())?;
                    writeln!(f, "}}")?;
                }
                Self::VarDecl {
                    id,
                    link_name: _, // Don't care about the link name on variables with a value.
                    availability,
                    ty,
                    value: Some(expr),
                    documentation,
                } => {
                    write!(f, "{}", documentation.fmt(Some(id)))?;
                    write!(f, "{}", self.cfg_gate_ln(config))?;
                    write!(f, "{availability}")?;
                    write!(f, "pub static {}: {} = ", id.name, ty.var())?;

                    if ty.is_floating_through_typedef() {
                        write!(f, "{expr} as _")?;
                    } else if ty.is_enum_through_typedef() {
                        write!(f, "{}({expr})", ty.var())?;
                    } else {
                        write!(f, "{expr}")?;
                    }
                    writeln!(f, ";")?;
                }
                Self::FnDecl {
                    c_name,
                    arguments,
                    result_type,
                    body: Some(_),
                    returns_retained,
                    ..
                } => {
                    write!(f, "// TODO: ")?;
                    write!(f, "pub fn {c_name}(")?;
                    for (param, arg_ty) in arguments {
                        let param = handle_reserved(&crate::to_snake_case(param));
                        write!(f, "{param}: {},", arg_ty.fn_argument())?;
                    }
                    let (ret, _) = result_type.fn_return(*returns_retained);
                    writeln!(f, "){ret};")?;
                }
                Self::FnDecl {
                    id,
                    c_name,
                    link_name,
                    availability,
                    arguments,
                    first_arg_is_self,
                    result_type,
                    body: None,
                    safe,
                    must_use,
                    abi,
                    returns_retained,
                    documentation,
                    no_implementor: _,
                    custom_implementor: _,
                } => {
                    let (ret, return_converter) = result_type.fn_return(*returns_retained);

                    let needs_wrapper = *safe
                        || return_converter.is_some()
                        || arguments
                            .iter()
                            .any(|(_, arg)| arg.fn_argument_converter().is_some())
                        || abi.rust_outer();

                    let raw_fn_decl = |f: &mut fmt::Formatter<'_>, vis| {
                        if c_name != link_name {
                            if id.library_name() == "Dispatch" {
                                // HACK: Currently only used in libdispatch on Apple targets.
                                writeln!(f, "#[cfg_attr(target_vendor = \"apple\", link_name = {link_name:?})]")?;
                            } else if link_name.contains("$LEGACYMAC") {
                                // HACK: Security needs this only on macOS.
                                writeln!(
                                    f,
                                    "#[cfg_attr(target_os = \"macos\", link_name = {link_name:?})]"
                                )?;
                            } else {
                                writeln!(f, "#[link_name = {link_name:?}]")?;
                            }
                        }

                        write!(f, "{vis}fn {c_name}(")?;
                        for (param, arg_ty) in arguments {
                            let param = handle_reserved(&crate::to_snake_case(param));
                            write!(f, "{param}: {},", arg_ty.fn_argument())?;
                        }
                        writeln!(f, "){ret};")?;

                        Ok(())
                    };

                    let vis = if id.name.starts_with("_") {
                        "pub(crate)"
                    } else {
                        "pub"
                    };

                    if needs_wrapper {
                        write!(f, "{}", documentation.fmt(None))?;
                        write!(f, "{}", self.cfg_gate_ln(config))?;
                        write!(f, "{availability}")?;
                        if *must_use {
                            writeln!(f, "#[must_use]")?;
                        }
                        writeln!(f, "#[inline]")?;
                        let unsafe_ = if *safe { "" } else { "unsafe " };
                        let fn_name = handle_reserved(&id.name);
                        write!(f, "{vis} {unsafe_}{}fn {fn_name}(", abi.extern_outer())?;
                        for (i, (param, arg_ty)) in arguments.iter().enumerate() {
                            if i == 0 && *first_arg_is_self {
                                // Might not be completely correct, but typeck
                                // should figure out for us when calling the
                                // inner function if the argument type isn't
                                // the same type this one.
                                if arg_ty.is_object_like_ptr() {
                                    write!(f, "&self")?;
                                } else {
                                    write!(f, "self")?;
                                }
                            } else {
                                let param = handle_reserved(&crate::to_snake_case(param));
                                write!(f, "{param}: ")?;

                                if let Some((converted_ty, _, _)) = arg_ty.fn_argument_converter() {
                                    write!(f, "{converted_ty}")?;
                                } else {
                                    write!(f, "{}", arg_ty.fn_argument())?;
                                }
                            }
                            write!(f, ",")?;
                        }
                        write!(f, ")")?;
                        if let Some((ty, _, _)) = &return_converter {
                            write!(f, "{ty}")?;
                        } else {
                            write!(f, "{ret}")?;
                        }
                        writeln!(f, " {{")?;

                        // Emit raw
                        writeln!(f, "    {}{{", abi.extern_inner())?;
                        raw_fn_decl(f, "")?;
                        writeln!(f, "    }}")?;

                        // Call raw
                        write!(f, "    ")?;
                        if let Some((_, converter_start, _)) = &return_converter {
                            write!(f, "{converter_start}")?;
                        }
                        write!(f, "unsafe {{ {c_name}(")?;
                        for (i, (param, ty)) in arguments.iter().enumerate() {
                            let param = if i == 0 && *first_arg_is_self {
                                "self".to_string()
                            } else {
                                handle_reserved(&crate::to_snake_case(param))
                            };
                            if let Some((_, converter_start, converter_end)) =
                                ty.fn_argument_converter()
                            {
                                write!(f, "{converter_start}{param}{converter_end}")?;
                            } else {
                                write!(f, "{param}")?;
                            }
                            write!(f, ",")?;
                        }
                        write!(f, ") }}")?;
                        if let Some((_, _, converter_end)) = &return_converter {
                            write!(f, "{converter_end}")?;
                        }
                        writeln!(f)?;

                        writeln!(f, "}}")?;
                    } else {
                        writeln!(f, "{}{{", abi.extern_outer())?;

                        write!(f, "{}", documentation.fmt(None))?;
                        write!(f, "    {}", self.cfg_gate_ln(config))?;
                        write!(f, "    {availability}")?;
                        if *must_use {
                            writeln!(f, "    #[must_use]")?;
                        }

                        raw_fn_decl(f, &format!("{vis} "))?;

                        writeln!(f, "}}")?;
                    }
                }
                Self::FnGetTypeId {
                    id,
                    cf_item,
                    link_name,
                    result_type,
                    availability: _, // #[deprecated] is useless on traits.
                    abi,
                    documentation,
                } => {
                    let (ret, _) = result_type.fn_return(false);

                    // Only emit for base types, not for mutable subclasses,
                    // as it's unclear whether it's safe to downcast to
                    // mutable subclasses. Similarly, we only implement this
                    // for the base generic type (`CFArray<c_void>`).
                    write!(f, "{}", self.cfg_gate_ln(config))?;
                    writeln!(f, "unsafe impl ConcreteType for {} {{", cf_item.id().path())?;

                    write!(f, "{}", documentation.fmt(None))?;
                    writeln!(f, "    #[inline]")?;
                    writeln!(f, "    {}fn {}(){ret} {{", abi.extern_outer(), id.name)?;

                    writeln!(f, "        {}{{", abi.extern_inner())?;
                    writeln!(f, "            fn {link_name}(){ret};")?;
                    writeln!(f, "        }}")?;

                    writeln!(f, "        unsafe {{ {link_name}() }}")?;

                    writeln!(f, "    }}")?;
                    writeln!(f, "}}")?;
                    return Ok(());
                }
                Self::AliasDecl {
                    id,
                    availability,
                    ty,
                    kind,
                    documentation,
                } => {
                    write!(f, "{}", documentation.fmt(Some(id)))?;
                    write!(f, "{availability}")?;
                    match kind {
                        Some(UnexposedAttr::TypedEnum) => {
                            // TODO: Handle this differently
                            writeln!(f, "// NS_TYPED_ENUM")?;
                            write!(f, "{}", self.cfg_gate_ln(config))?;
                            writeln!(f, "pub type {} = {};", id.name, ty.typedef())?;
                        }
                        Some(UnexposedAttr::TypedExtensibleEnum) => {
                            // TODO: Handle this differently
                            writeln!(f, "// NS_TYPED_EXTENSIBLE_ENUM")?;
                            write!(f, "{}", self.cfg_gate_ln(config))?;
                            writeln!(f, "pub type {} = {};", id.name, ty.typedef())?;
                        }
                        kind => {
                            if !matches!(
                                kind,
                                None | Some(
                                    UnexposedAttr::BridgedTypedef | UnexposedAttr::Bridged(_)
                                )
                            ) {
                                error!("invalid alias kind {kind:?} for {ty:?}");
                            }
                            // "bridged" typedefs should use a normal type alias.
                            write!(f, "{}", self.cfg_gate_ln(config))?;
                            writeln!(f, "pub type {} = {};", id.name, ty.typedef())?;
                        }
                    }
                }
                Self::OpaqueDecl {
                    id,
                    generics,
                    encoding_name,
                    availability,
                    documentation,
                    is_cf,
                    sendable,
                    bridged: _,
                    superclass,
                } => {
                    write!(f, "{}", documentation.fmt(Some(id)))?;
                    write!(f, "{}", self.cfg_gate_ln(config))?;
                    write!(f, "{availability}")?;
                    writeln!(f, "#[repr(C)]")?;
                    if !*is_cf {
                        // To avoid warnings, though mostly useless.
                        writeln!(f, "#[derive(Debug)]")?;
                    }
                    // Default generics to `Opaque` helper type.
                    writeln!(
                        f,
                        "pub struct {}{} {{",
                        id.name,
                        GenericParamsHelper(generics, "?Sized = Opaque")
                    )?;
                    // Make the type be considered FFI-safe.
                    writeln!(f, "    inner: [u8; 0],")?;
                    // Same as objc2::ffi::OpaqueData, almost equivalent to using `extern type`.
                    writeln!(
                        f,
                        "    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,"
                    )?;
                    if !generics.is_empty() {
                        write!(f, "    _generics: PhantomData<(")?;
                        for (generic, _) in generics {
                            // Make generics invariant (like in `objc2::extern_class!`).
                            write!(f, "*mut {generic}, ")?;
                        }
                        writeln!(f, ")>,")?;
                    }
                    writeln!(f, "}}")?;

                    writeln!(f)?;

                    // Similar to the output of extern_class!
                    if *is_cf {
                        let required_items = self
                            .required_items()
                            .chain(iter::once(ItemTree::cf("cf_type")));
                        let cfg_cf = self.cfg_gate_ln_for(required_items, config);
                        write!(f, "{cfg_cf}")?;
                        // SAFETY: The type is a CoreFoundation type, and
                        // correctly declared as a #[repr(C)] ZST.
                        writeln!(f, "cf_type!(")?;

                        // SAFETY: It's fine to implement helper traits for
                        // all generics (e.g. all of `CFArray<u32>`,
                        // `CFArray<CFString>` and `CFArray<Box<String>>`),
                        // since unlike e.g. `NSArray<u32>`, these are all
                        // perfectly valid types (their correct construction
                        // is controlled with CFArrayCallBacks).
                        write!(
                            f,
                            "    unsafe impl{} {}{}",
                            GenericParamsHelper(generics, "?Sized"),
                            id.name,
                            generic_ty(generics),
                        )?;
                        if let Some(superclass) = superclass {
                            write!(f, ": {}{}", superclass.name, generic_ty(generics))?;
                        }
                        writeln!(f, " {{}}")?;
                        writeln!(f, ");")?;

                        let required_items = self
                            .required_items()
                            .chain(iter::once(ItemTree::objc("cf_objc2_type")));
                        let cfg_objc2 = self.cfg_gate_ln_for(required_items, config);
                        write!(f, "{cfg_objc2}")?;
                        writeln!(f, "cf_objc2_type!(")?;
                        // SAFETY: The type is a CoreFoundation type.
                        writeln!(
                            f,
                            "    unsafe impl{} RefEncode<{encoding_name:?}> for {}{} {{}}",
                            GenericParamsHelper(generics, "?Sized"),
                            id.name,
                            generic_ty(generics),
                        )?;
                        writeln!(f, ");")?;
                    } else {
                        let required_items = self
                            .required_items()
                            .chain(iter::once(ItemTree::objc("Encoding")));
                        let cfg_encoding = self.cfg_gate_ln_for(required_items, config);
                        // SAFETY: The struct is a ZST type marked `#[repr(C)]`.
                        write!(f, "{cfg_encoding}")?;
                        writeln!(
                            f,
                            "unsafe impl{} RefEncode for {}{} {{",
                            GenericParamsHelper(generics, "UnknownBound"),
                            id.name,
                            generic_ty(generics)
                        )?;
                        write!(f, "    const ENCODING_REF: Encoding = ")?;
                        writeln!(f, "Encoding::Pointer(&Encoding::Struct(")?;
                        writeln!(f, "        {encoding_name:?},")?;
                        writeln!(f, "        &[],")?;
                        writeln!(f, "    ));")?;
                        writeln!(f, "}}")?;
                    }

                    if let Some(true) = sendable {
                        writeln!(f)?;
                        write!(f, "{}", self.cfg_gate_ln(config))?;
                        writeln!(f, "unsafe impl Send for {} {{}}", id.name)?;

                        writeln!(f)?;
                        write!(f, "{}", self.cfg_gate_ln(config))?;
                        writeln!(f, "unsafe impl Sync for {} {{}}", id.name)?;
                    }

                    // Add casting from `CFArray<T>` to `CFArray<U>`.
                    if !generics.is_empty() {
                        writeln!(f)?;
                        write!(f, "{}", self.cfg_gate_ln(config))?;
                        add_generic_cast_helpers(f, id, generics, true)?;
                    }
                }
                Self::GeneralImpl {
                    location: _,
                    item,
                    stmts,
                } => {
                    write!(f, "{}", self.cfg_gate_ln(config))?;
                    writeln!(f, "impl {} {{", item.id().path())?;
                    for stmt in stmts {
                        writeln!(f, "{}", stmt.fmt(config))?;
                    }
                    writeln!(f, "}}")?;
                }
            };
            Ok(())
        })
    }

    pub(crate) fn encoding_test<'a>(&'a self, config: &'a Config) -> Option<impl Display + 'a> {
        let (data, availability, cls, cls_generics, methods) = match self {
            Stmt::ExternMethods {
                location,
                availability,
                cls,
                cls_generics,
                methods,
                ..
            } => (
                config.library(location),
                availability,
                cls,
                &**cls_generics,
                methods,
            ),
            Stmt::ExternCategory {
                id,
                availability,
                cls,
                cls_generics,
                methods,
                ..
            } => (
                config.library(id),
                availability,
                cls,
                &**cls_generics,
                methods,
            ),
            // TODO: Test protocols too
            _ => return None,
        };

        // Availability is broken in MDLUtility.h for some reason?
        if cls.name == "MDLUtility" {
            return None;
        }

        Some(FormatterFn(move |f| {
            write!(
                f,
                "{}",
                simple_platform_gate(data, self.required_items(), [], config)
            )?;
            if let Some(check) = availability.check_is_available() {
                writeln!(f, "    if {check} ")?;
            }
            writeln!(f, "    {{")?;
            for (generic, _) in cls_generics {
                writeln!(f, "        type {generic} = AnyObject;")?;
            }
            write!(f, "        type This = {}<", cls.path())?;
            for (generic, _) in cls_generics {
                write!(f, "{generic}, ")?;
            }
            writeln!(f, ">;")?;
            writeln!(f, "        let cls = This::class();")?;
            writeln!(f, "        let metaclass = cls.metaclass();")?;

            for method in methods {
                write!(
                    f,
                    "{}",
                    simple_platform_gate(
                        data,
                        method.required_items(),
                        self.required_items(),
                        config
                    )
                )?;
                write!(f, "{}", method.encoding_test(false))?;
            }

            writeln!(f, "    }}")?;

            Ok(())
        }))
    }

    pub(crate) fn static_test<'a>(&'a self, config: &'a Config) -> Option<impl Display + 'a> {
        match self {
            Self::VarDecl {
                id,
                availability,
                ty,
                value: None,
                ..
            } => {
                // Certain statics are only set when the MIDI server is
                // running / when disk arbitration has been initialized.
                if ["CoreMIDI", "DiskArbitration"].contains(&id.library_name()) {
                    return None;
                }
                // Some dispatch statics are not exposed.
                if ["Dispatch"].contains(&id.library_name()) {
                    return None;
                }
                // Some statics are missing / have wrong availability attributes.
                if [
                    "CBUUIDCharacteristicObservationScheduleString",
                    "kMDItemXMPCredit",
                    "kMDItemXMPDigitalSourceType",
                    "HKDataTypeIdentifierStateOfMind",
                ]
                .contains(&&*id.name)
                {
                    return None;
                }
                if !availability.is_available_host() {
                    return None;
                }
                Some(FormatterFn(move |f| {
                    write!(
                        f,
                        "{}",
                        simple_platform_gate(config.library(id), self.required_items(), [], config,)
                    )?;
                    let ty = ty.var().to_string();
                    if ty.starts_with("&") {
                        writeln!(f, "    check_static_nonnull(unsafe {{ {} }});", id.path())?;
                    } else if ty.starts_with("Option<&") {
                        writeln!(f, "    let _ = unsafe {{ {} }};", id.path())?;
                    } else {
                        writeln!(f, "    let _ = unsafe {{ &{} }};", id.path())?;
                    }

                    Ok(())
                }))
            }
            _ => None,
        }
    }
}

struct GenericTyHelper<I>(I);

fn generic_ty<'a, I: IntoIterator<Item = &'a GenericWithBound>>(
    iter: I,
) -> GenericTyHelper<impl IntoIterator<Item = &'a String> + Clone>
where
    I::IntoIter: Clone,
{
    GenericTyHelper(iter.into_iter().map(|(generic, _bound)| generic))
}

impl<I: IntoIterator + Clone> fmt::Display for GenericTyHelper<I>
where
    I::Item: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.0.clone().into_iter();
        if let Some(first) = iter.next() {
            write!(f, "<{first}")?;
            for generic in iter {
                write!(f, ", {generic}")?;
            }
            write!(f, ">")?;
        }
        Ok(())
    }
}

struct GenericParamsHelper<'a, I: IntoIterator<Item = &'a GenericWithBound> + Clone>(I, &'a str);

impl<'a, I: IntoIterator<Item = &'a GenericWithBound> + Clone> fmt::Display
    for GenericParamsHelper<'a, I>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.0.clone().into_iter();
        if let Some((first, bound)) = iter.next() {
            write!(f, "<{first}: {}", self.1)?;
            if let Some(bound) = bound {
                write!(f, " + {}", bound.generic_bound())?;
            }
            for (generic, bound) in iter {
                write!(f, ", {generic}: {}", self.1)?;
                if let Some(bound) = bound {
                    write!(f, " + {}", bound.generic_bound())?;
                }
            }
            write!(f, ">")?;
        }
        Ok(())
    }
}

/// Add `.cast_unchecked()` and `.as_opaque()` methods for converting a
/// type's generic parameters.
///
/// Note: Ideally, this would probably be done with safe_transmute or smth.
fn add_generic_cast_helpers(
    f: &mut fmt::Formatter<'_>,
    id: &ItemIdentifier,
    generics: &[GenericWithBound],
    cf: bool,
) -> fmt::Result {
    let s = if generics.len() == 1 { "" } else { "s" };
    // Using just `?Sized` is intentional here, we have no
    // bound on `T: Type` (CF collections can contain
    // non-object types).
    let bound = if cf { "?Sized" } else { "?Sized + Message" };
    let casted_generics: Vec<_> = generics
        .iter()
        .map(|(generic, bound)| (format!("New{generic}"), bound.clone()))
        .collect();

    writeln!(
        f,
        "impl{} {}{} {{",
        GenericParamsHelper(generics, bound),
        id.path(),
        generic_ty(generics),
    )?;
    writeln!(
        f,
        "    /// Unchecked conversion of the generic parameter{s}."
    )?;
    writeln!(f, "    ///")?;
    writeln!(f, "    /// # Safety")?;
    writeln!(f, "    ///")?;
    writeln!(
        f,
        "    /// The generic{s} must be valid to reinterpret as the given type{s}."
    )?;
    writeln!(f, "    #[inline]")?;
    writeln!(
        f,
        "    pub unsafe fn cast_unchecked{}(&self) -> &{}{} {{",
        GenericParamsHelper(&casted_generics, bound),
        id.path(),
        generic_ty(&casted_generics),
    )?;
    // SAFETY: Upheld by the caller.
    writeln!(f, "        unsafe {{ &*((self as *const Self).cast()) }}",)?;
    writeln!(f, "    }}")?;

    // Add `as_opaque`.
    if cf {
        writeln!(f)?;
        writeln!(f, "    /// Convert to the opaque/untyped variant.")?;
        writeln!(f, "    #[inline]")?;
        writeln!(f, "    pub fn as_opaque(&self) -> &{} {{", id.path())?;
        // SAFETY: CF collections store objects behind a reference, and can
        // all be represented by `objc2_core_foundation::opaque::Opaque`.
        //
        // For invariant collections like `CFMutableArray`, while it would
        // not normally be safe to add upcasts like this, it should be safe
        // for the `Opaque` type, since there are no safe way to neither get
        // nor set that (and thus we cannot do the otherwise dangerous
        // `CFMutableArray<NSString> -> CFMutableArray<NSObject> -> set`).
        writeln!(f, "        unsafe {{ self.cast_unchecked() }}",)?;
        writeln!(f, "    }}")?;
    }

    writeln!(f, "}}")?;

    Ok(())
}

fn simple_platform_gate(
    data: &LibraryConfig,
    required_items: impl IntoIterator<Item = ItemTree>,
    implied_items: impl IntoIterator<Item = ItemTree>,
    config: &Config,
) -> impl Display {
    let mut platform_cfg = PlatformCfg::from_config(data);

    for item in required_items {
        platform_cfg.dependency(config.library(item.id()));
    }

    for item in implied_items {
        platform_cfg.implied(config.library(item.id()));
    }

    FormatterFn(move |f| {
        if let Some(cfg) = platform_cfg.cfgs() {
            writeln!(f, "#[cfg({cfg})]")?;
        }

        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_superclasses_required_items() {
        let superclasses = [
            ItemIdentifier::dummy(1),
            ItemIdentifier::dummy(2),
            ItemIdentifier::dummy(3),
        ];
        let required_items = [
            ItemTree::new(
                ItemIdentifier::dummy(1),
                [
                    ItemTree::new(
                        ItemIdentifier::dummy(2),
                        [
                            ItemTree::new(ItemIdentifier::dummy(3), [ItemTree::objc("__macros__")]),
                            ItemTree::objc("__macros__"),
                        ],
                    ),
                    ItemTree::new(ItemIdentifier::dummy(3), [ItemTree::objc("__macros__")]),
                    ItemTree::objc("__macros__"),
                ],
            ),
            ItemTree::new(
                ItemIdentifier::dummy(2),
                [
                    ItemTree::new(ItemIdentifier::dummy(3), [ItemTree::objc("__macros__")]),
                    ItemTree::objc("__macros__"),
                ],
            ),
            ItemTree::new(ItemIdentifier::dummy(3), [ItemTree::objc("__macros__")]),
            ItemTree::objc("__macros__"),
        ];
        assert_eq!(
            superclasses_required_items(superclasses).collect::<Vec<_>>(),
            required_items
        );
    }
}
