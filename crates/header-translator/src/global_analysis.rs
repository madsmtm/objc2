//! Perform analyses that requires full information about the parsed output.
//!
//! Try to keep these as few as possible, since they have a hard time
//! inspecting other crates.
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::mem;

use crate::availability::Availability;
use crate::expr::Expr;
use crate::id::ItemTree;
use crate::method::Method;
use crate::module::Module;
use crate::name_translation::{find_fn_implementor, shorten_name_when_on_parent};
use crate::stmt::Stmt;
use crate::{Config, ItemIdentifier, Library};

pub fn global_analysis(library: &mut Library, config: &Config) {
    let _span = info_span!("analyzing").entered();

    // Create a list of implement-able items.
    let mut implementable_mapping = create_implementable_mapping(&library.module);

    for (external_name, external_data) in &library.data.external {
        implementable_mapping.insert(ItemTree::from_id(
            external_data.clone().into_id(external_name.clone()),
        ));
    }

    let mut expected_bridged_types = config
        .libraries
        .values()
        .flat_map(|data| &data.class_data)
        .filter_map(|(name, data)| data.bridged_to.as_ref().map(|bridged| (&**name, bridged)))
        .filter(|(_, bridged)| bridged.library_name() == library.link_name)
        .collect();

    let ident_mapping = create_ident_mapping(&library.module);
    update_module(
        &mut library.module,
        &implementable_mapping,
        &ident_mapping,
        &mut expected_bridged_types,
    );

    if !expected_bridged_types.is_empty() {
        warn!("too many bridged-to in config: {expected_bridged_types:?}");
    }
}

fn create_implementable_mapping(module: &Module) -> BTreeSet<ItemTree> {
    let mut types = BTreeSet::new();
    for stmt in &module.stmts {
        if stmt.implementable() {
            types.insert(ItemTree::new(
                stmt.provided_item().unwrap(),
                stmt.required_items(),
            ));
        }
    }
    for submodule in module.submodules.values() {
        types.extend(create_implementable_mapping(submodule));
    }
    types
}

// TODO: Maybe do this only within the context of a single file?
fn create_ident_mapping(module: &Module) -> HashMap<String, Expr> {
    let mut mapping = HashMap::new();
    for stmt in &module.stmts {
        mapping.extend(stmt.get_ident_mapping());
    }
    for submodule in module.submodules.values() {
        mapping.extend(create_ident_mapping(submodule));
    }
    mapping
}

