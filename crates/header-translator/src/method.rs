use std::fmt;

use clang::{Entity, EntityKind, ObjCAttributes, ObjCQualifiers};
use tracing::span::EnteredSpan;

use crate::availability::Availability;
use crate::config::MethodData;
use crate::context::Context;
use crate::id::ItemIdentifier;
use crate::immediate_children;
use crate::objc2_utils::in_selector_family;
use crate::rust_type::{MethodArgumentQualifier, Ty};
use crate::stmt::get_category_cls;
use crate::unexposed_attr::UnexposedAttr;

impl MethodArgumentQualifier {
    pub fn parse(qualifiers: ObjCQualifiers) -> Self {
        match qualifiers {
            ObjCQualifiers {
                in_: true,
                inout: false,
                out: false,
                bycopy: false,
                byref: false,
                oneway: false,
            } => Self::In,
            ObjCQualifiers {
                in_: false,
                inout: true,
                out: false,
                bycopy: false,
                byref: false,
                oneway: false,
            } => Self::Inout,
            ObjCQualifiers {
                in_: false,
                inout: false,
                out: true,
                bycopy: false,
                byref: false,
                oneway: false,
            } => Self::Out,
            qualifiers => unreachable!("unsupported qualifiers {qualifiers:?}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
struct MethodModifiers {
    returns_inner_pointer: bool,
    consumes_self: bool,
    returns_retained: bool,
    returns_not_retained: bool,
    designated_initializer: bool,
    non_isolated: bool,
    sendable: Option<bool>,
    mainthreadonly: bool,
}

impl MethodModifiers {
    fn parse(entity: &Entity<'_>, context: &Context<'_>) -> Self {
        let mut this = Self::default();

        immediate_children(entity, |entity, _span| match entity.get_kind() {
            EntityKind::UnexposedAttr => {
                if let Some(attr) = UnexposedAttr::parse(&entity, context) {
                    match attr {
                        UnexposedAttr::ReturnsRetained => {
                            this.returns_retained = true;
                        }
                        UnexposedAttr::ReturnsNotRetained => {
                            this.returns_not_retained = true;
                        }
                        UnexposedAttr::NonIsolated => {
                            this.non_isolated = true;
                        }
                        UnexposedAttr::Sendable => {
                            this.sendable = Some(true);
                        }
                        UnexposedAttr::NonSendable => {
                            this.sendable = Some(false);
                        }
                        UnexposedAttr::UIActor => {
                            this.mainthreadonly = true;
                        }
                        attr => error!(?attr, "unknown attribute"),
                    }
                }
            }
            EntityKind::ObjCReturnsInnerPointer => {
                this.returns_inner_pointer = true;
            }
            EntityKind::NSConsumesSelf => {
                this.consumes_self = true;
            }
            EntityKind::NSReturnsAutoreleased => {
                error!("found NSReturnsAutoreleased, which requires manual handling");
            }
            EntityKind::NSReturnsRetained => {
                this.returns_retained = true;
            }
            EntityKind::NSReturnsNotRetained => {
                this.returns_not_retained = true;
            }
            EntityKind::ObjCDesignatedInitializer => {
                this.designated_initializer = true;
            }
            EntityKind::ObjCRequiresSuper => {
                // TODO: Can we use this for something?
                // <https://clang.llvm.org/docs/AttributeReference.html#objc-requires-super>
            }
            EntityKind::WarnUnusedResultAttr => {
                // TODO: Emit `#[must_use]` on this
            }
            EntityKind::ObjCClassRef
            | EntityKind::ObjCProtocolRef
            | EntityKind::TypeRef
            | EntityKind::ParmDecl => {
                // Ignore
            }
            EntityKind::IbActionAttr | EntityKind::IbOutletAttr => {
                // TODO: Do we want to do something special here?
            }
            EntityKind::ObjCInstanceMethodDecl => {
                warn!("method inside property/method");
            }
            EntityKind::VisibilityAttr => {
                // TODO: Handle these visibility attributes
            }
            _ => error!("unknown"),
        });

        this
    }
}

/// The retain semantics calling convention for a method.
///
/// This also encodes the "method family" that a method belongs to.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MemoryManagement {
    IdCopyOrMutCopy,
    IdNew,
    IdInit,
    IdOther,
    // TODO:
    // IdReturnsRetained,
    // IdReturnsNotRetained,
    Normal,
}

impl MemoryManagement {
    /// The calling convention depends solely on these arguments.
    ///
    /// See <https://clang.llvm.org/docs/AutomaticReferenceCounting.html#method-families>
    fn new(is_class: bool, selector: &str, result_type: &Ty, modifiers: MethodModifiers) -> Self {
        // The method has been checked already to not have a
        // `objc_method_family` attribute.

        // If:
        // > its selector falls into the corresponding selector family
        let bytes = selector.as_bytes();
        let id_type = match (
            in_selector_family(bytes, b"alloc"),
            in_selector_family(bytes, b"copy"),
            in_selector_family(bytes, b"mutableCopy"),
            in_selector_family(bytes, b"new"),
            in_selector_family(bytes, b"init"),
        ) {
            (true, false, false, false, false) => {
                // It's not really worth the effort to support these, since
                // they're only defined on `NSObject` and `NSProxy`, and we
                // have it in `ClassType` anyhow.
                error!("the `alloc` method-family requires manual handling");
                Self::IdOther
            }
            (false, true, false, false, false) => Self::IdCopyOrMutCopy,
            (false, false, true, false, false) => Self::IdCopyOrMutCopy,
            (false, false, false, true, false) => Self::IdNew,
            (false, false, false, false, true) => Self::IdInit,
            (false, false, false, false, false) => Self::IdOther,
            _ => unreachable!(),
        };

        // And if:
        // > its signature obeys the added restrictions of the method family.
        //
        // Which is:
        // > must return a retainable object pointer
        if result_type.is_id() {
            // We also check that the correct modifier flags were set for the
            // given method family.
            match (
                modifiers.returns_inner_pointer,
                modifiers.consumes_self,
                modifiers.returns_retained,
                modifiers.returns_not_retained,
                modifiers.designated_initializer,
                id_type,
            ) {
                (false, false, true, false, false, Self::IdCopyOrMutCopy) => Self::IdCopyOrMutCopy,
                (false, false, true, false, false, Self::IdNew) => Self::IdNew,
                // For the `init` family there's another restriction:
                // > must be instance methods
                //
                // Note: There's some extra restrictions about a program being
                // "ill-formed" if it contains certain selectors with `init`
                // methods that are not correct super/subclasses, but we don't
                // need to handle that since the header would fail to compile
                // in `clang` if that was the case.
                (false, true, true, false, _, Self::IdInit) => {
                    if is_class {
                        Self::IdOther
                    } else {
                        Self::IdInit
                    }
                }
                (false, false, false, false, false, Self::IdOther) => Self::IdOther,
                data => {
                    error!(?data, "invalid MemoryManagement id attributes");
                    Self::IdOther
                }
            }
        } else if let MethodModifiers {
            designated_initializer: false,
            // TODO: Maybe we can use this to emit things with lifetime of:
            // `'self + 'autoreleasepool`
            returns_inner_pointer: _,
            consumes_self: false,
            returns_retained: false,
            returns_not_retained: false,
            ..
        } = modifiers
        {
            Self::Normal
        } else {
            error!(?modifiers, "invalid MemoryManagement attributes");
            Self::Normal
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Method {
    pub selector: String,
    pub fn_name: String,
    availability: Availability,
    pub is_class: bool,
    is_optional_protocol: bool,
    memory_management: MemoryManagement,
    pub(crate) arguments: Vec<(String, Ty)>,
    pub result_type: Ty,
    safe: bool,
    mutating: bool,
    is_protocol: bool,
    // Thread-safe, even on main-thread only (@MainActor/@UIActor) classes
    non_isolated: bool,
    pub(crate) mainthreadonly: bool,
}

impl Method {
    /// Value that uniquely identifies the method in a class.
    pub fn id(&self) -> (bool, String) {
        (self.is_class, self.selector.clone())
    }

    pub(crate) fn usable_in_default_id(&self) -> bool {
        self.selector == "new"
            && self.is_class
            && self.arguments.is_empty()
            && self.safe
            && !self.mainthreadonly
    }

    fn parent_type_data(entity: &Entity<'_>, context: &Context<'_>) -> (bool, bool) {
        let parent = entity.get_semantic_parent().expect("method parent");
        let (parent, is_protocol) = match parent.get_kind() {
            EntityKind::ObjCInterfaceDecl => (parent, false),
            EntityKind::ObjCCategoryDecl => (get_category_cls(&parent), false),
            EntityKind::ObjCProtocolDecl => (parent, true),
            kind => {
                error!(?kind, "unknown method parent kind");
                (parent, false)
            }
        };
        let parent_id = ItemIdentifier::new(&parent, context);

        let is_mutable = if !is_protocol {
            context
                .class_data
                .get(&parent_id.name)
                .map(|data| data.mutability.is_mutable())
                .unwrap_or(false)
        } else {
            false
        };

        (is_mutable, is_protocol)
    }

    /// Takes one of `EntityKind::ObjCInstanceMethodDecl` or
    /// `EntityKind::ObjCClassMethodDecl`.
    pub fn partial(entity: Entity<'_>) -> PartialMethod<'_> {
        let selector = entity.get_name().expect("method selector");
        let fn_name = selector.trim_end_matches(|c| c == ':').replace(':', "_");

        let _span = debug_span!("method", fn_name).entered();

        let is_class = match entity.get_kind() {
            EntityKind::ObjCInstanceMethodDecl => false,
            EntityKind::ObjCClassMethodDecl => true,
            _ => unreachable!("unknown method kind"),
        };

        PartialMethod {
            entity,
            selector,
            is_class,
            fn_name,
            _span,
        }
    }

    /// Takes `EntityKind::ObjCPropertyDecl`.
    pub fn partial_property(entity: Entity<'_>) -> PartialProperty<'_> {
        let attributes = entity.get_objc_attributes();
        let has_setter = attributes.map(|a| !a.readonly).unwrap_or(true);

        let name = entity.get_display_name().expect("property name");
        let _span = debug_span!("property", name).entered();

        PartialProperty {
            entity,
            name,
            getter_name: entity.get_objc_getter_name().expect("property getter name"),
            setter_name: has_setter.then(|| {
                entity
                    .get_objc_setter_name()
                    .expect("property setter name")
                    .trim_end_matches(|c| c == ':')
                    .to_string()
            }),
            is_class: attributes.map(|a| a.class).unwrap_or(false),
            attributes,
            _span,
        }
    }

    pub fn visit_required_types(&self, mut f: impl FnMut(&ItemIdentifier)) {
        for (_, arg) in &self.arguments {
            arg.visit_required_types(&mut f);
        }

        self.result_type.visit_required_types(&mut f);
    }
}

#[derive(Debug)]
pub struct PartialMethod<'tu> {
    entity: Entity<'tu>,
    selector: String,
    pub is_class: bool,
    pub fn_name: String,
    _span: EnteredSpan,
}

impl<'tu> PartialMethod<'tu> {
    pub fn parse(self, data: MethodData, context: &Context<'_>) -> Option<(bool, Method)> {
        let Self {
            entity,
            selector,
            is_class,
            fn_name,
            _span,
        } = self;

        if data.skipped {
            return None;
        }

        if entity.is_variadic() {
            warn!("can't handle variadic method");
            return None;
        }

        let (parent_is_mutable, is_protocol) = Method::parent_type_data(&entity, context);

        let availability = Availability::parse(&entity, context);

        let modifiers = MethodModifiers::parse(&entity, context);

        if modifiers.sendable.is_some() {
            error!("sendable on method");
        }

        let mut arguments: Vec<_> = entity
            .get_arguments()
            .expect("method arguments")
            .into_iter()
            .map(|entity| {
                let name = entity.get_name().expect("arg display name");
                let _span = debug_span!("method argument", name).entered();
                let qualifier = entity
                    .get_objc_qualifiers()
                    .map(MethodArgumentQualifier::parse);
                let mut sendable = None;
                let mut no_escape = false;

                immediate_children(&entity, |entity, _span| match entity.get_kind() {
                    EntityKind::ObjCClassRef
                    | EntityKind::ObjCProtocolRef
                    | EntityKind::TypeRef
                    | EntityKind::ParmDecl => {
                        // Ignore
                    }
                    EntityKind::NSConsumed => {
                        error!("found NSConsumed, which requires manual handling");
                    }
                    EntityKind::UnexposedAttr => {
                        if let Some(attr) = UnexposedAttr::parse(&entity, context) {
                            match attr {
                                UnexposedAttr::Sendable => sendable = Some(true),
                                UnexposedAttr::NonSendable => sendable = Some(false),
                                UnexposedAttr::NoEscape => no_escape = true,
                                attr => error!(?attr, "unknown attribute"),
                            }
                        }
                    }
                    // For some reason we recurse into array types
                    EntityKind::IntegerLiteral => {}
                    _ => error!("unknown"),
                });

                let ty = entity.get_type().expect("argument type");
                let ty = Ty::parse_method_argument(ty, qualifier, sendable, no_escape, context);

                (name, ty)
            })
            .collect();

        let is_error = if let Some((_, ty)) = arguments.last() {
            ty.argument_is_error_out()
        } else {
            false
        };

        // TODO: Strip these from function name?
        // selector.ends_with("error:")
        // || selector.ends_with("AndReturnError:")
        // || selector.ends_with("WithError:")

        if is_error {
            arguments.pop();
        }

        if let Some(qualifiers) = entity.get_objc_qualifiers() {
            error!(?qualifiers, "unsupported qualifiers on return type");
        }

        let result_type = entity.get_result_type().expect("method return type");
        let default_nonnull = (selector == "init" && !is_class) || (selector == "new" && is_class);
        let mut result_type = Ty::parse_method_return(result_type, default_nonnull, context);

        let memory_management = MemoryManagement::new(is_class, &selector, &result_type, modifiers);

        // Related result types.
        // <https://clang.llvm.org/docs/AutomaticReferenceCounting.html#related-result-types>
        let is_related = if is_class {
            matches!(memory_management, MemoryManagement::IdNew)
        } else {
            matches!(memory_management, MemoryManagement::IdInit) || selector == "self"
        };

        if is_related {
            result_type.try_fix_related_result_type();
        }

        if is_error {
            result_type.set_is_error();
        }

        Some((
            modifiers.designated_initializer,
            Method {
                selector,
                fn_name,
                availability,
                is_class,
                is_optional_protocol: entity.is_objc_optional(),
                memory_management,
                arguments,
                result_type,
                safe: !data.unsafe_,
                // Mutable if the parent is mutable is a reasonable default,
                // since immutable methods are usually either declared on an
                // immutable subclass, or as a property.
                mutating: data.mutating.unwrap_or(parent_is_mutable),
                is_protocol,
                non_isolated: modifiers.non_isolated,
                mainthreadonly: modifiers.mainthreadonly,
            },
        ))
    }
}

#[derive(Debug)]
pub struct PartialProperty<'tu> {
    pub entity: Entity<'tu>,
    pub name: String,
    pub getter_name: String,
    pub setter_name: Option<String>,
    pub is_class: bool,
    pub attributes: Option<ObjCAttributes>,
    pub _span: EnteredSpan,
}

impl PartialProperty<'_> {
    pub fn parse(
        self,
        getter_data: MethodData,
        setter_data: Option<MethodData>,
        context: &Context<'_>,
    ) -> (Option<Method>, Option<Method>) {
        let Self {
            entity,
            name,
            getter_name,
            setter_name,
            is_class,
            attributes,
            _span,
        } = self;

        // Early return if both getter and setter are skipped
        //
        // To reduce warnings.
        if getter_data.skipped && setter_data.map(|data| data.skipped).unwrap_or(true) {
            return (None, None);
        }

        let (parent_is_mutable, is_protocol) = Method::parent_type_data(&entity, context);

        let availability = Availability::parse(&entity, context);

        let modifiers = MethodModifiers::parse(&entity, context);

        let is_copy = attributes.map(|a| a.copy).unwrap_or(false);

        if let Some(qualifiers) = entity.get_objc_qualifiers() {
            error!(?qualifiers, "properties do not support qualifiers");
        }

        let getter = if !getter_data.skipped {
            let ty = Ty::parse_property_return(
                entity.get_type().expect("property type"),
                is_copy,
                modifiers.sendable,
                context,
            );

            let memory_management = MemoryManagement::new(is_class, &getter_name, &ty, modifiers);

            Some(Method {
                selector: getter_name.clone(),
                fn_name: getter_name,
                availability: availability.clone(),
                is_class,
                is_optional_protocol: entity.is_objc_optional(),
                memory_management,
                arguments: Vec::new(),
                result_type: ty,
                safe: !getter_data.unsafe_,
                // Getters are usually not mutable, even if the class itself
                // is, so let's default to immutable.
                mutating: getter_data.mutating.unwrap_or(false),
                is_protocol,
                non_isolated: modifiers.non_isolated,
                mainthreadonly: modifiers.mainthreadonly,
            })
        } else {
            None
        };

        let setter = if let Some(setter_name) = setter_name {
            let setter_data = setter_data.expect("setter_data must be present if setter_name was");
            if !setter_data.skipped {
                let ty = Ty::parse_property(
                    entity.get_type().expect("property type"),
                    is_copy,
                    modifiers.sendable,
                    context,
                );

                let selector = setter_name.clone() + ":";
                let memory_management =
                    MemoryManagement::new(is_class, &selector, &Ty::VOID_RESULT, modifiers);

                Some(Method {
                    selector,
                    fn_name: setter_name,
                    availability,
                    is_class,
                    is_optional_protocol: entity.is_objc_optional(),
                    memory_management,
                    arguments: vec![(name, ty)],
                    result_type: Ty::VOID_RESULT,
                    safe: !setter_data.unsafe_,
                    // Setters are usually mutable if the class itself is.
                    mutating: setter_data.mutating.unwrap_or(parent_is_mutable),
                    is_protocol,
                    non_isolated: modifiers.non_isolated,
                    mainthreadonly: modifiers.mainthreadonly,
                })
            } else {
                None
            }
        } else {
            None
        };

        (getter, setter)
    }
}

impl Method {
    pub(crate) fn emit_on_subclasses(&self) -> bool {
        if !self.result_type.is_instancetype() {
            return false;
        }
        if self.is_class {
            true
        } else {
            self.memory_management == MemoryManagement::IdInit
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _span = debug_span!("method", self.fn_name).entered();

        // TODO: Use this somehow?
        // if self.non_isolated {
        //     writeln!(f, "// non_isolated")?;
        // }

        //
        // Attributes
        //

        write!(f, "{}", self.availability)?;

        if self.is_optional_protocol {
            writeln!(f, "        #[optional]")?;
        }

        let id_mm_name = match &self.memory_management {
            MemoryManagement::IdCopyOrMutCopy => Some("CopyOrMutCopy"),
            MemoryManagement::IdNew => Some("New"),
            MemoryManagement::IdInit => Some("Init"),
            MemoryManagement::IdOther => Some("Other"),
            MemoryManagement::Normal => None,
        };
        if let Some(id_mm_name) = id_mm_name {
            write!(f, "        #[method_id(@__retain_semantics {id_mm_name} ")?;
        } else {
            write!(f, "        #[method(")?;
        }
        let error_trailing = if self.result_type.is_error() { "_" } else { "" };
        writeln!(f, "{}{})]", self.selector, error_trailing)?;

        //
        // Signature
        //

        write!(f, "        ")?;
        if !self.is_protocol {
            write!(f, "pub ")?;
        }

        if !self.safe {
            write!(f, "unsafe ")?;
        }
        write!(f, "fn {}(", handle_reserved(&self.fn_name))?;

        // Receiver
        if let MemoryManagement::IdInit = self.memory_management {
            write!(f, "this: Allocated<Self>, ")?;
        } else if self.is_class {
            // Insert nothing; a class method is assumed
        } else if self.mutating {
            write!(f, "&mut self, ")?;
        } else {
            write!(f, "&self, ")?;
        }

        // Arguments
        for (param, arg_ty) in &self.arguments {
            let param = handle_reserved(&crate::to_snake_case(param));
            write!(f, "{param}: {arg_ty}, ")?;
        }
        if self.mainthreadonly {
            write!(f, "mtm: MainThreadMarker")?;
        }
        write!(f, ")")?;

        // Result
        writeln!(f, "{};", self.result_type)?;

        Ok(())
    }
}

pub(crate) fn handle_reserved(name: &str) -> String {
    // try to parse name as an identifier
    if let Ok(ident) = syn::parse_str::<syn::Ident>(name) {
        ident.to_string()
    }
    // try to parse as a raw identifier (NOTE: does not handle `self` or `super`)
    else if let Ok(ident) = syn::parse_str::<syn::Ident>(&format!("r#{name}")) {
        ident.to_string()
    }
    // translate whatever remains unchanged (needed for, e.g., `_`)
    else {
        name.into()
    }
}
