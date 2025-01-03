//! Dispatch semaphore definition.

use core::time::Duration;

use super::{ffi::*, rc::Retained, AsRawDispatchObject, WaitError};

/// Dispatch semaphore.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Semaphore {
    _inner: [u8; 0],
}

impl Semaphore {
    /// Creates a new [Semaphore] with an initial value.
    ///
    /// Returns None if value is negative or if creation failed.
    pub fn new(value: isize) -> Option<Retained<Self>> {
        // Per documentation creating a semaphore with a negative size isn't allowed.
        if value < 0 {
            return None;
        }

        // Safety: value is valid
        let object = unsafe { dispatch_semaphore_create(value) };

        // Safety: retained accepts null pointer.
        unsafe { Retained::from_raw(object.cast()) }
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

        let sema =
            // Safety: semaphore cannot be null.
            unsafe { Retained::retain(self.as_raw().cast()) }.expect("failed to retain semaphore");

        match result {
            0 => Ok(SemaphoreGuard(sema, false)),
            _ => Err(WaitError::Timeout),
        }
    }

    /// Get the raw [dispatch_semaphore_t] value.
    ///
    /// # Safety
    ///
    /// - Object shouldn't be released manually.
    pub fn as_raw(&self) -> dispatch_semaphore_t {
        self as *const Self as _
    }
}

impl AsRawDispatchObject for Semaphore {
    fn as_raw_object(&self) -> dispatch_object_t {
        self.as_raw().cast()
    }
}

// Safety: semaphore is inherently safe to move between threads.
unsafe impl Send for Semaphore {}

// Safety: semaphore is inherently safe to share between threads.
unsafe impl Sync for Semaphore {}

/// Dispatch semaphore guard.
#[derive(Debug)]
pub struct SemaphoreGuard(Retained<Semaphore>, bool);

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
