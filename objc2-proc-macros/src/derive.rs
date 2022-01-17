use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn impl_encode(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        unsafe impl ::objc2_encode::Encode for #name {
            const ENCODING: ::objc2_encode::Encoding<'static> = ::objc2_encode::Encoding::Struct(
                stringify!(#name),
                &[CGFloat::ENCODING, CGFloat::ENCODING],
            );
        }
    };
    gen.into()
}

pub(crate) fn impl_ref_encode(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        unsafe impl ::objc2_encode::RefEncode for #name {
            const ENCODING_REF: ::objc2_encode::Encoding<'static> = ::objc2_encode::Encoding::Pointer(
                &<Self as ::objc2_encode::Encode>::ENCODING
            );
        }
    };
    gen.into()
}
