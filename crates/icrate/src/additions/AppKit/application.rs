#[cfg(feature = "AppKit_NSResponder")]
pub fn NSApp(
    mtm: crate::Foundation::MainThreadMarker,
) -> objc2::rc::Id<crate::AppKit::NSApplication> {
    // TODO: Use the `NSApp` static
    crate::AppKit::NSApplication::sharedApplication(mtm)
}
