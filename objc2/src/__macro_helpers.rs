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

// https://clang.llvm.org/docs/AutomaticReferenceCounting.html#arc-method-families
#[doc(hidden)]
pub const fn in_method_family(mut selector: &[u8], mut family: &[u8]) -> bool {
    // Skip leading underscores from selector
    loop {
        selector = match selector {
            [b'_', selector @ ..] => (selector),
            _ => break,
        }
    }

    // Compare each character
    loop {
        (selector, family) = match (selector, family) {
            // Remaining items
            ([s, selector @ ..], [f, family @ ..]) => {
                if *s == *f {
                    // Next iteration
                    (selector, family)
                } else {
                    // Family does not begin with selector
                    return false;
                }
            }
            // Equal
            ([], []) => {
                return true;
            }
            // Selector can't be part of familiy if smaller than it
            ([], _) => {
                return false;
            }
            // Remaining items in selector
            // -> ensure next character is not lowercase
            ([s, ..], []) => {
                return !s.is_ascii_lowercase();
            }
        }
    }
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
    fn test_in_method_family() {
        // Common cases

        assert!(in_method_family(b"alloc", b"alloc"));
        assert!(in_method_family(b"allocWithZone:", b"alloc"));
        assert!(!in_method_family(b"dealloc", b"alloc"));
        assert!(!in_method_family(b"initialize", b"init"));
        assert!(!in_method_family(b"decimalNumberWithDecimal:", b"init"));
        assert!(in_method_family(b"initWithCapacity:", b"init"));
        assert!(in_method_family(b"_initButPrivate:withParam:", b"init"));
        assert!(!in_method_family(b"description", b"init"));
        assert!(!in_method_family(b"inIT", b"init"));

        assert!(!in_method_family(b"init", b"copy"));
        assert!(!in_method_family(b"copyingStuff:", b"copy"));
        assert!(in_method_family(b"copyWithZone:", b"copy"));
        assert!(!in_method_family(b"initWithArray:copyItems:", b"copy"));
        assert!(in_method_family(b"copyItemAtURL:toURL:error:", b"copy"));

        assert!(!in_method_family(b"mutableCopying", b"mutableCopy"));
        assert!(in_method_family(b"mutableCopyWithZone:", b"mutableCopy"));
        assert!(in_method_family(b"mutableCopyWithZone:", b"mutableCopy"));

        assert!(in_method_family(
            b"newScriptingObjectOfClass:forValueForKey:withContentsValue:properties:",
            b"new"
        ));
        assert!(in_method_family(
            b"newScriptingObjectOfClass:forValueForKey:withContentsValue:properties:",
            b"new"
        ));
        assert!(!in_method_family(b"newsstandAssetDownload", b"new"));

        // Trying to weed out edge-cases:

        assert!(in_method_family(b"__abcDef", b"abc"));
        assert!(in_method_family(b"_abcDef", b"abc"));
        assert!(in_method_family(b"abcDef", b"abc"));
        assert!(in_method_family(b"___a", b"a"));
        assert!(in_method_family(b"__a", b"a"));
        assert!(in_method_family(b"_a", b"a"));
        assert!(in_method_family(b"a", b"a"));

        assert!(!in_method_family(b"_abcdef", b"abc"));
        assert!(!in_method_family(b"_abcdef", b"def"));
        assert!(!in_method_family(b"_bcdef", b"abc"));
        assert!(!in_method_family(b"a_bc", b"abc"));
        assert!(!in_method_family(b"abcdef", b"abc"));
        assert!(!in_method_family(b"abcdef", b"def"));
        assert!(!in_method_family(b"abcdef", b"abb"));
        assert!(!in_method_family(b"___", b"a"));
        assert!(!in_method_family(b"_", b"a"));
        assert!(!in_method_family(b"", b"a"));

        assert!(in_method_family(b"copy", b"copy"));
        assert!(in_method_family(b"copy:", b"copy"));
        assert!(in_method_family(b"copyMe", b"copy"));
        assert!(in_method_family(b"_copy", b"copy"));
        assert!(in_method_family(b"_copy:", b"copy"));
        assert!(in_method_family(b"_copyMe", b"copy"));
        assert!(!in_method_family(b"copying", b"copy"));
        assert!(!in_method_family(b"copying:", b"copy"));
        assert!(!in_method_family(b"_copying", b"copy"));
        assert!(!in_method_family(b"Copy", b"copy"));
        assert!(!in_method_family(b"COPY", b"copy"));

        // Empty family (not supported)
        assert!(in_method_family(b"___", b""));
        assert!(in_method_family(b"__", b""));
        assert!(in_method_family(b"_", b""));
        assert!(in_method_family(b"", b""));
        assert!(!in_method_family(b"_a", b""));
        assert!(!in_method_family(b"a", b""));
        assert!(in_method_family(b"_A", b""));
        assert!(in_method_family(b"A", b""));
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
