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

use crate::ClassType;

/// Helper to make the structs uninhabited, without that being a public fact.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Never {}

/// Marker type for root classes.
///
/// This is used for `icrate::Foundation::NSObject` and
/// `icrate::Foundation::NSProxy`, which are the two fundamental types that
/// all others inherit from.
///
/// Functionality that is provided with this:
/// - [`IsIdCloneable`] -> [`Id::clone`][crate::rc::Id#impl-Clone-for-Id<T>].
/// - [`IsAllocableAnyThread`] -> [`ClassType::alloc`].
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Root {
    inner: Never,
}

/// Marker type for immutable classes.
///
/// Note that immutable objects are often both [`Send`] and [`Sync`], though
/// such implementations must be provided manually.
///
/// Functionality that is provided with this:
/// - [`IsRetainable`] -> [`ClassType::retain`].
/// - [`IsIdCloneable`] -> [`Id::clone`][crate::rc::Id#impl-Clone-for-Id<T>].
/// - [`IsAllocableAnyThread`] -> [`ClassType::alloc`].
/// - You are allowed to hand out pointers / references to an instance's
///   internal data, since you know such data will never be mutated.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Immutable {
    inner: Never,
}

/// Marker type for mutable classes.
///
/// Note that mutable objects are often both [`Send`] and [`Sync`], though
/// such implementations must be provided manually (and are usually only safe
/// if all mutation happens behind `&mut self`).
///
/// Functionality that is provided with this:
/// - [`IsAllocableAnyThread`] -> [`ClassType::alloc`].
/// - [`IsMutable`] -> [`impl DerefMut for Id`][crate::rc::Id#impl-DerefMut-for-Id<T>].
/// - You are allowed to hand out pointers / references to an instance's
///   internal data, since you know such data will never be mutated without
///   the borrowchecker catching it.
///
///
/// # Safety notice
///
/// - (Safe) methods that mutate the object (without synchronization) are
///   required to use `&mut self`.
/// - The `retain` selector is not generally safe to use on classes `T` that
///   specify this, since `Id<T>` allows having `&mut T` references, which
///   Rust assume are unique.
/// - As a special case of that, `-[NSCopying copy]` and
///   `-[NSMutableCopying mutableCopy]`, if implemented, must return a new
///   instance (e.g. cannot be implemented by just `retain`-ing the instance).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Mutable {
    inner: Never,
}

/// Marker type for immutable classes that have a mutable counterpart.
///
/// This is effectively the same as [`Immutable`], except for the fact that
/// classes that specify this does not implement [`IsRetainable`], meaning
/// that [`ClassType::retain`] does not work (see that for details on why).
///
/// The mutable counterpart must be specified as the type parameter `MS` to
/// allow `NSCopying` and `NSMutableCopying` to return the correct type.
///
/// Functionality that is provided with this:
/// - [`IsIdCloneable`].
/// - [`IsAllocableAnyThread`].
/// - You are allowed to hand out pointers / references to an instance's
///   internal data, since you know such data will never be mutated.
///
///
/// # Example
///
/// ```ignore
/// unsafe impl ClassType for NSString {
///     type Super = NSObject;
///     type Mutability = ImmutableWithMutableSubclass<NSMutableString>;
///     // ...
/// }
///
/// unsafe impl ClassType for NSMutableString {
///     type Super = NSString;
///     type Mutability = MutableWithImmutableSubclass<NSString>;
///     // ...
/// }
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ImmutableWithMutableSubclass<MS: ?Sized> {
    inner: Never,
    mutable_subclass: PhantomData<MS>,
}

/// Marker type for mutable classes that have a immutable counterpart.
///
/// This is effectively the same as [`Mutable`], except that the immutable
/// counterpart being be specified as the type parameter `IS` to allow
/// `NSCopying` and `NSMutableCopying` to return the correct type.
///
///
/// # Example
///
/// ```ignore
/// unsafe impl ClassType for NSData {
///     type Super = NSObject;
///     type Mutability = ImmutableWithMutableSubclass<NSMutableData>;
///     // ...
/// }
///
/// unsafe impl ClassType for NSMutableData {
///     type Super = NSData;
///     type Mutability = MutableWithImmutableSubclass<NSData>;
///     // ...
/// }
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct MutableWithImmutableSuperclass<IS: ?Sized> {
    inner: Never,
    immutable_superclass: PhantomData<IS>,
}

/// Marker type for classes that use interior mutability.
///
/// This is usually not `Send + Sync`, unless the class is guaranteed to use
/// thread-safe operations.
///
/// Functionality that is provided with this:
/// - [`IsRetainable`] -> [`ClassType::retain`].
/// - [`IsIdCloneable`] -> [`Id::clone`][crate::rc::Id#impl-Clone-for-Id<T>].
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

mod private {
    use super::*;

