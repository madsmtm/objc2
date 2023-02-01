use clang::source::{SourceLocation, SourceRange};
use clang::token::{Token, TokenKind};
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
    pub(crate) fn from_name<T>(
        s: &str,
        get_arguments: impl FnOnce() -> T,
    ) -> Result<Option<Self>, ()> {
        Ok(match s {
            "NS_ENUM" | "CF_ENUM" => {
                let _ = get_arguments();
                Some(Self::Enum)
            }
            "NS_OPTIONS" | "CF_OPTIONS" => {
                let _ = get_arguments();
                Some(Self::Options)
            }
            "NS_CLOSED_ENUM" | "CF_CLOSED_ENUM" => {
                let _ = get_arguments();
                Some(Self::ClosedEnum)
            }
            "NS_ERROR_ENUM" => {
                let _ = get_arguments();
                Some(Self::ErrorEnum)
            }
            "NS_TYPED_ENUM" | "NS_STRING_ENUM" | "CF_TYPED_ENUM" => Some(Self::TypedEnum),
            "_NS_TYPED_EXTENSIBLE_ENUM"
            | "NS_TYPED_EXTENSIBLE_ENUM"
            | "CF_TYPED_EXTENSIBLE_ENUM"
            | "NS_EXTENSIBLE_STRING_ENUM" => Some(Self::TypedExtensibleEnum),
            "NS_SWIFT_BRIDGED_TYPEDEF" | "CF_SWIFT_BRIDGED_TYPEDEF" => Some(Self::BridgedTypedef),
            "CF_BRIDGED_TYPE" => Some(Self::Bridged),
            "CF_BRIDGED_MUTABLE_TYPE" => Some(Self::BridgedMutable),
            "NS_RETURNS_RETAINED" | "CF_RETURNS_RETAINED" => Some(Self::ReturnsRetained),
            "NS_RETURNS_NOT_RETAINED" | "CF_RETURNS_NOT_RETAINED" => Some(Self::ReturnsNotRetained),
            "NS_RETURNS_INNER_POINTER" => None,
            // TODO
            "NS_FORMAT_FUNCTION" | "NS_FORMAT_ARGUMENT" => {
                let _ = get_arguments();
                None
            }
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
            // Weak specifiers - would be interesting if Rust supported weak statics
            "GK_EXTERN_WEAK" => None,
            // Availability attributes - their data is already exposed.
            "API_AVAILABLE"
            | "API_UNAVAILABLE"
            | "API_DEPRECATED"
            | "API_DEPRECATED_WITH_REPLACEMENT"
            | "API_UNAVAILABLE_BEGIN"
            | "NS_CLASS_AVAILABLE_MAC"
            | "NS_AVAILABLE"
            | "NS_OPENGL_DEPRECATED"
            | "NS_OPENGL_CLASS_DEPRECATED"
            | "NS_OPENGL_ENUM_DEPRECATED"
            | "OBJC_AVAILABLE"
            | "OBJC_DEPRECATED"
            | "NS_DEPRECATED"
            | "NS_DEPRECATED_IOS"
            | "NS_DEPRECATED_MAC"
            | "CG_AVAILABLE_STARTING"
            | "CG_AVAILABLE_BUT_DEPRECATED"
            | "NS_SWIFT_UNAVAILABLE"
            | "CF_SWIFT_UNAVAILABLE"
            | "OBJC_SWIFT_UNAVAILABLE"
            | "FPUI_AVAILABLE"
            | "WEBKIT_AVAILABLE_MAC"
            | "WEBKIT_DEPRECATED_MAC"
            | "WEBKIT_CLASS_DEPRECATED_MAC"
            | "WEBKIT_ENUM_DEPRECATED_MAC"
            | "NS_AVAILABLE_MAC"
            | "NS_CLASS_AVAILABLE"
            | "NS_CLASS_DEPRECATED"
            | "NS_CLASS_DEPRECATED_IOS"
            | "NS_ENUM_AVAILABLE"
            | "NS_ENUM_DEPRECATED"
            | "NS_ENUM_DEPRECATED_IOS"
            | "__OSX_AVAILABLE_STARTING"
            | "__TVOS_AVAILABLE"
            | "__WATCHOS_AVAILABLE"
            | "__IOS_AVAILABLE" => {
                let _ = get_arguments();
                None
            }
            "NS_UNAVAILABLE"
            | "UNAVAILABLE_ATTRIBUTE"
            | "NS_AUTOMATED_REFCOUNT_UNAVAILABLE"
            | "NS_AUTOMATED_REFCOUNT_WEAK_UNAVAILABLE"
            | "APPKIT_API_UNAVAILABLE_BEGIN_MACCATALYST"
            | "__OSX_UNAVAILABLE"
            | "__OSX_AVAILABLE_BUT_DEPRECATED"
            | "__TVOS_UNAVAILABLE"
            | "__TVOS_PROHIBITED"
            | "__WATCHOS_UNAVAILABLE"
            | "__WATCHOS_PROHIBITED"
            | "__IOS_UNAVAILABLE"
            | "UT_AVAILABLE_BEGIN"
            | "__IOS_PROHIBITED" => None,
            s if s.starts_with("AVAILABLE_MAC_OS_X_VERSION_") => None,
            s if s.starts_with("DEPRECATED_IN_MAC_OS_X_VERSION_") => None,
            s if s.starts_with("FILEPROVIDER_API_AVAILABILITY_") => None,
            // Might be interesting in the future
            "NS_SWIFT_NAME"
            | "CF_SWIFT_NAME"
            | "NS_SWIFT_ASYNC_NAME"
            | "WK_SWIFT_ASYNC_NAME"
            | "NS_SWIFT_ASYNC_THROWS_ON_FALSE"
            | "NS_SWIFT_UNAVAILABLE_FROM_ASYNC"
            | "NS_HEADER_AUDIT_BEGIN"
            | "NS_REFINED_FOR_SWIFT_ASYNC"
            | "NS_SWIFT_ASYNC"
            | "WK_SWIFT_ASYNC" => {
                let _ = get_arguments();
                None
            }
            "NS_SWIFT_NOTHROW"
            | "NS_REFINED_FOR_SWIFT"
            | "CF_REFINED_FOR_SWIFT"
            | "NS_SWIFT_DISABLE_ASYNC"
            | "NS_SWIFT_UI_ACTOR"
            | "NS_SWIFT_SENDABLE"
            | "NS_SWIFT_NONSENDABLE"
            | "NS_SWIFT_NONISOLATED" => None,
            _ => return Err(()),
        })
    }

    pub fn parse(entity: &Entity<'_>, context: &Context<'_>) -> Option<Self> {
        if let Some(location) = entity.get_location() {
            if let Some(entity) = context
                .macro_invocations
                .get(&location.get_spelling_location())
            {
                let name = entity.get_name().expect("macro name");
                return Self::from_name(&name, || get_argument_tokens(entity)).unwrap_or_else(
                    |()| {
                        error!(
                            name,
                            fnlike = entity.is_function_like_macro(),
                            "unknown unexposed attribute"
                        );
                        None
                    },
                );
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
                    Self::from_name(&macro_name, || get_argument_tokens(&parsed)).unwrap_or_else(
                        |()| {
                            error!(macro_name, "unknown unexposed attribute");
                            None
                        },
                    )
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

fn get_argument_tokens<'a>(entity: &Entity<'a>) -> Vec<Token<'a>> {
    if !entity.is_function_like_macro() {
        error!(?entity, "tried to get tokens from non-function-like macro");
        return vec![];
    }
    // Remove the macro name from the full macro tokens
    let name_ranges = entity.get_name_ranges();
    assert_eq!(name_ranges.len(), 1, "macro name ranges");
    let name_range = name_ranges.get(0).unwrap();
    let range = entity.get_range().expect("macro range");

    let mut tokens = SourceRange::new(name_range.get_end(), range.get_end()).tokenize();

    let start = tokens.remove(0);
    assert_eq!(start.get_kind(), TokenKind::Punctuation);
    assert_eq!(start.get_spelling(), "(");
    let end = tokens.pop().expect("tokens to have parentheses");
    assert_eq!(end.get_kind(), TokenKind::Punctuation);
    assert_eq!(end.get_spelling(), ")");

    tokens
}
