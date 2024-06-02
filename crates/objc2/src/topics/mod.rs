//! Various explanations and topics of discussion.

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

#[cfg(not(doctest))]
#[doc = include_str!("../../CHANGELOG.md")]
pub mod changelog {}
