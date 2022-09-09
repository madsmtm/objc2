use clang::{Entity, EntityKind, TypeKind};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};

use crate::rust_type::RustType;

#[derive(Debug, Clone)]
pub struct Method {
    tokens: TokenStream,
}

impl Method {
    /// Takes one of `EntityKind::ObjCInstanceMethodDecl` or
    /// `EntityKind::ObjCClassMethodDecl`.
    pub fn parse(entity: Entity<'_>) -> Option<Self> {
        // println!("Method {:?}", entity.get_display_name());
        // println!("Availability: {:?}", entity.get_platform_availability());
        // TODO: Handle `NSConsumesSelf` and `NSReturnsRetained`
        // println!("Children: {:?}", entity.get_children());

        let selector = entity.get_name().expect("method selector");
        let fn_name = format_ident!(
            "{}",
            selector.trim_end_matches(|c| c == ':').replace(':', "_")
        );

        if entity.is_variadic() {
            println!("Can't handle variadic method {}", selector);
            return None;
        }

        println!("{}", selector);

        let result_type = entity.get_result_type().expect("method return type");
        let (ret, is_id) = if result_type.get_kind() == TypeKind::Void {
            (quote! {}, false)
        } else {
            let RustType { tokens, is_id } = RustType::parse(result_type, true);
            (
                quote! {
                    -> #tokens
                },
                is_id,
            )
        };

        let macro_name = if is_id {
            format_ident!("msg_send_id")
        } else {
            format_ident!("msg_send")
        };

        let arguments: Vec<_> = entity
            .get_arguments()
            .expect("method arguments")
            .into_iter()
            .map(|arg| {
                (
                    format_ident!(
                        "{}",
                        handle_reserved(&arg.get_name().expect("arg display name"))
                    ),
                    arg.get_type().expect("argument type"),
                )
            })
            .collect();

        let fn_args = arguments.iter().map(|(param, arg_ty)| {
            let RustType { tokens, .. } = RustType::parse(arg_ty.clone(), false);
            quote!(#param: #tokens)
        });

        let method_call = if selector.contains(':') {
            let split_selector: Vec<_> =
                selector.split(':').filter(|sel| !sel.is_empty()).collect();
            assert!(
                arguments.len() == split_selector.len(),
                "incorrect method argument length",
            );

            let iter = arguments
                .iter()
                .zip(split_selector)
                .map(|((param, _), sel)| {
                    let sel = format_ident!("{}", sel);
                    quote!(#sel: #param)
                });
            quote!(#(#iter),*)
        } else {
            assert_eq!(arguments.len(), 0, "too many arguments");
            let sel = format_ident!("{}", selector);
            quote!(#sel)
        };

        let tokens = match entity.get_kind() {
            EntityKind::ObjCInstanceMethodDecl => quote! {
                pub unsafe fn #fn_name(&self #(, #fn_args)*) #ret {
                    #macro_name![self, #method_call]
                }
            },
            EntityKind::ObjCClassMethodDecl => quote! {
                pub unsafe fn #fn_name(#(#fn_args),*) #ret {
                    #macro_name![Self::class(), #method_call]
                }
            },
            _ => unreachable!("unknown method kind"),
        };
        Some(Self { tokens })
    }
}

impl ToTokens for Method {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.tokens.clone());
    }
}

fn handle_reserved(s: &str) -> &str {
    match s {
        "type" => "type_",
        s => s,
    }
}
