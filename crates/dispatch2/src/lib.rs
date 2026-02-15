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
#![warn(missing_debug_implementations)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![warn(clippy::missing_safety_doc)]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/dispatch2/0.3.0")]

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

pub use self::generated::{
    dispatch_allow_send_signals, dispatch_fd_t, dispatch_get_specific, dispatch_once_t,
    DispatchAutoReleaseFrequency, _dispatch_source_type_data_add, _dispatch_source_type_data_or,
    _dispatch_source_type_data_replace, _dispatch_source_type_mach_recv,
    _dispatch_source_type_mach_send, _dispatch_source_type_memorypressure,
    _dispatch_source_type_proc, _dispatch_source_type_read, _dispatch_source_type_signal,
    _dispatch_source_type_timer, _dispatch_source_type_vnode, _dispatch_source_type_write,
    DISPATCH_API_VERSION,
};
#[cfg(feature = "block2")]
pub use self::generated::{
    DispatchData, _dispatch_data_destructor_free, _dispatch_data_destructor_munmap,
    dispatch_block_cancel, dispatch_block_create, dispatch_block_create_with_qos_class,
    dispatch_block_notify, dispatch_block_perform, dispatch_block_t, dispatch_block_testcancel,
    dispatch_block_wait, dispatch_data_applier_t, dispatch_io_handler_t, dispatch_read,
    dispatch_write, DispatchGroup, DispatchIO, DispatchQueue, DispatchQueueAttr, DispatchSemaphore,
    DispatchSource, DispatchWorkloop,
};
pub use self::group::DispatchGroupGuard;
pub use self::io::{DispatchIOCloseFlags, DispatchIOIntervalFlags, DispatchIOStreamType};
#[cfg(feature = "objc2")]
pub use self::main_thread_bound::{run_on_main, MainThreadBound};
pub(crate) use self::object::dispatch_object_s;
pub use self::object::{
    DispatchObject, DispatchQoS, QualityOfServiceClassFloorError, QOS_MIN_RELATIVE_PRIORITY,
};
pub use self::once::DispatchOnce;
pub use self::queue::{dispatch_main, DispatchQueueGlobalPriority, GlobalQueueIdentifier};
pub use self::retained::DispatchRetained;
pub use self::semaphore::DispatchSemaphoreGuard;
pub use self::source::{
    dispatch_source_mach_recv_flags_t, dispatch_source_mach_send_flags_t,
    dispatch_source_memorypressure_flags_t, dispatch_source_proc_flags_t,
    dispatch_source_timer_flags_t, dispatch_source_type_s, dispatch_source_type_t,
    dispatch_source_vnode_flags_t,
};
pub use self::time::DispatchTime;
pub use self::timeout_error::DispatchTimeoutError;

#[cfg_attr(target_vendor = "apple", link(name = "System", kind = "dylib"))]
#[cfg_attr(not(target_vendor = "apple"), link(name = "dispatch", kind = "dylib"))]
extern "C" {}

/// The prototype of functions submitted to dispatch queues.
///
/// This is deliberately `extern "C"`, since libdispatch doesn't support
/// unwinding in handler functions, and this gives us better error messages
/// if that does happen.
#[allow(non_camel_case_types)]
pub type dispatch_function_t = extern "C" fn(*mut core::ffi::c_void);
