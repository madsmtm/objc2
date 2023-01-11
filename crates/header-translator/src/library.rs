use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

use crate::file::{File, FILE_PRELUDE};

#[derive(Debug, PartialEq, Default)]
pub struct Library {
    pub files: BTreeMap<String, File>,
}

impl Library {
    pub fn new() -> Self {
        Self {
            files: BTreeMap::new(),
        }
    }

    pub fn output(&self, path: &Path) -> io::Result<()> {
        for (name, file) in &self.files {
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

        for name in self.files.keys() {
            writeln!(f, "#[path = \"{name}.rs\"]")?;
            writeln!(f, "mod __{name};")?;
        }

        writeln!(f)?;

        for (name, file) in &self.files {
            for stmt in &file.stmts {
                let mut iter = stmt.declared_types();
                if let Some(item) = iter.next() {
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

                    writeln!(f, "pub use self::__{name}::{{{item}")?;
                    for item in iter {
                        writeln!(f, ", {item}")?;
                    }
                    writeln!(f, "}};")?;
                }
            }
        }

        Ok(())
    }
}
