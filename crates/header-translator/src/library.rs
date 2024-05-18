use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::io::ErrorKind;
use std::io::Write;
use std::path::Path;

use toml_edit::InlineTable;
use toml_edit::{value, Array, DocumentMut, Formatted, Item, Table, Value};

use crate::config::LibraryConfig;
use crate::display_helper::FormatterFn;
use crate::module::Module;
use crate::Config;
use crate::Location;
use crate::VERSION;

#[derive(Debug, PartialEq)]
pub struct Library {
    pub module: Module,
    link_name: String,
    pub data: LibraryConfig,
}

impl Library {
    pub fn new(name: &str, data: &LibraryConfig) -> Self {
        Self {
            module: Module::new(),
            link_name: name.to_string(),
            data: data.clone(),
        }
    }

    pub fn add_module(&mut self, path: Vec<String>) {
        let mut current = &mut self.module;
        for p in path {
            current = current.submodules.entry(p.clone()).or_default();
        }
    }

    pub fn module_mut(&mut self, location: &Location) -> &mut Module {
        let mut current = &mut self.module;
        for p in location.modules() {
            current = match current.submodules.entry(p.to_string()) {
                Entry::Occupied(entry) => entry.into_mut(),
                Entry::Vacant(entry) => {
                    error!(?location, "expected module to be available in library");
                    entry.insert(Default::default())
                }
            };
        }
        current
    }

