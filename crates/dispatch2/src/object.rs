//! Dispatch object definition.

use alloc::boxed::Box;

use super::{ffi::*, function_wrapper, queue::Queue};

/// Types convertible to raw pointer to dispatch object.
pub trait AsRawDispatchObject {
    /// Returns a raw pointer to dispatch object.
    fn as_raw_object(&self) -> dispatch_object_t;
}

/// Objects that inherit from `DispatchObject`
pub trait DispatchObjectLike {
    /// Activates the dispatch object.
    fn activate(&self);

    /// Resumes the invocation of block objects on a dispatch object.
    ///
    /// Calling this function decrements the suspension count.
    /// The object remains suspended while the count is greater than zero.
    fn resume(&self);

    /// Suspends the invocation of block objects on a dispatch object.
    ///
    /// Calling this function increments the suspension count.
    /// The object remains suspended while the count count is greater than zero.
    ///
    /// # Safety
    /// It is a programmer error to release an object that is currently suspended
    unsafe fn suspend(&self);

    /// Specifies the dispatch queue on which to perform work associated with the current object.
    ///
    /// # Safety
    /// - When setting up target queues, it is a programmer error to create cycles in the dispatch queue hierarchy.
    ///   In other words, don't set the target of queue A to queue B and the target of queue B to queue A.
    /// - Once a dispatch object has been activated, it cannot change its target queue.
    unsafe fn set_target_queue(&self, target_queue: Queue);

    /// Sets the finalizer function for a dispatch object.
    fn set_finalizer<F>(&mut self, destructor: F)
    where
        F: Send + FnOnce();
}

impl<T> DispatchObjectLike for T
where
    T: AsRawDispatchObject,
{
    fn activate(&self) {
        // Safety: pointer must be valid.
        unsafe { dispatch_activate(self.as_raw_object()) }
    }

    fn resume(&self) {
        // Safety: pointer must be valid.
        unsafe { dispatch_resume(self.as_raw_object()) };
    }

    unsafe fn suspend(&self) {
        // Safety: pointer must be valid.
        unsafe { dispatch_suspend(self.as_raw_object()) };
    }

    unsafe fn set_target_queue(&self, target_queue: Queue) {
        // Safety: pointers must be valid.
        unsafe { dispatch_set_target_queue(self.as_raw_object(), target_queue.as_raw()) }
    }

    fn set_finalizer<F>(&mut self, destructor: F)
    where
        F: Send + FnOnce(),
    {
        let destructor_boxed = Box::into_raw(Box::new(destructor)).cast();

        // Safety: As this use the dispatch object's context, and because we need some way to wrap the Rust function, we set the context.
        //         Once the finalizer is executed, the context will be dangling.
        //         This isn't an issue as the context shall not be accessed after the dispatch object is destroyed.
        unsafe {
            dispatch_set_context(self.as_raw_object(), destructor_boxed);
            dispatch_set_finalizer_f(self.as_raw_object(), function_wrapper::<F>)
        }
    }
}
