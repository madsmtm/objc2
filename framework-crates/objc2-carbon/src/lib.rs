//! # Bindings to the `Carbon` framework
//!
//! This is currently empty. [Open an issue][new-issue] if you need to use
//! symbols from this framework.
//!
//! [new-issue]: https://github.com/madsmtm/objc2/issues/new
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod generated;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;
