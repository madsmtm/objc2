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
        let result = Self::wait(self, timeout);

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
        let result = this.0.signal();

        result != 0
    }
}

impl Drop for DispatchSemaphoreGuard {
    fn drop(&mut self) {
        // SAFETY: DispatchSemaphore cannot be null.
        self.0.signal();
    }
}
