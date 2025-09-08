use core::panic;
use std::fmt;

use clang::{Entity, EntityKind, ObjCAttributes, ObjCQualifiers};

use crate::availability::Availability;
use crate::config::{MethodData, TypeOverride};
use crate::context::Context;
use crate::display_helper::FormatterFn;
use crate::documentation::Documentation;
use crate::id::ItemTree;
use crate::name_translation::is_likely_bounds_affecting;
use crate::objc2_utils::in_selector_family;
use crate::rust_type::{MethodArgumentQualifier, SafetyProperty, Ty};
use crate::thread_safety::ThreadSafety;
use crate::unexposed_attr::UnexposedAttr;
use crate::{immediate_children, Location};

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
pub(crate) struct MethodModifiers {
    returns_inner_pointer: bool,
    consumes_self: bool,
    returns_retained: bool,
    returns_not_retained: bool,
    designated_initializer: bool,
    non_isolated: bool,
    sendable: Option<bool>,
    pub(crate) mainthreadonly: bool,
    must_use: bool,
}

impl MethodModifiers {
    pub(crate) fn parse(entity: &Entity<'_>, context: &Context<'_>) -> Self {
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
                        UnexposedAttr::NoThrow => {
                            // TODO: Use this somehow?
                        }
                        attr => error!(?attr, "unknown attribute on method"),
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
                this.must_use = true;
            }
            EntityKind::ObjCClassRef
            | EntityKind::ObjCProtocolRef
            | EntityKind::TypeRef
            | EntityKind::ParmDecl
            | EntityKind::ObjCNSObject => {
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
            EntityKind::AnnotateAttr => {
                // TODO: `UI_APPEARANCE_SELECTOR`
            }
            kind => error!(?kind, "unknown property child"),
        });

        this
    }
}

/// The retain semantics calling convention for a method.
///
/// This also encodes the "method family" that a method belongs to.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MemoryManagement {
    RetainedAlloc,
    RetainedCopy { returns_not_retained: bool },
    RetainedMutableCopy { returns_not_retained: bool },
    RetainedNew { returns_not_retained: bool },
    RetainedInit,
    RetainedNone { returns_retained: bool },
    InnerPointer,
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
        let selector_family = match (
            in_selector_family(bytes, b"alloc"),
            in_selector_family(bytes, b"copy"),
            in_selector_family(bytes, b"mutableCopy"),
            in_selector_family(bytes, b"new"),
            in_selector_family(bytes, b"init"),
        ) {
            (true, false, false, false, false) => "alloc",
            (false, true, false, false, false) => "copy",
            (false, false, true, false, false) => "mutableCopy",
            (false, false, false, true, false) => "new",
            (false, false, false, false, true) => "init",
            (false, false, false, false, false) => "none",
            _ => unreachable!(),
        };

        // And if:
        // > its signature obeys the added restrictions of the method family.
        //
        // Which is:
        // > must return a retainable object pointer
        if result_type.is_retainable() {
            // We also check that the correct modifier flags were set for the
            // given method family.
            match (
                modifiers.returns_inner_pointer,
                modifiers.consumes_self,
                modifiers.returns_retained,
                modifiers.returns_not_retained,
                modifiers.designated_initializer,
                selector_family,
            ) {
                (false, false, true, false, false, "alloc") => {
                    // It's not really worth the effort to support these, since
                    // they're only defined on `NSObject` and `NSProxy`, and we
                    // have it in `ClassType` anyhow.
                    error!("the `alloc` method-family requires manual handling");
                    Self::RetainedAlloc
                }
                (false, false, true, false, false, "copy") => Self::RetainedCopy {
                    returns_not_retained: false,
                },
                (false, false, false, true, false, "copy") => Self::RetainedCopy {
                    returns_not_retained: true,
                },
                (false, false, true, false, false, "mutableCopy") => Self::RetainedMutableCopy {
                    returns_not_retained: false,
                },
                (false, false, false, true, false, "mutableCopy") => Self::RetainedMutableCopy {
                    returns_not_retained: true,
                },
                (false, false, true, false, false, "new") => Self::RetainedNew {
                    returns_not_retained: false,
                },
                (false, false, false, true, false, "new") => Self::RetainedNew {
                    returns_not_retained: true,
                },
                (false, false, false, false, false, "new") => Self::RetainedNone {
                    returns_retained: false,
                },
                // For the `init` family there's another restriction:
                // > must be instance methods
                //
                // Note: There's some extra restrictions about a program being
                // "ill-formed" if it contains certain selectors with `init`
                // methods that are not correct super/subclasses, but we don't
                // need to handle that since the header would fail to compile
                // in `clang` if that was the case.
                (false, true, true, false, _, "init") => {
                    if is_class {
                        // TODO: Is this actually correct, or should we still
                        // emit #[unsafe(method_family = init)] here?
                        Self::RetainedNone {
                            returns_retained: false,
                        }
                    } else {
                        Self::RetainedInit
                    }
                }
                // Don't care if the user specified CF_RETURNS_NOT_RETAINED,
                // that is the default for this selector anyhow.
                (false, false, returns_retained, _, false, "none") => {
                    Self::RetainedNone { returns_retained }
                }
                (true, false, false, _, false, "none") => Self::InnerPointer,
                data => {
                    error!(?data, "invalid MemoryManagement retainable attributes");
                    Self::RetainedNone {
                        returns_retained: false,
                    }
                }
            }
        } else if let MethodModifiers {
            // TODO: Maybe we can use this to emit things with lifetime of:
            // `'self + 'autoreleasepool`
            returns_inner_pointer: _,
            consumes_self: false,
            returns_retained: false,
            returns_not_retained: false,
            designated_initializer: false,
            ..
        } = modifiers
        {
            Self::Normal
        } else {
            warn!(?modifiers, "invalid MemoryManagement attributes");
            Self::Normal
        }
    }
}

