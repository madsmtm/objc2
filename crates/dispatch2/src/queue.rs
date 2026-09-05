use alloc::boxed::Box;
use core::ffi::c_long;
use core::ptr::NonNull;

use super::utils::function_wrapper;
use crate::generated::{_dispatch_main_q, _dispatch_queue_attr_concurrent};
use crate::{
    DispatchObject, DispatchQoS, DispatchQueue, DispatchQueueAttr, DispatchRetained, DispatchTime,
    QualityOfServiceClassFloorError,
};

enum_with_val! {
    /// Queue priority.
    #[doc(alias = "dispatch_queue_priority_t")]
    #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct DispatchQueueGlobalPriority(pub c_long) {
        /// High priority.
        #[doc(alias = "DISPATCH_QUEUE_PRIORITY_HIGH")]
        High = 0x2,
        /// Default priority.
        #[doc(alias = "DISPATCH_QUEUE_PRIORITY_DEFAULT")]
        Default = 0x0,
        /// Low priority.
        #[doc(alias = "DISPATCH_QUEUE_PRIORITY_LOW")]
        Low = -0x2,
        /// Background priority.
        #[doc(alias = "DISPATCH_QUEUE_PRIORITY_BACKGROUND")]
        Background = i16::MIN as c_long,
    }
}

impl DispatchQueue {
    /// A well-known global concurrent queue with a given quality of service.
    #[inline]
    pub fn global_from_qos(qos: DispatchQoS) -> DispatchRetained<Self> {
        // The identifier is valid and the flags are reserved.
        Self::__global(qos.0 as isize, 0)
    }

    /// A well-known global concurrent queue with a given priority.
    ///
    /// It is recommended to use quality of service values to identify the
    /// well-known global concurrent queues, see [`Self::global_from_qos`],
    /// but the global concurrent queues may instead be identified by their
    /// priority, which map to the following QOS classes:
    /// - [`DispatchQueueGlobalPriority::High`]       -> [`DispatchQoS::UserInitiated`]
    /// - [`DispatchQueueGlobalPriority::Default`]    -> [`DispatchQoS::Default`]
    /// - [`DispatchQueueGlobalPriority::Low`]        -> [`DispatchQoS::Utility`]
    /// - [`DispatchQueueGlobalPriority::Background`] -> [`DispatchQoS::Background`]
    #[inline]
    pub fn global_from_priority(priority: DispatchQueueGlobalPriority) -> DispatchRetained<Self> {
        // The identifier is valid and the flags are reserved.
        Self::__global(priority.0 as isize, 0)
    }

    /// Return the main queue.
    // TODO: Mark this as `const` once in MSRV.
    #[inline]
    #[doc(alias = "dispatch_get_main_queue")]
    pub fn main() -> &'static Self {
        // Inline function in the header

