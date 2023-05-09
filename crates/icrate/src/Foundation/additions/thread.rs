#![cfg(feature = "Foundation_NSThread")]
use core::fmt;
use core::marker::PhantomData;
use core::mem::{self, ManuallyDrop};
use core::panic::{RefUnwindSafe, UnwindSafe};

use crate::common::*;
use crate::Foundation::NSThread;

use objc2::msg_send_id;

unsafe impl Send for NSThread {}
unsafe impl Sync for NSThread {}

impl UnwindSafe for NSThread {}
impl RefUnwindSafe for NSThread {}

/// Whether the application is multithreaded according to Cocoa.
#[cfg(feature = "Foundation_NSThread")]
pub fn is_multi_threaded() -> bool {
    NSThread::isMultiThreaded()
}

/// Whether the current thread is the main thread.
#[cfg(feature = "Foundation_NSThread")]
pub fn is_main_thread() -> bool {
    NSThread::isMainThread_class()
}

#[allow(unused)]
#[cfg(feature = "Foundation_NSThread")]
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
    #[cfg(feature = "Foundation_NSThread")]
    #[inline]
    pub fn new() -> Option<Self> {
        if NSThread::isMainThread_class() {
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
    pub unsafe fn new_unchecked() -> Self {
        // SAFETY: Upheld by caller
        //
        // We can't debug_assert that this actually is the main thread, see
        // the comment above.
        Self { _priv: PhantomData }
    }

    /// Allocate a new instance of the specified class on the main thread.
    ///
    /// This is essentially the same as [`ClassType::alloc`], the difference
    /// being that it is also callable with classes that can only be used on
    /// the main thread.
    ///
    ///
    /// # Example
    ///
    /// Create an object on the main thread.
    ///
    /// ```
    /// use icrate::Foundation::MainThreadMarker;
    /// # use icrate::Foundation::NSObject as SomeClass;
    /// # #[cfg(for_example)]
    /// use icrate::SomeFramework::SomeClass;
    /// use objc2::rc::Id;
    /// use objc2::msg_send_id;
    ///
    /// # let mtm = unsafe { MainThreadMarker::new_unchecked() };
    /// # #[cfg(doctests_not_always_run_on_main_thread)]
    /// let mtm = MainThreadMarker::new().expect("must be on the main thread");
    ///
    /// // _All_ objects are safe to allocate on the main thread!
    /// let obj = mtm.alloc::<SomeClass>();
    ///
    /// // Though more knowledge is required for safe initialization
    /// let obj: Id<SomeClass> = unsafe { msg_send_id![obj, init] };
    /// ```
    #[inline]
    pub fn alloc<T: ClassType>(self) -> Option<Allocated<T>> {
        // SAFETY: Same as `ClassType::alloc`, with the addition that since we
        // take `self: MainThreadMarker`, the `IsAllocableAnyThread` bound is
        // not required.
        unsafe { msg_send_id![T::class(), alloc] }
    }

    /// Submit the given closure to the runloop on the main thread.
    ///
    /// If the current thread is the main thread, this simply runs the
    /// closure.
    ///
    /// The closure is passed a [`MainThreadMarker`] that it can further use
    /// to access APIs that are only accessible from the main thread.
    ///
    /// This function should only be used in applications whose main thread is
    /// running an event loop with `dispatch_main`, `UIApplicationMain`,
    /// `NSApplicationMain`, `CFRunLoop` or similar; it will block
    /// indefinitely if that is not the case.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// use icrate::Foundation::MainThreadMarker;
    /// MainThreadMarker::run_on_main(|mtm| {
    ///     // Do something on the main thread with the given marker
    /// });
    /// ```
    #[cfg(feature = "dispatch")]
    pub fn run_on_main<F, R>(f: F) -> R
    where
        F: Send + FnOnce(MainThreadMarker) -> R,
        R: Send,
    {
        if let Some(mtm) = MainThreadMarker::new() {
            f(mtm)
        } else {
            dispatch::Queue::main().exec_sync(|| {
                // SAFETY: The outer closure is submitted to run on the main
                // thread, so now, when the closure actually runs, it's
                // guaranteed to be on the main thread.
                f(unsafe { MainThreadMarker::new_unchecked() })
            })
        }
    }
}

impl fmt::Debug for MainThreadMarker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MainThreadMarker").finish()
    }
}

