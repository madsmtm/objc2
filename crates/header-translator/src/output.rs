use std::collections::{BTreeMap, BTreeSet};

use crate::config::Config;
use crate::library::Library;
use crate::stmt::Stmt;

#[derive(Debug, PartialEq)]
pub struct Output {
    pub libraries: BTreeMap<String, Library>,
}

impl Output {
    pub fn from_libraries(libraries: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let libraries = libraries
            .into_iter()
            .map(|name| (name.into(), Library::new()))
            .collect();
        Self { libraries }
    }

    pub fn compare(&self, other: &Self) {
        super::compare_btree(
            &self.libraries,
            &other.libraries,
            |libary_name, self_library, other_library| {
                let _span = debug_span!("library", libary_name).entered();
                self_library.compare(other_library);
            },
        );
    }

    pub fn cargo_features(&self, config: &Config) -> BTreeMap<String, Vec<String>> {
        let mut features = BTreeMap::new();

        // for (_, library) in &config.libraries {
        //
        // }

        for (library_name, library) in &self.libraries {
            let library_alias = config.get_library_alias(library_name.clone());
            let mut library_features = BTreeSet::from([library_alias.clone()]);

            for file in library.files.values() {
                for stmt in &file.stmts {
                    #[allow(clippy::single_match)] // There will be others
                    match stmt {
                        Stmt::ClassDecl {
                            id, superclasses, ..
                        } => {
                            if let Some(feature) = id.feature() {
                                // Only require the first superclass as feature,
                                // since the rest will be enabled transitively.
                                if let Some((superclass, _)) = superclasses.first() {
                                    let superclass_features: Vec<_> = superclass
                                        .feature()
                                        .map(|f| f.to_string())
                                        .into_iter()
                                        .collect();
                                    if let Some(existing) =
                                        features.insert(feature.to_string(), superclass_features)
                                    {
                                        error!(?existing, "duplicate feature");
                                    }
                                }

                                if !library_features.insert(feature.to_string()) {
                                    error!("duplicate feature");
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }

            let _ = features.insert(
                format!("{library_alias}_all"),
                library_features.into_iter().collect::<Vec<_>>(),
            );
        }

        features
    }
}
