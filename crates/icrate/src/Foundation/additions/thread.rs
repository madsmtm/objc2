use core::fmt;
use core::marker::PhantomData;
use core::panic::{RefUnwindSafe, UnwindSafe};

use objc2::extern_methods;
use objc2::rc::{Id, Shared};
use objc2::ClassType;

use crate::Foundation::NSThread;

unsafe impl Send for NSThread {}
unsafe impl Sync for NSThread {}

impl UnwindSafe for NSThread {}
impl RefUnwindSafe for NSThread {}

extern_methods!(
    unsafe impl NSThread {
        #[method_id(new)]
        unsafe fn new() -> Id<Self, Shared>;
    }
);

/// Whether the application is multithreaded according to Cocoa.
pub fn is_multi_threaded() -> bool {
    NSThread::isMultiThreaded()
}

/// Whether the current thread is the main thread.
pub fn is_main_thread() -> bool {
    NSThread::class_isMainThread()
}

#[allow(unused)]
fn make_multithreaded() {
    let thread = unsafe { NSThread::new() };
    unsafe { thread.start() };
    // Don't bother waiting for it to complete!
}

/// A marker type taken by functions that can only be executed on the main
/// thread.
///
/// By design, this is neither [`Send`] nor [`Sync`], and can only be created
/// on the main thread, meaning that if you're holding this, you know you're
/// running on the main thread.
///
/// See the following links for more information on main-thread-only APIs:
/// - [Main Thread Only APIs on OS X](https://www.dribin.org/dave/blog/archives/2009/02/01/main_thread_apis/)
/// - [Thread Safety Summary](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Multithreading/ThreadSafetySummary/ThreadSafetySummary.html#//apple_ref/doc/uid/10000057i-CH12-SW1)
/// - [Are the Cocoa Frameworks Thread Safe?](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/CocoaFundamentals/AddingBehaviortoaCocoaProgram/AddingBehaviorCocoa.html#//apple_ref/doc/uid/TP40002974-CH5-SW47)
/// - [Technical Note TN2028 - Threading Architectures](https://developer.apple.com/library/archive/technotes/tn/tn2028.html#//apple_ref/doc/uid/DTS10003065)
/// - [Thread Management](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Multithreading/CreatingThreads/CreatingThreads.html)
/// - [Mike Ash' article on thread safety](https://www.mikeash.com/pyblog/friday-qa-2009-01-09.html)
///
///
/// # Examples
///
/// Use when designing APIs that are only safe to use on the main thread:
///
/// ```no_run
/// use icrate::Foundation::MainThreadMarker;
/// use icrate::objc2::runtime::Object;
/// use icrate::objc2::msg_send;
/// # let obj = 0 as *const Object;
///
/// // This action requires the main thread, so we take a marker as parameter.
/// // It signals clearly to users "this requires the main thread".
/// unsafe fn do_thing(obj: *const Object, _mtm: MainThreadMarker) {
///     msg_send![obj, someActionThatRequiresTheMainThread]
/// }
///
/// // Usage
/// let mtm = MainThreadMarker::new().unwrap();
/// unsafe { do_thing(obj, mtm) }
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
// This is valid to Copy because it's still `!Send` and `!Sync`.
pub struct MainThreadMarker {
    // No lifetime information needed; the main thread is static and available
    // throughout the entire program!
    _priv: PhantomData<*mut ()>,
}

impl MainThreadMarker {
    /// Construct a new [`MainThreadMarker`].
    ///
    /// Returns [`None`] if the current thread was not the main thread.
    pub fn new() -> Option<Self> {
        if is_main_thread() {
            // SAFETY: We just checked that we are running on the main thread.
            Some(unsafe { Self::new_unchecked() })
        } else {
            None
        }
    }

    /// Construct a new [`MainThreadMarker`] without first checking whether
    /// the current thread is the main one.
    ///
    /// # Safety
    ///
    /// The current thread must be the main thread.
    pub unsafe fn new_unchecked() -> Self {
        // SAFETY: Upheld by caller
        Self { _priv: PhantomData }
    }
}

impl fmt::Debug for MainThreadMarker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MainThreadMarker").finish()
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;
    use core::panic::{RefUnwindSafe, UnwindSafe};

    use super::*;

    #[test]
    #[cfg_attr(
        feature = "gnustep-1-7",
        ignore = "Retrieving main thread is weirdly broken, only works with --test-threads=1"
    )]
    fn test_main_thread() {
        let current = NSThread::currentThread();
        let main = NSThread::mainThread();

        assert!(main.isMainThread());

        if main == current {
            assert!(current.isMainThread());
            assert!(is_main_thread());
        } else {
            assert!(!current.isMainThread());
            assert!(!is_main_thread());
        }
    }

    #[test]
    fn test_not_main_thread() {
        let res =
            std::thread::spawn(|| (is_main_thread(), NSThread::currentThread().isMainThread()))
                .join()
                .unwrap();
        assert_eq!(res, (false, false));
    }

    #[test]
    fn test_main_thread_auto_traits() {
        fn assert_traits<T: Unpin + UnwindSafe + RefUnwindSafe + Sized>() {}

        assert_traits::<MainThreadMarker>()
    }

    #[test]
    #[cfg_attr(
        feature = "gnustep-1-7",
        ignore = "Retrieving main thread is weirdly broken, only works with --test-threads=1"
    )]
    fn test_debug() {
        let thread = NSThread::mainThread();

        let actual = format!("{thread:?}");
        let expected = [
            // macOS 11
            format!("<NSThread: {thread:p}>{{number = 1, name = (null)}}"),
            format!("<NSThread: {thread:p}>{{number = 1, name = main}}"),
            // macOS 12
            format!("<_NSMainThread: {thread:p}>{{number = 1, name = (null)}}"),
            format!("<_NSMainThread: {thread:p}>{{number = 1, name = main}}"),
        ];
        assert!(
            expected.contains(&actual),
            "Expected one of {expected:?}, got {actual:?}",
        );

        // SAFETY: We don't use the marker for anything other than its Debug
        // impl, so this test doesn't actually need to run on the main thread!
        let marker = unsafe { MainThreadMarker::new_unchecked() };
        assert_eq!(format!("{marker:?}"), "MainThreadMarker");
    }
}
