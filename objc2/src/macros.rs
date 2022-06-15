/// Gets a reference to a [`Class`] from the given name.
///
/// # Panics
///
/// Panics if no class with the given name can be found.
///
/// To check for a class that may not exist, use [`Class::get`].
///
/// [`Class`]: crate::runtime::Class
/// [`Class::get`]: crate::runtime::Class::get
///
/// # Examples
///
/// ```no_run
/// # use objc2::class;
/// let cls = class!(NSObject);
/// ```
#[macro_export]
macro_rules! class {
    ($name:ident) => {{
        static CLASS: $crate::__CachedClass = $crate::__CachedClass::new();
        let name = concat!(stringify!($name), '\0');
        #[allow(unused_unsafe)]
        let cls = unsafe { CLASS.get(name) };
        match cls {
            Some(cls) => cls,
            None => panic!("Class with name {} could not be found", stringify!($name)),
        }
    }};
}

/// Registers a selector with the Objective-C runtime.
///
/// Returns a [`Sel`].
///
/// [`Sel`]: crate::runtime::Sel
///
/// # Examples
///
/// ```
/// # use objc2::sel;
/// let sel = sel!(description);
/// let sel = sel!(setObject:forKey:);
/// ```
#[macro_export]
macro_rules! sel {
    ($first:ident $(: $($rest:ident :)*)?) => ({
        static SEL: $crate::__CachedSel = $crate::__CachedSel::new();
        let name = concat!(stringify!($first), $(':', $(stringify!($rest), ':',)*)? '\0');
        #[allow(unused_unsafe)]
        unsafe { SEL.get(name) }
    });
}

