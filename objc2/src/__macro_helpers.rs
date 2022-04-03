use core::mem::ManuallyDrop;

use crate::rc::{Id, Ownership};
use crate::runtime::{Class, Sel};
use crate::{Message, MessageArguments, MessageError, MessageReceiver};

#[doc(hidden)]
pub struct Assert<const ALLOC: bool, const INIT: bool, const RETAINED: bool> {}

#[doc(hidden)]
pub trait MsgSendId<T, U> {
    unsafe fn send_message_id<A: MessageArguments>(
        obj: T,
        sel: Sel,
        args: A,
    ) -> Result<U, MessageError>;
}

// `alloc`, should mark the return value as "allocated, not initialized" somehow
impl<T: ?Sized + Message, O: Ownership> MsgSendId<&'_ Class, Id<T, O>>
    for Assert<true, false, true>
{
    #[inline(always)]
    unsafe fn send_message_id<A: MessageArguments>(
        cls: &Class,
        sel: Sel,
        args: A,
    ) -> Result<Id<T, O>, MessageError> {
        unsafe {
            MessageReceiver::send_message(cls, sel, args)
                .map(|r| Id::new(r).expect("Failed allocating"))
        }
    }
}

// `init`, should mark the input value as "allocated, not initialized" somehow
impl<T: ?Sized + Message, O: Ownership> MsgSendId<Id<T, O>, Option<Id<T, O>>>
    for Assert<false, true, true>
{
    #[inline(always)]
    unsafe fn send_message_id<A: MessageArguments>(
        obj: Id<T, O>,
        sel: Sel,
        args: A,
    ) -> Result<Option<Id<T, O>>, MessageError> {
        let obj = ManuallyDrop::new(obj);
        unsafe { MessageReceiver::send_message(obj, sel, args).map(|r| Id::new(r)) }
    }
}

// `copy`, `mutableCopy` and `new`
impl<T: MessageReceiver, U: ?Sized + Message, O: Ownership> MsgSendId<T, Option<Id<U, O>>>
    for Assert<false, false, true>
{
    #[inline(always)]
    unsafe fn send_message_id<A: MessageArguments>(
        obj: T,
        sel: Sel,
        args: A,
    ) -> Result<Option<Id<U, O>>, MessageError> {
        unsafe { MessageReceiver::send_message(obj, sel, args).map(|r| Id::new(r)) }
    }
}

// All other selectors
impl<T: MessageReceiver, U: Message, O: Ownership> MsgSendId<T, Option<Id<U, O>>>
    for Assert<false, false, false>
{
    #[inline(always)]
    unsafe fn send_message_id<A: MessageArguments>(
        obj: T,
        sel: Sel,
        args: A,
    ) -> Result<Option<Id<U, O>>, MessageError> {
        // All code between the message send and the `retain_autoreleased`
        // must be able to be optimized away for this to work.
        unsafe { MessageReceiver::send_message(obj, sel, args).map(|r| Id::retain_autoreleased(r)) }
    }
}

#[doc(hidden)]
pub const fn starts_with_str(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.len() > haystack.len() {
        return false;
    }
    let mut i = 0;
    while i < needle.len() {
        if needle[i] != haystack[i] {
            return false;
        }
        i += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rc::{Id, Owned, Shared};
    use crate::runtime::Object;

    #[test]
    fn test_macro() {
        let cls = class!(NSObject);

        let _obj: Id<Object, Owned> = unsafe { msg_send_id![cls, new].unwrap() };

        let obj = unsafe { msg_send_id![cls, alloc] };

        let obj: Id<Object, Owned> = unsafe { msg_send_id![obj, init].unwrap() };

        // TODO:
        // let copy: Id<Object, Shared> = unsafe { msg_send_id![&obj, copy].unwrap() };
        // let mutable_copy: Id<Object, Shared> = unsafe { msg_send_id![&obj, mutableCopy].unwrap() };

        let _desc: Option<Id<Object, Shared>> = unsafe { msg_send_id![&obj, description] };
    }

    #[test]
    fn test_starts_with_str() {
        assert!(starts_with_str(b"abcdef", b"abc"));
        assert!(starts_with_str(b"a", b""));
        assert!(starts_with_str(b"", b""));

        assert!(!starts_with_str(b"abcdef", b"def"));
        assert!(!starts_with_str(b"abcdef", b"abb"));
        assert!(!starts_with_str(b"", b"a"));
    }

    mod test_trait_disambugated {
        use super::*;

        trait Abc {
            fn send_message_id() {}
        }

        impl<T> Abc for T {}

        #[test]
        fn test_macro_still_works() {
            let cls = class!(NSObject);
            let _obj: Id<Object, Owned> = unsafe { msg_send_id![cls, new].unwrap() };
        }
    }
}
