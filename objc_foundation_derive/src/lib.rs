extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::{ToTokens, Tokens};

#[proc_macro_derive(INSObject)]
pub fn impl_object(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    // Build the impl
    let name = &ast.ident;
    let link_name = format!("OBJC_CLASS_$_{}", name);
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let mut gen = Tokens::new();
    quote!(
        unsafe impl #impl_generics ::objc::Message for #name #ty_generics #where_clause { }
    )
    .to_tokens(&mut gen);

    quote!(
        impl #impl_generics INSObject for #name #ty_generics #where_clause {
            fn class() -> &'static ::objc::runtime::Class {
                extern {
                    #[link_name = #link_name]
                    static OBJC_CLASS: ::objc::runtime::Class;
                }
                unsafe {
                    &OBJC_CLASS
                }
            }
        }
    )
    .to_tokens(&mut gen);

    quote!(
        impl #impl_generics ::core::cmp::PartialEq for #name #ty_generics #where_clause {
            fn eq(&self, other: &Self) -> bool {
                INSObject::is_equal(self, other)
            }
        }
    )
    .to_tokens(&mut gen);

    quote!(
        impl #impl_generics ::core::hash::Hash for #name #ty_generics #where_clause {
            fn hash<H>(&self, state: &mut H) where H: ::core::hash::Hasher {
                INSObject::hash_code(self).hash(state);
            }
        }
    )
    .to_tokens(&mut gen);

    quote!(
        impl #impl_generics ::core::fmt::Debug for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let s = INSObject::description(self);
                ::core::fmt::Display::fmt(&*s, f)
            }
        }
    )
    .to_tokens(&mut gen);

    // Return the generated impl
    gen.parse().unwrap()
}
