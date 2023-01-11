use core::fmt;

use clang::Entity;

use crate::context::Context;

pub trait ToOptionString {
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

impl<N: ToOptionString> ItemIdentifier<N> {
    pub fn with_name(name: N, entity: &Entity<'_>, context: &Context<'_>) -> Self {
        let (mut library_name, mut file_name) = context
            .get_library_and_file_name(entity)
            .unwrap_or_else(|| {
                warn!(?entity, "ItemIdentifier from unknown header");
                ("Unknown".to_string(), None)
            });

        // TODO: Get rid of this hack
        if library_name == "CoreGraphics" {
            if let Some("CGFloat" | "CGPoint" | "CGRect" | "CGSize") = name.to_option() {
                library_name = "Foundation".to_string();
                file_name = Some("NSGeometry".to_string());
            }
        }

        Self {
            name,
            library: context.get_library_alias(library_name),
            file_name,
        }
    }

    fn map_name<R: ToOptionString>(self, f: impl FnOnce(N) -> R) -> ItemIdentifier<R> {
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

    pub fn with_new_path<R: ToOptionString>(self, other: &ItemIdentifier<R>) -> Self {
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
        self.map_name(Some)
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

        (!self.is_system()).then_some(ItemIdentifierFeature(self))
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

    pub fn path_in_relation_to<'a, T: ToOptionString>(
        &'a self,
        other: &'a ItemIdentifier<T>,
    ) -> impl fmt::Display + 'a {
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
