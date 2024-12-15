use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::path::Path;
use std::{fmt, fs};

use crate::cfgs::PlatformCfg;
use crate::display_helper::FormatterFn;
use crate::id::{cfg_gate_ln, Location};
use crate::library::Dependencies;
use crate::stmt::Stmt;
use crate::{Config, ItemIdentifier};

#[derive(Default, Debug, PartialEq)]
pub struct Module {
    pub(crate) submodules: BTreeMap<String, Module>,
    pub(crate) stmts: Vec<Stmt>,
}

/// Some SDK files have '+' in the file name, so we change those to `_`.
pub(crate) fn clean_name(name: &str) -> String {
    name.replace('+', "_")
}

impl Module {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_stmt(&mut self, stmt: Stmt) {
        self.stmts.push(stmt);
    }

    pub fn all_items(&self) -> BTreeSet<ItemIdentifier> {
        self.stmts
            .iter()
            .flat_map(|stmt| stmt.required_items_inner())
            .chain(
                self.submodules
                    .values()
                    .flat_map(|module| module.all_items()),
            )
            .collect()
    }

    pub fn required_cargo_features_inner(
        &self,
        config: &Config,
        emission_library: &str,
        dependencies: &Dependencies<'_>,
    ) -> BTreeMap<String, BTreeSet<String>> {
        let mut required_features: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

        // Deliberately skipping own stmts

        for (file_name, module) in &self.submodules {
            let mut features = BTreeSet::new();
            for stmt in &module.stmts {
                for required_item in stmt.required_items_inner() {
                    let location = required_item.location();
                    if let Some(feature) = location
                        .library(config, emission_library)
                        .cargo_toml_feature()
                    {
                        if feature == "bitflags" {
                            if let Some((true, _, _)) = dependencies.get("bitflags") {
                                continue;
                            }
                        }
                        // Feature names are based on the file name, not the
                        // whole path to the feature.
                        features.insert(feature);
                    }
                }
            }
            required_features.insert(clean_name(file_name), features);
            required_features.extend(module.required_cargo_features_inner(
                config,
                emission_library,
                dependencies,
            ));
        }

        required_features
    }

    pub(crate) fn stmts<'a>(
        &'a self,
        config: &'a Config,
        emission_library: &'a str,
    ) -> impl fmt::Display + 'a {
        FormatterFn(move |f| {
            let imports: BTreeMap<_, _> = self
                .stmts
                .iter()
                .flat_map(|stmt| stmt.required_items_inner())
                .filter_map(|item| {
                    item.location()
                        .library(config, emission_library)
                        .import()
                        .map(|import_data| (item.library_name().to_string(), import_data))
                })
                .collect();

            let emission_config = &config.library(emission_library);
            for (library_name, (krate, required)) in imports {
                if !required {
                    writeln!(f, "#[cfg(feature = {:?})]", krate)?;
                }
                let mut platform_cfg = PlatformCfg::from_config(emission_config);
                platform_cfg.dependency(config.library(&library_name));
                if let Some(cfg) = platform_cfg.cfgs() {
                    writeln!(f, "#[cfg({cfg})]")?;
                }
                writeln!(f, "use {}::*;", krate.replace('-', "_"))?;
            }
            writeln!(f)?;
            writeln!(f, "use crate::*;")?;

            writeln!(f)?;

            for stmt in &self.stmts {
                writeln!(f, "{}", stmt.fmt(config))?;
            }

            Ok(())
        })
    }

    pub(crate) fn modules<'a>(&'a self, config: &'a Config) -> impl fmt::Display + 'a {
        FormatterFn(move |f| {
            for (name, module) in &self.submodules {
                let name = clean_name(name);
                write!(f, "#[cfg(feature = \"{name}\")]")?;
                if module.submodules.is_empty() {
                    writeln!(f, "#[path = \"{name}.rs\"]")?;
                } else {
                    writeln!(f, "#[path = \"{name}/mod.rs\"]")?;
                }
                writeln!(f, "mod __{name};")?;
            }

            writeln!(f)?;

            for (module_name, module) in &self.submodules {
                if !module.submodules.is_empty() {
                    write!(f, "#[cfg(feature = \"{module_name}\")]")?;
                    write!(f, "pub use self::__{}::*;", clean_name(module_name))?;
                    continue;
                }

                for stmt in &module.stmts {
                    if let Some(item) = stmt.provided_item() {
                        item.location().assert_file(module_name);

                        let mut items = stmt.required_items();
                        items.push(item.clone());
                        write!(
                            f,
                            "{}",
                            cfg_gate_ln(items, &[] as &[Location], config, item.location())
                        )?;

                        let visibility = if item.name.starts_with('_') {
                            "pub(crate)"
                        } else {
                            "pub"
                        };
                        write!(
                            f,
                            "{visibility} use self::__{}::{{{}}};",
                            clean_name(module_name),
                            item.name,
                        )?;
                    }
                }
            }

            Ok(())
        })
    }

    pub(crate) fn contents<'a>(
        &'a self,
        config: &'a Config,
        emission_library: &'a str,
    ) -> impl fmt::Display + 'a {
        FormatterFn(move |f| {
            writeln!(
                f,
                "//! This file has been automatically generated by `objc2`'s `header-translator`."
            )?;
            writeln!(f, "//! DO NOT EDIT")?;

            if !self.submodules.is_empty() {
                write!(f, "{}", self.modules(config))?;
            }

            if !self.stmts.is_empty() {
                write!(f, "{}", self.stmts(config, emission_library))?;
            }

            Ok(())
        })
    }

    pub fn output(
        &self,
        path: &Path,
        config: &Config,
        emission_library: &str,
        top_level_contents: impl fmt::Display,
        is_top_level: bool,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        if self.submodules.is_empty() && !is_top_level {
            // Only output a single file
            fs::write(path.with_extension("rs"), top_level_contents.to_string())?;
        } else {
            // Output an entire module
            fs::create_dir_all(path)?;

            // TODO: Fix this
            let mut expected_files: Vec<OsString> = vec![];

            for (name, module) in &self.submodules {
                let name = clean_name(name);
                let _span = debug_span!("writing file", name).entered();
                module.output(
                    &path.join(&name),
                    config,
                    emission_library,
                    module.contents(config, emission_library),
                    false,
                )?;
                if module.submodules.is_empty() {
                    expected_files.push(format!("{name}.rs").into());
                } else {
                    expected_files.push(name.into());
                }
            }

            fs::write(path.join("mod.rs"), top_level_contents.to_string())?;
            expected_files.push("mod.rs".into());

            // Remove previously generated files
            for file in path.read_dir()? {
                let file = file?;
                if expected_files.contains(&file.file_name()) {
                    continue;
                }
                error!("removing previous file {:?}", file.path());
                fs::remove_file(file.path())?;
            }
        }

        Ok(())
    }
}
