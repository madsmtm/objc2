#![no_std]
#![cfg_attr(feature = "unstable-docsrs", feature(doc_auto_cfg))]
#![warn(elided_lifetimes_in_paths)]
#![deny(non_ascii_idents)]
#![warn(unreachable_pub)]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::cargo)]
#![warn(clippy::ptr_as_ptr)]
#![allow(clippy::upper_case_acronyms)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/icrate/0.0.1")]
#![recursion_limit = "512"]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "objective-c")]
pub extern crate objc2;

mod common;
#[macro_use]
mod macros;

// Frameworks
#[cfg(feature = "AppKit")]
pub mod AppKit;
#[cfg(feature = "AuthenticationServices")]
pub mod AuthenticationServices;
#[cfg(feature = "BackgroundAssets")]
pub mod BackgroundAssets;
#[cfg(feature = "CoreAnimation")]
pub mod CoreAnimation;
#[cfg(feature = "CoreData")]
pub mod CoreData;
#[cfg(feature = "Foundation")]
pub mod Foundation;
#[cfg(feature = "Metal")]
pub mod Metal;
#[cfg(feature = "WebKit")]
pub mod WebKit;
