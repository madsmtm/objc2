use std::fmt;

use clang::{Entity, EntityKind, EntityVisitResult, ObjCQualifiers};

use crate::availability::Availability;
use crate::config::MethodData;
use crate::objc2_utils::in_selector_family;
use crate::rust_type::Ty;
use crate::unexposed_macro::UnexposedMacro;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Qualifier {
    In,
    Inout,
    Out,
    Bycopy,
    Byref,
    Oneway,
}

impl Qualifier {
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
            ObjCQualifiers {
                in_: false,
                inout: false,
                out: false,
                bycopy: true,
                byref: false,
                oneway: false,
            } => Self::Bycopy,
            ObjCQualifiers {
                in_: false,
                inout: false,
                out: false,
                bycopy: false,
                byref: true,
                oneway: false,
            } => Self::Byref,
            ObjCQualifiers {
                in_: false,
                inout: false,
                out: false,
                bycopy: false,
                byref: false,
                oneway: true,
            } => Self::Oneway,
            _ => unreachable!("invalid qualifiers: {:?}", qualifiers),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MemoryManagement {
    /// Consumes self and returns retained pointer
    Init,
    ReturnsRetained,
    ReturnsInnerPointer,
    Normal,
}

impl MemoryManagement {
    /// Verifies that the selector and the memory management rules match up
    /// in a way that we can just use `msg_send_id!`.
    fn verify_sel(self, sel: &str) {
        let bytes = sel.as_bytes();
        if in_selector_family(bytes, b"init") {
            assert!(self == Self::Init, "{:?} did not match {}", self, sel);
        } else if in_selector_family(bytes, b"new")
            || in_selector_family(bytes, b"alloc")
            || in_selector_family(bytes, b"copy")
            || in_selector_family(bytes, b"mutableCopy")
        {
            assert!(
                self == Self::ReturnsRetained,
                "{:?} did not match {}",
                self,
                sel
            );
        } else {
            if self == Self::ReturnsInnerPointer {
                return;
            }
            assert!(self == Self::Normal, "{:?} did not match {}", self, sel);
        }
    }

    /// Matches `objc2::__macro_helpers::retain_semantics`.
    fn get_memory_management_name(sel: &str) -> &'static str {
        let bytes = sel.as_bytes();
        match (
            in_selector_family(bytes, b"new"),
            in_selector_family(bytes, b"alloc"),
            in_selector_family(bytes, b"init"),
            in_selector_family(bytes, b"copy"),
            in_selector_family(bytes, b"mutableCopy"),
        ) {
            (true, false, false, false, false) => "New",
            (false, true, false, false, false) => "Alloc",
            (false, false, true, false, false) => "Init",
            (false, false, false, true, false) => "CopyOrMutCopy",
            (false, false, false, false, true) => "CopyOrMutCopy",
            (false, false, false, false, false) => "Other",
            _ => unreachable!(),
        }
    }

    pub fn is_init(sel: &str) -> bool {
        in_selector_family(sel.as_bytes(), b"init")
    }

    pub fn is_alloc(sel: &str) -> bool {
        in_selector_family(sel.as_bytes(), b"alloc")
    }

