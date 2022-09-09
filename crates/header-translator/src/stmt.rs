use clang::{Entity, EntityKind, EntityVisitResult};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};

use crate::availability::Availability;
use crate::method::Method;

#[derive(Debug, Clone)]
pub enum Stmt {
    /// @interface
    ClassDecl {
        name: String,
        availability: Availability,
        // TODO: Generics
        superclass: Option<String>,
        protocols: Vec<String>,
        methods: Vec<Method>,
    },
    CategoryDecl {
        class_name: String,
        availability: Availability,
        /// Some categories don't have a name. Example: NSClipView
        name: Option<String>,
        /// I don't quite know what this means?
        protocols: Vec<String>,
        methods: Vec<Method>,
    },
    ProtocolDecl {
        name: String,
        availability: Availability,
        protocols: Vec<String>,
        methods: Vec<Method>,
    },
}

impl Stmt {
    pub fn parse(entity: &Entity<'_>) -> Option<Self> {
        match entity.get_kind() {
            EntityKind::InclusionDirective
            | EntityKind::MacroExpansion
            | EntityKind::ObjCClassRef
            | EntityKind::ObjCProtocolRef
            | EntityKind::MacroDefinition => None,
            EntityKind::ObjCInterfaceDecl => {
                // entity.get_mangled_objc_names()
                let name = entity.get_name().expect("class name");
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
                            if let Some(method) = Method::parse(entity) {
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
                let mut class = None;
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
                            if class.is_some() {
                                panic!("could not find unique category class")
                            }
                            class = Some(entity);
                        }
                        EntityKind::ObjCProtocolRef => {
                            protocols.push(entity.get_name().expect("protocolref to have name"));
                        }
                        EntityKind::ObjCInstanceMethodDecl | EntityKind::ObjCClassMethodDecl => {
                            if let Some(method) = Method::parse(entity) {
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
                        EntityKind::UnexposedAttr => {}
                        _ => panic!("Unknown in ObjCCategoryDecl {:?}", entity),
                    }
                    EntityVisitResult::Continue
                });

                let class = class.expect("could not find category class");
                let class_name = class.get_name().expect("class name");

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
                            if let Some(method) = Method::parse(entity) {
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

                quote! {
                    extern_class!(
                        #[derive(Debug)]
                        struct #name;

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

                quote! {
                    extern_protocol!(
                        #[derive(Debug)]
                        struct #name;

                        unsafe impl ProtocolType for #name {
                            type Super = todo!();
                        }
                    );

                    impl #name {
                        #(#methods)*
                    }
                }
            }
        };
        tokens.append_all(result);
    }
}
