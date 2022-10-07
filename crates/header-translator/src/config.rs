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
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ClassData {
    #[serde(default)]
    pub methods: HashMap<String, MethodData>,
    #[serde(default)]
    pub properties: HashMap<String, MethodData>,
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