    pub trait Sealed {}
    impl Sealed for Root {}
    impl Sealed for Immutable {}
    impl Sealed for Mutable {}
    impl<MS: ?Sized> Sealed for ImmutableWithMutableSubclass<MS> {}
    impl<IS: ?Sized> Sealed for MutableWithImmutableSuperclass<IS> {}
    impl Sealed for InteriorMutable {}
    impl Sealed for MainThreadOnly {}

    pub trait MutabilityIsIdCloneable: Mutability {}
    impl MutabilityIsIdCloneable for Root {}
    impl MutabilityIsIdCloneable for Immutable {}
    impl<MS: ?Sized> MutabilityIsIdCloneable for ImmutableWithMutableSubclass<MS> {}
    impl MutabilityIsIdCloneable for InteriorMutable {}
    impl MutabilityIsIdCloneable for MainThreadOnly {}

    pub trait MutabilityIsRetainable: MutabilityIsIdCloneable {}
    impl MutabilityIsRetainable for Immutable {}
    impl MutabilityIsRetainable for InteriorMutable {}
    impl MutabilityIsRetainable for MainThreadOnly {}

    pub trait MutabilityIsAllocableAnyThread: Mutability {}
    impl MutabilityIsAllocableAnyThread for Root {}
    impl MutabilityIsAllocableAnyThread for Immutable {}
    impl MutabilityIsAllocableAnyThread for Mutable {}
    impl<MS: ?Sized> MutabilityIsAllocableAnyThread for ImmutableWithMutableSubclass<MS> {}
    impl<IS: ?Sized> MutabilityIsAllocableAnyThread for MutableWithImmutableSuperclass<IS> {}
    impl MutabilityIsAllocableAnyThread for InteriorMutable {}

    pub trait MutabilityIsMutable: Mutability {}
    impl MutabilityIsMutable for Mutable {}
    impl<IS: ?Sized> MutabilityIsMutable for MutableWithImmutableSuperclass<IS> {}

    pub trait MutabilityIsMainThreadOnly: Mutability {}
    impl MutabilityIsMainThreadOnly for MainThreadOnly {}

    pub trait MutabilityCounterpartOrSelf<T: ?Sized>: Mutability {
        type Immutable: ?Sized + ClassType;
        type Mutable: ?Sized + ClassType;
    }
    impl<T: ClassType<Mutability = Root>> MutabilityCounterpartOrSelf<T> for Root {
        type Immutable = T;
        type Mutable = T;
    }
    impl<T: ClassType<Mutability = Immutable>> MutabilityCounterpartOrSelf<T> for Immutable {
        type Immutable = T;
        type Mutable = T;
    }
    impl<T: ClassType<Mutability = Mutable>> MutabilityCounterpartOrSelf<T> for Mutable {
        type Immutable = T;
        type Mutable = T;
    }
    impl<T, S> MutabilityCounterpartOrSelf<T> for ImmutableWithMutableSubclass<S>
    where
        T: ClassType<Mutability = ImmutableWithMutableSubclass<S>>,
        S: ClassType<Mutability = MutableWithImmutableSuperclass<T>>,
    {
        type Immutable = T;
        type Mutable = S;
    }
    impl<T, S> MutabilityCounterpartOrSelf<T> for MutableWithImmutableSuperclass<S>
    where
        T: ClassType<Mutability = MutableWithImmutableSuperclass<S>>,
        S: ClassType<Mutability = ImmutableWithMutableSubclass<T>>,
    {
        type Immutable = S;
        type Mutable = T;
    }
    impl<T: ClassType<Mutability = InteriorMutable>> MutabilityCounterpartOrSelf<T>
        for InteriorMutable
    {
        type Immutable = T;
        type Mutable = T;
    }
    impl<T: ClassType<Mutability = MainThreadOnly>> MutabilityCounterpartOrSelf<T> for MainThreadOnly {
        type Immutable = T;
        type Mutable = T;
    }

    // TODO: Trait for objects whose `hash` is guaranteed to never change,
    // which allows it to be used as a key in `NSDictionary`.
}

/// Marker trait for the different types of mutability a class can have.
///
/// This is a sealed trait, and should not need to be implemented. Open an
/// issue if you know a use-case where this restrition should be lifted!
//
// Note: `Sized` is intentionally added to make the trait not object safe.
pub trait Mutability: private::Sealed + Sized {}
impl Mutability for Root {}
impl Mutability for Immutable {}
impl Mutability for Mutable {}
impl<MS: ?Sized> Mutability for ImmutableWithMutableSubclass<MS> {}
impl<IS: ?Sized> Mutability for MutableWithImmutableSuperclass<IS> {}
impl Mutability for InteriorMutable {}
impl Mutability for MainThreadOnly {}

