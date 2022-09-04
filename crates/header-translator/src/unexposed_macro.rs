use clang::{Entity, EntityKind};

/// Parts of `EntityKind::UnexposedAttr` that we can easily parse.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum UnexposedMacro {
    Enum,
    Options,
    ClosedEnum,
    ErrorEnum,
}

impl UnexposedMacro {
    fn from_name(s: &str) -> Option<Self> {
        match s {
            "NS_ENUM" => Some(Self::Enum),
            "NS_OPTIONS" => Some(Self::Options),
            "NS_CLOSED_ENUM" => Some(Self::ClosedEnum),
            "NS_ERROR_ENUM" => Some(Self::ErrorEnum),
            // TODO
            "NS_FORMAT_FUNCTION" => None,
            "NS_FORMAT_ARGUMENT" => None,
            // Uninteresting, their data is already exposed elsewhere
            "API_AVAILABLE"
            | "API_UNAVAILABLE"
            | "API_DEPRECATED"
            | "API_DEPRECATED_WITH_REPLACEMENT"
            | "NS_SWIFT_UNAVAILABLE"
            | "NS_SWIFT_NAME"
            | "NS_CLASS_AVAILABLE_MAC"
            | "NS_AVAILABLE"
            | "NS_OPENGL_DEPRECATED"
            | "NS_OPENGL_CLASS_DEPRECATED"
            | "NS_OPENGL_ENUM_DEPRECATED" => None,
            name => panic!("unknown unexposed macro name {name}"),
        }
    }

    pub fn parse(entity: &Entity<'_>) -> Option<Self> {
        let location = entity.get_location().expect("unexposed attr location");
        if let Some(parsed) = location.get_entity() {
            match parsed.get_kind() {
                EntityKind::MacroExpansion => {
                    let macro_name = parsed.get_name().expect("macro name");
                    Self::from_name(&macro_name)
                }
                // Some macros can't be found using this method,
                // for example NS_NOESCAPE.
                _ => None,
            }
        } else {
            // File-global macros like APPKIT_API_UNAVAILABLE_BEGIN_MACCATALYST
            // are not findable with this approach
            None
        }
    }
}
