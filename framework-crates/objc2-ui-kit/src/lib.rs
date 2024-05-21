//! # Bindings to the `UIKit` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/uikit/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-ui-kit/0.2.2")]
#![recursion_limit = "256"]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod generated;
mod geometry;
#[cfg(feature = "UIGestureRecognizer")]
mod gesture_recognizer;
#[cfg(feature = "UIPasteConfigurationSupporting")]
mod paste_configuration;
#[cfg(feature = "UIResponder")]
mod responder;
#[cfg(test)]
mod tests;
#[cfg(feature = "NSText")]
mod text;

#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;
pub use self::geometry::*;
#[cfg(feature = "UIResponder")]
pub use self::responder::*;
#[cfg(feature = "NSText")]
pub use self::text::*;

// Used by UIKeyConstants
// NOTE: CFIndex is c_long_long on __LLP64__ / Windows 64-bit (doesn't matter for us)
#[allow(unused)]
pub(crate) type CFIndex = std::os::raw::c_long;
