use core::fmt;
use core::hash;
use serde::de;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::error::Error;
use std::str::FromStr;

use clang::source::File;
use clang::Entity;

use crate::cfgs::PlatformCfg;
use crate::context::Context;
use crate::display_helper::FormatterFn;
use crate::module::clean_name;
use crate::Config;

pub trait ToOptionString: fmt::Debug {
    fn to_option(&self) -> Option<&str>;
}

impl ToOptionString for String {
    fn to_option(&self) -> Option<&str> {
        Some(self)
    }
}

impl ToOptionString for Option<String> {
    fn to_option(&self) -> Option<&str> {
        self.as_deref()
    }
}

impl ToOptionString for () {
    fn to_option(&self) -> Option<&str> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Location {
    // A Swift/Clang module path (dot-separated).
    module_path: Box<str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LocationLibrary<'location, 'config> {
    System,
    Bitflags,
    Block2,
    Libc,
    Objc2,
    InSameLibrary {
        library: &'location str,
        file_name: Option<&'location str>,
        krate: &'config str,
    },
    InExternalLibrary {
        library: &'location str,
        file_name: Option<&'location str>,
        krate: &'config str,
        required: bool,
    },
}

impl<'config> LocationLibrary<'_, 'config> {
    pub fn krate(&self) -> Option<(&'config str, bool)> {
        match self {
            Self::System => None,
            Self::Bitflags => Some(("bitflags", false)),
            Self::Block2 => Some(("block2", false)),
            Self::Libc => Some(("libc", false)),
            Self::Objc2 => Some(("objc2", true)),
            Self::InSameLibrary { .. } => None,
            Self::InExternalLibrary {
                krate, required, ..
            } => Some((krate, *required)),
        }
    }

    pub fn import(&self) -> Option<(&'config str, bool)> {
        match self {
            Self::Objc2 => Some(("objc2::__framework_prelude", true)),
            Self::InExternalLibrary {
                krate, required, ..
            } => Some((krate, *required)),
            _ => None,
        }
    }

    pub fn cargo_toml_feature(&self) -> Option<String> {
        match self {
            Self::Bitflags => Some("bitflags".to_string()),
            Self::InExternalLibrary {
                file_name: Some(file_name),
                krate,
                required,
                ..
            } => Some(format!(
                "{krate}{}/{}",
                if *required { "" } else { "?" },
                clean_name(file_name),
            )),
            _ => None,
        }
    }

    // FIXME: This is currently wrong for nested umbrella frameworks
    // (specifically MetalPerformanceShaders).
    fn feature(&self) -> Option<String> {
        match self {
            Self::Block2 => Some("block2".to_string()),
            Self::Libc => Some("libc".to_string()),
            // Always enabled in the current file
            Self::Bitflags | Self::Objc2 => None,
            Self::InSameLibrary {
                file_name: Some(file_name),
                ..
            } => Some(clean_name(file_name)),
            Self::InExternalLibrary {
                krate,
                required: false,
                ..
            } => Some(krate.to_string()),
            _ => None,
        }
    }
}

impl Location {
    fn new(module_path: impl Into<Box<str>>) -> Self {
        let module_path = module_path.into();
        let module_path = match &*module_path {
            // blocks
            "block" => "block2".into(),

            // Objective-C
            name if name.starts_with("ObjectiveC") => "objc2".into(),

            // Redefined in the framework crate itself.
            "Darwin.MacTypes" => "System".into(),

            // Built-ins
            "DarwinFoundation.types.machine_types" => "System".into(),
            "_Builtin_stdarg.va_list" => "System".into(),

            // Libc
            name if name.starts_with("sys_types") => "libc".into(),
            "DarwinFoundation.types.sys_types" => "libc".into(),
            "DarwinFoundation.qos" => "libc".into(),
            name if name.starts_with("Darwin.POSIX") => "libc".into(),
            "_stdio" => "libc".into(),
            "_time.timespec" => "libc".into(),

            // Will be moved to the `mach2` crate in `libc` v1.0
            name if name.starts_with("Darwin.Mach") => "libc".into(),
            "_mach_port_t" => "libc".into(),

            _ => module_path,
        };

        Self { module_path }
    }

