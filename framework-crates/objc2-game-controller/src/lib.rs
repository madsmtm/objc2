//! # Bindings to the `GameController` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/gamecontroller/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-game-controller/0.2.2")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "GCExtendedGamepadSnapshot")]
mod extended_gamepad_snapshot;
mod generated;
#[cfg(feature = "GCInputNames")]
mod input_names;

#[cfg(feature = "GCExtendedGamepadSnapshot")]
#[allow(deprecated)]
pub use self::extended_gamepad_snapshot::GCExtendedGamepadSnapshotData;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;
#[cfg(feature = "GCInputNames")]
pub use self::input_names::*;

// TODO: GCKeyCode = CFIndex
// NOTE: CFIndex is c_long_long on __LLP64__ / Windows 64-bit (doesn't matter for us)
#[cfg(feature = "GCKeyCodes")]
pub type GCKeyCode = std::os::raw::c_long;