    pub fn output(
        &self,
        crate_dir: &Path,
        config: &Config,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let generated_dir = crate_dir.join("src").join("generated");

        // Output `src/generated/*`.
        self.module
            .output(&generated_dir, config, &self.link_name)?;

        // Overwrite `src/generated/mod.rs` with more data.
        fs::write(
            generated_dir.join("mod.rs"),
            self.contents(config).to_string(),
        )?;

        if !self.data.custom_lib_rs {
            // Output `src/lib.rs`. Truncates if the file exists.
            let mut lib_rs = fs::File::create(crate_dir.join("src").join("lib.rs"))?;
            writeln!(
                lib_rs,
                "//! # Bindings to the `{}` framework",
                self.link_name
            )?;
            writeln!(lib_rs, "//!")?;
            writeln!(
                lib_rs,
                "//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.",
            )?;
            writeln!(lib_rs, "//!")?;
            writeln!(
                lib_rs,
                "//! [apple-doc]: https://developer.apple.com/documentation/{}/",
                self.link_name.to_lowercase(),
            )?;
            writeln!(lib_rs, "//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html")?;
            writeln!(lib_rs, "#![no_std]")?;
            writeln!(
                lib_rs,
                "#![cfg_attr(feature = \"unstable-docsrs\", feature(doc_auto_cfg))]"
            )?;
            writeln!(lib_rs, "// Update in Cargo.toml as well.")?;
            writeln!(
                lib_rs,
                "#![doc(html_root_url = \"https://docs.rs/{}/{VERSION}\")]",
                self.data.krate,
            )?;
            writeln!(lib_rs)?;
            writeln!(lib_rs, "#[cfg(feature = \"alloc\")]")?;
            writeln!(lib_rs, "extern crate alloc;")?;
            writeln!(lib_rs)?;
            writeln!(lib_rs, "#[cfg(feature = \"std\")]")?;
            writeln!(lib_rs, "extern crate std;")?;
            writeln!(lib_rs)?;
            writeln!(lib_rs, "mod generated;")?;
            writeln!(lib_rs, "#[allow(unused_imports, unreachable_pub)]")?;
            writeln!(lib_rs, "pub use self::generated::*;")?;
            lib_rs.flush()?;
        }

        // Output `README.md`.
        let mut readme = fs::File::create(crate_dir.join("README.md"))?;
        writeln!(readme, "# `{0}`

[![Latest version](https://badgen.net/crates/v/{0})](https://crates.io/crates/{0})
[![License](https://badgen.net/badge/license/MIT/blue)](../LICENSE.txt)
[![Documentation](https://docs.rs/{0}/badge.svg)](https://docs.rs/{0}/)
[![CI](https://github.com/madsmtm/objc2/actions/workflows/ci.yml/badge.svg)](https://github.com/madsmtm/objc2/actions/workflows/ci.yml)

Rust bindings to Apple's framework {1}.

This README is kept intentionally small to consolidate the documentation, see
[the Rust docs](https://docs.rs/{0}/) for more details on this crate.

This crate is part of the [`objc2` project](https://github.com/madsmtm/objc2),
see that for related crates.", self.data.krate, self.link_name)?;
        readme.flush()?;

        let mut cargo_toml: DocumentMut = include_str!("default_cargo.toml")
            .parse()
            .expect("invalid default toml");

        cargo_toml["package"]["name"] = value(&self.data.krate);
        match cargo_toml["package"]["version"].as_value_mut().unwrap() {
            // Preserve comment behind `version`
            Value::String(s) => {
                let decor = s.decor().clone();
                *s = Formatted::new(VERSION.to_string());
                *s.decor_mut() = decor;
            }
            _ => unreachable!(),
        }
        cargo_toml["package"]["description"] =
            value(format!("Bindings to the {} framework", self.link_name));
        let keywords = cargo_toml["package"]["keywords"].as_array_mut().unwrap();
        if self.data.macos.is_some() {
            keywords.push("macos");
        }
        if self.data.ios.is_some() {
            keywords.push("ios");
        }

        let targets = cargo_toml["package"]["metadata"]["docs"]["rs"]["targets"]
            .as_array_mut()
            .unwrap();

        let mut default_target = None;
        if self.data.macos.is_some() {
            // Default to `aarch64-apple-darwin`, as that is likely the more
            // popular target nowadays.
            default_target.get_or_insert("aarch64-apple-darwin");
            targets.push("aarch64-apple-darwin");
            targets.push("x86_64-apple-darwin");
        }
        if self.data.ios.is_some() {
            default_target.get_or_insert("aarch64-apple-ios");
            targets.push("aarch64-apple-ios");
            if self.data.macos.is_none() {
                targets.push("x86_64-apple-ios");
            }
        }
        if self.data.tvos.is_some() {
            default_target.get_or_insert("aarch64-apple-tvos");
            targets.push("aarch64-apple-tvos");
        }
        if self.data.watchos.is_some() {
            default_target.get_or_insert("aarch64-apple-watchos");
            targets.push("aarch64-apple-watchos");
        }
        if self.data.maccatalyst.is_some() {
            default_target.get_or_insert("aarch64-apple-ios-macabi");
            targets.push("aarch64-apple-ios-macabi");
        }
        if self.data.gnustep {
            default_target.get_or_insert("x86_64-unknown-linux-gnu");
            targets.push("x86_64-unknown-linux-gnu");
            targets.push("i686-unknown-linux-gnu");
        }
        for item in targets.iter_mut() {
            item.decor_mut().set_prefix("\n    ");
        }
        targets.set_trailing_comma(true);
        cargo_toml["package"]["metadata"]["docs"]["rs"]["default-target"] =
            value(default_target.unwrap());

        let dependencies: BTreeMap<_, _> = self
            .module
            .crates(config, &self.link_name)
            .into_iter()
            .chain(self.data.required_dependencies.iter().map(|krate| &**krate))
            .map(|krate| (krate, self.data.required_dependencies.contains(krate)))
            .collect();

        for (krate, required) in &dependencies {
            let mut table = match *krate {
                "block2" => InlineTable::from_iter([
                    ("path", Value::from("../../crates/block2".to_string())),
                    ("version", Value::from("0.5.0")),
                ]),
                // Use a reasonably new version of libc
                "libc" => InlineTable::from_iter([("version", Value::from("0.2.80"))]),
                // Use a version of bitflags that supports `impl`
                "bitflags" => InlineTable::from_iter([("version", Value::from("2.5.0"))]),
                krate => InlineTable::from_iter([
                    ("path", Value::from(format!("../{krate}"))),
                    ("version", Value::from(VERSION)),
                ]),
            };
            if self.data.gnustep && *krate != "libc" && *krate != "bitflags" {
                table.insert("default-features", Value::from(false));
            }
            if !required {
                table.insert("optional", Value::from(true));
            }

            // Don't override if set by Cargo.modified.toml
            cargo_toml["dependencies"]
                .as_table_mut()
                .unwrap()
                .entry(krate)
                .or_insert(Item::Value(Value::InlineTable(table)));
        }

        match fs::read_to_string(crate_dir.join("Cargo.modified.toml")) {
            Ok(s) => {
                let modified: DocumentMut = s.parse()?;
                merge_toml_table(&mut cargo_toml, modified.as_table().clone());
            }
            Err(e) if e.kind() == ErrorKind::NotFound => {
                // Skip
            }
            Err(e) => Err(e)?,
        }

        for (krate, required) in &dependencies {
            if !required {
                let array: Array = [format!("dep:{krate}")].iter().collect();
                cargo_toml["features"][krate] = value(array);
            }
        }

        add_newline_at_end(&mut cargo_toml["features"]);

        // Own features
        let mut generated_features = self.module.required_cargo_features(config, &self.link_name);

        let _ = generated_features.insert(
            "all".to_string(),
            generated_features
                .keys()
                .cloned()
                .chain(
                    dependencies
                        .iter()
                        .filter(|(_, required)| !*required)
                        .map(|(krate, _)| krate.to_string()),
                )
                .collect::<BTreeSet<_>>(),
        );

        for (feature, required_features) in generated_features {
            let mut array: Array = required_features.into_iter().collect();
            if 1 < array.len() {
                for item in array.iter_mut() {
                    item.decor_mut().set_prefix("\n    ");
                }
                array.set_trailing("\n");
                array.set_trailing_comma(true);
            }
            cargo_toml["features"][feature] = value(array);
        }

        fs::write(crate_dir.join("Cargo.toml"), cargo_toml.to_string())?;

        Ok(())
    }

    pub fn contents<'a>(&'a self, config: &'a Config) -> impl fmt::Display + 'a {
        FormatterFn(|f| {
            writeln!(
                f,
                "// This file has been automatically generated by `objc2`'s `header-translator`."
            )?;
            writeln!(f, "// DO NOT EDIT")?;
            writeln!(f)?;

            // Lints
            // We emit `use [framework]::*` more than necessary often.
            writeln!(f, "#![allow(unused_imports)]")?;
            // Deprecated items are often still used in other signatures.
            writeln!(f, "#![allow(deprecated)]")?;
            // Methods use a different naming scheme.
            writeln!(f, "#![allow(non_snake_case)]")?;
            // We emit C types with a different naming scheme.
            writeln!(f, "#![allow(non_camel_case_types)]")?;
            // Statics and enum fields use a different naming scheme.
            writeln!(f, "#![allow(non_upper_case_globals)]")?;
            // We don't yet emit documentation for methods.
            writeln!(f, "#![allow(missing_docs)]")?;

            // Clippy lints
            // We have no control over how many arguments a method takes.
            writeln!(f, "#![allow(clippy::too_many_arguments)]")?;
            // We have no control over how complex a type is.
            writeln!(f, "#![allow(clippy::type_complexity)]")?;
            // Apple's naming scheme allows this.
            writeln!(f, "#![allow(clippy::upper_case_acronyms)]")?;
            // Headers often use `x << 0` for clarity.
            writeln!(f, "#![allow(clippy::identity_op)]")?;
            // We don't have the manpower to document the safety of methods.
            writeln!(f, "#![allow(clippy::missing_safety_doc)]")?;

            writeln!(f)?;

            // Link to the correct framework.
            if self.data.gnustep {
                // Allow for different linking on GNUStep
                writeln!(
                    f,
                    "#[cfg_attr(feature = \"apple\", link(name = \"{}\", kind = \"framework\"))]",
                    self.link_name
                )?;
            } else {
                writeln!(
                    f,
                    "#[link(name = \"{}\", kind = \"framework\")]",
                    self.link_name
                )?;
            }
            writeln!(f, "extern \"C\" {{}}")?;
            writeln!(f)?;

            if !self.module.submodules.is_empty() {
                write!(f, "{}", self.module.modules(config))?;
            }

            if !self.module.stmts.is_empty() {
                write!(f, "{}", self.module.stmts(config, &self.link_name))?;
            }

            Ok(())
        })
    }
}

fn merge_toml_table(original: &mut Table, addition: Table) {
    for (key, mut addition) in addition {
        match original.entry(&key) {
            toml_edit::Entry::Occupied(mut original) => match (original.get_mut(), addition) {
                (Item::Value(original), Item::Value(addition)) => {
                    *original = addition;
                }
                (Item::Table(original), Item::Table(addition)) => {
                    merge_toml_table(original, addition);
                }
                (Item::ArrayOfTables(original), Item::ArrayOfTables(addition)) => {
                    *original = addition;
                }
                (original, addition) => panic!("cannot merge items {original:?} and {addition:?}"),
            },
            toml_edit::Entry::Vacant(original) => {
                match &mut addition {
                    Item::Table(table) => {
                        table.set_position(usize::MAX);
                        table.decor_mut().clear();
                    }
                    Item::ArrayOfTables(array) => {
                        for table in array.iter_mut() {
                            table.set_position(usize::MAX);
                            table.decor_mut().clear();
                        }
                    }
                    _ => {}
                }
                original.insert(addition);
            }
        }
    }
}

fn add_newline_at_end(item: &mut Item) {
    // Place an extra newline at the end of features table, before the
    // auto-generated features below are added.
    item.as_table_mut()
        .unwrap()
        .iter_mut()
        .last()
        .unwrap()
        .1
        .as_value_mut()
        .unwrap()
        .decor_mut()
        .set_suffix("\n");
}
