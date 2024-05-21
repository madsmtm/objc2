//! Procedural macros for the [`objc2` project](https://github.com/madsmtm/objc2).
//!
//! You should not need to use this crate directly, all its public items are
//! exported in other crates.

#![warn(missing_docs)]
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::missing_panics_doc)]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-proc-macros/0.1.2")]

use core::hash::{Hash, Hasher};

use proc_macro::Ident;
use proc_macro::Literal;
use proc_macro::TokenStream;
use proc_macro::TokenTree;

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
