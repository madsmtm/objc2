#![cfg(feature = "AppKit")]
use icrate::AppKit::NSAccessibilityElementProtocol;
use objc2::ProtocolType;

#[test]
fn accessibility_element_protocol() {
    let actual: &str = <dyn NSAccessibilityElementProtocol>::NAME;
    assert_eq!(actual, "NSAccessibilityElement");
}
