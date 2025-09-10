//! Various explanations and topics of discussion.
//!
//! - [The goals of the `objc2` project][goals].
//! - [The layers that make message-sending safer][layered_safety].
//! - [Interior mutability][interior_mutability].
//! - [Interop with other crates][crate_interop].
//! - [Migrating from the `objc` crate][migrating_from_objc].
//! - [Alternatives to `objc2`][alternatives].
//! - [The changelog][changelog].
//!
//! ## Framework crates
//!
//! - [About the generated framework crates][about_generated].
//! - [The full list of crates][frameworks_list].
//! - [Soundness analysis][frameworks_soundness].
//! - [How `Deref` is used][frameworks_deref].
//! - [How to reduce compile-times][frameworks_cargo_features].
//! - [The changelog][frameworks_changelog].
//!
//! ## Cocoa development
//!
//! Explanations of various concepts that are used when developing for Apple
//! platforms, and how these relate to Rust and `objc2`.
//!
//! - [Run loops and applications][run_loop].
//! - [Key-Value Observing][kvo].
//! - [The Model-View-Controller design pattern][mvc].
//! - [Weak properties][weak_property].
//! - [Interoperating with Swift][swift].
//! - [Cross-compiling from Linux/Windows][cross_compiling].
#![allow(clippy::needless_doctest_main)]

#[doc = include_str!("about_generated.md")]
pub mod about_generated {} // Referenced by header-translator
#[doc = include_str!("alternatives.md")]
pub mod alternatives {}
#[cfg(not(all(doctest, feature = "gnustep-1-7")))] // Don't doctest on GNUStep
#[doc = include_str!("crate_interop.md")]
pub mod crate_interop {}
#[doc = include_str!("cross_compiling.md")]
pub mod cross_compiling {}
#[doc = include_str!("goals.md")]
pub mod goals {}
#[doc = include_str!("kvo.md")]
pub mod kvo {}
#[doc = include_str!("layered_safety.md")]
pub mod layered_safety {}
#[doc = include_str!("migrating_from_objc.md")]
pub mod migrating_from_objc {}
#[doc = include_str!("mvc.md")]
pub mod mvc {}
#[doc = include_str!("interior_mutability.md")]
pub mod interior_mutability {}
#[doc = include_str!("swift.md")]
pub mod swift {}
#[doc = include_str!("weak_property.md")]
pub mod weak_property {} // Referenced by header-translator
#[cfg(not(all(doctest, feature = "gnustep-1-7")))] // Don't doctest on GNUStep
#[doc = include_str!("run_loop.md")]
pub mod run_loop {}

#[cfg(not(doctest))]
#[doc = include_str!("../../CHANGELOG.md")]
pub mod changelog {}

pub mod frameworks_list;
#[doc = include_str!("frameworks_cargo_features.md")]
pub mod frameworks_cargo_features {}
#[doc = include_str!("frameworks_deref.md")]
pub mod frameworks_deref {}
#[cfg(not(doctest))]
#[doc = include_str!("FRAMEWORKS_CHANGELOG.md")]
pub mod frameworks_changelog {}
#[doc = include_str!("frameworks_soundness.md")]
pub mod frameworks_soundness {}
