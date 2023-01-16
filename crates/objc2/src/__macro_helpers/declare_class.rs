use core::mem::ManuallyDrop;
use core::ptr;

use crate::declare::__IdReturnValue;
use crate::rc::{Allocated, Id, Ownership};
use crate::{Message, MessageReceiver};

use super::{CopyOrMutCopy, Init, MaybeUnwrap, New, Other};

// One could imagine a different design where we simply had a method like
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
impl<Ret, T, O> MessageRecieveId<Allocated<T>, Ret> for Init
where
    T: Message,
    O: Ownership,
    Ret: MaybeOptionId<Input = Id<T, O>>,
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

/// Helper trait for specifying an `Id<T, O>` or an `Option<Id<T, O>>`.
///
/// (Both of those are valid return types from declare_class! `#[method_id]`).
pub trait MaybeOptionId: MaybeUnwrap {
    fn consumed_return(self) -> __IdReturnValue;
    fn autorelease_return(self) -> __IdReturnValue;
}

impl<T: Message, O: Ownership> MaybeOptionId for Id<T, O> {
    #[inline]
    fn consumed_return(self) -> __IdReturnValue {
        let ptr: *mut T = Id::consume_as_ptr(ManuallyDrop::new(self));
        __IdReturnValue(ptr.cast())
    }

    #[inline]
    fn autorelease_return(self) -> __IdReturnValue {
        let ptr: *mut T = Id::autorelease_return(self);
        __IdReturnValue(ptr.cast())
    }
}

impl<T: Message, O: Ownership> MaybeOptionId for Option<Id<T, O>> {
    #[inline]
    fn consumed_return(self) -> __IdReturnValue {
        let ptr: *mut T = self
            .map(|this| Id::consume_as_ptr(ManuallyDrop::new(this)))
            .unwrap_or_else(ptr::null_mut);
        __IdReturnValue(ptr.cast())
    }

    #[inline]
    fn autorelease_return(self) -> __IdReturnValue {
        let ptr: *mut T = Id::autorelease_return_option(self);
        __IdReturnValue(ptr.cast())
    }
}