/// Send a message to an object or class.
///
/// This is wildly `unsafe`, even more so than sending messages in
/// Objective-C, because this macro can't inspect header files to see the
/// expected types, and because Rust has more safety invariants to uphold.
/// Make sure to review the safety section below!
///
/// The recommended way of using this macro is by defining a wrapper function:
///
/// ```
/// # use std::os::raw::{c_int, c_char};
/// # use objc2::msg_send;
/// # use objc2::runtime::Object;
/// unsafe fn do_something(obj: &Object, arg: c_int) -> *const c_char {
///     msg_send![obj, doSomething: arg]
/// }
/// ```
///
/// This way we are clearly communicating to Rust that: The method
/// `doSomething:` works with a shared reference to the object. It takes a
/// C-style signed integer, and returns a pointer to what is probably a
/// C-compatible string. Now it's much, _much_ easier to make a safe
/// abstraction around this!
///
/// There exists two variants of this macro, [`msg_send_bool!`] and
/// [`msg_send_id!`], which can help with upholding certain requirements of
/// methods that return respectively Objective-C's `BOOL` and `id` (or any
/// object pointer). Use those whenever you want to call such a method!
///
/// [`msg_send_bool!`]: crate::msg_send_bool
/// [`msg_send_id!`]: crate::msg_send_id
///
///
/// # Specification
///
/// The syntax is similar to the message syntax in Objective-C, except with
/// an (optional, though consider that deprecated) comma between arguments,
/// since that works much better with rustfmt.
///
/// The first expression, know as the "receiver", can be any type that
/// implements [`MessageReceiver`], like a reference or a pointer to an
/// object, or even a reference to an [`rc::Id`] containing an object.
///
/// All arguments, and the return type, must implement [`Encode`].
///
/// This macro translates into a call to [`sel!`], and afterwards a fully
/// qualified call to [`MessageReceiver::send_message`]. Note that this means
/// that auto-dereferencing of the receiver is not supported, and that the
/// receiver is consumed. You may encounter a little trouble with `&mut`
/// references, try refactoring into a separate method or reborrowing the
/// reference.
///
/// Variadic arguments are currently not supported.
///
/// [`MessageReceiver`]: crate::MessageReceiver
/// [`rc::Id`]: crate::rc::Id
/// [`Encode`]: crate::Encode
/// [`MessageReceiver::send_message`]: crate::MessageReceiver::send_message
///
///
/// # Panics
///
/// Panics if the `"catch_all"` feature is enabled and the Objective-C method
/// throws an exception. Exceptions may still cause UB until
/// `extern "C-unwind"` is stable, see [RFC-2945].
///
/// Panics if the `"verify_message"` feature is enabled and the Objective-C
/// method's argument's encoding does not match the encoding of the given
/// arguments. This is highly recommended to enable while testing!
///
/// [RFC-2945]: https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html
///
///
/// # Safety
///
/// Similar to defining and calling an `extern` function in a foreign function
/// interface. In particular, you must uphold the following requirements:
///
/// 1. The selector corresponds to a valid method that is available on the
///    receiver.
///
/// 2. The argument types match what the receiver excepts for this selector.
///
/// 3. The return type match what the receiver returns for this selector.
///
/// 4. The call must not violate Rust's mutability rules, for example if
///    passing an `&T`, the Objective-C method must not mutate the variable
///    (of course except if the variable is inside [`std::cell::UnsafeCell`]).
///
/// 5. If the receiver is a raw pointer it must be valid (aligned,
///    dereferenceable, initialized and so on). Messages to `null` pointers
///    are allowed (though heavily discouraged), but _only_ if the return type
///    itself is a pointer.
///
/// 6. The method must not (yet) throw an exception.
///
/// 7. You must uphold any additional safety requirements (explicit and
///    implicit) that the method has. For example:
///    - Methods that take pointers usually require that the pointer is valid,
///      and sometimes non-null.
///    - Sometimes, a method may only be called on the main thread.
///    - The lifetime of returned pointers usually follows certain rules, and
///      may not be valid outside of an [`autoreleasepool`] ([`msg_send_id!`]
///      can greatly help with that).
///
/// 8. TODO: Maybe more?
///
/// [`autoreleasepool`]: crate::rc::autoreleasepool
/// [`msg_send_id!`]: crate::msg_send_id
///
///
/// # Examples
///
/// ```no_run
/// # use objc2::msg_send;
/// # use objc2::runtime::Object;
/// let obj: *mut Object;
/// # let obj: *mut Object = 0 as *mut Object;
/// let description: *const Object = unsafe { msg_send![obj, description] };
/// // Usually you'd use msg_send_id here ^
/// let _: () = unsafe { msg_send![obj, setArg1: 1u32, arg2: 2i32] };
/// let arg1: i32 = unsafe { msg_send![obj, getArg1] };
/// let arg2: i32 = unsafe { msg_send![obj, getArg2] };
/// ```
#[macro_export]
macro_rules! msg_send {
    [super($obj:expr, $superclass:expr), $selector:ident $(,)?] => ({
        let sel = $crate::sel!($selector);
        let result;
        match $crate::MessageReceiver::send_super_message($obj, $superclass, sel, ()) {
            Err(s) => panic!("{}", s),
            Ok(r) => result = r,
        }
        result
    });
    [super($obj:expr, $superclass:expr), $($selector:ident : $argument:expr $(,)?)+] => ({
        let sel = $crate::sel!($($selector :)+);
        let result;
        match $crate::MessageReceiver::send_super_message($obj, $superclass, sel, ($($argument,)+)) {
            Err(s) => panic!("{}", s),
            Ok(r) => result = r,
        }
        result
    });
    [$obj:expr, $selector:ident $(,)?] => ({
        let sel = $crate::sel!($selector);
        let result;
        match $crate::MessageReceiver::send_message($obj, sel, ()) {
            Err(s) => panic!("{}", s),
            Ok(r) => result = r,
        }
        result
    });
    [$obj:expr, $($selector:ident : $argument:expr $(,)?)+] => ({
        let sel = $crate::sel!($($selector :)+);
        let result;
        match $crate::MessageReceiver::send_message($obj, sel, ($($argument,)+)) {
            Err(s) => panic!("{}", s),
            Ok(r) => result = r,
        }
        result
    });
}

/// [`msg_send!`] for methods returning `BOOL`.
///
/// Objective-C's `BOOL` is different from Rust's [`bool`], see
/// [`runtime::Bool`] for more information, so a conversion step must be
/// performed before using it - this can easily be forgotted using the
/// [`msg_send!`] macro, so this is a less error-prone version does the
/// conversion for you!
///
/// [`runtime::Bool`]: crate::runtime::Bool
///
///
/// # Specification
///
/// Equivalent to the following:
///
/// ```no_run
/// # use objc2::msg_send;
/// # use objc2::runtime::{Bool, Object};
/// # let obj: *mut Object = 0 as *mut Object;
/// # unsafe
/// {
///     let result: Bool = msg_send![obj, selector];
///     result.as_bool()
/// };
/// ```
///
///
/// # Safety
///
/// Same as [`msg_send!`], with the expected return type of `BOOL`.
///
///
/// # Examples
///
#[cfg_attr(feature = "apple", doc = "```")]
#[cfg_attr(not(feature = "apple"), doc = "```no_run")]
/// # use objc2::{class, msg_send_bool, msg_send_id};
/// # use objc2::rc::{Id, Owned};
/// # use objc2::runtime::Object;
/// let obj: Id<Object, Owned>;
/// # obj = unsafe { msg_send_id![class!(NSObject), new].unwrap() };
/// assert!(unsafe { msg_send_bool![&obj, isEqual: &*obj] });
/// ```
#[macro_export]
macro_rules! msg_send_bool {
    [$($msg_send_args:tt)+] => ({
        let result: $crate::runtime::Bool = $crate::msg_send![$($msg_send_args)+];
        result.as_bool()
    });
}

