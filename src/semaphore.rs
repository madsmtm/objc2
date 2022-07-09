use std::time::Duration;

use super::ffi::*;
use super::object::DispatchObject;
use super::WaitError;

#[derive(Debug, Clone)]
pub struct Semaphore {
    dispatch_object: DispatchObject<dispatch_semaphore_s>,
}

impl Semaphore {
    pub fn new(value: isize) -> Option<Self> {
        // Per documentation creating a semaphore with a negative size isn't allowed.
        if value < 0 {
            return None;
        }

        let object = unsafe { dispatch_semaphore_create(value) };

        if object.is_null() {
            return None;
        }

        let dispatch_object = unsafe {
            // Safety: object cannot be null.
            DispatchObject::new_owned(object as *mut _)
        };

        Some(Semaphore { dispatch_object })
    }

    pub fn try_acquire(&self, timeout: Option<Duration>) -> Result<SemaphoreGuard, WaitError> {
        let timeout = if let Some(timeout) = timeout {
            dispatch_time_t::try_from(timeout).map_err(|_| WaitError::TimeOverflow)?
        } else {
            DISPATCH_TIME_FOREVER
        };

        let result = unsafe { dispatch_semaphore_wait(self.as_raw(), timeout) };

        match result {
            0 => Ok(SemaphoreGuard(self.clone(), false)),
            _ => Err(WaitError::Timeout),
        }
    }

    pub fn set_finalizer<F>(&mut self, destructor: F)
    where
        F: Send + FnOnce(),
    {
        self.dispatch_object.set_finalizer(destructor);
    }

    pub const fn as_raw(&self) -> dispatch_semaphore_t {
        self.dispatch_object.as_raw()
    }
}

#[derive(Debug)]
pub struct SemaphoreGuard(Semaphore, bool);

impl SemaphoreGuard {
    pub fn release(mut self) -> bool {
        let result = unsafe { dispatch_semaphore_signal(self.0.as_raw()) };

        self.1 = true;

        result != 0
    }
}

impl Drop for SemaphoreGuard {
    fn drop(&mut self) {
        if !self.1 {
            unsafe {
                dispatch_semaphore_signal(self.0.as_raw());
            }

            self.1 = true;
        }
    }
}