        // SAFETY: The main queue is safe to access from anywhere, and is
        // valid forever.
        unsafe { &_dispatch_main_q }
    }

    /// Submit a function for synchronous execution on the [`DispatchQueue`].
    #[inline]
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
        unsafe { Self::exec_sync_f(self, work_boxed, function_wrapper::<F>) }
    }

    /// Submit a function for asynchronous execution on the [`DispatchQueue`].
    #[inline]
    pub fn exec_async<F>(&self, work: F)
    where
        // We need `'static` to make sure any referenced values are borrowed for
        // long enough since `work` will be performed asynchronously.
        F: Send + FnOnce() + 'static,
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast();

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe { Self::exec_async_f(self, work_boxed, function_wrapper::<F>) }
    }

    /// Enqueue a function for execution at the specified time on the [`DispatchQueue`].
    #[inline]
    pub fn after<F>(&self, when: DispatchTime, work: F)
    where
        F: Send + FnOnce(),
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast();

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe { Self::exec_after_f(when, self, work_boxed, function_wrapper::<F>) };
    }

    /// Enqueue a barrier function for asynchronous execution on the [`DispatchQueue`] and return immediately.
    #[inline]
    pub fn barrier_async<F>(&self, work: F)
    where
        // We need `'static` to make sure any referenced values are borrowed for
        // long enough since `work` will be performed asynchronously.
        F: Send + FnOnce() + 'static,
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast();

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe { Self::barrier_async_f(self, work_boxed, function_wrapper::<F>) }
    }

    /// Enqueue a barrier function for synchronous execution on the [`DispatchQueue`] and wait until that function completes.
    #[inline]
    pub fn barrier_sync<F>(&self, work: F)
    where
        F: Send + FnOnce(),
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast();

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe { Self::barrier_sync_f(self, work_boxed, function_wrapper::<F>) }
    }

    /// Submit a function for synchronous execution and mark the function as a barrier for subsequent concurrent tasks.
    #[inline]
    pub fn barrier_async_and_wait<F>(&self, work: F)
    where
        // We need `'static` to make sure any referenced values are borrowed for
        // long enough since `work` will be performed asynchronously.
        F: Send + FnOnce() + 'static,
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast();

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe { Self::barrier_async_and_wait_f(self, work_boxed, function_wrapper::<F>) }
    }

    /// Sets a function at the given key that will be executed at [`DispatchQueue`] destruction.
    #[inline]
    pub fn set_specific<F>(&self, key: NonNull<()>, destructor: F)
    where
        F: Send + FnOnce(),
    {
        let destructor_boxed = Box::into_raw(Box::new(destructor)).cast();

        // SAFETY: object cannot be null and destructor is wrapped to avoid
        // ABI incompatibility.
        //
        // The key is never dereferenced, so passing _any_ pointer here is
        // safe and allowed.
        unsafe { self.__set_specific(key.cast(), destructor_boxed, Some(function_wrapper::<F>)) }
    }

    /// Set the QOS class floor of the [`DispatchQueue`].
    #[inline]
    pub fn set_qos_class_floor(
        &self,
        qos_class: DispatchQoS,
        relative_priority: i32,
    ) -> Result<(), QualityOfServiceClassFloorError> {
        // SAFETY: We are a queue.
        unsafe { DispatchObject::set_qos_class_floor(self, qos_class, relative_priority) }
    }

    #[allow(missing_docs)]
    #[doc(alias = "DISPATCH_APPLY_AUTO")]
    pub const APPLY_AUTO: Option<&DispatchQueue> = None;

    #[allow(missing_docs)]
    #[doc(alias = "DISPATCH_TARGET_QUEUE_DEFAULT")]
    pub const TARGET_QUEUE_DEFAULT: Option<&DispatchQueue> = None;

    #[allow(missing_docs)]
    #[doc(alias = "DISPATCH_CURRENT_QUEUE_LABEL")]
    pub const CURRENT_QUEUE_LABEL: Option<&DispatchQueue> = None;
}

impl DispatchQueueAttr {
    /// A dispatch queue that executes blocks serially in FIFO order.
    #[doc(alias = "DISPATCH_QUEUE_SERIAL")]
    pub const SERIAL: Option<&Self> = None;

    // TODO(msrv): Expose this instead of a function once we bump MSRV to 1.83.
    // #[doc(alias = "DISPATCH_QUEUE_CONCURRENT")]
    // pub const CONCURRENT: Option<&Self> = {
    //     // Safety: immutable external definition
    //     unsafe { Some(&_dispatch_queue_attr_concurrent) }
    // };

    /// A dispatch queue that executes blocks concurrently.
    #[inline]
    pub fn concurrent() -> Option<&'static Self> {
        // SAFETY: Queue attributes are safe to access.
        unsafe { Some(&_dispatch_queue_attr_concurrent) }
    }
}

