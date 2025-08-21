use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::fmt::Write as _;
use std::path::Path;
use std::{fmt, fs};

use crate::cfgs::PlatformCfg;
use crate::display_helper::FormatterFn;
use crate::id::{cfg_gate_ln, ItemTree};
use crate::stmt::Stmt;
use crate::{Config, Location};

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

    pub fn submodule_mut(&mut self, location: Location) -> &mut Module {
        let mut current = self;
        for component in location.modules() {
            current = match current.submodules.entry(component.into()) {
                Entry::Occupied(entry) => entry.into_mut(),
                Entry::Vacant(entry) => {
                    error!(?location, "expected module to be available in library");
                    entry.insert(Default::default())
                }
            };
        }
        current
    }

    // TODO: Merge the functions below using some sort of visitor pattern?

    pub fn used_crates<'c>(
        &self,
        config: &'c Config,
        emission_location: &Location,
    ) -> BTreeSet<&'c str> {
        self.stmts
            .iter()
            .flat_map(|stmt| stmt.required_items_inner())
            .flat_map(|item| item.used_crates(config, emission_location))
            .chain(self.submodules.iter().flat_map(|(name, module)| {
                module.used_crates(config, &emission_location.add_module(name))
            }))
            .collect()
    }

    pub fn all_features(&self, emission_location: &Location) -> impl Iterator<Item = String> {
        emission_location
            .feature_names()
            .last()
            .into_iter()
            .chain(self.submodules.iter().flat_map(|(name, submodule)| {
                submodule.all_features(&emission_location.add_module(name))
            }))
            .collect::<BTreeSet<_>>()
            .into_iter()
    }

    pub fn enabled_features(
        &self,
        config: &Config,
        emission_location: &Location,
    ) -> impl Iterator<Item = (String, String)> {
        self.stmts
            .iter()
            .flat_map(|stmt| stmt.required_items_inner())
            .flat_map(|item| item.enabled_features(config, emission_location))
            .chain(self.submodules.iter().flat_map(|(name, module)| {
                module.enabled_features(config, &emission_location.add_module(name))
            }))
            .collect::<BTreeSet<_>>()
            .into_iter()
    }

    pub fn required_crate_features<'config>(
        &self,
        config: &'config Config,
        emission_location: &Location,
    ) -> impl Iterator<Item = (&'config str, String)> {
        self.stmts
            .iter()
            .flat_map(|stmt| stmt.required_items_inner())
            .flat_map(|item| item.required_crate_features(config, emission_location))
            .chain(self.submodules.iter().flat_map(|(name, module)| {
                module.required_crate_features(config, &emission_location.add_module(name))
            }))
            .collect::<BTreeSet<_>>()
            .into_iter()
    }

    pub(crate) fn stmts<'a>(
        &'a self,
        config: &'a Config,
        emission_location: &'a Location,
    ) -> impl fmt::Display + 'a {
        FormatterFn(move |f| {
            let imports: BTreeMap<_, _> = self
                .stmts
                .iter()
                .flat_map(|stmt| stmt.required_items_inner())
                .flat_map(|item| item.imports(config, emission_location))
                .collect();

            for (import, cfg_gate_ln) in imports {
                write!(f, "{cfg_gate_ln}")?;
                writeln!(f, "use {import};")?;
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

    pub(crate) fn modules<'a>(
        &'a self,
        config: &'a Config,
        emission_location: &'a Location,
    ) -> impl fmt::Display + 'a {
        FormatterFn(move |f| {
            for (name, module) in &self.submodules {
                let location = emission_location.add_module(name);
                write!(
                    f,
                    "#[cfg(feature = {:?})]",
                    location.feature_names().last().unwrap()
                )?;
                let name = clean_name(name);
                if module.submodules.is_empty() {
                    writeln!(f, "#[path = \"{name}.rs\"]")?;
                } else {
                    writeln!(f, "#[path = \"{name}/mod.rs\"]")?;
                }
                writeln!(f, "mod __{name};")?;
            }

            writeln!(f)?;

            let module_names: Vec<_> = self.submodules.keys().map(|name| &**name).collect();

            for (name, module) in &self.submodules {
                let location = emission_location.add_module(name);
                if !module.submodules.is_empty() {
                    write!(
                        f,
                        "#[cfg(feature = {:?})]",
                        location.feature_names().last().unwrap()
                    )?;
                    writeln!(f, "pub use self::__{}::*;", clean_name(name))?;
                    continue;
                }

                for stmt in &module.stmts {
                    if let Some(item) = stmt.provided_item() {
                        let visibility = if item.name.starts_with('_') {
                            if let Some(name) = item.name.strip_prefix("__") {
                                if module_names.contains(&name) {
                                    // Item name conflicts with module name, don't
                                    // expose at all.
                                    continue;
                                }
                            }
                            // Try to expose, but only to the rest of the crate.
                            "pub(crate)"
                        } else {
                            "pub"
                        };

                        item.location().assert_file(name);

                        write!(
                            f,
                            "{}",
                            cfg_gate_ln(
                                [ItemTree::new(item.clone(), stmt.required_items())],
                                &[] as &[ItemTree],
                                config,
                                emission_location,
                            )
                        )?;

                        writeln!(
                            f,
                            "{visibility} use self::__{}::{};",
                            clean_name(name),
                            item.name,
                        )?;
                    }
                }
            }

            Ok(())
        })
    }

    fn contents<'a>(
        &'a self,
        config: &'a Config,
        emission_location: &'a Location,
        prefix: impl fmt::Display + 'a,
    ) -> impl fmt::Display + 'a {
        FormatterFn(move |f| {
            write!(f, "{prefix}")?;

            if !self.submodules.is_empty() {
                write!(f, "{}", self.modules(config, emission_location))?;
            }

            if !self.stmts.is_empty() {
                write!(f, "{}", self.stmts(config, emission_location))?;
            }

            Ok(())
        })
    }

    pub fn output(
        &self,
        path: &Path,
        test_path: &Path,
        config: &Config,
        emission_location: &Location,
        top_level_prefix: impl fmt::Display,
        emit_tests: bool,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        if self.submodules.is_empty() && !emission_location.is_top_level() {
            // Only output a single file
            fs::write(
                path.with_extension("rs"),
                self.contents(config, emission_location, top_level_prefix)
                    .to_string(),
            )?;
            if emit_tests {
                fs::write(
                    test_path.with_extension("rs"),
                    self.tests(config).to_string(),
                )?;
            }
        } else {
            // Output an entire module
            fs::create_dir_all(path)?;
            if emit_tests {
                fs::create_dir_all(test_path)?;
            }

            // TODO: Fix this
            let mut expected_files: Vec<OsString> = vec![];

            for (name, module) in &self.submodules {
                let name = clean_name(name);
                let _span = debug_span!("writing file", name).entered();
                module.output(
                    &path.join(&name),
                    &test_path.join(&name),
                    config,
                    &emission_location.add_module(&name),
                    "//! This file has been automatically generated by `objc2`'s `header-translator`.\n//! DO NOT EDIT\n",
                    emit_tests,
                )?;
                if module.submodules.is_empty() {
                    expected_files.push(format!("{name}.rs").into());
                } else {
                    expected_files.push(name.into());
                }
            }

            fs::write(
                path.join("mod.rs"),
                self.contents(config, emission_location, top_level_prefix)
                    .to_string(),
            )?;
            if emit_tests {
                fs::write(test_path.join("mod.rs"), self.tests(config).to_string())?;
            }
            expected_files.push("mod.rs".into());

            // Remove previously generated files
            if let Ok(test_dir) = test_path.read_dir() {
                for file in path.read_dir()?.chain(test_dir) {
                    let file = file?;
                    if expected_files.contains(&file.file_name()) {
                        continue;
                    }
                    error!("removing previous file {:?}", file.path());
                    if file.path().is_dir() {
                        fs::remove_dir_all(file.path())?;
                    } else {
                        fs::remove_file(file.path())?;
                    }
                }
            }

            if emission_location.is_top_level() && emit_tests {
                let data = config.library(emission_location);
                let mut s = String::new();
                writeln!(&mut s, "#![cfg(feature = \"test-frameworks\")]")?;
                let platform_cfg = PlatformCfg::from_config_explicit(data);
                if let Some(cfgs) = platform_cfg.cfgs() {
                    writeln!(&mut s, "#![cfg({cfgs})]")?;
                }
                writeln!(
                    &mut s,
                    "#![allow(unused_imports, dead_code, unused_variables, deprecated, non_snake_case)]"
                )?;
                writeln!(&mut s)?;
                writeln!(&mut s, "#[path = \"{}/mod.rs\"]", data.framework)?;
                writeln!(&mut s, "mod {};", data.framework)?;
                fs::write(test_path.with_extension("rs"), s)?;
            }
        }

        Ok(())
    }

    fn tests<'a>(&'a self, config: &'a Config) -> impl fmt::Display + 'a {
        FormatterFn(move |f| {
            writeln!(
                f,
                "// This test file has been automatically generated by `objc2`'s `header-translator`."
            )?;
            writeln!(f)?;

            for (name, module) in &self.submodules {
                let name = clean_name(name);
                if module.submodules.is_empty() {
                    writeln!(f, "#[path = \"{name}.rs\"]")?;
                } else {
                    writeln!(f, "#[path = \"{name}/mod.rs\"]")?;
                }
                writeln!(f, "mod __{name};")?;
            }

            let encoding_tests: Vec<_> = self
                .stmts
                .iter()
                .filter_map(|stmt| stmt.encoding_test(config))
                .collect();

            let static_tests: Vec<_> = self
                .stmts
                .iter()
                .filter_map(|stmt| stmt.static_test(config))
                .collect();

            if !encoding_tests.is_empty() || !static_tests.is_empty() {
                writeln!(f)?;

                writeln!(f, "use test_frameworks::*;")?;
            }

            if !encoding_tests.is_empty() {
                writeln!(f)?;

                writeln!(f, "#[test]")?;
                writeln!(f, "fn test_encoding() {{")?;

                for test in encoding_tests {
                    write!(f, "{test}")?;
                }

                writeln!(f, "}}")?;
            }

            if !static_tests.is_empty() {
                writeln!(f)?;

                writeln!(f, "#[test]")?;
                writeln!(f, "fn test_statics() {{")?;

                for test in static_tests {
                    write!(f, "{test}")?;
                }

                writeln!(f, "}}")?;
            }

            Ok(())
        })
    }
}
