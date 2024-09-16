//! Various explanations and topics of discussion.
#![allow(clippy::needless_doctest_main)]

pub mod about_generated;

#[doc = include_str!("alternatives.md")]
pub mod alternatives {}
#[cfg(not(feature = "gnustep-1-7"))]
#[doc = include_str!("core_foundation_interop.md")]
pub mod core_foundation_interop {}
#[doc = include_str!("kvo.md")]
pub mod kvo {}
#[doc = include_str!("layered_safety.md")]
pub mod layered_safety {}
#[doc = include_str!("mvc.md")]
pub mod mvc {}
#[doc = include_str!("interior_mutability.md")]
pub mod interior_mutability {}
#[cfg(not(feature = "gnustep-1-7"))]
#[doc = include_str!("run_loop.md")]
pub mod run_loop {}

#[cfg(not(doctest))]
#[doc = include_str!("../../CHANGELOG.md")]
pub mod changelog {}
