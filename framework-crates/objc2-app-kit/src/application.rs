#[cfg(feature = "NSResponder")]
pub fn NSApp(mtm: objc2_foundation::MainThreadMarker) -> objc2::rc::Id<crate::NSApplication> {
    // TODO: Use the `NSApp` static
    crate::NSApplication::sharedApplication(mtm)
}
