use std::collections::HashSet;
use std::fmt;

use clang::{Entity, EntityKind, EntityVisitResult, TypeKind};

use crate::availability::Availability;
use crate::config::{ClassData, Config};
use crate::expr::Expr;
use crate::method::Method;
use crate::property::Property;
use crate::rust_type::{GenericType, RustType, RustTypeReturn, RustTypeStatic};
use crate::unexposed_macro::UnexposedMacro;

#[derive(Debug, Clone)]
pub enum MethodOrProperty {
    Method(Method),
    Property(Property),
}

impl fmt::Display for MethodOrProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Method(method) => write!(f, "{method}"),
            Self::Property(property) => write!(f, "{property}"),
        }
    }
}

/// Takes one of:
/// - `EntityKind::ObjCInterfaceDecl`
/// - `EntityKind::ObjCProtocolDecl`
/// - `EntityKind::ObjCCategoryDecl`
fn parse_objc_decl(
    entity: &Entity<'_>,
    mut superclass: Option<&mut Option<Option<String>>>,
    mut generics: Option<&mut Vec<String>>,
    class_data: Option<&ClassData>,
) -> (Vec<String>, Vec<MethodOrProperty>) {
    let mut protocols = Vec::new();
    let mut methods = Vec::new();

    // Track seen properties, so that when methods are autogenerated by the
    // compiler from them, we can skip them
    let mut properties = HashSet::new();

    entity.visit_children(|entity, _parent| {
        match entity.get_kind() {
            EntityKind::ObjCExplicitProtocolImpl if generics.is_none() && superclass.is_none() => {
                // TODO NS_PROTOCOL_REQUIRES_EXPLICIT_IMPLEMENTATION
            }
            EntityKind::ObjCIvarDecl if superclass.is_some() => {
                // Explicitly ignored
            }
            EntityKind::ObjCSuperClassRef => {
                if let Some(superclass) = &mut superclass {
                    **superclass = Some(Some(entity.get_name().expect("superclass name")));
                } else {
                    panic!("unsupported superclass {entity:?}");
                }
            }
            EntityKind::ObjCRootClass => {
                if let Some(superclass) = &mut superclass {
                    // TODO: Maybe just skip root classes entirely?
                    **superclass = Some(None);
                } else {
                    panic!("unsupported root class {entity:?}");
                }
            }
            EntityKind::ObjCClassRef if generics.is_some() => {
                // println!("ObjCClassRef: {:?}", entity.get_display_name());
            }
            EntityKind::TemplateTypeParameter => {
                if let Some(generics) = &mut generics {
                    // TODO: Generics with bounds (like NSMeasurement<UnitType: NSUnit *>)
                    // let ty = entity.get_type().expect("template type");
                    let name = entity.get_display_name().expect("template name");
                    generics.push(name);
                } else {
                    panic!("unsupported generics {entity:?}");
                }
            }
            EntityKind::ObjCProtocolRef => {
                protocols.push(entity.get_name().expect("protocolref to have name"));
            }
            EntityKind::ObjCInstanceMethodDecl | EntityKind::ObjCClassMethodDecl => {
                let partial = Method::partial(entity);

                if !properties.remove(&(partial.is_class, partial.fn_name.clone())) {
                    let data = class_data
                        .map(|class_data| {
                            class_data
                                .methods
                                .get(&partial.fn_name)
                                .copied()
                                .unwrap_or_default()
                        })
                        .unwrap_or_default();
                    if let Some(method) = partial.parse(data) {
                        methods.push(MethodOrProperty::Method(method));
                    }
                }
            }
            EntityKind::ObjCPropertyDecl => {
                let partial = Property::partial(entity);
                let data = class_data
                    .map(|class_data| {
                        class_data
                            .properties
                            .get(&partial.name)
                            .copied()
                            .unwrap_or_default()
                    })
                    .unwrap_or_default();

                assert!(
                    properties.insert((partial.is_class, partial.getter_name.clone())),
                    "already exisiting property"
                );
                if let Some(setter_name) = partial.setter_name.clone() {
                    assert!(
                        properties.insert((partial.is_class, setter_name)),
                        "already exisiting property"
                    );
                }
                if let Some(property) = partial.parse(data) {
                    methods.push(MethodOrProperty::Property(property));
                }
            }
            EntityKind::VisibilityAttr if superclass.is_some() => {
                // NS_CLASS_AVAILABLE_MAC??
                println!("TODO: VisibilityAttr")
            }
            EntityKind::TypeRef if superclass.is_some() => {
                // TODO
            }
            EntityKind::ObjCException if superclass.is_some() => {
                // Maybe useful for knowing when to implement `Error` for the type
            }
            EntityKind::UnexposedAttr => {
                if let Some(macro_) = UnexposedMacro::parse(&entity) {
                    println!("objc decl {entity:?}: {macro_:?}");
                }
            }
            _ => panic!("unknown objc decl child {entity:?}"),
        };
        EntityVisitResult::Continue
    });

    if !properties.is_empty() {
        if properties == HashSet::from([(false, "setDisplayName".to_owned())]) {
            // TODO
        } else {
            panic!("did not properly add methods to properties:\n{methods:?}\n{properties:?}");
        }
    }

    (protocols, methods)
}

