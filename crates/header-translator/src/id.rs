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
    //
    // Special modules:
    // __bitflags__
    // __builtin__
    // __core__
    // __libc__
    module_path: Box<str>,
}

impl Location {
    fn new(module_path: impl Into<Box<str>>) -> Self {
        let module_path = module_path.into();
        let module_path = match &*module_path {
            // Normalize Objective-C (remove submodules)
            name if name.starts_with("ObjectiveC") => "ObjectiveC".into(),

            // These types are redefined in the framework crate itself.
            "Darwin.MacTypes" => "__builtin__".into(),

            // int8_t, int16_t etc., translated to i8, i16 etc.
            "_stdint" => "__builtin__".into(),
            // Implementation of the above
            "DarwinFoundation.types.machine_types" => "__builtin__".into(),

            // `core::ffi` types
            "_Builtin_stdarg.va_list" => {
                warn!("va_list is not yet supported");
                "__core__.ffi".into()
            }
            // c_float and c_double
            "_float" | "_Builtin_float" => "__core__.ffi".into(),

            // `libc`
            name if name.starts_with("sys_types") => "__libc__".into(),
            "DarwinFoundation.types.sys_types" => "__libc__".into(),
            "DarwinFoundation.qos" => "__libc__".into(),
            name if name.starts_with("Darwin.POSIX") => "__libc__".into(),
            "_stdio" => "__libc__".into(),
            "_time.timespec" => "__libc__".into(),

            // Will be moved to the `mach2` crate in `libc` v1.0
            name if name.starts_with("Darwin.Mach") => "__libc__".into(),
            "_mach_port_t" => "__libc__".into(),

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
            // If it doesn't have a module, and doesn't come from a framework,
            // then it is probably a built-in macro from stddef.h, stdarg.h or
            // likewise.
            return Self::new("__builtin__");
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

    pub fn crate_dependency<'config>(
        &self,
        config: &'config Config,
        emission_library: &str,
    ) -> Option<&'config str> {
        match self.library_name() {
            "__builtin__" | "__core__" => None,
            library if library == emission_library => None,
            library => Some(&config.library(library).krate),
        }
    }

    // Feature names are based on the file name, not the whole path to the feature.
    pub fn cargo_toml_feature(&self, config: &Config, emission_library: &str) -> Option<String> {
        match self.library_name() {
            "__builtin__" | "__core__" => None,
            "__bitflags__" => {
                let required = config
                    .library(emission_library)
                    .required_crates
                    .contains("bitflags");

                // We want the bitflags feature to be enabled automatically
                // when a file with bitflags are imported.
                if !required {
                    Some("bitflags".into())
                } else {
                    None
                }
            }
            // Don't emit dependency for local features (we want files to be
            // independently activated).
            library if library == emission_library => None,
            // Matches e.g. objc2-foundation/NSArray, but not objc2 or
            // libc (since that is configured in the source itself).
            library => {
                let krate = &config.library(library).krate;
                let required = config
                    .library(emission_library)
                    .required_crates
                    .contains(krate);

                self.file_name().map(|file_name| {
                    format!(
                        "{krate}{}/{}",
                        if required { "" } else { "?" },
                        clean_name(file_name),
                    )
                })
            }
        }
    }

    pub fn cargo_toml_feature_on_top_level(&self, emission_library: &str) -> Option<String> {
        match self.library_name() {
            "__builtin__" | "__core__" => None,
            library if library == emission_library => None,
            _ => self.file_name().map(clean_name),
        }
    }

