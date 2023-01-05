use std::collections::HashMap;

use clang::source::Location;
use clang::{Entity, EntityKind};
use tracing::warn;

/// Parts of `EntityKind::UnexposedAttr` that we can easily parse.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum UnexposedMacro {
    Enum,
    Options,
    ClosedEnum,
    ErrorEnum,
    TypedEnum,
    TypedExtensibleEnum,
    BridgedTypedef,
}

impl UnexposedMacro {
    fn from_name(s: &str) -> Option<Self> {
        match s {
            "NS_ENUM" => Some(Self::Enum),
            "NS_OPTIONS" => Some(Self::Options),
            "NS_CLOSED_ENUM" => Some(Self::ClosedEnum),
            "NS_ERROR_ENUM" => Some(Self::ErrorEnum),
            "NS_TYPED_ENUM" => Some(Self::TypedEnum),
            "NS_TYPED_EXTENSIBLE_ENUM" => Some(Self::TypedExtensibleEnum),
            "NS_SWIFT_BRIDGED_TYPEDEF" => Some(Self::BridgedTypedef),
            // TODO
            "NS_FORMAT_FUNCTION" => None,
            "NS_FORMAT_ARGUMENT" => None,
            // Availability attributes - their data is already exposed.
            "API_AVAILABLE"
            | "API_UNAVAILABLE"
            | "API_DEPRECATED"
            | "API_DEPRECATED_WITH_REPLACEMENT"
            | "NS_CLASS_AVAILABLE_MAC"
            | "NS_AVAILABLE"
            | "NS_OPENGL_DEPRECATED"
            | "NS_OPENGL_CLASS_DEPRECATED"
            | "NS_OPENGL_ENUM_DEPRECATED"
            | "OBJC_AVAILABLE"
            | "OBJC_DEPRECATED"
            | "APPKIT_API_UNAVAILABLE_BEGIN_MACCATALYST" => None,
            // Might be interesting in the future
            "NS_SWIFT_NAME"
            | "NS_SWIFT_ASYNC_NAME"
            | "NS_SWIFT_ASYNC_THROWS_ON_FALSE"
            | "NS_SWIFT_UNAVAILABLE"
            | "OBJC_SWIFT_UNAVAILABLE" => None,
            name => {
                warn!(name, "unknown unexposed macro");
                None
            }
        }
    }

    pub fn parse(entity: &Entity<'_>) -> Option<Self> {
        Self::parse_plus_macros(entity, &HashMap::new())
    }

    pub fn parse_plus_macros(
        entity: &Entity<'_>,
        macro_invocations: &HashMap<Location<'_>, String>,
    ) -> Option<Self> {
        let location = entity.get_location().expect("unexposed attr location");

        if let Some(macro_name) = macro_invocations.get(&location.get_spelling_location()) {
            return Self::from_name(macro_name);
        }

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
            None
        }
    }
}
