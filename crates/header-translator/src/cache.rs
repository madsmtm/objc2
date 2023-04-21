use std::collections::BTreeMap;
use std::mem;

use crate::availability::Availability;
use crate::config::{ClassData, Config};
use crate::file::File;
use crate::id::ItemIdentifier;
use crate::method::Method;
use crate::output::Output;
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
            .flat_map(|cache| cache.methods.iter().map(|m| m.id()))
    }
}

/// A helper struct for doing global analysis on the output.
#[derive(Debug, PartialEq, Clone)]
pub struct Cache<'a> {
    classes: BTreeMap<ItemIdentifier, ClassCache>,
    config: &'a Config,
}

impl<'a> Cache<'a> {
    pub fn new(output: &Output, config: &'a Config) -> Self {
        let mut classes: BTreeMap<_, ClassCache> = BTreeMap::new();

        for (name, library) in &output.libraries {
            let _span = debug_span!("library", name).entered();
            for (name, file) in &library.files {
                let _span = debug_span!("file", name).entered();
                for stmt in &file.stmts {
                    if let Some((cls, method_cache)) = Self::cache_stmt(stmt) {
                        let cache = classes.entry(cls.clone()).or_default();
                        cache.to_emit.push(method_cache);
                    }
                }
            }
        }

        Self { classes, config }
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
            let category = category.clone().with_new_path(cls);
            Some((
                cls,
                MethodCache {
                    availability: availability.clone(),
                    methods,
                    category,
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
        // disambiguate duplicate names
        // NOTE: this only works within single files
        let mut names = BTreeMap::<(ItemIdentifier, String), &mut Method>::new();
        for stmt in file.stmts.iter_mut() {
            match stmt {
                Stmt::Methods {
                    cls: id, methods, ..
                }
                | Stmt::ProtocolDecl { id, methods, .. } => {
                    for method in methods.iter_mut() {
                        let key = (id.clone(), method.fn_name.clone());
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

        let mut new_stmts = Vec::new();
        for stmt in &mut file.stmts {
            #[allow(clippy::single_match)] // There will be others
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
                                let methods: Vec<_> = cache
                                    .methods
                                    .iter()
                                    .filter(|method| !seen_methods.contains(&method.id()))
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
                _ => {}
            }
        }
        file.stmts.extend(new_stmts);

        // Fix up a few typedef + enum declarations
        let mut iter = mem::take(&mut file.stmts).into_iter().peekable();
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
}
