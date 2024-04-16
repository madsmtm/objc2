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

#[cfg(feature = "Foundation_NSString")]
#[allow(unused_imports)]
use objc2_foundation::NSString;

extern "C" {
    #[cfg(all(
        feature = "InputMethodKit_IMKInputController",
        feature = "Foundation_NSString"
    ))]
    pub static kIMKCommandMenuItemName: &'static NSString;
    #[cfg(all(
        feature = "InputMethodKit_IMKInputController",
        feature = "Foundation_NSString"
    ))]
    pub static kIMKCommandClientName: &'static NSString;

    #[cfg(all(
        feature = "InputMethodKit_IMKCandidates",
        feature = "Foundation_NSString"
    ))]
    pub static IMKCandidatesOpacityAttributeName: &'static NSString;
    #[cfg(all(
        feature = "InputMethodKit_IMKCandidates",
        feature = "Foundation_NSString"
    ))]
    pub static IMKCandidatesSendServerKeyEventFirst: &'static NSString;

    #[cfg(all(feature = "InputMethodKit_IMKServer", feature = "Foundation_NSString"))]
    pub static IMKModeDictionary: &'static NSString;
    #[cfg(all(feature = "InputMethodKit_IMKServer", feature = "Foundation_NSString"))]
    pub static IMKControllerClass: &'static NSString;
    #[cfg(all(feature = "InputMethodKit_IMKServer", feature = "Foundation_NSString"))]
    pub static IMKDelegateClass: &'static NSString;
}
