use crate::common::*;

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
