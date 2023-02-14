use core::ffi::c_void;
#[cfg(not(all(debug_assertions, not(feature = "unstable-autoreleasesafe"))))]
use core::marker::PhantomData;
#[cfg(all(debug_assertions, not(feature = "unstable-autoreleasesafe")))]
use std::{cell::RefCell, thread_local, vec::Vec};

use crate::ffi;

/// The actual pool object.
///
/// It is drained when dropped.
///
/// This is not [`Send`], since `objc_autoreleasePoolPop` must be called on
/// the same thread as `objc_autoreleasePoolPush`.
///
/// And this is not [`Sync`], since that would make `AutoreleasePool` `Send`.
#[derive(Debug)]
struct Pool {
    /// This is an opaque handle, and is not guaranteed to be neither a valid
    /// nor an aligned pointer.
    context: *mut c_void,
}

impl Pool {
    /// Construct a new autorelease pool.
    ///
    ///
    /// # Safety
    ///
    /// The caller must ensure that when handing out `AutoreleasePool<'p>` to
    /// functions that this is the innermost pool.
    ///
    /// Additionally, the pools must be dropped in the same order they were
    /// created.
    #[inline]
    unsafe fn new() -> Self {
        let context = unsafe { ffi::objc_autoreleasePoolPush() };
        #[cfg(all(debug_assertions, not(feature = "unstable-autoreleasesafe")))]
        POOLS.with(|c| c.borrow_mut().push(context));
        Self { context }
    }
}

impl Drop for Pool {
    /// Drains the autoreleasepool.
    ///
    /// The [clang documentation] says that `@autoreleasepool` blocks are not
    /// drained when exceptions occur because:
    ///
    /// > Not draining the pool during an unwind is apparently required by the
    /// > Objective-C exceptions implementation.
    ///
    /// However, we would like to do this anyway whenever possible, since the
    /// unwind is probably caused by Rust, and forgetting to pop the pool will
    /// likely leak memory.
    ///
    /// Fortunately, the above statement was true in the past, but since
    /// [revision `371`] of objc4 (ships with MacOS 10.5) the exception is now
    /// retained when `@throw` is encountered.
    ///
    /// Hence it is safe to drain the pool when unwinding.
    ///
    /// TODO: Verify this claim on 32bit!
    ///
    /// [clang documentation]: https://clang.llvm.org/docs/AutomaticReferenceCounting.html#autoreleasepool
    /// [revision `371`]: https://github.com/apple-oss-distributions/objc4/blob/objc4-371/runtime/objc-exception.m#L479-L482
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::objc_autoreleasePoolPop(self.context) }
        #[cfg(all(debug_assertions, not(feature = "unstable-autoreleasesafe")))]
        POOLS.with(|c| {
            assert_eq!(
                c.borrow_mut().pop(),
                Some(self.context),
                "Popped pool that was not the innermost pool"
            )
        });
    }
}

/// An Objective-C autorelease pool.
///
/// Autorelease pools are a way to store objects in a certain thread-local
/// scope, such that they are only released at the end of said scope.
///
/// See [`autoreleasepool`] for how to use this.
///
/// This is not [`Send`] nor [`Sync`], since you can only autorelease a
/// reference to a pool on the current thread (autorelease pools are
/// thread-local).
#[derive(Debug, Copy, Clone)]
pub struct AutoreleasePool<'pool> {
    /// A reference to the pool.
    ///
    /// The lifetime is covariant, since shortening the lifetime is not a
    /// problem (the lifetime talks about the pool, and not any data inside
    /// the pool).
    ///
    /// To somewhat prove this, consider the following example using
    /// `typed-arena` to partially implement the autorelease pool:
    ///
    /// ```ignore
    /// struct Pool(typed_arena::Arena<String>);
    ///
    /// pub struct AutoreleasePool<'pool>(&'pool Pool);
    ///
    /// impl<'pool> AutoreleasePool<'pool> {
    ///     pub fn autorelease(self, s: String) -> &'pool str {
    ///         &*self.0.0.alloc(s)
    ///     }
    ///
    ///     pub fn autorelease_mut(self, s: String) -> &'pool mut str {
    ///         &mut *self.0.0.alloc(s)
    ///     }
    /// }
    ///
    /// pub fn autoreleasepool<F, R>(f: F) -> R
    /// where
    ///     F: for<'pool> FnOnce(AutoreleasePool<'pool>) -> R
    /// {
    ///     let pool = Pool(Default::default());
    ///     f(AutoreleasePool(&pool))
    /// }
    /// ```
    ///
    /// Hence assuming `typed-arena` is sound, having covariance here should
    /// also be sound.
    #[cfg(all(debug_assertions, not(feature = "unstable-autoreleasesafe")))]
    inner: &'pool Pool,
    /// We use `PhantomData` here to make `AutoreleasePool` a ZST.
    #[cfg(not(all(debug_assertions, not(feature = "unstable-autoreleasesafe"))))]
    inner: PhantomData<&'pool Pool>,
}