/// Marker trait for classes where [`Id::clone`] is safe.
///
/// Since the `Foundation` collection types (`NSArray<T>`,
/// `NSDictionary<K, V>`, ...) act as if they store [`Id`]s, this also makes
/// certain functionality on those types possible.
///
/// This is implemented for classes whose [`ClassType::Mutability`] is one of:
/// - [`Root`].
/// - [`Immutable`].
/// - [`ImmutableWithMutableSubclass`].
/// - [`InteriorMutable`].
/// - [`MainThreadOnly`].
///
/// [`Id`]: crate::rc::Id
/// [`Id::clone`]: crate::rc::Id#impl-Clone-for-Id<T>
pub trait IsIdCloneable: ClassType {}
impl<T: ?Sized + ClassType> IsIdCloneable for T where T::Mutability: private::MutabilityIsIdCloneable
{}

/// Marker trait for classes where the `retain` selector is always safe.
///
/// [`Id::clone`] takes `&Id<T>`, while [`ClassType::retain`] only takes `&T`;
/// the difference between these two is that in the former case, you know that
/// there are no live mutable subclasses.
///
/// This is implemented for classes whose [`ClassType::Mutability`] is one of:
/// - [`Immutable`].
/// - [`InteriorMutable`].
/// - [`MainThreadOnly`].
///
/// [`Id::clone`]: crate::rc::Id#impl-Clone-for-Id<T>
pub trait IsRetainable: IsIdCloneable {}
impl<T: ?Sized + ClassType> IsRetainable for T where T::Mutability: private::MutabilityIsRetainable {}

/// Marker trait for classes that can be allocated from any thread.
///
/// This is implemented for classes whose [`ClassType::Mutability`] is one of:
/// - [`Root`].
/// - [`Immutable`].
/// - [`Mutable`].
/// - [`ImmutableWithMutableSubclass`].
/// - [`MutableWithImmutableSuperclass`].
/// - [`InteriorMutable`].
pub trait IsAllocableAnyThread: ClassType {}
impl<T: ?Sized + ClassType> IsAllocableAnyThread for T where
    T::Mutability: private::MutabilityIsAllocableAnyThread
{
}

/// Marker trait for classes that are only mutable through `&mut`.
///
/// This is implemented for classes whose [`ClassType::Mutability`] is one of:
/// - [`Mutable`]
/// - [`MutableWithImmutableSuperclass`]
///
/// Notably, [`InteriorMutable`] does not implement this (though it is
/// technically mutable), since it is allowed to mutate through shared
/// references.
pub trait IsMutable: ClassType {}
impl<T: ?Sized + ClassType> IsMutable for T where T::Mutability: private::MutabilityIsMutable {}

/// Marker trait for classes that are only available on the main thread.
///
/// This is implemented for classes whose [`ClassType::Mutability`] is one of:
/// - [`MainThreadOnly`].
///
/// Since `MainThreadOnly` types must be `!Send` and `!Sync`, if you hold a
/// type that implements this trait, then you're guaranteed to be on the main
/// thread.
//
// Note: MainThreadMarker::from relies on this.
pub trait IsMainThreadOnly: ClassType {}
impl<T: ?Sized + ClassType> IsMainThreadOnly for T where
    T::Mutability: private::MutabilityIsMainThreadOnly
{
}

/// Retrieve the immutable/mutable counterpart class, and fall back to `Self`
/// if not applicable.
///
/// This is mostly used for describing the return type of `NSCopying` and
/// `NSMutableCopying`, since due to Rust trait limitations, those two can't
/// have associated types themselves (at least not since we want to use them
/// in `ProtocolObject<dyn NSCopying>`).
pub trait CounterpartOrSelf: ClassType {
    /// The immutable counterpart of the type, or `Self` if the type has no
    /// immutable counterpart.
    ///
    /// The implementation for `NSString` has itself (`NSString`) here, while
    /// `NSMutableString` instead has `NSString`.
    type Immutable: ?Sized + ClassType;

    /// The mutable counterpart of the type, or `Self` if the type has no
    /// mutable counterpart.
    ///
    /// The implementation for `NSString` has `NSMutableString` here, while
    /// `NSMutableString` has itself (`NSMutableString`).
    type Mutable: ?Sized + ClassType;
}
impl<T: ?Sized + ClassType> CounterpartOrSelf for T
where
    T::Mutability: private::MutabilityCounterpartOrSelf<T>,
{
    type Immutable = <T::Mutability as private::MutabilityCounterpartOrSelf<T>>::Immutable;
    type Mutable = <T::Mutability as private::MutabilityCounterpartOrSelf<T>>::Mutable;
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
        assert_traits::<Immutable>();
        assert_traits::<Mutable>();
        assert_traits::<ImmutableWithMutableSubclass<()>>();
        assert_traits::<MutableWithImmutableSuperclass<()>>();
        assert_traits::<InteriorMutable>();
        assert_traits::<MainThreadOnly>();

        #[allow(unused)]
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
}
