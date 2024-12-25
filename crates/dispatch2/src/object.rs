//! Dispatch object definition.

use alloc::boxed::Box;

use super::{ffi::*, queue::Queue, utils::function_wrapper, QualityOfServiceClass};

/// Error returned by [DispatchObject::set_target_queue].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum TargetQueueError {
    /// The [DispatchObject] is already active.
    ObjectAlreadyActive,
}

/// Error returned by [DispatchObject::set_qos_class_floor].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum QualityOfServiceClassFloorError {
    /// The relative priority is invalid.
    InvalidRelativePriority,
}

/// Represent a dispatch object.
#[repr(C)]
#[derive(Debug)]
pub struct DispatchObject<T> {
    object: *mut T,
    is_activated: bool,
}

impl<T> DispatchObject<T> {
    /// Create a new owned instance
    ///
    /// # Safety
    ///
    /// - ``object`` is expected to be a dispatch object that is owned.
    pub unsafe fn new_owned(object: *mut T) -> Self {
        Self {
            object,
            is_activated: false,
        }
    }

    /// Create a new shared instance
    ///
    /// # Safety
    ///
    /// - ``object`` is expected to be a dispatch object that is shared.
    pub unsafe fn new_shared(object: *mut T) -> Self {
        let result = Self {
            object,
            is_activated: false,
        };

        // Safety: We own a reference to the object.
        unsafe {
            dispatch_retain(result.object.cast());
        }

        result
    }

    /// Set the finalizer function for the object.
    pub fn set_finalizer<F>(&mut self, destructor: F)
    where
        F: Send + FnOnce(),
    {
        let destructor_boxed = Box::into_raw(Box::new(destructor)).cast();

        // Safety: As this use the dispatch object's context, and because we need some way to wrap the Rust function, we set the context.
        //         Once the finalizer is executed, the context will be dangling.
        //         This isn't an issue as the context shall not be accessed after the dispatch object is destroyed.
        unsafe {
            dispatch_set_context(self.object.cast(), destructor_boxed);
            dispatch_set_finalizer_f(self.object.cast(), function_wrapper::<F>)
        }
    }

    /// Set the target [Queue] of this object.
    ///
    /// # Safety
    ///
    /// - DispatchObject should be a queue or queue source.
    pub unsafe fn set_target_queue(&self, queue: &Queue) -> Result<(), TargetQueueError> {
        if self.is_activated {
            return Err(TargetQueueError::ObjectAlreadyActive);
        }

        // SAFETY: object and queue cannot be null.
        unsafe {
            dispatch_set_target_queue(self.as_raw().cast(), queue.as_raw());
        }

        Ok(())
    }

    /// Set the QOS class floor on a dispatch queue, source or workloop.
    ///
    /// # Safety
    ///
    /// - DispatchObject should be a queue or queue source.
    pub unsafe fn set_qos_class_floor(
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
                self.as_raw().cast(),
                dispatch_qos_class_t::from(qos_class),
                relative_priority,
            );
        }

        Ok(())
    }

    /// Activate the object.
    pub fn activate(&mut self) {
        // Safety: object cannot be null.
        unsafe {
            dispatch_activate(self.as_raw().cast());
        }

        self.is_activated = true;
    }

    /// Suspend the invocation of functions on the object.
    pub fn suspend(&self) {
        // Safety: object cannot be null.
        unsafe {
            dispatch_suspend(self.as_raw().cast());
        }
    }

    /// Resume the invocation of functions on the object.
    pub fn resume(&self) {
        // Safety: object cannot be null.
        unsafe {
            dispatch_resume(self.as_raw().cast());
        }
    }

    /// Get the raw object value.
    ///
    /// # Safety
    ///
    /// - Object shouldn't be released manually.
    pub const unsafe fn as_raw(&self) -> *mut T {
        self.object
    }
}

impl<T> Clone for DispatchObject<T> {
    fn clone(&self) -> Self {
        // Safety: We own a reference to the object.
        unsafe { Self::new_shared(self.object) }
    }
}

impl<T> Drop for DispatchObject<T> {
    fn drop(&mut self) {
        // Safety: We own a reference to the object.
        unsafe {
            dispatch_release(self.object.cast());
        }
    }
}
