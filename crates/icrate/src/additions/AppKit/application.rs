#[allow(unused_imports)]
use crate::common::*;

#[cfg(feature = "AppKit_NSResponder")]
pub fn NSApp(mtm: crate::Foundation::MainThreadMarker) -> Id<crate::AppKit::NSApplication> {
    // TODO: Use the `NSApp` static
    crate::AppKit::NSApplication::sharedApplication(mtm)
}
