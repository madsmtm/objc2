//! # Bindings to the `AppKit` framework
//!
//!
//! ## Examples
//!
//! Implementing `NSApplicationDelegate` for a custom class.
//!
//! ```ignore
#![doc = include_str!("../../../examples/delegate.rs")]
//! ```
//!
//! An example showing basic and a bit more advanced usage of `NSPasteboard`.
//!
//! ```ignore
#![doc = include_str!("../../../examples/nspasteboard.rs")]
//! ```

pub use crate::generated::AppKit::*;

use crate::common::*;

/// (!TARGET_CPU_X86_64 || (TARGET_OS_IPHONE && !TARGET_OS_MACCATALYST))
///
/// <https://github.com/xamarin/xamarin-macios/issues/12111>
// TODO: Make this work with mac catalyst
const TARGET_ABI_USES_IOS_VALUES: bool =
    !cfg!(any(target_arch = "x86", target_arch = "x86_64")) || cfg!(not(target_os = "macos"));

ns_enum!(
    #[underlying(NSInteger)]
    #[allow(clippy::bool_to_int_with_if)]
    pub enum NSImageResizingMode {
        NSImageResizingModeStretch = if TARGET_ABI_USES_IOS_VALUES { 0 } else { 1 },
        NSImageResizingModeTile = if TARGET_ABI_USES_IOS_VALUES { 1 } else { 0 },
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    #[allow(clippy::bool_to_int_with_if)]
    pub enum NSTextAlignment {
        NSTextAlignmentLeft = 0,
        NSTextAlignmentRight = if TARGET_ABI_USES_IOS_VALUES { 2 } else { 1 },
        NSTextAlignmentCenter = if TARGET_ABI_USES_IOS_VALUES { 1 } else { 2 },
        NSTextAlignmentJustified = 3,
        NSTextAlignmentNatural = 4,
    }
);

#[cfg(feature = "AppKit_NSImage")]
unsafe impl crate::Foundation::NSCoding for crate::AppKit::NSImage {}

#[cfg(feature = "AppKit_NSApplication")]
pub fn NSApp(mtm: crate::Foundation::MainThreadMarker) -> Id<crate::AppKit::NSApplication> {
    // TODO: Use the `NSApp` static
    crate::AppKit::NSApplication::sharedApplication(mtm)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::AppKit::NSAccessibilityElementProtocol;

    #[test]
    fn accessibility_element_protocol() {
        let actual = <dyn NSAccessibilityElementProtocol>::NAME;
        assert_eq!(actual, "NSAccessibilityElement");
    }
}
