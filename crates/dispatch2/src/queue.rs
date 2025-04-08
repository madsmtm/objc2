use alloc::boxed::Box;
use alloc::ffi::CString;
use core::ptr::NonNull;
use core::time::Duration;

use crate::DispatchRetained;

use super::object::{DispatchObject, QualityOfServiceClassFloorError};
use super::utils::function_wrapper;
use super::{ffi::*, QualityOfServiceClass};

/// Error returned by [`DispatchQueue::after`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum QueueAfterError {
    /// The given timeout value will result in an overflow when converting to dispatch time.
    TimeOverflow,
}

/// Queue type attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum QueueAttribute {
    /// Serial queue.
    Serial,
    /// Concurrent queue.
    Concurrent,
}

impl From<QueueAttribute> for dispatch_queue_attr_t {
    fn from(value: QueueAttribute) -> Self {
        match value {
            QueueAttribute::Serial => DISPATCH_QUEUE_SERIAL,
            QueueAttribute::Concurrent => DISPATCH_QUEUE_CONCURRENT as *const _ as *mut _,
            _ => panic!("Unknown QueueAttribute value: {:?}", value),
        }
    }
}

/// Queue priority.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum QueuePriority {
    /// High priority.
    High,
    /// Default priority.
    Default,
    /// Low priority.
    Low,
    /// Background priority.
    Background,
}

impl From<QueuePriority> for dispatch_queue_priority_t {
    fn from(value: QueuePriority) -> Self {
        match value {
            QueuePriority::High => dispatch_queue_priority_t::DISPATCH_QUEUE_PRIORITY_HIGH,
            QueuePriority::Default => dispatch_queue_priority_t::DISPATCH_QUEUE_PRIORITY_DEFAULT,
            QueuePriority::Low => dispatch_queue_priority_t::DISPATCH_QUEUE_PRIORITY_LOW,
            QueuePriority::Background => {
                dispatch_queue_priority_t::DISPATCH_QUEUE_PRIORITY_BACKGROUND
            }
            _ => panic!("Unknown QueuePriority value: {:?}", value),
        }
    }
}

/// Global queue identifier definition for [`DispatchQueue::new`] and [`DispatchQueue::new_with_target`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GlobalQueueIdentifier {
    /// Standard priority based queue.
    Priority(QueuePriority),
    /// Quality of service priority based queue.
    QualityOfService(QualityOfServiceClass),
}

impl GlobalQueueIdentifier {
    /// Convert and consume [GlobalQueueIdentifier] into its raw value.
    pub fn to_identifier(self) -> isize {
        match self {
            GlobalQueueIdentifier::Priority(queue_priority) => {
                dispatch_queue_priority_t::from(queue_priority).0 as isize
            }
            GlobalQueueIdentifier::QualityOfService(qos_class) => {
                dispatch_qos_class_t::from(qos_class).0 as isize
            }
        }
    }
}

dispatch_object!(
    /// Dispatch queue.
    pub struct DispatchQueue;
);

impl DispatchQueue {
    /// Create a new [`DispatchQueue`].
    pub fn new(label: &str, queue_attribute: QueueAttribute) -> DispatchRetained<Self> {
        let label = CString::new(label).expect("Invalid label!");

        // Safety: label and queue_attribute can only be valid.
        let object = unsafe {
            dispatch_queue_create(label.as_ptr(), dispatch_queue_attr_t::from(queue_attribute))
        };

        let object = NonNull::new(object).expect("dispatch_queue_create returned NULL");
        // SAFETY: Object came from a "create" method.
        unsafe { DispatchRetained::from_raw(object.cast()) }
    }

    /// Create a new [`DispatchQueue`] with a given target [`DispatchQueue`].
    pub fn new_with_target(
        label: &str,
        queue_attribute: QueueAttribute,
        target: &DispatchQueue,
    ) -> DispatchRetained<Self> {
        let label = CString::new(label).expect("Invalid label!");

        // Safety: label, queue_attribute and target can only be valid.
        // NOTE: dispatch_queue_create_with_target is in charge of retaining the target DispatchQueue.
        let object = unsafe {
            dispatch_queue_create_with_target(
                label.as_ptr(),
                dispatch_queue_attr_t::from(queue_attribute),
                target.as_raw(),
            )
        };

        let object = NonNull::new(object).expect("dispatch_queue_create_with_target returned NULL");
        // SAFETY: Object came from a "create" method.
        unsafe { DispatchRetained::from_raw(object.cast()) }
    }

    /// Return a system-defined global concurrent [`DispatchQueue`] with the priority derived from [GlobalQueueIdentifier].
    pub fn global_queue(identifier: GlobalQueueIdentifier) -> DispatchRetained<Self> {
        let raw_identifier = identifier.to_identifier();

        // Safety: raw_identifier cannot be invalid, flags is reserved.
        let object = unsafe { dispatch_get_global_queue(raw_identifier, 0) };

        let object = NonNull::new(object).expect("dispatch_get_global_queue returned NULL");
        // SAFETY: Object came from a "create" method.
        unsafe { DispatchRetained::from_raw(object.cast()) }
    }

