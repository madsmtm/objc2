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
use core::marker::PhantomData;

use crate::runtime::{AnyObject, ProtocolObject};
use crate::{ClassType, MainThreadMarker, Message};

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

impl<S: ?Sized> private_mutability::Sealed for InteriorMutableWithSubclass<S> {}
impl<S: ?Sized> Mutability for InteriorMutableWithSubclass<S> {}

impl<S: ?Sized> private_mutability::Sealed for InteriorMutableWithSuperclass<S> {}
impl<S: ?Sized> Mutability for InteriorMutableWithSuperclass<S> {}

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

/// Marker type for classes that have a mutable counterpart.
///
/// This is effectively the same as [`InteriorMutable`], except that the type
/// returned by `NSMutableCopying::mutableCopy` is the mutable counterpart.
///
/// Functionality that is provided with this:
/// - [`IsAllocableAnyThread`] -> [`ClassType::alloc`].
///
///
/// # Example
///
/// ```ignore
/// unsafe impl ClassType for NSString {
///     type Super = NSObject;
///     type Mutability = InteriorMutableWithSubclass<NSMutableString>;
///     // ...
/// }
///
/// unsafe impl ClassType for NSMutableString {
///     type Super = NSString;
///     type Mutability = InteriorMutableWithSuperclass<NSString>;
///     // ...
/// }
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct InteriorMutableWithSubclass<Subclass: ?Sized> {
    inner: Never,
    subclass: PhantomData<Subclass>,
}

