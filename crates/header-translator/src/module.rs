use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::fmt::Write as _;
use std::path::Path;
use std::{fmt, fs};

use crate::cfgs::PlatformCfg;
use crate::display_helper::FormatterFn;
use crate::id::{cfg_gate_ln, Location};
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
    ) -> BTreeMap<String, BTreeSet<String>> {
        let mut required_features: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

        // Deliberately skipping own stmts

        for (file_name, module) in &self.submodules {
            let mut features = BTreeSet::new();
            for stmt in &module.stmts {
                for required_item in stmt.required_items_inner() {
                    let location = required_item.location();
                    if let Some(feature) = location.cargo_toml_feature(config, emission_library) {
                        features.insert(feature);
                    }
                }
            }
            required_features.insert(clean_name(file_name), features);
            required_features
                .extend(module.required_cargo_features_inner(config, emission_library));
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
                    item.import(config, emission_library).map(|import| {
                        (
                            import,
                            (
                                item.location().cfg_feature(config, emission_library),
                                item.library_name().to_string(),
                            ),
                        )
                    })
                })
                .collect();

            let emission_config = &config.library(emission_library);
            for (import, (cfg_feature, library_name)) in imports {
                if let Some(cfg_feature) = cfg_feature {
                    writeln!(f, "#[cfg(feature = {cfg_feature:?})]")?;
                }
                let mut platform_cfg = PlatformCfg::from_config(emission_config);
                platform_cfg.dependency(config.library(&library_name));
                if let Some(cfg) = platform_cfg.cfgs() {
                    writeln!(f, "#[cfg({cfg})]")?;
                }
                writeln!(f, "use {import};")?;
            }
            writeln!(f)?;

            if emission_config.is_library {
                writeln!(f, "use crate::ffi::*;")?;
            } else {
                writeln!(f, "use crate::*;")?;
            }

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

    fn contents<'a>(
        &'a self,
        config: &'a Config,
        emission_library: &'a str,
        prefix: impl fmt::Display + 'a,
    ) -> impl fmt::Display + 'a {
        FormatterFn(move |f| {
            write!(f, "{prefix}")?;

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
        test_path: &Path,
        config: &Config,
        emission_library: &str,
        top_level_prefix: impl fmt::Display,
        is_top_level: bool,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        if self.submodules.is_empty() && !is_top_level {
            // Only output a single file
            fs::write(
                path.with_extension("rs"),
                self.contents(config, emission_library, top_level_prefix)
                    .to_string(),
            )?;
            fs::write(
                test_path.with_extension("rs"),
                self.tests(config).to_string(),
            )?;
        } else {
            // Output an entire module
            fs::create_dir_all(path)?;
            fs::create_dir_all(test_path)?;

            // TODO: Fix this
            let mut expected_files: Vec<OsString> = vec![];

            for (name, module) in &self.submodules {
                let name = clean_name(name);
                let _span = debug_span!("writing file", name).entered();
                module.output(
                    &path.join(&name),
                    &test_path.join(&name),
                    config,
                    emission_library,
                    "//! This file has been automatically generated by `objc2`'s `header-translator`.\n//! DO NOT EDIT\n",
                    false,
                )?;
                if module.submodules.is_empty() {
                    expected_files.push(format!("{name}.rs").into());
                } else {
                    expected_files.push(name.into());
                }
            }

            fs::write(
                path.join("mod.rs"),
                self.contents(config, emission_library, top_level_prefix)
                    .to_string(),
            )?;
            fs::write(test_path.join("mod.rs"), self.tests(config).to_string())?;
            expected_files.push("mod.rs".into());

            // Remove previously generated files
            for file in path.read_dir()?.chain(test_path.read_dir()?) {
                let file = file?;
                if expected_files.contains(&file.file_name()) {
                    continue;
                }
                error!("removing previous file {:?}", file.path());
                fs::remove_file(file.path())?;
            }

            if is_top_level {
                let data = config.library(emission_library);
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

            if !encoding_tests.is_empty() {
                writeln!(f)?;

                writeln!(f, "use test_frameworks::*;")?;

                writeln!(f)?;

                writeln!(f, "#[test]")?;
                writeln!(f, "fn test_encoding() {{")?;

                for test in encoding_tests {
                    write!(f, "{test}")?;
                }

                writeln!(f, "}}")?;
            }

            Ok(())
        })
    }
}
