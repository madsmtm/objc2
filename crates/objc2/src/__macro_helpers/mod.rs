#[cfg(all(debug_assertions, feature = "verify"))]
use alloc::vec::Vec;
use core::ptr;
#[cfg(all(debug_assertions, feature = "verify"))]
use std::collections::HashSet;

use crate::declare::ClassBuilder;
use crate::declare::MethodImplementation;
use crate::encode::Encode;
use crate::message::__TupleExtender;
use crate::rc::{Allocated, Id};
#[cfg(all(debug_assertions, feature = "verify"))]
use crate::runtime::MethodDescription;
use crate::runtime::{AnyClass, AnyObject, AnyProtocol, Sel};
use crate::{Message, MessageArguments, MessageReceiver};

pub use core::borrow::{Borrow, BorrowMut};
pub use core::cell::UnsafeCell;
pub use core::convert::{AsMut, AsRef};
pub use core::marker::{PhantomData, Sized};
pub use core::mem::{needs_drop, size_of, ManuallyDrop};
pub use core::ops::{Deref, DerefMut};
pub use core::option::Option::{self, None, Some};
pub use core::primitive::{bool, str, u8};
pub use core::ptr::drop_in_place;
pub use core::{compile_error, concat, panic, stringify};
// TODO: Use `core::cell::LazyCell`
pub use std::sync::Once;

mod cache;
mod common_selectors;
mod declare_class;

pub use self::cache::{CachedClass, CachedSel};
pub use self::common_selectors::{alloc_sel, dealloc_sel, init_sel, new_sel};
pub use self::declare_class::{
    assert_mutability_matches_superclass_mutability, MaybeOptionId, MessageRecieveId,
    ValidSubclassMutability,
};

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
///     Objective-C pointer type - We ensure this by taking
///     `Option<Allocated<T>>`, which means it can't be a class method!
///
/// <https://clang.llvm.org/docs/AutomaticReferenceCounting.html#retainable-object-pointers-as-operands-and-arguments>
// TODO: Use an enum instead of u8 here when stable
pub struct RetainSemantics<const INNER: u8> {}

pub type New = RetainSemantics<1>;
pub type Alloc = RetainSemantics<2>;
pub type Init = RetainSemantics<3>;
pub type CopyOrMutCopy = RetainSemantics<4>;
pub type Other = RetainSemantics<5>;

pub const fn retain_semantics(selector: &str) -> u8 {
    let selector = selector.as_bytes();
    match (
        in_selector_family(selector, b"new"),
        in_selector_family(selector, b"alloc"),
        in_selector_family(selector, b"init"),
        in_selector_family(selector, b"copy"),
        in_selector_family(selector, b"mutableCopy"),
    ) {
        (true, false, false, false, false) => 1,
        (false, true, false, false, false) => 2,
        (false, false, true, false, false) => 3,
        (false, false, false, true, false) => 4,
        (false, false, false, false, true) => 4,
        (false, false, false, false, false) => 5,
        _ => unreachable!(),
    }
}

pub trait MsgSendId<T, U> {
    #[track_caller]
    unsafe fn send_message_id<A: MessageArguments, R: MaybeUnwrap<Input = U>>(
        obj: T,
        sel: Sel,
        args: A,
    ) -> R;

