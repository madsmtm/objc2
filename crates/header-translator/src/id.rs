use core::fmt;

use clang::Entity;

use crate::context::Context;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ItemIdentifier<N = String> {
    /// Names in Objective-C are global, so this is always enough to uniquely
    /// identify the item.
    ///
    /// Often, though, we want to know the library an item came from as well.
    pub name: N,
    pub library: String,
    pub file_name: Option<String>,
}

impl<N> ItemIdentifier<N> {
    pub fn with_name(name: N, entity: &Entity<'_>, context: &Context<'_>) -> Self {
        let (library, file_name) = context
            .get_library_and_file_name(entity)
            .expect("ItemIdentifier get library and file");
        Self {
            name,
            library,
            file_name,
        }
    }

    fn map_name<R>(self, f: impl FnOnce(N) -> R) -> ItemIdentifier<R> {
        let Self {
            name,
            library,
            file_name,
        } = self;
        ItemIdentifier {
            name: f(name),
            library,
            file_name,
        }
    }

    pub fn with_new_path<R>(self, other: &ItemIdentifier<R>) -> Self {
        Self {
            name: self.name,
            library: other.library.clone(),
            file_name: other.file_name.clone(),
        }
    }
}

impl ItemIdentifier {
    pub fn new(entity: &Entity<'_>, context: &Context<'_>) -> Self {
        let name = entity.get_name().expect("ItemIdentifier get name");
        Self::with_name(name, entity, context)
    }

    pub fn to_some(self) -> ItemIdentifier<Option<String>> {
        self.map_name(|s| Some(s))
    }

    pub fn is_system(&self) -> bool {
        self.library == "System"
    }

    // pub fn is_nsobject(&self) -> bool {
    //     self.library == "System" && self.name == "NSObject"
    // }

    pub fn is_nserror(&self) -> bool {
        self.library == "Foundation" && self.name == "NSError"
    }

    pub fn nserror() -> Self {
        Self {
            name: "NSError".to_string(),
            library: "Foundation".to_string(),
            file_name: Some("NSError".to_string()),
        }
    }

    pub fn is_nsstring(&self) -> bool {
        self.library == "Foundation" && self.name == "NSString"
    }

    pub fn feature(&self) -> Option<impl fmt::Display + '_> {
        struct ItemIdentifierFeature<'a>(&'a ItemIdentifier);

        impl fmt::Display for ItemIdentifierFeature<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}_{}", self.0.library, self.0.name)
            }
        }

        (!self.is_system()).then(|| ItemIdentifierFeature(self))
    }

    pub fn path(&self) -> impl fmt::Display + '_ {
        struct ItemIdentifierPath<'a>(&'a ItemIdentifier);

        impl fmt::Display for ItemIdentifierPath<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if self.0.is_system() {
                    write!(f, "{}", self.0.name)
                } else {
                    write!(f, "{}::{}", self.0.library, self.0.name)
                }
            }
        }

        ItemIdentifierPath(self)
    }

    pub fn path_in_relation_to<'a, T>(&'a self, other: &'a ItemIdentifier<T>) -> impl fmt::Display + 'a {
        struct ItemIdentifierPathInRelationTo<'a, T>(&'a ItemIdentifier, &'a ItemIdentifier<T>);

        impl<T> fmt::Display for ItemIdentifierPathInRelationTo<'_, T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if self.1.file_name == self.0.file_name {
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
