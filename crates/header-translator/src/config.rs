use std::collections::{BTreeMap, HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::sync::OnceLock;
use std::{fmt, ptr};

use clang::{Entity, EntityKind};
use heck::ToTrainCase;
use semver::Version;
use serde::{de, Deserialize, Deserializer};

use crate::name_translation::cf_no_ref;
use crate::stmt::{Counterpart, Derives};
use crate::{ItemIdentifier, Location};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub(crate) libraries: BTreeMap<String, LibraryConfig>,
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

        for lib in libraries.values() {
            lib.validate();
        }

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
                        cf_no_ref(&name).to_string()
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
    pub(crate) module: Location,
}

impl ExternalData {
    pub(crate) fn into_id(self, name: String) -> ItemIdentifier {
        ItemIdentifier::from_raw(name, self.module)
    }
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
    #[serde(rename = "located-outside-sdk")]
    #[serde(default)]
    pub located_outside_sdk: bool,
    // TODO: Maybe flip this config option, and make most dependencies non-default?
    #[serde(rename = "undesired-default-dependencies")]
    #[serde(default)]
    pub undesired_default_dependencies: HashSet<String>,

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

    /// Data about an external item whose header isn't imported.
    ///
    /// Usually a bare `@protocol X;` or `@class X;`, but might also be used
    /// for other things.
    #[serde(default)]
    pub external: BTreeMap<String, ExternalData>,

    #[serde(rename = "class")]
    #[serde(default)]
    pub class_data: HashMap<String, StmtData>,
    #[serde(rename = "protocol")]
    #[serde(default)]
    pub protocol_data: HashMap<String, StmtData>,
    #[serde(rename = "struct")]
    #[serde(default)]
    pub struct_data: HashMap<String, StmtData>,
    #[serde(rename = "union")]
    #[serde(default)]
    pub union_data: HashMap<String, StmtData>,
    #[serde(rename = "enum")]
    #[serde(default)]
    pub enum_data: HashMap<String, StmtData>,
    #[serde(rename = "fn")]
    #[serde(default)]
    pub fns: HashMap<String, StmtData>,
    #[serde(rename = "static")]
    #[serde(default)]
    pub statics: HashMap<String, StmtData>,
    #[serde(rename = "typedef")]
    #[serde(default)]
    pub typedef_data: HashMap<String, StmtData>,
    #[serde(rename = "const")]
    #[serde(default)]
    pub const_data: HashMap<String, StmtData>,

    #[serde(rename = "unsafe-default-safety")]
    #[serde(default)]
    pub default_safety: DefaultSafety,
    #[serde(default)]
    pub module: HashMap<String, ModuleConfig>,
}

