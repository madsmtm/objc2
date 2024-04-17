//! # Bindings to the `InputMethodKit` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/inputmethodkit/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(feature = "unstable-docsrs", feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-input-method-kit/0.2.0")]
#![allow(non_upper_case_globals)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod generated;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;

extern "C" {
    #[cfg(feature = "IMKInputController")]
    pub static kIMKCommandMenuItemName: &'static objc2_foundation::NSString;
    #[cfg(feature = "IMKInputController")]
    pub static kIMKCommandClientName: &'static objc2_foundation::NSString;

    #[cfg(all(feature = "IMKCandidates", feature = "NSString_TODO"))]
    pub static IMKCandidatesOpacityAttributeName: &'static objc2_foundation::NSString;
    #[cfg(all(feature = "IMKCandidates", feature = "NSString_TODO"))]
    pub static IMKCandidatesSendServerKeyEventFirst: &'static objc2_foundation::NSString;

    #[cfg(feature = "IMKServer")]
    pub static IMKModeDictionary: &'static objc2_foundation::NSString;
    #[cfg(feature = "IMKServer")]
    pub static IMKControllerClass: &'static objc2_foundation::NSString;
    #[cfg(feature = "IMKServer")]
    pub static IMKDelegateClass: &'static objc2_foundation::NSString;
}
