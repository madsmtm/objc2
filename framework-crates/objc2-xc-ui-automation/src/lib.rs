//! # Bindings to the `XCUIAutomation` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/xcuiautomation/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(feature = "unstable-darwin-objc", feature(darwin_objc))]
#![cfg_attr(docsrs, feature(doc_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-xc-ui-automation/0.3.2")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod generated;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;

// Everything but macOS.
#[cfg(not(any(target_os = "macos", target_env = "macabi")))]
mod device_buttons;

#[cfg(target_os = "ios")]
mod siri;

#[cfg(all(target_os = "ios", feature = "objc2-ui-kit"))]
mod orientation;

#[cfg(not(target_os = "macos"))]
pub use device_buttons::*;

#[cfg(all(target_os = "ios", feature = "objc2-ui-kit"))]
pub use orientation::*;

#[cfg(target_os = "ios")]
pub use siri::*;

// Link to XCTest instead of XCUIAutomation, since the latter is only
// available in newer Xcode versions.
#[link(name = "XCTest", kind = "framework")]
extern "C" {}
