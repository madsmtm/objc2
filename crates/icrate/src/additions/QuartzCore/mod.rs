//! # Bindings to the `CoreAnimation` framework
//!
//! This actually lives in the `QuartzCore` framework, but `CoreAnimation` is
//! the name that people use to refer to it.
#![doc(alias = "CoreAnimation")]

pub use crate::generated::QuartzCore::*;

// CoreFoundation
pub(crate) type CFTimeInterval = std::os::raw::c_double;
