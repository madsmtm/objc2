use clang::{Entity, EntityKind, EntityVisitResult, TypeKind};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};

use crate::availability::Availability;
use crate::config::Config;
use crate::method::Method;
use crate::rust_type::RustType;

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
                let mut protocols = Vec::new();
                let mut methods = Vec::new();

                entity.visit_children(|entity, _parent| {
                    match entity.get_kind() {
                        EntityKind::ObjCIvarDecl => {
                            // Explicitly ignored
                        }
                        EntityKind::ObjCSuperClassRef => {
                            superclass = Some(Some(entity.get_name().expect("superclass name")));
                        }
                        EntityKind::ObjCRootClass => {
                            // TODO: Maybe just skip root classes entirely?
                            superclass = Some(None);
                        }
                        EntityKind::ObjCClassRef => {
                            // println!("ObjCClassRef: {:?}", entity.get_display_name());
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
                        EntityKind::TemplateTypeParameter => {
                            // TODO: Generics with bounds (like NSMeasurement<UnitType: NSUnit *>)
                            // let ty = entity.get_type().expect("template type");
                            let name = entity.get_display_name().expect("template name");
                            generics.push(name);
                        }
                        EntityKind::VisibilityAttr => {
                            // NS_CLASS_AVAILABLE_MAC??
                            println!("TODO: VisibilityAttr")
                        }
                        EntityKind::TypeRef => {
                            // TODO
                        }
                        EntityKind::ObjCException => {
                            // Maybe useful for knowing when to implement `Error` for the type
                        }
                        EntityKind::UnexposedAttr => {}
                        _ => panic!("Unknown in ObjCInterfaceDecl {:?}", entity),
                    }
                    EntityVisitResult::Continue
                });

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
                let mut class_name = None;
                let mut generics = Vec::new();
                let availability = Availability::parse(
                    entity
                        .get_platform_availability()
                        .expect("category availability"),
                );
                let mut protocols = Vec::new();
                let mut methods = Vec::new();

                entity.visit_children(|entity, _parent| {
                    match entity.get_kind() {
                        EntityKind::ObjCClassRef => {
                            if class_name.is_some() {
                                panic!("could not find unique category class")
                            }
                            class_name = Some(entity.get_name().expect("class name"));
                        }
                        EntityKind::ObjCProtocolRef => {
                            protocols.push(entity.get_name().expect("protocolref to have name"));
                        }
                        EntityKind::ObjCInstanceMethodDecl | EntityKind::ObjCClassMethodDecl => {
                            let class_data = config.class_data.get(
                                class_name
                                    .as_ref()
                                    .expect("no category class before methods"),
                            );
                            if let Some(method) = Method::parse(entity, class_data) {
                                methods.push(method);
                            }
                        }
                        EntityKind::ObjCPropertyDecl => {
                            // println!(
                            //     "Property {:?}, {:?}, {:?}",
                            //     class_name,
                            //     entity.get_display_name(),
                            //     entity.get_objc_attributes(),
                            // );
                            // methods.push(quote! {});
                        }
                        EntityKind::TemplateTypeParameter => {
                            let name = entity.get_display_name().expect("template name");
                            generics.push(name);
                        }
                        EntityKind::UnexposedAttr => {}
                        _ => panic!("Unknown in ObjCCategoryDecl {:?}", entity),
                    }
                    EntityVisitResult::Continue
                });

                let class_name = class_name.expect("could not find category class");

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
                let mut protocols = Vec::new();
                let mut methods = Vec::new();

                entity.visit_children(|entity, _parent| {
                    match entity.get_kind() {
                        EntityKind::ObjCExplicitProtocolImpl => {
                            // TODO
                        }
                        EntityKind::ObjCProtocolRef => {
                            protocols.push(entity.get_name().expect("protocolref to have name"));
                        }
                        EntityKind::ObjCInstanceMethodDecl | EntityKind::ObjCClassMethodDecl => {
                            // TODO: Required vs. optional methods
                            if let Some(method) = Method::parse(entity, class_data) {
                                methods.push(method);
                            }
                        }
                        EntityKind::ObjCPropertyDecl => {
                            // TODO
                        }
                        EntityKind::UnexposedAttr => {}
                        _ => panic!("Unknown in ObjCProtocolDecl {:?}", entity),
                    }
                    EntityVisitResult::Continue
                });

                Some(Self::ProtocolDecl {
                    name,
                    availability,
                    protocols,
                    methods,
                })
            }
            EntityKind::TypedefDecl => {
                let name = entity.get_name().expect("typedef name");
                let underlying_ty = entity
                    .get_typedef_underlying_type()
                    .expect("typedef underlying type");
                match underlying_ty.get_kind() {
                    TypeKind::ObjCObjectPointer => {
                        let type_name = RustType::typedef_is_id(underlying_ty).expect("typedef id");
                        Some(Self::AliasDecl {
                            name,
                            type_: RustType::TypeDef { name: type_name },
                        })
                    }
                    TypeKind::Typedef => {
                        let type_ = RustType::parse(underlying_ty, false, false);
                        Some(Self::AliasDecl { name, type_ })
                    }
                    _ => {
                        // println!(
                        //     "typedef: {:#?}, {:#?}, {:#?}, {:#?}",
                        //     entity.get_display_name(),
                        //     entity.has_attributes(),
                        //     entity.get_children(),
                        //     underlying_ty,
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
