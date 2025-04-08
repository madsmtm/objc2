//! Dispatch object definition.

use core::ptr::NonNull;

use alloc::boxed::Box;

use crate::DispatchRetained;

use super::{ffi::*, queue::DispatchQueue, utils::function_wrapper, QualityOfServiceClass};

/// Error returned by [DispatchObject::set_qos_class_floor].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum QualityOfServiceClassFloorError {
    /// The relative priority is invalid.
    InvalidRelativePriority,
}

/// Types that represent dispatch objects.
///
/// # Safety
///
/// The object must represent a dispatch object, and be usable in
/// `dispatch_retain` / `dispatch_release`.
pub unsafe trait DispatchObject {
    /// Increment the reference count of the object.
    ///
    /// This extends the duration in which the object is alive by detaching it
    /// from the lifetime information carried by the reference.
    fn retain(&self) -> DispatchRetained<Self> {
        let ptr: NonNull<Self> = NonNull::from(self);
        // SAFETY:
        // - The pointer is valid since it came from `&self`.
        // - The lifetime of the pointer itself is extended, but any lifetime
        //   that the object may carry is still kept within the type itself.
        unsafe { DispatchRetained::retain(ptr) }
    }

    /// Set the finalizer function for the object.
    fn set_finalizer<F>(&mut self, destructor: F)
    where
        F: Send + FnOnce(),
    {
        let destructor_boxed = Box::into_raw(Box::new(destructor)).cast();

        // Safety: As this use the dispatch object's context, and because we need some way to wrap the Rust function, we set the context.
        //         Once the finalizer is executed, the context will be dangling.
        //         This isn't an issue as the context shall not be accessed after the dispatch object is destroyed.
        unsafe {
            dispatch_set_context(self.as_raw(), destructor_boxed);
            dispatch_set_finalizer_f(self.as_raw(), function_wrapper::<F>)
        }
    }

    /// Set the target [`DispatchQueue`] of this object.
    ///
    /// # Aborts
    ///
    /// Aborts if the object has been activated.
    ///
    /// # Safety
    ///
    /// - There must not be a cycle in the hierarchy of queues.
    #[doc(alias = "dispatch_set_target_queue")]
    unsafe fn set_target_queue(&self, queue: &DispatchQueue) {
        // SAFETY: `object` and `queue` cannot be null, rest is upheld by caller.
        unsafe { dispatch_set_target_queue(self.as_raw(), queue.as_raw()) };
    }

    /// Set the QOS class floor on a dispatch queue, source or workloop.
    ///
    /// # Safety
    ///
    /// - DispatchObject should be a queue or queue source.
    unsafe fn set_qos_class_floor(
        &self,
        qos_class: QualityOfServiceClass,
        relative_priority: i32,
    ) -> Result<(), QualityOfServiceClassFloorError> {
        if !(QOS_MIN_RELATIVE_PRIORITY..=0).contains(&relative_priority) {
            return Err(QualityOfServiceClassFloorError::InvalidRelativePriority);
        }

        // SAFETY: Safe as relative_priority can only be valid.
        unsafe {
            dispatch_set_qos_class_floor(
                self.as_raw(),
                dispatch_qos_class_t::from(qos_class),
                relative_priority,
            );
        }

        Ok(())
    }

    /// Activate the object.
    fn activate(&mut self) {
        // Safety: object cannot be null.
        unsafe { dispatch_activate(self.as_raw()) };
    }

    /// Suspend the invocation of functions on the object.
    fn suspend(&self) {
        // Safety: object cannot be null.
        unsafe { dispatch_suspend(self.as_raw()) };
    }

    /// Resume the invocation of functions on the object.
    fn resume(&self) {
        // Safety: object cannot be null.
        unsafe { dispatch_resume(self.as_raw()) };
    }

    #[doc(hidden)]
    fn as_raw(&self) -> dispatch_object_t {
        let ptr: *const Self = self;
        ptr as dispatch_object_t
    }
}