/// There are many different things that influence the soundness
/// of an API. A few of these:
/// 1. Initialization safety.
/// 2. Lifetime safety.
/// 3. Type safety.
/// 4. Bounds safety.
/// 5. Thread safety.
/// 6. Arbitrary additional restrictions.
///
/// Objective-C's object model upholds the initialization requirement, we can
/// reasonably ensure that lifetime safety is upheld (object pointers are
/// reference-counted, and we can disallow direct pointers), type safety is
/// also doable (the subtyping model is fairly intricate, but we can be
/// conservative), collections like NSArray etc. are internally bounds checked
/// and thread safety is handled by `nonatomic` on properties, as well as the
/// NS_SWIFT_SENDABLE- and NS_SWIFT_UI_ACTOR-like macros.
///
/// The only thing we really can't check automatically is if a function/method
/// does something outside the usual. One example of this is the
/// `NSAutoreleasePool` class, which may allow one to violate lifetime safety
/// if misused.
///
/// As such, each framework must be explicitly marked with the kinds of safety
/// it has been reviewed for.
#[derive(Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
pub struct DefaultSafety {
    /// Property getters are marked as safe by default.
    #[serde(rename = "property-getters")]
    #[serde(default)]
    pub property_getters: bool,
    /// Property setters are marked as safe by default, provided that they do
    /// not take pointers as arguments.
    #[serde(rename = "property-setters")]
    #[serde(default)]
    pub property_setters: bool,
    /// Class and protocol instance methods are marked as safe by default,
    /// provided that they do not take pointers as arguments.
    #[serde(rename = "instance-methods")]
    #[serde(default)]
    pub instance_methods: bool,
    /// Class and protocol class methods are marked as safe by default,
    /// provided that they do not take pointers as arguments.
    ///
    /// This includes initializers.
    #[serde(rename = "class-methods")]
    #[serde(default)]
    pub class_methods: bool,
    /// Functions are marked as safe by default, provided that they do not
    /// take pointers as arguments.
    #[serde(default)]
    pub functions: bool,
    /// Whether functions or methods with bounds-affecting parameters are
    /// excluded from this default.
    ///
    /// The documentation in CoreFoundation-like frameworks often say it's
    /// "undefined behaviour" to have indexes or ranges out of bounds - but
    /// then the function often checks it anyhow, so whether that is something
    /// we can rely on is unclear?
    #[serde(rename = "not-bounds-affecting")]
    #[serde(default)]
    pub not_bounds_affecting: bool,
}

