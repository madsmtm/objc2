use core::mem::ManuallyDrop;
use core::time::Duration;

use crate::{DispatchObject, DispatchRetained};

use super::ffi::*;
use super::WaitError;

dispatch_object!(
    /// Dispatch semaphore.
    #[doc(alias = "dispatch_semaphore_t")]
    #[doc(alias = "dispatch_semaphore_s")]
    pub struct DispatchSemaphore;
);

dispatch_object_not_data!(unsafe DispatchSemaphore);

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
        // TODO: Allow this to be NULL.
        Some(unsafe { dispatch_semaphore_create(value) })
    }

    /// Attempt to acquire the [`DispatchSemaphore`] and return a [`DispatchSemaphoreGuard`].
    ///
    /// # Errors
    ///
    /// Return [WaitError::TimeOverflow] if the passed ``timeout`` is too big.
    ///
    /// Return [WaitError::Timeout] in case of timeout.
    pub fn try_acquire(
        &self,
        timeout: Option<Duration>,
    ) -> Result<DispatchSemaphoreGuard, WaitError> {
        let timeout = if let Some(timeout) = timeout {
            dispatch_time_t::try_from(timeout).map_err(|_| WaitError::TimeOverflow)?
        } else {
            DISPATCH_TIME_FOREVER
        };

        // Safety: DispatchSemaphore cannot be null.
        let result = unsafe { Self::wait(self, timeout) };

        match result {
            0 => Ok(DispatchSemaphoreGuard(self.retain())),
            _ => Err(WaitError::Timeout),
        }
    }
}

/// Dispatch semaphore guard.
#[derive(Debug)]
pub struct DispatchSemaphoreGuard(DispatchRetained<DispatchSemaphore>);

impl DispatchSemaphoreGuard {
    /// Release the [`DispatchSemaphore`].
    pub fn release(self) -> bool {
        let this = ManuallyDrop::new(self);

        // SAFETY: DispatchSemaphore cannot be null.
        let result = unsafe { DispatchSemaphore::signal(&this.0) };

        result != 0
    }
}

impl Drop for DispatchSemaphoreGuard {
    fn drop(&mut self) {
        // SAFETY: DispatchSemaphore cannot be null.
        unsafe { DispatchSemaphore::signal(&self.0) };
    }
}
