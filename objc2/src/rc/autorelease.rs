use core::cell::UnsafeCell;
use core::ffi::c_void;
use core::fmt;
use core::marker::PhantomData;
#[cfg(all(debug_assertions, not(feature = "unstable_autoreleasesafe")))]
use std::{cell::RefCell, thread_local, vec::Vec};

use crate::ffi;

/// An Objective-C autorelease pool.
///
/// The pool is drained when dropped.
///
/// This is not [`Send`], since `objc_autoreleasePoolPop` must be called on
/// the same thread.
///
/// And this is not [`Sync`], since you can only autorelease a reference to a
/// pool on the current thread.
#[derive(Debug)]
pub struct AutoreleasePool {
    context: *mut c_void,
    // May pointer to data that is mutated (even though we hold shared access)
    p: PhantomData<*mut UnsafeCell<c_void>>,
}

/// ```
/// use objc2::rc::AutoreleasePool;
/// fn needs_nothing<T>() {}
/// needs_nothing::<AutoreleasePool>();
/// ```
/// ```compile_fail
/// use objc2::rc::AutoreleasePool;
/// fn needs_sync<T: Sync>() {}
/// needs_sync::<AutoreleasePool>();
/// ```
/// ```compile_fail
/// use objc2::rc::AutoreleasePool;
/// fn needs_send<T: Send>() {}
/// needs_send::<AutoreleasePool>();
/// ```
#[cfg(doctest)]
pub struct AutoreleasePoolNotSendNorSync;

#[cfg(all(debug_assertions, not(feature = "unstable_autoreleasesafe")))]
thread_local! {
    /// We track the thread's pools to verify that object lifetimes are only
    /// taken from the innermost pool.
    static POOLS: RefCell<Vec<*mut c_void>> = RefCell::new(Vec::new());
}

impl AutoreleasePool {
    /// Construct a new autorelease pool.
    ///
    /// Use the [`autoreleasepool`] block for a safe alternative.
    ///
    /// # Safety
    ///
    /// The caller must ensure that when handing out `&'p AutoreleasePool` to
    /// functions that this is the innermost pool.
    ///
    /// Additionally, the pools must be dropped in the same order they were
    /// created.
    #[doc(alias = "objc_autoreleasePoolPush")]
    unsafe fn new() -> Self {
        // TODO: Make this function pub when we're more certain of the API
        let context = unsafe { ffi::objc_autoreleasePoolPush() };
        #[cfg(all(debug_assertions, not(feature = "unstable_autoreleasesafe")))]
        POOLS.with(|c| c.borrow_mut().push(context));
        Self {
            context,
            p: PhantomData,
        }
    }

    /// This will be removed in a future version.
    #[cfg_attr(
        not(all(debug_assertions, not(feature = "unstable_autoreleasesafe"))),
        inline
    )]
    #[doc(hidden)]
    pub fn __verify_is_inner(&self) {
        #[cfg(all(debug_assertions, not(feature = "unstable_autoreleasesafe")))]
        POOLS.with(|c| {
            assert_eq!(
                c.borrow().last(),
                Some(&self.context),
                "Tried to use lifetime from pool that was not innermost"
            )
        });
    }

    /// Returns a shared reference to the given autoreleased pointer object.
    ///
    /// This is the preferred way to make references from autoreleased
    /// objects, since it binds the lifetime of the reference to the pool, and
    /// does some extra checks when debug assertions are enabled.
    ///
    /// For the mutable counterpart see [`ptr_as_mut`](#method.ptr_as_mut).
    ///
    /// # Safety
    ///
    /// This is equivalent to `&*ptr`, and shares the unsafety of that, except
    /// the lifetime is bound to the pool instead of being unbounded.
    #[inline]
    #[allow(clippy::needless_lifetimes)]
    pub unsafe fn ptr_as_ref<'p, T: ?Sized>(&'p self, ptr: *const T) -> &'p T {
        self.__verify_is_inner();
        // SAFETY: Checked by the caller
        unsafe { &*ptr }
    }

    /// Returns a unique reference to the given autoreleased pointer object.
    ///
    /// This is the preferred way to make mutable references from autoreleased
    /// objects, since it binds the lifetime of the reference to the pool, and
    /// does some extra checks when debug assertions are enabled.
    ///
    /// For the shared counterpart see [`ptr_as_ref`](#method.ptr_as_ref).
    ///
    /// # Safety
    ///
    /// This is equivalent to `&mut *ptr`, and shares the unsafety of that,
    /// except the lifetime is bound to the pool instead of being unbounded.
    #[inline]
    #[allow(clippy::needless_lifetimes)]
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn ptr_as_mut<'p, T: ?Sized>(&'p self, ptr: *mut T) -> &'p mut T {
        self.__verify_is_inner();
        // SAFETY: Checked by the caller
        unsafe { &mut *ptr }
    }
}