#[derive(Debug, Clone)]
pub enum Stmt {
    /// @interface name: superclass <protocols*>
    ClassDecl {
        name: String,
        availability: Availability,
        // TODO: Generics
        superclass: Option<String>,
        generics: Vec<String>,
        protocols: Vec<String>,
        methods: Vec<MethodOrProperty>,
    },
    /// @interface class_name (name) <protocols*>
    CategoryDecl {
        class_name: String,
        availability: Availability,
        /// Some categories don't have a name. Example: NSClipView
        name: Option<String>,
        generics: Vec<String>,
        /// I don't quite know what this means?
        protocols: Vec<String>,
        methods: Vec<MethodOrProperty>,
    },
    /// @protocol name <protocols*>
    ProtocolDecl {
        name: String,
        availability: Availability,
        protocols: Vec<String>,
        methods: Vec<MethodOrProperty>,
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
        name: Option<String>,
        ty: RustType,
        kind: Option<UnexposedMacro>,
        variants: Vec<(String, Expr)>,
    },
    /// static const ty name = expr;
    /// extern const ty name;
    VarDecl {
        name: String,
        ty: RustTypeStatic,
        value: Option<Option<Expr>>,
    },
    /// extern ret name(args*);
    ///
    /// static inline ret name(args*) {
    ///     body
    /// }
    FnDecl {
        name: String,
        arguments: Vec<(String, RustType)>,
        result_type: RustTypeReturn,
        // Some -> inline function.
        body: Option<()>,
    },
    /// typedef Type TypedefName;
    AliasDecl { name: String, type_: RustType },
    // /// typedef struct Name { fields } TypedefName;
    // X,
}

