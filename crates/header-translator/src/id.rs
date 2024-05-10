use core::fmt;
use core::hash;
use std::cmp::Ordering;
use std::collections::BTreeSet;

use clang::Entity;

use crate::context::Context;
use crate::display_helper::FormatterFn;
use crate::file::clean_name;
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

#[derive(Debug, Clone)]
pub struct Location {
    pub library: String,
    file_name: Option<String>,
}

impl PartialEq for Location {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for Location {}

impl hash::Hash for Location {
    fn hash<H: hash::Hasher>(&self, _state: &mut H) {}
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Location {
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl Location {
    pub fn krate<'a>(&self, config: &'a Config) -> Option<&'a str> {
        if self.library == "block2" {
            return Some("block2");
        }
        Some(&config.libraries.get(&self.library)?.krate)
    }

    pub fn assert_file(&self, file_name: &str) {
        assert_eq!(self.file_name.as_deref(), Some(file_name));
    }

    /// Only the library of the emmision location matters.
    pub fn cargo_toml_feature(&self, config: &Config, emission_library: &str) -> Option<String> {
        if self.library == "System" || self.library == "block2" || self.library == emission_library
        {
            None
        } else if let Some(krate) = self.krate(config) {
            let required = config.libraries[emission_library]
                .required_dependencies
                .contains(krate);
            let feature_name = if let Some(file_name) = &self.file_name {
                clean_name(file_name)
            } else {
                error!("tried to get feature name of location with an unknown file name");
                format!("{}_Unknown", self.library)
            };
            Some(format!(
                "{krate}{}/{feature_name}",
                if required { "" } else { "?" }
            ))
        } else {
            debug!(?self, "failed getting crate name");
            None
        }
    }

    /// Only the library of the emmision location matters.
    fn feature(&self, config: &Config, emission_location: &Self) -> Option<String> {
        if self.library == "System" {
            None
        } else if self.library == "block2" {
            Some("block2".to_string())
        } else if self.library == emission_location.library {
            if let Some(file_name) = &self.file_name {
                Some(clean_name(file_name))
            } else {
                error!("tried to get feature name of location with an unknown file name");
                Some(format!("{}_Unknown", self.library))
            }
        } else if let Some(krate) = self.krate(config) {
            let required = config.libraries[&emission_location.library]
                .required_dependencies
                .contains(krate);
            if required {
                None
            } else {
                Some(krate.to_string())
            }
        } else {
            debug!(
                library = self.library,
                "tried to get feature name of unknown library"
            );
            None
        }
    }
}

/// Names in C and Objective-C are global, so this is always enough to
/// uniquely identify an item.
///
/// Often, though, we want to know the library, file name and general location
/// an item came from as well.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ItemIdentifier<N = String> {
    pub name: N,
    location: Location,
}

impl<N: ToOptionString> ItemIdentifier<N> {
    pub fn library(&self) -> &str {
        &self.location.library
    }

    pub fn from_raw(name: N, library: String, file_name: String) -> Self {
        Self {
            name,
            location: Location {
                library,
                file_name: Some(file_name),
            },
        }
    }

    pub fn with_name(name: N, entity: &Entity<'_>, context: &Context<'_>) -> Self {
        let (mut library_name, mut file_name) = context
            .get_library_and_file_name(entity)
            .unwrap_or_else(|| {
                warn!(?entity, "ItemIdentifier from unknown header");
                ("__Unknown__".to_string(), None)
            });

        // TODO: Get rid of these hacks
        if let Some("CGFloat" | "CGPoint" | "CGRect" | "CGSize") = name.to_option() {
            library_name = "Foundation".to_string();
            file_name = Some("NSGeometry".to_string());
        }
        if let Some("CFTimeInterval") = name.to_option() {
            library_name = "System".to_string();
            file_name = None;
        }

        Self {
            name,
            location: Location {
                library: library_name,
                file_name,
            },
        }
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
        self.location.library == "System"
            && (self.name == "NSObject" || self.name == "NSObjectProtocol")
    }

    pub fn is_nserror(&self) -> bool {
        self.location.library == "Foundation" && self.name == "NSError"
    }

    pub fn nserror() -> Self {
        Self {
            name: "NSError".to_string(),
            location: Location {
                library: "Foundation".to_string(),
                file_name: Some("NSError".to_string()),
            },
        }
    }

    pub fn block() -> Self {
        Self {
            name: "Block".to_string(),
            location: Location {
                library: "block2".to_string(),
                file_name: None,
            },
        }
    }

    pub fn is_nsstring(&self) -> bool {
        self.location.library == "Foundation" && self.name == "NSString"
    }

    pub fn is_nscomparator(&self) -> bool {
        self.location.library == "Foundation" && self.name == "NSComparator"
    }

    pub fn path(&self) -> impl fmt::Display + '_ {
        struct ItemIdentifierPath<'a>(&'a ItemIdentifier);

        impl fmt::Display for ItemIdentifierPath<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0.name)
                // if self.0.is_system() {
                //     write!(f, "{}", self.0.name)
                // } else {
                //     write!(f, "{}::{}", self.0.library, self.0.name)
                // }
            }
        }

        ItemIdentifierPath(self)
    }

    pub fn path_in_relation_to<'a>(&'a self, other: &'a Location) -> impl fmt::Display + 'a {
        struct ItemIdentifierPathInRelationTo<'a>(&'a ItemIdentifier, &'a Location);

        impl fmt::Display for ItemIdentifierPathInRelationTo<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if self.1.file_name == self.0.location.file_name {
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
        .filter_map(|id| id.as_ref().feature(config, emission_location))
        .collect();

    for location in implied_features {
        if let Some(feature_name) = location.as_ref().feature(config, emission_location) {
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
