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

/// Sends a message to an object or class.
///
/// This is wildly `unsafe`, even more so than sending messages in
/// Objective-C, because this macro doesn't know the expected types and
/// because Rust has more safety invariants to uphold. Make sure to review the
/// safety section below!
///
/// # General information
///
/// The syntax is similar to the message syntax in Objective-C, except we
/// allow an optional comma between arguments (works better with rustfmt).
///
/// The first argument (know as the "receiver") can be any type that
/// implements [`MessageReceiver`], like a reference or a pointer to an
/// object, or even a reference to an [`rc::Id`] containing an object.
/// Each subsequent argument must implement [`Encode`].
///
/// Behind the scenes this translates into a call to [`sel!`], and afterwards
/// a fully qualified call to [`MessageReceiver::send_message`] (note that
/// this means that auto-dereferencing of the receiver is not supported,
/// making the ergonomics when using this slightly worse).
///
/// Variadic arguments are not currently supported.
///
/// [`MessageReceiver`]: crate::MessageReceiver
/// [`rc::Id`]: crate::rc::Id
/// [`Encode`]: crate::Encode
/// [`MessageReceiver::send_message`]: crate::MessageReceiver::send_message
///
/// # Panics
///
/// Panics if the `catch_all` feature is enabled and the Objective-C method
/// throws an exception. Exceptions may however still cause UB until we get
/// `extern "C-unwind"`, see [RFC-2945].
///
/// And panics if the `verify_message` feature is enabled and the Objective-C
/// method's argument's encoding does not match the encoding of the given
/// arguments. This is highly recommended to enable while testing!
///
/// # Safety
///
/// This macro can't inspect header files to see the expected types, so it is
/// your responsibility that the selector exists on the receiver, and that the
/// argument types and return type are what the receiver excepts for this
/// selector - similar to defining an external function in FFI.
///
/// The recommended way of doing this is by defining a wrapper function:
/// ```
/// # use std::os::raw::{c_int, c_char};
/// # use objc2::msg_send;
/// # use objc2::runtime::Object;
/// unsafe fn do_something(obj: &Object, arg: c_int) -> *const c_char {
///     msg_send![obj, doSomething: arg]
/// }
/// ```
///
/// This way we are clearly communicating to Rust that this method takes an
/// immutable object, a C-integer, and returns a pointer to (probably) a
/// C-compatible string. Afterwards, it becomes fairly trivial to make a safe
/// abstraction around this.
///
/// In particular, you must uphold the following requirements:
///
/// 1. The selector is a valid method that is available on the given receiver.
///
/// 2. The types of the receiver and arguments must match what is expected on
///   the Objective-C side.
///
/// 3. The call must not violate Rust's mutability rules, e.g. if passing an
///   `&T`, the Objective-C method must not mutate the variable (this is true
///   for receivers as well).
///
/// 4. If the receiver is a raw pointer the user must ensure that it is valid
///   (aligned, dereferenceable, initialized and so on). Messages to `null`
///   pointers are allowed (though heavily discouraged), but only if the
///   return type itself is a pointer.
///
/// 5. The method must not (yet, see [RFC-2945]) throw an exception.
///
/// 6. You must uphold any additional safety requirements (explicit and
///   implicit) that the method has (for example, methods that take pointers
///   usually require that the pointer is valid, and sometimes non-null.
///   Another example, some methods may only be called on the main thread).
///
/// 7. TODO: Maybe more?
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
/// let _: () = unsafe { msg_send![obj, setArg1: 1 arg2: 2] };
/// // Or with an optional comma between arguments:
/// let _: () = unsafe { msg_send![obj, setArg1: 1, arg2: 2] };
/// ```
///
/// [RFC-2945]: https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html
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

/// A less error-prone version of [`msg_send!`] for methods returning `BOOL`.
///
/// Objective-C's `BOOL` is different from Rust's [`bool`] (see [`Bool`]), so
/// a conversion step must be performed before using it - this macro does that
/// for you!
///
/// [`Bool`]: crate::runtime::Bool
///
/// Equivalent to the following:
///
/// ```ignore
/// # use objc2::msg_send;
/// # use objc2::runtime::Bool;
/// # let obj: *mut Object = 0 as *mut Object;
/// {
///     let result: Bool = msg_send![obj, selector];
///     result.as_bool()
/// };
/// ```
///
/// # Examples
///
/// ```no_run
/// # use objc2::msg_send_bool;
/// # use objc2::runtime::Object;
/// # let obj: *mut Object = 0 as *mut Object;
/// assert!(unsafe { msg_send_bool![obj, isEqual: obj] });
/// ```
#[macro_export]
macro_rules! msg_send_bool {
    [$($msg_send_args:tt)+] => ({
        let result: $crate::runtime::Bool = $crate::msg_send![$($msg_send_args)+];
        result.as_bool()
    });
}

/// TODO
#[macro_export]
macro_rules! msg_send_id {
    [$obj:expr, $selector:ident $(,)?] => ({
        let sel = $crate::sel!($selector);
        const NAME: &[u8] = stringify!($selector).as_bytes();
        $crate::msg_send_id!(@__get_assert_consts NAME);
        use $crate::__macro_helpers::{MsgSendId, Assert};
        let result;
        match <Assert<ALLOC, INIT, RETAINED> as MsgSendId<_, _>>::send_message_id($obj, sel, ()) {
            Err(s) => panic!("{}", s),
            Ok(r) => result = r,
        }
        result
    });
    [$obj:expr, $($selector:ident : $argument:expr),+ $(,)?] => ({
        let sel = $crate::sel!($($selector:)+);
        const NAME: &[u8] = concat!($(stringify!($selector), ':'),+).as_bytes();
        $crate::msg_send_id!(@__get_assert_consts NAME);
        use $crate::__macro_helpers::{MsgSendId, Assert};
        let result;
        match <Assert<ALLOC, INIT, RETAINED> as MsgSendId<_, _>>::send_message_id($obj, sel, ($($argument,)+)) {
            Err(s) => panic!("{}", s),
            Ok(r) => result = r,
        }
        result
    });
    (@__get_assert_consts $name:ident) => {
        const ALLOC: bool = $crate::__macro_helpers::in_method_family($name, b"alloc");
        // https://clang.llvm.org/docs/AutomaticReferenceCounting.html#consumed-parameters
        const INIT: bool = $crate::__macro_helpers::in_method_family($name, b"init");
        // https://clang.llvm.org/docs/AutomaticReferenceCounting.html#retained-return-values
        const RETAINED: bool = {
            $crate::__macro_helpers::in_method_family($name, b"alloc")
                || $crate::__macro_helpers::in_method_family($name, b"new")
                || $crate::__macro_helpers::in_method_family($name, b"copy")
                || $crate::__macro_helpers::in_method_family($name, b"mutableCopy")
                || $crate::__macro_helpers::in_method_family($name, b"init")
        };
    };
}
