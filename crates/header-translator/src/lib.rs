use clang::Entity;
use proc_macro2::TokenStream;
use quote::quote;

mod availability;
mod method;
mod objc2_utils;
mod rust_type;
mod stmt;

use self::stmt::Stmt;

pub fn create_rust_file(entities: &[Entity<'_>]) -> TokenStream {
    let iter = entities.iter().filter_map(Stmt::parse);
    quote! {
        #(#iter)*
    }
}
