use clang::EntityKind;
use core::fmt;
use core::hash;
use serde::de;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::error::Error;
use std::iter;
use std::str::FromStr;

use clang::source::File;
use clang::Entity;

use crate::cfgs::cfg_features_ln;
use crate::cfgs::PlatformCfg;
use crate::context::Context;
use crate::display_helper::FormatterFn;
use crate::module::clean_name;
use crate::Config;

pub trait ToOptionString: fmt::Debug {
    fn set(&mut self, name: Option<String>);
    fn to_option(&self) -> Option<&str>;
}

impl ToOptionString for String {
    fn set(&mut self, name: Option<String>) {
        *self = name.unwrap();
    }

    fn to_option(&self) -> Option<&str> {
        Some(self)
    }
}

impl ToOptionString for Option<String> {
    fn set(&mut self, name: Option<String>) {
        *self = name;
    }

    fn to_option(&self) -> Option<&str> {
        self.as_deref()
    }
}

impl ToOptionString for () {
    fn set(&mut self, name: Option<String>) {
        assert_eq!(name, None);
    }

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

        // We don't care about the difference between the different
        // DarwinFoundation modules (for now at least).
        if let Some(rest) = module_path.strip_prefix("DarwinFoundation.") {
            return Self::new(rest);
        }
        if let Some(rest) = module_path.strip_prefix("_DarwinFoundation1.") {
            return Self::new(rest);
        }
        if let Some(rest) = module_path.strip_prefix("_DarwinFoundation2.") {
            return Self::new(rest);
        }
        if let Some(rest) = module_path.strip_prefix("_DarwinFoundation3.") {
            return Self::new(rest);
        }