fn link_default() -> bool {
    true
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

        !self.undesired_default_dependencies.contains(&other.krate)
            && inner(&self.macos, &other.macos, semver::Version::new(10, 12, 0))
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

    fn validate(&self) {
        use std::iter::empty;

        let all = empty()
            .chain(self.class_data.values())
            .chain(self.protocol_data.values())
            .chain(self.struct_data.values())
            .chain(self.union_data.values())
            .chain(self.enum_data.values())
            .chain(self.fns.values())
            .chain(self.statics.values())
            .chain(self.typedef_data.values())
            .chain(self.const_data.values());

        fn filter_ptr<'a>(
            allowed_in: impl Iterator<Item = &'a StmtData> + Clone,
        ) -> impl Fn(&&StmtData) -> bool {
            move |data| !allowed_in.clone().any(|allowed| ptr::eq(*data, allowed))
        }

        let allowed_in = empty()
            .chain(self.class_data.values())
            .chain(self.protocol_data.values());
        for data in all.clone().filter(filter_ptr(allowed_in)) {
            assert_eq!(data.methods, Default::default());
        }

        let allowed_in = empty()
            .chain(self.enum_data.values())
            .chain(self.statics.values())
            .chain(self.const_data.values());
        for data in all.clone().filter(filter_ptr(allowed_in)) {
            assert_eq!(data.use_value, Default::default());
        }

        let allowed_in = self.class_data.values();
        for data in all.clone().filter(filter_ptr(allowed_in)) {
            assert_eq!(data.derives, Default::default());
            assert_eq!(data.definition_skipped, Default::default());
            assert_eq!(data.categories, Default::default());
            assert_eq!(data.counterpart, Default::default());
            assert_eq!(data.skipped_protocols, Default::default());
            assert_eq!(data.main_thread_only, Default::default());
            assert_eq!(data.bridged_to, Default::default());
        }

        let allowed_in = self.protocol_data.values();
        for data in all.clone().filter(filter_ptr(allowed_in)) {
            assert_eq!(data.requires_mainthreadonly, Default::default());
        }

        let allowed_in = self.typedef_data.values();
        for data in all.clone().filter(filter_ptr(allowed_in)) {
            assert!(data.generics.is_empty());
        }

        let allowed_in = self.fns.values();
        for data in all.clone().filter(filter_ptr(allowed_in)) {
            assert_eq!(data.no_implementor, Default::default());
            assert_eq!(data.implementor, Default::default());
            assert_eq!(data.arguments, Default::default());
            assert_eq!(data.return_, Default::default());
        }

        let allowed_in = self
            .fns
            .values()
            .chain(self.class_data.values())
            .chain(self.protocol_data.values());
        for data in all.clone().filter(filter_ptr(allowed_in)) {
            assert_eq!(data.unsafe_, Default::default());
        }

        let allowed_in = self.typedef_data.values().chain(self.statics.values());
        for data in all.clone().filter(filter_ptr(allowed_in)) {
            assert_eq!(data.nullability, Default::default());
        }
    }

    pub(crate) fn get(&self, entity: &Entity<'_>) -> &StmtData {
        let name = if entity.is_anonymous() {
            // union (unnamed at /Applications/Xcode.app/...)
            // union (anonymous at /Applications/Xcode.app/...)
            // enum (unnamed at /Applications/Xcode.app/...)
            // struct (unnamed at /Applications/Xcode.app/...)
            //
            // Anonymous enums don't have a name that we can use to look up
            // a config. So we use the special "__anonymous__" instead.
            "__anonymous__".to_string()
        } else {
            if let Some(name) = entity.get_name() {
                name
            } else {
                return StmtData::empty();
            }
        };

        let data = match entity.get_kind() {
            EntityKind::ObjCInterfaceDecl | EntityKind::ObjCClassRef => self.class_data.get(&name),
            EntityKind::ObjCCategoryDecl => None, // TODO
            EntityKind::ObjCProtocolDecl | EntityKind::ObjCProtocolRef => {
                self.protocol_data.get(&name)
            }
            EntityKind::TypedefDecl => self.typedef_data.get(&name),
            EntityKind::StructDecl => self.struct_data.get(&name),
            EntityKind::UnionDecl => self.union_data.get(&name),
            EntityKind::EnumDecl => self.enum_data.get(&name),
            EntityKind::EnumConstantDecl => self.const_data.get(&name),
            EntityKind::VarDecl => self.statics.get(&name),
            EntityKind::FunctionDecl => self.fns.get(&name),
            EntityKind::FieldDecl => None, // TODO
            // TODO: Add #[doc(alias = ...)] on methods?
            EntityKind::ObjCClassMethodDecl
            | EntityKind::ObjCInstanceMethodDecl
            | EntityKind::ObjCPropertyDecl => None,
            EntityKind::MacroDefinition | EntityKind::MacroExpansion => None,
            EntityKind::UnexposedDecl => None,
            kind => {
                error!(
                    ?kind,
                    "tried to look up ItemIdentifier from unknown entity kind"
                );
                None
            }
        };

        data.unwrap_or_else(|| StmtData::empty())
    }
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct StmtData {
    // Common
    #[serde(default)]
    pub skipped: Option<bool>,
    #[serde(default)]
    pub renamed: Option<String>,

    // Classes and protocols.
    #[serde(default)]
    pub methods: HashMap<String, MethodData>,

    // Enums, constants and statics.
    #[serde(rename = "use-value")]
    #[serde(default)]
    pub use_value: Option<bool>,

    // Class only.
    #[serde(default)]
    pub derives: Derives,
    #[serde(rename = "definition-skipped")]
    #[serde(default)]
    pub definition_skipped: bool,
    #[serde(default)]
    pub categories: HashMap<String, CategoryData>,
    #[serde(default)]
    pub counterpart: Counterpart,
    #[serde(rename = "skipped-protocols")]
    #[serde(default)]
    pub skipped_protocols: HashSet<String>,
    #[serde(default)]
    #[serde(rename = "main-thread-only")]
    pub main_thread_only: bool,
    /// Toll-free bridging is declared on the CF-typedef, while we need it on
    /// the class in a different framework. `header-translator` tries to avoid
    /// too much global analysis (to allow processing a single framework at a
    /// time), so we must define each of these manually.
    ///
    /// They are correctness-checked in `global_analysis.rs` though.
    #[serde(default)]
    #[serde(rename = "bridged-to")]
    pub bridged_to: Option<ItemIdentifier>,

    // Protocol only.
    #[serde(default)]
    #[serde(rename = "requires-mainthreadonly")]
    pub requires_mainthreadonly: Option<bool>,

    // Typedef only.
    #[serde(default)]
    pub generics: Vec<String>,

    // Functions only.
    #[serde(rename = "no-implementor")]
    #[serde(default)]
    pub no_implementor: bool,
    #[serde(default)]
    pub implementor: Option<ItemIdentifier>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_argument_overrides")]
    pub arguments: HashMap<usize, TypeOverride>,
    #[serde(rename = "return")]
    #[serde(default)]
    pub return_: TypeOverride,

    // Classes, protocols and functions
    #[serde(rename = "unsafe")]
    #[serde(default)]
    pub unsafe_: Option<bool>,

    // Typedef and statics
    #[serde(default)]
    pub nullability: Option<Nullability>,
}

