use std::collections::BTreeMap;

use crate::library::Library;
use crate::stmt::Stmt;
use crate::config::Config;

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

        for (_, library) in &self.libraries {
            for (_, file) in &library.files {
                for stmt in &file.stmts {
                    match stmt {
                        Stmt::ClassDecl {
                            ty, superclasses, ..
                        } => {
                            // Only require the first superclass as feature,
                            // since the rest will be enabled transitively.
                            if let Some(superclass) = superclasses.first() {
                                let feature = format!("{}_{}", ty.library, ty.name);
                                let superclass_feature = format!("{}_{}", superclass.library, superclass.name);
                                if let Some(existing) = features.insert(feature, vec![superclass_feature]) {
                                    error!(?existing, "duplicate feature");
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        features
    }
}
