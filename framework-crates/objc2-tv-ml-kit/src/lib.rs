//! # Bindings to the `TVMLKit` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/tvmlkit/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-tv-ml-kit/0.3.0")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod generated;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;

#[allow(deprecated, non_upper_case_globals)]
#[cfg(feature = "TVViewElement")]
impl TVElementUpdateType {
    /// Signifies that the node itself and its subtree is modified.
    #[deprecated = "Please use SwiftUI or UIKit"]
    #[doc(alias = "TVElementUpdateTypeSelf")]
    pub const Node: Self = Self(4);
}
