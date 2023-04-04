use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Result;
use std::path::Path;

use serde::Deserialize;

use crate::availability::Unavailable;
use crate::data;
use crate::rust_type::Ownership;
use crate::stmt::Derives;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(rename = "class")]
    #[serde(default)]
    pub class_data: HashMap<String, ClassData>,
    #[serde(rename = "protocol")]
    #[serde(default)]
    pub protocol_data: HashMap<String, ProtocolData>,
    #[serde(rename = "struct")]
    #[serde(default)]
    pub struct_data: HashMap<String, StructData>,
    #[serde(rename = "enum")]
    #[serde(default)]
    pub enum_data: HashMap<String, EnumData>,
    #[serde(rename = "fn")]
    #[serde(default)]
    pub fns: HashMap<String, FnData>,
    #[serde(rename = "static")]
    #[serde(default)]
    pub statics: HashMap<String, StaticData>,
    #[serde(rename = "typedef")]
    #[serde(default)]
    pub typedef_data: HashMap<String, TypedefData>,
    #[serde(rename = "library")]
    #[serde(default)]
    pub libraries: HashMap<String, LibraryData>,
}

impl Config {
    pub fn replace_protocol_name(&self, name: String) -> String {
        self.protocol_data
            .get(&name)
            .and_then(|data| data.renamed.clone())
            .unwrap_or(name)
    }

    pub fn get_library_alias(&self, library_name: String) -> String {
        self.libraries
            .iter()
            .find_map(|(n, data)| {
                if let Some(name) = &data.name {
                    if n == &library_name {
                        return Some(name.clone());
                    }
                }
                None
            })
            .unwrap_or(library_name)
    }
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LibraryData {
    #[serde(default)]
    pub name: Option<String>,
    pub imports: Vec<String>,
    #[serde(rename = "extra-features")]
    #[serde(default)]
    pub extra_features: Vec<String>,
    #[serde(default)]
    pub macos: Option<semver::VersionReq>,
    #[serde(default)]
    pub maccatalyst: Option<semver::VersionReq>,
    #[serde(default)]
    pub ios: Option<semver::VersionReq>,
    #[serde(default)]
    pub tvos: Option<semver::VersionReq>,
    #[serde(default)]
    pub watchos: Option<semver::VersionReq>,
}
impl LibraryData {
    pub(crate) fn unavailability(&self) -> Unavailable {
        Unavailable {
            ios: self.ios.is_none(),
            macos: self.macos.is_none(),
            tvos: self.tvos.is_none(),
            watchos: self.watchos.is_none(),
            maccatalyst: self.maccatalyst.is_none(),
            ..Default::default()
        }
    }
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ClassData {
    #[serde(default)]
    pub skipped: bool,
    #[serde(rename = "definition-skipped")]
    #[serde(default)]
    pub definition_skipped: bool,
    #[serde(default)]
    pub methods: HashMap<String, MethodData>,
    #[serde(default)]
    pub categories: HashMap<String, CategoryData>,
    #[serde(default)]
    pub derives: Derives,
    #[serde(rename = "owned")]
    #[serde(default)]
    pub ownership: Ownership,
    #[serde(rename = "skipped-protocols")]
    #[serde(default)]
    pub skipped_protocols: HashSet<String>,
}

impl ClassData {
    pub fn get_method_data(this: Option<&Self>, name: &str) -> MethodData {
        this.map(|data| data.methods.get(name).copied().unwrap_or_default())
            .unwrap_or_default()
    }
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CategoryData {
    #[serde(default)]
    pub skipped: bool,
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProtocolData {
    #[serde(default)]
    pub renamed: Option<String>,
    #[serde(default)]
    pub skipped: bool,
    #[serde(default)]
    pub methods: HashMap<String, MethodData>,
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct StructData {
    #[serde(default)]
    pub skipped: bool,
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EnumData {
    #[serde(default)]
    pub skipped: bool,
    #[serde(rename = "use-value")]
    #[serde(default)]
    pub use_value: bool,
    #[serde(default)]
    pub constants: HashMap<String, StructData>,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct MethodData {
    #[serde(rename = "unsafe")]
    #[serde(default = "unsafe_default")]
    pub unsafe_: bool,
    #[serde(default = "skipped_default")]
    pub skipped: bool,
    #[serde(default = "mutating_default")]
    pub mutating: bool,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FnData {
    #[serde(default)]
    pub skipped: bool,
    #[serde(rename = "unsafe")]
    #[serde(default = "unsafe_default")]
    pub unsafe_: bool,
}

impl Default for FnData {
    fn default() -> Self {
        Self {
            skipped: skipped_default(),
            unsafe_: unsafe_default(),
        }
    }
}

// TODO
pub type StaticData = StructData;
pub type TypedefData = StructData;

fn unsafe_default() -> bool {
    true
}

fn skipped_default() -> bool {
    false
}

fn mutating_default() -> bool {
    false
}

impl Default for MethodData {
    fn default() -> Self {
        Self {
            unsafe_: unsafe_default(),
            skipped: skipped_default(),
            mutating: mutating_default(),
        }
    }
}

impl Config {
    pub fn from_file(file: &Path) -> Result<Self> {
        let s = fs::read_to_string(file)?;

        let mut this = toml::from_str(&s)?;

        data::apply_tweaks(&mut this);

        Ok(this)
    }
}
