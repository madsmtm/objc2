use clang::Entity;
use proc_macro2::TokenStream;
use quote::quote;

mod availability;
mod config;
mod method;
mod objc2_utils;
mod rust_type;
mod stmt;

pub use self::config::Config;

use self::stmt::Stmt;

pub fn create_rust_file(entities: &[Entity<'_>], config: &Config) -> TokenStream {
    let iter = entities
        .iter()
        .filter_map(|entity| Stmt::parse(entity, config));
    quote! {
        #[allow(unused_imports)]
        use objc2::{ClassType, extern_class, msg_send, msg_send_id};
        #[allow(unused_imports)]
        use objc2::rc::{Id, Shared};

        #(#iter)*
    }
}
