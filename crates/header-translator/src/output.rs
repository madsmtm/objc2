use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::{self, Write};
use std::fs;
use std::path::Path;

use crate::config::{Config, LibraryData};
use crate::feature::Feature;
use crate::library::Library;
use crate::stmt::Stmt;

use semver::VersionReq;

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

        let mut macos_10_12_features: BTreeSet<String> =
            vec!["unstable-example-basic_usage".into()]
                .into_iter()
                .collect();
        let mut macos_10_13_features: BTreeSet<String> = vec![
            "unstable-frameworks-macos-10-12".into(),
            "unstable-example-delegate".into(),
            "unstable-example-metal".into(),
            "unstable-example-nspasteboard".into(),
            "unstable-example-speech_synthesis".into(),
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
        let mut macos_14_features: BTreeSet<String> = vec!["unstable-frameworks-macos-13".into()]
            .into_iter()
            .collect();

        for (library_name, library) in &config.libraries {
            let library_features = library
                .imports
                .iter()
                .chain(library.extra_features.iter())
                .cloned();
            let _ = features.insert(library_name.to_string(), library_features.collect());

            if let Some(version) = &library.macos {
                if VersionReq::parse("<=10.12").unwrap().matches(version) {
                    macos_10_12_features.insert(format!("{library_name}_all"));
                } else if VersionReq::parse("<=10.13").unwrap().matches(version) {
                    macos_10_13_features.insert(format!("{library_name}_all"));
                } else if VersionReq::parse("<=11.0").unwrap().matches(version) {
                    macos_11_features.insert(format!("{library_name}_all"));
                } else if VersionReq::parse("<=12.0").unwrap().matches(version) {
                    macos_12_features.insert(format!("{library_name}_all"));
                } else if VersionReq::parse("<=13.0").unwrap().matches(version) {
                    macos_13_features.insert(format!("{library_name}_all"));
                } else if VersionReq::parse("<=14.0").unwrap().matches(version) {
                    macos_14_features.insert(format!("{library_name}_all"));
                } else {
                    error!(?library_name, "has library that does not fit any version");
                }
            }
        }

        let _ = features.insert(
            "unstable-frameworks-macos-10-12".into(),
            macos_10_12_features.into_iter().collect(),
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
            "unstable-frameworks-macos-14".into(),
            macos_14_features.into_iter().collect(),
        );

        for (library_name, library) in &self.libraries {
            let mut library_features = BTreeSet::from([library_name.clone()]);

            for file in library.files.values() {
                for stmt in &file.stmts {
                    #[allow(clippy::single_match)] // There will be others
                    match stmt {
                        Stmt::ClassDecl {
                            id, superclasses, ..
                        } => {
                            if let Some(feature) = Feature::new(id).name() {
                                // Only require the first superclass as feature,
                                // since the rest will be enabled transitively.
                                if let Some((superclass, _)) = superclasses.first() {
                                    let superclass_features: Vec<_> =
                                        Feature::new(superclass).name().into_iter().collect();
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
