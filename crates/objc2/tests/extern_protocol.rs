//! Test that Clippy sees the docs of traits created with `extern_protocol!`.
//!
//! This has to be outside `objc2` itself, since Clippy only ignores docs that
//! come from an external macro (#887).
#![deny(clippy::missing_safety_doc)]
use objc2::{extern_protocol, ProtocolType};

extern_protocol!(
    /// # Safety
    ///
    /// Clippy must see this section.
    #[name = "NSObject"]
    pub unsafe trait WithSafetyDoc {}
);

extern_protocol!(
    /// No safety section, so the lint must still fire.
    #[expect(clippy::missing_safety_doc)]
    #[name = "NSObject"]
    pub unsafe trait WithoutSafetyDoc {}
);

#[test]
fn name() {
    assert_eq!(<dyn WithSafetyDoc>::NAME.to_str().unwrap(), "NSObject");
    assert_eq!(<dyn WithoutSafetyDoc>::NAME.to_str().unwrap(), "NSObject");
}