    // FIXME: This is currently wrong for nested umbrella frameworks
    // (specifically MetalPerformanceShaders).
    pub fn cfg_feature<'a>(
        &self,
        config: &'a Config,
        emission_library: &str,
    ) -> Option<Cow<'a, str>> {
        match self.library_name() {
            "__builtin__" | "__core__" => None,
            library if library == emission_library => {
                self.file_name().map(clean_name).map(Cow::Owned)
            }
            library => {
                let krate = &config.library(library).krate;
                let required = config
                    .library(emission_library)
                    .required_crates
                    .contains(krate);
                if !required {
                    Some(Cow::Borrowed(krate))
                } else {
                    None
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
            .file;

        let mut location = if let Some(file) = file {
            Location::from_file(file)
        } else {
            // Assume item to be a built-in macro like __nonnull if no file.
            Location::new("__builtin__")
        };

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

    pub fn copyhelper(mutable: bool) -> Self {
        let name = if mutable {
            "MutableCopyingHelper"
        } else {
            "CopyingHelper"
        };
        Self {
            name: name.into(),
            location: Location::new("Foundation.NSObject"),
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
            location: Location::new("__bitflags__"),
        }
    }

    pub fn objc(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            location: Location::new("ObjectiveC"),
        }
    }

    pub fn cf(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            location: Location::new("CoreFoundation"),
        }
    }

    pub fn core_ffi(name: &str) -> Self {
        Self {
            name: name.into(),
            location: Location::new("__core__.ffi"),
        }
    }

    pub fn core_ptr_nonnull() -> Self {
        Self {
            name: "NonNull".into(),
            location: Location::new("__core__.ptr"),
        }
    }

    pub fn unsafecell() -> Self {
        Self {
            name: "UnsafeCell".into(),
            location: Location::new("__core__.cell"),
        }
    }

    pub fn phantoms() -> Self {
        Self {
            name: "__phantoms__".into(),
            location: Location::new("__core__.marker"),
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
            location: Location::new("__builtin__"),
        }
    }

    pub fn is_nsstring(&self) -> bool {
        self.location.library_name() == "Foundation" && self.name == "NSString"
    }

    pub fn is_nscomparator(&self) -> bool {
        self.location.library_name() == "Foundation" && self.name == "NSComparator"
    }

    /// The import needed for a given item to exist.
    pub fn import(&self, config: &Config, emission_library: &str) -> Option<Cow<'static, str>> {
        match self.library_name() {
            "__builtin__" => None,
            "__core__" => match &*self.location().module_path {
                "__core__.ffi" => Some("core::ffi::*".into()),
                // HACKs
                "__core__.ptr" if self.name == "NonNull" => Some("core::ptr::NonNull".into()),
                "__core__.cell" if self.name == "UnsafeCell" => {
                    Some("core::cell::UnsafeCell".into())
                }
                "__core__.marker" => Some("core::marker::{PhantomData, PhantomPinned}".into()),
                _ => {
                    error!("unknown __core__: {self:?}");
                    None
                }
            },
            // Rare enough that it's written directly instead of
            // glob-imported, see `ItemIdentifier::path` below.
            "__bitflags__" | "__libc__" | "block" => None,
            "ObjectiveC" => Some("objc2::__framework_prelude::*".into()),
            // Not currently needed, but might be useful to emit
            // `Some("crate")` here in the future.
            library if library == emission_library => None,
            library => {
                let krate = &config.library(library).krate;
                Some(format!("{}::*", krate.replace('-', "_")).into())
            }
        }
    }

    pub fn path(&self) -> impl fmt::Display + '_ {
        struct ItemIdentifierPath<'a>(&'a ItemIdentifier);

        impl fmt::Display for ItemIdentifierPath<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self.0.location.library_name() {
                    "__bitflags__" => write!(f, "bitflags::{}", self.0.name),
                    "__libc__" => write!(f, "libc::{}", self.0.name),
                    "block" => write!(f, "block2::{}", self.0.name),
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
        if let Some(feature_name) = location.cfg_feature(config, emission_library) {
            feature_names.insert(feature_name);
        }

        platform_cfg.dependency(config.library(location.library_name()));
    }

    for location in implied_features {
        let location: &Location = location.as_ref();
        if let Some(feature_name) = location.cfg_feature(config, emission_library) {
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
