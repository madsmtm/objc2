//! # Marker types for class mutability.
//!
//! Every class must indicate which kind of mutability its instances use:
//! - Is the instance mutable or immutable?
//! - Does it use interior mutability (mutable behind `&self`, like
//!   [`UnsafeCell`])?
//! - Does it access global statics in such a way that the type is only safe
//!   to use from the main thread?
//!
//! The answer to these facts influence the final capabilities the type has,
//! as encoded in [the traits in this module](#traits).
//!
//! Concretely, you set [`ClassType::Mutability`] to [one of the types in this
//! module](#structs) to indicate the properties of class you're dealing with
//! (can be done inside [`extern_class!`] and [`declare_class!`]).
//!
//! Note that precious little of Objective-C follows Rust's usual shared xor
//! unique ownership model, most often objects assume interior mutability, so
//! a safe default is often [`InteriorMutable`], or of you're working with GUI
//! code, [`MainThreadOnly`].
//!
//! An explicit design decision in `objc2` is that once you type-erase a
//! mutable object, you are allowed to do normal reference-counting with it
//! (that's also an implicit assumption in Objective-C, e.g. classes are
//! allowed to pass `NSString` in their public API, while actually giving
//! access to a `NSMutableString`).
//!
//! [`UnsafeCell`]: core::cell::UnsafeCell
//! [`ClassType::Mutability`]: crate::ClassType::Mutability
//! [`extern_class!`]: crate::extern_class
//! [`declare_class!`]: crate::declare_class
//!
//!
//! # SemVer
//!
//! It is considered a major change to change the [`ClassType::Mutability`] of
//! an object, though it can be done as a minor change in some cases to fix a
//! bug.
use crate::runtime::{AnyObject, ProtocolObject};
use crate::{ClassType, MainThreadMarker};

mod private_mutability {
    pub trait Sealed {}
}

/// Marker trait for the different types of mutability a class can have.
///
/// This is a sealed trait, and should not need to be implemented. Open an
/// issue if you know a use-case where this restrition should be lifted!
//
// Note: `Sized` is intentionally added to make the trait not object safe.
pub trait Mutability: private_mutability::Sealed + Sized {}

impl private_mutability::Sealed for Root {}
impl Mutability for Root {}

impl private_mutability::Sealed for InteriorMutable {}
impl Mutability for InteriorMutable {}

impl private_mutability::Sealed for MainThreadOnly {}
impl Mutability for MainThreadOnly {}

/// Helper to make the structs uninhabited, without that being a public fact.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Never {}

/// Marker type for root classes.
///
/// This is used for `objc2_foundation::NSObject` and
/// `objc2_foundation::NSProxy`, which are the two fundamental types that
/// all others inherit from.
///
/// Functionality that is provided with this:
/// - [`IsAllocableAnyThread`] -> [`ClassType::alloc`].
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Root {
    inner: Never,
}

/// Marker type for classes that use interior mutability.
///
/// This is usually not `Send + Sync`, unless the class is guaranteed to use
/// thread-safe operations.
///
/// Functionality that is provided with this:
/// - [`IsAllocableAnyThread`] -> [`ClassType::alloc`].
///
///
/// # Safety notice
///
/// When declaring classes, it is recommended that you wrap your instance
/// variables in [`Cell`], [`RefCell`], atomics or other similar interior
/// mutability abstractions to allow mutating your instance variables through
/// `&self`.
///
/// Declared classes that use this cannot take `&mut self`, except in
/// initializers.
///
/// [`Cell`]: core::cell::Cell
/// [`RefCell`]: core::cell::RefCell
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct InteriorMutable {
    inner: Never,
}

/// Marker type for classes that are only safe to use from the main thread.
///
/// This is effectively the same as [`InteriorMutable`], except that classes
/// that specify this are only allowed to be used from the main thread, and
/// hence are not [`IsAllocableAnyThread`].
///
/// This is commonly used in GUI code like `AppKit` and `UIKit`, e.g.
/// `UIWindow` is only usable from the application's main thread.
///
/// It is unsound to implement [`Send`] or [`Sync`] on a type with this
/// mutability.
///
/// Functionality that is provided with this:
/// - [`IsMainThreadOnly`] -> `IsMainThreadOnly::mtm`.
//
// While Xcode's Main Thread Checker doesn't report `alloc` and `dealloc` as
// unsafe from other threads, things like `NSView` and `NSWindow` still do a
// non-trivial amount of stuff on `dealloc`, even if the object is freshly
// `alloc`'d - so let's disallow that to be sure.
//
// This also has the nice property that `Allocated<T>` is guaranteed to be
// allowed to initialize on the current thread.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct MainThreadOnly {
    inner: Never,
}

