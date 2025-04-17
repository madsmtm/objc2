//! Perform analyses that requires full information about the parsed output.
//!
//! Try to keep these as few as possible, since they have a hard time
//! inspecting other crates.
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::mem;

use crate::availability::Availability;
use crate::documentation::Documentation;
use crate::expr::Expr;
use crate::id::ItemTree;
use crate::method::Method;
use crate::module::Module;
use crate::name_translation::cf_fn;
use crate::rust_type::Ty;
use crate::stmt::Stmt;
use crate::{ItemIdentifier, Library};

pub fn global_analysis(library: &mut Library) {
    let _span = info_span!("analyzing").entered();

    // Create a list of implement-able items.
    let mut implementable_mapping = create_implementable_mapping(&library.module);

    for (external_name, external_data) in &library.data.external {
        implementable_mapping.insert(ItemTree::from_id(ItemIdentifier::from_raw(
            external_name.clone(),
            external_data.module.clone(),
        )));
    }

    let ident_mapping = create_ident_mapping(&library.module);
    update_module(&mut library.module, &implementable_mapping, &ident_mapping);
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
) {
    let mut deprecated_fns = vec![];

    // Fix location for GetTypeId functions
    for stmt in module.stmts.iter_mut() {
        if let Stmt::FnDecl {
            id,
            renamed,
            availability,
            arguments,
            first_arg_is_self,
            result_type,
            body,
            safe,
            must_use,
            abi,
            link_name,
            returns_retained,
            documentation,
        } = stmt
        {
            let omit_memory_management_words = result_type.fn_return(*returns_retained).1.is_some();
            let implementor = find_implementor(
                implementable_mapping,
                id,
                arguments,
                result_type,
                omit_memory_management_words,
            );
            if let Some((cf_item, name)) = implementor {
                if id.library_name() != cf_item.id().library_name() {
                    // Cannot emit into different library (or it at least requires a helper trait).
                    continue;
                }
                if name == "type_id" {
                    assert!(arguments.is_empty(), "{id:?} must have no arguments");
                    assert!(result_type.is_cf_type_id(), "{id:?} must return CFTypeID");
                    assert!(body.is_none(), "{id:?} must not be inline");
                    assert!(!*safe, "{id:?} must not have manually modified safety");
                    assert!(!*must_use, "{id:?} must not have must_use");
                    assert!(link_name.is_none(), "{id:?} must not have link_name");
                    assert!(!*returns_retained, "{id:?} must not have returns_retained");

                    *stmt = Stmt::FnGetTypeId {
                        id: id.clone(),
                        cf_id: cf_item.id().clone(),
                        result_type: result_type.clone(),
                        availability: availability.clone(),
                        abi: abi.as_rust_outer(),
                        documentation: documentation.clone(),
                    };
                } else {
                    // TODO(breaking): Remove in next version
                    if body.is_none() {
                        deprecated_fns.push(Stmt::FnDecl {
                            id: id.clone(),
                            renamed: None, // Never rename these
                            availability: Availability::new_deprecated(format!(
                                "renamed to `{}::{}`",
                                cf_item.id().name,
                                name,
                            )),
                            arguments: arguments.clone(),
                            result_type: result_type.clone(),
                            first_arg_is_self: *first_arg_is_self,
                            body: *body,
                            safe: *safe,
                            must_use: *must_use,
                            abi: abi.clone(),
                            link_name: link_name.clone(),
                            returns_retained: *returns_retained,
                            documentation: Documentation::empty(),
                        });
                    }

                    if renamed.is_none() {
                        *renamed = Some(name);
                    }
                    if arguments
                        .first()
                        .is_some_and(|(_, arg_ty)| arg_ty.is_self_ty_legal(cf_item.id()))
                    {
                        *first_arg_is_self = true;
                    }
                    // Wrappers have normal Rust ABI (mostly to unclutter docs).
                    *abi = abi.as_rust_outer();

                    let location = id.location().clone();
                    let fn_stmt = mem::replace(
                        stmt,
                        Stmt::GeneralImpl {
                            location,
                            item: cf_item,
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

    module.stmts.extend(deprecated_fns);

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

    // Recurse for submodules
    for (name, module) in &mut module.submodules {
        let _span = debug_span!("file", name).entered();
        update_module(module, implementable_mapping, ident_mapping);
    }
}

/// Find the type onto whom the function should be inserted.
fn find_implementor(
    implementable_mapping: &BTreeSet<ItemTree>,
    fn_id: &ItemIdentifier,
    arguments: &[(String, Ty)],
    result_type: &Ty,
    omit_memory_management_words: bool,
) -> Option<(ItemTree, String)> {
    // TODO: Maybe do this only within the context of a single file?

    let mut found = None;

    // Check the first argument, and see if that matches.
    if let Some((_, first_arg_ty)) = arguments.first() {
        // Skip CFAllocator if that's the first argument.
        // Useful for e.g. `CGEventCreateData` and `CFDateFormatterCreateDateFromString`.
        let mut first_arg_ty = first_arg_ty;
        if first_arg_ty.is_cf_allocator() && !fn_id.name.starts_with("CFAllocator") {
            // TODO: Consider shuffling around so that the allocator becomes
            // the second argument?
            if let Some((_, arg_ty)) = arguments.get(1) {
                first_arg_ty = arg_ty;
            }
        }

        if let Some(type_id) = first_arg_ty.implementable() {
            if let Some(name) = cf_fn(
                &fn_id.name,
                &type_id.id().name,
                true,
                omit_memory_management_words,
            ) {
                found = Some((type_id.clone(), name.to_string()));
            }
        }
    }

    // Check the return type, and see if that matches.
    //
    // Letting this override the above means that things like
    // `CGPathCreateMutableCopy` are considered part of `CFMutablePath`. Same
    // for `CFStringTokenizerCreate` and `CFDateFormatterCreateDateFromString`.
    if let Some(type_id) = result_type.implementable() {
        if let Some(name) = cf_fn(
            &fn_id.name,
            &type_id.id().name,
            true,
            omit_memory_management_words,
        ) {
            found = Some((type_id.clone(), name.to_string()));
        } else {
            // TODO: Special-case CFArray returns?
        }
    }

    if let Some((type_id, name)) = found {
        return Some((type_id, name));
    }

    // If none of the type matches above worked, look for a type that matches
    // the name as closely as possible.
    let mut best_match: Option<(ItemTree, String)> = None;

    for item in implementable_mapping {
        // Ignore most functions that are not in the same file as the
        // implementor, to avoid cases where the function has nothing to do
        // with the implementor. Relevant examples from Foundation include
        // NSSetUncaughtExceptionHandler and NSHostByteOrder.
        // TODO: Is this worth the effort?
        if !fn_id.location().semi_part_of(item.id().location())
            && !fn_id.name.contains("GetTypeID")
            // FIXME: CFString, CFPlugIn and CFTimeZone are defined in other
            // files than their "native" file.
            && !fn_id.name.contains("CFString")
            && !fn_id.name.contains("CFPlugIn")
            && !fn_id.name.contains("CFTimeZone")
            // FIXME: CGEvent is defined in CGEventTypes
            && !fn_id.name.contains("CGEvent")
            // FIXME: A lot of Security types are defined in SecBase.
            && fn_id.library_name() != "Security"
        {
            continue;
        }
        // FIXME: CTFontManager seems to be separate from CTFont.
        // Similarly for ASAuthorizationAllSupportedPublicKeyCredentialDescriptor
        if fn_id.name.contains("CTFontManager")
            || fn_id
                .name
                .contains("ASAuthorizationAllSupportedPublicKeyCredentialDescriptor")
        {
            continue;
        }
        if let Some(name) = cf_fn(
            &fn_id.name,
            &item.id().name,
            false,
            omit_memory_management_words,
        ) {
            if let Some(best_match) = &mut best_match {
                // Best match is the longest type name that prefixes the function.
                if best_match.0.id().name.len() < item.id().name.len() {
                    *best_match = (item.clone(), name);
                }
            } else {
                best_match = Some((item.clone(), name));
            }
        }
    }

    best_match
}
