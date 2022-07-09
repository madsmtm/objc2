use super::{ffi::*, queue::Queue, utils::function_wrapper, QualityOfServiceClass};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum TargetQueueError {
    ObjectAlreadyActive,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum QualityOfServiceClassFloorError {
    InvalidRelativePriority,
}

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

        unsafe {
            // Safety: We own a reference to the object.
            dispatch_retain(result.object as *mut _);
        }

        result
    }

    pub fn set_finalizer<F>(&mut self, destructor: F)
    where
        F: Send + FnOnce(),
    {
        let destructor_boxed = Box::leak(Box::new(destructor)) as *mut F as *mut _;

        unsafe {
            // As this use the dispatch object's context, and because we need some way to wrap the Rust function, we set the context.
            // Once the finalizer is executed, the context will be dangling.
            // This isn't an issue as the context shall not be accessed after the dispatch object is destroyed.
            dispatch_set_context(self.object as *mut _, destructor_boxed);

            dispatch_set_finalizer_f(self.object as *mut _, function_wrapper::<F>)
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

        unsafe {
            dispatch_set_target_queue(self.as_raw() as *mut _, queue.as_raw());
        }

        Ok(())
    }

    /// Sets the QOS class floor on a dispatch queue, source or workloop.
    ///
    /// # Safety
    ///
    /// - DispatchObject should be a queue or queue source.
    pub unsafe fn set_qos_class_floor(
        &self,
        qos_class: QualityOfServiceClass,
        relative_priority: i32,
    ) -> Result<(), QualityOfServiceClassFloorError> {
        if relative_priority > 0 || relative_priority < QOS_MIN_RELATIVE_PRIORITY {
            return Err(QualityOfServiceClassFloorError::InvalidRelativePriority);
        }

        unsafe {
            // Safety: Safe as relative_priority can only be valid.
            dispatch_set_qos_class_floor(
                self.as_raw() as *mut _,
                dispatch_qos_class_t::from(qos_class),
                relative_priority,
            );
        }

        Ok(())
    }

    pub fn activate(&mut self) {
        unsafe {
            dispatch_activate(self.as_raw() as *mut _);
        }

        self.is_activated = true;
    }

    pub fn suspend(&self) {
        unsafe {
            dispatch_suspend(self.as_raw() as *mut _);
        }
    }

    pub fn resume(&self) {
        unsafe {
            dispatch_resume(self.as_raw() as *mut _);
        }
    }

    pub const fn as_raw(&self) -> *mut T {
        self.object
    }
}

impl<T> Clone for DispatchObject<T> {
    fn clone(&self) -> Self {
        unsafe {
            // Safety: We own a reference to the object.
            Self::new_shared(self.object)
        }
    }
}

impl<T> Drop for DispatchObject<T> {
    fn drop(&mut self) {
        unsafe {
            // Safety: We own a reference to the object.
            dispatch_release(self.object as *mut _);
        }
    }
}
