//! # Apple's Dispatch (Grand Central Dispatch)
//!
//! This crate provides a safe and sound interface to Apple's Grand Central
//! dispatch.
//!
//! See [Apple's documentation](https://developer.apple.com/documentation/dispatch)
//! and [the source code for libdispatch](https://github.com/swiftlang/swift-corelibs-libdispatch)
//! for more details.
//!
//! ## Example
//!
//! ```
//! use dispatch2::{DispatchQueue, DispatchQueueAttr};
//!
//! let queue = DispatchQueue::new(Some(c"example_queue"), DispatchQueueAttr::SERIAL);
//! queue.exec_async(|| println!("Hello"));
//! queue.exec_sync(|| println!("World"));
//! ```
#![no_std]
#![allow(unreachable_patterns)]
#![warn(missing_docs)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![warn(clippy::missing_safety_doc)]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/dispatch2/0.3.1")]

#[cfg(not(feature = "alloc"))]
compile_error!("The `alloc` feature currently must be enabled.");

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[macro_use]
mod macros;

mod data;
#[allow(clippy::undocumented_unsafe_blocks, unreachable_pub)]
mod generated;
mod group;
mod io;
#[cfg(feature = "objc2")]
mod main_thread_bound;
mod object;
mod once;
mod queue;
mod retained;
mod semaphore;
mod source;
mod time;
mod timeout_error;
mod utils;
mod workloop;

#[cfg(feature = "block2")]
pub use self::generated::{
    _dispatch_data_destructor_free, _dispatch_data_destructor_munmap, dispatch_block_cancel,
    dispatch_block_create, dispatch_block_create_with_qos_class, dispatch_block_notify,
    dispatch_block_perform, dispatch_block_testcancel, dispatch_block_wait, DispatchBlock,
    DispatchDataApplier, DispatchIOHandler,
};
pub use self::generated::{
    dispatch_allow_send_signals, dispatch_get_specific, DispatchAutoReleaseFrequency,
    DispatchBlockFlags, DispatchData, DispatchGroup, DispatchIO, DispatchQueue, DispatchQueueAttr,
    DispatchSemaphore, DispatchSource, DispatchSourceType, DispatchWorkloop, DISPATCH_API_VERSION,
};
pub use self::group::DispatchGroupGuard;
pub use self::io::{DispatchIOCloseFlags, DispatchIOIntervalFlags, DispatchIOStreamType};
#[cfg(feature = "objc2")]
pub use self::main_thread_bound::{run_on_main, MainThreadBound};
pub use self::object::{
    DispatchObject, DispatchQoS, QualityOfServiceClassFloorError, QOS_MIN_RELATIVE_PRIORITY,
};
pub use self::once::DispatchOnce;
pub use self::queue::{dispatch_main, DispatchQueueGlobalPriority};
pub use self::retained::DispatchRetained;
pub use self::semaphore::DispatchSemaphoreGuard;
pub use self::source::{
    DispatchSourceFileSystemFlags, DispatchSourceMachReceiveFlags, DispatchSourceMachSendFlags,
    DispatchSourceMemoryPressureFlags, DispatchSourceProcessFlags, DispatchSourceTimerFlags,
};
pub use self::time::DispatchTime;
pub use self::timeout_error::DispatchTimeoutError;

pub(crate) use self::io::DispatchFd;
pub(crate) use self::object::__DispatchObject;

#[cfg_attr(target_vendor = "apple", link(name = "System", kind = "dylib"))]
#[cfg_attr(not(target_vendor = "apple"), link(name = "dispatch", kind = "dylib"))]
extern "C" {}

/// The prototype of functions submitted to dispatch queues.
///
/// This is deliberately `extern "C"`, since libdispatch doesn't support
/// unwinding in handler functions, and this gives us better error messages
/// if that does happen.
#[allow(non_camel_case_types)]
pub type DispatchFunction = extern "C" fn(*mut core::ffi::c_void);
