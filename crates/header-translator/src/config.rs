use std::collections::{BTreeMap, HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use heck::ToTrainCase;
use semver::Version;
use serde::{de, Deserialize, Deserializer};

use crate::stmt::{Counterpart, Derives};
use crate::{ItemIdentifier, Location};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    libraries: BTreeMap<String, LibraryConfig>,
}

pub fn load_skipped() -> Result<BTreeMap<String, String>, Box<dyn Error + Send + Sync>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("configs")
        .join("skipped.toml");
    Ok(basic_toml::from_str(&fs::read_to_string(path)?)?)
}

pub fn load_config() -> Result<Config, Box<dyn Error + Send + Sync>> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir.parent().unwrap().parent().unwrap();

    let _span = info_span!("loading configs").entered();

    let mut libraries = BTreeMap::default();

    for dir in fs::read_dir(workspace_dir.join("framework-crates"))? {
        let dir = dir?;
        if !dir.file_type()?.is_dir() {
            continue;
        }
        let path = dir.path().join("translation-config.toml");
        let config =
            LibraryConfig::from_file(&path).unwrap_or_else(|e| panic!("read {path:?} config: {e}"));
        assert_eq!(*config.krate, *dir.file_name());
        libraries.insert(config.framework.to_string(), config);
    }

    let path = workspace_dir
        .join("crates")
        .join("block2")
        .join("translation-config.toml");
    let objc = basic_toml::from_str(&fs::read_to_string(path)?)?;
    libraries.insert("block".to_string(), objc);

    let path = workspace_dir
        .join("crates")
        .join("objc2")
        .join("translation-config.toml");
    let objc = basic_toml::from_str(&fs::read_to_string(path)?)?;
    libraries.insert("ObjectiveC".to_string(), objc);

    let path = workspace_dir
        .join("crates")
        .join("dispatch2")
        .join("translation-config.toml");
    let objc = basic_toml::from_str(&fs::read_to_string(path)?)?;
    libraries.insert("Dispatch".to_string(), objc);

    Config::new(libraries)
}

impl Config {
    pub fn new(
        mut libraries: BTreeMap<String, LibraryConfig>,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let configs_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("configs");

        let builtin_files = ["bitflags.toml", "builtin.toml", "core.toml", "libc.toml"];

        for builtin_file in builtin_files {
            let path = configs_dir.join(builtin_file);
            let config: LibraryConfig = basic_toml::from_str(&fs::read_to_string(path)?)?;
            libraries.insert(config.framework.clone(), config);
        }

        for framework in load_skipped()?.keys() {
            assert!(
                !libraries.contains_key(framework),
                "skipped framework {framework} was not actually skipped"
            );
        }

        Ok(Self { libraries })
    }

    pub fn try_library(&self, library_name: &str) -> Option<&LibraryConfig> {
        self.libraries.get(library_name)
    }

    /// Look up the library config.
    ///
    /// This only needs the library name, but it takes ItemIdentifier or
    /// Location for better error reporting.
    pub fn library(&self, location: impl AsRef<Location> + fmt::Debug) -> &LibraryConfig {
        self.try_library(location.as_ref().library_name())
            .unwrap_or_else(|| {
                error!("tried to get library config from {location:?}");
                self.libraries
                    .get("__builtin__")
                    .expect("could not find builtin library")
            })
    }

    pub fn library_from_crate(&self, krate: &str) -> &LibraryConfig {
        self.try_library_from_crate(krate).unwrap_or_else(|| {
            error!("tried to get library config from krate {krate:?}");
            self.libraries
                .get("__builtin__")
                .expect("could not find builtin library")
        })
    }

    pub fn try_library_from_crate(&self, krate: &str) -> Option<&LibraryConfig> {
        self.libraries.values().find(|lib| lib.krate == krate)
    }

    pub fn replace_protocol_name(&self, id: ItemIdentifier) -> ItemIdentifier {
        let library_config = self.library(&id);
        id.map_name(|name| {
            library_config
                .protocol_data
                .get(&name)
                .and_then(|data| data.renamed.clone())
                .unwrap_or(name)
        })
    }

