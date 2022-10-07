use clang::{Entity, EntityKind, EntityVisitResult, TypeKind};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};

use crate::availability::Availability;
use crate::config::{ClassData, Config};
use crate::method::Method;
use crate::rust_type::{GenericType, RustType};

/// Takes one of:
/// - `EntityKind::ObjCInterfaceDecl`
/// - `EntityKind::ObjCProtocolDecl`
/// - `EntityKind::ObjCCategoryDecl`
fn parse_objc_decl(
    entity: &Entity<'_>,
    mut superclass: Option<&mut Option<Option<String>>>,
    mut generics: Option<&mut Vec<String>>,
    class_data: Option<&ClassData>,
) -> (Vec<String>, Vec<Method>) {
    let mut protocols = Vec::new();
    let mut methods = Vec::new();

    entity.visit_children(|entity, _parent| {
        match entity.get_kind() {
            EntityKind::ObjCExplicitProtocolImpl if generics.is_some() && superclass.is_none() => {
                // TODO
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
                if let Some(method) = Method::parse(entity, class_data) {
                    methods.push(method);
                }
            }
            EntityKind::ObjCPropertyDecl => {
                // println!(
                //     "Property {:?}, {:?}",
                //     entity.get_display_name().unwrap(),
                //     entity.get_objc_attributes().unwrap()
                // );
                // methods.push(quote! {});
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
            EntityKind::UnexposedAttr => {}
            _ => panic!("unknown objc decl child {entity:?}"),
        };
        EntityVisitResult::Continue
    });

    (protocols, methods)
}

#[derive(Debug, Clone)]
pub enum Stmt {
    /// #import <framework/file.h>
    FileImport { framework: String, file: String },
    /// @class Name;
    /// @protocol Name;
    ItemImport { name: String },
    /// @interface name: superclass <protocols*>
    ClassDecl {
        name: String,
        availability: Availability,
        // TODO: Generics
        superclass: Option<String>,
        generics: Vec<String>,
        protocols: Vec<String>,
        methods: Vec<Method>,
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
        methods: Vec<Method>,
    },
    /// @protocol name <protocols*>
    ProtocolDecl {
        name: String,
        availability: Availability,
        protocols: Vec<String>,
        methods: Vec<Method>,
    },
    /// typedef Type TypedefName;
    AliasDecl { name: String, type_: RustType },
    // /// typedef struct Name { fields } TypedefName;
    // X,
}

impl Stmt {
    pub fn parse(entity: &Entity<'_>, config: &Config) -> Option<Self> {
        match entity.get_kind() {
            EntityKind::InclusionDirective => {
                // let file = entity.get_file().expect("inclusion file");
                let name = entity.get_name().expect("inclusion name");
                let mut iter = name.split('/');
                let framework = iter.next().expect("inclusion name has framework");
                let file = iter.next()?;
                if iter.count() != 0 {
                    // TODO: Fix this
                    println!("skipping inclusion of {name:?}");
                    return None;
                }

                Some(Self::FileImport {
                    framework: framework.to_string(),
                    file: file
                        .strip_suffix(".h")
                        .expect("inclusion name file is header")
                        .to_string(),
                })
            }
            EntityKind::ObjCClassRef | EntityKind::ObjCProtocolRef => {
                let name = entity.get_name().expect("objc ref has name");

                // We intentionally don't handle generics here
                Some(Self::ItemImport { name })
            }
            EntityKind::MacroExpansion | EntityKind::MacroDefinition => None,
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
                        let type_ = RustType::parse_typedef(ty);
                        Some(Self::AliasDecl { name, type_ })
                    }
                    _ => {
                        // println!(
                        //     "typedef: {:#?}, {:#?}, {:#?}, {:#?}",
                        //     entity.get_display_name(),
                        //     entity.has_attributes(),
                        //     entity.get_children(),
                        //     ty,
                        // );
                        None
                    }
                }
            }
            EntityKind::StructDecl => {
                // println!("struct: {:?}", entity.get_display_name());
                None
            }
            EntityKind::EnumDecl | EntityKind::VarDecl | EntityKind::FunctionDecl => {
                // TODO
                None
            }
            _ => {
                panic!("Unknown: {:?}", entity)
            }
        }
    }
}

impl ToTokens for Stmt {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let result = match self {
            Self::FileImport { framework, file } => {
                let framework = format_ident!("{}", framework);
                let file = format_ident!("{}", file);

                quote! {
                    use crate::#framework::generated::#file::*;
                }
            }
            Self::ItemImport { name } => {
                let name = format_ident!("{}", name);

                quote! {
                    use super::__exported::#name;
                }
            }
            Self::ClassDecl {
                name,
                availability: _,
                superclass,
                generics,
                protocols: _,
                methods,
            } => {
                let name = format_ident!("{}", name);
                let generics: Vec<_> = generics
                    .iter()
                    .map(|name| format_ident!("{}", name))
                    .collect();

                let generic_params = if generics.is_empty() {
                    quote!()
                } else {
                    quote!(<#(#generics: Message),*>)
                };

                let type_ = if generics.is_empty() {
                    quote!(#name)
                } else {
                    quote!(#name<#(#generics),*>)
                };

                let superclass_name =
                    format_ident!("{}", superclass.as_deref().unwrap_or("Object"));

                // TODO: Use ty.get_objc_protocol_declarations()

                let macro_name = if generics.is_empty() {
                    quote!(extern_class)
                } else {
                    quote!(__inner_extern_class)
                };

                quote! {
                    #macro_name!(
                        #[derive(Debug)]
                        pub struct #name #generic_params;

                        unsafe impl #generic_params ClassType for #type_ {
                            type Super = #superclass_name;
                        }
                    );

                    impl #generic_params #type_ {
                        #(#methods)*
                    }
                }
            }
            Self::CategoryDecl {
                class_name,
                availability: _,
                name,
                generics,
                protocols: _,
                methods,
            } => {
                let meta = if let Some(name) = name {
                    quote!(#[doc = #name])
                } else {
                    quote!()
                };
                let class_name = format_ident!("{}", class_name);

                let generics: Vec<_> = generics
                    .iter()
                    .map(|name| format_ident!("{}", name))
                    .collect();

                quote! {
                    #meta
                    impl<#(#generics: Message),*> #class_name<#(#generics),*> {
                        #(#methods)*
                    }
                }
            }
            Self::ProtocolDecl {
                name,
                availability: _,
                protocols: _,
                methods: _,
            } => {
                let name = format_ident!("{}", name);

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
                quote!(pub type #name = NSObject;)
            }
            Self::AliasDecl { name, type_ } => {
                let name = format_ident!("{}", name);

                quote!(pub type #name = #type_;)
            }
        };
        tokens.append_all(result);
    }
}
