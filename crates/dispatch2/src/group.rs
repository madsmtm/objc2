//! Dispatch group definition.

use alloc::boxed::Box;
use core::ffi::c_void;
use core::time::Duration;

use super::object::DispatchObject;
use super::queue::Queue;
use super::utils::function_wrapper;
use super::{ffi::*, WaitError};

/// Dispatch group.
#[derive(Debug, Clone)]
pub struct Group {
    dispatch_object: DispatchObject<dispatch_group_s>,
}

/// Dispatch group guard.
#[derive(Debug)]
pub struct GroupGuard(Group, bool);

impl Group {
    /// Creates a new [Group].
    pub fn new() -> Option<Self> {
        // Safety: valid to call.
        let object = unsafe { dispatch_group_create() };

        if object.is_null() {
            return None;
        }

        // Safety: object cannot be null.
        let dispatch_object = unsafe { DispatchObject::new_owned(object.cast()) };

        Some(Group { dispatch_object })
    }

    /// Submit a function to a [Queue] and associates it with the [Group].
    pub fn exec_async<F>(&self, queue: &Queue, work: F)
    where
        // We need `'static` to make sure any referenced values are borrowed for
        // long enough since `work` will be performed asynchronously.
        F: Send + FnOnce() + 'static,
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

        GroupGuard(self.clone(), false)
    }

    /// Set the finalizer function for the object.
    pub fn set_finalizer<F>(&mut self, destructor: F)
    where
        F: Send + FnOnce(),
    {
        self.dispatch_object.set_finalizer(destructor);
    }

    /// Get the raw [dispatch_group_t] value.
    ///
    /// # Safety
    ///
    /// - Object shouldn't be released manually.
    pub const unsafe fn as_raw(&self) -> dispatch_group_t {
        // SAFETY: Upheld by caller
        unsafe { self.dispatch_object.as_raw() }
    }
}

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
