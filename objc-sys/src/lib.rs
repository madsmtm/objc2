//! # Raw bindings to Objective-C runtimes
//!
//! These bindings contain almost no documentation, so it is highly
//! recommended to read the documentation of the original libraries:
//! - Apple's [official documentation][apple].
//! - Apple's `objc4` [source code][objc4] ([`git` mirror][objc4-mirror]), in
//!   particular `runtime.h`.
//! - GNUStep's `libobjc2` [source code][libobjc2], in particular `runtime.h`.
//!
//! See also the [`README.md`](https://crates.io/crates/objc-sys) for more
//! background information, and for how to configure the desired runtime.
//!
//! [apple]: https://developer.apple.com/documentation/objectivec/objective-c_runtime?language=objc
//! [libobjc2]: https://github.com/gnustep/libobjc2/tree/v2.1/objc
//! [objc4]: https://opensource.apple.com/source/objc4/objc4-818.2/runtime/
//! [objc4-mirror]: https://github.com/madsmtm/objc4-mirror.git

#![no_std]
#![allow(clippy::upper_case_acronyms)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![doc(html_root_url = "https://docs.rs/objc-sys/0.2.0-alpha.0")]

// TODO: Replace `extern "C"` with `extern "C-unwind"` where applicable.
// See https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html.

// TODO: Remove this and add "no-std" category to Cargo.toml
// Requires a better solution for C-types in `no_std` crates.
// See https://github.com/japaric/cty/issues/14.
extern crate std;

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
extern "C" {}

use core::cell::UnsafeCell;
use core::marker::{PhantomData, PhantomPinned};

mod class;
mod constants;
mod exception;
mod message;
mod method;
mod object;
mod property;
mod protocol;
mod rc;
mod selector;
mod types;
mod various;

pub use class::*;
pub use constants::*;
pub use exception::*;
pub use message::*;
pub use method::*;
pub use object::*;
pub use property::*;
pub use protocol::*;
pub use rc::*;
pub use selector::*;
pub use types::*;
pub use various::*;

/// We don't know much about the actual structs, so better mark them `!Send`,
/// `!Sync`, `!UnwindSafe`, `!RefUnwindSafe`, `!Unpin` and as mutable behind
/// shared references.
///
/// Downstream libraries can always manually opt in to these types afterwards.
/// (It's also less of a breaking change on our part if we re-add these).
///
/// TODO: Replace this with `extern type` to also mark it as `!Sized`.
type OpaqueData = PhantomData<(UnsafeCell<()>, *const UnsafeCell<()>, PhantomPinned)>;
