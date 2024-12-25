//! Dispatch semaphore definition.

use core::time::Duration;

use super::ffi::*;
use super::object::DispatchObject;
use super::WaitError;

/// Dispatch semaphore.
#[derive(Debug, Clone)]
pub struct Semaphore {
    dispatch_object: DispatchObject<dispatch_semaphore_s>,
}

impl Semaphore {
    /// Creates a new [Semaphore] with an initial value.
    ///
    /// Returns None if value is negative or if creation failed.
    pub fn new(value: isize) -> Option<Self> {
        // Per documentation creating a semaphore with a negative size isn't allowed.
        if value < 0 {
            return None;
        }

        // Safety: value is valid
        let object = unsafe { dispatch_semaphore_create(value) };

        if object.is_null() {
            return None;
        }

        // Safety: object cannot be null.
        let dispatch_object = unsafe { DispatchObject::new_owned(object.cast()) };

        Some(Semaphore { dispatch_object })
    }

    /// Attempt to acquire the [Semaphore] and return a [SemaphoreGuard].
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

        // Safety: Semaphore cannot be null.
        let result = unsafe { dispatch_semaphore_wait(self.as_raw(), timeout) };

        match result {
            0 => Ok(SemaphoreGuard(self.clone(), false)),
            _ => Err(WaitError::Timeout),
        }
    }

    /// Set the finalizer function for the object.
    pub fn set_finalizer<F>(&mut self, destructor: F)
    where
        F: Send + FnOnce(),
    {
        self.dispatch_object.set_finalizer(destructor);
    }

    /// Get the raw [dispatch_semaphore_t] value.
    ///
    /// # Safety
    ///
    /// - Object shouldn't be released manually.
    pub const unsafe fn as_raw(&self) -> dispatch_semaphore_t {
        // SAFETY: Upheld by caller.
        unsafe { self.dispatch_object.as_raw() }
    }
}

/// Dispatch semaphore guard.
#[derive(Debug)]
pub struct SemaphoreGuard(Semaphore, bool);

impl SemaphoreGuard {
    /// Release the [Semaphore].
    pub fn release(mut self) -> bool {
        // Safety: Semaphore cannot be null.
        let result = unsafe { dispatch_semaphore_signal(self.0.as_raw()) };

        self.1 = true;

        result != 0
    }
}

impl Drop for SemaphoreGuard {
    fn drop(&mut self) {
        if !self.1 {
            // Safety: Semaphore cannot be null.
            unsafe {
                dispatch_semaphore_signal(self.0.as_raw());
            }

            self.1 = true;
        }
    }
}
