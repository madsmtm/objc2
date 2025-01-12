//! Perform analyses that requires a full information about the parsed output.
//!
//! Try to keep these as few as possible.
use std::collections::BTreeMap;
use std::mem;

use crate::method::Method;
use crate::module::Module;
use crate::stmt::Stmt;
use crate::{ItemIdentifier, Library};

pub fn global_analysis(library: &mut Library) {
    let _span = info_span!("analyzing").entered();
    let mut cf_type_id_mapping = find_cf_type_id_mapping(&library.module);
    for (external_name, external_data) in &library.data.external {
        if let Some(maybe_type_id_base) = external_name.strip_suffix("Ref") {
            cf_type_id_mapping.insert(
                format!("{maybe_type_id_base}GetTypeID"),
                ItemIdentifier::from_raw(external_name.clone(), external_data.module.clone()),
            );
        }
    }
    update_module(&mut library.module, &cf_type_id_mapping);
}

fn find_cf_type_id_mapping(module: &Module) -> BTreeMap<String, ItemIdentifier> {
    let mut types = BTreeMap::new();
    for stmt in &module.stmts {
        if let Stmt::OpaqueDecl {
            id, is_cf: true, ..
        } = stmt
        {
            let type_id_base = id.name.strip_suffix("Ref").unwrap_or(&id.name);
            types.insert(format!("{type_id_base}GetTypeID"), id.clone());
        }
    }
    for submodule in module.submodules.values() {
        types.extend(find_cf_type_id_mapping(submodule));
    }
    types
}

fn update_module(module: &mut Module, cf_type_id_mapping: &BTreeMap<String, ItemIdentifier>) {
    // Fix location for GetTypeId functions
    for stmt in module.stmts.iter_mut() {
        if let Stmt::FnGetTypeId {
            id,
            cf_id,
            availability,
            result_type,
            can_unwind,
            documentation,
        } = stmt
        {
            if let Some(actual_cf_id) = cf_type_id_mapping.get(&id.name) {
                *cf_id = actual_cf_id.clone();
            } else {
                warn!(?id, ?cf_id, "could not find GetTypeId CF typedef");
                *stmt = Stmt::FnDecl {
                    id: id.clone(),
                    availability: availability.clone(),
                    arguments: vec![],
                    result_type: result_type.clone(),
                    body: None,
                    safe: true,
                    must_use: false,
                    can_unwind: *can_unwind,
                    link_name: None,
                    returns_retained: false,
                    documentation: documentation.clone(),
                };
            }
        }
    }

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

    let mut iter = mem::take(&mut module.stmts).into_iter().peekable();
    while let Some(mut stmt) = iter.next() {
        // Fix up a few enum + typedef declarations. Example:
        //
        // typedef enum _SecureDownloadTrustCallbackResult {
        //     kSecureDownloadDoNotEvaluateSigner = 0,
        //     kSecureDownloadEvaluateSigner = 1,
        //     kSecureDownloadFailEvaluation = 2
        // } SecureDownloadTrustCallbackResult;
        if let Stmt::EnumDecl { id, .. } = &mut stmt {
            if let Some(Stmt::AliasDecl {
                id: typedef_id,
                ty,
                kind: None,
                ..
            }) = iter.peek()
            {
                if ty.is_enum(&id.name) && typedef_id.name == id.name.trim_start_matches("_") {
                    *id = typedef_id.clone();
                    // Skip adding the now-redundant alias to the list of statements
                    let _ = iter.next();
                }
            }
        }

        // Fix up a few struct/union + typedef declarations. Example:
        //
        // typedef struct _AVBeatRange {
        //     AVMusicTimeStamp start;
        //     AVMusicTimeStamp length;
        // } AVBeatRange;
        if let Stmt::RecordDecl {
            id, encoding_name, ..
        } = &mut stmt
        {
            if let Some(Stmt::AliasDecl {
                id: typedef_id,
                ty,
                kind: None,
                ..
            }) = iter.peek()
            {
                if ty.is_record(&id.name) && typedef_id.name == id.name.trim_start_matches("_") {
                    if encoding_name.is_none() {
                        *encoding_name = Some(id.name.clone());
                    }
                    *id = typedef_id.clone();
                    // Skip adding the now-redundant alias to the list of statements
                    let _ = iter.next();
                }
            }
        }

        module.stmts.push(stmt);
    }

    // Recurse for submodules
    for (name, module) in &mut module.submodules {
        let _span = debug_span!("file", name).entered();
        update_module(module, cf_type_id_mapping);
    }
}