/// Executes blocks submitted to the main queue.
///
/// This function "parks" the main thread and waits for blocks to be submitted
/// to the main queue. This function never returns.
///
/// Applications that call NSApplicationMain() or CFRunLoopRun() on the
/// main thread do not need to call dispatch_main().
///
/// # Safety
///
/// The program must not rely on blocks submitted to the main queue to execute
/// on the main thread, see [dispatch#20] for details.
///
/// This includes e.g. [`run_on_main`][crate::run_on_main] and
/// [`MainThreadBound`][crate::MainThreadBound].
///
/// [dispatch#20]: https://github.com/SSheldon/rust-dispatch/issues/20
#[inline]
// Doesn't take `MainThreadMarker` even though it probably should; we want to
// be able to use this without depending on `objc2`.
pub unsafe fn dispatch_main() -> ! {
    extern "C" {
        // `dispatch_main` is marked DISPATCH_NOTHROW.
        fn dispatch_main() -> !;
    }

    // SAFETY: `dispatch_main` is safe to call from any thread, though it'll
    // (safely) crash if not called from the main thread.
    unsafe { dispatch_main() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::ffi::CStr;

    #[test]
    fn test_create_main_queue() {
        let _ = DispatchQueue::main();
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_serial_queue() {
        let queue = DispatchQueue::new(
            Some(CStr::from_bytes_with_nul(b"com.github.madsmtm.objc2\0").unwrap()),
            DispatchQueueAttr::SERIAL,
        );
        let (tx, rx) = std::sync::mpsc::channel();
        queue.exec_async(move || {
            tx.send(()).unwrap();
        });
        rx.recv().unwrap();
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_concurrent_queue() {
        let queue = DispatchQueue::new(
            Some(CStr::from_bytes_with_nul(b"com.github.madsmtm.objc2\0").unwrap()),
            DispatchQueueAttr::concurrent(),
        );
        let (tx, rx) = std::sync::mpsc::channel();
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
    fn test_global_queues() {
        let _qos_user_interactive = DispatchQueue::global_from_qos(DispatchQoS::UserInteractive);
        let qos_user_initiated = DispatchQueue::global_from_qos(DispatchQoS::UserInitiated);
        let qos_default = DispatchQueue::global_from_qos(DispatchQoS::Default);
        let qos_utility = DispatchQueue::global_from_qos(DispatchQoS::Utility);
        let qos_background = DispatchQueue::global_from_qos(DispatchQoS::Background);
        let qos_unspecified = DispatchQueue::global_from_qos(DispatchQoS::Unspecified);

        assert_eq!(qos_unspecified, qos_default);

        let global_high = DispatchQueue::global_from_priority(DispatchQueueGlobalPriority::High);
        let global_default =
            DispatchQueue::global_from_priority(DispatchQueueGlobalPriority::Default);
        let global_low = DispatchQueue::global_from_priority(DispatchQueueGlobalPriority::Low);
        let global_background =
            DispatchQueue::global_from_priority(DispatchQueueGlobalPriority::Background);

        assert_eq!(global_high, qos_user_initiated);
        assert_eq!(global_default, qos_default);
        assert_eq!(global_low, qos_utility);
        assert_eq!(global_background, qos_background);
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_global_default_queue() {
        let queue = DispatchQueue::global_from_qos(DispatchQoS::Default);
        let (tx, rx) = std::sync::mpsc::channel();
        queue.exec_async(move || {
            tx.send(()).unwrap();
        });
        rx.recv().unwrap();
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_share_queue_across_threads() {
        let queue = DispatchQueue::new(
            Some(CStr::from_bytes_with_nul(b"com.github.madsmtm.objc2\0").unwrap()),
            DispatchQueueAttr::SERIAL,
        );
        let (tx, rx) = std::sync::mpsc::channel();
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
    #[cfg(feature = "std")]
    fn test_move_queue_between_threads() {
        let queue = DispatchQueue::new(
            Some(CStr::from_bytes_with_nul(b"com.github.madsmtm.objc2\0").unwrap()),
            DispatchQueueAttr::SERIAL,
        );
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            queue.exec_async(move || {
                tx.send(()).unwrap();
            });
        });
        rx.recv().unwrap();
    }
}
