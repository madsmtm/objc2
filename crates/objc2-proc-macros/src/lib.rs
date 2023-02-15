//! Procedural macros for the [`objc2` project](https://github.com/madsmtm/objc2).
//!
//! You should not need to use this crate directly, all its public items are
//! exported in other crates.

#![warn(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![deny(non_ascii_idents)]
#![warn(unreachable_pub)]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::cargo)]
#![warn(clippy::ptr_as_ptr)]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-proc-macros/0.1.1")]

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
extern "C" {}

use core::hash::{Hash, Hasher};

use proc_macro::Ident;
use proc_macro::Literal;
use proc_macro::TokenStream;
use proc_macro::TokenTree;
use proc_macro2::Span;
use quote::{quote, TokenStreamExt};
use syn::spanned::Spanned;

mod common;
mod interface;
mod protocol;

/// Extract all identifiers in the given tokenstream.
fn get_idents(input: TokenStream) -> impl Iterator<Item = Ident> {
    input.into_iter().flat_map(|token| {
        match token {
            TokenTree::Group(group) => get_idents(group.stream()).collect::<Vec<_>>(),
            TokenTree::Ident(ident) => {
                vec![ident]
            }
            TokenTree::Punct(_) | TokenTree::Literal(_) => {
                vec![]
            }
        }
        .into_iter()
    })
}

/// Creates a hash from the input and source code locations in the provided
/// idents.
///
/// This hash is not guaranteed to be stable across compiler versions.
///
/// Tests are in [`objc2::__macro_helpers`].
#[proc_macro]
#[doc(hidden)]
pub fn __hash_idents(input: TokenStream) -> TokenStream {
    // Create the hasher
    let mut hasher = std::collections::hash_map::DefaultHasher::new();

    // Hash each ident
    for ident in get_idents(input) {
        ident.to_string().hash(&mut hasher);

        // Hash the source code location of the ident
        //
        // HACK: the only somewhat-reasonable way to get "unique" data in a
        // proc macro right now is from the `Debug` formatter for spans which
        // includes the source code location... so just hash the whole `Debug`
        // format output of the span
        //
        // Prior art in the `defmt` crate, see here:
        // https://github.com/knurling-rs/defmt/blob/defmt-v0.3.1/macros/src/construct.rs
        format!("{:?}", ident.span()).hash(&mut hasher);
    }

    // Get the hash from the hasher and return it as 16 hexadecimal characters
    let s = format!("{:016x}", hasher.finish());
    TokenTree::Literal(Literal::string(&s)).into()
}

#[proc_macro_attribute]
#[allow(missing_docs)]
pub fn interface(attr: TokenStream, item: TokenStream) -> TokenStream {
    match (move || match syn::parse(attr)? {
        crate::interface::MacroArgs::Intern(args) => {
            let item_struct = syn::parse(item)?;
            crate::interface::internal::item_interface(args, item_struct)
        }
        crate::interface::MacroArgs::Extern(args) => {
            let item_type = syn::parse(item)?;
            crate::interface::external::item_interface(args, item_type)
        }
    })() {
        Ok(value) => value.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

#[proc_macro_attribute]
#[allow(missing_docs)]
pub fn protocol(attr: TokenStream, item: TokenStream) -> TokenStream {
    match (move || {
        let args = syn::parse::<crate::protocol::MacroArgs>(attr)?.into();
        let item_trait = syn::parse(item)?;
        crate::protocol::item_trait(args, item_trait)
    })() {
        Ok(value) => value.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn process_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    match (move || {
        let attr = proc_macro2::TokenStream::from(attr);
        if !attr.is_empty() {
            return Err(syn::Error::new(
                attr.span(),
                "objc2: unexpected arguments for `extern_enum",
            ));
        }
        let syn::ItemEnum {
            attrs: mut enum_attrs,
            vis,
            ident: enum_ident,
            generics,
            variants,
            ..
        } = syn::parse::<syn::ItemEnum>(item)?;
        if !generics.params.is_empty() {
            return Err(syn::Error::new(
                generics.span(),
                "objc2: unexpected generics for `extern_enum`",
            ));
        }
        if !generics.where_clause.is_none() {
            return Err(syn::Error::new(
                generics.where_clause.span(),
                "objc2: unexpected where-clause for `extern_enum`",
            ));
        }
        let mut underlying = None;
        for i in 0..enum_attrs.len() {
            if enum_attrs[i].path.is_ident("underlying") {
                let attr = enum_attrs.remove(i);
                underlying = Some(syn::parse2::<syn::TypeParen>(attr.tokens)?);
                break;
            }
        }
        if let Some(syn::TypeParen { elem: ty, .. }) = underlying {
            let mut tokens = proc_macro2::TokenStream::new();
            if enum_ident != "__anonymous__" {
                tokens.append_all(quote!(
                    #(#enum_attrs)*
                    #vis type #enum_ident = #ty;
                ));
            }
            for variant @ syn::Variant {
                attrs: variant_attrs,
                ident: variant_ident,
                fields,
                discriminant,
            } in variants.iter()
            {
                if !fields.is_empty() {
                    return Err(syn::Error::new(
                        fields.span(),
                        "objc2: unexpected fields for variant",
                    ));
                }
                if let Some((_, expr)) = &discriminant {
                    tokens.append_all(quote!(
                        #(#variant_attrs)*
                        #vis const #variant_ident: #ty = #expr;
                    ));
                } else {
                    return Err(syn::Error::new(
                        variant.span(),
                        "objc2: expecting explicit discriminant for variant",
                    ));
                }
            }
            Ok(tokens)
        } else {
            Err(syn::Error::new(
                Span::call_site(),
                "objc2: couldn't find `#[underlying(<ty>)]` attribute",
            ))
        }
    })() {
        Ok(value) => value.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

#[proc_macro_attribute]
#[allow(missing_docs)]
pub fn extern_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    process_enum(attr, item)
}

#[proc_macro_attribute]
#[allow(missing_docs)]
pub fn ns_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    process_enum(attr, item)
}

#[proc_macro_attribute]
#[allow(missing_docs)]
pub fn ns_options(attr: TokenStream, item: TokenStream) -> TokenStream {
    process_enum(attr, item)
}

#[proc_macro_attribute]
#[allow(missing_docs)]
pub fn ns_closed_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    process_enum(attr, item)
}

#[proc_macro_attribute]
#[allow(missing_docs)]
pub fn ns_error_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    process_enum(attr, item)
}