impl Stmt {
    pub fn parse(entity: &Entity<'_>, config: &Config) -> Option<Self> {
        match entity.get_kind() {
            // These are inconsequential for us, since we resolve imports differently
            EntityKind::ObjCClassRef | EntityKind::ObjCProtocolRef => None,
            EntityKind::ObjCInterfaceDecl => {
                // entity.get_mangled_objc_names()
                let name = entity.get_name().expect("class name");
                let class_data = config.class_data.get(&name);
                let availability = Availability::parse(
                    entity
                        .get_platform_availability()
                        .expect("class availability"),
                );
                // println!("Availability: {:?}", entity.get_platform_availability());
                let mut superclass = None;
                let mut generics = Vec::new();

                let (protocols, methods) = parse_objc_decl(
                    &entity,
                    Some(&mut superclass),
                    Some(&mut generics),
                    class_data,
                );

                let superclass = superclass.expect("no superclass found");

                Some(Self::ClassDecl {
                    name,
                    availability,
                    superclass,
                    generics,
                    protocols,
                    methods,
                })
            }
            EntityKind::ObjCCategoryDecl => {
                let name = entity.get_name();
                let availability = Availability::parse(
                    entity
                        .get_platform_availability()
                        .expect("category availability"),
                );

                let mut class_name = None;
                entity.visit_children(|entity, _parent| {
                    if entity.get_kind() == EntityKind::ObjCClassRef {
                        if class_name.is_some() {
                            panic!("could not find unique category class")
                        }
                        class_name = Some(entity.get_name().expect("class name"));
                        EntityVisitResult::Break
                    } else {
                        EntityVisitResult::Continue
                    }
                });
                let class_name = class_name.expect("could not find category class");
                let class_data = config.class_data.get(&class_name);

                let mut generics = Vec::new();

                let (protocols, methods) =
                    parse_objc_decl(&entity, None, Some(&mut generics), class_data);

                Some(Self::CategoryDecl {
                    class_name,
                    availability,
                    name,
                    generics,
                    protocols,
                    methods,
                })
            }
            EntityKind::ObjCProtocolDecl => {
                let name = entity.get_name().expect("protocol name");
                let class_data = config.class_data.get(&name);
                let availability = Availability::parse(
                    entity
                        .get_platform_availability()
                        .expect("protocol availability"),
                );

                let (protocols, methods) = parse_objc_decl(&entity, None, None, class_data);

                Some(Self::ProtocolDecl {
                    name,
                    availability,
                    protocols,
                    methods,
                })
            }
            EntityKind::TypedefDecl => {
                let name = entity.get_name().expect("typedef name");

                entity.visit_children(|entity, _parent| {
                    match entity.get_kind() {
                        // TODO: Parse NS_TYPED_EXTENSIBLE_ENUM vs. NS_TYPED_ENUM
                        EntityKind::UnexposedAttr => {
                            if let Some(macro_) = UnexposedMacro::parse(&entity) {
                                panic!("unexpected attribute: {macro_:?}");
                            }
                        }
                        EntityKind::StructDecl => {
                            // TODO?
                        }
                        EntityKind::ObjCClassRef
                        | EntityKind::ObjCProtocolRef
                        | EntityKind::TypeRef
                        | EntityKind::ParmDecl => {}
                        _ => panic!("unknown typedef child in {name}: {entity:?}"),
                    };
                    EntityVisitResult::Continue
                });

                let ty = entity
                    .get_typedef_underlying_type()
                    .expect("typedef underlying type");
                match ty.get_kind() {
                    // Note: When we encounter a typedef declaration like this:
                    //     typedef NSString* NSAbc;
                    //
                    // We parse it as one of:
                    //     type NSAbc = NSString;
                    //     struct NSAbc(NSString);
                    //
                    // Instead of:
                    //     type NSAbc = *const NSString;
                    //
                    // Because that means we can use ordinary Id<...> handling.
                    TypeKind::ObjCObjectPointer => {
                        let ty = ty.get_pointee_type().expect("pointer type to have pointee");
                        let type_ = GenericType::parse_objc_pointer(ty);

                        match &*type_.name {
                            "NSString" => {}
                            "NSUnit" => {}        // TODO: Handle this differently
                            "TodoProtocols" => {} // TODO
                            _ => panic!("typedef declaration was not NSString: {type_:?}"),
                        }

                        if !type_.generics.is_empty() {
                            panic!("typedef declaration generics not empty");
                        }

                        Some(Self::AliasDecl {
                            name,
                            type_: RustType::TypeDef { name: type_.name },
                        })
                    }
                    TypeKind::Typedef => {
                        // println!(
                        //     "typedef: {:?}, {:?}, {:#?}, {:#?}, {:#?}",
                        //     entity.get_display_name(),
                        //     entity.get_name(),
                        //     entity.has_attributes(),
                        //     entity.get_children(),
                        //     ty,
                        // );
                        let type_ = RustType::parse_typedef(ty);
                        Some(Self::AliasDecl { name, type_ })
                    }
                    TypeKind::Elaborated => {
                        let ty = ty.get_elaborated_type().expect("elaborated");
                        match ty.get_kind() {
                            TypeKind::Enum => {
                                // Handled below
                                None
                            }
                            _ => {
                                // println!(
                                //     "elaborated: {:?}, {:?}, {:?}, {:#?}, {:#?}, {:?}, {:#?}",
                                //     entity.get_kind(),
                                //     entity.get_display_name(),
                                //     entity.get_name(),
                                //     entity.has_attributes(),
                                //     entity.get_children(),
                                //     ty.get_kind(),
                                //     ty,
                                // );
                                None
                            }
                        }
                    }
                    _ => {
                        // println!(
                        //     "typedef2: {:?}, {:?}, {:?}, {:#?}, {:#?}, {:?}, {:#?}",
                        //     entity.get_kind(),
                        //     entity.get_display_name(),
                        //     entity.get_name(),
                        //     entity.has_attributes(),
                        //     entity.get_children(),
                        //     ty.get_kind(),
                        //     ty,
                        // );
                        None
                    }
                }
            }
            EntityKind::StructDecl => {
                // println!(
                //     "struct: {:?}, {:?}, {:#?}, {:#?}",
                //     entity.get_display_name(),
                //     entity.get_name(),
                //     entity.has_attributes(),
                //     entity.get_children(),
                // );
                // EntityKind::FieldDecl | EntityKind::IntegerLiteral | EntityKind::ObjCBoxable => {}
                None
            }
            EntityKind::EnumDecl => {
                // Enum declarations show up twice for some reason, but
                // luckily this flag is set on the least descriptive entity.
                if !entity.is_definition() {
                    return None;
                }

                let name = entity.get_name();
                let ty = entity.get_enum_underlying_type().expect("enum type");
                let is_signed = ty.is_signed_integer();
                let ty = RustType::parse_enum(ty);
                let mut kind = None;
                let mut variants = Vec::new();

                entity.visit_children(|entity, _parent| {
                    match entity.get_kind() {
                        EntityKind::EnumConstantDecl => {
                            let name = entity.get_name().expect("enum constant name");
                            let val = entity
                                .get_enum_constant_value()
                                .expect("enum constant value");
                            let expr = Expr::parse_enum_constant(&entity)
                                .unwrap_or_else(|| Expr::from_val(val, is_signed));
                            variants.push((name, expr));
                        }
                        EntityKind::UnexposedAttr => {
                            if let Some(macro_) = UnexposedMacro::parse(&entity) {
                                if let Some(kind) = &kind {
                                    assert_eq!(
                                        kind, &macro_,
                                        "got differing enum kinds in {name:?}"
                                    );
                                } else {
                                    kind = Some(macro_);
                                }
                            }
                        }
                        EntityKind::FlagEnum => {
                            let macro_ = UnexposedMacro::Options;
                            if let Some(kind) = &kind {
                                assert_eq!(kind, &macro_, "got differing enum kinds in {name:?}");
                            } else {
                                kind = Some(macro_);
                            }
                        }
                        _ => {
                            panic!("unknown enum child {entity:?} in {name:?}");
                        }
                    }
                    EntityVisitResult::Continue
                });

                Some(Self::EnumDecl {
                    name,
                    ty,
                    kind,
                    variants,
                })
            }
            EntityKind::VarDecl => {
                let name = entity.get_name().expect("var decl name");
                let ty = entity.get_type().expect("var type");
                let ty = RustTypeStatic::parse(ty);
                let mut value = None;

                entity.visit_children(|entity, _parent| {
                    match entity.get_kind() {
                        EntityKind::UnexposedAttr => {
                            if let Some(macro_) = UnexposedMacro::parse(&entity) {
                                panic!("unexpected attribute: {macro_:?}");
                            }
                        }
                        EntityKind::ObjCClassRef => {}
                        EntityKind::TypeRef => {}
                        _ if entity.is_expression() => {
                            if value.is_none() {
                                value = Some(Expr::parse_var(&entity));
                            } else {
                                panic!("got variable value twice")
                            }
                        }
                        _ => panic!("unknown typedef child in {name}: {entity:?}"),
                    };
                    EntityVisitResult::Continue
                });

                Some(Self::VarDecl { name, ty, value })
            }
            EntityKind::FunctionDecl => {
                let name = entity.get_name().expect("function name");

                if entity.is_variadic() {
                    println!("can't handle variadic function {name}");
                    return None;
                }

                let result_type = entity.get_result_type().expect("function result type");
                let result_type = RustTypeReturn::parse(result_type);
                let mut arguments = Vec::new();

                assert!(
                    !entity.is_static_method(),
                    "unexpected static method {name}"
                );

                entity.visit_children(|entity, _parent| {
                    match entity.get_kind() {
                        EntityKind::UnexposedAttr => {
                            if let Some(macro_) = UnexposedMacro::parse(&entity) {
                                panic!("unexpected function attribute: {macro_:?}");
                            }
                        }
                        EntityKind::ObjCClassRef | EntityKind::TypeRef => {}
                        EntityKind::ParmDecl => {
                            // Could also be retrieved via. `get_arguments`
                            let name = entity.get_name().unwrap_or_else(|| "_".into());
                            let ty = entity.get_type().expect("function argument type");
                            let ty = RustType::parse_argument(ty, false);
                            arguments.push((name, ty))
                        }
                        _ => panic!("unknown function child in {name}: {entity:?}"),
                    };
                    EntityVisitResult::Continue
                });

                let body = if entity.is_inline_function() {
                    Some(())
                } else {
                    None
                };

                Some(Self::FnDecl {
                    name,
                    arguments,
                    result_type,
                    body,
                })
            }
            EntityKind::UnionDecl => {
                // println!(
                //     "union: {:?}, {:?}, {:#?}, {:#?}",
                //     entity.get_display_name(),
                //     entity.get_name(),
                //     entity.has_attributes(),
                //     entity.get_children(),
                // );
                None
            }
            _ => {
                panic!("Unknown: {:?}", entity)
            }
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ClassDecl {
                name,
                availability: _,
                superclass,
                generics,
                protocols: _,
                methods,
            } => {
                let struct_generic = if generics.is_empty() {
                    name.clone()
                } else {
                    format!(
                        "{name}<{}: Message = Object>",
                        generics.join(": Message = Object,")
                    )
                };

                let generic_params = if generics.is_empty() {
                    String::new()
                } else {
                    format!("<{}: Message>", generics.join(": Message,"))
                };

                let type_ = if generics.is_empty() {
                    name.clone()
                } else {
                    format!("{name}<{}>", generics.join(","))
                };

                let superclass_name = superclass.as_deref().unwrap_or("Object");

                // TODO: Use ty.get_objc_protocol_declarations()

                let macro_name = if generics.is_empty() {
                    "extern_class"
                } else {
                    "__inner_extern_class"
                };

                writeln!(f, "{macro_name}!(")?;
                writeln!(f, "    #[derive(Debug)]")?;
                write!(f, "    pub struct {struct_generic}")?;
                if generics.is_empty() {
                    writeln!(f, ";")?;
                } else {
                    writeln!(f, " {{")?;
                    for (i, generic) in generics.iter().enumerate() {
                        // Invariant over the generic (for now)
                        writeln!(f, "_inner{i}: PhantomData<*mut {generic}>,")?;
                    }
                    writeln!(f, "}}")?;
                }
                writeln!(f, "")?;
                writeln!(
                    f,
                    "    unsafe impl{generic_params} ClassType for {type_} {{"
                )?;
                writeln!(f, "        type Super = {superclass_name};")?;
                writeln!(f, "    }}")?;
                writeln!(f, ");")?;
                writeln!(f, "")?;
                writeln!(f, "extern_methods!(")?;
                writeln!(f, "    unsafe impl{generic_params} {type_} {{")?;
                for method in methods {
                    writeln!(f, "{method}")?;
                }
                writeln!(f, "    }}")?;
                writeln!(f, ");")?;
            }
            Self::CategoryDecl {
                class_name,
                availability: _,
                name,
                generics,
                protocols: _,
                methods,
            } => {
                let generic_params = if generics.is_empty() {
                    String::new()
                } else {
                    format!("<{}: Message>", generics.join(": Message,"))
                };

                let type_ = if generics.is_empty() {
                    class_name.clone()
                } else {
                    format!("{class_name}<{}>", generics.join(","))
                };

                writeln!(f, "extern_methods!(")?;
                if let Some(name) = name {
                    writeln!(f, "    /// {name}")?;
                }
                writeln!(f, "    unsafe impl{generic_params} {type_} {{")?;
                for method in methods {
                    writeln!(f, "{method}")?;
                }
                writeln!(f, "    }}")?;
                writeln!(f, ");")?;
            }
            Self::ProtocolDecl {
                name,
                availability: _,
                protocols: _,
                methods: _,
            } => {
                // TODO

                // quote! {
                //     extern_protocol!(
                //         #[derive(Debug)]
                //         struct #name;
                //
                //         unsafe impl ProtocolType for #name {
                //             type Super = todo!();
                //         }
                //     );
                //
                //     impl #name {
                //         #(#methods)*
                //     }
                // }
                writeln!(f, "pub type {name} = NSObject;")?;
            }
            Self::EnumDecl {
                name,
                ty,
                // TODO: Use the enum kind
                kind: _,
                variants,
            } => {
                if let Some(name) = name {
                    writeln!(f, "pub type {name} = {ty};")?;
                    for (variant_name, expr) in variants {
                        writeln!(f, "pub const {variant_name}: {name} = {expr};")?;
                    }
                } else {
                    for (variant_name, expr) in variants {
                        writeln!(f, "pub const {variant_name}: i32 = {expr};")?;
                    }
                }
            }
            Self::VarDecl {
                name,
                ty,
                value: None,
            } => {
                writeln!(f, r#"extern "C" {{"#)?;
                writeln!(f, "    static {name}: {ty};")?;
                writeln!(f, "}}")?;
            }
            Self::VarDecl {
                name,
                ty,
                value: Some(None),
            } => {
                writeln!(f, "static {name}: {ty} = todo;")?;
            }
            Self::VarDecl {
                name,
                ty,
                value: Some(Some(expr)),
            } => {
                writeln!(f, "static {name}: {ty} = {expr};")?;
            }
            Self::FnDecl {
                name: _,
                arguments: _,
                result_type: _,
                body: None,
            } => {
                // TODO
                // writeln!(f, r#"extern "C" {{"#)?;
                // write!(f, "    fn {name}(")?;
                // for (param, arg_ty) in arguments {
                //     write!(f, "{}: {arg_ty},", handle_reserved(&param))?;
                // }
                // writeln!(f, "){result_type};")?;
                // writeln!(f, "}}")?;
            }
            Self::FnDecl {
                name: _,
                arguments: _,
                result_type: _,
                body: Some(_body),
            } => {
                // TODO
                // write!(f, "unsafe fn {name}(")?;
                // for (param, arg_ty) in arguments {
                //     write!(f, "{}: {arg_ty},", handle_reserved(&param))?;
                // }
                // writeln!(f, "){result_type} {{")?;
                // writeln!(f, "    todo!()")?;
                // writeln!(f, "}}")?;
            }
            Self::AliasDecl { name, type_ } => {
                writeln!(f, "pub type {name} = {type_};")?;
            }
        };
        Ok(())
    }
}
