//! # Bindings to the `AppKit` framework
//!
//! Note that a lot of functionality in AppKit requires that the application
//! has initialized properly, which is only done after the application
//! delegate has received `applicationDidFinishLaunching`.
//!
//! You should aspire to do all your UI initialization work in there!
//!
//!
//! ## NSWindow
//!
//! Be careful when creating `NSWindow` if it's not inside a window
//! controller; in those cases you're _required_ to call
//! `window.releasedWhenClosed(false)` to get correct memory management, which
//! is also why the creation methods for `NSWindow` are `unsafe`.
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
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unreachable_pub)]

#[allow(unreachable_pub)]
#[allow(unused_imports)]
pub use crate::generated::AppKit::*;

#[cfg_attr(feature = "gnustep-1-7", link(name = "gnustep-gui", kind = "dylib"))]
extern "C" {}

/// (!TARGET_CPU_X86_64 || (TARGET_OS_IPHONE && !TARGET_OS_MACCATALYST))
///
/// <https://github.com/xamarin/xamarin-macios/issues/12111>
// TODO: Make this work with mac catalyst
#[allow(dead_code)]
pub(crate) const TARGET_ABI_USES_IOS_VALUES: bool =
    !cfg!(any(target_arch = "x86", target_arch = "x86_64")) || cfg!(not(target_os = "macos"));

#[cfg(feature = "AppKit_NSApplication")]
mod application;
#[cfg(feature = "AppKit_NSImage")]
mod image;
#[cfg(feature = "AppKit_NSText")]
mod text;

#[cfg(feature = "AppKit_NSApplication")]
pub use self::application::*;
#[cfg(feature = "AppKit_NSImage")]
pub use self::image::*;
#[cfg(feature = "AppKit_NSText")]
pub use self::text::*;

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "AppKit_NSAccessibilityProtocols")]
    fn accessibility_element_protocol() {
        use crate::AppKit::NSAccessibilityElementProtocol;
        use objc2::ProtocolType;
        let actual = <dyn NSAccessibilityElementProtocol>::NAME;
        assert_eq!(actual, "NSAccessibilityElement");
    }
}