    pub fn is_new(sel: &str) -> bool {
        in_selector_family(sel.as_bytes(), b"new")
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Method {
    pub selector: String,
    pub fn_name: String,
    pub availability: Availability,
    pub is_class: bool,
    pub is_optional_protocol: bool,
    pub memory_management: MemoryManagement,
    pub designated_initializer: bool,
    pub arguments: Vec<(String, Option<Qualifier>, Ty)>,
    pub result_type: Ty,
    pub safe: bool,
}

impl Method {
    /// Takes one of `EntityKind::ObjCInstanceMethodDecl` or
    /// `EntityKind::ObjCClassMethodDecl`.
    pub fn partial(entity: Entity<'_>) -> PartialMethod<'_> {
        let selector = entity.get_name().expect("method selector");
        let fn_name = selector.trim_end_matches(|c| c == ':').replace(':', "_");

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
        }
    }
}

#[derive(Debug, Clone)]
pub struct PartialMethod<'tu> {
    entity: Entity<'tu>,
    selector: String,
    pub is_class: bool,
    pub fn_name: String,
}

impl<'tu> PartialMethod<'tu> {
    pub fn parse(self, data: MethodData) -> Option<Method> {
        let Self {
            entity,
            selector,
            is_class,
            fn_name,
        } = self;

        // println!("Method {:?}", selector);
        if data.skipped {
            return None;
        }

        if entity.is_variadic() {
            println!("Can't handle variadic method {}", selector);
            return None;
        }

        let availability = Availability::parse(
            entity
                .get_platform_availability()
                .expect("method availability"),
        );

        let arguments: Vec<_> = entity
            .get_arguments()
            .expect("method arguments")
            .into_iter()
            .map(|entity| {
                let name = entity.get_name().expect("arg display name");
                let qualifier = entity.get_objc_qualifiers().map(Qualifier::parse);
                let mut is_consumed = false;

                entity.visit_children(|entity, _parent| {
                    match entity.get_kind() {
                        EntityKind::ObjCClassRef
                        | EntityKind::ObjCProtocolRef
                        | EntityKind::TypeRef
                        | EntityKind::ParmDecl => {
                            // Ignore
                        }
                        EntityKind::NSConsumed => {
                            if is_consumed {
                                panic!("got NSConsumed twice");
                            }
                            is_consumed = true;
                        }
                        EntityKind::UnexposedAttr => {
                            if let Some(macro_) = UnexposedMacro::parse(&entity) {
                                println!("method argument {fn_name}/{name}: {macro_:?}");
                            }
                        }
                        // For some reason we recurse into array types
                        EntityKind::IntegerLiteral => {}
                        _ => panic!("Unknown method argument child: {:?}, {:?}", entity, _parent),
                    };
                    EntityVisitResult::Continue
                });

                let ty = entity.get_type().expect("argument type");
                let ty = Ty::parse_method_argument(ty, is_consumed);

                (name, qualifier, ty)
            })
            .collect();

        if let Some(qualifiers) = entity.get_objc_qualifiers() {
            let qualifier = Qualifier::parse(qualifiers);
            panic!(
                "unexpected qualifier `{:?}` on return type: {:?}",
                qualifier, entity
            );
        }

        let result_type = entity.get_result_type().expect("method return type");
        let mut result_type = Ty::parse_method_return(result_type);
        result_type.fix_related_result_type(is_class, &selector);

        let mut designated_initializer = false;
        let mut consumes_self = false;
        let mut memory_management = MemoryManagement::Normal;

        entity.visit_children(|entity, _parent| {
            match entity.get_kind() {
                EntityKind::ObjCClassRef
                | EntityKind::ObjCProtocolRef
                | EntityKind::TypeRef
                | EntityKind::ParmDecl => {
                    // Ignore
                }
                EntityKind::ObjCDesignatedInitializer => {
                    if designated_initializer {
                        panic!("encountered ObjCDesignatedInitializer twice");
                    }
                    designated_initializer = true;
                }
                EntityKind::NSConsumesSelf => {
                    consumes_self = true;
                }
                EntityKind::NSReturnsRetained => {
                    if memory_management != MemoryManagement::Normal {
                        panic!("got unexpected NSReturnsRetained")
                    }
                    memory_management = MemoryManagement::ReturnsRetained;
                }
                EntityKind::ObjCReturnsInnerPointer => {
                    if memory_management != MemoryManagement::Normal {
                        panic!("got unexpected ObjCReturnsInnerPointer")
                    }
                    memory_management = MemoryManagement::ReturnsInnerPointer;
                }
                EntityKind::NSConsumed => {
                    // Handled inside arguments
                }
                EntityKind::IbActionAttr => {
                    // TODO: What is this?
                }
                EntityKind::ObjCRequiresSuper => {
                    // TODO: Can we use this for something?
                    // <https://clang.llvm.org/docs/AttributeReference.html#objc-requires-super>
                }
                EntityKind::UnexposedAttr => {
                    if let Some(macro_) = UnexposedMacro::parse(&entity) {
                        println!("method {fn_name}: {macro_:?}");
                    }
                }
                _ => panic!("Unknown method child: {:?}, {:?}", entity, _parent),
            };
            // TODO: Verify that Continue is good enough
            EntityVisitResult::Continue
        });

        if consumes_self {
            if memory_management != MemoryManagement::ReturnsRetained {
                panic!("got NSConsumesSelf without NSReturnsRetained");
            }
            memory_management = MemoryManagement::Init;
        }

        // Verify that memory management is as expected
        if result_type.is_id() {
            memory_management.verify_sel(&selector);
        }

        Some(Method {
            selector,
            fn_name,
            availability,
            is_class,
            is_optional_protocol: entity.is_objc_optional(),
            memory_management,
            designated_initializer,
            arguments,
            result_type,
            safe: !data.unsafe_,
        })
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut arguments = self.arguments.clone();

        let is_error =
            if self.selector.ends_with("error:") || self.selector.ends_with("AndReturnError:") {
                let (_, _, ty) = arguments.last().expect("arguments last");
                ty.is_error_out()
            } else {
                false
            };

        if is_error {
            arguments.pop();
        }

        if self.result_type.is_id() {
            writeln!(
                f,
                "        #[method_id(@__retain_semantics {} {})]",
                MemoryManagement::get_memory_management_name(&self.selector),
                self.selector
            )?;
        } else {
            writeln!(f, "        #[method({})]", self.selector)?;
        };

        write!(f, "        pub ")?;
        if !self.safe {
            write!(f, "unsafe ")?;
        }
        write!(f, "fn {}(", handle_reserved(&self.fn_name))?;
        if !self.is_class {
            if MemoryManagement::is_init(&self.selector) {
                write!(f, "this: Option<Allocated<Self>>, ")?;
            } else {
                write!(f, "&self, ")?;
            }
        }
        for (param, _qualifier, arg_ty) in arguments {
            write!(f, "{}: {arg_ty},", handle_reserved(&param))?;
        }
        write!(f, ")")?;

        if is_error {
            writeln!(f, "{};", self.result_type.as_error())?;
        } else {
            if MemoryManagement::is_alloc(&self.selector) {
                assert!(
                    self.result_type.is_alloc(),
                    "{:?}, {:?}",
                    self.result_type,
                    self.fn_name
                );
                writeln!(f, "-> Option<Allocated<Self>>;")?;
            } else {
                writeln!(f, "{};", self.result_type)?;
            }
        };

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
