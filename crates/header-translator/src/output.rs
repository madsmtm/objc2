use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::{self, Write};
use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::config::{Config, LibraryData};
use crate::library::Library;
use crate::stmt::Stmt;

#[derive(Debug, PartialEq)]
pub struct Output {
    pub libraries: BTreeMap<String, Library>,
}

impl Output {
    pub fn from_libraries(libraries: &HashMap<String, LibraryData>) -> Self {
        let libraries = libraries
            .iter()
            .map(|(name, data)| (name.into(), Library::new(name, data)))
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

    pub fn output_module(&self, path: &Path) -> fmt::Result {
        let mut f = String::new();

        for library_name in self.libraries.keys() {
            writeln!(&mut f, "#[cfg(feature = \"{library_name}\")]")?;
            writeln!(&mut f, "pub mod {library_name};")?;
        }

        fs::write(path, f).unwrap();

        Ok(())
    }

    pub fn cargo_features(&self, config: &Config) -> BTreeMap<String, Vec<String>> {
        let mut features = BTreeMap::new();

        let mut macos_10_7_features: BTreeSet<String> = vec!["unstable-example-basic_usage".into()]
            .into_iter()
            .collect();
        let mut macos_10_13_features: BTreeSet<String> = vec![
            "unstable-frameworks-macos-10-7".into(),
            "unstable-example-delegate".into(),
            "unstable-example-metal".into(),
            "unstable-example-nspasteboard".into(),
            "unstable-example-speech_synthesis".into(),
            "unstable-example-tutorial".into(),
        ]
        .into_iter()
        .collect();
        let mut macos_11_features: BTreeSet<String> =
            vec!["unstable-frameworks-macos-10-13".into()]
                .into_iter()
                .collect();
        let mut macos_12_features: BTreeSet<String> = vec!["unstable-frameworks-macos-11".into()]
            .into_iter()
            .collect();
        let mut macos_13_features: BTreeSet<String> = vec![
            "unstable-frameworks-macos-12".into(),
            "unstable-example-browser".into(),
        ]
        .into_iter()
        .collect();
        let mut gnustep_features: BTreeSet<String> = vec![].into_iter().collect();

        for (mut library_name, library) in &config.libraries {
            if let Some(alias) = &library.name {
                library_name = alias;
            }
            let library_features = library
                .imports
                .iter()
                .chain(library.extra_features.iter())
                .cloned();
            let _ = features.insert(library_name.to_string(), library_features.collect());

            if let Some(version) = &library.macos {
                if version.matches(&semver::Version::from_str("10.7.0").unwrap()) {
                    macos_10_7_features.insert(format!("{library_name}_all"));
                } else if version.matches(&semver::Version::from_str("10.13.0").unwrap()) {
                    macos_10_13_features.insert(format!("{library_name}_all"));
                } else if version.matches(&semver::Version::from_str("11.0.0").unwrap()) {
                    macos_11_features.insert(format!("{library_name}_all"));
                } else if version.matches(&semver::Version::from_str("12.0.0").unwrap()) {
                    macos_12_features.insert(format!("{library_name}_all"));
                } else {
                    macos_13_features.insert(format!("{library_name}_all"));
                }
            }

            if library.gnustep_library.is_some() {
                gnustep_features.insert(format!("{library_name}_all"));
            }
        }

        let _ = features.insert(
            "unstable-frameworks-macos-10-7".into(),
            macos_10_7_features.into_iter().collect(),
        );
        let _ = features.insert(
            "unstable-frameworks-macos-10-13".into(),
            macos_10_13_features.into_iter().collect(),
        );
        let _ = features.insert(
            "unstable-frameworks-macos-11".into(),
            macos_11_features.into_iter().collect(),
        );
        let _ = features.insert(
            "unstable-frameworks-macos-12".into(),
            macos_12_features.into_iter().collect(),
        );
        let _ = features.insert(
            "unstable-frameworks-macos-13".into(),
            macos_13_features.into_iter().collect(),
        );
        let _ = features.insert(
            "unstable-frameworks-gnustep".into(),
            gnustep_features.into_iter().collect(),
        );

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