    /// Add an extra error argument to the argument list, call
    /// `send_message_id` with that, and return an error if one occurred.
    #[inline]
    #[track_caller]
    unsafe fn send_message_id_error<A, E>(obj: T, sel: Sel, args: A) -> Result<U, Id<E>>
    where
        *mut *mut E: Encode,
        A: __TupleExtender<*mut *mut E>,
        <A as __TupleExtender<*mut *mut E>>::PlusOneArgument: MessageArguments,
        E: Message,
        Option<U>: MaybeUnwrap<Input = U>,
    {
        let mut err: *mut E = ptr::null_mut();
        let args = args.add_argument(&mut err);
        let res: Option<U> = unsafe { Self::send_message_id(obj, sel, args) };
        // As per the Cocoa documentation:
        // > Success or failure is indicated by the return value of the
        // > method. Although Cocoa methods that indirectly return error
        // > objects in the Cocoa error domain are guaranteed to return such
        // > objects if the method indicates failure by directly returning
        // > `nil` or `NO`, you should always check that the return value is
        // > `nil` or `NO` before attempting to do anything with the `NSError`
        // > object.
        if let Some(res) = res {
            // In this case, the error is likely not created. If it is, it is
            // autoreleased anyhow, so it would be a waste to retain and
            // release it here.
            Ok(res)
        } else {
            // In this case, the error has very likely been created, but has
            // been autoreleased (as is common for "out parameters", see
            // `src/rc/writeback.rs`). Hence we need to retain it if we want
            // it to live across autorelease pools.
            //
            // SAFETY: The message send is guaranteed to populate the error
            // object, or leave it as NULL. The error is shared, and all
            // holders of the error know this, so is safe to retain.
            Err(unsafe { encountered_error(err) })
        }
    }
}

// Marked `cold` to tell the optimizer that errors are comparatively rare.
// And intentionally not inlined, for much the same reason.
#[cold]
#[track_caller]
unsafe fn encountered_error<E: Message>(err: *mut E) -> Id<E> {
    // SAFETY: Ensured by caller
    unsafe { Id::retain(err) }.expect("error parameter should be set if the method returns NULL")
}

impl<T: MessageReceiver, U: ?Sized + Message> MsgSendId<T, Id<U>> for New {
    #[inline]
    unsafe fn send_message_id<A: MessageArguments, R: MaybeUnwrap<Input = Id<U>>>(
        obj: T,
        sel: Sel,
        args: A,
    ) -> R {
        let ptr = obj.__as_raw_receiver();
        // SAFETY: Checked by caller
        let obj = unsafe { MessageReceiver::send_message(ptr, sel, args) };
        // SAFETY: The selector is `new`, so this has +1 retain count
        let obj = unsafe { Id::new(obj) };

        // SAFETY: The object is still valid after a message send to a `new`
        // method - it would not be if the method was `init`.
        R::maybe_unwrap::<Self>(obj, (unsafe { ptr.as_ref() }, sel))
    }
}

impl<T: ?Sized + Message> MsgSendId<&'_ AnyClass, Allocated<T>> for Alloc {
    #[inline]
    unsafe fn send_message_id<A: MessageArguments, R: MaybeUnwrap<Input = Allocated<T>>>(
        cls: &AnyClass,
        sel: Sel,
        args: A,
    ) -> R {
        // SAFETY: Checked by caller
        let obj = unsafe { MessageReceiver::send_message(cls, sel, args) };
        // SAFETY: The selector is `alloc`, so this has +1 retain count
        let obj = unsafe { Allocated::new(obj) };
        R::maybe_unwrap::<Self>(obj, (cls, sel))
    }
}

impl<T: ?Sized + Message> MsgSendId<Option<Allocated<T>>, Id<T>> for Init {
    #[inline]
    unsafe fn send_message_id<A: MessageArguments, R: MaybeUnwrap<Input = Id<T>>>(
        obj: Option<Allocated<T>>,
        sel: Sel,
        args: A,
    ) -> R {
        let ptr = Allocated::option_into_ptr(obj);
        // SAFETY: `ptr` may be null here, but that's fine since the return
        // is `*mut T`, which is one of the few types where messages to nil is
        // allowed.
        //
        // We do this for efficiency, to avoid having a branch that the user
        // did not intend after every `alloc`.
        let obj = unsafe { MessageReceiver::send_message(ptr, sel, args) };
        // SAFETY: The selector is `init`, so this has +1 retain count
        let obj = unsafe { Id::new(obj) };
        R::maybe_unwrap::<Self>(obj, (ptr.cast(), sel))
    }
}

