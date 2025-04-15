use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::io::ErrorKind;
use std::io::Write;
use std::iter;
use std::path::Path;

use toml_edit::InlineTable;
use toml_edit::{value, Array, DocumentMut, Item, Table, Value};

use crate::cfgs::PlatformCfg;
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

    pub fn add_module(&mut self, location: Location) {
        let mut current = &mut self.module;
        for component in location.modules() {
            current = current.submodules.entry(component.into()).or_default();
        }
    }

    pub fn module_mut(&mut self, location: Location) -> &mut Module {
        let mut current = &mut self.module;
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

    fn dependencies<'c>(&'c self, config: &'c Config) -> BTreeMap<&'c str, BTreeSet<String>> {
        let emission_location = Location::from_library(&self.link_name);
        let mut dependencies: BTreeMap<&'c str, _> = self
            .module
            .used_crates(config, &emission_location)
            .into_iter()
            .map(|krate| (krate, BTreeSet::new()))
            .collect();

        // Compute required features.
        for (krate, feature) in self
            .module
            .required_crate_features(config, &emission_location)
        {
            if let Some(features) = dependencies.get_mut(krate) {
                features.insert(feature.to_string());
            } else {
                error!(?krate, ?feature, "tried to set krate dependency feature");
            }
        }

        // HACK: Encode impls need the inner encode impl to be available.
        if self.data.required_crates.contains("objc2") {
            for (krate, features) in &mut dependencies {
                let data = config.library_from_crate(krate);
                if !data.required_crates.contains("objc2") && !data.skipped {
                    features.insert("objc2".into());
                }
            }
        }

        for (krate, features) in &mut dependencies {
            // std is currently required for objc2
            if *krate == "objc2" {
                features.insert("std".into());
            }
            // alloc is currently required for these crates
            if matches!(*krate, "block2" | "dispatch2" | "objc2-foundation") {
                features.insert("alloc".into());
            }
            // We can't do `std = ["bitflags?/std"]`, as that'd make
            // `bitflags` show up in user's Cargo.toml / make it downloaded,
            // even if they didn't use it.
            //
            // So for now, we just always enable the `std` feature.
            if *krate == "bitflags" {
                features.insert("std".into());
            }
        }

        dependencies
    }

    /// All features are enabled by default, except for frameworks that
    /// would bump the minimum version of the library.
    ///
    /// The reasoning is that default features are meant for ease of use,
    /// especially so in end-user binaries and hobby projects. But we also
    /// don't want to make the user's binary incompatible with older OSes
    /// if they didn't explicitly opt-in to that.
    ///
    /// End result: The "only" cost is compilation time (vs. wasted
    /// developer time in finding each feature gate, or an unintentionally
    /// raised minimum OS version).
    ///
    /// And yes, libraries that use these crates _will_ want to disable
    /// default features, but that's the name of the game.
    ///
    /// We _could_ technically try to do something fancy to avoid e.g.
    /// `objc2-app-kit` pulling in `objc2-core-data`, since that is rarely
    /// needed, but where do we draw the line? And besides, that just masks
    /// the problem, library developers _should_ also disable the file
    /// features that they don't use if they really care about compilation
    /// time.
    ///
    /// See also <https://github.com/madsmtm/objc2/issues/627>.
    pub fn is_default_feature(&self, feature: &str, config: &Config) -> bool {
        if let Some(lib) = config.try_library_from_crate(feature) {
            // Dependency feature
            self.data.can_safely_depend_on(lib) || !lib.link
        } else {
            // File feature
            true
        }
    }

    pub fn emitted_features(&self, config: &Config) -> BTreeMap<String, BTreeSet<String>> {
        let emission_location = Location::from_library(&self.link_name);
        let mut emitted_features = BTreeMap::new();

        // Add crates
        let mut objc2_features = BTreeSet::new();
        for krate in self.module.used_crates(config, &emission_location) {
            let required = self.data.required_crates.contains(krate);

            let data = config.library_from_crate(krate);
            if !data.required_crates.contains("objc2") && krate != "objc2" && !data.skipped {
                // Uses optional feature enablement. Sub-optimal since it's
                // buggy in Cargo, but it's what we have to work with.
                //
                // NOTE: This assumes that the dependency even _has_ an
                // `objc2` feature, which is not necessarily the case.
                objc2_features.insert(format!("{krate}{}/objc2", if required { "" } else { "?" }));
            }

            if required {
                continue;
            }

            emitted_features.insert(krate.to_string(), BTreeSet::from([format!("dep:{krate}")]));
        }

        // HACK: Encode impls need the inner encode impl to be available.
        if !self.data.required_crates.contains("objc2") {
            if let Some(objc2) = emitted_features.get_mut("objc2") {
                objc2.extend(objc2_features);
            }
        }

        // Add features from modules.
        for feature in self.module.all_features(&emission_location) {
            if emitted_features.contains_key(&feature) {
                // Already added above.
                continue;
            }
            emitted_features.insert(feature, BTreeSet::new());
        }

        // Enable features.
        for (feature, enabled_feature) in self.module.enabled_features(config, &emission_location) {
            if let Some(features) = emitted_features.get_mut(&feature) {
                features.insert(enabled_feature);
            } else {
                error!(
                    ?feature,
                    ?enabled_feature,
                    "tried to set non-existent feature"
                );
            }
        }

        emitted_features
    }

    pub fn output(
        &self,
        crate_dir: &Path,
        test_crate_dir: &Path,
        config: &Config,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let generated_dir = crate_dir.join("src").join("generated");
        let test_path = test_crate_dir.join("tests").join(&self.data.framework);
        let emission_location = Location::from_library(&self.link_name);

        // Output `src/generated/*`.
        self.module.output(
            &generated_dir,
            &test_path,
            config,
            &emission_location,
            self.top_level_prefix(),
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
            writeln!(lib_rs, "#![cfg_attr(docsrs, feature(doc_auto_cfg))]")?;
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
        let framework_or_lib = if self.data.krate == "dispatch2" {
            "Grand Central Dispatch".to_string()
        } else {
            format!("framework {}", self.link_name)
        };
        let license = if self.data.krate == "objc2-foundation" {
            "MIT"
        } else {
            "Zlib%20OR%20Apache-2.0%20OR%20MIT"
        };
        writeln!(readme, "# `{0}`

[![Latest version](https://badgen.net/crates/v/{0})](https://crates.io/crates/{0})
[![License](https://badgen.net/badge/license/{license}/blue)](../../LICENSE.md)
[![Documentation](https://docs.rs/{0}/badge.svg)](https://docs.rs/{0}/)
[![CI](https://github.com/madsmtm/objc2/actions/workflows/ci.yml/badge.svg)](https://github.com/madsmtm/objc2/actions/workflows/ci.yml)

Rust bindings to Apple's {framework_or_lib}.

This README is kept intentionally small to consolidate the documentation, see
[the Rust docs](https://docs.rs/{0}/) for more details on this crate.

This crate is part of the [`objc2` project](https://github.com/madsmtm/objc2),
see that for related crates.", self.data.krate)?;
        readme.flush()?;

        let mut cargo_toml: DocumentMut = include_str!("default_cargo.toml")
            .parse()
            .expect("invalid default toml");

        cargo_toml["package"]["name"] = value(&self.data.krate);
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
        // Having the default target be tier 3 seems to not be supported.
        // https://github.com/rust-lang/docs.rs/issues/2748
        default_target.get_or_insert("aarch64-apple-darwin");
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
        if self.data.visionos.is_some() {
            default_target.get_or_insert("aarch64-apple-visionos");
            targets.push("aarch64-apple-visionos");
            if self.data.macos.is_none() && self.data.ios.is_none() {
                targets.push("aarch64-apple-visionos-sim");
            }
        }
        if self.data.gnustep {
            default_target.get_or_insert("x86_64-unknown-linux-gnu");
            targets.push("x86_64-unknown-linux-gnu");
            targets.push("i686-unknown-linux-gnu");
        }

        // Default max number of build targets is 10.
        if 10 < targets.len() {
            panic!(
                "won't be able to document this crate on docs.rs, too many targets:\n{targets:#?}"
            );
        }

        for item in targets.iter_mut() {
            item.decor_mut().set_prefix("\n    ");
        }
        targets.set_trailing_comma(true);
        cargo_toml["package"]["metadata"]["docs"]["rs"]["default-target"] =
            value(default_target.unwrap());

        for (krate, required_features) in self.dependencies(config) {
            let library = config.library_from_crate(krate);
            let required = self.data.required_crates.contains(krate);
            let mut table = InlineTable::from_iter([("workspace", Value::from(true))]);
            if !required {
                table.insert("optional", Value::from(true));
            }
            if !required_features.is_empty() {
                let mut array: Array = required_features.iter().collect();
                if 1 < array.len() {
                    for item in array.iter_mut() {
                        item.decor_mut().set_prefix("\n    ");
                    }
                    array.set_trailing("\n");
                    array.set_trailing_comma(true);
                }
                table.insert("features", Value::from(array));
            }

            let mut platform_cfg = PlatformCfg::from_config(&self.data);
            platform_cfg.dependency(library);
            let dependency_table = if let Some(cfgs) = platform_cfg.cfgs() {
                let dep_position = cargo_toml["dependencies"]
                    .as_table()
                    .unwrap()
                    .position()
                    .unwrap();

                let target = cargo_toml.entry("target").implicit_table();

                target.set_position(dep_position);

                let key = format!("'cfg({cfgs})'").parse().unwrap();
                target
                    .entry_format(&key)
                    .implicit_table()
                    .entry("dependencies")
                    .implicit_table()
            } else {
                cargo_toml["dependencies"].as_table_mut().unwrap()
            };

            // Don't override if set by Cargo.modified.toml
            dependency_table
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

        let mut emitted_features = self.emitted_features(config);

        cargo_toml["features"]["default"] = array_with_newlines(
            iter::once("std".to_string()).chain(
                emitted_features
                    .keys()
                    .filter(|feature| self.is_default_feature(feature, config))
                    .cloned(),
            ),
        );

        // Enable non-default features when building docs.
        let non_default_features: Vec<_> = emitted_features
            .keys()
            .filter(|feature| !self.is_default_feature(feature, config))
            .cloned()
            .collect();
        if !non_default_features.is_empty() {
            cargo_toml["package"]["metadata"]["docs"]["rs"]["features"] =
                array_with_newlines(non_default_features);
        }

        // Emit crate features first (the "default" feature overrides in
        // `default_cargo.toml`).
        for (feature, _) in emitted_features.clone().iter() {
            if config.try_library_from_crate(feature).is_none() {
                continue;
            }
            let enabled_features = emitted_features.remove(feature).unwrap();
            cargo_toml["features"][feature] = array_with_newlines(enabled_features);
        }

        // And then the rest of the features.
        if !emitted_features.is_empty() {
            add_newline_at_end(&mut cargo_toml["features"]);
        }
        for (feature, enabled_features) in emitted_features {
            if cargo_toml["features"].get(&feature).is_none() {
                cargo_toml["features"][feature] = array_with_newlines(enabled_features);
            }
        }

        fs::write(crate_dir.join("Cargo.toml"), cargo_toml.to_string())?;

        Ok(())
    }

    fn top_level_prefix(&self) -> impl fmt::Display + '_ {
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

            // Our auto-generated rustdoc is kinda bad.
            writeln!(f, "#![allow(clippy::doc_lazy_continuation)]")?;
            writeln!(f, "#![allow(rustdoc::broken_intra_doc_links)]")?;
            writeln!(f, "#![allow(rustdoc::bare_urls)]")?;
            writeln!(f, "#![allow(rustdoc::unportable_markdown)]")?;
            writeln!(f, "#![allow(rustdoc::invalid_html_tags)]")?;

            writeln!(f)?;

            if self.data.link {
                // Link to the correct framework.
                if self.data.gnustep {
                    // Allow for different linking on GNUStep
                    writeln!(
                    f,
                    "#[cfg_attr(target_vendor = \"apple\", link(name = \"{}\", kind = \"framework\"))]",
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
                (original, addition) => {
                    // Overwrite
                    *original = addition;
                }
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

fn array_with_newlines(features: impl IntoIterator<Item = String>) -> Item {
    let mut array: Array = features.into_iter().collect();
    if 1 < array.len() {
        for item in array.iter_mut() {
            item.decor_mut().set_prefix("\n    ");
        }
        array.set_trailing("\n");
        array.set_trailing_comma(true);
    }
    value(array)
}

pub trait EntryExt<'a> {
    fn implicit_table(self) -> &'a mut Table;
}

impl<'a> EntryExt<'a> for toml_edit::Entry<'a> {
    fn implicit_table(self) -> &'a mut Table {
        self.or_insert({
            let mut table = Table::new();
            table.set_implicit(true);
            Item::Table(table)
        })
        .as_table_mut()
        .unwrap()
    }
}
