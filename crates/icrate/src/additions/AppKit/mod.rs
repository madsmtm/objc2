use crate::common::*;

#[cfg(feature = "AppKit_NSApplication")]
pub fn NSApp(mtm: crate::Foundation::MainThreadMarker) -> Id<crate::AppKit::NSApplication> {
    // TODO: Use the `NSApp` static
    crate::AppKit::NSApplication::sharedApplication(mtm)
}
