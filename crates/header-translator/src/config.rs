use std::collections::HashMap;
use std::fs;
use std::io::Result;
use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(rename = "class")]
    #[serde(default)]
    pub class_data: HashMap<String, ClassData>,
    #[serde(rename = "protocol")]
    #[serde(default)]
    pub protocol_data: HashMap<String, ClassData>,
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
    #[serde(default)]
    pub imports: Vec<String>,
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ClassData {
    #[serde(default)]
    pub skipped: bool,
    #[serde(rename = "superclass-name")]
    #[serde(default)]
    pub new_superclass_name: Option<String>,
    #[serde(default)]
    pub methods: HashMap<String, MethodData>,
    #[serde(default)]
    pub properties: HashMap<String, MethodData>,
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
    // TODO: mutating
}

// TODO
pub type FnData = StructData;
pub type StaticData = StructData;
pub type TypedefData = StructData;

fn unsafe_default() -> bool {
    true
}

fn skipped_default() -> bool {
    false
}

impl Default for MethodData {
    fn default() -> Self {
        Self {
            unsafe_: unsafe_default(),
            skipped: skipped_default(),
        }
    }
}

impl Config {
    pub fn from_file(file: &Path) -> Result<Self> {
        let s = fs::read_to_string(file)?;

        Ok(toml::from_str(&s)?)
    }
}