fn update_module(
    module: &mut Module,
    implementable_mapping: &BTreeSet<ItemTree>,
    ident_mapping: &HashMap<String, Expr>,
    expected_bridged_types: &mut BTreeMap<&str, &ItemIdentifier>,
) {
    // Fix location for GetTypeId functions
    for stmt in module.stmts.iter_mut() {
        if let Stmt::FnDecl {
            id,
            c_name,
            link_name,
            availability,
            arguments,
            first_arg_is_self,
            result_type,
            body,
            safe: _,
            must_use,
            abi,
            returns_retained,
            documentation,
            no_implementor,
            custom_implementor,
        } = stmt
        {
            if *no_implementor {
                continue;
            }

            let implementor = if let Some(implementor) = custom_implementor {
                Some(implementor.clone())
            } else {
                find_fn_implementor(
                    implementable_mapping,
                    c_name,
                    id.location(),
                    arguments,
                    result_type,
                )
            };

            if let Some(parent_item) = implementor {
                let is_instance_method = arguments
                    .first()
                    .is_some_and(|(_, arg_ty)| arg_ty.is_self_ty_legal(parent_item.id()));
                let omit_memory_management_words =
                    result_type.fn_return(*returns_retained).1.is_some();

                let name = if id.name != *c_name {
                    // Has been renamed already
                    id.name.clone()
                } else {
                    shorten_name_when_on_parent(
                        c_name,
                        &parent_item.id().name,
                        is_instance_method,
                        omit_memory_management_words,
                    )
                };

                *id = id.clone().map_name(|_| name.clone());

                if is_instance_method {
                    *first_arg_is_self = true;
                }

                documentation.set_alias(c_name.clone());

                // Wrappers have normal Rust ABI (mostly to unclutter docs).
                *abi = abi.as_rust_outer();

                if link_name.contains("GetTypeID") {
                    assert!(arguments.is_empty(), "{id:?} must have no arguments");
                    assert!(result_type.is_cf_type_id(), "{id:?} must return CFTypeID");
                    assert!(body.is_none(), "{id:?} must not be inline");
                    assert!(!*must_use, "{id:?} must not have must_use");
                    assert!(!*returns_retained, "{id:?} must not have returns_retained");

                    *stmt = Stmt::FnGetTypeId {
                        id: id.clone(),
                        cf_item: parent_item,
                        link_name: link_name.clone(),
                        result_type: result_type.clone(),
                        availability: availability.clone(),
                        abi: abi.clone(),
                        documentation: documentation.clone(),
                    };
                } else {
                    let location = id.location().clone();
                    let fn_stmt = mem::replace(
                        stmt,
                        Stmt::GeneralImpl {
                            location,
                            item: parent_item,
                            stmts: vec![],
                        },
                    );
                    if let Stmt::GeneralImpl { stmts, .. } = stmt {
                        stmts.push(fn_stmt);
                    } else {
                        unreachable!()
                    }
                }
            }
        }
    }

    // Propagate availability information of `init` to `new`.
    // NOTE: this only works within single files.
    let init_availability: BTreeMap<String, Availability> = module
        .stmts
        .iter()
        .filter_map(|stmt| match stmt {
            Stmt::ExternMethods {
                cls: id, methods, ..
            }
            | Stmt::ExternCategory {
                cls: id, methods, ..
            }
            | Stmt::ProtocolDecl { id, methods, .. } => Some(
                methods
                    .iter()
                    .filter(|method| method.selector == "init")
                    .map(|method| (id.name.clone(), method.availability.clone())),
            ),
            _ => None,
        })
        .flatten()
        .collect();
    for stmt in module.stmts.iter_mut() {
        if let Stmt::ExternMethods {
            cls: id, methods, ..
        }
        | Stmt::ExternCategory {
            cls: id, methods, ..
        }
        | Stmt::ProtocolDecl { id, methods, .. } = stmt
        {
            if let Some(init_availability) = init_availability.get(&id.name) {
                if let Some(method) = methods.iter_mut().find(|method| method.selector == "new") {
                    method
                        .availability
                        .method_update_new_from_init(init_availability);
                    // TODO(breaking): Remove unavailable instead of just marking them unsafe.
                    if !method.availability.is_available() {
                        method.safe = false;
                    }
                }
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
        if let Stmt::RecordDecl { id: record_id, .. } = &mut stmt {
            if let Some(Stmt::AliasDecl {
                id: typedef_id,
                ty,
                kind: None,
                ..
            }) = iter.peek()
            {
                if ty.is_record(&record_id.name)
                    && typedef_id.name == record_id.name.trim_start_matches("_")
                {
                    *record_id = typedef_id.clone();
                    // Skip adding the now-redundant alias to the list of statements
                    let _ = iter.next();
                }
            }
        }

        // Fix expressions in #define constants
        if let Stmt::ConstDecl { id, value, .. } = &mut stmt {
            let has_unknowns = value.update_idents(ident_mapping);
            if has_unknowns {
                warn!(name = ?id.name, "did not emit const, it had unknown values");
                continue;
            }
        }

        module.stmts.push(stmt);
    }

    // Merge Stmt::GeneralImpl
    let mut iter = mem::take(&mut module.stmts).into_iter().peekable();
    while let Some(mut stmt) = iter.next() {
        if let Stmt::GeneralImpl { item, stmts, .. } = &mut stmt {
            while let Some(Stmt::GeneralImpl {
                item: item_next, ..
            }) = iter.peek()
            {
                if item == item_next {
                    if let Some(Stmt::GeneralImpl {
                        stmts: next_stmts, ..
                    }) = iter.next()
                    {
                        stmts.extend(next_stmts);
                    } else {
                        unreachable!()
                    }
                } else {
                    break;
                }
            }
        }

        module.stmts.push(stmt);
    }

    // Check bridged types.
    for stmt in &module.stmts {
        if let Stmt::OpaqueDecl {
            id,
            bridged: Some(bridged_class),
            ..
        } = stmt
        {
            if let Some(bridged_typedef) = expected_bridged_types.remove(&**bridged_class) {
                if bridged_typedef != id {
                    warn!("incorrect bridged typedef for {bridged_class}: found `{bridged_typedef}`, expected `{id}`");
                }
            } else {
                warn!("missing bridging decl, add:    class.{bridged_class}.bridged-to = \"{id}\"");
            }
        }
    }

    // Recurse for submodules
    for (name, module) in &mut module.submodules {
        let _span = debug_span!("file", name).entered();
        update_module(
            module,
            implementable_mapping,
            ident_mapping,
            expected_bridged_types,
        );
    }
}
