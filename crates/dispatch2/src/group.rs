use alloc::boxed::Box;
use core::ffi::c_void;
use core::num::NonZeroIsize;

use crate::{DispatchQueue, DispatchTime, DispatchTimeoutError};

use super::utils::function_wrapper;

dispatch_object!(
    /// Dispatch group.
    #[doc(alias = "dispatch_group_t")]
    #[doc(alias = "dispatch_group_s")]
    pub struct DispatchGroup;
);

dispatch_object_not_data!(unsafe DispatchGroup);

impl DispatchGroup {
    /// Submit a function to a [`DispatchQueue`] and associates it with the [`DispatchGroup`].
    #[inline]
    pub fn exec_async<F>(&self, queue: &DispatchQueue, work: F)
    where
        // We need `'static` to make sure any referenced values are borrowed for
        // long enough since `work` will be performed asynchronously.
        F: Send + FnOnce() + 'static,
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast::<c_void>();

        // Safety: All parameters cannot be null.
        unsafe { Self::exec_async_f(self, queue, work_boxed, function_wrapper::<F>) };
    }

    /// Wait synchronously for the previously submitted functions to finish.
    ///
    /// # Errors
    ///
    /// Returns [`DispatchTimeoutError`] in case of timeout.
    #[inline]
    pub fn wait(&self, timeout: DispatchTime) -> Result<(), DispatchTimeoutError> {
        let result = Self::__wait(self, timeout);

        match NonZeroIsize::new(result) {
            None => Ok(()),
            Some(result) => Err(DispatchTimeoutError(result)),
        }
    }

    /// Schedule a function to be submitted to a [`DispatchQueue`] when a group of previously submitted functions have completed.
    #[inline]
    pub fn notify<F>(&self, queue: &DispatchQueue, work: F)
    where
        F: Send + FnOnce(),
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast::<c_void>();

        // Safety: All parameters cannot be null.
        unsafe {
            Self::notify_f(self, queue, work_boxed, function_wrapper::<F>);
        }
    }

    /// Explicitly indicates that the function has entered the [`DispatchGroup`].
    #[inline]
    pub fn enter(&self) -> DispatchGroupGuard<'_> {
        // SAFETY: TODO: Is it a soundness requirement that this is paired with leave?
        unsafe { Self::__enter(self) };

        DispatchGroupGuard(self)
    }
}

/// Dispatch group guard.
#[derive(Debug)]
pub struct DispatchGroupGuard<'group>(&'group DispatchGroup);

impl DispatchGroupGuard<'_> {
    /// Explicitly indicate that the function in the [`DispatchGroup`]
    /// finished executing.
    #[inline]
    pub fn leave(self) {
        // Drop.
        let _ = self;
    }
}

impl Drop for DispatchGroupGuard<'_> {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: TODO: Is it a soundness requirement that this is paired
        // with enter?
        unsafe { self.0.leave() };
    }
}
