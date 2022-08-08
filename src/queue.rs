use std::borrow::{Borrow, BorrowMut};
use std::ffi::CString;
use std::ops::{Deref, DerefMut};
use std::time::Duration;

use super::object::{DispatchObject, QualityOfServiceClassFloorError, TargetQueueError};
use super::utils::function_wrapper;
use super::{ffi::*, QualityOfServiceClass};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum QueueAfterError {
    TimeOverflow,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum QueueAttribute {
    Serial,
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum QueuePriority {
    High,
    Default,
    Low,
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GlobalQueueIdentifier {
    Priority(QueuePriority),
    QualityOfService(QualityOfServiceClass),
}

impl GlobalQueueIdentifier {
    pub fn to_identifier(self) -> usize {
        match self {
            GlobalQueueIdentifier::Priority(queue_priority) => {
                dispatch_queue_priority_t::from(queue_priority).0 as usize
            }
            GlobalQueueIdentifier::QualityOfService(qos_class) => {
                dispatch_qos_class_t::from(qos_class).0 as usize
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum DispatchAutoReleaseFrequency {
    Inherit,
    WorkItem,
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

#[derive(Debug, Clone)]
pub struct Queue {
    dispatch_object: DispatchObject<dispatch_queue_s>,
    is_workloop: bool,
}

impl Queue {
    pub fn new(label: &str, queue_attribute: QueueAttribute) -> Self {
        let label = CString::new(label).expect("Invalid label!");

        // Safety: label and queue_attribute can only be valid.
        let object = unsafe {
            dispatch_queue_create(label.as_ptr(), dispatch_queue_attr_t::from(queue_attribute))
        };

        assert!(!object.is_null(), "dispatch_queue_create shouldn't fail!");

        // Safety: object cannot be null.
        let dispatch_object = unsafe { DispatchObject::new_owned(object as *mut _) };

        Queue {
            dispatch_object,
            is_workloop: false,
        }
    }

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
        let dispatch_object = unsafe { DispatchObject::new_owned(object as *mut _) };

        // NOTE: dispatch_queue_create_with_target is in charge of retaining the target Queue.

        Queue {
            dispatch_object,
            is_workloop: false,
        }
    }

    pub fn global_queue(identifier: GlobalQueueIdentifier) -> Self {
        let raw_identifier = identifier.to_identifier();

        // Safety: raw_identifier cannot be invalid, flags is reserved.
        let object = unsafe { dispatch_get_global_queue(raw_identifier, 0) };

        assert!(
            !object.is_null(),
            "dispatch_get_global_queue shouldn't fail!"
        );

        // Safety: object cannot be null.
        let dispatch_object = unsafe { DispatchObject::new_shared(object as *mut _) };

        Queue {
            dispatch_object,
            is_workloop: false,
        }
    }

    pub fn exec_sync<F>(&self, work: F)
    where
        F: Send + FnOnce(),
    {
        assert!(!self.is_workloop, "exec_sync is invalid for WorkloopQueue");

        let work_boxed = Box::leak(Box::new(work)) as *mut F as *mut _;

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe { dispatch_sync_f(self.as_raw(), work_boxed, function_wrapper::<F>) }
    }

    pub fn exec_async<F>(&self, work: F)
    where
        F: Send + FnOnce(),
    {
        let work_boxed = Box::leak(Box::new(work)) as *mut F as *mut _;

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe { dispatch_async_f(self.as_raw(), work_boxed, function_wrapper::<F>) }
    }

    pub fn after<F>(&self, wait_time: Duration, work: F) -> Result<(), QueueAfterError>
    where
        F: Send + FnOnce(),
    {
        let when =
            dispatch_time_t::try_from(wait_time).map_err(|_| QueueAfterError::TimeOverflow)?;
        let work_boxed = Box::leak(Box::new(work)) as *mut F as *mut _;

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe {
            dispatch_after_f(when, self.as_raw(), work_boxed, function_wrapper::<F>);
        }

        Ok(())
    }

    pub fn barrier_async<F>(&self, work: F)
    where
        F: Send + FnOnce(),
    {
        let work_boxed = Box::leak(Box::new(work)) as *mut F as *mut _;

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe { dispatch_barrier_async_f(self.as_raw(), work_boxed, function_wrapper::<F>) }
    }

    pub fn barrier_sync<F>(&self, work: F)
    where
        F: Send + FnOnce(),
    {
        let work_boxed = Box::leak(Box::new(work)) as *mut F as *mut _;

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe { dispatch_barrier_sync_f(self.as_raw(), work_boxed, function_wrapper::<F>) }
    }

    pub fn barrier_async_and_wait<F>(&self, work: F)
    where
        F: Send + FnOnce(),
    {
        let work_boxed = Box::leak(Box::new(work)) as *mut F as *mut _;

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe {
            dispatch_barrier_async_and_wait_f(self.as_raw(), work_boxed, function_wrapper::<F>)
        }
    }

    pub fn set_specific<F>(&mut self, key: usize, destructor: F)
    where
        F: Send + FnOnce(),
    {
        let destructor_boxed = Box::leak(Box::new(destructor)) as *mut F as *mut _;

        // Safety: object cannot be null and destructor is wrapped to avoid ABI incompatibility.
        unsafe {
            dispatch_queue_set_specific(
                self.as_raw(),
                key as *const _,
                destructor_boxed,
                function_wrapper::<F>,
            )
        }
    }

    pub fn set_finalizer<F>(&mut self, destructor: F)
    where
        F: Send + FnOnce(),
    {
        self.dispatch_object.set_finalizer(destructor);
    }

    pub fn set_target_queue(&self, queue: &Queue) -> Result<(), TargetQueueError> {
        // Safety: We are in Queue instance.
        unsafe { self.dispatch_object.set_target_queue(queue) }
    }

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

    pub fn activate(&mut self) {
        self.dispatch_object.activate();
    }

    pub fn suspend(&self) {
        self.dispatch_object.suspend();
    }

    pub fn resume(&self) {
        self.dispatch_object.resume();
    }

    pub const fn as_raw(&self) -> dispatch_queue_t {
        self.dispatch_object.as_raw()
    }
}

#[derive(Debug, Clone)]
pub struct WorkloopQueue {
    queue: Queue,
}

impl WorkloopQueue {
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
        let dispatch_object = unsafe { DispatchObject::new_owned(object as *mut _) };

        WorkloopQueue {
            queue: Queue {
                dispatch_object,
                is_workloop: true,
            },
        }
    }

    pub fn set_autorelease_frequency(&self, frequency: DispatchAutoReleaseFrequency) {
        // Safety: object and frequency can only be valid.
        unsafe {
            dispatch_workloop_set_autorelease_frequency(
                self.as_raw(),
                dispatch_autorelease_frequency_t::from(frequency),
            );
        }
    }

    pub const fn as_raw(&self) -> dispatch_workloop_t {
        self.queue.as_raw() as dispatch_workloop_t
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
        &*self
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
        &*self
    }
}

impl BorrowMut<Queue> for WorkloopQueue {
    #[inline]
    fn borrow_mut(&mut self) -> &mut Queue {
        &mut *self
    }
}
