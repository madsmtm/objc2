use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

use crate::availability::Unavailable;
use crate::file::{File, FILE_PRELUDE};

#[derive(Debug, PartialEq, Default)]
pub struct Library {
    pub files: BTreeMap<String, File>,
    pub unavailability: Unavailable,
}

impl Library {
    pub(crate) fn new(unavailability: Unavailable) -> Self {
        Self {
            files: BTreeMap::new(),
            unavailability,
        }
    }

    pub fn output(&self, path: &Path) -> io::Result<()> {
        for (name, file) in &self.files {
            // NOTE: some SDK files have '+' in the file name
            let name = name.replace('+', "_");
            let mut path = path.join(name);
            path.set_extension("rs");
            fs::write(&path, file.to_string())?;
        }

        // truncate if the file exists
        fs::write(path.join("mod.rs"), self.to_string())?;

        Ok(())
    }

    pub fn compare(&self, other: &Self) {
        super::compare_btree(&self.files, &other.files, |name, self_file, other_file| {
            let _span = debug_span!("file", name).entered();
            self_file.compare(other_file);
        });
    }
}

impl fmt::Display for Library {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{FILE_PRELUDE}")?;
        writeln!(f, "#![allow(unused_imports)]")?;
        writeln!(f, "#![allow(deprecated)]")?;

        for name in self.files.keys() {
            // NOTE: some SDK files have '+' in the file name
            let name = name.replace('+', "_");
            writeln!(f, "#[path = \"{name}.rs\"]")?;
            writeln!(f, "mod __{name};")?;
        }

        writeln!(f)?;

        for (name, file) in &self.files {
            // NOTE: some SDK files have '+' in the file name
            let name = name.replace('+', "_");
            for stmt in &file.stmts {
                // Use a set to deduplicate features, and to have them in
                // a consistent order
                let mut features = BTreeSet::new();
                stmt.visit_required_types(|item| {
                    if let Some(feature) = item.feature() {
                        features.insert(format!("feature = \"{feature}\""));
                    }
                });
                match features.len() {
                    0 => {}
                    1 => {
                        writeln!(f, "#[cfg({})]", features.first().unwrap())?;
                    }
                    _ => {
                        writeln!(
                            f,
                            "#[cfg(all({}))]",
                            features
                                .iter()
                                .map(|s| &**s)
                                .collect::<Vec<&str>>()
                                .join(",")
                        )?;
                    }
                }
                let iter = stmt.declared_types();

                for (item, unavailability) in iter {
                    match features.len() {
                        0 => {}
                        1 => {
                            writeln!(f, "#[cfg({})]", features.first().unwrap())?;
                        }
                        _ => {
                            writeln!(
                                f,
                                "#[cfg(all({}))]",
                                features
                                    .iter()
                                    .map(|s| &**s)
                                    .collect::<Vec<&str>>()
                                    .join(",")
                            )?;
                        }
                    }
                    writeln!(f, "{unavailability}")?;
                    writeln!(f, "pub use self::__{name}::{{{item}}};")?;
                }
            }
        }

        Ok(())
    }
}