    pub fn replace_typedef_name(&self, id: ItemIdentifier, is_cf: bool) -> ItemIdentifier {
        let library_config = self.library(&id);
        id.map_name(|name| {
            library_config
                .typedef_data
                .get(&name)
                .and_then(|data| data.renamed.clone())
                .unwrap_or_else(|| {
                    // If a typedef's underlying type is itself a "CF pointer"
                    // typedef, the "alias" typedef will be imported as a
                    // regular typealias, with the suffix "Ref" still dropped
                    // from its name (if present).
                    //
                    // <https://github.com/swiftlang/swift/blob/swift-6.0.3-RELEASE/docs/CToSwiftNameTranslation.md#cf-types>
                    //
                    // NOTE: There's an extra clause that we don't support:
                    // > unless doing so would conflict with another
                    // > declaration in the same module as the typedef.
                    //
                    // We'll have to manually keep the name of those in
                    // translation-config.toml.
                    if is_cf {
                        if let Some(name) = name.strip_suffix("Ref") {
                            name.to_string()
                        } else {
                            name
                        }
                    } else {
                        name
                    }
                })
        })
    }

    pub fn to_parse(&self) -> impl Iterator<Item = (&str, &LibraryConfig)> + Clone {
        self.libraries
            .iter()
            .filter(|(_, data)| !data.skipped)
            .map(|(name, data)| (&**name, data))
    }

