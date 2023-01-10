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

    pub fn cargo_features(&self, _config: &Config) -> BTreeMap<String, Vec<String>> {
        let mut features = BTreeMap::new();

        // for (_, library) in &config.libraries {
        //
        // }

        for (library_name, library) in &self.libraries {
            let mut library_features = BTreeSet::new();

            for (_, file) in &library.files {
                for stmt in &file.stmts {
                    match stmt {
                        Stmt::ClassDecl {
                            ty, superclasses, ..
                        } => {
                            let feature = format!("{}_{}", ty.library, ty.name);
                            // Only require the first superclass as feature,
                            // since the rest will be enabled transitively.
                            if let Some(superclass) = superclasses.first() {
                                let superclass_features = (superclass.library != "Foundation"
                                    && superclass.name != "NSObject")
                                    .then(|| format!("{}_{}", superclass.library, superclass.name))
                                    .into_iter()
                                    .collect::<Vec<_>>();
                                if let Some(existing) =
                                    features.insert(feature.clone(), superclass_features)
                                {
                                    error!(?existing, "duplicate feature");
                                }
                            }

                            if !library_features.insert(feature) {
                                error!("duplicate feature");
                            }
                        }
                        _ => {}
                    }
                }
            }

            let _ = features.insert(
                format!("{library_name}_all"),
                library_features.into_iter().collect::<Vec<_>>(),
            );
        }

        features
    }
}
