#![allow(unused_unsafe, unreachable_patterns)]
#![deny(clippy::undocumented_unsafe_blocks)]

use self::ffi::dispatch_qos_class_t;

pub mod ffi;
pub mod group;
pub mod object;
pub mod queue;
pub mod semaphore;
mod utils;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum WaitError {
    TimeOverflow,
    Timeout,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum QualityOfServiceClass {
    UserInteractive,
    UserInitiated,
    Default,
    Utility,
    Background,
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
