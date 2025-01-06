//! Dispatch queue definition.

use alloc::boxed::Box;
use alloc::ffi::CString;
use core::borrow::{Borrow, BorrowMut};
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;
use core::time::Duration;

use super::object::{DispatchObject, QualityOfServiceClassFloorError, TargetQueueError};
use super::utils::function_wrapper;
use super::{ffi::*, QualityOfServiceClass};

/// Error returned by [Queue::after].
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

/// Global queue identifier definition for [Queue::new] and [Queue::new_with_target].
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

/// Auto release frequency for [WorkloopQueue::set_autorelease_frequency].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum DispatchAutoReleaseFrequency {
    /// Inherit autorelease frequency from the target [Queue].
    Inherit,
    /// Configure an autorelease pool before the execution of a function and releases the objects in that pool after the function finishes executing.
    WorkItem,
    /// Never setup an autorelease pool.
    Never,
}

impl From<DispatchAutoReleaseFrequency> for dispatch_autorelease_frequency_t {
    fn from(value: DispatchAutoReleaseFrequency) -> Self {
        match value {
            DispatchAutoReleaseFrequency::Inherit => {
                dispatch_autorelease_frequency_t::DISPATCH_AUTORELEASE_FREQUENCY_INHERIT
            }
            DispatchAutoReleaseFrequency::WorkItem => {
                dispatch_autorelease_frequency_t::DISPATCH_AUTORELEASE_FREQUENCY_WORK_ITEM
            }
            DispatchAutoReleaseFrequency::Never => {
                dispatch_autorelease_frequency_t::DISPATCH_AUTORELEASE_FREQUENCY_NEVER
            }
            _ => panic!("Unknown DispatchAutoReleaseFrequency value: {:?}", value),
        }
    }
}

/// Dispatch queue.
#[derive(Debug, Clone)]
pub struct Queue {
    dispatch_object: DispatchObject<dispatch_queue_s>,
    is_workloop: bool,
}

impl Queue {
    /// Create a new [Queue].
    pub fn new(label: &str, queue_attribute: QueueAttribute) -> Self {
        let label = CString::new(label).expect("Invalid label!");

        // Safety: label and queue_attribute can only be valid.
        let object = unsafe {
            dispatch_queue_create(label.as_ptr(), dispatch_queue_attr_t::from(queue_attribute))
        };

        assert!(!object.is_null(), "dispatch_queue_create shouldn't fail!");

        // Safety: object cannot be null.
        let dispatch_object = unsafe { DispatchObject::new_owned(object.cast()) };

        Queue {
            dispatch_object,
            is_workloop: false,
        }
    }

    /// Create a new [Queue] with a given target [Queue].
    pub fn new_with_target(label: &str, queue_attribute: QueueAttribute, target: &Queue) -> Self {
        let label = CString::new(label).expect("Invalid label!");

        // Safety: label, queue_attribute and target can only be valid.
        let object = unsafe {
            dispatch_queue_create_with_target(
                label.as_ptr(),
                dispatch_queue_attr_t::from(queue_attribute),
                target.dispatch_object.as_raw(),
            )
        };

        assert!(!object.is_null(), "dispatch_queue_create shouldn't fail!");

        // Safety: object cannot be null.
        let dispatch_object = unsafe { DispatchObject::new_owned(object.cast()) };

        // NOTE: dispatch_queue_create_with_target is in charge of retaining the target Queue.

        Queue {
            dispatch_object,
            is_workloop: false,
        }
    }

    /// Return a system-defined global concurrent [Queue] with the priority derived from [GlobalQueueIdentifier].
    pub fn global_queue(identifier: GlobalQueueIdentifier) -> Self {
        let raw_identifier = identifier.to_identifier();

        // Safety: raw_identifier cannot be invalid, flags is reserved.
        let object = unsafe { dispatch_get_global_queue(raw_identifier, 0) };

        assert!(
            !object.is_null(),
            "dispatch_get_global_queue shouldn't fail!"
        );

        // Safety: object cannot be null.
        let dispatch_object = unsafe { DispatchObject::new_shared(object.cast()) };

        Queue {
            dispatch_object,
            is_workloop: false,
        }
    }

    /// Return the main queue.
    pub fn main() -> Self {
        // Safety: raw_identifier cannot be invalid, flags is reserved.
        let object = dispatch_get_main_queue();

        assert!(!object.is_null(), "dispatch_get_main_queue shouldn't fail!");

        // Safety: object cannot be null.
        let dispatch_object = unsafe { DispatchObject::new_shared(object.cast()) };

        Queue {
            dispatch_object,
            is_workloop: false,
        }
    }

