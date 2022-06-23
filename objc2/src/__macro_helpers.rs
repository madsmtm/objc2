use crate::rc::{Id, Ownership};
use crate::runtime::{Class, Sel};
use crate::{Message, MessageArguments, MessageError, MessageReceiver};

pub use crate::cache::CachedClass;
pub use crate::cache::CachedSel;

pub use core::cell::UnsafeCell;
pub use core::option::Option::{self, None, Some};
pub use core::primitive::{bool, str, u8};
pub use core::result::Result::{Err, Ok};
pub use core::{compile_error, concat, panic, stringify};
#[cfg(feature = "objc2-proc-macros")]
pub use objc2_proc_macros::__hash_idents;

/// Helper for specifying the retain semantics for a given selector family.
///
/// Note that we can't actually check if a method is in a method family; only
/// whether the _selector_ is in a _selector_ family.
///
/// The slight difference here is:
/// - The method may be annotated with the `objc_method_family` attribute,
///   which would cause it to be in a different family. That this is not the
///   case is part of the `unsafe` contract of `msg_send_id!`.
/// - The method may not obey the added restrictions of the method family.
///   The added restrictions are:
///   - `new`, `alloc`, `copy` and `mutableCopy`: The method must return a
///     retainable object pointer type - we ensure this by making
///     `message_send_id` return `Id`.
///   - `init`: The method must be an instance method and must return an
///     Objective-C pointer type - We ensure this by taking `Id<T, O>`, which
///     means it can't be a class method!
///
/// While we're at it, we also limit a few other things to help the user out,
/// like only allowing `&Class` in `new` - this is not strictly required by
/// ARC though!
///
/// <https://clang.llvm.org/docs/AutomaticReferenceCounting.html#retainable-object-pointers-as-operands-and-arguments>
pub struct RetainSemantics<
    // `new` family
    const NEW: bool,
    // `alloc` family
    const ALLOC: bool,
    // `init` family
    const INIT: bool,
    // `copy` or `mutableCopy` family
    const COPY_OR_MUT_COPY: bool,
> {}

pub trait MsgSendId<T, U> {
    unsafe fn send_message_id<A: MessageArguments>(
        obj: T,
        sel: Sel,
        args: A,
    ) -> Result<Option<U>, MessageError>;
}

// `new`
impl<T: ?Sized + Message, O: Ownership> MsgSendId<&'_ Class, Id<T, O>>
    for RetainSemantics<true, false, false, false>
{
    #[inline]
    unsafe fn send_message_id<A: MessageArguments>(
        obj: &Class,
        sel: Sel,
        args: A,
    ) -> Result<Option<Id<T, O>>, MessageError> {
        unsafe { MessageReceiver::send_message(obj, sel, args).map(|r| Id::new(r)) }
    }
}

// `alloc`, should mark the return value as "allocated, not initialized" somehow
impl<T: ?Sized + Message, O: Ownership> MsgSendId<&'_ Class, Id<T, O>>
    for RetainSemantics<false, true, false, false>
{
    #[inline]
    unsafe fn send_message_id<A: MessageArguments>(
        cls: &Class,
        sel: Sel,
        args: A,
    ) -> Result<Option<Id<T, O>>, MessageError> {
        unsafe { MessageReceiver::send_message(cls, sel, args).map(|r| Id::new(r)) }
    }
}

// `init`, should mark the input value as "allocated, not initialized" somehow
impl<T: ?Sized + Message, O: Ownership> MsgSendId<Option<Id<T, O>>, Id<T, O>>
    for RetainSemantics<false, false, true, false>
{
    #[inline]
    unsafe fn send_message_id<A: MessageArguments>(
        obj: Option<Id<T, O>>,
        sel: Sel,
        args: A,
    ) -> Result<Option<Id<T, O>>, MessageError> {
        let ptr = Id::option_into_ptr(obj);
        // SAFETY: `ptr` may be null here, but that's fine since the return
        // is `*mut T`, which is one of the few types where messages to nil is
        // allowed.
        //
        // We do this for efficiency, to avoid having a branch after every
        // `alloc`, that the user did not intend.
        unsafe { MessageReceiver::send_message(ptr, sel, args).map(|r| Id::new(r)) }
    }
}

// `copy` and `mutableCopy`
impl<T: MessageReceiver, U: ?Sized + Message, O: Ownership> MsgSendId<T, Id<U, O>>
    for RetainSemantics<false, false, false, true>
{
    #[inline]
    unsafe fn send_message_id<A: MessageArguments>(
        obj: T,
        sel: Sel,
        args: A,
    ) -> Result<Option<Id<U, O>>, MessageError> {
        unsafe { MessageReceiver::send_message(obj, sel, args).map(|r| Id::new(r)) }
    }
}

