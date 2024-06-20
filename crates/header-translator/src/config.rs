use std::collections::{BTreeMap, HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;

use heck::ToTrainCase;
use semver::Version;
use serde::{de, Deserialize, Deserializer};

use crate::stmt::{Derives, Mutability};
use crate::ItemIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub libraries: BTreeMap<String, LibraryConfig>,
    pub system: LibraryConfig,
}

fn uses_system_config(library_name: &str) -> bool {
    match library_name {
        "System" | "bitflags" | "block2" | "libc" | "objc2" => true,
        // Temporary
        "CoreFoundation" => true,
        _ => false,
    }
}

impl Config {
    pub fn library(&self, library_name: &str) -> &LibraryConfig {
        if uses_system_config(library_name) {
            &self.system
        } else {
            self.libraries.get(library_name).unwrap_or_else(|| {
                error!("tried to get library config from {library_name:?}");
                &self.system
            })
        }
    }

    pub fn library_from_crate(&self, krate: &str) -> &LibraryConfig {
        if uses_system_config(krate) {
            &self.system
        } else {
            self.libraries
                .values()
                .find(|lib| lib.krate == krate)
                .unwrap_or_else(|| {
                    error!("tried to get library config from krate {krate:?}");
                    &self.system
                })
        }
    }

    pub fn replace_protocol_name(&self, id: ItemIdentifier) -> ItemIdentifier {
        let library_config = self.library(id.library_name());
        id.map_name(|name| {
            library_config
                .protocol_data
                .get(&name)
                .and_then(|data| data.renamed.clone())
                .unwrap_or(name)
        })
    }
}

fn get_version<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Version>, D::Error> {
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
pub struct LibraryConfig {
    pub framework: String,
    #[serde(rename = "crate")]
    pub krate: String,
    #[serde(rename = "umbrella-header")]
    pub umbrella_header: Option<String>,
    /// Dependencies are optional by default, this can be used to make a
    /// dependency required.
    ///
    /// This is used when depending on `objc2-foundation`, as we don't really
    /// want a feature for something as fundamental as `NSString`.
    /// Additionally, it is used for things like `MetalKit` always wanting
    /// `Metal` enabled.
    #[serde(rename = "required-dependencies")]
    pub required_dependencies: HashSet<String>,
    #[serde(rename = "custom-lib-rs")]
    #[serde(default)]
    pub custom_lib_rs: bool,

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
    #[serde(deserialize_with = "get_version")]
    pub visionos: Option<Version>,
    #[serde(default)]
    pub gnustep: bool,

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
    #[serde(default)]
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
    #[serde(default)]
    pub renamed: Option<String>,
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

impl LibraryConfig {
    pub fn from_file(file: &Path) -> Result<Self, Box<dyn Error>> {
        let s = fs::read_to_string(file)?;

        let config: Self = basic_toml::from_str(&s)?;

        assert_eq!(
            config.framework.to_lowercase(),
            config.krate.replace("objc2-", "").replace('-', ""),
            "crate name had an unexpected format",
        );
        assert_eq!(
            Some(&*config.framework.to_train_case().to_lowercase()),
            config.krate.strip_prefix("objc2-"),
            "crate name had an unexpected format",
        );

        Ok(config)
    }
}

impl<'de> Deserialize<'de> for Mutability {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        fn parse_itemidentifier(value: &str) -> Option<ItemIdentifier> {
            let (library, rest) = value.split_once("::")?;
            let (file_name, name) = rest.split_once("::")?;
            Some(ItemIdentifier::from_raw(
                name.into(),
                vec![library.to_string().into(), file_name.to_string().into()],
            ))
        }

        struct MutabilityVisitor;

        impl<'de> de::Visitor<'de> for MutabilityVisitor {
            type Value = Mutability;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("mutability")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if let Some(value) = value.strip_prefix("ImmutableWithMutableSubclass(") {
                    let value = value
                        .strip_suffix(')')
                        .ok_or(de::Error::custom("end parenthesis"))?;
                    let item =
                        parse_itemidentifier(value).ok_or(de::Error::custom("requires ::"))?;
                    return Ok(Mutability::ImmutableWithMutableSubclass(item));
                }

                if let Some(value) = value.strip_prefix("MutableWithImmutableSuperclass(") {
                    let value = value
                        .strip_suffix(')')
                        .ok_or(de::Error::custom("end parenthesis"))?;
                    let item =
                        parse_itemidentifier(value).ok_or(de::Error::custom("requires ::"))?;
                    return Ok(Mutability::MutableWithImmutableSuperclass(item));
                }

                if let Some(value) = value.strip_prefix("InteriorMutableWithSubclass(") {
                    let value = value
                        .strip_suffix(')')
                        .ok_or(de::Error::custom("end parenthesis"))?;
                    let item =
                        parse_itemidentifier(value).ok_or(de::Error::custom("requires ::"))?;
                    return Ok(Mutability::InteriorMutableWithSubclass(item));
                }

                if let Some(value) = value.strip_prefix("InteriorMutableWithSuperclass(") {
                    let value = value
                        .strip_suffix(')')
                        .ok_or(de::Error::custom("end parenthesis"))?;
                    let item =
                        parse_itemidentifier(value).ok_or(de::Error::custom("requires ::"))?;
                    return Ok(Mutability::InteriorMutableWithSuperclass(item));
                }

                match value {
                    "Immutable" => Ok(Mutability::Immutable),
                    "Mutable" => Ok(Mutability::Mutable),
                    "InteriorMutable" => Ok(Mutability::InteriorMutable),
                    "MainThreadOnly" => Ok(Mutability::MainThreadOnly),
                    value => Err(de::Error::custom(format!("unknown variant {value:?}"))),
                }
            }
        }

        deserializer.deserialize_str(MutabilityVisitor)
    }
}
