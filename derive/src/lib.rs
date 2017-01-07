#![recursion_limit = "128"]
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
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let gen = quote! {
        unsafe impl #impl_generics ::objc::Message for #name #ty_generics #where_clause { }

        impl #impl_generics ::objc_foundation::INSObject for #name #ty_generics #where_clause {
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

        impl #impl_generics ::std::cmp::PartialEq for #name #ty_generics #where_clause {
            fn eq(&self, other: &Self) -> bool {
                use ::objc_foundation::INSObject;
                self.is_equal(other)
            }
        }

        impl #impl_generics ::std::hash::Hash for #name #ty_generics #where_clause {
            fn hash<H>(&self, state: &mut H) where H: ::std::hash::Hasher {
                use ::objc_foundation::INSObject;
                self.hash_code().hash(state);
            }
        }

        impl #impl_generics ::std::fmt::Debug for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                use ::objc_foundation::{INSObject, INSString};
                ::std::fmt::Debug::fmt(self.description().as_str(), f)
            }
        }
    };

    // Return the generated impl
    gen.parse().unwrap()
}
