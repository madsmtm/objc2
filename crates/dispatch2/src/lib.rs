//! # Apple's Dispatch (Grand Central Dispatch)
//!
//! This crate provides a safe and sound interface to Apple's Grand Central
//! dispatch, as well as the ability to drop into lower-level bindings
//! ([`ffi`] module).
//!
//! See [Apple's documentation](https://developer.apple.com/documentation/dispatch)
//! and [the source code for libdispatch](https://github.com/swiftlang/swift-corelibs-libdispatch)
//! for more details.
//!
//! ## Example
//!
//! ```
//! use dispatch2::{DispatchQueue, QueueAttribute};
//!
//! let queue = DispatchQueue::new("example_queue", QueueAttribute::Serial);
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
#![doc(html_root_url = "https://docs.rs/dispatch2/0.2.0")]

#[cfg(not(feature = "alloc"))]
compile_error!("The `alloc` feature currently must be enabled.");

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[macro_use]
mod macros;

use core::cell::UnsafeCell;
use core::marker::{PhantomData, PhantomPinned};

use self::ffi::dispatch_qos_class_t;

pub mod ffi;
#[allow(clippy::undocumented_unsafe_blocks)]
mod generated;
mod group;
#[cfg(feature = "objc2")]
mod main_thread_bound;
mod object;
mod once;
mod queue;
mod retained;
mod semaphore;
mod utils;
mod workloop;

/// Wait error.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum WaitError {
    /// The given timeout value will result in an overflow when converting to dispatch time.
    TimeOverflow,
    /// The operation timed out.
    Timeout,
}

/// Quality of service that specify the priorities for executing tasks.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum QualityOfServiceClass {
    /// Quality of service for user-interactive tasks.
    UserInteractive,
    /// Quality of service for tasks that prevent the user from actively using your app.
    UserInitiated,
    /// Default Quality of service.
    Default,
    /// Quality of service for tasks that the user does not track actively.
    Utility,
    /// Quality of service for maintenance or cleanup tasks.
    Background,
    /// The absence of a Quality of service.
    Unspecified,
}

impl From<QualityOfServiceClass> for dispatch_qos_class_t {
    fn from(value: QualityOfServiceClass) -> Self {
        match value {
            QualityOfServiceClass::UserInteractive => {
                dispatch_qos_class_t::QOS_CLASS_USER_INTERACTIVE
            }
            QualityOfServiceClass::UserInitiated => dispatch_qos_class_t::QOS_CLASS_USER_INITIATED,
            QualityOfServiceClass::Default => dispatch_qos_class_t::QOS_CLASS_DEFAULT,
            QualityOfServiceClass::Utility => dispatch_qos_class_t::QOS_CLASS_UTILITY,
            QualityOfServiceClass::Background => dispatch_qos_class_t::QOS_CLASS_BACKGROUND,
            QualityOfServiceClass::Unspecified => dispatch_qos_class_t::QOS_CLASS_UNSPECIFIED,
            _ => panic!("Unknown QualityOfServiceClass value: {:?}", value),
        }
    }
}

pub use self::group::{DispatchGroup, DispatchGroupGuard};
#[cfg(feature = "objc2")]
pub use self::main_thread_bound::{run_on_main, MainThreadBound};
pub use self::object::{DispatchObject, QualityOfServiceClassFloorError};
pub use self::once::DispatchOnce;
pub use self::queue::{
    DispatchQueue, GlobalQueueIdentifier, QueueAfterError, QueueAttribute, QueuePriority,
};
pub use self::retained::DispatchRetained;
pub use self::semaphore::{DispatchSemaphore, DispatchSemaphoreGuard};
pub use self::workloop::{DispatchAutoReleaseFrequency, DispatchWorkloop};

// Helper type
type OpaqueData = UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>;

/// Deprecated alias for [`DispatchGroup`].
#[deprecated = "renamed to DispatchGroup"]
pub type Group = DispatchGroup;

/// Deprecated alias for [`DispatchOnce`].
#[deprecated = "renamed to DispatchOnce"]
pub type Once = DispatchOnce;

/// Deprecated alias for [`DispatchQueue`].
#[deprecated = "renamed to DispatchQueue"]
pub type Queue = DispatchQueue;

/// Deprecated alias for [`DispatchSemaphore`].
#[deprecated = "renamed to DispatchSemaphore"]
pub type Semaphore = DispatchSemaphore;

/// Deprecated alias for [`DispatchWorkloop`].
#[deprecated = "renamed to DispatchWorkloop"]
pub type WorkloopQueue = DispatchWorkloop;
