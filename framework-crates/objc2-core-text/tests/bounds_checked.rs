#![cfg(feature = "CTTypesetter")]
#![cfg(feature = "CTLine")]
use objc2_core_foundation::{CFAttributedString, CFRange, CFString};
use objc2_core_text::CTTypesetter;

/// Test that passing invalid indices doesn't crash.
#[test]
fn invalid_typesetter_indices() {
    let s = unsafe {
        CFAttributedString::new(None, Some(&CFString::from_static_str("foo")), None).unwrap()
    };
    let typesetter = CTTypesetter::with_attributed_string(&s);
    let _ = typesetter.line_with_offset(CFRange::new(10, 10), 0.0);
    let idx = typesetter.suggest_line_break_with_offset(10, 0.0, 0.0);
    assert_eq!(idx, 0);
}
