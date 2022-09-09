use clang::{Entity, EntityKind, EntityVisitResult};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

mod method;
mod objc2_utils;
mod rust_type;
use self::method::Method;

pub fn get_tokens(entity: &Entity<'_>) -> TokenStream {
    match entity.get_kind() {
        EntityKind::InclusionDirective
        | EntityKind::MacroExpansion
        | EntityKind::ObjCClassRef
        | EntityKind::ObjCProtocolRef
        | EntityKind::MacroDefinition => TokenStream::new(),
        EntityKind::ObjCInterfaceDecl => {
            // entity.get_mangled_objc_names()
            let name = format_ident!("{}", entity.get_name().expect("class name"));
            // println!("Availability: {:?}", entity.get_platform_availability());
            let mut superclass_name = None;
            let mut protocols = Vec::new();
            let mut methods = Vec::new();

            entity.visit_children(|entity, _parent| {
                match entity.get_kind() {
                    EntityKind::ObjCIvarDecl => {
                        // Explicitly ignored
                    }
                    EntityKind::ObjCSuperClassRef => {
                        superclass_name = Some(format_ident!(
                            "{}",
                            entity.get_name().expect("superclass name")
                        ));
                    }
                    EntityKind::ObjCRootClass => {
                        // TODO: Maybe just skip root classes entirely?
                        superclass_name = Some(format_ident!("Object"));
                    }
                    EntityKind::ObjCClassRef => {
                        println!("ObjCClassRef: {:?}", entity.get_display_name());
                    }
                    EntityKind::ObjCProtocolRef => {
                        protocols.push(entity);
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

            let superclass_name =
                superclass_name.expect("only classes with a superclass is supported");

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
        EntityKind::ObjCCategoryDecl => {
            let meta = if let Some(doc) = entity.get_name() {
                quote!(#[doc = #doc])
            } else {
                // Some categories don't have a name. Example: NSClipView
                quote!()
            };
            let mut class = None;
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
                        protocols.push(entity);
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
            let class_name = format_ident!("{}", class.get_name().expect("class name"));

            quote! {
                #meta
                impl #class_name {
                    #(#methods)*
                }
            }
        }
        EntityKind::ObjCProtocolDecl => {
            let name = format_ident!("{}", entity.get_name().expect("protocol name"));
            let mut protocols = Vec::new();
            let mut methods = Vec::new();

            entity.visit_children(|entity, _parent| {
                match entity.get_kind() {
                    EntityKind::ObjCExplicitProtocolImpl => {
                        // TODO
                    }
                    EntityKind::ObjCProtocolRef => {
                        protocols.push(entity);
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
        EntityKind::EnumDecl
        | EntityKind::VarDecl
        | EntityKind::FunctionDecl
        | EntityKind::TypedefDecl
        | EntityKind::StructDecl => {
            // TODO
            TokenStream::new()
        }
        _ => {
            panic!("Unknown: {:?}", entity)
        }
    }
}

pub fn create_rust_file(entities: &[Entity<'_>]) -> TokenStream {
    let iter = entities.iter().map(get_tokens);
    quote! {
        #(#iter)*
    }
}