#[cfg(all(debug_assertions, not(feature = "unstable-autoreleasesafe")))]
thread_local! {
    /// We track the thread's pools to verify that object lifetimes are only
    /// taken from the innermost pool.
    static POOLS: RefCell<Vec<*mut c_void>> = RefCell::new(Vec::new());
}

impl<'pool> AutoreleasePool<'pool> {
    /// This will be removed in a future version.
    #[inline]
    #[doc(hidden)]
    pub fn __verify_is_inner(self) {
        #[cfg(all(debug_assertions, not(feature = "unstable-autoreleasesafe")))]
        POOLS.with(|c| {
            assert_eq!(
                c.borrow().last(),
                Some(&self.inner.context),
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
    ///
    /// # Safety
    ///
    /// This is equivalent to `&*ptr`, and shares the unsafety of that, except
    /// the lifetime is bound to the pool instead of being unbounded.
    #[inline]
    pub unsafe fn ptr_as_ref<T: ?Sized>(self, ptr: *const T) -> &'pool T {
        self.__verify_is_inner();
        // SAFETY: Checked by the caller
        unsafe { ptr.as_ref().unwrap_unchecked() }
    }

    /// Returns a unique reference to the given autoreleased pointer object.
    ///
    /// This is the preferred way to make mutable references from autoreleased
    /// objects, since it binds the lifetime of the reference to the pool, and
    /// does some extra checks when debug assertions are enabled.
    ///
    /// For the shared counterpart see [`ptr_as_ref`](#method.ptr_as_ref).
    ///
    ///
    /// # Safety
    ///
    /// This is equivalent to `&mut *ptr`, and shares the unsafety of that,
    /// except the lifetime is bound to the pool instead of being unbounded.
    #[inline]
    pub unsafe fn ptr_as_mut<T: ?Sized>(self, ptr: *mut T) -> &'pool mut T {
        self.__verify_is_inner();
        // SAFETY: Checked by the caller
        unsafe { ptr.as_mut().unwrap_unchecked() }
    }
}

/// We use a macro here so that the documentation is included whether the
/// feature is enabled or not.
#[cfg(not(feature = "unstable-autoreleasesafe"))]
macro_rules! auto_trait {
    {$(#[$fn_meta:meta])* $v:vis unsafe trait AutoreleaseSafe {}} => {
        $(#[$fn_meta])*
        $v unsafe trait AutoreleaseSafe {}
    }
}

#[cfg(feature = "unstable-autoreleasesafe")]
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
    /// With the `"unstable-autoreleasesafe"` feature enabled, this is an auto
    /// trait that is implemented for all types except [`AutoreleasePool`].
    ///
    /// Otherwise it is just a dummy trait that is implemented for all types;
    /// the safety invariants are checked with debug assertions instead.
    ///
    /// You should not normally need to implement this trait yourself.
    ///
    ///
    /// # Safety
    ///
    /// Must not be implemented for types that interract with the autorelease
    /// pool. So if you reimplement the [`AutoreleasePool`] struct or
    /// likewise, this should be negatively implemented for that.
    ///
    /// This can easily be accomplished with an
    /// `PhantomData<AutoreleasePool<'_>>` if the `"unstable-autoreleasesafe"`
    /// feature is enabled.
    pub unsafe trait AutoreleaseSafe {}
}

#[cfg(not(feature = "unstable-autoreleasesafe"))]
unsafe impl<T: ?Sized> AutoreleaseSafe for T {}

#[cfg(feature = "unstable-autoreleasesafe")]
impl !AutoreleaseSafe for Pool {}
#[cfg(feature = "unstable-autoreleasesafe")]
impl !AutoreleaseSafe for AutoreleasePool<'_> {}

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
/// `unstable-autoreleasesafe` crate feature on nightly Rust.
///
/// Note that this is mostly useful for preventing leaks (as any Objective-C
/// method may leak internally). If implementing an interface to an object,
/// you should try to return retained pointers with [`msg_send_id!`] wherever
/// you can instead, since having to use this function can be quite cumbersome
/// for your users!
///
/// [`msg_send_id!`]: crate::msg_send_id
///
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use core::mem::ManuallyDrop;
/// use objc2::{class, msg_send, msg_send_id};
/// use objc2::rc::{autoreleasepool, AutoreleasePool, Id, Owned};
/// use objc2::runtime::Object;
///
/// fn needs_lifetime_from_pool<'p>(pool: AutoreleasePool<'p>) -> &'p mut Object {
///     let obj: Id<Object, Owned> = unsafe { msg_send_id![class!(NSObject), new] };
///     let obj = ManuallyDrop::new(obj);
///     let obj: *mut Object = unsafe { msg_send![obj, autorelease] };
///     // Lifetime of the returned reference is bounded by the pool
///     unsafe { pool.ptr_as_mut(obj) }
///
///     // Or simply
///     // let obj: Id<Object, Owned> = unsafe { msg_send_id![class!(NSObject), new] };
///     // obj.autorelease(pool)
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
/// # use objc2::{class, msg_send_id};
/// # use objc2::rc::{autoreleasepool, AutoreleasePool, Id, Owned};
/// # use objc2::runtime::Object;
/// #
/// # fn needs_lifetime_from_pool<'p>(pool: AutoreleasePool<'p>) -> &'p mut Object {
/// #     let obj: Id<Object, Owned> = unsafe { msg_send_id![class!(NSObject), new] };
/// #     obj.autorelease(pool)
/// # }
/// #
/// let obj = autoreleasepool(|pool| {
///     let obj = needs_lifetime_from_pool(pool);
///     // Use `obj`
///     obj
/// });
/// ```
///
/// Incorrect usage which panics (with debug assertions enabled) because we
/// tried to pass an outer pool to an inner pool:
///
#[cfg_attr(feature = "unstable-autoreleasesafe", doc = "```compile_fail")]
#[cfg_attr(not(feature = "unstable-autoreleasesafe"), doc = "```should_panic")]
/// # use objc2::{class, msg_send_id};
/// # use objc2::rc::{autoreleasepool, AutoreleasePool, Id, Owned};
/// # use objc2::runtime::Object;
/// #
/// # fn needs_lifetime_from_pool<'p>(pool: AutoreleasePool<'p>) -> &'p mut Object {
/// #     let obj: Id<Object, Owned> = unsafe { msg_send_id![class!(NSObject), new] };
/// #     obj.autorelease(pool)
/// # }
/// #
/// autoreleasepool(|outer_pool| {
///     let obj = autoreleasepool(|inner_pool| {
///         let obj = needs_lifetime_from_pool(outer_pool);
///         obj
///     });
///     // `obj` could wrongly be used here because its lifetime was
///     // assigned to the outer pool, even though it was released by the
///     // inner pool already.
/// });
/// #
/// # panic!("Does not panic in release mode, so for testing we make it!");
/// ```
#[doc(alias = "@autoreleasepool")]
#[doc(alias = "objc_autoreleasePoolPush")]
#[doc(alias = "objc_autoreleasePoolPop")]
#[inline]
pub fn autoreleasepool<T, F>(f: F) -> T
where
    for<'pool> F: AutoreleaseSafe + FnOnce(AutoreleasePool<'pool>) -> T,
{
    let inner = unsafe { Pool::new() };
    let pool = AutoreleasePool {
        #[cfg(all(debug_assertions, not(feature = "unstable-autoreleasesafe")))]
        inner: &inner,
        #[cfg(not(all(debug_assertions, not(feature = "unstable-autoreleasesafe"))))]
        inner: PhantomData,
    };
    let result = f(pool);
    drop(inner);
    result
}

#[cfg(all(test, feature = "unstable-autoreleasesafe"))]
mod tests {
    use core::marker::Unpin;
    use core::mem;
    use core::panic::{RefUnwindSafe, UnwindSafe};

    use super::{AutoreleasePool, AutoreleaseSafe};
    use crate::runtime::Object;

    fn requires_autoreleasesafe<T: AutoreleaseSafe>() {}

    #[test]
    fn test_autoreleasesafe() {
        requires_autoreleasesafe::<usize>();
        requires_autoreleasesafe::<*mut Object>();
        requires_autoreleasesafe::<&mut Object>();
    }

    #[test]
    fn unwindsafe() {
        fn assert_unwindsafe<T: UnwindSafe + RefUnwindSafe>() {}

        assert_unwindsafe::<AutoreleasePool<'static>>();
    }

    #[test]
    fn unpin() {
        fn assert_unpin<T: Unpin>() {}

        assert_unwindsafe::<AutoreleasePool<'static>>();
    }

    #[allow(unused)]
    fn assert_covariant1<'a>(pool: AutoreleasePool<'static>) -> AutoreleasePool<'a> {
        pool
    }

    #[allow(unused)]
    fn assert_covariant2<'long: 'short, 'short>(
        pool: AutoreleasePool<'long>,
    ) -> AutoreleasePool<'short> {
        pool
    }

    #[cfg_attr(
        not(feature = "unstable-autoreleasesafe"),
        ignore = "only stably ZST when `unstable-autoreleasesafe` is enabled"
    )]
    #[test]
    fn assert_zst() {
        assert_eq!(mem::size_of::<AutoreleasePool<'static>>(), 0);
    }
}