/// [`msg_send!`] for methods returning `id`, `NSObject*`, or similar object
/// pointers.
///
/// Object pointers in Objective-C have certain rules for when they should be
/// retained and released across function calls. This macro helps doing that,
/// and returns an [`Option`] (letting you handle failures) containing an
/// [`rc::Id`] with the object.
///
/// [`rc::Id`]: crate::rc::Id
///
///
/// # A little history
///
/// Objective-C's type system is... limited, so you can't easily tell who is
/// responsible for releasing an object. To remedy this problem, Apple/Cocoa
/// introduced approximately the following rule:
///
/// The caller is responsible for releasing objects return from methods that
/// begin with `new`, `alloc`, `copy`, `mutableCopy` or `init`, and method
/// that begins with `init` takes ownership of the receiver. See [Cocoa's
/// Memory Management Policy][mmRules] for a user-friendly introduction to
/// this concept.
///
/// In the past, users had to do `retain` and `release` calls themselves to
/// properly follow these rules. To avoid the memory management problems
/// associated with manual stuff like that, they [introduced "ARC"][arc-rel],
/// which codifies the rules as part of the language, and inserts the required
/// `retain` and `release` calls automatically.
///
/// [`msg_send!`] is similar to pre-ARC; you have to know when to retain and
/// when to release an object. [`msg_send_id!`] is similar to ARC; the rules
/// are simple enough that we can do them automatically!
///
/// [mmRules]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/MemoryMgmt/Articles/mmRules.html#//apple_ref/doc/uid/20000994-SW1
/// [arc-rel]: https://developer.apple.com/library/archive/releasenotes/ObjectiveC/RN-TransitioningToARC/Introduction/Introduction.html#//apple_ref/doc/uid/TP40011226
///
/// [`msg_send_id!`]: crate::msg_send_id
///
///
/// # Specification
///
/// The syntax is the same as in [`msg_send!`].
///
/// Attributes like `objc_method_family`, `ns_returns_retained`, `ns_consumed`
/// and so on must not present on the method - if they are, you should do
/// manual memory management using the [`msg_send!`] macro instead.
///
/// The accepted receiver and return types, and how we handle them, differ
/// depending on which, if any, of the [recognized selector
/// families][sel-families] the selector belongs to (here `T: Message` and
/// `O: Ownership`):
///
/// - The `new` family: The receiver must be `&Class`, and the return type
///   is a generic `Option<Id<T, O>>`.
///
/// - The `alloc` family: The receiver must be `&Class`, and the return type
///   is a generic `Option<Id<T, O>>`. (This will change, see [#172]).
///
/// - The `init` family: The receiver must be `Option<Id<T, O>>` as returned
///   from `alloc`. The receiver is consumed, and a the now-initialized
///   `Option<Id<T, O>>` (with the same `T` and `O`) is returned.
///
/// - The `copy` family: The receiver may be anything that implements
///   [`MessageReceiver`] and the return type is a generic `Option<Id<T, O>>`.
///
/// - The `mutableCopy` family: Same as the `copy` family.
///
/// - No family: The receiver may be anything that implements
///   [`MessageReceiver`]. The result is retained using
///   [`Id::retain_autoreleased`], and a generic `Option<Id<T, O>>` is
///   returned. This retain is in most cases faster than using autorelease
///   pools!
///
/// See [the clang documentation][arc-retainable] for the precise
/// specification of Objective-C's ownership rules.
///
/// This macro doesn't support super methods yet, see [#173].
/// The `retain`, `release` and `autorelease` selectors are not supported, use
/// [`Id::retain`], [`Id::drop`] and [`Id::autorelease`] for that.
///
/// [sel-families]: https://clang.llvm.org/docs/AutomaticReferenceCounting.html#arc-method-families
/// [#172]: https://github.com/madsmtm/objc2/pull/172
/// [`MessageReceiver`]: crate::MessageReceiver
/// [`Id::retain_autoreleased`]: crate::rc::Id::retain_autoreleased
/// [arc-retainable]: https://clang.llvm.org/docs/AutomaticReferenceCounting.html#retainable-object-pointers-as-operands-and-arguments
/// [#173]: https://github.com/madsmtm/objc2/pull/173
/// [`Id::retain`]: crate::rc::Id::retain
/// [`Id::drop`]: crate::rc::Id::drop
/// [`Id::autorelease`]: crate::rc::Id::autorelease
///
///
/// # Safety
///
/// Same as [`msg_send!`], with an expected return type of `id`,
/// `instancetype`, `NSObject*`, or other such object pointers. The method
/// must not have any attributes that changes the how it handles memory
/// management.
///
///
/// # Examples
///
/// ```no_run
/// use objc2::{class, msg_send, msg_send_bool, msg_send_id};
/// use objc2::ffi::NSUInteger;
/// use objc2::rc::{Id, Shared};
/// use objc2::runtime::Object;
// Allocate new object
/// let obj = unsafe { msg_send_id![class!(NSObject), alloc] };
/// // Consume the allocated object, return initialized object
/// let obj: Id<Object, Shared> = unsafe { msg_send_id![obj, init].unwrap() };
/// // Copy the object
/// let copy: Id<Object, Shared> = unsafe { msg_send_id![&obj, copy].unwrap() };
/// // Call ordinary selector that returns an object
/// let s: Id<Object, Shared> = unsafe { msg_send_id![&obj, description].unwrap() };
/// ```
#[macro_export]
macro_rules! msg_send_id {
    [super($obj:expr, $superclass:expr), $selector:ident $(,)?] => ({
        $crate::__msg_send_id_helper!(@verify $selector);
        let sel = $crate::sel!($selector);
        const NAME: &[u8] = stringify!($selector).as_bytes();
        $crate::__msg_send_id_helper!(@get_assert_consts NAME);
        let result: Option<$crate::rc::Id<_, _>>;
        match <RS as $crate::__macro_helpers::MsgSendSuperId<_, _>>::send_super_message_id($obj, $superclass, sel, ()) {
            Err(s) => panic!("{}", s),
            Ok(r) => result = r,
        }
        result
    });
    [super($obj:expr, $superclass:expr), $($selector:ident : $argument:expr),+ $(,)?] => ({
        let sel = $crate::sel!($($selector:)+);
        const NAME: &[u8] = concat!($(stringify!($selector), ':'),+).as_bytes();
        $crate::__msg_send_id_helper!(@get_assert_consts NAME);
        let result: Option<$crate::rc::Id<_, _>>;
        match <RS as $crate::__macro_helpers::MsgSendSuperId<_, _>>::send_super_message_id($obj, $superclass, sel, ($($argument,)+)) {
            Err(s) => panic!("{}", s),
            Ok(r) => result = r,
        }
        result
    });
    [$obj:expr, $selector:ident $(,)?] => ({
        $crate::__msg_send_id_helper!(@verify $selector);
        let sel = $crate::sel!($selector);
        const NAME: &[u8] = stringify!($selector).as_bytes();
        $crate::__msg_send_id_helper!(@get_assert_consts NAME);
        let result: Option<$crate::rc::Id<_, _>>;
        match <RS as $crate::__macro_helpers::MsgSendId<_, _>>::send_message_id($obj, sel, ()) {
            Err(s) => panic!("{}", s),
            Ok(r) => result = r,
        }
        result
    });
    [$obj:expr, $($selector:ident : $argument:expr),+ $(,)?] => ({
        let sel = $crate::sel!($($selector:)+);
        const NAME: &[u8] = concat!($(stringify!($selector), ':'),+).as_bytes();
        $crate::__msg_send_id_helper!(@get_assert_consts NAME);
        let result: Option<$crate::rc::Id<_, _>>;
        match <RS as $crate::__macro_helpers::MsgSendId<_, _>>::send_message_id($obj, sel, ($($argument,)+)) {
            Err(s) => panic!("{}", s),
            Ok(r) => result = r,
        }
        result
    });
}

