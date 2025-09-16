//! # Bindings to the `UIKit` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/uikit/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-ui-kit/0.3.1")]
#![recursion_limit = "256"]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "UIApplication")]
mod application;
#[cfg(feature = "UIView")]
mod coordinate_space;
mod generated;
#[cfg(feature = "UIGeometry")]
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

#[cfg(feature = "UIView")]
pub use self::coordinate_space::*;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;
#[cfg(feature = "UIGeometry")]
pub use self::geometry::*;
#[cfg(feature = "UIResponder")]
pub use self::responder::*;
#[cfg(feature = "NSText")]
pub use self::text::*;