impl<T: MessageReceiver, U: ?Sized + Message> MsgSendId<T, Id<U>> for CopyOrMutCopy {
    #[inline]
    unsafe fn send_message_id<A: MessageArguments, R: MaybeUnwrap<Input = Id<U>>>(
        obj: T,
        sel: Sel,
        args: A,
    ) -> R {
        // SAFETY: Checked by caller
        let obj = unsafe { MessageReceiver::send_message(obj, sel, args) };
        // SAFETY: The selector is `copy` or `mutableCopy`, so this has +1
        // retain count
        let obj = unsafe { Id::new(obj) };
        R::maybe_unwrap::<Self>(obj, ())
    }
}

impl<T: MessageReceiver, U: Message> MsgSendId<T, Id<U>> for Other {
    #[inline]
    unsafe fn send_message_id<A: MessageArguments, R: MaybeUnwrap<Input = Id<U>>>(
        obj: T,
        sel: Sel,
        args: A,
    ) -> R {
        let ptr = obj.__as_raw_receiver();
        // SAFETY: Checked by caller
        let obj = unsafe { MessageReceiver::send_message(ptr, sel, args) };
        // All code between the message send and the `retain_autoreleased`
        // must be able to be optimized away for this to work.

        // SAFETY: The selector is not `new`, `alloc`, `init`, `copy` nor
        // `mutableCopy`, so the object must be manually retained.
        let obj = unsafe { Id::retain_autoreleased(obj) };

        // SAFETY: The object is still valid after a message send to a
        // normal method - it would not be if the method was `init`.
        R::maybe_unwrap::<Self>(obj, (unsafe { ptr.as_ref() }, sel))
    }
}

pub trait MaybeUnwrap {
    type Input;
    #[track_caller]
    fn maybe_unwrap<'a, F: MsgSendIdFailed<'a>>(obj: Option<Self::Input>, args: F::Args) -> Self;
}

impl<T: ?Sized> MaybeUnwrap for Option<Id<T>> {
    type Input = Id<T>;

    #[inline]
    fn maybe_unwrap<'a, F: MsgSendIdFailed<'a>>(obj: Option<Id<T>>, _args: F::Args) -> Self {
        obj
    }
}

impl<T: ?Sized> MaybeUnwrap for Id<T> {
    type Input = Id<T>;

    #[inline]
    fn maybe_unwrap<'a, F: MsgSendIdFailed<'a>>(obj: Option<Id<T>>, args: F::Args) -> Self {
        match obj {
            Some(obj) => obj,
            None => F::failed(args),
        }
    }
}

impl<T: ?Sized> MaybeUnwrap for Option<Allocated<T>> {
    type Input = Allocated<T>;

    #[inline]
    fn maybe_unwrap<'a, F: MsgSendIdFailed<'a>>(obj: Option<Allocated<T>>, _args: F::Args) -> Self {
        obj
    }
}

impl<T: ?Sized> MaybeUnwrap for Allocated<T> {
    type Input = Allocated<T>;

    #[inline]
    fn maybe_unwrap<'a, F: MsgSendIdFailed<'a>>(obj: Option<Allocated<T>>, args: F::Args) -> Self {
        match obj {
            Some(obj) => obj,
            None => F::failed(args),
        }
    }
}

// Note: It would have been much easier to do this kind of thing using
// closures, but then `track_caller` doesn't work properly!
//
// Also note: This behavior (that #[method_id(...)] always unwraps instead of
// using `unwrap_unchecked`) is relied upon by `header-translator` for
// soundness, see e.g. `parse_property_return`.
pub trait MsgSendIdFailed<'a> {
    type Args;

    #[track_caller]
    fn failed(args: Self::Args) -> !;
}

impl<'a> MsgSendIdFailed<'a> for New {
    type Args = (Option<&'a AnyObject>, Sel);

    #[cold]
    fn failed((obj, sel): Self::Args) -> ! {
        if let Some(obj) = obj {
            let cls = obj.class();
            if cls.is_metaclass() {
                if sel == new_sel() {
                    panic!("failed creating new instance of {cls}")
                } else {
                    panic!("failed creating new instance using +[{cls} {sel}]")
                }
            } else {
                panic!("unexpected NULL returned from -[{cls} {sel}]")
            }
        } else {
            panic!("unexpected NULL {sel}; receiver was NULL");
        }
    }
}

impl<'a> MsgSendIdFailed<'a> for Alloc {
    type Args = (&'a AnyClass, Sel);

    #[cold]
    fn failed((cls, sel): Self::Args) -> ! {
        if sel == alloc_sel() {
            panic!("failed allocating {cls}")
        } else {
            panic!("failed allocating with +[{cls} {sel}]")
        }
    }
}

impl MsgSendIdFailed<'_> for Init {
    type Args = (*const AnyObject, Sel);

