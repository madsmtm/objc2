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
//! use dispatch2::{Queue, QueueAttribute};
//!
//! let queue = Queue::new("example_queue", QueueAttribute::Serial);
//! queue.exec_async(|| println!("Hello"));
//! queue.exec_sync(|| println!("World"));
//! ```
#![allow(unreachable_patterns)]
#![warn(missing_docs)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![warn(clippy::missing_safety_doc)]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/dispatch2/0.1.0")]

use self::ffi::dispatch_qos_class_t;

pub mod ffi;
mod generated;
pub mod group;
pub mod object;
pub mod queue;
pub mod semaphore;
mod utils;

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

pub use group::*;
pub use object::*;
pub use queue::*;
pub use semaphore::*;
