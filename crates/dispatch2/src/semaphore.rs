//! Dispatch semaphore definition.

use core::ptr::NonNull;
use core::time::Duration;

use crate::{DispatchObject, DispatchRetained};

use super::ffi::*;
use super::WaitError;

dispatch_object!(
    /// Dispatch semaphore.
    pub struct DispatchSemaphore;
);

impl DispatchSemaphore {
    /// Creates a new [`DispatchSemaphore`] with an initial value.
    ///
    /// Returns None if value is negative or if creation failed.
    pub fn new(value: isize) -> Option<DispatchRetained<Self>> {
        // Per documentation creating a semaphore with a negative size isn't allowed.
        if value < 0 {
            return None;
        }

        // SAFETY: The value is valid.
        let ptr = unsafe { dispatch_semaphore_create(value) };
        NonNull::new(ptr).map(|ptr| {
            // SAFETY: The object came from a "create" method.
            unsafe { DispatchRetained::from_raw(ptr.cast()) }
        })
    }

    /// Attempt to acquire the [`DispatchSemaphore`] and return a [`SemaphoreGuard`].
    ///
    /// # Errors
    ///
    /// Return [WaitError::TimeOverflow] if the passed ``timeout`` is too big.
    ///
    /// Return [WaitError::Timeout] in case of timeout.
    pub fn try_acquire(&self, timeout: Option<Duration>) -> Result<SemaphoreGuard, WaitError> {
        let timeout = if let Some(timeout) = timeout {
            dispatch_time_t::try_from(timeout).map_err(|_| WaitError::TimeOverflow)?
        } else {
            DISPATCH_TIME_FOREVER
        };

        // Safety: DispatchSemaphore cannot be null.
        let result = unsafe { dispatch_semaphore_wait(self.as_raw(), timeout) };

        match result {
            0 => Ok(SemaphoreGuard(self.retain(), false)),
            _ => Err(WaitError::Timeout),
        }
    }

    /// Get the raw [dispatch_semaphore_t] value.
    ///
    /// # Safety
    ///
    /// - Object shouldn't be released manually.
    pub const unsafe fn as_raw(&self) -> dispatch_semaphore_t {
        let ptr: *const Self = self;
        ptr as dispatch_semaphore_t
    }
}

/// Dispatch semaphore guard.
#[derive(Debug)]
pub struct SemaphoreGuard(DispatchRetained<DispatchSemaphore>, bool);

impl SemaphoreGuard {
    /// Release the [`DispatchSemaphore`].
    pub fn release(mut self) -> bool {
        // Safety: DispatchSemaphore cannot be null.
        let result = unsafe { dispatch_semaphore_signal(self.0.as_raw()) };

        self.1 = true;

        result != 0
    }
}

impl Drop for SemaphoreGuard {
    fn drop(&mut self) {
        if !self.1 {
            // Safety: DispatchSemaphore cannot be null.
            unsafe {
                dispatch_semaphore_signal(self.0.as_raw());
            }

            self.1 = true;
        }
    }
}