    pub fn from_file(file: File<'_>) -> Self {
        // Get from module first if available
        if let Some(module) = file.get_module() {
            return Self::new(module.get_full_name());
        }

        let path = file.get_path();

        if !path.to_string_lossy().contains("System/Library/Frameworks") {
            // Likely a built-in macro from stddef.h, stdarg.h or assert.h.
            return Self::new("System");
        }

        // The item likely comes from a private sub-framework, so let's try
        // to parse framework names from the sub-framework here.
        let mut components: Vec<Cow<'_, str>> = path
            .components()
            .map(|component| component.as_os_str())
            .skip_while(|s| !s.as_encoded_bytes().ends_with(b".sdk"))
            .skip(1)
            .map(|s| s.to_str().expect("component to_str"))
            .filter(|s| {
                !matches!(
                    *s,
                    "System" | "Library" | "Frameworks" | "Headers" | "iOSSupport"
                )
            })
            .map(|component| component.strip_suffix(".framework").unwrap_or(component))
            .map(|component| component.strip_suffix(".h").unwrap_or(component))
            .map(|s| s.to_string().into())
            .collect();

        if let [.., second_last, last] = &*components {
            if second_last == last {
                // Remove umbrella header
                components.pop();
            }
        }

        Self {
            module_path: components.join(".").into_boxed_str(),
        }
    }

    pub fn library_name(&self) -> &str {
        if let Some((library, _rest)) = self.module_path.split_once('.') {
            library
        } else {
            // Top-level
            &self.module_path
        }
    }

    fn file_name(&self) -> Option<&str> {
        if let Some((_umbrella, rest)) = self.module_path.split_once('.') {
            if let Some((_rest, file_name)) = rest.rsplit_once('.') {
                Some(file_name)
            } else {
                Some(rest)
            }
        } else {
            // Umbrella header
            None
        }
    }

    pub fn modules(&self) -> impl IntoIterator<Item = &'_ str> + '_ {
        self.module_path.split('.').skip(1)
    }

    pub fn library<'location, 'config>(
        &'location self,
        config: &'config Config,
        emission_library: &str,
    ) -> LocationLibrary<'location, 'config> {
        match self.library_name() {
            "System" => LocationLibrary::System,
            "bitflags" => LocationLibrary::Bitflags,
            "block2" => LocationLibrary::Block2,
            "libc" => LocationLibrary::Libc,
            "objc2" => LocationLibrary::Objc2,
            library => {
                if let Some(krate) = config.libraries.get(library).map(|lib| &*lib.krate) {
                    if library == emission_library {
                        LocationLibrary::InSameLibrary {
                            library,
                            file_name: self.file_name(),
                            krate,
                        }
                    } else {
                        let file_name = self.file_name();
                        let required = config
                            .libraries
                            .get(emission_library)
                            .unwrap_or_else(|| panic!("{emission_library} not found in libraries"))
                            .required_dependencies
                            .contains(krate);
                        LocationLibrary::InExternalLibrary {
                            library,
                            file_name,
                            krate,
                            required,
                        }
                    }
                } else {
                    error!(location = ?self, "failed getting crate name");
                    LocationLibrary::System
                }
            }
        }
    }

    pub fn assert_file(&self, file_name: &str) {
        if self.file_name() != Some(file_name) {
            error!(?self, ?file_name, "expected to be in file");
        }
    }
}

/// Names in C and Objective-C are global, so this is always enough to
/// uniquely identify an item.
///
/// Often, though, we want to know the library, file name and general location
/// an item came from as well.
#[derive(Debug, Clone)]
pub struct ItemIdentifier<N = String> {
    pub name: N,
    location: Location,
}

impl<N: ToOptionString + PartialEq> PartialEq for ItemIdentifier<N> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<N: ToOptionString + Eq> Eq for ItemIdentifier<N> {}

impl<N: ToOptionString + hash::Hash> hash::Hash for ItemIdentifier<N> {
    fn hash<H: hash::Hasher>(&self, _state: &mut H) {}
}

impl<N: ToOptionString + PartialOrd> PartialOrd for ItemIdentifier<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl<N: ToOptionString + Ord> Ord for ItemIdentifier<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl<N: ToOptionString> ItemIdentifier<N> {
    pub fn library_name(&self) -> &str {
        self.location.library_name()
    }

