use std::collections::BTreeMap;
use std::mem;

use crate::availability::Availability;
use crate::config::{ClassData, Config};
use crate::file::File;
use crate::id::ItemIdentifier;
use crate::method::Method;
use crate::output::Output;
use crate::rust_type::Ownership;
use crate::stmt::Stmt;

#[derive(Debug, PartialEq, Clone)]
struct MethodCache {
    availability: Availability,
    methods: Vec<Method>,
    category: ItemIdentifier<Option<String>>,
}

#[derive(Debug, PartialEq, Clone, Default)]
struct ClassCache {
    /// Methods that should be duplicated onto any subclass.
    to_emit: Vec<MethodCache>,
    // We don't need availability here, since a superclass' availability
    // should always be greater than the subclass'.
}

impl ClassCache {
    fn all_methods_data(&self) -> impl Iterator<Item = (bool, &str)> {
        self.to_emit
            .iter()
            .flat_map(|cache| cache.methods.iter().map(|m| (m.is_class, &*m.selector)))
    }
}

/// A helper struct for doing global analysis on the output.
#[derive(Debug, PartialEq, Clone)]
pub struct Cache<'a> {
    classes: BTreeMap<ItemIdentifier, ClassCache>,
    ownership_map: BTreeMap<String, Ownership>,
    config: &'a Config,
}

impl<'a> Cache<'a> {
    pub fn new(output: &Output, config: &'a Config) -> Self {
        let mut classes: BTreeMap<_, ClassCache> = BTreeMap::new();
        let mut ownership_map: BTreeMap<_, Ownership> = BTreeMap::new();

        for (name, library) in &output.libraries {
            let _span = debug_span!("library", name).entered();
            for (name, file) in &library.files {
                let _span = debug_span!("file", name).entered();
                for stmt in &file.stmts {
                    if let Some((cls, method_cache)) = Self::cache_stmt(stmt) {
                        let cache = classes.entry(cls.clone()).or_default();
                        cache.to_emit.push(method_cache);
                    }
                    if let Stmt::ClassDecl { id, ownership, .. } = stmt {
                        if *ownership != Ownership::default() {
                            ownership_map.insert(id.name.clone(), ownership.clone());
                        }
                    }
                }
            }
        }

        Self {
            classes,
            ownership_map,
            config,
        }
    }

    fn cache_stmt(stmt: &Stmt) -> Option<(&ItemIdentifier, MethodCache)> {
        if let Stmt::Methods {
            cls,
            generics: _,
            category,
            availability,
            superclasses: _,
            methods,
            description,
        } = stmt
        {
            let _span = debug_span!("Stmt::Methods", ?cls).entered();
            let methods: Vec<Method> = methods
                .iter()
                .filter(|method| method.emit_on_subclasses())
                .cloned()
                .collect();
            if methods.is_empty() {
                return None;
            }
            if description.is_some() {
                warn!(description, "description was set");
            }
            Some((
                cls,
                MethodCache {
                    availability: availability.clone(),
                    methods,
                    category: category.clone(),
                },
            ))
        } else {
            None
        }
    }

    pub fn update(&self, output: &mut Output) {
        for (name, library) in &mut output.libraries {
            let _span = debug_span!("library", name).entered();
            for (name, file) in &mut library.files {
                let _span = debug_span!("file", name).entered();
                self.update_file(file);
            }
        }
    }

    fn update_file(&self, file: &mut File) {
        let mut new_stmts = Vec::new();
        for stmt in &mut file.stmts {
            match stmt {
                Stmt::ClassDecl {
                    id,
                    generics,
                    superclasses,
                    ..
                } => {
                    let _span = debug_span!("Stmt::ClassDecl", ?id).entered();
                    let data = self.config.class_data.get(&id.name);

                    // Used for duplicate checking (sometimes the subclass
                    // defines the same method that the superclass did).
                    let mut seen_methods: Vec<_> = self
                        .classes
                        .get(id)
                        .map(|cache| cache.all_methods_data())
                        .into_iter()
                        .flatten()
                        .collect();

                    for (superclass, _) in &*superclasses {
                        if let Some(cache) = self.classes.get(superclass) {
                            new_stmts.extend(cache.to_emit.iter().filter_map(|cache| {
                                let mut methods: Vec<_> = cache
                                    .methods
                                    .iter()
                                    .filter(|method| {
                                        !seen_methods.contains(&(method.is_class, &method.selector))
                                    })
                                    .filter_map(|method| {
                                        method.clone().update(ClassData::get_method_data(
                                            data,
                                            &method.fn_name,
                                        ))
                                    })
                                    .collect();
                                if methods.is_empty() {
                                    return None;
                                }

                                self.update_methods(&mut methods, &id.name);

                                Some(Stmt::Methods {
                                    cls: id.clone(),
                                    generics: generics.clone(),
                                    category: cache.category.clone(),
                                    availability: cache.availability.clone(),
                                    superclasses: superclasses.clone(),
                                    methods,
                                    description: Some(format!(
                                        "Methods declared on superclass `{}`",
                                        superclass.name
                                    )),
                                })
                            }));

                            seen_methods.extend(cache.all_methods_data());
                        }
                    }
                }
                Stmt::Methods { cls, methods, .. } => {
                    self.update_methods(methods, &cls.name);
                }
                Stmt::ProtocolDecl { id, methods, .. } => {
                    self.update_methods(methods, &id.name);
                }
                _ => {}
            }
        }
        file.stmts.extend(new_stmts);

        // Fix up a few typedef + enum declarations
        let mut iter = mem::take(&mut file.stmts).into_iter().peekable();
        while let Some(stmt) = iter.next() {
            if let Stmt::AliasDecl { id, ty, kind: None } = &stmt {
                if let Some(Stmt::EnumDecl {
                    id: enum_id,
                    ty: enum_ty,
                    ..
                }) = iter.peek_mut()
                {
                    if enum_ty.is_typedef_to(&id.name) {
                        *enum_id = id.clone().to_some();
                        *enum_ty = ty.clone();
                        // Skip adding the now-redundant alias to the list of statements
                        continue;
                    }
                }
            }
            file.stmts.push(stmt);
        }
    }

    fn update_methods(&self, methods: &mut [Method], self_means: &str) {
        for method in methods {
            // Beware! We make instance methods return `Owned` as well, though
            // those are basically never safe (since they'd refer to mutable
            // data without a lifetime tied to the primary owner).
            method.result_type.set_ownership(|name| {
                let name = if name == "Self" { self_means } else { name };

                self.ownership_map.get(name).cloned().unwrap_or_default()
            });
        }
    }
}