        let module_path = match &*module_path {
            // Remove submodules for Objective-C.
            name if name.starts_with("ObjectiveC") => "ObjectiveC".into(),

            // Remove "Darwin" prefix for Darwin.block.
            "Darwin.block" => "block".into(),

            // TODO: Handle OS?
            "os_object" => "__builtin__".into(),
            name if name.starts_with("Darwin.os.") => "__builtin__".into(),
            "os_workgroup.workgroup" => "__builtin__".into(),

            // Various macros
            name if name.starts_with("os_availability") => "__builtin__".into(),
            name if name.starts_with("_AvailabilityInternal") => "__builtin__".into(),
            name if name.starts_with("availability") => "__builtin__".into(),
            "cdefs" => "__builtin__".into(),
            "Darwin.libkern.OSByteOrder" => "__builtin__".into(),
            "TargetConditionals" => "__builtin__".into(),
            "Darwin.AssertMacros" => "__builtin__".into(),
            "Darwin.ConditionalMacros" => "__builtin__".into(),
            "AvailabilityMacros" => "__builtin__".into(),
            "ExtensionFoundation.EXMacros" => "__builtin__".into(),
            name if name.starts_with("_assert") => "__builtin__".into(),

            // These types are redefined in the framework crate itself.
            "Darwin.MacTypes" => "__builtin__".into(),
            "Darwin.device" => "__builtin__".into(),
            "uuid.uuid_t" => "__builtin__".into(),
            "libkern.OSTypes" => "__builtin__".into(),
            "Darwin.net.if_media" => "__builtin__".into(),
            "XPC" => "__builtin__".into(),

            // We don't emit the `hfs`, so let's act as-if CoreServices is the
            // one that defines the types in there (such as HFSUniStr255).
            name if name.starts_with("Darwin.hfs") => "CoreServices.Files".into(),

            // int8_t, int16_t etc., translated to i8, i16 etc.
            "_Builtin_stdint" | "_stdint" => "__builtin__".into(),
            name if name.starts_with("_Builtin_stddef") => "__builtin__".into(),
            // Implementation of the above
            name if name.starts_with("types.machine_types") => "__builtin__".into(),
            // UINT_MAX, FLT_MIN, DBL_MAX, etc.
            // Handled manually in `expr.rs`.
            "_Builtin_limits" => "__builtin__".into(),
            // C99 bool
            "_Builtin_stdbool" => "__builtin__".into(),
            // float_t and double_t
            "_math" => "__builtin__".into(),

            // `core::ffi` types
            name if name.starts_with("_Builtin_stdarg") => {
                warn!("va_list is not yet supported");
                "__core__.ffi".into()
            }
            // c_float and c_double
            "_float" | "_Builtin_float" => "__core__.ffi".into(),

            // Unstable in FFI.
            name if name.starts_with("simd") => "__core__.simd".into(),

            // `libc`
            name if name.starts_with("sys_types") => "__libc__".into(),
            name if name.starts_with("Darwin.POSIX") => "__libc__".into(),
            name if name.starts_with("_signal") => "__libc__".into(),
            "types.sys_types" => "__libc__".into(),
            "qos" => "__libc__".into(),
            "_stdio" => "__libc__".into(),
            "_time.timespec" => "__libc__".into(),
            "_fenv" => "__libc__".into(),
            "Darwin.sys.acl" => "__libc__".into(),
            "Darwin.sys.event" => "__libc__".into(),
            "_ctype" => "__libc__".into(),
            "_errno" => "__libc__".into(),
            "_locale.locale" => "__libc__".into(),
            "_setjmp" => "__libc__".into(),
            "_stdlib" => "__libc__".into(),
            "_string" => "__libc__".into(),
            "_time" => "__libc__".into(),
            "ptrauth" => "__libc__".into(),
            "Darwin.uuid" => "__libc__".into(),
            "unistd" => "__libc__".into(),
            "Darwin.malloc" => "__libc__".into(),
            "_stdlib.malloc.malloc_type" => "__libc__".into(),

            // Will be moved to the `mach2` crate in `libc` v1.0
            name if name.starts_with("Darwin.Mach") => "__libc__".into(),
            "mach.port.mach_port_t" => "__libc__".into(),
            "mach.mach_port_t" => "__libc__".into(),
            "_mach_port_t" => "__libc__".into(),

            // Rename "ObjC" modules to `objc2`, such that the feature flag matches.
            "IOSurface.ObjC" => "IOSurface.objc2".into(),
            name if name.starts_with("IOBluetooth.objc") => {
                name.replace("IOBluetooth.objc", "IOBluetooth.objc2").into()
            }
            name if name.starts_with("IOBluetoothUI.objc") => name
                .replace("IOBluetoothUI.objc", "IOBluetoothUI.objc2")
                .into(),

            // UIUtilities "subframework". It doesn't seem to be intentionally
            // exposed. It's small enough that we'll just inline it into
            // UIKit for now (which is where it was extracted from anyhow).
            "UIUtilities.UIGeometry" => "UIKit.UIGeometry".into(),
            "UIUtilities.UICoordinateSpace" => "UIKit.UIView".into(),
            "UIUtilities.UIDefines" => "UIKit.UIKitDefines".into(),

            // Similarly, _LocationEssentials was extracted from CoreLocation.
            "_LocationEssentials" => "CoreLocation".into(),
            "_LocationEssentials.CLEssentionsAvailability" => "CoreLocation.CLAvailability".into(),
            "_LocationEssentials.CLLocationEssentials" => "CoreLocation.CLLocation".into(),

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

    pub fn from_library(library_name: &str) -> Self {
        Self {
            module_path: library_name.into(),
        }
    }

    pub fn add_module(&self, module_name: &str) -> Self {
        Self {
            module_path: format!("{}.{}", self.module_path, module_name).into(),
        }
    }

    /// Inside umbrella header / top-level module.
    pub fn is_top_level(&self) -> bool {
        !self.module_path.contains('.')
    }

    pub fn library_name(&self) -> &str {
        if let Some((library, _rest)) = self.module_path.split_once('.') {
            library
        } else {
            // Top-level
            &self.module_path
        }
    }

    /// Feature names are based on the file name, not the whole path to the feature.
    pub fn feature_names(&self) -> impl Iterator<Item = String> + Clone + '_ {
        // NOTE: We _could_ choose to prefix each module name here (e.g.
        // "$module_$submodule"), but that'd be pretty redundant, as module
        // names are usually prefixed internally (e.g.
        // `MetalPerformanceShaders.MPSImage.MPSImageConversion` or
        // `OpenDirectory.CFOpenDirectory.CFODContext`).
        //
        // This does lead to some feature overlap, like
        // `MetalPerformanceShaders.MPSCore.MPSImage` and
        // `MetalPerformanceShaders.MPSImage`, but that's probably desirable
        // anyhow.
        //
        // NOTE: We have no way of knowing whether a module is considered
        // private or not by the modulemap, so for now we just always assume
        // it's public.
        self.modules().map(clean_name)
    }

