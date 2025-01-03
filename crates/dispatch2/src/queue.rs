//! Dispatch queue definition.

use alloc::{boxed::Box, ffi::CString};
use core::{ptr::NonNull, time::Duration};

use super::{
    ffi::*, function_wrapper, rc::Retained, AsRawDispatchObject, QualityOfServiceClass,
    QualityOfServiceClassFloorError, TryFromDurationError,
};

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
            QueueAttribute::Concurrent => DISPATCH_QUEUE_CONCURRENT.0 as *const _ as *mut _,
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

impl Default for GlobalQueueIdentifier {
    fn default() -> Self {
        Self::QualityOfService(QualityOfServiceClass::default())
    }
}

/// Dispatch queue.
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Queue {
    _inner: [u8; 0],
}

impl Queue {
    /// Create a new [Queue].
    pub fn new(label: &str, queue_attribute: QueueAttribute) -> Retained<Self> {
        let label = CString::new(label).expect("Invalid label!");

        // Safety: label and queue_attribute can only be valid.
        let object = unsafe {
            dispatch_queue_create(label.as_ptr(), dispatch_queue_attr_t::from(queue_attribute))
        };

        // Safety: object must be valid.
        unsafe { Retained::from_raw(object.cast()) }.expect("dispatch_queue_create failed")
    }

    /// Create a new [Queue] with a given target [Queue].
    pub fn new_with_target(
        label: &str,
        queue_attribute: QueueAttribute,
        target: &Queue,
    ) -> Retained<Self> {
        let label = CString::new(label).expect("Invalid label!");

        // Safety: label, queue_attribute and target can only be valid.
        let object = unsafe {
            dispatch_queue_create_with_target(
                label.as_ptr(),
                dispatch_queue_attr_t::from(queue_attribute),
                target.as_raw(),
            )
        };

        assert!(!object.is_null());

        // NOTE: dispatch_queue_create_with_target is in charge of retaining the target Queue.

        // Safety: object must be valid.
        unsafe { Retained::from_raw(object.cast()) }.expect("dispatch_queue_create failed")
    }

    /// Return a system-defined global concurrent [Queue] with the priority derived from [GlobalQueueIdentifier].
    pub fn global_queue(identifier: GlobalQueueIdentifier) -> Retained<Self> {
        // Safety: raw_identifier cannot be invalid, flags is reserved.
        let object = unsafe { dispatch_get_global_queue(identifier.to_identifier(), 0) };
        assert!(!object.is_null());

        // Safety: object must be valid.
        unsafe { Retained::from_raw(object.cast()) }.expect("dispatch_get_global_queue failed")
    }

    /// Return the main queue.
    pub fn main() -> Retained<Self> {
        let object = dispatch_get_main_queue();
        assert!(!object.is_null());

        // Safety: object must be valid.
        unsafe { Retained::from_raw(object.cast()) }.expect("dispatch_get_main_queue failed")
    }

    /// Submit a function for synchronous execution on the [Queue].
    pub fn exec_sync<F>(&self, work: F)
    where
        F: Send + FnOnce(),
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast();

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe { dispatch_sync_f(self.as_raw(), work_boxed, function_wrapper::<F>) }
    }

    /// Submit a function for asynchronous execution on the [Queue].
    pub fn exec_async<F>(&self, work: F)
    where
        F: Send + FnOnce() + 'static,
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast();

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe { dispatch_async_f(self.as_raw(), work_boxed, function_wrapper::<F>) }
    }

    /// Enqueue a function for execution at the specified time on the [Queue].
    pub fn after<F>(&self, wait_time: Duration, work: F) -> Result<(), TryFromDurationError>
    where
        F: Send + FnOnce() + 'static,
    {
        let when = dispatch_time_t::try_from(wait_time)?;
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
        F: Send + FnOnce(),
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

    /// Set the QOS class floor of the [Queue].
    pub fn set_qos_class_floor(
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
                self.as_raw_object(),
                dispatch_qos_class_t::from(qos_class),
                relative_priority,
            );
        }

        Ok(())
    }

    /// Get the raw [dispatch_queue_t] value.
    ///
    /// # Safety
    ///
    /// - Object shouldn't be released manually.
    pub fn as_raw(&self) -> dispatch_queue_t {
        self as *const Self as _
    }
}

impl AsRawDispatchObject for Queue {
    fn as_raw_object(&self) -> dispatch_object_t {
        self.as_raw().cast()
    }
}

// Safety: it's safe to move queue between threads.
unsafe impl Send for Queue {}

// Safety: it's safe to share queue between threads.
unsafe impl Sync for Queue {}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    use super::*;

    #[test]
    fn test_create_main_queue() {
        let _ = Queue::main();
    }

    #[test]
    fn test_serial_queue() {
        let queue = Queue::new("com.github.madsmtm.objc2", QueueAttribute::Serial);
        let (tx, rx) = mpsc::channel();
        queue.exec_async(move || {
            tx.send(()).unwrap();
        });
        rx.recv().unwrap();
    }

    #[test]
    fn test_concurrent_queue() {
        let queue = Queue::new("com.github.madsmtm.objc2", QueueAttribute::Concurrent);
        let (tx, rx) = mpsc::channel();
        let cloned_tx = tx.clone();
        queue.exec_async(move || {
            tx.send(()).unwrap();
        });
        queue.exec_async(move || {
            cloned_tx.send(()).unwrap();
        });
        for _ in 0..2 {
            rx.recv().unwrap();
        }
    }

    #[test]
    fn test_global_default_queue() {
        let queue = Queue::global_queue(GlobalQueueIdentifier::default());
        let (tx, rx) = mpsc::channel();
        queue.exec_async(move || {
            tx.send(()).unwrap();
        });
        rx.recv().unwrap();
    }

    #[test]
    fn test_share_queue_across_threads() {
        let queue = Queue::new("com.github.madsmtm.objc2", QueueAttribute::Serial);
        let (tx, rx) = mpsc::channel();
        let cloned_tx = tx.clone();
        let cloned_queue = queue.clone();
        queue.exec_async(move || {
            cloned_queue.exec_async(move || {
                cloned_tx.send(()).unwrap();
            });
        });
        queue.exec_async(move || {
            tx.send(()).unwrap();
        });
        for _ in 0..2 {
            rx.recv().unwrap();
        }
    }

    #[test]
    fn test_move_queue_between_threads() {
        let queue = Queue::new("com.github.madsmtm.objc2", QueueAttribute::Serial);
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            queue.exec_async(move || {
                tx.send(()).unwrap();
            });
        });
        rx.recv().unwrap();
    }
}
