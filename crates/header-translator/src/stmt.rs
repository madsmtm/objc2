use std::borrow::Cow;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::fmt;
use std::iter;
use std::mem;

use clang::{Entity, EntityKind, EntityVisitResult};

use crate::availability::Availability;
use crate::config::{ClassData, MethodData};
use crate::context::Context;
use crate::expr::Expr;
use crate::feature::Feature;
use crate::feature::Features;
use crate::id::ItemIdentifier;
use crate::immediate_children;
use crate::method::{handle_reserved, Method};
use crate::rust_type::Ty;
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
fn parse_protocols(
    entity: &Entity<'_>,
    protocols: &mut BTreeSet<ItemIdentifier>,
    context: &Context<'_>,
) {
    immediate_children(entity, |entity, _span| match entity.get_kind() {
        EntityKind::ObjCProtocolRef => {
            let entity = entity
                .get_reference()
                .expect("ObjCProtocolRef to reference entity");
            if protocols.insert(
                ItemIdentifier::new(&entity, context)
                    .map_name(|name| context.replace_protocol_name(name)),
            ) {
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

fn parse_direct_protocols(entity: &Entity<'_>, context: &Context<'_>) -> BTreeSet<ItemIdentifier> {
    let mut protocols = BTreeSet::new();

    #[allow(clippy::single_match)]
    immediate_children(entity, |entity, _span| match entity.get_kind() {
        EntityKind::ObjCProtocolRef => {
            let entity = entity
                .get_reference()
                .expect("ObjCProtocolRef to reference entity");
            protocols.insert(
                ItemIdentifier::new(&entity, context)
                    .map_name(|name| context.replace_protocol_name(name)),
            );
        }
        _ => {}
    });

    protocols
}

fn parse_superclasses<'ty>(
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

fn parse_class_generics(entity: &Entity<'_>, _context: &Context<'_>) -> Vec<String> {
    let mut generics = Vec::new();

    #[allow(clippy::single_match)]
    immediate_children(entity, |entity, _span| match entity.get_kind() {
        EntityKind::TemplateTypeParameter => {
            // TODO: Generics with bounds (like NSMeasurement<UnitType: NSUnit *>)
            // let ty = entity.get_type().expect("template type");
            let name = entity.get_name().expect("template name");
            generics.push(name);
        }
        _ => {}
    });

    generics
}

fn parse_attributes(entity: &Entity<'_>, context: &Context<'_>) -> (Option<bool>, bool) {
    let mut sendable = None;
    let mut mainthreadonly = false;

    immediate_children(entity, |entity, _span| {
        if let EntityKind::UnexposedAttr = entity.get_kind() {
            if let Some(attr) = UnexposedAttr::parse(&entity, context) {
                match attr {
                    UnexposedAttr::Sendable => sendable = Some(true),
                    UnexposedAttr::NonSendable => sendable = Some(false),
                    UnexposedAttr::UIActor => {
                        sendable = Some(false);
                        mainthreadonly = true;
                    }
                    attr => error!(?attr, "unknown attribute"),
                }
            }
        }
    });

    (sendable, mainthreadonly)
}

fn parse_methods(
    entity: &Entity<'_>,
    get_data: impl Fn(&str) -> MethodData,
    is_mutable: bool,
    is_pub: bool,
    implied_features: &[ItemIdentifier],
    context: &Context<'_>,
) -> (Vec<Method>, Vec<String>) {
    let mut methods = Vec::new();
    let mut designated_initializers = Vec::new();

    // Track seen properties, so that when methods are autogenerated by the
    // compiler from them, we can skip them
    let mut properties = HashSet::new();

    immediate_children(entity, |entity, span| match entity.get_kind() {
        EntityKind::ObjCInstanceMethodDecl | EntityKind::ObjCClassMethodDecl => {
            drop(span);
            let partial = Method::partial(entity);

            if !properties.remove(&(partial.is_class, partial.selector.clone())) {
                let data = get_data(&partial.selector);
                if let Some((designated_initializer, method)) =
                    partial.parse(data, is_mutable, is_pub, implied_features, context)
                {
                    if designated_initializer {
                        designated_initializers.push(method.selector.clone());
                    }
                    methods.push(method);
                }
            }
        }
        EntityKind::ObjCPropertyDecl => {
            drop(span);
            let partial = Method::partial_property(entity);

            // TODO: Use `get_overridden_methods` to deduplicate property
            // getters (when declared on both immutable and mutable class).

            let getter_data = get_data(&partial.getter_sel);
            let setter_data = partial
                .setter_sel
                .as_ref()
                .map(|setter_sel| get_data(setter_sel));

            let (getter, setter) = partial.parse(
                getter_data,
                setter_data,
                is_mutable,
                is_pub,
                implied_features,
                context,
            );
            if let Some(getter) = getter {
                if !properties.insert((getter.is_class, getter.selector.clone())) {
                    error!(?setter, "already exisiting property");
                }
                methods.push(getter);
            }
            if let Some(setter) = setter {
                if !properties.insert((setter.is_class, setter.selector.clone())) {
                    error!(?setter, "already exisiting property");
                }
                methods.push(setter);
            }
        }
        _ => {}
    });

    if !properties.is_empty() {
        error!(
            ?methods,
            ?properties,
            "did not properly add methods to properties"
        );
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
                // Parsed in parse_methods
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
            (_, parent_kind) => error!(?parent_kind, "unknown in parent"),
        }
    });
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum Mutability {
    Immutable,
    Mutable,
    ImmutableWithMutableSubclass(ItemIdentifier),
    MutableWithImmutableSuperclass(ItemIdentifier),
    #[default]
    InteriorMutable,
    MainThreadOnly,
}

impl Mutability {
    pub fn is_mutable(&self) -> bool {
        matches!(
            self,
            Mutability::Mutable | Mutability::MutableWithImmutableSuperclass(_)
        )
    }
}

impl fmt::Display for Mutability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Immutable => write!(f, "Immutable"),
            Self::Mutable => write!(f, "Mutable"),
            Self::ImmutableWithMutableSubclass(subclass) => {
                write!(f, "ImmutableWithMutableSubclass<{}>", subclass.path())
            }
            Self::MutableWithImmutableSuperclass(superclass) => {
                write!(f, "MutableWithImmutableSuperclass<{}>", superclass.path())
            }
            Self::InteriorMutable => write!(f, "InteriorMutable"),
            Self::MainThreadOnly => write!(f, "MainThreadOnly"),
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
        generics: Vec<String>,
        availability: Availability,
        superclasses: Vec<(ItemIdentifier, Vec<String>)>,
        designated_initializers: Vec<String>,
        derives: Derives,
        mutability: Mutability,
        skipped: bool,
        sendable: bool,
    },
    /// @interface class_name (category_name) <protocols*>
    /// ->
    /// extern_methods!
    ExternMethods {
        availability: Availability,
        cls: ItemIdentifier,
        source_superclass: Option<ItemIdentifier>,
        cls_generics: Vec<String>,
        category_name: Option<String>,
        methods: Vec<Method>,
    },
    /// @interface class_name (category_name) <protocols*>
    /// ->
    /// extern_category!
    ExternCategory {
        id: ItemIdentifier,
        actual_name: Option<String>,
        availability: Availability,
        cls: ItemIdentifier,
        methods: Vec<Method>,
    },
    /// @protocol name <protocols*>
    /// ->
    /// extern_protocol!
    ProtocolDecl {
        id: ItemIdentifier,
        actual_name: Option<String>,
        availability: Availability,
        protocols: BTreeSet<ItemIdentifier>,
        methods: Vec<Method>,
        required_sendable: bool,
        required_mainthreadonly: bool,
    },
    /// @interface ty: _ <protocols*>
    /// @interface ty (_) <protocols*>
    ProtocolImpl {
        cls: ItemIdentifier,
        protocol: ItemIdentifier,
        generics: Vec<String>,
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
    StructDecl {
        id: ItemIdentifier,
        // internal objc struct name (before typedef). shows up in encoding
        // and is used in message verification.
        encoding_name: Option<String>,
        availability: Availability,
        boxable: bool,
        fields: Vec<(String, Ty)>,
        sendable: Option<bool>,
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
        variants: Vec<(String, Availability, Expr)>,
        sendable: Option<bool>,
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
    },
    /// static const ty name = expr;
    /// extern const ty name;
    VarDecl {
        id: ItemIdentifier,
        availability: Availability,
        ty: Ty,
        value: Option<Expr>,
    },
    /// extern ret name(args*);
    ///
    /// static inline ret name(args*) {
    ///     body
    /// }
    FnDecl {
        id: ItemIdentifier,
        availability: Availability,
        arguments: Vec<(String, Ty)>,
        result_type: Ty,
        // Some -> inline function.
        body: Option<()>,
        safe: bool,
    },
    /// typedef Type TypedefName;
    AliasDecl {
        id: ItemIdentifier,
        availability: Availability,
        ty: Ty,
        kind: Option<UnexposedAttr>,
    },
}

fn parse_fn_param_children(entity: &Entity<'_>, context: &Context<'_>) {
    immediate_children(entity, |entity, _span| match entity.get_kind() {
        EntityKind::UnexposedAttr => {
            if let Some(attr) = UnexposedAttr::parse(&entity, context) {
                error!(?attr, "unknown attribute");
            }
        }
        EntityKind::ObjCClassRef | EntityKind::TypeRef | EntityKind::ObjCProtocolRef => {}
        EntityKind::NSConsumed => {
            error!("found NSConsumed, which requires manual handling");
        }
        kind => error!(?kind, "unknown"),
    });
}

fn get_class_implied_features(cls: &Entity<'_>, context: &Context<'_>) -> Vec<ItemIdentifier> {
    // The feature is guaranteed enabled if the class
    // itself is enabled.
    let mut implied_features = vec![ItemIdentifier::new(cls, context)];

    // Same for superclasses.
    for (id, _, _) in parse_superclasses(cls, context) {
        implied_features.push(id);
    }

    implied_features
}

pub(crate) fn new_enum_id(
    entity: &Entity<'_>,
    context: &Context<'_>,
) -> ItemIdentifier<Option<String>> {
    assert_eq!(entity.get_kind(), EntityKind::EnumDecl);
    let mut id = ItemIdentifier::new_optional(entity, context);

    if id
        .name
        .as_deref()
        .map(|name| name.starts_with("enum (unnamed at"))
        .unwrap_or(false)
    {
        id.name = None;
    }

    id
}

impl Stmt {
    pub fn parse(entity: &Entity<'_>, context: &Context<'_>) -> Vec<Self> {
        let _span = debug_span!(
            "stmt",
            kind = ?entity.get_kind(),
            dbg = entity.get_name(),
        )
        .entered();

        match entity.get_kind() {
            // These are inconsequential for us, since we resolve imports differently
            EntityKind::ObjCClassRef | EntityKind::ObjCProtocolRef => vec![],
            EntityKind::ObjCInterfaceDecl => {
                // entity.get_mangled_objc_names()
                let id = ItemIdentifier::new(entity, context);
                let data = context.class_data.get(&id.name);

                if data.map(|data| data.skipped).unwrap_or_default() {
                    return vec![];
                }

                let availability = Availability::parse(entity, context);

                let implied_features = get_class_implied_features(entity, context);

                verify_objc_decl(entity, context);
                let generics = parse_class_generics(entity, context);
                let (methods, designated_initializers) = parse_methods(
                    entity,
                    |name| ClassData::get_method_data(data, name),
                    data.map(|data| data.mutability.is_mutable())
                        .unwrap_or(false),
                    true,
                    &implied_features,
                    context,
                );

                let (sendable, mut mainthreadonly) = parse_attributes(entity, context);

                let mut protocols = Default::default();
                parse_protocols(entity, &mut protocols, context);

                let skipped_protocols = data
                    .map(|data| data.skipped_protocols.clone())
                    .unwrap_or_default();
                protocols.retain(|protocol| !skipped_protocols.contains(&protocol.name));

                let superclasses_full = parse_superclasses(entity, context);

                let superclasses: Vec<_> = superclasses_full
                    .iter()
                    .map(|(id, generics, entity)| {
                        // Ignore sendability on superclasses; because it's an auto trait, it's propagated to subclasses anyhow!
                        let (_sendable, superclass_mainthreadonly) =
                            parse_attributes(entity, context);

                        if superclass_mainthreadonly {
                            mainthreadonly = true;
                        }

                        (id.clone(), generics.clone())
                    })
                    .collect();

                // Used for duplicate checking (sometimes the subclass
                // defines the same method that the superclass did).
                let mut seen_methods: BTreeSet<_> =
                    methods.iter().map(|method| method.id()).collect();

                let superclass_methods: Vec<_> = superclasses_full
                    .iter()
                    .filter_map(|(superclass_id, _, entity)| {
                        let superclass_data = context.class_data.get(&superclass_id.name);

                        // Explicitly keep going, even if the class itself is skipped
                        // if superclass_data.skipped

                        let (mut methods, _) = parse_methods(
                            entity,
                            |name| {
                                let data = ClassData::get_method_data(data, name);
                                let superclass_data =
                                    ClassData::get_method_data(superclass_data, name);
                                data.merge_with_superclass(superclass_data)
                            },
                            data.map(|data| data.mutability.is_mutable())
                                .or(superclass_data.map(|data| data.mutability.is_mutable()))
                                .unwrap_or(false),
                            true,
                            // Even though the methods are originally defined
                            // elsewhere, we're going to be _emitting_ them on
                            // the current class, so that's also where we're
                            // taking our implied features from.
                            &implied_features,
                            context,
                        );
                        methods.retain(|method| {
                            method.emit_on_subclasses() && !seen_methods.contains(&method.id())
                        });
                        seen_methods.extend(methods.iter().map(|method| method.id()));
                        if methods.is_empty() {
                            None
                        } else {
                            Some(Self::ExternMethods {
                                availability: Availability::parse(entity, context),
                                cls: id.clone(),
                                source_superclass: Some(superclass_id.clone()),
                                cls_generics: generics.clone(),
                                category_name: None,
                                methods,
                            })
                        }
                    })
                    .collect();

                let methods = Self::ExternMethods {
                    availability: availability.clone(),
                    cls: id.clone(),
                    source_superclass: None,
                    cls_generics: generics.clone(),
                    category_name: None,
                    methods,
                };

                iter::once(Self::ClassDecl {
                    id: id.clone(),
                    generics: generics.clone(),
                    availability: availability.clone(),
                    superclasses,
                    designated_initializers,
                    derives: data.map(|data| data.derives.clone()).unwrap_or_default(),
                    mutability: if mainthreadonly {
                        Mutability::MainThreadOnly
                    } else {
                        data.map(|data| data.mutability.clone()).unwrap_or_default()
                    },
                    skipped: data.map(|data| data.definition_skipped).unwrap_or_default(),
                    sendable: sendable.unwrap_or(false),
                })
                .chain(protocols.into_iter().map(|protocol| Self::ProtocolImpl {
                    cls: id.clone(),
                    protocol,
                    generics: generics.clone(),
                    availability: availability.clone(),
                }))
                .chain(iter::once(methods))
                .chain(superclass_methods)
                .collect()
            }
            EntityKind::ObjCCategoryDecl => {
                let category = ItemIdentifier::new_optional(entity, context);
                let availability = Availability::parse(entity, context);

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
                let data = context.class_data.get(&cls.name);

                if data.map(|data| data.skipped).unwrap_or_default() {
                    return vec![];
                }

                if let Some(category_name) = &category.name {
                    let category_data = data
                        .and_then(|data| data.categories.get(category_name))
                        .cloned()
                        .unwrap_or_default();

                    if category_data.skipped {
                        return vec![];
                    }
                }

                verify_objc_decl(entity, context);
                let generics = parse_class_generics(entity, context);
                let mut protocols = parse_direct_protocols(entity, context);

                let skipped_protocols = data
                    .map(|data| data.skipped_protocols.clone())
                    .unwrap_or_default();
                protocols.retain(|protocol| !skipped_protocols.contains(&protocol.name));

                let protocol_impls = protocols.into_iter().map(|protocol| Self::ProtocolImpl {
                    cls: cls.clone(),
                    generics: generics.clone(),
                    availability: availability.clone(),
                    protocol,
                });

                let (sendable, mainthreadonly) = parse_attributes(entity, context);
                if let Some(sendable) = sendable {
                    error!(?sendable, "sendable on category");
                }
                if mainthreadonly {
                    error!("@UIActor on category");
                }

                // For ease-of-use, if the category is defined in the same
                // library as the class, we just emit it as `extern_methods!`.
                if cls.library == category.library {
                    // extern_methods!

                    let (methods, designated_initializers) = parse_methods(
                        entity,
                        |name| ClassData::get_method_data(data, name),
                        data.map(|data| data.mutability.is_mutable())
                            .unwrap_or(false),
                        true,
                        &get_class_implied_features(&cls_entity, context),
                        context,
                    );

                    if !designated_initializers.is_empty() {
                        warn!(
                            ?designated_initializers,
                            "designated initializer in category"
                        );
                    }

                    let extra_methods = if let Mutability::ImmutableWithMutableSubclass(subclass) =
                        data.map(|data| data.mutability.clone()).unwrap_or_default()
                    {
                        let subclass_data = context.class_data.get(&subclass.name);
                        assert!(!subclass_data.map(|data| data.skipped).unwrap_or_default());

                        let (mut methods, _) = parse_methods(
                            entity,
                            |name| {
                                let data = ClassData::get_method_data(data, name);
                                let subclass_data = ClassData::get_method_data(subclass_data, name);
                                subclass_data.merge_with_superclass(data)
                            },
                            data.map(|data| data.mutability.is_mutable())
                                .or(subclass_data.map(|data| data.mutability.is_mutable()))
                                .unwrap_or(false),
                            true,
                            &get_class_implied_features(&cls_entity, context),
                            context,
                        );
                        methods.retain(|method| method.emit_on_subclasses());
                        if methods.is_empty() {
                            None
                        } else {
                            Some(Self::ExternMethods {
                                source_superclass: Some(cls.clone()),
                                // Assume that immutable/mutable pairs have the
                                // same availability.
                                availability: availability.clone(),
                                cls: subclass,
                                // And that they have the same amount of generics.
                                cls_generics: generics.clone(),
                                category_name: category.name.clone(),
                                methods,
                            })
                        }
                    } else {
                        None
                    };

                    iter::once(Self::ExternMethods {
                        availability: availability.clone(),
                        cls: cls.clone(),
                        source_superclass: None,
                        cls_generics: generics.clone(),
                        category_name: category.name.clone(),
                        methods,
                    })
                    .chain(extra_methods)
                    .chain(protocol_impls)
                    .collect()
                } else {
                    // extern_category!

                    if !generics.is_empty() {
                        panic!("external category: cannot handle generics");
                    }

                    // Rough heuristic to determine category name.
                    //
                    // Note: There isn't really a good way to do this, as
                    // category names are not part of the public API in
                    // Objective-C.
                    let id = category.clone().map_name(|name| match name {
                        None => format!("{}Category", cls.name),
                        Some(name) => {
                            if name.contains(&cls.name)
                                || name.contains(&cls.name.replace("Mutable", ""))
                            {
                                name.clone()
                            } else {
                                format!("{}{}", cls.name, name)
                            }
                        }
                    });

                    let (methods, designated_initializers) = parse_methods(
                        entity,
                        |name| ClassData::get_method_data(data, name),
                        false,
                        false,
                        &get_class_implied_features(&cls_entity, context),
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
                            actual_name: category.name.clone(),
                            availability: availability.clone(),
                            cls: cls.clone(),
                            methods,
                        })
                    }
                    .into_iter()
                    .chain(protocol_impls)
                    .collect()
                }
            }
            EntityKind::ObjCProtocolDecl => {
                let actual_id = ItemIdentifier::new(entity, context);
                let data = context.protocol_data.get(&actual_id.name);
                let actual_name = data
                    .map(|data| data.renamed.is_some())
                    .unwrap_or_default()
                    .then(|| actual_id.name.clone());

                let id = actual_id.map_name(|name| context.replace_protocol_name(name));

                if data.map(|data| data.skipped).unwrap_or_default() {
                    return vec![];
                }

                let availability = Availability::parse(entity, context);

                verify_objc_decl(entity, context);
                let protocols = parse_direct_protocols(entity, context);
                let (methods, designated_initializers) = parse_methods(
                    entity,
                    |name| {
                        data.and_then(|data| data.methods.get(name))
                            .copied()
                            .unwrap_or_default()
                    },
                    false,
                    false,
                    &[],
                    context,
                );

                let (sendable, mut mainthreadonly) = parse_attributes(entity, context);

                if !designated_initializers.is_empty() {
                    warn!(
                        ?designated_initializers,
                        "designated initializer in protocol"
                    )
                }

                // Set the protocol as main thread only if all methods are
                // main thread only.
                //
                // This is done to make the UI nicer when the user tries to
                // implement such traits.
                //
                // Note: This is a deviation from the headers, but I don't
                // see a way for this to be unsound? As an example, let's say
                // there is some Objective-C code that assumes it can create
                // an object which is not `MainThreadOnly`, and then sets it
                // as the application delegate.
                //
                // Rust code that later retrieves the delegate would assume
                // that the object is `MainThreadOnly`, and could use this
                // information to create `MainThreadMarker`; but they can
                // _already_ do that, since the only way to retrieve the
                // delegate in the first place would be through
                // `NSApplication`!
                if !methods.is_empty() && methods.iter().all(|method| method.mainthreadonly) {
                    mainthreadonly = true;
                }

                // Overwrite with config preference
                if let Some(data) = data
                    .map(|data| data.requires_mainthreadonly)
                    .unwrap_or_default()
                {
                    if mainthreadonly == data {
                        warn!(
                            mainthreadonly,
                            data,
                            "set requires-mainthreadonly to the same value that it already has"
                        );
                    }
                    mainthreadonly = data;
                }

                vec![Self::ProtocolDecl {
                    id,
                    actual_name,
                    availability,
                    protocols,
                    methods,
                    required_sendable: sendable.unwrap_or(false),
                    required_mainthreadonly: mainthreadonly,
                }]
            }
            EntityKind::TypedefDecl => {
                let id = ItemIdentifier::new(entity, context);
                let availability = Availability::parse(entity, context);
                let mut kind = None;

                immediate_children(entity, |entity, _span| match entity.get_kind() {
                    EntityKind::UnexposedAttr => {
                        if let Some(attr) = UnexposedAttr::parse(&entity, context) {
                            if kind.is_some() {
                                panic!("got multiple unexposed attributes {kind:?}, {attr:?}");
                            }
                            match attr {
                                // TODO
                                UnexposedAttr::Sendable => warn!("sendable on typedef"),
                                _ => kind = Some(attr),
                            }
                        }
                    }
                    EntityKind::StructDecl
                    | EntityKind::ObjCClassRef
                    | EntityKind::ObjCProtocolRef
                    | EntityKind::TypeRef
                    | EntityKind::ParmDecl => {}
                    _ => error!("unknown"),
                });

                if context
                    .typedef_data
                    .get(&id.name)
                    .map(|data| data.skipped)
                    .unwrap_or_default()
                {
                    return vec![];
                }

                let ty = entity
                    .get_typedef_underlying_type()
                    .expect("typedef underlying type");
                if let Some(ty) = Ty::parse_typedef(ty, &id.name, context) {
                    vec![Self::AliasDecl {
                        id,
                        availability,
                        ty,
                        kind,
                    }]
                } else {
                    vec![]
                }
            }
            EntityKind::StructDecl => {
                let id = ItemIdentifier::new(entity, context);

                let availability = Availability::parse(entity, context);

                if context
                    .struct_data
                    .get(&id.name)
                    .map(|data| data.skipped)
                    .unwrap_or_default()
                {
                    return vec![];
                }

                // See https://github.com/rust-lang/rust-bindgen/blob/95fd17b874910184cc0fcd33b287fa4e205d9d7a/bindgen/ir/comp.rs#L1392-L1408
                if !entity.is_definition() {
                    return vec![];
                }

                let ty = entity.get_type().unwrap();
                let enc = ty.get_objc_encoding().unwrap();
                let encoding_name = enc.strip_prefix('{').unwrap().split_once('=').unwrap().0;
                let encoding_name = if encoding_name == id.name {
                    None
                } else {
                    Some(encoding_name.to_string())
                };

                let mut boxable = false;
                let mut fields = Vec::new();
                let mut sendable = None;

                immediate_children(entity, |entity, span| match entity.get_kind() {
                    EntityKind::UnexposedAttr => {
                        if let Some(attr) = UnexposedAttr::parse(&entity, context) {
                            match attr {
                                UnexposedAttr::Sendable => sendable = Some(true),
                                UnexposedAttr::NonSendable => sendable = Some(false),
                                attr => error!(?attr, "unknown attribute"),
                            }
                        }
                    }
                    EntityKind::FieldDecl => {
                        drop(span);
                        let name = entity.get_name().expect("struct field name");
                        let _span = debug_span!("field", name).entered();

                        let ty = entity.get_type().expect("struct field type");
                        let ty = Ty::parse_struct_field(ty, context);

                        if entity.is_bit_field() {
                            error!("unsound struct bitfield");
                        }

                        fields.push((name, ty))
                    }
                    EntityKind::ObjCBoxable => {
                        boxable = true;
                    }
                    EntityKind::UnionDecl => error!("can't handle unions in structs yet"),
                    _ => error!("unknown"),
                });

                vec![Self::StructDecl {
                    id,
                    encoding_name,
                    availability,
                    boxable,
                    fields,
                    sendable,
                }]
            }
            EntityKind::EnumDecl => {
                // Enum declarations show up twice for some reason, but
                // luckily this flag is set on the least descriptive entity.
                if !entity.is_definition() {
                    return vec![];
                }

                let id = new_enum_id(entity, context);

                let data = context
                    .enum_data
                    .get(id.name.as_deref().unwrap_or("anonymous"))
                    .cloned()
                    .unwrap_or_default();
                if data.skipped {
                    return vec![];
                }

                let availability = Availability::parse(entity, context);

                let ty = entity.get_enum_underlying_type().expect("enum type");
                let is_signed = ty.is_signed_integer();
                let ty = Ty::parse_enum(ty, context);
                let mut kind = None;
                let mut variants = Vec::new();
                let mut sendable = None;

                immediate_children(entity, |entity, _span| match entity.get_kind() {
                    EntityKind::EnumConstantDecl => {
                        let name = entity.get_name().expect("enum constant name");
                        let availability = Availability::parse(&entity, context);

                        if data
                            .constants
                            .get(&name)
                            .map(|data| data.skipped)
                            .unwrap_or_default()
                        {
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

                        if !data.use_value {
                            // Some enums constants don't declare a value, but
                            // let it be inferred from the position in the
                            // enum instead; in those cases, we use the value
                            // generated above.
                            immediate_children(&entity, |entity, _span| match entity.get_kind() {
                                EntityKind::UnexposedAttr => {
                                    if let Some(attr) = UnexposedAttr::parse(&entity, context) {
                                        error!(?attr, "unknown attribute");
                                    }
                                }
                                EntityKind::VisibilityAttr => {}
                                _ if entity.is_expression() => {
                                    expr = Expr::parse_enum_constant(&entity, context);
                                }
                                _ => {
                                    panic!("unknown EnumConstantDecl child in {name:?}: {entity:?}")
                                }
                            });
                        };

                        variants.push((name, availability, expr));
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
                        .map(|(i, (name, availability, value))| Self::ConstDecl {
                            id: id.clone().map_name(|_| name),
                            availability,
                            ty: ty.clone(),
                            value,
                            is_last: i == variants_len - 1,
                        })
                        .collect()
                } else {
                    vec![Self::EnumDecl {
                        id: id.map_name(|name| name.unwrap()),
                        availability,
                        ty,
                        kind,
                        variants,
                        sendable,
                    }]
                }
            }
            EntityKind::VarDecl => {
                let id = ItemIdentifier::new(entity, context);

                if context
                    .statics
                    .get(&id.name)
                    .map(|data| data.skipped)
                    .unwrap_or_default()
                {
                    return vec![];
                }

                let availability = Availability::parse(entity, context);
                let ty = entity.get_type().expect("var type");
                let ty = Ty::parse_static(ty, context);
                let mut value = None;

                immediate_children(entity, |entity, _span| match entity.get_kind() {
                    EntityKind::UnexposedAttr => {
                        if let Some(attr) = UnexposedAttr::parse(&entity, context) {
                            error!(?attr, "unknown attribute");
                        }
                    }
                    EntityKind::VisibilityAttr => {}
                    EntityKind::ObjCClassRef => {}
                    EntityKind::TypeRef => {}
                    _ if entity.is_expression() => {
                        if value.is_none() {
                            value = Some(Expr::parse_var(&entity, context));
                        } else {
                            panic!("got variable value twice")
                        }
                    }
                    _ => panic!("unknown vardecl child in {id:?}: {entity:?}"),
                });

                vec![Self::VarDecl {
                    id,
                    availability,
                    ty,
                    value,
                }]
            }
            EntityKind::FunctionDecl => {
                let id = ItemIdentifier::new(entity, context);

                let data = context.fns.get(&id.name).cloned().unwrap_or_default();

                if data.skipped {
                    return vec![];
                }

                if entity.is_variadic() {
                    warn!("can't handle variadic function");
                    return vec![];
                }

                let availability = Availability::parse(entity, context);
                let result_type = entity.get_result_type().expect("function result type");
                let result_type = Ty::parse_function_return(result_type, context);
                let mut arguments = Vec::new();

                if entity.is_static_method() {
                    warn!("unexpected static method");
                }

                immediate_children(entity, |entity, _span| match entity.get_kind() {
                    EntityKind::UnexposedAttr => {
                        if let Some(attr) = UnexposedAttr::parse(&entity, context) {
                            error!(?attr, "unknown attribute");
                        }
                    }
                    EntityKind::ObjCClassRef
                    | EntityKind::TypeRef
                    | EntityKind::ObjCProtocolRef => {}
                    EntityKind::ParmDecl => {
                        parse_fn_param_children(&entity, context);
                        // Could also be retrieved via. `get_arguments`
                        let name = entity.get_name().unwrap_or_else(|| "_".into());
                        let ty = entity.get_type().expect("function argument type");
                        let ty = Ty::parse_function_argument(ty, context);
                        arguments.push((name, ty))
                    }
                    EntityKind::VisibilityAttr => {
                        // CG_EXTERN or UIKIT_EXTERN
                    }
                    _ => error!("unknown"),
                });

                let body = if entity.is_inline_function() {
                    Some(())
                } else {
                    None
                };

                vec![Self::FnDecl {
                    id,
                    availability,
                    arguments,
                    result_type,
                    body,
                    safe: !data.unsafe_,
                }]
            }
            EntityKind::UnionDecl => {
                let id = ItemIdentifier::new_optional(entity, context);
                error!(
                    ?id,
                    has_attributes = ?entity.has_attributes(),
                    children = ?entity.get_children(),
                    "union",
                );
                vec![]
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

    pub fn compare(&self, other: &Self) {
        if self != other {
            if let (
                Self::ExternMethods {
                    methods: self_methods,
                    ..
                },
                Self::ExternMethods {
                    methods: other_methods,
                    ..
                },
            ) = (&self, &other)
            {
                super::compare_slice(
                    self_methods,
                    other_methods,
                    |i, self_method, other_method| {
                        let _span = debug_span!("method", i).entered();
                        assert_eq!(self_method, other_method, "methods were not equal");
                    },
                );
            }

            panic!("statements were not equal:\n{self:#?}\n{other:#?}");
        }
    }

    pub(crate) fn features(&self) -> Features {
        let mut features = Features::new();
        match self {
            Stmt::ClassDecl { id, .. } => features.add_item(id),
            Stmt::ExternCategory { cls, .. } => features.add_item(cls),
            Stmt::FnDecl {
                arguments,
                result_type,
                ..
            } => {
                for (_, arg) in arguments {
                    arg.visit_required_types(&mut |item| features.add_item(item));
                }
                result_type.visit_required_types(&mut |item| features.add_item(item));
            }
            _ => {}
        }
        features
    }

    pub(crate) fn name(&self) -> Option<&str> {
        match self {
            Stmt::ClassDecl { id, skipped, .. } => {
                if *skipped {
                    None
                } else {
                    Some(&id.name)
                }
            }
            Stmt::ExternMethods { .. } => None,
            Stmt::ExternCategory { id, .. } => Some(&id.name),
            Stmt::ProtocolDecl { id, .. } => Some(&id.name),
            Stmt::ProtocolImpl { .. } => None,
            Stmt::StructDecl { id, .. } => Some(&id.name),
            Stmt::EnumDecl { id, .. } => Some(&id.name),
            Stmt::ConstDecl { id, .. } => Some(&id.name),
            Stmt::VarDecl { id, .. } => Some(&id.name),
            Stmt::FnDecl { id, body, .. } if body.is_none() => Some(&*id.name),
            // TODO
            Stmt::FnDecl { .. } => None,
            Stmt::AliasDecl { id, .. } => Some(&id.name),
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _span = debug_span!("stmt", discriminant = ?mem::discriminant(self)).entered();

        struct GenericTyHelper<'a>(&'a [String]);

        impl fmt::Display for GenericTyHelper<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if !self.0.is_empty() {
                    write!(f, "<")?;
                    for generic in self.0 {
                        write!(f, "{generic}, ")?;
                    }
                    write!(f, ">")?;
                }
                Ok(())
            }
        }

        struct GenericParamsHelper<'a>(&'a [String], &'a str);

        impl fmt::Display for GenericParamsHelper<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if !self.0.is_empty() {
                    write!(f, "<")?;
                    for generic in self.0 {
                        write!(f, "{generic}: {}, ", self.1)?;
                    }
                    write!(f, ">")?;
                }
                Ok(())
            }
        }

        struct WhereBoundHelper<'a>(&'a [String], Option<&'a str>);

        impl fmt::Display for WhereBoundHelper<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if let Some(bound) = self.1 {
                    if !self.0.is_empty() {
                        writeln!(f, "where")?;
                        for generic in self.0 {
                            writeln!(f, "{generic}{bound},")?;
                        }
                    }
                }
                Ok(())
            }
        }

        match self {
            Self::ClassDecl {
                id,
                generics,
                availability,
                superclasses,
                designated_initializers: _,
                derives,
                mutability,
                skipped,
                sendable,
            } => {
                if *skipped {
                    return Ok(());
                }

                let macro_name = if generics.is_empty() {
                    "extern_class"
                } else {
                    "__inner_extern_class"
                };

                let main_feature = match mutability {
                    Mutability::MutableWithImmutableSuperclass(superclass) => {
                        Feature::new(superclass)
                    }
                    Mutability::Immutable
                    | Mutability::Mutable
                    | Mutability::ImmutableWithMutableSubclass(_)
                    | Mutability::InteriorMutable
                    | Mutability::MainThreadOnly => Feature::new(id),
                };

                let (superclass, superclasses_rest) = superclasses.split_at(1);
                let (superclass, superclass_generics) = superclass
                    .first()
                    .expect("must have a least one superclass");

                writeln!(f, "{macro_name}!(")?;
                writeln!(f, "    {derives}")?;
                write!(f, "    {}", main_feature.cfg_gate_ln())?;
                write!(f, "{availability}")?;
                write!(f, "    pub struct {}", id.name)?;
                if !generics.is_empty() {
                    write!(f, "<")?;
                    for generic in generics {
                        write!(f, "{generic}: ?Sized = AnyObject, ")?;
                    }
                    write!(f, ">")?;
                };
                if generics.is_empty() {
                    writeln!(f, ";")?;
                } else {
                    writeln!(f, " {{")?;
                    writeln!(
                        f,
                        "__superclass: {}{},",
                        superclass.path_in_relation_to(id),
                        GenericTyHelper(superclass_generics),
                    )?;
                    for (i, generic) in generics.iter().enumerate() {
                        // Invariant over the generic by default
                        writeln!(f, "_inner{i}: PhantomData<*mut {generic}>,")?;
                    }
                    writeln!(f, "notunwindsafe: PhantomData<&'static mut ()>,")?;
                    writeln!(f, "}}")?;
                }

                writeln!(f)?;

                write!(f, "    {}", main_feature.cfg_gate_ln())?;
                writeln!(
                    f,
                    "    unsafe impl{} ClassType for {}{} {{",
                    GenericParamsHelper(generics, "?Sized + Message"),
                    id.name,
                    GenericTyHelper(generics),
                )?;
                if !superclasses_rest.is_empty() {
                    write!(f, "        #[inherits(")?;
                    let mut iter = superclasses_rest.iter();
                    // Using generics in here is not technically correct, but
                    // should work for our use-cases.
                    if let Some((superclass, generics)) = iter.next() {
                        write!(
                            f,
                            "{}{}",
                            superclass.path_in_relation_to(id),
                            GenericTyHelper(generics)
                        )?;
                    }
                    for (superclass, generics) in iter {
                        write!(
                            f,
                            ", {}{}",
                            superclass.path_in_relation_to(id),
                            GenericTyHelper(generics)
                        )?;
                    }
                    writeln!(f, ")]")?;
                }
                writeln!(
                    f,
                    "        type Super = {}{};",
                    superclass.path_in_relation_to(id),
                    GenericTyHelper(superclass_generics),
                )?;
                writeln!(f, "        type Mutability = {mutability};")?;
                if !generics.is_empty() {
                    writeln!(f)?;
                    writeln!(
                        f,
                        "        fn as_super(&self) -> &Self::Super {{ &self.__superclass }}"
                    )?;
                    writeln!(f)?;
                    writeln!(f, "        fn as_super_mut(&mut self) -> &mut Self::Super {{ &mut self.__superclass }}")?;
                }
                writeln!(f, "    }}")?;
                writeln!(f, ");")?;

                if *sendable && generics.is_empty() {
                    writeln!(f)?;
                    write!(f, "{}", main_feature.cfg_gate_ln())?;
                    writeln!(f, "unsafe impl Send for {} {{}}", id.name)?;

                    writeln!(f)?;
                    write!(f, "{}", main_feature.cfg_gate_ln())?;
                    writeln!(f, "unsafe impl Sync for {} {{}}", id.name)?;
                }
            }
            Self::ExternMethods {
                availability: _,
                cls,
                source_superclass,
                cls_generics,
                category_name,
                methods,
            } => {
                writeln!(f, "extern_methods!(")?;
                if let Some(source_superclass) = source_superclass {
                    writeln!(
                        f,
                        "    /// Methods declared on superclass `{}`",
                        source_superclass.name
                    )?;
                    if let Some(category_name) = category_name {
                        writeln!(f, "///")?;
                        writeln!(f, "    /// {category_name}")?;
                    }
                } else if let Some(category_name) = category_name {
                    writeln!(f, "    /// {category_name}")?;
                }
                write!(f, "    {}", Feature::new(cls).cfg_gate_ln())?;
                // TODO: Add ?Sized here once `extern_methods!` supports it.
                writeln!(
                    f,
                    "    unsafe impl{} {}{} {{",
                    GenericParamsHelper(cls_generics, "Message"),
                    cls.path(),
                    GenericTyHelper(cls_generics),
                )?;
                for method in methods {
                    writeln!(f, "{method}")?;
                }
                writeln!(f, "    }}")?;
                writeln!(f, ");")?;

                if let Some(method) = methods.iter().find(|method| method.usable_in_default_id()) {
                    writeln!(f)?;
                    // Assume `new` methods require no extra features
                    write!(f, "    {}", Feature::new(cls).cfg_gate_ln())?;
                    writeln!(
                        f,
                        "impl{} DefaultId for {}{} {{",
                        GenericParamsHelper(cls_generics, "Message"),
                        cls.path(),
                        GenericTyHelper(cls_generics),
                    )?;
                    writeln!(f, "    #[inline]")?;
                    writeln!(f, "    fn default_id() -> Id<Self> {{")?;
                    writeln!(f, "        Self::{}()", method.fn_name)?;
                    writeln!(f, "    }}")?;
                    writeln!(f, "}}")?;
                }
            }
            Self::ExternCategory {
                id,
                actual_name,
                availability,
                cls,
                methods,
            } => {
                writeln!(f, "extern_category!(")?;

                if let Some(actual_name) = actual_name {
                    if *actual_name != id.name {
                        writeln!(f, "    /// Category \"{actual_name}\" on [`{}`].", cls.name)?;
                        writeln!(f, "    #[doc(alias = \"{actual_name}\")]")?;
                    } else {
                        writeln!(f, "    /// Category on [`{}`].", cls.name)?;
                    }
                } else {
                    writeln!(f, "    /// Category on [`{}`].", cls.name)?;
                }

                write!(f, "    {}", Feature::new(cls).cfg_gate_ln())?;
                write!(f, "{availability}")?;
                writeln!(f, "    pub unsafe trait {} {{", id.name)?;
                for method in methods {
                    writeln!(f, "{method}")?;
                }
                writeln!(f, "    }}")?;

                writeln!(f)?;

                write!(f, "    {}", Feature::new(cls).cfg_gate_ln())?;
                writeln!(
                    f,
                    "    unsafe impl {} for {} {{}}",
                    id.name,
                    cls.path_in_relation_to(id),
                )?;

                writeln!(f, ");")?;
            }
            Self::ProtocolImpl {
                cls,
                generics,
                protocol,
                availability: _,
            } => {
                let (generic_bound, where_bound) = if !generics.is_empty() {
                    match (&*protocol.library, &*protocol.name) {
                        // The object inherits from `NSObject` or `NSProxy` no
                        // matter what the generic type is, so this must be
                        // safe.
                        (_, _) if protocol.is_nsobject() => ("?Sized", None),
                        // Encoding and decoding requires that the inner types
                        // are codable as well.
                        ("Foundation", "NSCoding") => ("?Sized + NSCoding", None),
                        ("Foundation", "NSSecureCoding") => ("?Sized + NSSecureCoding", None),
                        // Copying collections is done as a shallow copy:
                        // <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Collections/Articles/Copying.html>
                        //
                        // E.g. it does a retain count bump on the items, and
                        // hence does not require the inner type to implement
                        // `NSCopying`.
                        //
                        // The types does have to be cloneable, since generic
                        // types effectively store an `Id<T>` of the type.
                        ("Foundation", "NSCopying") => ("?Sized + IsIdCloneable", None),
                        ("Foundation", "NSMutableCopying") => ("?Sized + IsIdCloneable", None),
                        // TODO: Do we need further tweaks to this?
                        ("Foundation", "NSFastEnumeration") => ("?Sized", None),
                        // AppKit fixes. TODO: Should we add more bounds here?
                        ("AppKit", "NSCollectionViewDataSource") => ("?Sized", None),
                        ("AppKit", "NSTableViewDataSource") => ("?Sized", None),
                        _ => {
                            error!(
                                ?protocol,
                                ?cls,
                                "unknown where bound for generic protocol impl"
                            );
                            ("?Sized", None)
                        }
                    }
                } else {
                    ("InvalidGenericBound", None)
                };

                write!(f, "{}", Feature::new(cls).cfg_gate_ln())?;
                writeln!(
                    f,
                    "unsafe impl{} {} for {}{} {}{{}}",
                    GenericParamsHelper(generics, generic_bound),
                    protocol.path_in_relation_to(cls),
                    cls.path(),
                    GenericTyHelper(generics),
                    WhereBoundHelper(generics, where_bound)
                )?;
            }
            Self::ProtocolDecl {
                id,
                actual_name,
                availability,
                protocols,
                methods,
                required_sendable: _,
                required_mainthreadonly,
            } => {
                writeln!(f, "extern_protocol!(")?;
                write!(f, "{availability}")?;

                write!(f, "    pub unsafe trait {}", id.name)?;
                if !protocols.is_empty() {
                    for (i, protocol) in protocols.iter().enumerate() {
                        if i == 0 {
                            write!(f, ": ")?;
                        } else {
                            write!(f, "+ ")?;
                        }
                        write!(f, "{}", protocol.path())?;
                    }
                }
                // TODO
                // if *required_sendable {
                //     if protocols.is_empty() {
                //         write!(f, ": ")?;
                //     } else {
                //         write!(f, "+ ")?;
                //     }
                //     write!(f, "Send + Sync")?;
                // }
                if *required_mainthreadonly {
                    if protocols.is_empty() {
                        write!(f, ": ")?;
                    } else {
                        write!(f, "+ ")?;
                    }
                    write!(f, "IsMainThreadOnly")?;
                }
                writeln!(f, " {{")?;

                for method in methods {
                    writeln!(f, "{method}")?;
                }
                writeln!(f, "    }}")?;
                writeln!(f)?;
                writeln!(f, "    unsafe impl ProtocolType for dyn {} {{", id.name)?;
                if let Some(actual_name) = actual_name {
                    writeln!(f)?;
                    writeln!(f, "        const NAME: &'static str = {actual_name:?};")?;
                    write!(f, "    ")?;
                }
                writeln!(f, "}}")?;
                writeln!(f, ");")?;
            }
            Self::StructDecl {
                id,
                encoding_name,
                availability,
                boxable: _,
                fields,
                sendable,
            } => {
                writeln!(f, "extern_struct!(")?;
                if let Some(encoding_name) = encoding_name {
                    writeln!(f, "    #[encoding_name({encoding_name:?})]")?;
                }
                write!(f, "{availability}")?;
                writeln!(f, "    pub struct {} {{", id.name)?;
                for (name, ty) in fields {
                    write!(f, "        ")?;
                    if !name.starts_with('_') {
                        write!(f, "pub ")?;
                    }
                    let name = handle_reserved(name);
                    writeln!(f, "{name}: {},", ty.struct_())?;
                }
                writeln!(f, "    }}")?;
                writeln!(f, ");")?;

                if let Some(true) = sendable {
                    writeln!(f)?;
                    writeln!(f, "unsafe impl Send for {} {{}}", id.name)?;

                    writeln!(f)?;
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
            } => {
                let macro_name = match kind {
                    None => "extern_enum",
                    Some(UnexposedAttr::Enum) => "ns_enum",
                    Some(UnexposedAttr::Options) => "ns_options",
                    Some(UnexposedAttr::ClosedEnum) => "ns_closed_enum",
                    Some(UnexposedAttr::ErrorEnum) => "ns_error_enum",
                    _ => panic!("invalid enum kind"),
                };
                writeln!(f, "{macro_name}!(")?;
                writeln!(f, "    #[underlying({})]", ty.enum_())?;
                write!(f, "{availability}")?;
                writeln!(f, "    pub enum {} {{", id.name)?;
                for (name, availability, expr) in variants {
                    write!(f, "{availability}")?;
                    writeln!(f, "        {name} = {expr},")?;
                }
                writeln!(f, "    }}")?;
                writeln!(f, ");")?;

                if let Some(true) = sendable {
                    writeln!(f)?;
                    writeln!(f, "unsafe impl Send for {} {{}}", id.name)?;

                    writeln!(f)?;
                    writeln!(f, "unsafe impl Sync for {} {{}}", id.name)?;
                }
            }
            Self::ConstDecl {
                id,
                availability,
                ty,
                value,
                is_last,
            } => {
                write!(f, "{availability}")?;
                write!(f, "pub const {}: {} = {value};", id.name, ty.enum_())?;
                if *is_last {
                    writeln!(f)?;
                }
            }
            Self::VarDecl {
                id,
                availability: _,
                ty,
                value: None,
            } => {
                writeln!(f, "extern_static!({}: {});", id.name, ty.var())?;
            }
            Self::VarDecl {
                id,
                availability: _,
                ty,
                value: Some(expr),
            } => {
                if ty.is_enum_through_typedef() {
                    writeln!(
                        f,
                        "extern_static!({}: {} = {}({expr}));",
                        id.name,
                        ty.var(),
                        ty.var()
                    )?;
                } else {
                    writeln!(f, "extern_static!({}: {} = {expr});", id.name, ty.var())?;
                }
            }
            Self::FnDecl {
                id,
                availability,
                arguments,
                result_type,
                body,
                safe,
            } => {
                if body.is_some() {
                    writeln!(f, "inline_fn!(")?;
                } else {
                    writeln!(f, "extern_fn!(")?;
                }

                write!(f, "    {}", self.features().cfg_gate_ln())?;

                let unsafe_ = if *safe { "" } else { " unsafe" };

                write!(f, "{availability}")?;
                write!(f, "    pub{unsafe_} fn {}(", id.name)?;
                for (param, arg_ty) in arguments {
                    let param = handle_reserved(&crate::to_snake_case(param));
                    write!(f, "{param}: {},", arg_ty.fn_argument())?;
                }
                write!(f, "){}", result_type.fn_return())?;

                if body.is_some() {
                    writeln!(f, "{{")?;
                    writeln!(f, "        todo!()")?;
                    writeln!(f, "    }}")?;
                } else {
                    writeln!(f, ";")?;
                }

                writeln!(f, ");")?;
            }
            Self::AliasDecl {
                id,
                availability: _,
                ty,
                kind,
            } => {
                match kind {
                    Some(UnexposedAttr::TypedEnum) => {
                        writeln!(f, "typed_enum!(pub type {} = {};);", id.name, ty.typedef())?;
                    }
                    Some(UnexposedAttr::TypedExtensibleEnum) => {
                        writeln!(
                            f,
                            "typed_extensible_enum!(pub type {} = {};);",
                            id.name,
                            ty.typedef()
                        )?;
                    }
                    None | Some(UnexposedAttr::BridgedTypedef) => {
                        // "bridged" typedefs should use a normal type alias.
                        writeln!(f, "pub type {} = {};", id.name, ty.typedef())?;
                    }
                    kind => panic!("invalid alias kind {kind:?} for {ty:?}"),
                }
            }
        };
        Ok(())
    }
}