    pub fn modules(&self) -> impl Iterator<Item = &'_ str> + Clone + '_ {
        self.module_path.split('.').skip(1)
    }

    pub fn assert_file(&self, file_name: &str) {
        if self.modules().last() != Some(file_name) {
            warn!(?self, ?file_name, "expected to be in file");
        }
    }

    /// Whether the location is textually similar to another.
    ///
    /// Example would be:
    /// "CoreFoundation.CFURLAccess".semi_part_of("CoreFoundation.CFURL")
    pub fn semi_part_of(&self, other: &Location) -> bool {
        self.module_path.starts_with(&*other.module_path)
    }
}

/// Names in C and Objective-C are global, so this is always enough to
/// uniquely identify an item.
///
/// Often, though, we want to know the library, file name and general location
/// an item came from as well.
#[derive(Debug, Clone)]
pub struct ItemIdentifier<N = String> {
    /// The name of the item in Rust (i.e. it may have been renamed).
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
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
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

    /// Construct a new `ItemIdentifier` from the given entity and C name.
    ///
    /// The C name will be renamed according to the configuration.
    pub fn with_name(mut name: N, entity: &Entity<'_>, context: &Context<'_>) -> Self {
        let file = entity
            .get_location()
            .and_then(|loc| loc.get_expansion_location().file);

        let mut location = if let Some(file) = file {
            Location::from_file(file)
        } else {
            // Assume item to be a built-in macro like __nonnull if no file.
            Location::new("__builtin__")
        };

        // Remove unnecessary CFBase module, a lot of trait impls aren't
        // available without it, which can be quite confusing.
        if location == Location::new("CoreFoundation.CFBase") {
            location = Location::new("CoreFoundation");
        }

        // Replace module from external data if it exists, such that all
        // subsequent usage of the location, including in other configuration
        // lookups, is done in the external library.
        if let Some(name) = name.to_option() {
            // TODO: Lookup only in current library? Or always there?
            if let Some(external) = context.library(&location).external.get(name) {
                location = external.module.clone();
            } else if let EntityKind::ObjCClassRef | EntityKind::ObjCProtocolRef = entity.get_kind()
            {
                error!(?entity, "could not get declaration, add appropriate external.{name}.module = \"...\" to translation-config.toml");
            }
        }

        // Rename if the config contains a rename.
        if let Some(renamed) = context.library(&location).get(entity).renamed.clone() {
            name.set(Some(renamed));
        }

        Self { name, location }
    }