/// Helper macro to avoid exposing these in the docs for [`msg_send_id!`].
#[doc(hidden)]
#[macro_export]
macro_rules! __msg_send_id_helper {
    (@verify retain) => {{
        $crate::__macro_helpers::compile_error!(
            "msg_send_id![obj, retain] is not supported. Use `Id::retain` instead"
        )
    }};
    (@verify release) => {{
        $crate::__macro_helpers::compile_error!(
            "msg_send_id![obj, release] is not supported. Drop an `Id` instead"
        )
    }};
    (@verify autorelease) => {{
        $crate::__macro_helpers::compile_error!(
            "msg_send_id![obj, autorelease] is not supported. Use `Id::autorelease`"
        )
    }};
    (@verify $selector:ident) => {{}};
    (@get_assert_consts $selector:ident) => {
        const NEW: bool = $crate::__macro_helpers::in_selector_family($selector, b"new");
        const ALLOC: bool = $crate::__macro_helpers::in_selector_family($selector, b"alloc");
        const INIT: bool = $crate::__macro_helpers::in_selector_family($selector, b"init");
        const COPY_OR_MUT_COPY: bool = {
            $crate::__macro_helpers::in_selector_family($selector, b"copy")
                || $crate::__macro_helpers::in_selector_family($selector, b"mutableCopy")
        };
        type RS = $crate::__macro_helpers::RetainSemantics<NEW, ALLOC, INIT, COPY_OR_MUT_COPY>;
    };
}