/// Marker type for classes that have an immutable counterpart.
///
/// This is effectively the same as [`InteriorMutable`], except that the type
/// returned by `NSCopying::copy` is the immutable counterpart.
///
/// Functionality that is provided with this:
/// - [`IsAllocableAnyThread`] -> [`ClassType::alloc`].
///
///
/// # Example
///
/// ```ignore
/// unsafe impl ClassType for NSData {
///     type Super = NSObject;
///     type Mutability = InteriorMutableWithSubclass<NSMutableData>;
///     // ...
/// }
///
/// unsafe impl ClassType for NSMutableData {
///     type Super = NSData;
///     type Mutability = InteriorMutableWithSuperclass<NSData>;
///     // ...
/// }
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct InteriorMutableWithSuperclass<Superclass: ?Sized> {
    inner: Never,
    superclass: PhantomData<Superclass>,
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
impl<S: ?Sized> MutabilityIsAllocableAnyThread for InteriorMutableWithSubclass<S> {}
impl<S: ?Sized> MutabilityIsAllocableAnyThread for InteriorMutableWithSuperclass<S> {}

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

/// Retrieve the immutable/mutable counterpart class, and fall back to `Self`
/// if not applicable.
///
/// This is used for describing the return type of `NSCopying` and
/// `NSMutableCopying`, since due to Rust trait limitations, those two can't
/// have associated types themselves (since we want to use them in
/// `ProtocolObject<dyn NSCopying>`).
///
///
/// # Usage notes
///
/// You may not rely on this being implemented entirely correctly for protocol
/// objects, since we have less type-information available there.
///
/// In particular, the immutable counterpart of a mutable object converted to
/// `ProtocolObject<dyn AProtocol>` may not itself implement the protocol, and
/// invalidly assuming it does is unsound.
///
/// All of this is to say: Do not use this trait in isolation, either require
/// `NSCopying` or `ClassType` along with it.
///
///
/// # Safety
///
/// This is a sealed trait, and should not need to be implemented. Open an
/// issue if you know a use-case where this restrition should be lifted!
pub unsafe trait CounterpartOrSelf: private_traits::Sealed {
    /// The immutable counterpart of the type, or `Self` if the type has no
    /// immutable counterpart.
    ///
    /// The implementation for `NSString` has itself (`NSString`) here, while
    /// `NSMutableString` instead has `NSString`.
    type Immutable: ?Sized + Message;

    /// The mutable counterpart of the type, or `Self` if the type has no
    /// mutable counterpart.
    ///
    /// The implementation for `NSString` has `NSMutableString` here, while
    /// `NSMutableString` has itself (`NSMutableString`).
    type Mutable: ?Sized + Message;
}

mod private_counterpart {
    use super::*;

    pub trait MutabilityCounterpartOrSelf<T: ?Sized>: Mutability {
        type Immutable: ?Sized + Message;
        type Mutable: ?Sized + Message;
    }
    impl<T: ClassType<Mutability = Root>> MutabilityCounterpartOrSelf<T> for Root {
        type Immutable = T;
        type Mutable = T;
    }
    impl<T: ClassType<Mutability = InteriorMutable>> MutabilityCounterpartOrSelf<T>
        for InteriorMutable
    {
        type Immutable = T;
        type Mutable = T;
    }
    impl<T, S> MutabilityCounterpartOrSelf<T> for InteriorMutableWithSubclass<S>
    where
        T: ClassType<Mutability = InteriorMutableWithSubclass<S>>,
        S: ClassType<Mutability = InteriorMutableWithSuperclass<T>>,
    {
        type Immutable = T;
        type Mutable = S;
    }
    impl<T, S> MutabilityCounterpartOrSelf<T> for InteriorMutableWithSuperclass<S>
    where
        T: ClassType<Mutability = InteriorMutableWithSuperclass<S>>,
        S: ClassType<Mutability = InteriorMutableWithSubclass<T>>,
    {
        type Immutable = S;
        type Mutable = T;
    }
    impl<T: ClassType<Mutability = MainThreadOnly>> MutabilityCounterpartOrSelf<T> for MainThreadOnly {
        type Immutable = T;
        type Mutable = T;
    }
}

unsafe impl<T: ?Sized + ClassType> CounterpartOrSelf for T
where
    T::Mutability: private_counterpart::MutabilityCounterpartOrSelf<T>,
{
    type Immutable =
        <T::Mutability as private_counterpart::MutabilityCounterpartOrSelf<T>>::Immutable;
    type Mutable = <T::Mutability as private_counterpart::MutabilityCounterpartOrSelf<T>>::Mutable;
}

unsafe impl<P: ?Sized> CounterpartOrSelf for ProtocolObject<P> {
    // SAFETY: The only place where this would differ from `Self` is for
    // classes with `InteriorMutableWithSuperclass<IS>`.
    //
    // Superclasses are not in general required to implement the same traits
    // as their subclasses, but we're not dealing with normal classes, we're
    // dealing with with immutable/mutable class counterparts!
    //
    // We could probably get away with requiring that mutable classes
    // only implement the same protocols as their immutable counterparts, but
    // for now we relax the requirements of `CounterpartOrSelf`.
    type Immutable = Self;
    // SAFETY: The only place where this would differ from `Self` is for
    // classes with `InteriorMutableWithSubclass<MS>`.
    //
    // But subclasses are required to always implement the same traits as
    // their superclasses, so a mutable subclass is required to implement the
    // same traits too.
    type Mutable = Self;
}

#[cfg(test)]
mod tests {
    use crate::runtime::NSObject;

    use super::*;

    use core::any::TypeId;
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
        assert_traits::<InteriorMutableWithSubclass<()>>();
        assert_traits::<InteriorMutableWithSuperclass<()>>();
        assert_traits::<MainThreadOnly>();

        #[allow(unused)]
        #[allow(clippy::needless_maybe_sized)]
        fn test_mutability_implies_sized<M: ?Sized + Mutability>() {
            fn assert_sized<T: Sized>() {}
            assert_sized::<M>();
        }
    }

    #[test]
    fn counterpart_root() {
        assert_eq!(
            TypeId::of::<NSObject>(),
            TypeId::of::<<NSObject as CounterpartOrSelf>::Immutable>(),
        );
        assert_eq!(
            TypeId::of::<NSObject>(),
            TypeId::of::<<NSObject as CounterpartOrSelf>::Mutable>(),
        );
    }

    #[allow(unused, clippy::too_many_arguments)]
    fn object_safe(
        _: &dyn IsAllocableAnyThread,
        _: &dyn IsMainThreadOnly,
        _: &dyn CounterpartOrSelf<Immutable = (), Mutable = ()>,
    ) {
    }
}