    #[track_caller]
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
    #[track_caller]
    pub fn new(entity: &Entity<'_>, context: &Context<'_>) -> Self {
        let name = entity.get_name().expect("ItemIdentifier must have name");
        ItemIdentifier::with_name(name, entity, context)
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

    pub fn builtin(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            location: Location::new("__builtin__"),
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

    pub fn cf_string() -> Self {
        Self {
            name: "CFString".into(),
            location: Location::new("CoreFoundation.CFString"),
        }
    }

    pub fn ns_string() -> Self {
        Self {
            name: "NSString".into(),
            location: Location::new("Foundation.NSString"),
        }
    }

    pub fn cf_uuid() -> Self {
        Self {
            name: "CFUUID".into(),
            location: Location::new("CoreFoundation.CFUUID"),
        }
    }

    pub fn dummy(n: usize) -> Self {
        Self {
            name: format!("DUMMY{n}"),
            location: Location::new("__dummy__"),
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
                    "__bitflags__" => write!(f, "bitflags::{}", self.0.name),
                    "__libc__" => write!(f, "libc::{}", self.0.name),
                    "block" => write!(f, "block2::Dyn{}", self.0.name),
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
                if self.0.location == *self.1 {
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
        let name = if !entity.is_anonymous() {
            entity.get_name()
        } else {
            None
        };
        Self::with_name(name, entity, context)
    }

    pub fn to_option(self) -> Option<ItemIdentifier> {
        if let Some(name) = self.name {
            Some(ItemIdentifier::from_raw(name, self.location))
        } else {
            None
        }
    }

    #[track_caller]
    pub fn require_name(self) -> ItemIdentifier {
        self.to_option().expect("item must name a name")
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
/// The emission location is also considered an "implied" item.
pub fn cfg_gate_ln<'a, R: AsRef<ItemTree> + 'a, I: AsRef<ItemTree> + 'a>(
    required: impl IntoIterator<Item = R> + 'a,
    implied: impl IntoIterator<Item = I> + 'a,
    config: &'a Config,
    emission_location: &'a Location,
) -> impl fmt::Display + 'a {
    // Use a set to deduplicate features, and to have them in
    // a consistent order.
    let mut feature_names = BTreeSet::new();
    let mut platform_cfg = PlatformCfg::from_config(config.library(emission_location));

    for item in required {
        let item: &ItemTree = item.as_ref();
        feature_names.extend(item.cfg_features(config, emission_location));

        item.visit(emission_location, |id, _| {
            platform_cfg.dependency(config.library(id));
        });
    }

    for item in implied {
        let item: &ItemTree = item.as_ref();
        for feature_name in item.cfg_features(config, emission_location) {
            feature_names.remove(&feature_name);
        }

        item.visit(emission_location, |id, _| {
            platform_cfg.implied(config.library(id));
        });
    }

    // Treat emission location as implied
    for feature_name in ItemTree::cfg_features_inner(emission_location, config, emission_location) {
        feature_names.remove(&feature_name);
    }
    platform_cfg.implied(config.library(emission_location));

    FormatterFn(move |f| {
        write!(f, "{}", cfg_features_ln(&feature_names))?;

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

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.module_path)
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

impl fmt::Display for ItemIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.location, self.name)
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

/// Items relative to their "super" item, forming a DAG, potentially spanning
/// many different libraries/frameworks.
///
/// Allows us to emit e.g.:
/// XYZ = ["objc2-foundation/NSGeometry", "objc2-foundation/objc2-core-foundation"]
///
/// Instead of:
/// XYZ = ["objc2-foundation/NSGeometry", "objc2-core-foundation"]
#[derive(Debug, Clone)]
pub struct ItemTree {
    id: ItemIdentifier,
    children: BTreeSet<ItemTree>,
}

impl PartialEq for ItemTree {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for ItemTree {}

impl hash::Hash for ItemTree {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialOrd for ItemTree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ItemTree {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl ItemTree {
    pub fn new(id: ItemIdentifier, children: impl IntoIterator<Item = ItemTree>) -> Self {
        Self {
            id,
            children: children.into_iter().collect(),
        }
    }

    // Use of this might be a sign that we need more information to emit correctly.
    pub fn from_id(id: ItemIdentifier) -> Self {
        Self {
            id,
            children: BTreeSet::new(),
        }
    }

    pub fn id(&self) -> &ItemIdentifier {
        &self.id
    }

    fn visit_inner(&self, libraries: &[&str], f: &mut impl FnMut(&ItemIdentifier, &[&str])) {
        let current = *libraries.last().expect("at least one location");
        if matches!(self.id.location.library_name(), "__builtin__" | "__core__") {
            // TODO: Should we visit here?
        } else if current == self.id.location.library_name() {
            f(&self.id, &libraries[1..libraries.len()]);
            for item in &self.children {
                item.visit_inner(libraries, f);
            }
        } else {
            let libraries: Vec<&str> = libraries
                .iter()
                .copied()
                .chain(iter::once(self.id.location.library_name()))
                .collect();
            f(&self.id, &libraries[1..libraries.len()]);
            for item in &self.children {
                item.visit_inner(&libraries, f);
            }
        }
    }

    fn visit(&self, emission_location: &Location, mut f: impl FnMut(&ItemIdentifier, &[&str])) {
        self.visit_inner(&[emission_location.library_name()], &mut f);
    }

    pub fn nserror() -> Self {
        Self::from_id(ItemIdentifier::nserror())
    }

    pub fn block() -> Self {
        Self::from_id(ItemIdentifier {
            name: "Block".into(),
            location: Location::new("block"),
        })
    }

    pub fn bitflags() -> Self {
        Self::from_id(ItemIdentifier {
            name: "bitflags".into(),
            location: Location::new("__bitflags__"),
        })
    }

    pub fn objc(name: impl Into<String>) -> Self {
        Self::from_id(ItemIdentifier {
            name: name.into(),
            location: Location::new("ObjectiveC"),
        })
    }

    pub fn cf(name: impl Into<String>) -> Self {
        Self::from_id(ItemIdentifier {
            name: name.into(),
            location: Location::new("CoreFoundation"),
        })
    }

    pub fn dispatch(name: impl Into<String>) -> Self {
        Self::from_id(ItemIdentifier {
            name: name.into(),
            location: Location::new("Dispatch"),
        })
    }

    pub fn cf_string_macro() -> Self {
        Self::from_id(ItemIdentifier {
            name: "cf_string".into(),
            location: Location::new("CoreFoundation.CFString"),
        })
    }

    pub fn ns_string_macro() -> Self {
        Self::from_id(ItemIdentifier {
            name: "ns_string".into(),
            location: Location::new("Foundation.NSString"),
        })
    }

    pub fn core_ffi(name: &str) -> Self {
        Self::from_id(ItemIdentifier {
            name: name.into(),
            location: Location::new("__core__.ffi"),
        })
    }

    pub fn core_ptr_nonnull() -> Self {
        Self::from_id(ItemIdentifier {
            name: "NonNull".into(),
            location: Location::new("__core__.ptr"),
        })
    }

    pub fn core_simd_simd() -> Self {
        Self::from_id(ItemIdentifier {
            name: "Simd".into(),
            location: Location::new("__core__.simd"),
        })
    }

    pub fn unsafecell() -> Self {
        Self::from_id(ItemIdentifier {
            name: "UnsafeCell".into(),
            location: Location::new("__core__.cell"),
        })
    }

    pub fn phantoms() -> Self {
        Self::from_id(ItemIdentifier {
            name: "__phantoms__".into(),
            location: Location::new("__core__.marker"),
        })
    }

    pub fn main_thread_marker() -> Self {
        Self::from_id(ItemIdentifier {
            name: "MainThreadMarker".into(),
            location: Location::new("ObjectiveC"),
        })
    }

    /// The crates used directly by this item.
    pub fn used_crates<'config>(
        &self,
        config: &'config Config,
        emission_location: &Location,
    ) -> impl Iterator<Item = &'config str> {
        match self.id.library_name() {
            "__builtin__" | "__core__" => None,
            library if library == emission_location.library_name() => None,
            library => config.try_library(library).map(|data| &*data.krate),
        }
        .into_iter()
    }

    /// Required Cargo.toml features in `[dependencies]` table.
    ///
    /// Returns (crate, required_feature) pairs.
    pub fn required_crate_features<'config>(
        &self,
        config: &'config Config,
        emission_location: &Location,
    ) -> impl Iterator<Item = (&'config str, String)> {
        let mut crate_features = vec![];
        self.visit(emission_location, |id, libraries| match libraries {
            [] => {}
            [library] => {
                let Some(data) = config.try_library(library) else {
                    error!(library, "unknown library");
                    return;
                };

                let krate = &*data.krate;
                let required = config
                    .library(emission_location)
                    .required_crates
                    .contains(krate);

                if !required || emission_location.is_top_level() {
                    crate_features.extend(
                        id.location
                            .feature_names()
                            .map(|feature_name| (krate, feature_name)),
                    );
                }
            }
            [library, dependent_library] => {
                let Some(data) = config.try_library(library) else {
                    error!(library, dependent_library, "unknown library");
                    return;
                };
                let krate = &*data.krate;
                let required = config
                    .library(emission_location)
                    .required_crates
                    .contains(krate);

                let Some(dependent_data) = config.try_library(dependent_library) else {
                    error!(library, dependent_library, "unknown dependent library");
                    return;
                };
                let dependent_krate = &*dependent_data.krate;
                let dependent_required = data.required_crates.contains(dependent_krate);

                if (!required || emission_location.is_top_level()) && !dependent_required {
                    crate_features.push((krate, dependent_krate.to_string()));
                }
            }
            _ => {
                // Assume that the library handles this itself internally.
                trace!(
                    ?libraries,
                    ?self,
                    "nested required_crate_features enablement"
                );
            }
        });
        crate_features.into_iter()
    }

    /// The Cargo.toml features that will be enabled in `[features]` table.
    ///
    /// Returns (feature, enabled_feature) pairs.
    pub fn enabled_features(
        &self,
        config: &Config,
        emission_location: &Location,
    ) -> impl Iterator<Item = (String, String)> {
        let mut enabled_features = vec![];
        self.visit(emission_location, |id, libraries| match libraries {
            // Don't emit dependency for local features (we want files to be
            // independently activated).
            [] => {}
            ["__bitflags__"] => {
                let required = config
                    .library(emission_location)
                    .required_crates
                    .contains("bitflags");

                // We want the bitflags feature to be enabled automatically
                // when a file with bitflags are imported.
                if !required {
                    if let Some(feature_name) = emission_location.feature_names().last() {
                        enabled_features.push((feature_name, "bitflags".into()));
                    }
                }
            }
            // Matches e.g. objc2-foundation/NSArray, but not objc2 or libc
            // (since that is configured in the source itself).
            [library] => {
                if let Some(data) = config.try_library(library) {
                    let krate = &*data.krate;

                    let required = config
                        .library(emission_location)
                        .required_crates
                        .contains(krate);

                    // We don't use optional `crate?/feature`, since that
                    // currently works poorly in Cargo, and hence users would
                    // end up downloading more crates than they actually use:
                    // https://github.com/rust-lang/cargo/issues/10801
                    //
                    // Instead, we prefer to compile more of the dependent
                    // crate. This is unfortunate, but probably the best we
                    // can do atm.
                    if required {
                        if let Some(emission_feature_name) =
                            emission_location.feature_names().last()
                        {
                            enabled_features.extend(id.location.feature_names().map(
                                |feature_name| {
                                    (
                                        emission_feature_name.clone(),
                                        format!("{krate}/{feature_name}"),
                                    )
                                },
                            ));
                        }
                    }
                }
            }
            [library, dependent_library] => {
                let Some(data) = config.try_library(library) else {
                    error!(library, dependent_library, "unknown library");
                    return;
                };
                let krate = &*data.krate;
                let required = config
                    .library(emission_location)
                    .required_crates
                    .contains(krate);

                let Some(dependent_data) = config.try_library(dependent_library) else {
                    error!(library, dependent_library, "unknown dependent library");
                    return;
                };
                let dependent_krate = &*dependent_data.krate;
                let dependent_required = data.required_crates.contains(dependent_krate);

                if required && !dependent_required {
                    if let Some(emission_feature_name) = emission_location.feature_names().last() {
                        enabled_features
                            .push((emission_feature_name, format!("{krate}/{dependent_krate}")));
                    }
                }
            }
            _ => {
                // Assume that the library handles this itself internally.
                trace!(?libraries, ?self, "nested enabled_features enablement");
            }
        });
        enabled_features.into_iter()
    }

    fn cfg_features_inner<'a>(
        location: &Location,
        config: &'a Config,
        emission_location: &Location,
    ) -> impl Iterator<Item = Cow<'a, str>> {
        match location.library_name() {
            "__builtin__" | "__core__" => vec![],
            library if library == emission_location.library_name() => {
                location.feature_names().map(Cow::Owned).collect()
            }
            _ => {
                let krate = &*config.library(location).krate;
                let required = config
                    .library(emission_location)
                    .required_crates
                    .contains(krate);
                if !required
                    && !emission_location
                        .feature_names()
                        .any(|feature| feature == krate)
                {
                    vec![Cow::Borrowed(krate)]
                } else {
                    vec![]
                }
            }
        }
        .into_iter()
    }

    pub fn cfg_features<'config>(
        &self,
        config: &'config Config,
        emission_location: &Location,
    ) -> impl Iterator<Item = Cow<'config, str>> {
        let mut feature_names = vec![];
        self.visit(emission_location, |id, libraries| match libraries {
            [] => {
                feature_names.extend(Self::cfg_features_inner(
                    &id.location,
                    config,
                    emission_location,
                ));
            }
            [_] => {
                feature_names.extend(Self::cfg_features_inner(
                    &id.location,
                    config,
                    emission_location,
                ));
            }
            _ => {
                // Assume that the library handles this itself internally.
                trace!(?libraries, ?self, "nested cfg_features enablement");
            }
        });
        feature_names.into_iter()
    }

    /// The import data needed for a given item to exist.
    ///
    /// This is intentionally an `Option`, and doesn't inspect the children,
    /// because that's not required when determining imports.
    pub fn imports<'config>(
        &self,
        config: &'config Config,
        emission_location: &Location,
    ) -> Option<(Cow<'static, str>, impl fmt::Display + 'config)> {
        let import = match self.id.library_name() {
            "__builtin__" => None,
            "__core__" => match &*self.id.location().module_path {
                "__core__.ffi" => Some("core::ffi::*".into()),
                // HACKs
                "__core__.ptr" if self.id.name == "NonNull" => Some("core::ptr::NonNull".into()),
                "__core__.simd" if self.id.name == "Simd" => Some("core::simd::*".into()),
                "__core__.cell" if self.id.name == "UnsafeCell" => {
                    Some("core::cell::UnsafeCell".into())
                }
                "__core__.marker" => Some("core::marker::{PhantomData, PhantomPinned}".into()),
                _ => {
                    error!("unknown __core__: {self:?}");
                    None
                }
            },
            // Rare enough that it's written directly instead of
            // glob-imported, see `ItemIdentifier::path`.
            "__bitflags__" | "__libc__" | "block" => None,
            "ObjectiveC" => Some("objc2::__framework_prelude::*".into()),
            // Not currently needed, but might be useful to emit
            // `Some("crate")` here in the future.
            library if library == emission_location.library_name() => None,
            _ => {
                let krate = &config.library(&self.id).krate;
                Some(format!("{}::*", krate.replace('-', "_")).into())
            }
        };
        import.map(|import: Cow<'static, str>| {
            let feature_names: Vec<_> =
                Self::cfg_features_inner(&self.id.location, config, emission_location).collect();
            let mut platform_cfg = PlatformCfg::from_config(config.library(emission_location));
            platform_cfg.dependency(config.library(&self.id));
            (
                import,
                FormatterFn(move |f| {
                    write!(f, "{}", cfg_features_ln(&feature_names))?;
                    if let Some(cfg) = platform_cfg.cfgs() {
                        writeln!(f, "#[cfg({cfg})]")?;
                    }
                    Ok(())
                }),
            )
        })
    }
}

