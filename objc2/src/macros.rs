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
    ($name:ident) => ({
        static SEL: $crate::__CachedSel = $crate::__CachedSel::new();
        let name = concat!(stringify!($name), '\0');
        #[allow(unused_unsafe)]
        unsafe { SEL.get(name) }
    });
    ($($name:ident :)+) => ({
        static SEL: $crate::__CachedSel = $crate::__CachedSel::new();
        let name = concat!($(stringify!($name), ':'),+, '\0');
        #[allow(unused_unsafe)]
        unsafe { SEL.get(name) }
    });
}

/// Sends a message to an object or class.
///
/// The first argument can be any type that implements [`MessageReceiver`],
/// like a reference, a pointer, or an [`rc::Id`] to an object (where the
/// object implements [`Message`]).
///
/// In general this is wildly `unsafe`, even more so than sending messages in
/// Objective-C, because this macro doesn't know the expected types and
/// because Rust has more safety invariants to uphold. Make sure to review the
/// safety section below.
///
/// The syntax is similar to the message syntax in Objective-C.
///
/// Variadic arguments are not currently supported.
///
/// [`MessageReceiver`]: crate::MessageReceiver
/// [`Message`]: crate::Message
/// [`rc::Id`]: crate::rc::Id
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
/// The user must ensure that the selector is a valid method and is available
/// on the given receiver.
///
/// Since this macro can't inspect header files to see the expected types, it
/// is the users responsibility that the argument types and return type are
/// what the receiver excepts for this selector. A way of doing this is by
/// defining a wrapper function:
/// ```
/// # use std::os::raw::{c_int, c_char};
/// # use objc2::msg_send;
/// # use objc2::runtime::Object;
/// unsafe fn do_something(obj: &Object, arg: c_int) -> *const c_char {
///     msg_send![obj, doSomething: arg]
/// }
/// ```
///
/// The user must also uphold any safety requirements (explicit and implicit)
/// that the method has (e.g. methods that take pointers as an argument
/// usually require that the pointer is valid and often non-null).
///
/// Additionally, the call must not violate Rust's mutability rules, e.g. if
/// passing an `&T` the Objective-C method must not mutate the variable.
///
/// If the receiver is a raw pointer the user must ensure that it is valid
/// (aligned, dereferenceable, initialized and so on). Messages to `null`
/// pointers are allowed (though discouraged), but only if the return type
/// itself is a pointer.
///
/// Finally, the method must not (yet, see [RFC-2945]) throw an exception.
///
/// # Examples
///
/// ```no_run
/// # use objc2::msg_send;
/// # use objc2::runtime::Object;
/// let obj: *mut Object;
/// # let obj: *mut Object = 0 as *mut Object;
/// let description: *const Object = unsafe { msg_send![obj, description] };
/// let _: () = unsafe { msg_send![obj, setArg1: 1 arg2: 2] };
/// // Or with an optional comma between arguments:
/// let _: () = unsafe { msg_send![obj, setArg1: 1, arg2: 2] };
/// ```
///
/// [RFC-2945]: https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html
#[macro_export]
macro_rules! msg_send {
    (super($obj:expr, $superclass:expr), $name:ident) => ({
        let sel = $crate::sel!($name);
        let result;
        match $crate::MessageReceiver::send_super_message(&$obj, $superclass, sel, ()) {
            Err(s) => panic!("{}", s),
            Ok(r) => result = r,
        }
        result
    });
    (super($obj:expr, $superclass:expr), $($name:ident : $arg:expr $(,)?)+) => ({
        let sel = $crate::sel!($($name:)+);
        let result;
        match $crate::MessageReceiver::send_super_message(&$obj, $superclass, sel, ($($arg,)+)) {
            Err(s) => panic!("{}", s),
            Ok(r) => result = r,
        }
        result
    });
    ($obj:expr, $name:ident) => ({
        let sel = $crate::sel!($name);
        let result;
        match $crate::MessageReceiver::send_message(&$obj, sel, ()) {
            Err(s) => panic!("{}", s),
            Ok(r) => result = r,
        }
        result
    });
    ($obj:expr, $($name:ident : $arg:expr $(,)?)+) => ({
        let sel = $crate::sel!($($name:)+);
        let result;
        match $crate::MessageReceiver::send_message(&$obj, sel, ($($arg,)+)) {
            Err(s) => panic!("{}", s),
            Ok(r) => result = r,
        }
        result
    });
}