impl StmtData {
    pub fn empty() -> &'static Self {
        static DEFAULT: OnceLock<StmtData> = OnceLock::new();
        DEFAULT.get_or_init(StmtData::default)
    }

    pub fn method(&self, key: &str) -> MethodData {
        let mut data = self.methods.get(key).cloned().unwrap_or_default();
        if data.unsafe_.is_none() {
            // Propagate safety of class/protocol to methods.
            data.unsafe_ = self.unsafe_;
        }
        data
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

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "lowercase")]
pub enum Nullability {
    Nullable,
    NonNull,
}

impl From<Nullability> for clang::Nullability {
    fn from(nullability: Nullability) -> Self {
        match nullability {
            Nullability::Nullable => clang::Nullability::Nullable,
            Nullability::NonNull => clang::Nullability::NonNull,
        }
    }
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TypeOverride {
    #[serde(default)]
    pub nullability: Option<Nullability>,
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct MethodData {
    #[serde(default)]
    pub skipped: bool,
    #[serde(default)]
    pub renamed: Option<String>,
    #[serde(rename = "unsafe")]
    #[serde(default)]
    pub unsafe_: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_argument_overrides")]
    pub arguments: HashMap<usize, TypeOverride>,
    #[serde(rename = "return")]
    #[serde(default)]
    pub return_: TypeOverride,
}

impl MethodData {
    pub(crate) fn merge_with_superclass(self, superclass: Self) -> Self {
        let unsafe_ = match (self.unsafe_, superclass.unsafe_) {
            // Prefer safety attribute on the item itself.
            (Some(unsafe_), _) => Some(unsafe_),
            // Otherwise, take from the superclass if explicitly set as unsafe.
            (_, Some(true)) => Some(true),
            // Otherwise assume nothing.
            _ => None,
        };
        Self {
            unsafe_,
            renamed: self.renamed.or(superclass.renamed).clone(),
            skipped: self.skipped | superclass.skipped,
            arguments: self.arguments,
            return_: self.return_,
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
        if config.krate == "objc2-xc-ui-automation" {
            // Better match `objc2-xc-test`.
            return Ok(config);
        }
        if config.krate == "objc2-open-gl-es" {
            // Better match `objc2-open-gl`.
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

fn deserialize_argument_overrides<'de, D>(
    deserializer: D,
) -> Result<HashMap<usize, TypeOverride>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let str_map = HashMap::<&str, TypeOverride>::deserialize(deserializer)?;
    let original_len = str_map.len();
    let data = {
        str_map
            .into_iter()
            .map(|(str_key, value)| match str_key.parse() {
                Ok(int_key) => Ok((int_key, value)),
                Err(_) => Err({
                    de::Error::invalid_value(
                        de::Unexpected::Str(str_key),
                        &"a non-negative integer",
                    )
                }),
            })
            .collect::<Result<HashMap<_, _>, _>>()?
    };
    // multiple strings could parse to the same int, e.g "0" and "00"
    if data.len() < original_len {
        return Err(de::Error::custom("duplicate integer key"));
    }
    Ok(data)
}