impl AsRef<Self> for ItemTree {
    fn as_ref(&self) -> &Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visit() {
        let tree = ItemTree::new(
            ItemIdentifier::from_str("foo.a").unwrap(),
            [
                ItemTree::new(
                    ItemIdentifier::from_str("foo.b").unwrap(),
                    [ItemTree::new(
                        ItemIdentifier::from_str("bar.x").unwrap(),
                        [],
                    )],
                ),
                ItemTree::new(
                    ItemIdentifier::from_str("foo.c").unwrap(),
                    [ItemTree::new(
                        ItemIdentifier::from_str("foo.d").unwrap(),
                        [
                            ItemTree::new(
                                ItemIdentifier::from_str("bar.y").unwrap(),
                                [
                                    ItemTree::new(
                                        ItemIdentifier::from_str("foobar.dy").unwrap(),
                                        [],
                                    ),
                                    ItemTree::new(ItemIdentifier::from_str("bar.z").unwrap(), []),
                                ],
                            ),
                            ItemTree::new(
                                ItemIdentifier::from_str("bar.w").unwrap(),
                                [ItemTree::new(
                                    ItemIdentifier::from_str("__builtin__.xyz").unwrap(),
                                    [],
                                )],
                            ),
                        ],
                    )],
                ),
            ],
        );
        let mut expected = [
            (ItemIdentifier::from_str("foo.a").unwrap(), vec![]),
            (ItemIdentifier::from_str("foo.b").unwrap(), vec![]),
            (
                ItemIdentifier::from_str("bar.x").unwrap(),
                vec!["bar".to_string()],
            ),
            (ItemIdentifier::from_str("foo.c").unwrap(), vec![]),
            (ItemIdentifier::from_str("foo.d").unwrap(), vec![]),
            (
                ItemIdentifier::from_str("bar.y").unwrap(),
                vec!["bar".to_string()],
            ),
            (
                ItemIdentifier::from_str("foobar.dy").unwrap(),
                vec!["bar".to_string(), "foobar".to_string()],
            ),
            (
                ItemIdentifier::from_str("bar.z").unwrap(),
                vec!["bar".to_string()],
            ),
            (
                ItemIdentifier::from_str("bar.w").unwrap(),
                vec!["bar".to_string()],
            ),
            // (
            //     ItemIdentifier::from_str("__builtin__.xyz").unwrap(),
            //     vec!["bar".to_string()],
            // ),
        ];
        let mut actual = vec![];
        tree.visit_inner(&["foo"], &mut |id, libraries| {
            actual.push((
                id.clone(),
                libraries.iter().map(|l| l.to_string()).collect::<Vec<_>>(),
            ));
        });

        expected.sort();
        actual.sort();
        assert_eq!(&expected, &*actual, "\n\n{expected:#?}\n\n{actual:#?}");
    }
}
