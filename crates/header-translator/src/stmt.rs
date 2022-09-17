use clang::{Entity, EntityKind, EntityVisitResult};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};

use crate::availability::Availability;
use crate::config::Config;
use crate::method::Method;

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
        protocols: Vec<String>,
        methods: Vec<Method>,
    },
    /// @interface class_name (name) <protocols*>
    CategoryDecl {
        class_name: String,
        availability: Availability,
        /// Some categories don't have a name. Example: NSClipView
        name: Option<String>,
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
}

impl Stmt {
    pub fn parse(entity: &Entity<'_>, config: &Config) -> Option<Self> {
        match entity.get_kind() {
            EntityKind::InclusionDirective => {
                // let file = entity.get_file().expect("inclusion file");
                let name = dbg!(entity.get_name().expect("inclusion name"));
                let mut iter = name.split("/");
                let framework = iter.next().expect("inclusion name has framework");
                let file = if let Some(file) = iter.next() {
                    file
                } else {
                    // Ignore
                    return None;
                };
                assert!(iter.count() == 0, "no more left");

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
            EntityKind::MacroExpansion
            | EntityKind::ObjCClassRef
            | EntityKind::ObjCProtocolRef
            | EntityKind::MacroDefinition => None,
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
                            println!("ObjCClassRef: {:?}", entity.get_display_name());
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
                            println!("TODO: Template parameters")
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
                    protocols,
                    methods,
                })
            }
            EntityKind::ObjCCategoryDecl => {
                let mut class_name = None;
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
                            println!(
                                "Property {:?}, {:?}, {:?}",
                                class_name,
                                entity.get_display_name(),
                                entity.get_objc_attributes(),
                            );
                            // methods.push(quote! {});
                        }
                        EntityKind::TemplateTypeParameter => {
                            println!("TODO: Template parameters")
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
                    name: entity.get_name(),
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
            EntityKind::EnumDecl
            | EntityKind::VarDecl
            | EntityKind::FunctionDecl
            | EntityKind::TypedefDecl
            | EntityKind::StructDecl => {
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
                    use super::#name;
                }
            }
            Self::ClassDecl {
                name,
                availability,
                superclass,
                protocols,
                methods,
            } => {
                let name = format_ident!("{}", name);
                let superclass_name =
                    format_ident!("{}", superclass.as_deref().unwrap_or("Object"));

                // TODO: Use ty.get_objc_protocol_declarations()

                quote! {
                    extern_class!(
                        #[derive(Debug)]
                        pub struct #name;

                        unsafe impl ClassType for #name {
                            type Super = #superclass_name;
                        }
                    );

                    impl #name {
                        #(#methods)*
                    }
                }
            }
            Self::CategoryDecl {
                class_name,
                availability,
                name,
                protocols,
                methods,
            } => {
                let meta = if let Some(name) = name {
                    quote!(#[doc = #name])
                } else {
                    quote!()
                };
                let class_name = format_ident!("{}", class_name);

                quote! {
                    #meta
                    impl #class_name {
                        #(#methods)*
                    }
                }
            }
            Self::ProtocolDecl {
                name,
                availability,
                protocols,
                methods,
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
        };
        tokens.append_all(result);
    }
}
