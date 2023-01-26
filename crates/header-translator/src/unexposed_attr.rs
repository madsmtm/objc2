use clang::source::SourceLocation;
use clang::{Entity, EntityKind};

use crate::context::Context;

/// Parts of `EntityKind::UnexposedAttr` that we can easily parse.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum UnexposedAttr {
    Enum,
    Options,
    ClosedEnum,
    ErrorEnum,
    TypedEnum,
    TypedExtensibleEnum,

    BridgedTypedef,
    Bridged,
    BridgedMutable,

    ReturnsRetained,
    ReturnsNotRetained,
}

impl UnexposedAttr {
    fn from_name(s: &str) -> Option<Self> {
        match s {
            "NS_ENUM" | "CF_ENUM" => Some(Self::Enum),
            "NS_OPTIONS" | "CF_OPTIONS" => Some(Self::Options),
            "NS_CLOSED_ENUM" | "CF_CLOSED_ENUM" => Some(Self::ClosedEnum),
            "NS_ERROR_ENUM" => Some(Self::ErrorEnum),
            "NS_TYPED_ENUM" | "NS_STRING_ENUM" | "CF_TYPED_ENUM" => Some(Self::TypedEnum),
            "NS_TYPED_EXTENSIBLE_ENUM" | "CF_TYPED_EXTENSIBLE_ENUM" => {
                Some(Self::TypedExtensibleEnum)
            }
            "NS_SWIFT_BRIDGED_TYPEDEF" | "CF_SWIFT_BRIDGED_TYPEDEF" => Some(Self::BridgedTypedef),
            "CF_BRIDGED_TYPE" => Some(Self::Bridged),
            "CF_BRIDGED_MUTABLE_TYPE" => Some(Self::BridgedMutable),
            "NS_RETURNS_RETAINED" | "CF_RETURNS_RETAINED" => Some(Self::ReturnsRetained),
            "NS_RETURNS_NOT_RETAINED" | "CF_RETURNS_NOT_RETAINED" => Some(Self::ReturnsNotRetained),
            // TODO
            "NS_FORMAT_FUNCTION" | "NS_FORMAT_ARGUMENT" => None,
            // TODO
            "NS_NOESCAPE" => None,
            // TODO: We could potentially automatically elide this argument
            // from the method call, though it's rare enough that it's
            // probably not really worth the effort.
            "__unused" => None,
            // We assume that a function is inline if it has a body, so not
            // interesting.
            "NS_INLINE" => None,
            // We don't synthethize properties, so irrelevant for us.
            "NS_REQUIRES_PROPERTY_DEFINITIONS" => None,
            // Availability attributes - their data is already exposed.
            "API_AVAILABLE"
            | "API_UNAVAILABLE"
            | "API_DEPRECATED"
            | "API_DEPRECATED_WITH_REPLACEMENT"
            | "NS_CLASS_AVAILABLE_MAC"
            | "NS_AVAILABLE"
            | "NS_UNAVAILABLE"
            | "NS_AUTOMATED_REFCOUNT_UNAVAILABLE"
            | "NS_AUTOMATED_REFCOUNT_WEAK_UNAVAILABLE"
            | "NS_OPENGL_DEPRECATED"
            | "NS_OPENGL_CLASS_DEPRECATED"
            | "NS_OPENGL_ENUM_DEPRECATED"
            | "OBJC_AVAILABLE"
            | "OBJC_DEPRECATED"
            | "UNAVAILABLE_ATTRIBUTE"
            | "APPKIT_API_UNAVAILABLE_BEGIN_MACCATALYST"
            | "CG_AVAILABLE_STARTING"
            | "CG_AVAILABLE_BUT_DEPRECATED" => None,
            // Might be interesting in the future
            "NS_SWIFT_NAME"
            | "CF_SWIFT_NAME"
            | "NS_SWIFT_ASYNC_NAME"
            | "NS_SWIFT_NOTHROW"
            | "NS_SWIFT_ASYNC_THROWS_ON_FALSE"
            | "NS_SWIFT_UNAVAILABLE_FROM_ASYNC"
            | "NS_SWIFT_DISABLE_ASYNC"
            | "NS_SWIFT_UI_ACTOR"
            | "NS_REFINED_FOR_SWIFT"
            | "CF_REFINED_FOR_SWIFT"
            | "NS_SWIFT_UNAVAILABLE"
            | "CF_SWIFT_UNAVAILABLE"
            | "OBJC_SWIFT_UNAVAILABLE" => None,
            name => {
                error!(name, "unknown unexposed macro");
                None
            }
        }
    }

    pub fn parse(entity: &Entity<'_>, context: &Context<'_>) -> Option<Self> {
        if let Some(location) = entity.get_location() {
            if let Some(macro_name) = context
                .macro_invocations
                .get(&location.get_spelling_location())
            {
                return Self::from_name(macro_name);
            }

            Self::parse_location(location)
        } else {
            error!("unexposed attr location");
            None
        }
    }

    fn parse_location(location: SourceLocation<'_>) -> Option<Self> {
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
