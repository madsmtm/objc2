use core::fmt;
use core::hash;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::BTreeSet;

use clang::Entity;

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
    path_components: Vec<Cow<'static, str>>,
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

impl<'location, 'config> LocationLibrary<'location, 'config> {
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
    pub(crate) fn from_components(path_components: Vec<Cow<'static, str>>) -> Self {
        Self { path_components }
    }

    pub fn library_name(&self) -> &str {
        self.path_components
            .first()
            .expect("location to have at least one component")
    }

    fn file_name(&self) -> Option<&str> {
        match &*self.path_components {
            [_, .., file_name] => Some(&**file_name),
            // Umbrella header
            [_] | [] => None,
        }
    }

    pub fn modules(&self) -> impl IntoIterator<Item = &'_ str> + '_ {
        self.path_components.iter().skip(1).map(|s| &**s)
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
                        let required = config.libraries[emission_library]
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
                    debug!(location = ?self, "failed getting crate name");
                    LocationLibrary::System
                }
            }
        }
    }

    pub fn assert_file(&self, file_name: &str) {
        assert_eq!(self.file_name(), Some(file_name));
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

    pub fn from_raw(name: N, path_components: Vec<Cow<'static, str>>) -> Self {
        Self {
            name,
            location: Location { path_components },
        }
    }

    pub fn with_name(name: N, entity: &Entity<'_>, context: &Context<'_>) -> Self {
        let mut location = context.get_location(entity).unwrap_or_else(|| {
            warn!(?entity, "ItemIdentifier from unknown header");
            Location::from_components(vec!["__Unknown__".into()])
        });

        // TODO: Get rid of these hacks
        if let Some("CGFloat" | "CGPoint" | "CGRect" | "CGSize") = name.to_option() {
            location = Location::from_components(vec!["Foundation".into(), "NSGeometry".into()]);
        }
        if let Some("CFTimeInterval") = name.to_option() {
            location = Location::from_components(vec!["System".into()]);
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

    pub fn is_nsobject(&self) -> bool {
        self.library_name() == "objc2"
            && (self.name == "NSObject" || self.name == "NSObjectProtocol")
    }

    pub fn is_nserror(&self) -> bool {
        self.library_name() == "Foundation" && self.name == "NSError"
    }

    pub fn nserror() -> Self {
        Self {
            name: "NSError".into(),
            location: Location::from_components(vec!["Foundation".into(), "NSError".into()]),
        }
    }

    pub fn block() -> Self {
        Self {
            name: "Block".into(),
            location: Location::from_components(vec!["block2".into()]),
        }
    }

    pub fn bitflags() -> Self {
        Self {
            name: "bitflags".into(),
            location: Location::from_components(vec!["bitflags".into()]),
        }
    }

    pub fn objc2(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            location: Location::from_components(vec!["objc2".into()]),
        }
    }

    pub fn main_thread_marker() -> Self {
        Self {
            name: "MainThreadMarker".into(),
            location: Location::from_components(vec!["Foundation".into()]),
        }
    }

    #[cfg(test)]
    pub fn dummy() -> Self {
        Self {
            name: "DUMMY".into(),
            location: Location::from_components(vec!["System".into()]),
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
/// Only the library of the emmision location matters.
pub fn cfg_gate_ln<R: AsRef<Location>, I: AsRef<Location>>(
    required_features: impl IntoIterator<Item = R>,
    implied_features: impl IntoIterator<Item = I>,
    config: &Config,
    emission_location: &Location,
) -> impl fmt::Display {
    // Use a set to deduplicate features, and to have them in
    // a consistent order.
    let mut feature_names: BTreeSet<String> = required_features
        .into_iter()
        .filter_map(|id| {
            id.as_ref()
                .library(config, emission_location.library_name())
                .feature()
        })
        .collect();

    for location in implied_features {
        if let Some(feature_name) = location
            .as_ref()
            .library(config, emission_location.library_name())
            .feature()
        {
            feature_names.remove(&feature_name);
        }
    }

    FormatterFn(move |f| match feature_names.len() {
        0 => Ok(()),
        1 => {
            let feature = feature_names.first().unwrap();
            writeln!(f, "#[cfg(feature = \"{feature}\")]")
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

            Ok(())
        }
    })
}
