//! Perform analyses that requires a full information about the parsed output.
//!
//! Try to keep these as few as possible.
use std::collections::BTreeMap;
use std::mem;

use crate::method::Method;
use crate::module::Module;
use crate::stmt::Stmt;
use crate::Library;

pub fn global_analysis(library: &mut Library) {
    update_module(&mut library.module);
}

fn update_module(module: &mut Module) {
    // disambiguate duplicate names
    // NOTE: this only works within single files
    let mut names = BTreeMap::<(String, String), &mut Method>::new();
    for stmt in module.stmts.iter_mut() {
        match stmt {
            Stmt::ExternMethods {
                cls: id, methods, ..
            }
            | Stmt::ExternCategory {
                cls: id, methods, ..
            }
            | Stmt::ProtocolDecl { id, methods, .. } => {
                for method in methods.iter_mut() {
                    let key = (id.clone().name, method.fn_name.clone());
                    if let Some(other) = names.get_mut(&key) {
                        match (method.is_class, other.is_class) {
                            // Assume that the methods clashed because one
                            // of them was a class method
                            (true, false) => {
                                method.fn_name += "_class";
                            }
                            (false, true) => {
                                other.fn_name += "_class";
                            }
                            // Otherwise assume that they clashed because
                            // one of them were `myMethod:`, while the
                            // other were `myMethod`.
                            (true, true) | (false, false) => {
                                other.fn_name = other.selector.replace(':', "_");
                                method.fn_name = method.selector.replace(':', "_");
                            }
                        }
                    } else {
                        names.insert(key, method);
                    }
                }
            }
            _ => {}
        }
    }

    // Fix up a few typedef + enum declarations
    let mut iter = mem::take(&mut module.stmts).into_iter().peekable();
    while let Some(stmt) = iter.next() {
        if let Stmt::AliasDecl {
            id,
            availability: _,
            ty,
            kind: None,
        } = &stmt
        {
            if let Some(Stmt::EnumDecl {
                id: enum_id,
                availability: _,
                ty: enum_ty,
                kind: _,
                variants: _,
                sendable: _,
            }) = iter.peek_mut()
            {
                if enum_ty.is_typedef_to(&id.name) {
                    *enum_id = id.clone();
                    *enum_ty = ty.clone();
                    // Skip adding the now-redundant alias to the list of statements
                    continue;
                }
            }
        }
        module.stmts.push(stmt);
    }

    // Recurse for submodules
    for (name, module) in &mut module.submodules {
        let _span = debug_span!("file", name).entered();
        update_module(module);
    }
}
