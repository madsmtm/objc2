use alloc::ffi::CString;
use core::{borrow::Borrow, ops::Deref, ptr::NonNull};

use crate::{ffi::*, DispatchQueue, DispatchRetained};

dispatch_object!(
    /// Dispatch workloop queue.
    pub struct DispatchWorkloop;
);

dispatch_object_not_data!(unsafe DispatchWorkloop);

impl DispatchWorkloop {
    /// Create a new [`DispatchWorkloop`].
    pub fn new(label: &str, inactive: bool) -> DispatchRetained<Self> {
        let label = CString::new(label).expect("Invalid label!");

        // Safety: label can only be valid.
        let object = unsafe {
            if inactive {
                dispatch_workloop_create_inactive(label.as_ptr())
            } else {
                dispatch_workloop_create(label.as_ptr())
            }
        };

        let object = NonNull::new(object).expect("dispatch_workloop_create returned NULL");
        // SAFETY: Object came from a "create" method.
        unsafe { DispatchRetained::from_raw(object.cast()) }
    }

    /// Configure how the [`DispatchWorkloop`] manage the autorelease pools for the functions it executes.
    pub fn set_autorelease_frequency(&self, frequency: DispatchAutoReleaseFrequency) {
        // Safety: object and frequency can only be valid.
        unsafe {
            dispatch_workloop_set_autorelease_frequency(
                self.as_raw(),
                dispatch_autorelease_frequency_t::from(frequency),
            );
        }
    }

    /// Get the raw [dispatch_workloop_t] value.
    ///
    /// # Safety
    ///
    /// - Object shouldn't be released manually.
    pub const unsafe fn as_raw(&self) -> dispatch_workloop_t {
        let ptr: *const Self = self;
        ptr as dispatch_workloop_t
    }
}

impl Deref for DispatchWorkloop {
    type Target = DispatchQueue;

    /// Access the workloop as a [`DispatchQueue`].
    #[inline]
    fn deref(&self) -> &Self::Target {
        let ptr: *const DispatchWorkloop = self;
        let ptr: *const DispatchQueue = ptr.cast();
        // SAFETY: Workloop queues are "subclasses" of queues (they can be
        // used in all the same places that normal queues can).
        unsafe { &*ptr }
    }
}

impl AsRef<DispatchQueue> for DispatchWorkloop {
    #[inline]
    fn as_ref(&self) -> &DispatchQueue {
        self
    }
}

// PartialEq, Eq and Hash work the same for workloops and queues.
impl Borrow<DispatchQueue> for DispatchWorkloop {
    #[inline]
    fn borrow(&self) -> &DispatchQueue {
        self
    }
}

/// Auto release frequency for [`DispatchWorkloop::set_autorelease_frequency`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum DispatchAutoReleaseFrequency {
    /// Inherit autorelease frequency from the target [`DispatchQueue`].
    Inherit,
    /// Configure an autorelease pool before the execution of a function and releases the objects in that pool after the function finishes executing.
    WorkItem,
    /// Never setup an autorelease pool.
    Never,
}

impl From<DispatchAutoReleaseFrequency> for dispatch_autorelease_frequency_t {
    fn from(value: DispatchAutoReleaseFrequency) -> Self {
        match value {
            DispatchAutoReleaseFrequency::Inherit => {
                dispatch_autorelease_frequency_t::DISPATCH_AUTORELEASE_FREQUENCY_INHERIT
            }
            DispatchAutoReleaseFrequency::WorkItem => {
                dispatch_autorelease_frequency_t::DISPATCH_AUTORELEASE_FREQUENCY_WORK_ITEM
            }
            DispatchAutoReleaseFrequency::Never => {
                dispatch_autorelease_frequency_t::DISPATCH_AUTORELEASE_FREQUENCY_NEVER
            }
            _ => panic!("Unknown DispatchAutoReleaseFrequency value: {:?}", value),
        }
    }
}
