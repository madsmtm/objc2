use std::collections::BTreeMap;

use tracing::{debug_span, warn};

use crate::availability::Availability;
use crate::config::{ClassData, Config};
use crate::file::File;
use crate::method::Method;
use crate::output::Output;
use crate::rust_type::{GenericType, Ownership};
use crate::stmt::Stmt;

#[derive(Debug, PartialEq, Clone)]
struct MethodCache {
    availability: Availability,
    methods: Vec<Method>,
    category_name: Option<String>,
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
pub struct Cache {
    classes: BTreeMap<GenericType, ClassCache>,
    ownership_map: BTreeMap<String, Ownership>,
}

impl Cache {
    pub fn new(output: &Output) -> Self {
        let mut classes: BTreeMap<_, ClassCache> = BTreeMap::new();
        let mut ownership_map: BTreeMap<_, Ownership> = BTreeMap::new();

        for (name, library) in &output.libraries {
            let _span = debug_span!("library", name).entered();
            for (name, file) in &library.files {
                let _span = debug_span!("file", name).entered();
                for stmt in &file.stmts {
                    if let Some((ty, method_cache)) = Self::cache_stmt(stmt) {
                        let cache = classes.entry(ty.clone()).or_default();
                        cache.to_emit.push(method_cache);
                    }
                    if let Stmt::ClassDecl { ty, ownership, .. } = stmt {
                        if *ownership != Ownership::default() {
                            ownership_map.insert(ty.name.clone(), ownership.clone());
                        }
                    }
                }
            }
        }

        Self {
            classes,
            ownership_map,
        }
    }

    fn cache_stmt(stmt: &Stmt) -> Option<(&GenericType, MethodCache)> {
        if let Stmt::Methods {
            ty,
            availability,
            methods,
            category_name,
            description,
        } = stmt
        {
            let _span = debug_span!("Stmt::Methods", %ty).entered();
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
                ty,
                MethodCache {
                    availability: availability.clone(),
                    methods,
                    category_name: category_name.clone(),
                },
            ))
        } else {
            None
        }
    }

    pub fn update(&self, output: &mut Output, configs: &BTreeMap<String, Config>) {
        for (name, library) in &mut output.libraries {
            let _span = debug_span!("library", name).entered();
            let config = configs.get(name).expect("configs get library");
            for (name, file) in &mut library.files {
                let _span = debug_span!("file", name).entered();
                self.update_file(file, config);
            }
        }
    }

    fn update_file(&self, file: &mut File, config: &Config) {
        let mut new_stmts = Vec::new();
        for stmt in &mut file.stmts {
            match stmt {
                Stmt::ClassDecl {
                    ty, superclasses, ..
                } => {
                    let _span = debug_span!("Stmt::ClassDecl", %ty).entered();
                    let data = config.class_data.get(&ty.name);

                    // Used for duplicate checking (sometimes the subclass
                    // defines the same method that the superclass did).
                    let mut seen_methods: Vec<_> = self
                        .classes
                        .get(ty)
                        .map(|cache| cache.all_methods_data())
                        .into_iter()
                        .flatten()
                        .collect();

                    for superclass in superclasses {
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

                                self.update_methods(&mut methods, &ty.name);

                                Some(Stmt::Methods {
                                    ty: ty.clone(),
                                    availability: cache.availability.clone(),
                                    methods,
                                    category_name: cache.category_name.clone(),
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
                Stmt::Methods { ty, methods, .. } => {
                    self.update_methods(methods, &ty.name);
                }
                Stmt::ProtocolDecl { name, methods, .. } => {
                    self.update_methods(methods, name);
                }
                _ => {}
            }
        }
        file.stmts.extend(new_stmts);
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