    pub fn module_configs<'l>(
        &'l self,
        location: &'l Location,
    ) -> impl Iterator<Item = &'l ModuleConfig> + 'l {
        self.try_library(location.library_name())
            .map(|library| {
                let mut current = &library.module;
                location.modules().map_while(move |component| {
                    if let Some(module_config) = current.get(component) {
                        current = &module_config.module;
                        Some(module_config)
                    } else {
                        None
                    }
                })
            })
            .into_iter()
            .flatten()
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

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ExternalData {
    pub module: Location,
    #[serde(rename = "thread-safety")]
    #[serde(default)]
    pub thread_safety: Option<String>,
    #[serde(rename = "super-items")]
    #[serde(default)]
    pub super_items: Vec<ItemIdentifier>,
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LibraryConfig {
    pub framework: String,
    #[serde(rename = "crate")]
    pub krate: String,
    /// Dependencies are optional by default, this can be used to make a
    /// dependency required.
    ///
    /// This is used when depending on `objc2-foundation`, as we don't really
    /// want a feature for something as fundamental as `NSString`.
    /// Additionally, it is used for things like `MetalKit` always wanting
    /// `Metal` enabled.
    #[serde(rename = "required-crates")]
    pub required_crates: HashSet<String>,
    #[serde(rename = "custom-lib-rs")]
    #[serde(default)]
    pub custom_lib_rs: bool,
    #[serde(default)]
    pub modulemap: Option<String>,
    #[serde(rename = "is-library")]
    #[serde(default)]
    pub is_library: bool,

    #[serde(default = "link_default")]
    pub link: bool,
    /// Whether we will attempt to parse and emit the library
    /// (used for built-in modules).
    #[serde(default)]
    pub skipped: bool,

    /// Extra compilation flags.
    #[serde(default)]
    pub flags: Vec<String>,

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

    /// Data about an external class or protocol whose header isn't imported.
    ///
    /// I.e. a bare `@protocol X;` or `@class X;`.
    #[serde(default)]
    pub external: BTreeMap<String, ExternalData>,

    #[serde(rename = "class")]
    #[serde(default)]
    pub class_data: HashMap<String, ClassData>,
    #[serde(rename = "protocol")]
    #[serde(default)]
    pub protocol_data: HashMap<String, ProtocolData>,
    #[serde(rename = "struct")]
    #[serde(default)]
    pub struct_data: HashMap<String, RecordData>,
    #[serde(rename = "union")]
    #[serde(default)]
    pub union_data: HashMap<String, RecordData>,
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
    #[serde(rename = "const")]
    #[serde(default)]
    pub const_data: HashMap<String, ConstantData>,

    #[serde(default)]
    pub module: HashMap<String, ModuleConfig>,
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ModuleConfig {
    #[serde(default)]
    pub skipped: bool,
    #[serde(default)]
    pub module: HashMap<String, ModuleConfig>,
}

impl LibraryConfig {
    // TODO: Merge this with `Availability` somehow.
    pub(crate) fn can_safely_depend_on(&self, other: &Self) -> bool {
        fn inner(
            ours: &Option<semver::Version>,
            other: &Option<semver::Version>,
            rust_min: semver::Version,
        ) -> bool {
            match (ours, other) {
                // If both libraries have a platform version, then ensure that
                // ours is within the minimum of the other, OR that Rust's
                // default min version is high enough that it won't matter.
                (Some(ours), Some(other)) => other <= ours || *other <= rust_min,
                // If only we have support for a platform, then we will emit a
                // cfg-guarded [dependencies] table (done elsewhere), and thus
                // it won't affect whether we can safely depend on it.
                (Some(_), None) => true,
                // If only the other library has support for platform, then
                // that's fine.
                (None, Some(_)) => true,
                // If neither library support the platform, that's also fine.
                (None, None) => true,
            }
        }

        inner(&self.macos, &other.macos, semver::Version::new(10, 12, 0))
            && inner(
                &self.maccatalyst,
                &other.maccatalyst,
                semver::Version::new(13, 1, 0),
            )
            && inner(&self.ios, &other.ios, semver::Version::new(10, 0, 0))
            && inner(&self.tvos, &other.tvos, semver::Version::new(10, 0, 0))
            && inner(&self.watchos, &other.watchos, semver::Version::new(5, 0, 0))
            && inner(
                &self.visionos,
                &other.visionos,
                semver::Version::new(1, 0, 0),
            )
    }
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
    pub counterpart: Counterpart,
    #[serde(default)]
    #[serde(rename = "main-thread-only")]
    pub main_thread_only: bool,
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
    pub skipped: bool,
    #[serde(default)]
    pub renamed: Option<String>,
    #[serde(default)]
    #[serde(rename = "requires-mainthreadonly")]
    pub requires_mainthreadonly: Option<bool>,
    #[serde(default)]
    pub methods: HashMap<String, MethodData>,
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RecordData {
    #[serde(default)]
    pub skipped: bool,
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ConstantData {
    #[serde(default)]
    pub skipped: bool,
    #[serde(rename = "use-value")]
    #[serde(default)]
    pub use_value: Option<bool>,
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EnumData {
    #[serde(default)]
    pub skipped: bool,
    #[serde(rename = "use-value")]
    #[serde(default)]
    pub use_value: Option<bool>,
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct StaticData {
    #[serde(default)]
    pub skipped: bool,
    #[serde(rename = "use-value")]
    #[serde(default)]
    pub use_value: bool,
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TypedefData {
    #[serde(default)]
    pub skipped: bool,
    #[serde(default)]
    pub renamed: Option<String>,
    #[serde(default)]
    pub generics: Vec<String>,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct MethodData {
    #[serde(rename = "unsafe")]
    #[serde(default = "unsafe_default")]
    pub unsafe_: bool,
    #[serde(default = "skipped_default")]
    pub skipped: bool,
}

impl MethodData {
    pub(crate) fn merge_with_superclass(self, superclass: Self) -> Self {
        Self {
            // Only use `unsafe` from itself, never take if from the superclass
            unsafe_: self.unsafe_,
            skipped: self.skipped | superclass.skipped,
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

fn unsafe_default() -> bool {
    true
}

fn skipped_default() -> bool {
    false
}

fn link_default() -> bool {
    true
}

impl Default for MethodData {
    fn default() -> Self {
        Self {
            unsafe_: unsafe_default(),
            skipped: skipped_default(),
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
        if matches!(&*config.krate, "objc2-tv-ml-kit" | "objc2-tv-ui-kit") {
            // Named this way for better consistency with other tv-specific crates.
            return Ok(config);
        }
        if config.krate == "objc2-javascript-core" {
            // Nobody writes "java-script".
            return Ok(config);
        }
        if config.krate == "objc2-itunes-library" {
            // "i-tunes" is confusing.
            return Ok(config);
        }
        if config.krate == "objc2-io-usb-host" {
            // "io-usb" is clearer than "iousb", and more consistent with other "io" crates.
            return Ok(config);
        }
        assert_eq!(
            Some(&*config.framework.to_train_case().to_lowercase()),
            config.krate.strip_prefix("objc2-"),
            "crate name had an unexpected format",
        );

        Ok(config)
    }
}

impl<'de> de::Deserialize<'de> for Counterpart {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct CounterpartVisitor;

        impl de::Visitor<'_> for CounterpartVisitor {
            type Value = Counterpart;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("item identifier")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if let Some(value) = value.strip_prefix("ImmutableSuperclass(") {
                    let value = value
                        .strip_suffix(')')
                        .ok_or_else(|| de::Error::custom("end parenthesis"))?;
                    let item = ItemIdentifier::from_str(value).map_err(de::Error::custom)?;
                    return Ok(Counterpart::ImmutableSuperclass(item));
                }

                if let Some(value) = value.strip_prefix("MutableSubclass(") {
                    let value = value
                        .strip_suffix(')')
                        .ok_or_else(|| de::Error::custom("end parenthesis"))?;
                    let item = ItemIdentifier::from_str(value).map_err(de::Error::custom)?;
                    return Ok(Counterpart::MutableSubclass(item));
                }

                Err(de::Error::custom(format!("unknown variant {value:?}")))
            }
        }

        deserializer.deserialize_str(CounterpartVisitor)
    }
}