    pub fn from_raw(name: N, location: Location) -> Self {
        Self { name, location }
    }

    pub fn with_name(name: N, entity: &Entity<'_>, _context: &Context<'_>) -> Self {
        let file = entity
            .get_location()
            .unwrap_or_else(|| panic!("no entity location: {entity:?}"))
            .get_expansion_location()
            .file
            .expect("expanded location file");

        let mut location = Location::from_file(file);

        // Defined in multiple places for some reason.
        if let Some("IOSurfaceRef" | "__IOSurface") = name.to_option() {
            location = Location::new("IOSurface.IOSurfaceRef");
        }

        Self { name, location }
    }

    pub fn map_name<R: ToOptionString>(self, f: impl FnOnce(N) -> R) -> ItemIdentifier<R> {
        let Self { name, location } = self;
        ItemIdentifier {
            name: f(name),
            location,
        }
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn into_location(self) -> Location {
        self.location
    }

    pub fn with_new_path<R: ToOptionString>(self, other: &ItemIdentifier<R>) -> Self {
        Self {
            name: self.name,
            location: other.location.clone(),
        }
    }
}

impl ItemIdentifier {
    pub fn new(entity: &Entity<'_>, context: &Context<'_>) -> Self {
        let name = entity.get_name().expect("ItemIdentifier get name");
        Self::with_name(name, entity, context)
    }

    pub fn to_some(self) -> ItemIdentifier<Option<String>> {
        self.map_name(Some)
    }

    pub fn is_nserror(&self) -> bool {
        self.library_name() == "Foundation" && self.name == "NSError"
    }

    pub fn nserror() -> Self {
        Self {
            name: "NSError".into(),
            location: Location::new("Foundation.NSError"),
        }
    }

    pub fn block() -> Self {
        Self {
            name: "Block".into(),
            location: Location::new("block"),
        }
    }

    pub fn bitflags() -> Self {
        Self {
            name: "bitflags".into(),
            location: Location::new("bitflags"),
        }
    }

    pub fn objc2(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            location: Location::new("ObjectiveC"),
        }
    }

    pub fn main_thread_marker() -> Self {
        Self {
            name: "MainThreadMarker".into(),
            location: Location::new("ObjectiveC"),
        }
    }

    #[cfg(test)]
    pub fn dummy() -> Self {
        Self {
            name: "DUMMY".into(),
            location: Location::new("System"),
        }
    }

    pub fn is_nsstring(&self) -> bool {
        self.location.library_name() == "Foundation" && self.name == "NSString"
    }

    pub fn is_nscomparator(&self) -> bool {
        self.location.library_name() == "Foundation" && self.name == "NSComparator"
    }

    pub fn path(&self) -> impl fmt::Display + '_ {
        struct ItemIdentifierPath<'a>(&'a ItemIdentifier);

        impl fmt::Display for ItemIdentifierPath<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self.0.location.library_name() {
                    "bitflags" => write!(f, "bitflags::{}", self.0.name),
                    "block2" => write!(f, "block2::{}", self.0.name),
                    "libc" => write!(f, "libc::{}", self.0.name),
                    _ => write!(f, "{}", self.0.name),
                }
            }
        }

        ItemIdentifierPath(self)
    }

    pub fn path_in_relation_to<'a>(&'a self, other: &'a Location) -> impl fmt::Display + 'a {
        struct ItemIdentifierPathInRelationTo<'a>(&'a ItemIdentifier, &'a Location);

        impl fmt::Display for ItemIdentifierPathInRelationTo<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if self.1.file_name() == self.0.location.file_name() {
                    write!(f, "{}", self.0.name)
                } else {
                    write!(f, "{}", self.0.path())
                }
            }
        }

        ItemIdentifierPathInRelationTo(self, other)
    }

    /// Generate a markdown link to Apple's documentation.
    ///
    /// This is best effort only, and doesn't work for functions and methods,
    /// and possibly some renamed classes and traits. Additionally, the link
    /// may redirect.
    pub(crate) fn doc_link(&self) -> impl fmt::Display + '_ {
        FormatterFn(|f| {
            write!(
                f,
                "[Apple's documentation](https://developer.apple.com/documentation/{}/{}?language=objc)",
                self.library_name().to_lowercase(),
                self.name.to_lowercase()
            )
        })
    }
}

