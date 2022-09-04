use std::collections::HashMap;
use std::path::{Path, PathBuf};

use clang::source::File;
use clang::{Clang, Entity, EntityKind, EntityVisitResult, Index, Type};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

pub fn get_rust_name(selector: &str) -> Ident {
    format_ident!("{}", selector.split(':').collect::<Vec<&str>>().join("_"))
}

pub fn get_return_type(return_type: &Type<'_>) -> TokenStream {
    // let return_item = ctx.resolve_item(sig.return_type());
    // if let TypeKind::Void = *return_item.get_kind().expect_type().kind() {
    //     quote! {}
    // } else {
    //     let ret_ty = return_item.to_rust_ty_or_opaque(ctx, &());
    //     quote! {
    //         -> #ret_ty
    //     }
    // }
    quote! {}
}

pub fn get_macro_name(return_type: &Type<'_>) -> Ident {
    if true {
        format_ident!("msg_send")
    } else {
        format_ident!("msg_send_id")
    }
}

pub fn get_tokens(entity: &Entity<'_>) -> TokenStream {
    match entity.get_kind() {
        EntityKind::InclusionDirective | EntityKind::MacroExpansion | EntityKind::ObjCClassRef => {
            TokenStream::new()
        }
        EntityKind::ObjCInterfaceDecl => {
            let name = format_ident!("{}", entity.get_name().expect("class name"));
            let mut superclass = None;
            let mut protocols = Vec::new();
            let mut methods = Vec::new();

            entity.visit_children(|entity, _parent| {
                match entity.get_kind() {
                    EntityKind::ObjCSuperClassRef => {
                        superclass = Some(entity);
                    }
                    EntityKind::ObjCClassRef => {
                        println!("ObjCClassRef: {:?}", entity.get_display_name());
                    }
                    EntityKind::ObjCProtocolRef => {
                        protocols.push(entity);
                    }
                    EntityKind::ObjCInstanceMethodDecl => {
                        let selector = entity.get_name().expect("instance method selector");
                        let fn_name = get_rust_name(&selector);

                        let result_type = entity
                            .get_result_type()
                            .expect("instance method return type");
                        let ret = get_return_type(&result_type);
                        let macro_name = get_macro_name(&result_type);

                        methods.push(quote! {
                            pub unsafe fn #fn_name(&self) #ret {
                                #macro_name![self, ]
                            }
                        });
                    }
                    EntityKind::ObjCClassMethodDecl => {
                        let selector = entity.get_name().expect("class method selector");
                        let fn_name = get_rust_name(&selector);

                        let result_type =
                            entity.get_result_type().expect("class method return type");
                        let ret = get_return_type(&result_type);
                        let macro_name = get_macro_name(&result_type);

                        methods.push(quote! {
                            pub unsafe fn #fn_name() #ret {
                                #macro_name![Self::class(), ]
                            }
                        });
                    }
                    EntityKind::ObjCPropertyDecl => {
                        methods.push(quote! {});
                    }
                    _ => {
                        println!("{:?}: {:?}", entity.get_kind(), entity.get_display_name());
                    }
                }
                EntityVisitResult::Continue
            });

            let superclass = superclass.expect("only classes with a superclass is supported");
            let superclass_name =
                format_ident!("{}", superclass.get_name().expect("superclass name"));

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
            quote! {}
        }
        _ => {
            println!(
                "Unknown: {:?}: {}",
                entity.get_kind(),
                entity
                    .get_display_name()
                    .unwrap_or_else(|| "`None`".to_string())
            );
            TokenStream::new()
        }
    }
}

pub fn create_rust_file(entities: &[Entity<'_>]) -> TokenStream {
    let mut iter = entities.iter().map(get_tokens);
    quote! {
        #(#iter)*
    }
}
