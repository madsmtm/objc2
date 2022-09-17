use std::collections::HashSet;

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

pub fn create_rust_file(
    entities: &[Entity<'_>],
    config: &Config,
) -> (HashSet<String>, TokenStream) {
    let stmts: Vec<_> = entities
        .iter()
        .filter_map(|entity| Stmt::parse(entity, config))
        .collect();

    let mut declared_types = HashSet::new();
    for stmt in stmts.iter() {
        match stmt {
            Stmt::ClassDecl { name, .. } => {
                declared_types.insert(name.clone());
            }
            Stmt::ProtocolDecl { name, .. } => {
                declared_types.insert(name.clone());
            }
            _ => {}
        }
    }

    let iter = stmts.iter().filter(|stmt| match stmt {
        Stmt::ItemImport { name } => !declared_types.contains(name),
        _ => true,
    });

    let tokens = quote! {
        #[allow(unused_imports)]
        use objc2::{ClassType, extern_class, msg_send, msg_send_id};
        #[allow(unused_imports)]
        use objc2::rc::{Id, Shared};

        #(#iter)*
    };

    (declared_types, tokens)
}