/// <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjectiveC/Chapters/ocProperties.html>
/// <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ProgrammingWithObjectiveC/EncapsulatingData/EncapsulatingData.html#//apple_ref/doc/uid/TP40011210-CH5-SW3>.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub enum PropertyKind {
    /// By default, properties are retained.
    #[default]
    Normal,
    /// The object is copied into the property when set. These are retained.
    Copy,
    /// The object is weakly referenced by the property.
    Weak,
    /// Whether the property is marked as not internally retained,
    /// neither strongly nor weakly.
    UnsafeRetained,
}

impl PropertyKind {
    fn parse(attrs: Option<ObjCAttributes>) -> Self {
        let Some(attrs) = attrs else {
            return Self::Normal;
        };

        let retained = attrs.retain || attrs.strong;

        let unsafe_retained = attrs.assign || attrs.unsafe_retained;

        match (retained, attrs.copy, attrs.weak, unsafe_retained) {
            (true, false, false, false) => Self::Normal,
            (false, true, false, false) => Self::Copy,
            (false, false, true, false) => Self::Weak,
            (false, false, false, true) => Self::UnsafeRetained,
            (false, false, false, false) => Self::Normal,
            _ => {
                error!(?attrs, "unclear property attributes");
                Self::Normal
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Method {
    pub selector: String,
    pub fn_name: String,
    pub availability: Availability,
    pub is_class: bool,
    is_optional: bool,
    memory_management: MemoryManagement,
    arguments: Vec<(String, Ty)>,
    result_type: Ty,
    safe: bool,
    is_pub: bool,
    // Thread-safe, even on main-thread only (@MainActor/@UIActor) classes
    non_isolated: bool,
    mainthreadonly: bool,
    must_use: bool,
    encoding: String,
    documentation: Documentation,
}

#[derive(Debug)]
pub struct PartialProperty<'tu> {
    pub entity: Entity<'tu>,
    pub name: String,
    pub getter_sel: String,
    pub setter_sel: Option<String>,
    pub is_class: bool,
    pub attributes: Option<ObjCAttributes>,
}

fn mainthreadonly_override<'a>(
    result_type: &Ty,
    argument_types: impl IntoIterator<Item = &'a Ty>,
    parent_is_mainthreadonly: bool,
    is_class: bool,
    mainthreadonly_modifier: bool,
) -> bool {
    let mut result_type_requires_mainthreadmarker =
        result_type.requires_mainthreadmarker(parent_is_mainthreadonly);

    let mut any_argument_provides_mainthreadmarker = argument_types
        .into_iter()
        .any(|arg_ty| arg_ty.provides_mainthreadmarker(parent_is_mainthreadonly));

    if parent_is_mainthreadonly {
        if is_class {
            // Assume the method needs main thread if it's
            // declared on a main thread only class.
            result_type_requires_mainthreadmarker = true;
        } else {
            // Method takes `&self` or `&mut self`, or is
            // an initialization method, all of which
            // already require the main thread.
            //
            // Note: Initialization methods can be passed
            // `None`, but in that case the return will
            // always be NULL.
            any_argument_provides_mainthreadmarker = true;
        }
    }

    if any_argument_provides_mainthreadmarker {
        // MainThreadMarker can be retrieved via `MainThreadOnly::mtm`
        // inside these methods, and hence passing it is redundant.
        false
    } else if result_type_requires_mainthreadmarker {
        true
    } else {
        // If neither, then we respect any annotation
        // the method may have had before
        mainthreadonly_modifier
    }
}

impl Method {
    /// Value that uniquely identifies the method in a class.
    pub fn id(&self) -> (bool, String) {
        (self.is_class, self.selector.clone())
    }

    pub(crate) fn usable_in_default_retained(&self) -> bool {
        self.selector == "new"
            && self.is_class
            && self.arguments.is_empty()
            && self.safe
            && !self.mainthreadonly
            && self.availability.is_available_non_deprecated()
    }

    /// Takes `EntityKind::ObjCPropertyDecl`.
    pub(crate) fn partial_property(entity: Entity<'_>) -> PartialProperty<'_> {
        let attributes = entity.get_objc_attributes();
        let has_setter = attributes.map(|a| !a.readonly).unwrap_or(true);

        let name = entity.get_display_name().expect("property name");

        PartialProperty {
            entity,
            name,
            getter_sel: entity.get_objc_getter_name().expect("property getter name"),
            setter_sel: has_setter.then(|| {
                entity
                    .get_objc_setter_name()
                    .expect("property setter name")
                    .to_string()
            }),
            is_class: attributes.map(|a| a.class).unwrap_or(false),
            attributes,
        }
    }

    pub(crate) fn parse_method(
        entity: Entity<'_>,
        data: MethodData,
        parent_thread_safety: &ThreadSafety,
        is_pub: bool,
        context: &Context<'_>,
    ) -> Option<(bool, Method)> {
        let selector = entity.get_name().expect("method selector");
        let _span = debug_span!("method", selector).entered();

        // TODO: Strip these from function name?
        // selector.ends_with("error:")
        // || selector.ends_with("AndReturnError:")
        // || selector.ends_with("WithError:")

        if data.skipped {
            return None;
        }

        let is_class = match entity.get_kind() {
            EntityKind::ObjCInstanceMethodDecl => false,
            EntityKind::ObjCClassMethodDecl => true,
            _ => unreachable!("unknown method kind"),
        };

        // Don't emit memory-management methods
        match &*selector {
            // Available via `ClassType`
            "alloc" | "allocWithZone:" if is_class => {
                return None;
            }
            // Available via `Retained` (and disallowed by ARC).
            "retain" | "release" | "autorelease" | "dealloc" if !is_class => {
                return None;
            }
            _ => {}
        }

        if entity.is_variadic() {
            debug!(?selector, "can't handle variadic method");
            return None;
        }

        let availability = Availability::parse(&entity, context);

        let modifiers = MethodModifiers::parse(&entity, context);

        if modifiers.sendable.is_some() {
            error!("sendable on method");
        }

        let arguments: Vec<_> = entity
            .get_arguments()
            .expect("method arguments")
            .into_iter()
            .enumerate()
            .map(|(index, entity)| {
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
                    // `ns_consumed`, `cf_consumed` and `os_consumed`
                    EntityKind::NSConsumed => {
                        error!("found NSConsumed, which requires manual handling");
                    }
                    EntityKind::UnexposedAttr => {
                        if let Some(attr) = UnexposedAttr::parse(&entity, context) {
                            match attr {
                                UnexposedAttr::Sendable => sendable = Some(true),
                                UnexposedAttr::NonSendable => sendable = Some(false),
                                UnexposedAttr::NoEscape => no_escape = true,
                                attr => error!(?attr, "unknown attribute on method argument"),
                            }
                        }
                    }
                    // For some reason we recurse into array types
                    EntityKind::IntegerLiteral => {}
                    _ => error!("unknown"),
                });

                let ty = entity.get_type().expect("argument type");
                let mut ty = Ty::parse_method_argument(ty, qualifier, sendable, no_escape, context);

                if let Some(ty_or) = data.arguments.get(&index) {
                    apply_type_override(&mut ty, ty_or);
                }

                (name, ty)
            })
            .collect();

        let last_arg_override = *data.arguments.keys().max().unwrap_or(&0);
        if arguments.len() < last_arg_override {
            panic!(
                "argument override index out of bounds {}",
                last_arg_override
            );
        }

        if let Some(qualifiers) = entity.get_objc_qualifiers() {
            error!(?qualifiers, "unsupported qualifiers on return type");
        }

        let result_type = entity.get_result_type().expect("method return type");
        let default_nonnull = (selector == "init" && !is_class) || (selector == "new" && is_class);
        let mut result_type = Ty::parse_method_return(result_type, default_nonnull, context);

        if result_type.needs_simd() || arguments.iter().any(|(_, arg_ty)| arg_ty.needs_simd()) {
            debug!("simd types are not yet possible in methods");
            return None;
        }

        let memory_management = MemoryManagement::new(is_class, &selector, &result_type, modifiers);

        // Related result types.
        // <https://clang.llvm.org/docs/AutomaticReferenceCounting.html#related-result-types>
        let is_related = if is_class {
            matches!(memory_management, MemoryManagement::RetainedNew { .. })
        } else {
            matches!(memory_management, MemoryManagement::RetainedInit) || selector == "self"
        };

        if is_related {
            result_type.try_fix_related_result_type();
        }

        let fn_name = selector.trim_end_matches(':').replace(':', "_");

        let mainthreadonly = mainthreadonly_override(
            &result_type,
            arguments.iter().map(|(_, ty)| ty),
            parent_thread_safety.inferred_mainthreadonly(),
            is_class,
            modifiers.mainthreadonly,
        );

        apply_type_override(&mut result_type, &data.return_);

        let encoding = entity
            .get_objc_type_encoding()
            .expect("method to have encoding");

        let safety = arguments
            .iter()
            .fold(SafetyProperty::Safe, |safety, (_, arg_ty)| {
                safety.merge(arg_ty.safety_in_method_argument())
            })
            .merge(result_type.safety_in_fn_return());

        let default_safety = &context
            .library(Location::from_entity(&entity, context).unwrap())
            .default_safety;

        let default_safe = if is_class || memory_management == MemoryManagement::RetainedInit {
            default_safety.class_methods
        } else {
            default_safety.instance_methods
        };

        let safe = if let Some(unsafe_) = data.unsafe_ {
            if safety == SafetyProperty::Unsafe && !unsafe_ {
                // TODO(breaking): Disallow these.
                error!(?selector, ?arguments, "unsafe method was marked as safe");
            }
            !unsafe_
        } else if safety == SafetyProperty::Safe && default_safe {
            if default_safety.not_bounds_affecting {
                !is_likely_bounds_affecting(&selector)
                    && arguments
                        .iter()
                        .all(|(arg_name, _)| !is_likely_bounds_affecting(arg_name))
            } else {
                true
            }
        } else {
            false
        };

        Some((
            modifiers.designated_initializer,
            Method {
                selector,
                fn_name,
                availability,
                is_class,
                is_optional: entity.is_objc_optional(),
                memory_management,
                arguments,
                result_type,
                safe,
                is_pub,
                non_isolated: modifiers.non_isolated,
                mainthreadonly,
                must_use: modifiers.must_use,
                encoding,
                documentation: Documentation::from_entity(&entity, context),
            },
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn parse_property(
        property: PartialProperty<'_>,
        getter_data: MethodData,
        setter_data: Option<MethodData>,
        parent_thread_safety: &ThreadSafety,
        is_pub: bool,
        context: &Context<'_>,
    ) -> (Option<Method>, Option<Method>) {
        let PartialProperty {
            entity,
            name,
            getter_sel,
            setter_sel,
            is_class,
            attributes,
        } = property;
        let _span = debug_span!("property", name).entered();

        // Early return if both getter and setter are skipped
        //
        // To reduce warnings.
        if getter_data.skipped
            && setter_data
                .as_ref()
                .map(|data| data.skipped)
                .unwrap_or(true)
        {
            return (None, None);
        }

        let availability = Availability::parse(&entity, context);

        let modifiers = MethodModifiers::parse(&entity, context);

        let kind = PropertyKind::parse(attributes);

        // Properties are atomic by default, but only if synthethized. It is
        // unclear if properties that `!nonatomic` are guaranteed to be
        // atomic, see https://github.com/madsmtm/objc2/issues/757.
        //
        // As such, we consider atomic-ness a `Option<bool>`, where `None`
        // means "not specified".
        let atomic = match (
            attributes.map(|a| a.atomic).unwrap_or(false),
            attributes.map(|a| a.nonatomic).unwrap_or(false),
        ) {
            (false, false) => None,
            (true, false) => Some(true),
            (false, true) => Some(false),
            (true, true) => {
                error!("a property cannot both be atomic and nonatomic");
                None
            }
        };

        let default_safety = &context
            .library(Location::from_entity(&entity, context).unwrap())
            .default_safety;

        if let Some(qualifiers) = entity.get_objc_qualifiers() {
            error!(?qualifiers, "properties do not support qualifiers");
        }

        let encoding = entity
            .get_objc_type_encoding()
            .expect("method to have encoding");

        let getter = if !getter_data.skipped {
            let mut ty = Ty::parse_property_getter(
                entity.get_type().expect("property type"),
                kind == PropertyKind::Copy,
                modifiers.sendable,
                context,
            );

            if kind == PropertyKind::Copy && ty.is_class_with_mutable_in_name() {
                // Unclear what the semantics are here? How can this be of
                // the correct type if the type is immutably copied?
                warn!(?getter_sel, "property is both `copy` and a mutable object");
            }

            if ty.needs_simd() {
                debug!("simd types are not yet possible in properties");
                return (None, None);
            }

            let memory_management = MemoryManagement::new(is_class, &getter_sel, &ty, modifiers);

            let mainthreadonly = mainthreadonly_override(
                &ty,
                &[],
                parent_thread_safety.inferred_mainthreadonly(),
                is_class,
                modifiers.mainthreadonly,
            );

            apply_type_override(&mut ty, &getter_data.return_);

            let mut documentation = Documentation::from_entity(&entity, context);

            let mut safety = ty.safety_in_fn_return();
            if kind == PropertyKind::UnsafeRetained && !ty.is_primitive_or_record() {
                // We cannot mark these as safe, since the pointer is not
                // guaranteed to remain valid while being stored inside the
                // class.
                safety = SafetyProperty::Unsafe;
            }
            if atomic == Some(false)
                && parent_thread_safety.inferred_sendable()
                && !(is_class && setter_sel.is_none())
            {
                // `nonatomic` properties on sendable classes are not safe
                // by default (unless they are readonly class-properties, then
                // the nonatomic-ness in the header is probably incorrect,
                // since they are global).
                safety = safety.merge(SafetyProperty::Unknown);
                documentation.add("This property is not atomic.");
                // TODO: Should we document atomic-ness in further cases?
            };

            let safe = if let Some(unsafe_) = getter_data.unsafe_ {
                if safety == SafetyProperty::Unsafe && !unsafe_ {
                    // TODO(breaking): Disallow these.
                    error!(?getter_sel, ?ty, "unsafe property was marked as safe");
                }
                !unsafe_
            } else if safety == SafetyProperty::Safe && default_safety.property_getters {
                true // Also if bounds affecting
            } else {
                false
            };

            if kind == PropertyKind::UnsafeRetained && ty.is_object_like_ptr() {
                documentation.add("# Safety");
                documentation.add(
                    "This is not retained internally, you must ensure the object is still alive.",
                );
            }

            Some(Method {
                selector: getter_sel.clone(),
                fn_name: getter_sel.clone(),
                availability: availability.clone(),
                is_class,
                is_optional: entity.is_objc_optional(),
                memory_management,
                arguments: Vec::new(),
                result_type: ty,
                safe,
                is_pub,
                non_isolated: modifiers.non_isolated,
                mainthreadonly,
                must_use: modifiers.must_use,
                encoding: encoding.clone(),
                documentation,
            })
        } else {
            None
        };

        let setter = if let Some(selector) = setter_sel {
            let setter_data = setter_data.expect("setter_data must be present if setter_sel was");
            if !setter_data.skipped {
                let result_type = Ty::VOID_RESULT;
                let mut ty = Ty::parse_property_setter(
                    entity.get_type().expect("property type"),
                    kind == PropertyKind::Copy,
                    modifiers.sendable,
                    context,
                );

                let fn_name = selector.strip_suffix(':').unwrap().to_string();
                let memory_management =
                    MemoryManagement::new(is_class, &selector, &result_type, modifiers);

                let mainthreadonly = mainthreadonly_override(
                    &result_type,
                    std::iter::once(&ty),
                    parent_thread_safety.inferred_mainthreadonly(),
                    is_class,
                    modifiers.mainthreadonly,
                );

                if let Some(ty_or) = setter_data.arguments.get(&0) {
                    apply_type_override(&mut ty, ty_or);
                }

                let mut safety = ty.safety_in_fn_argument();
                if kind == PropertyKind::UnsafeRetained && !ty.is_primitive_or_record() {
                    // We could _possibly_ allow these to be safe, but let's
                    // not for now, they interact weirdly with the getters
                    // (which have to be unsafe).
                    safety = SafetyProperty::Unsafe;
                }
                if atomic == Some(false) && parent_thread_safety.inferred_sendable() {
                    // `nonatomic` properties on sendable classes are not safe
                    // by default.
                    safety = safety.merge(SafetyProperty::Unknown);
                };

                let safe = if let Some(unsafe_) = setter_data.unsafe_ {
                    if safety == SafetyProperty::Unsafe && !unsafe_ {
                        // TODO(breaking): Disallow these.
                        error!(?selector, ?ty, "unsafe property setter was marked as safe");
                    }
                    !unsafe_
                } else if safety == SafetyProperty::Safe && default_safety.property_setters {
                    if default_safety.not_bounds_affecting {
                        !is_likely_bounds_affecting(&selector)
                    } else {
                        true
                    }
                } else {
                    false
                };

                // Do not emit normal docs on setters, otherwise we'd be
                // duplicating the documentation across getter and setter.
                let mut documentation = Documentation::empty();
                documentation.add(format!("Setter for [`{getter_sel}`][Self::{getter_sel}]."));

                // Only show `weak`-ness and `copy`-ness on setters (that's
                // where it's most relevant).
                match kind {
                    PropertyKind::Copy => {
                        if context.current_library == "Foundation" {
                            documentation.add("This is [copied][crate::NSCopying::copy] when set.");
                        } else {
                            documentation.add(
                                "This is [copied][objc2_foundation::NSCopying::copy] when set.",
                            );
                        }
                    }
                    PropertyKind::Weak => {
                        documentation
                            .add("This is a [weak property][objc2::topics::weak_property].");
                    }
                    _ => {}
                }

                Some(Method {
                    selector,
                    fn_name,
                    availability,
                    is_class,
                    is_optional: entity.is_objc_optional(),
                    memory_management,
                    arguments: vec![(name, ty)],
                    result_type,
                    safe,
                    is_pub,
                    non_isolated: modifiers.non_isolated,
                    mainthreadonly,
                    must_use: modifiers.must_use,
                    encoding,
                    documentation,
                })
            } else {
                None
            }
        } else {
            None
        };

        (getter, setter)
    }

    pub(crate) fn emit_on_subclasses(&self) -> bool {
        if !self.result_type.is_instancetype() {
            return false;
        }
        if self.is_class {
            true
        } else {
            self.memory_management == MemoryManagement::RetainedInit
        }
    }

    pub(crate) fn required_items(&self) -> impl Iterator<Item = ItemTree> {
        let mut items = Vec::new();
        for (_, arg_ty) in &self.arguments {
            items.extend(arg_ty.required_items());
        }
        items.extend(self.result_type.required_items());
        if self.mainthreadonly {
            items.push(ItemTree::main_thread_marker());
        }
        items.into_iter()
    }

    pub(crate) fn encoding_test(&self, is_protocol: bool) -> impl fmt::Display + '_ {
        FormatterFn(move |f| {
            let check = self.availability.check_is_available();

            if let Some(check) = &check {
                writeln!(f, "if {check} {{")?;
            }

            if is_protocol {
                writeln!(f, "assert!(true); // TODO")?;
            } else {
                write!(f, "check_method::<(")?;
                for (_, arg_ty) in &self.arguments {
                    write!(f, "{},", arg_ty.method_argument_encoding_type())?;
                }
                write!(f, "), {}>(", self.result_type.method_return_encoding_type())?;
                if self.is_class {
                    write!(f, "metaclass")?;
                } else {
                    write!(f, "cls")?;
                }
                writeln!(f, ", sel!({}), {:?});", self.selector, self.encoding)?;
            }

            if check.is_some() {
                writeln!(f, "}}")?;
            }

            Ok(())
        })
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _span = debug_span!("method", self.fn_name).entered();

        let mut arguments = &self.arguments[..];
        let error_return = if let Some(((_, ty), rest)) = arguments.split_last() {
            if ty.argument_is_error_out() {
                if let Some(error_return) = self.result_type.method_return_with_error() {
                    arguments = rest;
                    Some(error_return)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        // TODO: Use this somehow?
        // if self.non_isolated {
        //     writeln!(f, "// non_isolated")?;
        // }

        //
        // Attributes
        //

        write!(f, "{}", self.documentation.fmt(None))?;
        write!(f, "{}", self.availability)?;

        if self.must_use {
            writeln!(f, "        #[must_use]")?;
        }

        if self.is_optional {
            writeln!(f, "        #[optional]")?;
        }

        let error_trailing = if error_return.is_some() { "_" } else { "" };
        writeln!(
            f,
            "        #[unsafe(method({}{}))]",
            self.selector, error_trailing
        )?;

        let (method_family, attr) = match &self.memory_management {
            MemoryManagement::RetainedAlloc => ("alloc", None),
            MemoryManagement::RetainedCopy {
                returns_not_retained: false,
            } => ("copy", None),
            MemoryManagement::RetainedCopy {
                returns_not_retained: true,
            } => ("none", Some("returns_not_retained")),
            MemoryManagement::RetainedMutableCopy {
                returns_not_retained: false,
            } => ("mutableCopy", None),
            MemoryManagement::RetainedMutableCopy {
                returns_not_retained: true,
            } => ("none", Some("returns_not_retained")),
            MemoryManagement::RetainedNew {
                returns_not_retained: false,
            } => ("new", None),
            MemoryManagement::RetainedNew {
                returns_not_retained: true,
            } => ("none", Some("returns_not_retained")),
            MemoryManagement::RetainedInit => ("init", None),
            MemoryManagement::RetainedNone {
                returns_retained: false,
            } => ("none", None),
            MemoryManagement::RetainedNone {
                returns_retained: true,
            } => ("copy", Some("returns_retained")),
            MemoryManagement::InnerPointer => ("none", None),
            MemoryManagement::Normal => ("none", None),
        };
        if let Some(attr) = attr {
            writeln!(
                f,
                "        // required for soundness, method has `{attr}` attribute."
            )?;
        }
        writeln!(f, "        #[unsafe(method_family = {method_family})]")?;

        //
        // Signature
        //

        write!(f, "        ")?;
        if self.is_pub {
            write!(f, "pub ")?;
        }

        if !self.safe {
            write!(f, "unsafe ")?;
        }
        write!(f, "fn {}(", handle_reserved(&self.fn_name))?;

        // Receiver
        if let MemoryManagement::RetainedInit = self.memory_management {
            write!(f, "this: Allocated<Self>, ")?;
        } else if self.is_class {
            // Insert nothing; a class method is assumed
        } else {
            write!(f, "&self, ")?;
        }

        // Arguments
        for (param, arg_ty) in arguments {
            let param = handle_reserved(&crate::to_snake_case(param));
            write!(f, "{param}: {}, ", arg_ty.method_argument())?;
        }
        if self.mainthreadonly {
            write!(f, "mtm: MainThreadMarker")?;
        }
        write!(f, ")")?;

        // Result
        if let MemoryManagement::InnerPointer = self.memory_management {
            write!(f, "{}", self.result_type.method_return_inner_pointer())?;
        } else if let Some(error_return) = error_return {
            write!(f, "{error_return}")?;
        } else {
            write!(f, "{}", self.result_type.method_return())?;
        }
        writeln!(f, ";")?;

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
    else if name == "self" {
        "self_".into()
    } else if name == "Self" {
        "r#Self".into()
    } else if name == "super" {
        "super_".into()
    } else if name == "_" {
        // Hack: Assumes that there are not other fields / parameters with the
        // name `_`.
        "param1".into()
    } else {
        name.into()
    }
}

pub(crate) fn apply_type_override(ty: &mut Ty, or: &TypeOverride) {
    if let Some(nullability) = or.nullability {
        ty.change_nullability(nullability.into());
    }
}