impl Drop for AutoreleasePool {
    /// Drains the autoreleasepool.
    ///
    /// The [clang documentation] says that `@autoreleasepool` blocks are not
    /// drained when exceptions occur because:
    ///
    /// > Not draining the pool during an unwind is apparently required by the
    /// > Objective-C exceptions implementation.
    ///
    /// This was true in the past, but since [revision `371`] of
    /// `objc-exception.m` (ships with MacOS 10.5) the exception is now
    /// retained when `@throw` is encountered.
    ///
    /// Hence it is safe to drain the pool when unwinding.
    ///
    /// [clang documentation]: https://clang.llvm.org/docs/AutomaticReferenceCounting.html#autoreleasepool
    /// [revision `371`]: https://opensource.apple.com/source/objc4/objc4-371/runtime/objc-exception.m.auto.html
    #[doc(alias = "objc_autoreleasePoolPop")]
    fn drop(&mut self) {
        unsafe { ffi::objc_autoreleasePoolPop(self.context) }
        #[cfg(all(debug_assertions, not(feature = "unstable_autoreleasesafe")))]
        POOLS.with(|c| {
            assert_eq!(
                c.borrow_mut().pop(),
                Some(self.context),
                "Popped pool that was not the innermost pool"
            )
        });
    }
}

impl fmt::Pointer for AutoreleasePool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.context, f)
    }
}

