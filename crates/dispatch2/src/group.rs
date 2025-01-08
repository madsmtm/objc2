//! Dispatch group definition.

use alloc::boxed::Box;
use core::{ffi::c_void, time::Duration};

use super::{ffi::*, function_wrapper, queue::Queue, rc::Retained, AsRawDispatchObject, WaitError};

/// Dispatch group.
#[derive(Debug, Clone, Copy)]
pub struct Group {
    _inner: [u8; 0],
}

impl Group {
    /// Creates a new [Group].
    pub fn new() -> Option<Retained<Self>> {
        // Safety: valid to call.
        let object = unsafe { dispatch_group_create() };
        assert!(!object.is_null());

        // Safety: object must be valid.
        unsafe { Retained::from_raw(object.cast()) }
    }

    /// Submit a function to a [Queue] and associates it with the [Group].
    pub fn exec_async<F>(&self, queue: &Queue, work: F)
    where
        F: Send + FnOnce(),
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast::<c_void>();

        // Safety: All parameters cannot be null.
        unsafe {
            dispatch_group_async_f(
                self.as_raw(),
                queue.as_raw(),
                work_boxed,
                function_wrapper::<F>,
            );
        }
    }

    /// Wait synchronously for the previously submitted functions to finish.
    ///
    /// # Errors
    ///
    /// Return [WaitError::TimeOverflow] if the passed ``timeout`` is too big.
    ///
    /// Return [WaitError::Timeout] in case of timeout.
    pub fn wait(&self, timeout: Option<Duration>) -> Result<(), WaitError> {
        let timeout = if let Some(timeout) = timeout {
            dispatch_time_t::try_from(timeout).map_err(|_| WaitError::TimeOverflow)?
        } else {
            DISPATCH_TIME_FOREVER
        };

        // Safety: object cannot be null and timeout is valid.
        let result = unsafe { dispatch_group_wait(self.as_raw(), timeout) };

        match result {
            0 => Ok(()),
            _ => Err(WaitError::Timeout),
        }
    }

    /// Schedule a function to be submitted to a [Queue] when a group of previously submitted functions have completed.
    pub fn notify<F>(&self, queue: &Queue, work: F)
    where
        F: Send + FnOnce(),
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast::<c_void>();

        // Safety: All parameters cannot be null.
        unsafe {
            dispatch_group_notify_f(
                self.as_raw(),
                queue.as_raw(),
                work_boxed,
                function_wrapper::<F>,
            );
        }
    }

    /// Explicitly indicates that the function has entered the [Group].
    pub fn enter(&self) -> GroupGuard {
        // Safety: object cannot be null.
        unsafe {
            dispatch_group_enter(self.as_raw());
        }

        let group =
            // Safety: group cannot be null.
            unsafe { Retained::retain(self.as_raw().cast()) }.expect("failed to retain semaphore");

        GroupGuard(group, false)
    }

    /// Get the raw [dispatch_group_t] value.
    ///
    /// # Safety
    ///
    /// - Object shouldn't be released manually.
    pub fn as_raw(&self) -> dispatch_group_t {
        self as *const Self as _
    }
}

impl AsRawDispatchObject for Group {
    fn as_raw_object(&self) -> dispatch_object_t {
        self.as_raw().cast()
    }
}

// Safety: group is inherently safe to move between threads.
unsafe impl Send for Group {}

// Safety: group is inherently safe to share between threads.
unsafe impl Sync for Group {}

/// Dispatch group guard.
#[derive(Debug)]
pub struct GroupGuard(Retained<Group>, bool);

impl GroupGuard {
    /// Explicitly indicates that the function in the [Group] finished executing.
    pub fn leave(mut self) {
        // Safety: object cannot be null.
        unsafe {
            dispatch_group_leave(self.0.as_raw());
        }

        self.1 = true;
    }
}

impl Drop for GroupGuard {
    fn drop(&mut self) {
        if !self.1 {
            // Safety: object cannot be null.
            unsafe {
                dispatch_group_leave(self.0.as_raw());
            }

            self.1 = true;
        }
    }
}
