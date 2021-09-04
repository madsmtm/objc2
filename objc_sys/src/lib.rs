//! # Bindings to the objc_objective-C core runtime
//!
//! # Notable differences
//!
//! Protocol / objc_protocol is no longer a type alias of objc_object, for
//! better type safety. Their internal representation is the same, so the
//! functionality is just a cast away.
//!
//! Deprecated functions are not included since they could be removed at any
//! macOS release, and then our code would break.

// TODO: Replace `extern "C"` with `extern "C-unwind"`.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![doc(html_root_url = "https://docs.rs/objc_sys/0.0.0")]

// TODO: Remove this and add "no-std" category to Cargo.toml
extern crate std;

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
/// `!Sync`, `!Unpin` and as mutable behind shared references. Downstream
/// libraries can always manually opt in to these types afterwards. (It's
/// also less of a breaking change on our part if we re-add these later).
///
/// TODO: Replace this with `extern type` to also mark it as unsized.
type OpaqueData = PhantomData<(UnsafeCell<*const ()>, PhantomPinned)>;
