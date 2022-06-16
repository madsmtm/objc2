use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

use crate::utils::get_repr;

pub(crate) fn impl_encode(ast: &DeriveInput) -> TokenStream {
    let DeriveInput {
        ident, data, attrs, ..
    } = ast;

    let repr = match get_repr(&attrs) {
        Some(repr) => repr,
        None => panic!("Missing repr"),
    };

    let encoding = match data {
        Data::Struct(data) => {
            let fields = data
                .fields
                .iter()
                .map(|field| &field.ty)
                .collect::<Vec<_>>();
            match &*repr.to_string() {
                "transparent" => {
                    let field = fields[0];
                    assert_eq!(fields.len(), 1, "Expected one item");
                    quote!(<#field as ::objc2_encode::Encode>::ENCODING)
                }
                "C" => {
                    quote!(
                        ::objc2_encode::Encoding::Struct(
                            stringify!(#ident),
                            &[#(<#fields as ::objc2_encode::Encode>::ENCODING),*],
                        )
                    )
                }
                _ => panic!("Unknown repr"),
            }
        }
        Data::Enum(_) => {
            let ty = match &*repr.to_string() {
                "usize" => quote! { core::primitive::usize },
                "isize" => quote! { core::primitive::isize },
                "u64" => quote! { core::primitive::u64 },
                "i64" => quote! { core::primitive::i64 },
                "u32" => quote! { core::primitive::u32 },
                "i32" => quote! { core::primitive::i32 },
                "u16" => quote! { core::primitive::u16 },
                "i16" => quote! { core::primitive::i16 },
                "u8" => quote! { core::primitive::u8 },
                "i8" => quote! { core::primitive::i8 },
                _ => panic!("Unknown repr"),
            };
            quote! { <#ty as ::objc2_encode::Encode>::ENCODING }
        }
        Data::Union(_) => unimplemented!(),
    };

    // TODO: Generics
    quote! {
        unsafe impl ::objc2_encode::Encode for #ident {
            const ENCODING: ::objc2_encode::Encoding<'static> = #encoding;
        }
    }
    .into()
}

pub(crate) fn impl_ref_encode(ast: &DeriveInput) -> TokenStream {
    let DeriveInput { ident, .. } = ast;
    // TODO: Generics
    // TODO: Objects

    let gen = quote! {
        unsafe impl ::objc2_encode::RefEncode for #ident {
            const ENCODING_REF: ::objc2_encode::Encoding<'static> = ::objc2_encode::Encoding::Pointer(
                &<Self as ::objc2_encode::Encode>::ENCODING
            );
        }
    };
    gen.into()
}
