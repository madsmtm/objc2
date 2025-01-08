//! Dispatch workloop definition.

use alloc::{boxed::Box, ffi::CString};
use core::{ptr::NonNull, time::Duration};

use super::{
    ffi::*, function_wrapper, rc::Retained, AsRawDispatchObject, QualityOfServiceClass,
    QualityOfServiceClassFloorError, TryFromDurationError,
};

/// Dispatch workloop queue.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct WorkloopQueue {
    _inner: [u8; 0],
}

impl WorkloopQueue {
    /// Create a new [WorkloopQueue].
    pub fn new(label: &str, inactive: bool) -> Retained<Self> {
        let label = CString::new(label).expect("Invalid label!");

        // Safety: label can only be valid.
        let object = unsafe {
            if inactive {
                dispatch_workloop_create_inactive(label.as_ptr())
            } else {
                dispatch_workloop_create(label.as_ptr())
            }
        };

        // Safety: object must be valid.
        unsafe { Retained::from_raw(object.cast()) }.expect("dispatch_queue_create failed")
    }

    /// Configure how the [WorkloopQueue] manage the autorelease pools for the functions it executes.
    pub fn set_autorelease_frequency(&self, frequency: DispatchAutoReleaseFrequency) {
        // Safety: object and frequency can only be valid.
        unsafe {
            dispatch_workloop_set_autorelease_frequency(
                self.as_raw(),
                dispatch_autorelease_frequency_t::from(frequency),
            );
        }
    }

    /// Submit a function for synchronous execution on the [WorkloopQueue].
    pub fn exec_sync<F>(&self, work: F)
    where
        F: Send + FnOnce(),
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast();

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe { dispatch_sync_f(self.as_raw().cast(), work_boxed, function_wrapper::<F>) }
    }

    /// Submit a function for asynchronous execution on the [WorkloopQueue].
    pub fn exec_async<F>(&self, work: F)
    where
        F: Send + FnOnce() + 'static,
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast();

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe { dispatch_async_f(self.as_raw().cast(), work_boxed, function_wrapper::<F>) }
    }

    /// Enqueue a function for execution at the specified time on the [WorkloopQueue].
    pub fn after<F>(&self, wait_time: Duration, work: F) -> Result<(), TryFromDurationError>
    where
        F: Send + FnOnce() + 'static,
    {
        let when = dispatch_time_t::try_from(wait_time)?;
        let work_boxed = Box::into_raw(Box::new(work)).cast();

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe {
            dispatch_after_f(
                when,
                self.as_raw().cast(),
                work_boxed,
                function_wrapper::<F>,
            );
        }

        Ok(())
    }

    /// Enqueue a barrier function for asynchronous execution on the [WorkloopQueue] and return immediately.
    pub fn barrier_async<F>(&self, work: F)
    where
        F: Send + FnOnce() + 'static,
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast();

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe { dispatch_barrier_async_f(self.as_raw().cast(), work_boxed, function_wrapper::<F>) }
    }

    /// Enqueue a barrier function for synchronous execution on the [WorkloopQueue] and wait until that function completes.
    pub fn barrier_sync<F>(&self, work: F)
    where
        F: Send + FnOnce(),
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast();

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe { dispatch_barrier_sync_f(self.as_raw().cast(), work_boxed, function_wrapper::<F>) }
    }

    /// Submit a function for synchronous execution and mark the function as a barrier for subsequent concurrent tasks.
    pub fn barrier_async_and_wait<F>(&self, work: F)
    where
        F: Send + FnOnce(),
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast();

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe {
            dispatch_barrier_async_and_wait_f(
                self.as_raw().cast(),
                work_boxed,
                function_wrapper::<F>,
            )
        }
    }

    /// Sets a function at the given key that will be executed at [WorkloopQueue] destruction.
    pub fn set_specific<F>(&mut self, key: NonNull<()>, destructor: F)
    where
        F: Send + FnOnce(),
    {
        let destructor_boxed = Box::into_raw(Box::new(destructor)).cast();

        // SAFETY: object cannot be null and destructor is wrapped to avoid
        // ABI incompatibility.
        //
        // The key is never dereferenced, so passing _any_ pointer here is
        // safe and allowed.
        unsafe {
            dispatch_queue_set_specific(
                self.as_raw().cast(),
                key.cast(),
                destructor_boxed,
                function_wrapper::<F>,
            )
        }
    }

    /// Set the QOS class floor of the [WorkloopQueue].
    pub fn set_qos_class_floor(
        &self,
        qos_class: QualityOfServiceClass,
        relative_priority: i32,
    ) -> Result<(), QualityOfServiceClassFloorError> {
        if !(QOS_MIN_RELATIVE_PRIORITY..=0).contains(&relative_priority) {
            return Err(QualityOfServiceClassFloorError::InvalidRelativePriority);
        }

        // SAFETY: Safe as relative_priority can only be valid.
        unsafe {
            dispatch_set_qos_class_floor(
                self.as_raw_object(),
                dispatch_qos_class_t::from(qos_class),
                relative_priority,
            );
        }

        Ok(())
    }

    /// Get the raw [dispatch_workloop_t] value.
    ///
    /// # Safety
    ///
    /// - Object shouldn't be released manually.
    pub fn as_raw(&self) -> dispatch_workloop_t {
        self as *const Self as _
    }
}

impl AsRawDispatchObject for WorkloopQueue {
    fn as_raw_object(&self) -> dispatch_object_t {
        self.as_raw().cast()
    }
}

// Safety: it's safe to move workloop queue between threads.
unsafe impl Send for WorkloopQueue {}

// Safety: it's safe to share workloop queue between threads.
unsafe impl Sync for WorkloopQueue {}

/// Auto release frequency for [WorkloopQueue::set_autorelease_frequency].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum DispatchAutoReleaseFrequency {
    /// Inherit autorelease frequency from the target [crate::Queue].
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