impl ItemIdentifier<Option<String>> {
    pub fn new_optional(entity: &Entity<'_>, context: &Context<'_>) -> Self {
        Self::with_name(entity.get_name(), entity, context)
    }
}

impl AsRef<Self> for Location {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<N> AsRef<Self> for ItemIdentifier<N> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<N> AsRef<Location> for ItemIdentifier<N> {
    fn as_ref(&self) -> &Location {
        &self.location
    }
}

/// Helper to emit a `#[cfg(feature = "...")]`-gate based on the required
/// items and the implied features.
///
/// Only the library of the emission location matters.
pub fn cfg_gate_ln<'a, R: AsRef<Location> + 'a, I: AsRef<Location> + 'a>(
    required_features: impl IntoIterator<Item = R> + 'a,
    implied_features: impl IntoIterator<Item = I> + 'a,
    config: &'a Config,
    emission_location: &'a Location,
) -> impl fmt::Display + 'a {
    let emission_library = emission_location.library_name();
    // Use a set to deduplicate features, and to have them in
    // a consistent order.
    let mut feature_names = BTreeSet::new();
    let mut platform_cfg = PlatformCfg::from_config(config.library(emission_library));

    for location in required_features {
        let location: &Location = location.as_ref();
        if let Some(feature_name) = location.library(config, emission_library).feature() {
            feature_names.insert(feature_name);
        }

        platform_cfg.dependency(config.library(location.library_name()));
    }

    for location in implied_features {
        let location: &Location = location.as_ref();
        if let Some(feature_name) = location.library(config, emission_library).feature() {
            feature_names.remove(&feature_name);
        }

        platform_cfg.implied(config.library(location.library_name()));
    }

    FormatterFn(move |f| {
        match feature_names.len() {
            0 => {}
            1 => {
                let feature = feature_names.first().unwrap();
                writeln!(f, "#[cfg(feature = \"{feature}\")]")?;
            }
            _ => {
                write!(f, "#[cfg(all(")?;

                for (i, feature) in feature_names.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "feature = \"{feature}\"")?;
                }

                write!(f, "))]")?;
                writeln!(f)?;
            }
        }

        if let Some(cfg) = platform_cfg.cfgs() {
            writeln!(f, "#[cfg({cfg})]")?;
        }

        Ok(())
    })
}

impl FromStr for Location {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("::") {
            return Err(Box::new(std::io::Error::other("requires ., not ::")));
        }

        Ok(Location {
            module_path: s.into(),
        })
    }
}

impl<'de> de::Deserialize<'de> for Location {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct LocationVisitor;

        impl de::Visitor<'_> for LocationVisitor {
            type Value = Location;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("location")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Location::from_str(value).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(LocationVisitor)
    }
}

impl FromStr for ItemIdentifier {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("::") {
            return Err(Box::new(std::io::Error::other("requires ., not ::")));
        }

        let (module_path, name) = s
            .rsplit_once('.')
            .ok_or_else(|| std::io::Error::other("requires at least one ."))?;

        Ok(Self {
            name: name.into(),
            location: Location {
                module_path: module_path.into(),
            },
        })
    }
}

impl<'de> de::Deserialize<'de> for ItemIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct ItemIdentifierVisitor;

        impl de::Visitor<'_> for ItemIdentifierVisitor {
            type Value = ItemIdentifier;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("item identifier")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                ItemIdentifier::from_str(value).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(ItemIdentifierVisitor)
    }
}
