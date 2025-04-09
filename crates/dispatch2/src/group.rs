use alloc::boxed::Box;
use core::ffi::c_void;
use core::time::Duration;

use crate::{DispatchObject, DispatchQueue, DispatchRetained};

use super::utils::function_wrapper;
use super::{ffi::*, WaitError};

dispatch_object!(
    /// Dispatch group.
    #[doc(alias = "dispatch_group_t")]
    #[doc(alias = "dispatch_group_s")]
    pub struct DispatchGroup;
);

dispatch_object_not_data!(unsafe DispatchGroup);

impl DispatchGroup {
    /// Creates a new [`DispatchGroup`].
    pub fn new() -> Option<DispatchRetained<Self>> {
        // SAFETY: Valid to call.
        // TODO: Properly allow NULL again (`dispatch_group_create` is incorrectly mapped).
        Some(unsafe { dispatch_group_create() })
    }

    /// Submit a function to a [`DispatchQueue`] and associates it with the [`DispatchGroup`].
    pub fn exec_async<F>(&self, queue: &DispatchQueue, work: F)
    where
        // We need `'static` to make sure any referenced values are borrowed for
        // long enough since `work` will be performed asynchronously.
        F: Send + FnOnce() + 'static,
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast::<c_void>();

        // Safety: All parameters cannot be null.
        unsafe {
            dispatch_group_async_f(self, queue, work_boxed, function_wrapper::<F>);
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
        let result = unsafe { dispatch_group_wait(self, timeout) };

        match result {
            0 => Ok(()),
            _ => Err(WaitError::Timeout),
        }
    }

    /// Schedule a function to be submitted to a [`DispatchQueue`] when a group of previously submitted functions have completed.
    pub fn notify<F>(&self, queue: &DispatchQueue, work: F)
    where
        F: Send + FnOnce(),
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast::<c_void>();

        // Safety: All parameters cannot be null.
        unsafe {
            dispatch_group_notify_f(self, queue, work_boxed, function_wrapper::<F>);
        }
    }

    /// Explicitly indicates that the function has entered the [`DispatchGroup`].
    pub fn enter(&self) -> DispatchGroupGuard {
        // Safety: object cannot be null.
        unsafe { dispatch_group_enter(self) };

        DispatchGroupGuard(self.retain())
    }
}

/// Dispatch group guard.
#[derive(Debug)]
pub struct DispatchGroupGuard(DispatchRetained<DispatchGroup>);

impl DispatchGroupGuard {
    /// Explicitly indicate that the function in the [`DispatchGroup`] finished executing.
    pub fn leave(self) {
        // Drop.
        let _ = self;
    }
}

impl Drop for DispatchGroupGuard {
    fn drop(&mut self) {
        // SAFETY: Dispatch group cannot be null.
        unsafe { dispatch_group_leave(&self.0) };
    }
}