/// Make a type that can only be used on the main thread be `Send` + `Sync`.
///
/// On `Drop`, the inner type is sent to the main thread's runloop and dropped
/// there. This may lead to deadlocks if the main runloop is not running, or
/// if it is waiting on a lock that the dropping thread is holding. See
/// [`MainThreadMarker::run_on_main`] for some of the caveats around that.
///
///
/// # Related
///
/// This type takes inspiration from `threadbound::ThreadBound`.
///
/// The functionality also somewhat resembles Swift's `@MainActor`, which
/// ensures that a type is only usable from the main thread.
#[doc(alias = "@MainActor")]
#[cfg(feature = "dispatch")]
pub struct MainThreadBound<T>(ManuallyDrop<T>);

// SAFETY: The inner value is guaranteed to originate from the main thread
// because `new` takes [`MainThreadMarker`].
//
// `into_inner` is the only way to get the value out, and that is also
// guaranteed to happen on the main thread.
//
// Finally, the value is dropped on the main thread in `Drop`.
#[cfg(feature = "dispatch")]
unsafe impl<T> Send for MainThreadBound<T> {}

// SAFETY: We only provide access to the inner value via. `get` and `get_mut`.
//
// Both of these take [`MainThreadMarker`], which guarantees that the access
// is done from the main thread.
#[cfg(feature = "dispatch")]
unsafe impl<T> Sync for MainThreadBound<T> {}

#[cfg(feature = "dispatch")]
impl<T> Drop for MainThreadBound<T> {
    fn drop(&mut self) {
        if mem::needs_drop::<T>() {
            // TODO: Figure out whether we should assume the main thread to be
            // dead if we're panicking, and just leak instead?
            MainThreadMarker::run_on_main(|_mtm| {
                let this = self;
                // SAFETY: The value is dropped on the main thread, which is
                // the same thread that it originated from (guaranteed by
                // `new` taking `MainThreadMarker`).
                //
                // Additionally, the value is never used again after this
                // point.
                unsafe { ManuallyDrop::drop(&mut this.0) };
            })
        }
    }
}

/// Main functionality.
#[cfg(feature = "dispatch")]
impl<T> MainThreadBound<T> {
    /// Create a new [`MainThreadBound`] value of type `T`.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// use icrate::Foundation::{MainThreadMarker, MainThreadBound};
    ///
    /// let foo;
    /// # foo = ();
    /// let mtm = MainThreadMarker::new().expect("must be on the main thread");
    /// let foo = MainThreadBound::new(foo, mtm);
    ///
    /// // `foo` is now `Send + Sync`.
    /// ```
    #[inline]
    pub fn new(inner: T, _mtm: MainThreadMarker) -> Self {
        Self(ManuallyDrop::new(inner))
    }

    /// Returns a reference to the value.
    #[inline]
    pub fn get(&self, _mtm: MainThreadMarker) -> &T {
        &self.0
    }

    /// Returns a mutable reference to the value.
    #[inline]
    pub fn get_mut(&mut self, _mtm: MainThreadMarker) -> &mut T {
        &mut self.0
    }

    /// Extracts the value from the [`MainThreadBound`] container.
    #[inline]
    pub fn into_inner(self, _mtm: MainThreadMarker) -> T {
        // Prevent our `Drop` impl from running.
        //
        // This is a bit confusing, now `this` is:
        // `ManuallyDrop<Self(ManuallyDrop<T>)>`
        let mut this = ManuallyDrop::new(self);

        // SAFETY: `self` is consumed by this function, and wrapped in
        // `ManuallyDrop`, so the item's destructor is never run.
        unsafe { ManuallyDrop::take(&mut this.0) }
    }
}

/// Helper functions for running [`MainThreadMarker::run_on_main`].
#[cfg(feature = "dispatch")]
impl<T> MainThreadBound<T> {
    /// Access the item on the main thread.
    ///
    /// See [`MainThreadMarker::run_on_main`] for caveats.
    #[inline]
    pub fn get_on_main<F, R>(&self, f: F) -> R
    where
        F: Send + FnOnce(&T, MainThreadMarker) -> R,
        R: Send,
    {
        MainThreadMarker::run_on_main(|mtm| f(self.get(mtm), mtm))
    }

    /// Access the item mutably on the main thread.
    ///
    /// See [`MainThreadMarker::run_on_main`] for caveats.
    #[inline]
    pub fn get_on_main_mut<F, R>(&mut self, f: F) -> R
    where
        F: Send + FnOnce(&mut T, MainThreadMarker) -> R,
        R: Send,
    {
        MainThreadMarker::run_on_main(|mtm| f(self.get_mut(mtm), mtm))
    }
}

#[cfg(feature = "dispatch")]
impl<T> fmt::Debug for MainThreadBound<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MainThreadBound").finish_non_exhaustive()
    }
}
