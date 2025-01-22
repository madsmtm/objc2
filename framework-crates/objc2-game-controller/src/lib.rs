//! # Bindings to the `GameController` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/gamecontroller/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-game-controller/0.3.0")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod generated;
#[cfg(feature = "GCInputNames")]
mod input_names;

#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;
#[cfg(feature = "GCInputNames")]
pub use self::input_names::*;
