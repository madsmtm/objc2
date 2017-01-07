#![feature(proc_macro, proc_macro_lib)]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(INSObject)]
pub fn impl_object(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    // Build the impl
    let name = &ast.ident;
    let link_name = format!("OBJC_CLASS_$_{}", name);

    let gen = quote! {
        unsafe impl ::objc::Message for #name { }

        impl ::objc_foundation::INSObject for #name {
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
    };

    // Return the generated impl
    gen.parse().unwrap()
}
