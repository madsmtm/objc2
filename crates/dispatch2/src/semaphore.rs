use core::mem::ManuallyDrop;
use core::num::NonZeroIsize;

use crate::{DispatchSemaphore, DispatchTime, DispatchTimeoutError};

impl DispatchSemaphore {
    /// Attempt to acquire the [`DispatchSemaphore`] and return a [`DispatchSemaphoreGuard`].
    ///
    /// # Errors
    ///
    /// Returns [`DispatchTimeoutError`] in case of a timeout.
    #[inline]
    pub fn try_acquire(
        &self,
        timeout: DispatchTime,
    ) -> Result<DispatchSemaphoreGuard<'_>, DispatchTimeoutError> {
        // Safety: DispatchSemaphore cannot be null.
        let result = Self::wait(self, timeout);

        match NonZeroIsize::new(result) {
            None => Ok(DispatchSemaphoreGuard(self)),
            Some(result) => Err(DispatchTimeoutError(result)),
        }
    }
}

/// Dispatch semaphore guard.
///
/// The semaphore will be signaled when the guard is dropped, or when
/// [`release`] is called.
///
/// [`release`]: DispatchSemaphoreGuard::release
#[derive(Debug)]
pub struct DispatchSemaphoreGuard<'semaphore>(&'semaphore DispatchSemaphore);

impl DispatchSemaphoreGuard<'_> {
    /// Release the [`DispatchSemaphore`].
    #[inline]
    pub fn release(self) -> bool {
        // We suppress `Drop` for the guard because that would signal the
        // semaphore again.
        let this = ManuallyDrop::new(self);
        this.0.signal() != 0
    }
}

impl Drop for DispatchSemaphoreGuard<'_> {
    #[inline]
    fn drop(&mut self) {
        self.0.signal();
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    #[cfg(feature = "objc2")]
    #[cfg_attr(
        not(target_vendor = "apple"),
        ignore = "only Apple's libdisptch is interoperable with `objc2`"
    )]
    fn acquire_release() {
        fn retain_count(semaphore: &DispatchSemaphore) -> usize {
            // SAFETY: semaphore is valid and the method signature is correct.
            unsafe { objc2::msg_send![semaphore, retainCount] }
        }

        let semaphore = DispatchSemaphore::new(1);

        assert_eq!(retain_count(&semaphore), 1);
        {
            let _guard = semaphore.try_acquire(DispatchTime::NOW).unwrap();
            assert_eq!(retain_count(&semaphore), 1);
        }
        assert_eq!(retain_count(&semaphore), 1);
        {
            let guard = semaphore.try_acquire(DispatchTime::NOW).unwrap();
            assert_eq!(retain_count(&semaphore), 1);
            guard.release();
            assert_eq!(retain_count(&semaphore), 1);
        }
        assert_eq!(retain_count(&semaphore), 1);
    }
}
