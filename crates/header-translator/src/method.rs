use clang::{Entity, EntityKind, TypeKind};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};

use crate::rust_type::RustType;

#[derive(Debug, Clone)]
pub struct Method {
    selector: String,
    is_class_method: bool,
    arguments: Vec<(String, RustType)>,
    result_type: Option<RustType>,
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

        if entity.is_variadic() {
            println!("Can't handle variadic method {}", selector);
            return None;
        }

        let is_class_method = match entity.get_kind() {
            EntityKind::ObjCInstanceMethodDecl => false,
            EntityKind::ObjCClassMethodDecl => true,
            _ => unreachable!("unknown method kind"),
        };

        let arguments: Vec<_> = entity
            .get_arguments()
            .expect("method arguments")
            .into_iter()
            .map(|arg| {
                (
                    arg.get_name().expect("arg display name"),
                    RustType::parse(arg.get_type().expect("argument type"), false),
                )
            })
            .collect();

        let result_type = entity.get_result_type().expect("method return type");
        let result_type = if result_type.get_kind() != TypeKind::Void {
            Some(RustType::parse(result_type, true))
        } else {
            None
        };

        Some(Self {
            selector,
            is_class_method,
            arguments,
            result_type,
        })
    }
}

impl ToTokens for Method {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let fn_name = format_ident!(
            "{}",
            self.selector
                .trim_end_matches(|c| c == ':')
                .replace(':', "_")
        );

        let arguments: Vec<_> = self
            .arguments
            .iter()
            .map(|(param, ty)| (format_ident!("{}", handle_reserved(&param)), ty))
            .collect();

        let fn_args = arguments.iter().map(|(param, arg_ty)| {
            let tokens = &arg_ty.tokens;
            quote!(#param: #tokens)
        });

        let method_call = if self.selector.contains(':') {
            let split_selector: Vec<_> = self
                .selector
                .split(':')
                .filter(|sel| !sel.is_empty())
                .collect();
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
            let sel = format_ident!("{}", self.selector);
            quote!(#sel)
        };

        let (ret, is_id) = if let Some(RustType { tokens, is_id }) = &self.result_type {
            (quote!(-> #tokens), *is_id)
        } else {
            (quote!(), false)
        };

        let macro_name = if is_id {
            format_ident!("msg_send_id")
        } else {
            format_ident!("msg_send")
        };

        let result = if self.is_class_method {
            quote! {
                pub unsafe fn #fn_name(#(#fn_args),*) #ret {
                    #macro_name![Self::class(), #method_call]
                }
            }
        } else {
            quote! {
                pub unsafe fn #fn_name(&self #(, #fn_args)*) #ret {
                    #macro_name![self, #method_call]
                }
            }
        };
        tokens.append_all(result);
    }
}

fn handle_reserved(s: &str) -> &str {
    match s {
        "type" => "type_",
        s => s,
    }
}