// All other selectors
impl<T: MessageReceiver, U: Message, O: Ownership> MsgSendId<T, Id<U, O>>
    for RetainSemantics<false, false, false, false>
{
    #[inline]
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

/// Checks whether a given selector is said to be in a given selector family.
///
/// <https://clang.llvm.org/docs/AutomaticReferenceCounting.html#arc-method-families>
pub const fn in_selector_family(mut selector: &[u8], mut family: &[u8]) -> bool {
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

    use core::ptr;

    use crate::rc::{Owned, RcTestObject, Shared, ThreadTestData};
    use crate::runtime::Object;
    use crate::{Encoding, RefEncode};

    #[test]
    fn test_macro_alloc() {
        let mut expected = ThreadTestData::current();
        let cls = RcTestObject::class();

        let obj: Id<RcTestObject, Shared> = unsafe { msg_send_id![cls, alloc].unwrap() };
        expected.alloc += 1;
        expected.assert_current();

        drop(obj);
        expected.release += 1;
        expected.dealloc += 1;
        expected.assert_current();
    }

    #[test]
    #[cfg_attr(
        all(feature = "gnustep-1-7", feature = "verify_message"),
        ignore = "NSZone's encoding is quite complex on GNUStep"
    )]
    fn test_alloc_with_zone() {
        #[repr(C)]
        struct _NSZone {
            _inner: [u8; 0],
        }

        unsafe impl RefEncode for _NSZone {
            const ENCODING_REF: Encoding<'static> =
                Encoding::Pointer(&Encoding::Struct("_NSZone", &[]));
        }

        let expected = ThreadTestData::current();
        let cls = RcTestObject::class();

        let zone: *const _NSZone = ptr::null();
        let _obj: Id<RcTestObject, Owned> =
            unsafe { msg_send_id![cls, allocWithZone: zone].unwrap() };
        // `+[NSObject alloc]` delegates to `+[NSObject allocWithZone:]`, but
        // `RcTestObject` only catches `alloc`.
        // expected.alloc += 1;
        expected.assert_current();
    }

    #[test]
    fn test_macro_init() {
        let mut expected = ThreadTestData::current();
        let cls = RcTestObject::class();

        let obj: Option<Id<RcTestObject, Shared>> = unsafe { msg_send_id![cls, alloc] };
        expected.alloc += 1;
        // Don't check allocation error
        let _obj: Id<RcTestObject, Shared> = unsafe { msg_send_id![obj, init].unwrap() };
        expected.init += 1;
        expected.assert_current();

        let obj: Option<Id<RcTestObject, Shared>> = unsafe { msg_send_id![cls, alloc] };
        expected.alloc += 1;
        // Check allocation error before init
        let obj = obj.unwrap();
        let _obj: Id<RcTestObject, Shared> = unsafe { msg_send_id![Some(obj), init].unwrap() };
        expected.init += 1;
        expected.assert_current();
    }

    #[test]
    fn test_macro() {
        let mut expected = ThreadTestData::current();
        let cls = RcTestObject::class();
        crate::rc::autoreleasepool(|_| {
            let _obj: Id<RcTestObject, Owned> = unsafe { msg_send_id![cls, new].unwrap() };
            expected.alloc += 1;
            expected.init += 1;
            expected.assert_current();

            let obj = unsafe { msg_send_id![cls, alloc] };
            expected.alloc += 1;
            expected.assert_current();

            let obj: Id<RcTestObject, Owned> = unsafe { msg_send_id![obj, init].unwrap() };
            expected.init += 1;
            expected.assert_current();

            // TODO:
            // let copy: Id<RcTestObject, Shared> = unsafe { msg_send_id![&obj, copy].unwrap() };
            // let mutable_copy: Id<RcTestObject, Shared> = unsafe { msg_send_id![&obj, mutableCopy].unwrap() };

            let _self: Id<RcTestObject, Shared> = unsafe { msg_send_id![&obj, self].unwrap() };
            expected.retain += 1;
            expected.assert_current();

            let _desc: Option<Id<RcTestObject, Shared>> =
                unsafe { msg_send_id![&obj, description] };
            expected.assert_current();
        });
        expected.release += 3;
        expected.dealloc += 2;
        expected.assert_current();
    }

    #[test]
    fn test_in_selector_family() {
        // Common cases

        assert!(in_selector_family(b"alloc", b"alloc"));
        assert!(in_selector_family(b"allocWithZone:", b"alloc"));
        assert!(!in_selector_family(b"dealloc", b"alloc"));
        assert!(!in_selector_family(b"initialize", b"init"));
        assert!(!in_selector_family(b"decimalNumberWithDecimal:", b"init"));
        assert!(in_selector_family(b"initWithCapacity:", b"init"));
        assert!(in_selector_family(b"_initButPrivate:withParam:", b"init"));
        assert!(!in_selector_family(b"description", b"init"));
        assert!(!in_selector_family(b"inIT", b"init"));

        assert!(!in_selector_family(b"init", b"copy"));
        assert!(!in_selector_family(b"copyingStuff:", b"copy"));
        assert!(in_selector_family(b"copyWithZone:", b"copy"));
        assert!(!in_selector_family(b"initWithArray:copyItems:", b"copy"));
        assert!(in_selector_family(b"copyItemAtURL:toURL:error:", b"copy"));

        assert!(!in_selector_family(b"mutableCopying", b"mutableCopy"));
        assert!(in_selector_family(b"mutableCopyWithZone:", b"mutableCopy"));
        assert!(in_selector_family(b"mutableCopyWithZone:", b"mutableCopy"));

        assert!(in_selector_family(
            b"newScriptingObjectOfClass:forValueForKey:withContentsValue:properties:",
            b"new"
        ));
        assert!(in_selector_family(
            b"newScriptingObjectOfClass:forValueForKey:withContentsValue:properties:",
            b"new"
        ));
        assert!(!in_selector_family(b"newsstandAssetDownload", b"new"));

        // Trying to weed out edge-cases:

        assert!(in_selector_family(b"__abcDef", b"abc"));
        assert!(in_selector_family(b"_abcDef", b"abc"));
        assert!(in_selector_family(b"abcDef", b"abc"));
        assert!(in_selector_family(b"___a", b"a"));
        assert!(in_selector_family(b"__a", b"a"));
        assert!(in_selector_family(b"_a", b"a"));
        assert!(in_selector_family(b"a", b"a"));

        assert!(!in_selector_family(b"_abcdef", b"abc"));
        assert!(!in_selector_family(b"_abcdef", b"def"));
        assert!(!in_selector_family(b"_bcdef", b"abc"));
        assert!(!in_selector_family(b"a_bc", b"abc"));
        assert!(!in_selector_family(b"abcdef", b"abc"));
        assert!(!in_selector_family(b"abcdef", b"def"));
        assert!(!in_selector_family(b"abcdef", b"abb"));
        assert!(!in_selector_family(b"___", b"a"));
        assert!(!in_selector_family(b"_", b"a"));
        assert!(!in_selector_family(b"", b"a"));

        assert!(in_selector_family(b"copy", b"copy"));
        assert!(in_selector_family(b"copy:", b"copy"));
        assert!(in_selector_family(b"copyMe", b"copy"));
        assert!(in_selector_family(b"_copy", b"copy"));
        assert!(in_selector_family(b"_copy:", b"copy"));
        assert!(in_selector_family(b"_copyMe", b"copy"));
        assert!(!in_selector_family(b"copying", b"copy"));
        assert!(!in_selector_family(b"copying:", b"copy"));
        assert!(!in_selector_family(b"_copying", b"copy"));
        assert!(!in_selector_family(b"Copy", b"copy"));
        assert!(!in_selector_family(b"COPY", b"copy"));

        // Empty family (not supported)
        assert!(in_selector_family(b"___", b""));
        assert!(in_selector_family(b"__", b""));
        assert!(in_selector_family(b"_", b""));
        assert!(in_selector_family(b"", b""));
        assert!(!in_selector_family(b"_a", b""));
        assert!(!in_selector_family(b"a", b""));
        assert!(in_selector_family(b"_A", b""));
        assert!(in_selector_family(b"A", b""));
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

    #[test]
    #[cfg(feature = "objc2-proc-macros")]
    fn hash_idents_different() {
        assert_ne!(__hash_idents!(abc), __hash_idents!(def));
    }

    #[test]
    #[cfg(feature = "objc2-proc-macros")]
    fn hash_idents_same_no_equal() {
        assert_ne!(__hash_idents!(abc), __hash_idents!(abc));
        assert_ne!(__hash_idents!(abc def ghi), __hash_idents!(abc def ghi));
    }

    #[test]
    #[cfg(feature = "objc2-proc-macros")]
    fn hash_idents_exact_same_ident() {
        macro_rules! x {
            ($x:ident) => {
                (__hash_idents!($x), __hash_idents!($x))
            };
        }
        let (ident1, ident2) = x!(abc);
        // This is a limitation of `__hash_idents`, ideally we'd like these
        // to be different!
        assert_eq!(ident1, ident2);
    }
}
