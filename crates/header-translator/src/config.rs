use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;

use semver::Version;
use serde::Deserialize;

use crate::data;
use crate::stmt::{Derives, Mutability};

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
}

fn get_version<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Version>, D::Error> {
    use serde::de;

    struct VersionVisitor;

    impl de::Visitor<'_> for VersionVisitor {
        type Value = Option<Version>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a version string")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_borrowed_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(
                lenient_semver_parser::parse::<Version>(v).map_err(de::Error::custom)?,
            ))
        }
    }

    deserializer.deserialize_str(VersionVisitor)
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LibraryData {
    #[serde(rename = "pretty-name")]
    #[serde(default)]
    pub pretty_name: Option<String>,
    pub imports: Vec<String>,
    #[serde(rename = "gnustep-library")]
    #[serde(default)]
    pub gnustep_library: Option<String>,
    #[serde(default)]
    #[serde(rename = "extra-docs")]
    pub extra_docs: String,
    #[serde(default)]
    #[serde(rename = "additions")]
    pub has_additions: bool,
    #[serde(default)]
    #[serde(rename = "fixes")]
    pub has_fixes: bool,
    #[serde(rename = "extra-features")]
    #[serde(default)]
    pub extra_features: Vec<String>,
    #[serde(default)]
    #[serde(deserialize_with = "get_version")]
    pub macos: Option<Version>,
    #[serde(default)]
    #[serde(deserialize_with = "get_version")]
    pub maccatalyst: Option<Version>,
    #[serde(default)]
    #[serde(deserialize_with = "get_version")]
    pub ios: Option<Version>,
    #[serde(default)]
    #[serde(deserialize_with = "get_version")]
    pub tvos: Option<Version>,
    #[serde(default)]
    #[serde(deserialize_with = "get_version")]
    pub watchos: Option<Version>,
    #[serde(default)]
    pub examples: Vec<Example>,
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Example {
    pub name: String,
    #[serde(default)]
    pub description: String,
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
    #[serde(skip)]
    pub mutability: Mutability,
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
    #[serde(rename = "requires-mainthreadonly")]
    pub requires_mainthreadonly: Option<bool>,
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
    pub mutating: Option<bool>,
}

impl MethodData {
    pub(crate) fn merge_with_superclass(self, superclass: Self) -> Self {
        Self {
            // Only use `unsafe` from itself, never take if from the superclass
            unsafe_: self.unsafe_,
            skipped: self.skipped | superclass.skipped,
            mutating: self.mutating.or(superclass.mutating),
        }
    }
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

impl Default for MethodData {
    fn default() -> Self {
        Self {
            unsafe_: unsafe_default(),
            skipped: skipped_default(),
            mutating: None,
        }
    }
}

impl Config {
    pub fn from_file(file: &Path) -> Result<Self, Box<dyn Error>> {
        let s = fs::read_to_string(file)?;

        let mut this = basic_toml::from_str(&s)?;

        data::apply_tweaks(&mut this);

        Ok(this)
    }
}
