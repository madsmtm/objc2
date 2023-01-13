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
    designated_initializer: bool,
    returns_inner_pointer: bool,
    consumes_self: bool,
    returns_retained: bool,
    returns_not_retained: bool,
}

impl MethodModifiers {
    fn parse(entity: &Entity<'_>, context: &Context<'_>) -> Self {
        let mut this = Self::default();

        immediate_children(&entity, |entity, _span| match entity.get_kind() {
            EntityKind::UnexposedAttr => {
                if let Some(attr) = UnexposedAttr::parse(&entity, context) {
                    match attr {
                        UnexposedAttr::ReturnsRetained => {
                            this.returns_retained = true;
                        }
                        UnexposedAttr::ReturnsNotRetained => {
                            this.returns_not_retained = true;
                        }
                        attr => error!(?attr, "unknown attribute"),
                    }
                }
            }
            EntityKind::ObjCDesignatedInitializer => {
                this.designated_initializer = true;
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
    ReturnsInnerPointer,
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
        // Which is just:
        // > must return a retainable object pointer
        if result_type.is_id() {
            // We also check that the correct modifier flags were set for the
            // given method family.
            match (
                modifiers.returns_inner_pointer,
                modifiers.consumes_self,
                modifiers.returns_retained,
                modifiers.returns_not_retained,
                id_type,
            ) {
                (false, false, true, false, Self::IdCopyOrMutCopy) => Self::IdCopyOrMutCopy,
                (false, false, true, false, Self::IdNew) => Self::IdNew,
                // For the `init` family there's another restriction:
                // > must be instance methods
                //
                // Note: There's some extra restrictions about a program being
                // "ill-formed" if it contains certain selectors with `init`
                // methods that are not correct super/subclasses, but we don't
                // need to handle that since the header would fail to compile
                // in `clang` if that was the case.
                (false, true, true, false, Self::IdInit) => {
                    if is_class {
                        Self::IdOther
                    } else {
                        Self::IdInit
                    }
                }
                (false, false, false, false, Self::IdOther) => Self::IdOther,
                data => {
                    error!(?data, "invalid MemoryManagement id attributes");
                    Self::IdOther
                }
            }
        } else {
            match (
                modifiers.returns_inner_pointer,
                modifiers.consumes_self,
                modifiers.returns_retained,
                modifiers.returns_not_retained,
            ) {
                (false, false, false, false) => Self::Normal,
                (true, false, false, false) => Self::ReturnsInnerPointer,
                data => {
                    error!(?data, "invalid MemoryManagement attributes");
                    Self::Normal
                }
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Method {
    selector: String,
    pub fn_name: String,
    availability: Availability,
    pub is_class: bool,
    is_optional_protocol: bool,
    memory_management: MemoryManagement,
    arguments: Vec<(String, Ty)>,
    pub result_type: Ty,
    safe: bool,
    mutating: bool,
}

impl Method {
    /// Value that uniquely identifies the method in a class.
    pub fn id(&self) -> (bool, &str) {
        (self.is_class, &self.selector)
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

    pub fn update(mut self, data: MethodData) -> Option<Self> {
        if data.skipped {
            return None;
        }

        self.mutating = data.mutating;
        self.safe = !data.unsafe_;

        Some(self)
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

        let availability = Availability::parse(
            entity
                .get_platform_availability()
                .expect("method availability"),
        );

        let modifiers = MethodModifiers::parse(&entity, context);

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
                            error!(?attr, "unknown attribute");
                        }
                    }
                    // For some reason we recurse into array types
                    EntityKind::IntegerLiteral => {}
                    _ => error!("unknown"),
                });

                let ty = entity.get_type().expect("argument type");
                let ty = Ty::parse_method_argument(ty, qualifier, context);

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
        let mut result_type = Ty::parse_method_return(result_type, context);

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
                mutating: data.mutating,
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

        let availability = Availability::parse(
            entity
                .get_platform_availability()
                .expect("method availability"),
        );

        let modifiers = MethodModifiers::parse(&entity, context);

        let is_copy = attributes.map(|a| a.copy).unwrap_or(false);

        if let Some(qualifiers) = entity.get_objc_qualifiers() {
            error!(?qualifiers, "properties do not support qualifiers");
        }

        let getter = if !getter_data.skipped {
            let ty = Ty::parse_property_return(
                entity.get_type().expect("property type"),
                is_copy,
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
                mutating: getter_data.mutating,
            })
        } else {
            None
        };

        let setter = if let Some(setter_name) = setter_name {
            let setter_data = setter_data.expect("setter_data must be present if setter_name was");
            if !setter_data.skipped {
                let ty =
                    Ty::parse_property(entity.get_type().expect("property type"), is_copy, context);

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
                    mutating: setter_data.mutating,
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
            !matches!(&*self.selector, "new" | "supportsSecureCoding")
        } else {
            self.memory_management == MemoryManagement::IdInit
                && !matches!(&*self.selector, "init" | "initWithCoder:")
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _span = debug_span!("method", self.fn_name).entered();

        //
        // Attributes
        //

        if self.is_optional_protocol {
            writeln!(f, "        #[optional]")?;
        }

        let id_mm_name = match &self.memory_management {
            MemoryManagement::IdCopyOrMutCopy => Some("CopyOrMutCopy"),
            MemoryManagement::IdNew => Some("New"),
            MemoryManagement::IdInit => Some("Init"),
            MemoryManagement::IdOther => Some("Other"),
            MemoryManagement::Normal | MemoryManagement::ReturnsInnerPointer => None,
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

        write!(f, "        pub ")?;
        if !self.safe {
            write!(f, "unsafe ")?;
        }
        write!(f, "fn {}(", handle_reserved(&self.fn_name))?;

        // Receiver
        if let MemoryManagement::IdInit = self.memory_management {
            if self.mutating {
                error!("invalid mutating method");
            }
            write!(f, "this: Option<Allocated<Self>>, ")?;
        } else if self.is_class {
            if self.mutating {
                error!("invalid mutating method");
            }
            // Insert nothing; a class method is assumed
        } else {
            if self.mutating {
                write!(f, "&mut self, ")?;
            } else {
                write!(f, "&self, ")?;
            }
        }

        // Arguments
        for (param, arg_ty) in &self.arguments {
            write!(f, "{}: {arg_ty},", handle_reserved(param))?;
        }
        write!(f, ")")?;

        // Result
        writeln!(f, "{};", self.result_type)?;

        Ok(())
    }
}

pub(crate) fn handle_reserved(s: &str) -> &str {
    match s {
        "type" => "type_",
        "trait" => "trait_",
        "abstract" => "abstract_",
        s => s,
    }
}
