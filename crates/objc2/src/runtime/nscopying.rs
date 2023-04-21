use crate::mutability::{
    Immutable, ImmutableWithMutableSubclass, InteriorMutable, MainThreadOnly, Mutable,
    MutableWithImmutableSuperclass, Root,
};
use crate::rc::Id;
use crate::{msg_send_id, ClassType, ProtocolType};

/// Helper trait for [`NSCopying`] and [`NSMutableCopying`].
///
/// This is needed since those two can't have associated types themselves (at
/// least not if they want to be usable as `ProtocolObject<dyn NSCopying>`),
/// and because the return type of those differ if the class has a mutable or
/// an immutable counterpart (as is the case for `NSString` and
/// `NSMutableString`).
pub trait Copyhelper<T: ?Sized> {
    /// The output type of [`NSCopying`] for the given `T`.
    type CopyOutput: ?Sized + ClassType;
    /// The output type of [`NSMutableCopying`] for the given `T`.
    type MutableCopyOutput: ?Sized + ClassType;

    // TODO: Use this to autogenerate `ToOwned` impls
    #[doc(hidden)]
    fn __do_copy(t: &T) -> Id<T>;
}
impl<T: NSCopying + ClassType<Mutability = Root>> Copyhelper<T> for Root {
    type CopyOutput = T;
    type MutableCopyOutput = T;

    #[inline]
    fn __do_copy(t: &T) -> Id<T> {
        t.copy()
    }
}
impl<T: NSCopying + ClassType<Mutability = Immutable>> Copyhelper<T> for Immutable {
    type CopyOutput = T;
    type MutableCopyOutput = T;

    #[inline]
    fn __do_copy(t: &T) -> Id<T> {
        t.retain()
    }
}
impl<T: NSCopying + ClassType<Mutability = Mutable>> Copyhelper<T> for Mutable {
    type CopyOutput = T;
    type MutableCopyOutput = T;

    #[inline]
    fn __do_copy(t: &T) -> Id<T> {
        t.copy()
    }
}
impl<T, S> Copyhelper<T> for ImmutableWithMutableSubclass<S>
where
    T: NSCopying + ClassType<Mutability = ImmutableWithMutableSubclass<S>>,
    S: ClassType<Mutability = MutableWithImmutableSuperclass<T>>,
{
    type CopyOutput = T;
    type MutableCopyOutput = S;

    #[inline]
    fn __do_copy(t: &T) -> Id<T> {
        t.copy()
    }
}
impl<T, S> Copyhelper<T> for MutableWithImmutableSuperclass<S>
where
    T: NSMutableCopying + ClassType<Mutability = MutableWithImmutableSuperclass<S>>,
    S: ClassType<Mutability = ImmutableWithMutableSubclass<T>>,
{
    type CopyOutput = S;
    type MutableCopyOutput = T;

    #[inline]
    fn __do_copy(t: &T) -> Id<T> {
        t.mutableCopy()
    }
}
impl<T: NSCopying + ClassType<Mutability = InteriorMutable>> Copyhelper<T> for InteriorMutable {
    type CopyOutput = T;
    type MutableCopyOutput = T;

    #[inline]
    fn __do_copy(t: &T) -> Id<T> {
        t.retain()
    }
}
impl<T: NSCopying + ClassType<Mutability = MainThreadOnly>> Copyhelper<T> for MainThreadOnly {
    type CopyOutput = T;
    type MutableCopyOutput = T;

    #[inline]
    fn __do_copy(t: &T) -> Id<T> {
        t.retain()
    }
}

/// A protocol to provide functional copies of objects.
///
/// This is similar to Rust's [`Clone`] trait, along with sharing a few
/// similarities to the [`std::borrow::ToOwned`] trait with regards to the
/// output type.
///
/// See [Apple's documentation][apple-doc] for details.
///
/// [apple-doc]: https://developer.apple.com/documentation/foundation/nscopying
#[allow(clippy::missing_safety_doc)] // Same as all other traits
pub unsafe trait NSCopying {
    /// Returns a new instance that's a copy of the receiver.
    ///
    /// The output type is usually `Self`, but e.g. `NSMutableString` returns
    /// `NSString`.
    fn copy(&self) -> Id<<Self::Mutability as Copyhelper<Self>>::CopyOutput>
    where
        Self: Sized + ClassType,
        Self::Mutability: Copyhelper<Self>,
    {
        unsafe { msg_send_id![self, copy] }
    }
}

crate::__inner_extern_protocol!(
    ()
    (NSCopying)
    (dyn NSCopying)
    ("NSCopying")
);

/// A protocol to provide mutable copies of objects.
///
/// Only classes that have an “immutable vs. mutable” distinction should adopt
/// this protocol.
///
/// See [Apple's documentation][apple-doc] for details.
///
/// [apple-doc]: https://developer.apple.com/documentation/foundation/nsmutablecopying
#[allow(clippy::missing_safety_doc)] // Same as all other traits
pub unsafe trait NSMutableCopying {
    /// Returns a new instance that's a mutable copy of the receiver.
    ///
    /// The output type is the mutable counterpart of the object. E.g. both
    /// `NSString` and `NSMutableString` return `NSMutableString`.
    #[allow(non_snake_case)]
    fn mutableCopy(&self) -> Id<<Self::Mutability as Copyhelper<Self>>::MutableCopyOutput>
    where
        Self: Sized + ClassType,
        Self::Mutability: Copyhelper<Self>,
    {
        unsafe { msg_send_id![self, mutableCopy] }
    }
}

crate::__inner_extern_protocol!(
    ()
    (NSMutableCopying)
    (dyn NSMutableCopying)
    ("NSMutableCopying")
);