    /// Return the main queue.
    pub fn main() -> DispatchRetained<Self> {
        // Safety: raw_identifier cannot be invalid, flags is reserved.
        let object = dispatch_get_main_queue();

        let object = NonNull::new(object).expect("dispatch_get_main_queue returned NULL");
        // SAFETY: Object came from a "create" method.
        unsafe { DispatchRetained::from_raw(object.cast()) }
    }

    /// Submit a function for synchronous execution on the [`DispatchQueue`].
    pub fn exec_sync<F>(&self, work: F)
    where
        F: Send + FnOnce(),
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast();

        // NOTE: `dispatch_sync*` functions are discouraged on workloops for
        // performance reasons, but they should still work, so we won't forbid
        // it here.
        //
        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe { dispatch_sync_f(self.as_raw(), work_boxed, function_wrapper::<F>) }
    }

    /// Submit a function for asynchronous execution on the [`DispatchQueue`].
    pub fn exec_async<F>(&self, work: F)
    where
        // We need `'static` to make sure any referenced values are borrowed for
        // long enough since `work` will be performed asynchronously.
        F: Send + FnOnce() + 'static,
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast();

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe { dispatch_async_f(self.as_raw(), work_boxed, function_wrapper::<F>) }
    }

    /// Enqueue a function for execution at the specified time on the [`DispatchQueue`].
    pub fn after<F>(&self, wait_time: Duration, work: F) -> Result<(), QueueAfterError>
    where
        F: Send + FnOnce(),
    {
        let when =
            dispatch_time_t::try_from(wait_time).map_err(|_| QueueAfterError::TimeOverflow)?;
        let work_boxed = Box::into_raw(Box::new(work)).cast();

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe {
            dispatch_after_f(when, self.as_raw(), work_boxed, function_wrapper::<F>);
        }

        Ok(())
    }

    /// Enqueue a barrier function for asynchronous execution on the [`DispatchQueue`] and return immediately.
    pub fn barrier_async<F>(&self, work: F)
    where
        // We need `'static` to make sure any referenced values are borrowed for
        // long enough since `work` will be performed asynchronously.
        F: Send + FnOnce() + 'static,
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast();

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe { dispatch_barrier_async_f(self.as_raw(), work_boxed, function_wrapper::<F>) }
    }

    /// Enqueue a barrier function for synchronous execution on the [`DispatchQueue`] and wait until that function completes.
    pub fn barrier_sync<F>(&self, work: F)
    where
        F: Send + FnOnce(),
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast();

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe { dispatch_barrier_sync_f(self.as_raw(), work_boxed, function_wrapper::<F>) }
    }

    /// Submit a function for synchronous execution and mark the function as a barrier for subsequent concurrent tasks.
    pub fn barrier_async_and_wait<F>(&self, work: F)
    where
        // We need `'static` to make sure any referenced values are borrowed for
        // long enough since `work` will be performed asynchronously.
        F: Send + FnOnce() + 'static,
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast();

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe {
            dispatch_barrier_async_and_wait_f(self.as_raw(), work_boxed, function_wrapper::<F>)
        }
    }

    /// Sets a function at the given key that will be executed at [`DispatchQueue`] destruction.
    pub fn set_specific<F>(&mut self, key: NonNull<()>, destructor: F)
    where
        F: Send + FnOnce(),
    {
        let destructor_boxed = Box::into_raw(Box::new(destructor)).cast();

        // SAFETY: object cannot be null and destructor is wrapped to avoid
        // ABI incompatibility.
        //
        // The key is never dereferenced, so passing _any_ pointer here is
        // safe and allowed.
        unsafe {
            dispatch_queue_set_specific(
                self.as_raw(),
                key.cast(),
                destructor_boxed,
                function_wrapper::<F>,
            )
        }
    }

    /// Set the QOS class floor of the [`DispatchQueue`].
    pub fn set_qos_class_floor(
        &self,
        qos_class: QualityOfServiceClass,
        relative_priority: i32,
    ) -> Result<(), QualityOfServiceClassFloorError> {
        // SAFETY: We are a queue.
        unsafe { DispatchObject::set_qos_class_floor(self, qos_class, relative_priority) }
    }

    /// Get the raw [dispatch_queue_t] value.
    ///
    /// # Safety
    ///
    /// - Object shouldn't be released manually.
    pub const unsafe fn as_raw(&self) -> dispatch_queue_t {
        let ptr: *const Self = self;
        ptr as dispatch_queue_t
    }
}