    #[cold]
    fn failed((ptr, sel): Self::Args) -> ! {
        if ptr.is_null() {
            panic!("failed allocating object")
        } else {
            // We can't really display a more descriptive message here since the
            // object is consumed by `init` and may not be valid any more.
            if sel == init_sel() {
                panic!("failed initializing object")
            } else {
                panic!("failed initializing object with -{sel}")
            }
        }
    }
}

impl MsgSendIdFailed<'_> for CopyOrMutCopy {
    type Args = ();

    #[cold]
    fn failed(_: Self::Args) -> ! {
        panic!("failed copying object")
    }
}

impl<'a> MsgSendIdFailed<'a> for Other {
    type Args = (Option<&'a AnyObject>, Sel);

    #[cold]
    fn failed((obj, sel): Self::Args) -> ! {
        if let Some(obj) = obj {
            let cls = obj.class();
            panic!(
                "unexpected NULL returned from {}[{cls} {sel}]",
                if cls.is_metaclass() { "+" } else { "-" },
            )
        } else {
            panic!("unexpected NULL {sel}; receiver was NULL");
        }
    }
}

/// Checks whether a given selector is said to be in a given selector family.
///
/// <https://clang.llvm.org/docs/AutomaticReferenceCounting.html#arc-method-families>
const fn in_selector_family(mut selector: &[u8], mut family: &[u8]) -> bool {
    // Skip leading underscores from selector
    loop {
        selector = match selector {
            [b'_', rest @ ..] => rest,
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

/// Helper struct for emitting the module info that macOS 32-bit requires.
///
/// <https://github.com/llvm/llvm-project/blob/release/13.x/clang/lib/CodeGen/CGObjCMac.cpp#L5211-L5234>
#[repr(C)]
pub struct ModuleInfo {
    version: usize,
    size: usize,
    name: *const u8,
    symtab: *const (),
}

// SAFETY: ModuleInfo is immutable.
unsafe impl Sync for ModuleInfo {}

impl ModuleInfo {
    /// This is hardcoded in clang as 7.
    const VERSION: usize = 7;

    pub const fn new(name: *const u8) -> Self {
        Self {
            version: Self::VERSION,
            size: core::mem::size_of::<Self>(),
            name,
            // We don't expose any symbols
            symtab: core::ptr::null(),
        }
    }
}

impl ClassBuilder {
    #[doc(hidden)]
    pub fn __add_protocol_methods<'a, 'b>(
        &'a mut self,
        protocol: Option<&'b AnyProtocol>,
    ) -> ClassProtocolMethodsBuilder<'a, 'b> {
        if let Some(protocol) = protocol {
            self.add_protocol(protocol);
        }

        #[cfg(all(debug_assertions, feature = "verify"))]
        {
            ClassProtocolMethodsBuilder {
                builder: self,
                protocol,
                required_instance_methods: protocol
                    .map(|p| p.method_descriptions(true))
                    .unwrap_or_default(),
                optional_instance_methods: protocol
                    .map(|p| p.method_descriptions(false))
                    .unwrap_or_default(),
                registered_instance_methods: HashSet::new(),
                required_class_methods: protocol
                    .map(|p| p.class_method_descriptions(true))
                    .unwrap_or_default(),
                optional_class_methods: protocol
                    .map(|p| p.class_method_descriptions(false))
                    .unwrap_or_default(),
                registered_class_methods: HashSet::new(),
            }
        }

        #[cfg(not(all(debug_assertions, feature = "verify")))]
        {
            ClassProtocolMethodsBuilder {
                builder: self,
                protocol,
            }
        }
    }
}

/// Helper for ensuring that:
/// - Only methods on the protocol are overriden.
/// - TODO: The methods have the correct signature.
/// - All required methods are overridden.
pub struct ClassProtocolMethodsBuilder<'a, 'b> {
    builder: &'a mut ClassBuilder,
    #[allow(unused)]
    protocol: Option<&'b AnyProtocol>,
    #[cfg(all(debug_assertions, feature = "verify"))]
    required_instance_methods: Vec<MethodDescription>,
    #[cfg(all(debug_assertions, feature = "verify"))]
    optional_instance_methods: Vec<MethodDescription>,
    #[cfg(all(debug_assertions, feature = "verify"))]
    registered_instance_methods: HashSet<Sel>,
    #[cfg(all(debug_assertions, feature = "verify"))]
    required_class_methods: Vec<MethodDescription>,
    #[cfg(all(debug_assertions, feature = "verify"))]
    optional_class_methods: Vec<MethodDescription>,
    #[cfg(all(debug_assertions, feature = "verify"))]
    registered_class_methods: HashSet<Sel>,
}

impl ClassProtocolMethodsBuilder<'_, '_> {
    #[inline]
    pub unsafe fn add_method<T, F>(&mut self, sel: Sel, func: F)
    where
        T: Message + ?Sized,
        F: MethodImplementation<Callee = T>,
    {
        #[cfg(all(debug_assertions, feature = "verify"))]
        if let Some(protocol) = self.protocol {
            let _types = self
                .required_instance_methods
                .iter()
                .chain(&self.optional_instance_methods)
                .find(|desc| desc.sel == sel)
                .map(|desc| desc.types)
                .unwrap_or_else(|| {
                    panic!(
                        "failed overriding protocol method -[{protocol} {sel}]: method not found"
                    )
                });
        }

        // SAFETY: Checked by caller
        unsafe { self.builder.add_method(sel, func) };

        #[cfg(all(debug_assertions, feature = "verify"))]
        if !self.registered_instance_methods.insert(sel) {
            unreachable!("already added")
        }
    }

    #[inline]
    pub unsafe fn add_class_method<F>(&mut self, sel: Sel, func: F)
    where
        F: MethodImplementation<Callee = AnyClass>,
    {
        #[cfg(all(debug_assertions, feature = "verify"))]
        if let Some(protocol) = self.protocol {
            let _types = self
                .required_class_methods
                .iter()
                .chain(&self.optional_class_methods)
                .find(|desc| desc.sel == sel)
                .map(|desc| desc.types)
                .unwrap_or_else(|| {
                    panic!(
                        "failed overriding protocol method +[{protocol} {sel}]: method not found"
                    )
                });
        }

        // SAFETY: Checked by caller
        unsafe { self.builder.add_class_method(sel, func) };

        #[cfg(all(debug_assertions, feature = "verify"))]
        if !self.registered_class_methods.insert(sel) {
            unreachable!("already added")
        }
    }

    #[inline]
    pub fn __finish(self) {
        #[cfg(all(debug_assertions, feature = "verify"))]
        if let Some(protocol) = self.protocol {
            for desc in &self.required_instance_methods {
                if !self.registered_instance_methods.contains(&desc.sel) {
                    panic!(
                        "must implement required protocol method -[{protocol} {}]",
                        desc.sel
                    )
                }
            }
        }

        #[cfg(all(debug_assertions, feature = "verify"))]
        if let Some(protocol) = self.protocol {
            for desc in &self.required_class_methods {
                if !self.registered_class_methods.contains(&desc.sel) {
                    panic!(
                        "must implement required protocol method +[{protocol} {}]",
                        desc.sel
                    )
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::string::ToString;
    use core::ptr;

    #[cfg(feature = "objc2-proc-macros")]
    use crate::__hash_idents;
    use crate::rc::{__RcTestObject, __ThreadTestData};
    use crate::runtime::{AnyObject, NSObject, NSZone};
    use crate::{class, msg_send_id, ClassType};

    #[test]
    fn test_new() {
        let _obj: Id<AnyObject> = unsafe { msg_send_id![NSObject::class(), new] };
        let _obj: Option<Id<AnyObject>> = unsafe { msg_send_id![NSObject::class(), new] };
    }

    #[test]
    fn test_new_not_on_class() {
        let mut expected = __ThreadTestData::current();
        let obj = __RcTestObject::new();

        let _obj: Id<AnyObject> = unsafe { msg_send_id![&obj, newMethodOnInstance] };
        let _obj: Option<Id<AnyObject>> = unsafe { msg_send_id![&obj, newMethodOnInstance] };
        expected.alloc += 3;
        expected.init += 3;
        expected.assert_current();
    }

    #[test]
    // newScriptingObjectOfClass only available on macOS
    #[cfg_attr(not(all(feature = "apple", target_os = "macos")), ignore)]
    fn test_new_with_args() {
        let mut expected = __ThreadTestData::current();

        let object_class = __RcTestObject::class();
        let key: Id<AnyObject> = unsafe { msg_send_id![class!(NSString), new] };
        let contents_value: *const AnyObject = ptr::null();
        let properties: Id<AnyObject> = unsafe { msg_send_id![class!(NSDictionary), new] };

        let _obj: Option<Id<AnyObject>> = unsafe {
            msg_send_id![
                NSObject::class(),
                newScriptingObjectOfClass: object_class,
                forValueForKey: &*key,
                withContentsValue: contents_value,
                properties: &*properties,
            ]
        };
        expected.alloc += 1;
        expected.init += 1;
        expected.assert_current();
    }

    #[test]
    fn test_macro_alloc() {
        let mut expected = __ThreadTestData::current();
        let cls = __RcTestObject::class();

        let obj: Allocated<__RcTestObject> = unsafe { msg_send_id![cls, alloc] };
        expected.alloc += 1;
        expected.assert_current();

        drop(obj);
        expected.release += 1;
        expected.dealloc += 1;
        expected.assert_current();
    }

    #[test]
    fn test_alloc_with_zone() {
        let mut expected = __ThreadTestData::current();
        let cls = __RcTestObject::class();

        let zone: *const NSZone = ptr::null();
        let _obj: Allocated<__RcTestObject> = unsafe { msg_send_id![cls, allocWithZone: zone] };
        expected.alloc += 1;
        expected.assert_current();
    }

    #[test]
    fn test_macro_init() {
        let mut expected = __ThreadTestData::current();
        let cls = __RcTestObject::class();

        let obj: Option<Allocated<__RcTestObject>> = unsafe { msg_send_id![cls, alloc] };
        expected.alloc += 1;
        // Don't check allocation error
        let _obj: Id<__RcTestObject> = unsafe { msg_send_id![obj, init] };
        expected.init += 1;
        expected.assert_current();

        let obj: Option<Allocated<__RcTestObject>> = unsafe { msg_send_id![cls, alloc] };
        expected.alloc += 1;
        // Check allocation error before init
        let obj = obj.unwrap();
        let _obj: Id<__RcTestObject> = unsafe { msg_send_id![Some(obj), init] };
        expected.init += 1;
        expected.assert_current();
    }

    #[test]
    fn test_macro() {
        let mut expected = __ThreadTestData::current();
        let cls = __RcTestObject::class();
        crate::rc::autoreleasepool(|_| {
            let _obj: Id<__RcTestObject> = unsafe { msg_send_id![cls, new] };
            expected.alloc += 1;
            expected.init += 1;
            expected.assert_current();

            let obj = unsafe { msg_send_id![cls, alloc] };
            expected.alloc += 1;
            expected.assert_current();

            let obj: Id<__RcTestObject> = unsafe { msg_send_id![obj, init] };
            expected.init += 1;
            expected.assert_current();

            let _copy: Id<__RcTestObject> = unsafe { msg_send_id![&obj, copy] };
            expected.copy += 1;
            expected.alloc += 1;
            expected.init += 1;
            expected.assert_current();

            let _mutable_copy: Id<__RcTestObject> = unsafe { msg_send_id![&obj, mutableCopy] };
            expected.mutable_copy += 1;
            expected.alloc += 1;
            expected.init += 1;
            expected.assert_current();

            let _self: Id<__RcTestObject> = unsafe { msg_send_id![&obj, self] };
            expected.retain += 1;
            expected.assert_current();

            let _desc: Option<Id<__RcTestObject>> = unsafe { msg_send_id![&obj, description] };
            expected.assert_current();
        });
        expected.release += 5;
        expected.dealloc += 4;
        expected.assert_current();
    }

    #[test]
    #[should_panic = "failed creating new instance of NSValue"]
    // GNUStep instead returns an invalid instance that panics on accesses
    #[cfg_attr(feature = "gnustep-1-7", ignore)]
    fn new_nsvalue_fails() {
        let _val: Id<AnyObject> = unsafe { msg_send_id![class!(NSValue), new] };
    }

    #[test]
    #[should_panic = "failed creating new instance using +[__RcTestObject newReturningNull]"]
    fn test_new_with_null() {
        let _obj: Id<__RcTestObject> =
            unsafe { msg_send_id![__RcTestObject::class(), newReturningNull] };
    }

    #[test]
    #[should_panic = "unexpected NULL returned from -[__RcTestObject newMethodOnInstanceNull]"]
    fn test_new_any_with_null() {
        let obj = __RcTestObject::new();
        let _obj: Id<AnyObject> = unsafe { msg_send_id![&obj, newMethodOnInstanceNull] };
    }

    #[test]
    #[should_panic = "unexpected NULL newMethodOnInstance; receiver was NULL"]
    #[cfg(not(debug_assertions))] // Does NULL receiver checks
    fn test_new_any_with_null_receiver() {
        let obj: *const NSObject = ptr::null();
        let _obj: Id<AnyObject> = unsafe { msg_send_id![obj, newMethodOnInstance] };
    }

    #[test]
    #[should_panic = "failed allocating with +[__RcTestObject allocReturningNull]"]
    fn test_alloc_with_null() {
        let _obj: Allocated<__RcTestObject> =
            unsafe { msg_send_id![__RcTestObject::class(), allocReturningNull] };
    }

    #[test]
    #[should_panic = "failed initializing object with -initReturningNull"]
    fn test_init_with_null() {
        let obj: Option<Allocated<__RcTestObject>> =
            unsafe { msg_send_id![__RcTestObject::class(), alloc] };
        let _obj: Id<__RcTestObject> = unsafe { msg_send_id![obj, initReturningNull] };
    }

    #[test]
    #[should_panic = "failed allocating object"]
    #[cfg(not(debug_assertions))] // Does NULL receiver checks
    fn test_init_with_null_receiver() {
        let obj: Option<Allocated<__RcTestObject>> = None;
        let _obj: Id<__RcTestObject> = unsafe { msg_send_id![obj, init] };
    }

    #[test]
    #[should_panic = "failed copying object"]
    fn test_copy_with_null() {
        let obj = __RcTestObject::new();
        let _obj: Id<__RcTestObject> = unsafe { msg_send_id![&obj, copyReturningNull] };
    }

    #[test]
    #[should_panic = "unexpected NULL returned from -[__RcTestObject methodReturningNull]"]
    fn test_normal_with_null() {
        let obj = __RcTestObject::new();
        let _obj: Id<__RcTestObject> = unsafe { msg_send_id![&obj, methodReturningNull] };
    }

    #[test]
    #[should_panic = "unexpected NULL returned from -[__RcTestObject aMethod:]"]
    fn test_normal_with_param_and_null() {
        let obj = __RcTestObject::new();
        let _obj: Id<__RcTestObject> = unsafe { msg_send_id![&obj, aMethod: false] };
    }

    #[test]
    #[should_panic = "unexpected NULL description; receiver was NULL"]
    #[cfg(not(debug_assertions))] // Does NULL receiver checks
    fn test_normal_with_null_receiver() {
        let obj: *const NSObject = ptr::null();
        let _obj: Id<AnyObject> = unsafe { msg_send_id![obj, description] };
    }

    #[test]
    fn test_in_selector_family() {
        #[track_caller]
        fn assert_in_family(selector: &str, family: &str) {
            assert!(in_selector_family(selector.as_bytes(), family.as_bytes()));
            let selector = selector.to_string() + "\0";
            assert!(in_selector_family(selector.as_bytes(), family.as_bytes()));
        }

        #[track_caller]
        fn assert_not_in_family(selector: &str, family: &str) {
            assert!(!in_selector_family(selector.as_bytes(), family.as_bytes()));
            let selector = selector.to_string() + "\0";
            assert!(!in_selector_family(selector.as_bytes(), family.as_bytes()));
        }

        // Common cases

        assert_in_family("alloc", "alloc");
        assert_in_family("allocWithZone:", "alloc");
        assert_not_in_family("dealloc", "alloc");
        assert_not_in_family("initialize", "init");
        assert_not_in_family("decimalNumberWithDecimal:", "init");
        assert_in_family("initWithCapacity:", "init");
        assert_in_family("_initButPrivate:withParam:", "init");
        assert_not_in_family("description", "init");
        assert_not_in_family("inIT", "init");

        assert_not_in_family("init", "copy");
        assert_not_in_family("copyingStuff:", "copy");
        assert_in_family("copyWithZone:", "copy");
        assert_not_in_family("initWithArray:copyItems:", "copy");
        assert_in_family("copyItemAtURL:toURL:error:", "copy");

        assert_not_in_family("mutableCopying", "mutableCopy");
        assert_in_family("mutableCopyWithZone:", "mutableCopy");
        assert_in_family("mutableCopyWithZone:", "mutableCopy");

        assert_in_family(
            "newScriptingObjectOfClass:forValueForKey:withContentsValue:properties:",
            "new",
        );
        assert_in_family(
            "newScriptingObjectOfClass:forValueForKey:withContentsValue:properties:",
            "new",
        );
        assert_not_in_family("newsstandAssetDownload", "new");

        // Trying to weed out edge-cases:

        assert_in_family("__abcDef", "abc");
        assert_in_family("_abcDef", "abc");
        assert_in_family("abcDef", "abc");
        assert_in_family("___a", "a");
        assert_in_family("__a", "a");
        assert_in_family("_a", "a");
        assert_in_family("a", "a");

        assert_not_in_family("_abcdef", "abc");
        assert_not_in_family("_abcdef", "def");
        assert_not_in_family("_bcdef", "abc");
        assert_not_in_family("a_bc", "abc");
        assert_not_in_family("abcdef", "abc");
        assert_not_in_family("abcdef", "def");
        assert_not_in_family("abcdef", "abb");
        assert_not_in_family("___", "a");
        assert_not_in_family("_", "a");
        assert_not_in_family("", "a");

        assert_in_family("copy", "copy");
        assert_in_family("copy:", "copy");
        assert_in_family("copyMe", "copy");
        assert_in_family("_copy", "copy");
        assert_in_family("_copy:", "copy");
        assert_in_family("_copyMe", "copy");
        assert_not_in_family("copying", "copy");
        assert_not_in_family("copying:", "copy");
        assert_not_in_family("_copying", "copy");
        assert_not_in_family("Copy", "copy");
        assert_not_in_family("COPY", "copy");

        // Empty family (not supported)
        assert_in_family("___", "");
        assert_in_family("__", "");
        assert_in_family("_", "");
        assert_in_family("", "");
        assert_not_in_family("_a", "");
        assert_not_in_family("a", "");
        assert_in_family("_A", "");
        assert_in_family("A", "");

        // Double-colon selectors
        assert_in_family("abc::abc::", "abc");
        assert_in_family("abc:::", "abc");
        assert_in_family("abcDef::xyz:", "abc");
        // Invalid selector (probably)
        assert_not_in_family("::abc:", "abc");
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
            let _obj: Id<AnyObject> = unsafe { msg_send_id![cls, new] };
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

    #[test]
    #[cfg_attr(
        not(all(feature = "apple", target_os = "macos", target_arch = "x86")),
        ignore = "Only relevant on macOS 32-bit"
    )]
    fn ensure_size_of_module_info() {
        assert_eq!(core::mem::size_of::<ModuleInfo>(), 16);
    }
}
