use core::fmt;
use core::marker::PhantomData;

use crate::rc::Allocated;
use crate::{msg_send_id, ClassType, MainThreadOnly};

/// Whether the current thread is the main thread.
#[inline]
fn is_main_thread() -> bool {
    #[cfg(target_vendor = "apple")]
    {
        // Normally you would use `+[NSThread isMainThread]`, but benchmarks
        // have shown that calling the underlying `pthread_main_np` directly
        // is up to four times faster, so we use that instead.

        // SAFETY: The signature in here is the exact same as in `libc`.
        //
        // `pthread_main_np` is included via `libSystem` when `libstd` is
        // linked. All of this is done to avoid a dependency on the `libc`
        // crate.
        //
        // `extern "C"` is safe because this will never unwind.
        #[cfg_attr(not(feature = "std"), link(name = "c", kind = "dylib"))]
        extern "C" {
            fn pthread_main_np() -> core::ffi::c_int;
        }

        // SAFETY: Can be called from any thread.
        unsafe { pthread_main_np() != 0 }
    }

    #[cfg(not(target_vendor = "apple"))]
    {
        // Fall back to isMainThread on non-Apple platforms, as
        // `pthread_main_np` is not always available there.
        unsafe { crate::msg_send![crate::class!(NSThread), isMainThread] }
    }
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
/// # Main Thread Checker
///
/// Xcode provides a tool called the ["Main Thread Checker"][mtc] which
/// verifies that UI APIs are being used from the correct thread. This is not
/// as principled as `MainThreadMarker`, but is helpful for catching mistakes.
///
/// You can use this tool on macOS by loading `libMainThreadChecker.dylib`
/// into your process using `DYLD_INSERT_LIBRARIES`:
///
/// ```console
/// DYLD_INSERT_LIBRARIES=/Applications/Xcode.app/Contents/Developer/usr/lib/libMainThreadChecker.dylib MTC_RESET_INSERT_LIBRARIES=0 cargo run
/// ```
///
/// If you're not running your binary through Cargo, you can omit
/// [`MTC_RESET_INSERT_LIBRARIES`][mtc-reset].
///
/// ```console
/// DYLD_INSERT_LIBRARIES=/Applications/Xcode.app/Contents/Developer/usr/lib/libMainThreadChecker.dylib target/debug/myapp
/// ```
///
/// If you're developing for iOS, you probably better off enabling the tool in
/// Xcode's own UI.
///
/// See [this excellent blog post][mtc-cfg] for details on further
/// configuration options.
///
/// [mtc]: https://developer.apple.com/documentation/xcode/diagnosing-memory-thread-and-crash-issues-early#Detect-improper-UI-updates-on-background-threads
/// [mtc-reset]: https://bryce.co/main-thread-checker-configuration/#mtc_reset_insert_libraries
/// [mtc-cfg]: https://bryce.co/main-thread-checker-configuration/
///
///
/// # Examples
///
/// Use when designing APIs that are only safe to use on the main thread:
///
/// ```no_run
/// use objc2::{MainThreadMarker, msg_send};
/// use objc2::runtime::NSObject;
/// # let obj = 0 as *const NSObject;
///
/// // This action requires the main thread, so we take a marker as parameter.
/// // It signals clearly to users "this requires the main thread".
/// unsafe fn do_thing(obj: *const NSObject, _mtm: MainThreadMarker) {
///     msg_send![obj, someActionThatRequiresTheMainThread]
/// }
///
/// // Usage
///
/// // Create a new MainThreadMarker.
/// let mtm = MainThreadMarker::new().expect("must be on the main thread");
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
    #[inline]
    #[doc(alias = "is_main_thread")]
    #[doc(alias = "pthread_main_np")]
    #[doc(alias = "isMainThread")]
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
    ///
    /// # Safety
    ///
    /// The current thread must be the main thread.
    ///
    /// Alternatively, you may create this briefly if you know that a an API
    /// is safe in a specific case, but is not marked so. If you do that, you
    /// must ensure that any use of the marker is actually safe to do from
    /// another thread than the main one.
    #[inline]
    pub const unsafe fn new_unchecked() -> Self {
        // SAFETY: Upheld by caller
        //
        // We can't debug_assert that this actually is the main thread, see
        // the comment above.
        Self { _priv: PhantomData }
    }

    /// Allocate a new instance of the specified class on the main thread.
    ///
    /// This can be useful in certain situations, such as generic contexts
    /// where you don't know whether the class is main thread or not, but
    /// usually you should prefer [`MainThreadOnly::alloc`].
    #[inline]
    pub fn alloc<T: ClassType>(self) -> Allocated<T> {
        // SAFETY: We hold `MainThreadMarker`, and classes are either only
        // safe to allocate on the main thread, or safe to allocate
        // everywhere.
        unsafe { msg_send_id![T::class(), alloc] }
    }
}

/// Get a [`MainThreadMarker`] from a main-thread-only object.
///
/// This is a shorthand for [`MainThreadOnly::mtm`].
impl<T: ?Sized + MainThreadOnly> From<&T> for MainThreadMarker {
    #[inline]
    fn from(obj: &T) -> Self {
        obj.mtm()
    }
}

impl fmt::Debug for MainThreadMarker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MainThreadMarker").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{RefUnwindSafe, UnwindSafe};

    static_assertions::assert_impl_all!(MainThreadMarker: Unpin, UnwindSafe, RefUnwindSafe, Sized);
    static_assertions::assert_not_impl_any!(MainThreadMarker: Send, Sync);

    #[test]
    fn debug() {
        // SAFETY: We don't use the marker for anything other than its Debug
        // impl, so this test doesn't actually need to run on the main thread!
        let marker = unsafe { MainThreadMarker::new_unchecked() };
        assert_eq!(std::format!("{marker:?}"), "MainThreadMarker");
    }

    #[test]
    fn test_not_main_thread() {
        let res = std::thread::spawn(|| MainThreadMarker::new().is_none())
            .join()
            .unwrap();
        assert!(res);
    }
}