/// We use a macro here so that the documentation is included whether the
/// feature is enabled or not.
#[cfg(not(feature = "unstable_autoreleasesafe"))]
macro_rules! auto_trait {
    {$(#[$fn_meta:meta])* $v:vis unsafe trait AutoreleaseSafe {}} => {
        $(#[$fn_meta])*
        $v unsafe trait AutoreleaseSafe {}
    }
}

#[cfg(feature = "unstable_autoreleasesafe")]
macro_rules! auto_trait {
    {$(#[$fn_meta:meta])* $v:vis unsafe trait AutoreleaseSafe {}} => {
        $(#[$fn_meta])*
        $v unsafe auto trait AutoreleaseSafe {}
    }
}

auto_trait! {
    /// Marks types that are safe to pass across the closure in an
    /// [`autoreleasepool`].
    ///
    /// With the `unstable_autoreleasesafe` feature enabled, this is an auto
    /// trait that is implemented for all types except [`AutoreleasePool`].
    ///
    /// Otherwise it is just a dummy trait that is implemented for all types;
    /// the safety invariants are checked with debug assertions instead.
    ///
    /// You should not normally need to implement this trait yourself.
    ///
    /// # Safety
    ///
    /// Must not be implemented for types that interract with the autorelease
    /// pool. So if you reimplement the [`AutoreleasePool`] struct or
    /// likewise, this should be negatively implemented for that.
    ///
    /// This can easily be accomplished with an `PhantomData<AutoreleasePool>`
    /// if the `unstable_autoreleasesafe` feature is enabled.
    pub unsafe trait AutoreleaseSafe {}
}

#[cfg(not(feature = "unstable_autoreleasesafe"))]
unsafe impl<T: ?Sized> AutoreleaseSafe for T {}

#[cfg(feature = "unstable_autoreleasesafe")]
impl !AutoreleaseSafe for AutoreleasePool {}

/// Execute `f` in the context of a new autorelease pool. The pool is
/// drained after the execution of `f` completes.
///
/// This corresponds to `@autoreleasepool` blocks in Objective-C and
/// Swift.
///
/// The pool is passed as a reference to the enclosing function to give it
/// a lifetime parameter that autoreleased objects can refer to.
///
/// The given reference must not be used in an inner `autoreleasepool`,
/// doing so will panic with debug assertions enabled, and be a compile
/// error in a future release. You can test the compile error with the
/// `unstable_autoreleasesafe` crate feature on nightly Rust.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use objc2::{class, msg_send};
/// use objc2::rc::{autoreleasepool, AutoreleasePool};
/// use objc2::runtime::Object;
///
/// fn needs_lifetime_from_pool<'p>(pool: &'p AutoreleasePool) -> &'p mut Object {
///     let obj: *mut Object = unsafe { msg_send![class!(NSObject), new] };
///     let obj: *mut Object = unsafe { msg_send![obj, autorelease] };
///     // Lifetime of the returned reference is bounded by the pool
///     unsafe { pool.ptr_as_mut(obj) }
/// }
///
/// autoreleasepool(|pool| {
///     // Create `obj` and autorelease it to the pool
///     let obj = needs_lifetime_from_pool(pool);
///     // ... use `obj` here
///     // `obj` is deallocated when the pool ends
/// });
/// ```
///
/// Fails to compile because `obj` does not live long enough for us to
/// safely take it out of the pool:
///
/// ```compile_fail
/// # use objc2::{class, msg_send};
/// # use objc2::rc::{autoreleasepool, AutoreleasePool};
/// # use objc2::runtime::Object;
/// #
/// # fn needs_lifetime_from_pool<'p>(pool: &'p AutoreleasePool) -> &'p mut Object {
/// #     let obj: *mut Object = unsafe { msg_send![class!(NSObject), new] };
/// #     let obj: *mut Object = unsafe { msg_send![obj, autorelease] };
/// #     unsafe { pool.ptr_as_mut(obj) }
/// # }
/// #
/// let obj = autoreleasepool(|pool| {
///     let obj = needs_lifetime_from_pool(pool);
///     // Use `obj`
///     obj
/// });
/// ```
///
/// Incorrect usage which panics because we tried to pass an outer pool to an
/// inner pool:
///
#[cfg_attr(feature = "unstable_autoreleasesafe", doc = "```compile_fail")]
#[cfg_attr(not(feature = "unstable_autoreleasesafe"), doc = "```should_panic")]
/// # use objc2::{class, msg_send};
/// # use objc2::rc::{autoreleasepool, AutoreleasePool};
/// # use objc2::runtime::Object;
/// #
/// # fn needs_lifetime_from_pool<'p>(pool: &'p AutoreleasePool) -> &'p mut Object {
/// #     let obj: *mut Object = unsafe { msg_send![class!(NSObject), new] };
/// #     let obj: *mut Object = unsafe { msg_send![obj, autorelease] };
/// #     unsafe { pool.ptr_as_mut(obj) }
/// # }
/// #
/// autoreleasepool(|outer_pool| {
///     let obj = autoreleasepool(|inner_pool| {
///         let obj = needs_lifetime_from_pool(outer_pool);
///         obj
///     });
///     // `obj` could wrongly be used here because it's lifetime was
///     // assigned to the outer pool, even though it was released by the
///     // inner pool already.
/// });
/// ```
#[doc(alias = "@autoreleasepool")]
pub fn autoreleasepool<T, F>(f: F) -> T
where
    for<'p> F: FnOnce(&'p AutoreleasePool) -> T + AutoreleaseSafe,
{
    let pool = unsafe { AutoreleasePool::new() };
    f(&pool)
}

#[cfg(all(test, feature = "unstable_autoreleasesafe"))]
mod tests {
    use super::AutoreleaseSafe;
    use crate::runtime::Object;

    fn requires_autoreleasesafe<T: AutoreleaseSafe>() {}

    #[test]
    fn test_autoreleasesafe() {
        requires_autoreleasesafe::<usize>();
        requires_autoreleasesafe::<*mut Object>();
        requires_autoreleasesafe::<&mut Object>();
    }
}