mod private_traits {
    pub trait Sealed {}
}

impl<T: ?Sized + ClassType> private_traits::Sealed for T {}
impl<P: ?Sized> private_traits::Sealed for ProtocolObject<P> {}
impl private_traits::Sealed for AnyObject {}

/// Marker trait for classes that can be allocated from any thread.
///
/// This is implemented for classes whose [`ClassType::Mutability`] is one of:
/// - [`Root`].
/// - [`InteriorMutable`].
///
///
/// # Safety
///
/// This is a sealed trait, and should not need to be implemented. Open an
/// issue if you know a use-case where this restrition should be lifted!
pub unsafe trait IsAllocableAnyThread: private_traits::Sealed {}

trait MutabilityIsAllocableAnyThread: Mutability {}
impl MutabilityIsAllocableAnyThread for Root {}
impl MutabilityIsAllocableAnyThread for InteriorMutable {}

unsafe impl<T: ?Sized + ClassType> IsAllocableAnyThread for T where
    T::Mutability: MutabilityIsAllocableAnyThread
{
}
unsafe impl<P: ?Sized + IsAllocableAnyThread> IsAllocableAnyThread for ProtocolObject<P> {}

/// Marker trait for classes that are only available on the main thread.
///
/// This is implemented for classes whose [`ClassType::Mutability`] is one of:
/// - [`MainThreadOnly`].
///
/// Since `MainThreadOnly` types must be `!Send` and `!Sync`, if you hold a
/// type that implements this trait, then you're guaranteed to be on the main
/// thread (and can get a `MainThreadMarker` using `IsMainThreadOnly::mtm`).
///
///
/// # Safety
///
/// This is a sealed trait, and should not need to be implemented. Open an
/// issue if you know a use-case where this restrition should be lifted!
pub unsafe trait IsMainThreadOnly: private_traits::Sealed {
    /// Get a [`MainThreadMarker`] from a main-thread-only object.
    ///
    /// This function exists purely in the type-system, and will succeed at
    /// runtime (with a safety check when debug assertions are enabled).
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    fn mtm(&self) -> MainThreadMarker {
        #[cfg(debug_assertions)]
        assert!(
            MainThreadMarker::new().is_some(),
            "the main-thread-only object that we tried to fetch a MainThreadMarker from was somehow not on the main thread",
        );

        // SAFETY: Objects which are `IsMainThreadOnly` are guaranteed
        // `!Send + !Sync` and are only constructible on the main thread.
        //
        // Since we hold a reference to such an object, and we know it cannot
        // now possibly be on another thread than the main, we know that the
        // current thread is the main thread.
        unsafe { MainThreadMarker::new_unchecked() }
    }
}

trait MutabilityIsMainThreadOnly: Mutability {}
impl MutabilityIsMainThreadOnly for MainThreadOnly {}

unsafe impl<T: ?Sized + ClassType> IsMainThreadOnly for T where
    T::Mutability: MutabilityIsMainThreadOnly
{
}
unsafe impl<P: ?Sized + IsMainThreadOnly> IsMainThreadOnly for ProtocolObject<P> {}

#[cfg(test)]
mod tests {
    use super::*;

    use core::fmt;
    use core::hash;

    #[test]
    fn generic_traits() {
        fn assert_traits<T>()
        where
            T: Sync + Send,
            T: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + hash::Hash + fmt::Debug,
        {
        }

        assert_traits::<Root>();
        assert_traits::<InteriorMutable>();
        assert_traits::<MainThreadOnly>();

        #[allow(unused)]
        #[allow(clippy::needless_maybe_sized)]
        fn test_mutability_implies_sized<M: ?Sized + Mutability>() {
            fn assert_sized<T: Sized>() {}
            assert_sized::<M>();
        }
    }

    #[allow(unused, clippy::too_many_arguments)]
    fn object_safe(_: &dyn IsAllocableAnyThread, _: &dyn IsMainThreadOnly) {}
}