    /// Submit a function for synchronous execution on the [Queue].
    pub fn exec_sync<F>(&self, work: F)
    where
        F: Send + FnOnce(),
    {
        assert!(!self.is_workloop, "exec_sync is invalid for WorkloopQueue");

        let work_boxed = Box::into_raw(Box::new(work)).cast();

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe { dispatch_sync_f(self.as_raw(), work_boxed, function_wrapper::<F>) }
    }

    /// Submit a function for asynchronous execution on the [Queue].
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

    /// Enqueue a function for execution at the specified time on the [Queue].
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

    /// Enqueue a barrier function for asynchronous execution on the [Queue] and return immediately.
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

    /// Enqueue a barrier function for synchronous execution on the [Queue] and wait until that function completes.
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

    /// Sets a function at the given key that will be executed at [Queue] destruction.
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

    /// Set the finalizer function for the [Queue].
    pub fn set_finalizer<F>(&mut self, destructor: F)
    where
        F: Send + FnOnce(),
    {
        self.dispatch_object.set_finalizer(destructor);
    }

    /// Set the target [Queue] of this [Queue].
    pub fn set_target_queue(&self, queue: &Queue) -> Result<(), TargetQueueError> {
        // Safety: We are in Queue instance.
        unsafe { self.dispatch_object.set_target_queue(queue) }
    }

    /// Set the QOS class floor of the [Queue].
    pub fn set_qos_class_floor(
        &self,
        qos_class: QualityOfServiceClass,
        relative_priority: i32,
    ) -> Result<(), QualityOfServiceClassFloorError> {
        // Safety: We are in Queue instance.
        unsafe {
            self.dispatch_object
                .set_qos_class_floor(qos_class, relative_priority)
        }
    }

    /// Activate the [Queue].
    pub fn activate(&mut self) {
        self.dispatch_object.activate();
    }

    /// Suspend the invocation of functions on the [Queue].
    pub fn suspend(&self) {
        self.dispatch_object.suspend();
    }

    /// Resume the invocation of functions on the [Queue].
    pub fn resume(&self) {
        self.dispatch_object.resume();
    }

    /// Get the raw [dispatch_queue_t] value.
    ///
    /// # Safety
    ///
    /// - Object shouldn't be released manually.
    pub const unsafe fn as_raw(&self) -> dispatch_queue_t {
        // SAFETY: Upheld by caller.
        unsafe { self.dispatch_object.as_raw() }
    }
}

/// Dispatch workloop queue.
#[derive(Debug, Clone)]
pub struct WorkloopQueue {
    queue: Queue,
}

impl WorkloopQueue {
    /// Create a new [WorkloopQueue].
    pub fn new(label: &str, inactive: bool) -> Self {
        let label = CString::new(label).expect("Invalid label!");

        // Safety: label can only be valid.
        let object = unsafe {
            if inactive {
                dispatch_workloop_create_inactive(label.as_ptr())
            } else {
                dispatch_workloop_create(label.as_ptr())
            }
        };

        assert!(!object.is_null(), "dispatch_queue_create shouldn't fail!");

        // Safety: object cannot be null.
        let dispatch_object = unsafe { DispatchObject::new_owned(object.cast()) };

        WorkloopQueue {
            queue: Queue {
                dispatch_object,
                is_workloop: true,
            },
        }
    }

    /// Configure how the [WorkloopQueue] manage the autorelease pools for the functions it executes.
    pub fn set_autorelease_frequency(&self, frequency: DispatchAutoReleaseFrequency) {
        // Safety: object and frequency can only be valid.
        unsafe {
            dispatch_workloop_set_autorelease_frequency(
                self.as_raw(),
                dispatch_autorelease_frequency_t::from(frequency),
            );
        }
    }

    /// Get the raw [dispatch_workloop_t] value.
    ///
    /// # Safety
    ///
    /// - Object shouldn't be released manually.
    pub const unsafe fn as_raw(&self) -> dispatch_workloop_t {
        // SAFETY: Upheld by caller.
        unsafe { self.queue.as_raw() as dispatch_workloop_t }
    }
}

impl Deref for WorkloopQueue {
    type Target = Queue;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.queue
    }
}

impl DerefMut for WorkloopQueue {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.queue
    }
}

impl AsRef<Queue> for WorkloopQueue {
    #[inline]
    fn as_ref(&self) -> &Queue {
        self
    }
}

impl AsMut<Queue> for WorkloopQueue {
    #[inline]
    fn as_mut(&mut self) -> &mut Queue {
        &mut *self
    }
}

impl Borrow<Queue> for WorkloopQueue {
    #[inline]
    fn borrow(&self) -> &Queue {
        self
    }
}

impl BorrowMut<Queue> for WorkloopQueue {
    #[inline]
    fn borrow_mut(&mut self) -> &mut Queue {
        &mut *self
    }
}
