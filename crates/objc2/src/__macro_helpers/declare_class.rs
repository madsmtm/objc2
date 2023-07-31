use crate::declare::__IdReturnValue;
use crate::rc::{Allocated, Id};
use crate::{ClassType, Message, MessageReceiver};

use super::{CopyOrMutCopy, Init, MaybeUnwrap, New, Other};
use crate::mutability;

// One could imagine a different design where we had a method like
// `fn convert_receiver()`, but that won't work in `declare_class!` since we
// can't actually modify the `self` argument (e.g. `let self = foo(self)` is
// not allowed).
//
// See `MsgSendId` and `RetainSemantics` for details on the retain semantics
// we're following here.
pub trait MessageRecieveId<Receiver, Ret> {
    fn into_return(obj: Ret) -> __IdReturnValue;
}

// Receiver and return type have no correlation
impl<Receiver, Ret> MessageRecieveId<Receiver, Ret> for New
where
    Receiver: MessageReceiver,
    Ret: MaybeOptionId,
{
    #[inline]
    fn into_return(obj: Ret) -> __IdReturnValue {
        obj.consumed_return()
    }
}

// Explicitly left unimplemented for now!
// impl MessageRecieveId<impl MessageReceiver, Option<Allocated<T>>> for Alloc {}

// Note: `MethodImplementation` allows for `Allocated` as the receiver, so we
// restrict it here to only be when the selector is `init`.
//
// Additionally, the receiver and return type must have the same generic
// generic parameter `T`.
impl<Ret, T> MessageRecieveId<Allocated<T>, Ret> for Init
where
    T: Message,
    Ret: MaybeOptionId<Input = Id<T>>,
{
    #[inline]
    fn into_return(obj: Ret) -> __IdReturnValue {
        obj.consumed_return()
    }
}

// Receiver and return type have no correlation
impl<Receiver, Ret> MessageRecieveId<Receiver, Ret> for CopyOrMutCopy
where
    Receiver: MessageReceiver,
    Ret: MaybeOptionId,
{
    #[inline]
    fn into_return(obj: Ret) -> __IdReturnValue {
        obj.consumed_return()
    }
}

// Receiver and return type have no correlation
impl<Receiver, Ret> MessageRecieveId<Receiver, Ret> for Other
where
    Receiver: MessageReceiver,
    Ret: MaybeOptionId,
{
    #[inline]
    fn into_return(obj: Ret) -> __IdReturnValue {
        obj.autorelease_return()
    }
}

/// Helper trait for specifying an `Id<T>` or an `Option<Id<T>>`.
///
/// (Both of those are valid return types from declare_class! `#[method_id]`).
pub trait MaybeOptionId: MaybeUnwrap {
    fn consumed_return(self) -> __IdReturnValue;
    fn autorelease_return(self) -> __IdReturnValue;
}

impl<T: Message> MaybeOptionId for Id<T> {
    #[inline]
    fn consumed_return(self) -> __IdReturnValue {
        let ptr: *mut T = Id::consume_as_ptr(self);
        __IdReturnValue(ptr.cast())
    }

    #[inline]
    fn autorelease_return(self) -> __IdReturnValue {
        let ptr: *mut T = Id::autorelease_return(self);
        __IdReturnValue(ptr.cast())
    }
}

impl<T: Message> MaybeOptionId for Option<Id<T>> {
    #[inline]
    fn consumed_return(self) -> __IdReturnValue {
        let ptr: *mut T = Id::consume_as_ptr_option(self);
        __IdReturnValue(ptr.cast())
    }

    #[inline]
    fn autorelease_return(self) -> __IdReturnValue {
        let ptr: *mut T = Id::autorelease_return_option(self);
        __IdReturnValue(ptr.cast())
    }
}

/// Helper for ensuring that `ClassType::Mutability` is implemented correctly
/// for subclasses.
pub trait ValidSubclassMutability<T: mutability::Mutability> {}

// Root
impl ValidSubclassMutability<mutability::Immutable> for mutability::Root {}
impl ValidSubclassMutability<mutability::Mutable> for mutability::Root {}
impl<MS, IS> ValidSubclassMutability<mutability::ImmutableWithMutableSubclass<MS>>
    for mutability::Root
where
    MS: ?Sized + ClassType<Mutability = mutability::MutableWithImmutableSuperclass<IS>>,
    IS: ?Sized + ClassType<Mutability = mutability::ImmutableWithMutableSubclass<MS>>,
{
}
impl ValidSubclassMutability<mutability::InteriorMutable> for mutability::Root {}
impl ValidSubclassMutability<mutability::MainThreadOnly> for mutability::Root {}

// Immutable
impl ValidSubclassMutability<mutability::Immutable> for mutability::Immutable {}

// Mutable
impl ValidSubclassMutability<mutability::Mutable> for mutability::Mutable {}

// ImmutableWithMutableSubclass
impl<MS, IS> ValidSubclassMutability<mutability::MutableWithImmutableSuperclass<IS>>
    for mutability::ImmutableWithMutableSubclass<MS>
where
    MS: ?Sized + ClassType<Mutability = mutability::MutableWithImmutableSuperclass<IS>>,
    IS: ?Sized + ClassType<Mutability = mutability::ImmutableWithMutableSubclass<MS>>,
{
}
// Only valid when `NSCopying`/`NSMutableCopying` is not implemented!
impl<MS: ?Sized + ClassType> ValidSubclassMutability<mutability::Immutable>
    for mutability::ImmutableWithMutableSubclass<MS>
{
}

// MutableWithImmutableSuperclass
// Only valid when `NSCopying`/`NSMutableCopying` is not implemented!
impl<IS: ?Sized + ClassType> ValidSubclassMutability<mutability::Mutable>
    for mutability::MutableWithImmutableSuperclass<IS>
{
}

// InteriorMutable
impl ValidSubclassMutability<mutability::InteriorMutable> for mutability::InteriorMutable {}
impl ValidSubclassMutability<mutability::MainThreadOnly> for mutability::InteriorMutable {}

// MainThreadOnly
impl ValidSubclassMutability<mutability::MainThreadOnly> for mutability::MainThreadOnly {}

/// Ensure that:
/// 1. The type is not a root class (it's superclass implements `ClassType`,
///    and it's mutability is not `Root`), and therefore also implements basic
///    memory management methods, as required by `unsafe impl Message`.
/// 2. The mutability is valid according to the superclass' mutability.
#[inline]
pub fn assert_mutability_matches_superclass_mutability<T>()
where
    T: ?Sized + ClassType,
    T::Super: ClassType,
    T::Mutability: mutability::Mutability,
    <T::Super as ClassType>::Mutability: ValidSubclassMutability<T::Mutability>,
{
    // Noop
}
