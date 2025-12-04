mod null_error;
mod parse;
mod retained;

pub use self::retained::*;

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
/// # use std::ffi::{c_int, c_char};
/// # use objc2::msg_send;
/// # use objc2::runtime::NSObject;
/// unsafe fn do_something(obj: &NSObject, arg: c_int) -> *const c_char {
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
/// The [`extern_methods!`] macro can help with coding this pattern.
///
/// [`extern_methods!`]: crate::extern_methods
///
///
/// # Memory management
///
/// If an Objective-C method returns `id`, `NSObject*`, or similar object
/// pointers, you should use [`Retained<T>`] on the Rust side, or
/// `Option<Retained<T>>` if the pointer is nullable.
///
/// This is necessary because object pointers in Objective-C have certain
/// rules for when they should be retained and released across function calls.
///
/// [`Retained<T>`]: crate::rc::Retained
///
///
/// ## A little history
///
/// Objective-C's type system is... limited, so you can't tell without
/// consulting the documentation who is responsible for releasing an object.
/// To remedy this problem, Apple/Cocoa introduced (approximately) the
/// following rule:
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
/// Returning a `*const T` pointer is similar to pre-ARC; you have to know
/// when to retain and when to release an object. Returning `Retained` is
/// similar to ARC; the rules are simple enough that we can do them
/// automatically!
///
/// [mmRules]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/MemoryMgmt/Articles/mmRules.html#//apple_ref/doc/uid/20000994-SW1
/// [arc-rel]: https://developer.apple.com/library/archive/releasenotes/ObjectiveC/RN-TransitioningToARC/Introduction/Introduction.html#//apple_ref/doc/uid/TP40011226
///
///
/// # Specification
///
/// The syntax is somewhat similar to the message syntax in Objective-C,
/// except with a comma between arguments. Eliding the comma is possible, but
/// deprecated, and may be removed in a future version of `objc2`.
///
/// The first expression, know as the "receiver", can be any type that
/// implements [`MessageReceiver`], like a reference or a pointer to an
/// object. Additionally, it can even be a reference to an [`Retained`]
/// containing an object.
///
/// The expression can be wrapped in `super`, with an optional superclass
/// as the second argument. If no specific superclass is specified, the
/// direct superclass is retrieved from [`ClassType`].
///
/// All arguments, as well as the return type, must implement [`Encode`] (bar
/// the exceptions below).
///
/// If the last argument is the special marker `_`, the macro will return a
/// `Result<_, Retained<E>>`, see below.
///
/// This macro roughly translates into a call to [`sel!`], and afterwards a
/// fully qualified call to [`MessageReceiver::send_message`]. Note that this
/// means that auto-dereferencing of the receiver is not supported, and that
/// the receiver is consumed. You may encounter a little trouble with `&mut`
/// references, try refactoring into a separate method or reborrowing the
/// reference.
///
/// Variadic arguments are currently not supported.
///
/// [`MessageReceiver`]: crate::runtime::MessageReceiver
/// [`Retained`]: crate::rc::Retained
/// [`ClassType`]: crate::ClassType
/// [`Encode`]: crate::Encode
/// [`sel!`]: crate::sel
/// [`MessageReceiver::send_message`]: crate::runtime::MessageReceiver::send_message
///
///
/// ## Memory management details
///
/// The accepted receiver and return types, and how we handle them, differ
/// depending on which, if any, of the [recognized selector
/// families][sel-families] the selector belongs to:
///
/// - The `new` family: The receiver may be anything that implements
///   [`MessageReceiver`] (though often you'll want to use `&AnyClass`). The
///   return type is a generic `Retained<T>` or `Option<Retained<T>>`.
///
/// - The `alloc` family: The receiver must be `&AnyClass`, and the return
///   type is a generic `Allocated<T>`.
///
/// - The `init` family: The receiver must be `Allocated<T>` as returned from
///   `alloc`, or if sending messages to the superclass, it must be
///   `PartialInit<T>`.
///
///   The receiver is consumed, and a the now-initialized `Retained<T>` or
///   `Option<Retained<T>>` (with the same `T`) is returned.
///
/// - The `copy` family: The receiver may be anything that implements
///   [`MessageReceiver`] and the return type is a generic `Retained<T>` or
///   `Option<Retained<T>>`.
///
/// - The `mutableCopy` family: Same as the `copy` family.
///
/// - No family: The receiver may be anything that implements
///   [`MessageReceiver`]. The result is retained using
///   [`Retained::retain_autoreleased`], and a generic `Retained<T>` or
///   `Option<Retained<T>>` is returned. This retain is in most cases faster
///   than using autorelease pools!
///
/// See [the clang documentation][arc-retainable] for the precise
/// specification of Objective-C's ownership rules.
///
/// As you may have noticed, the return type is usually either `Retained` or
/// `Option<Retained>`. Internally, the return type is always
/// `Option<Retained>` (for example: almost all `new` methods can fail if the
/// allocation failed), but for convenience, if the return type is
/// `Retained<T>`, this macro will automatically unwrap the object, or panic
/// with an error message if it couldn't be retrieved.
///
/// As a special case, if the last argument is the marker `_`, the macro will
/// return a `Result<Retained<T>, Retained<E>>`, see below.
///
/// The `retain`, `release` and `autorelease` selectors are not supported, use
/// [`Retained::retain`], [`Retained::drop`] and [`Retained::autorelease_ptr`]
/// for that.
///
/// [sel-families]: https://clang.llvm.org/docs/AutomaticReferenceCounting.html#arc-method-families
/// [`MessageReceiver`]: crate::runtime::MessageReceiver
/// [`Retained::retain_autoreleased`]: crate::rc::Retained::retain_autoreleased
/// [arc-retainable]: https://clang.llvm.org/docs/AutomaticReferenceCounting.html#retainable-object-pointers-as-operands-and-arguments
/// [`Retained::retain`]: crate::rc::Retained::retain
/// [`Retained::drop`]: crate::rc::Retained::drop
/// [`Retained::autorelease_ptr`]: crate::rc::Retained::autorelease_ptr
///
///
/// # `bool` handling
///
/// Objective-C's `BOOL` is slightly different from Rust's [`bool`], and hence
/// a conversion step must be performed before using it. This is _very_ easy
/// to forget (because it'll happen to work in _most_ cases), so this macro
/// does the conversion step automatically whenever an argument or the return
/// type is `bool`.
///
/// That means that any Objective-C method that take or return `BOOL` can be
/// translated to use `bool` on the Rust side.
///
/// If you want to handle the conversion explicitly, or the Objective-C method
/// expects e.g. a pointer to a `BOOL`, use [`runtime::Bool`] instead.
///
/// [`runtime::Bool`]: crate::runtime::Bool
///
///
/// # `CStr` handling
///
/// Since the ABI of [`CStr`][std::ffi::CStr] isn't yet stable, `&CStr` and
/// `Option<&CStr>` are converted to/from `*const c_char`.
///
///
/// # Out-parameters
///
/// Parameters like `NSString**` in Objective-C are passed by "writeback",
/// which means that the callee autoreleases any value that they may write
/// into the parameter.
///
/// This macro has support for passing such parameters using the following
/// types:
/// - `&mut Retained<_>`
/// - `Option<&mut Retained<_>>`
/// - `&mut Option<Retained<_>>`,
/// - `Option<&mut Option<Retained<_>>>`
///
/// Beware with the first two, since they will cause undefined behaviour if
/// the method overwrites the value with `nil`.
///
/// See [clang's documentation][clang-out-params] for more details.
///
/// [clang-out-params]: https://clang.llvm.org/docs/AutomaticReferenceCounting.html#passing-to-an-out-parameter-by-writeback
///
///
/// # Errors
///
/// The most common place you'll see out-parameters is as `NSError**` the last
/// parameter, which is used to communicate errors to the caller, see [Error
/// Handling Programming Guide For Cocoa][cocoa-error].
///
/// Similar to Swift's [importing of error parameters][swift-error], this
/// macro supports an even more convenient version than the out-parameter
/// support, which transforms methods whose last parameter is `NSError**` into
/// the Rust equivalent, the [`Result`] type.
///
/// In particular, if you make the last argument the special marker `_`, then
/// the macro will return a `Result<R, Retained<E>>`. The error type `E` must
/// be either [`NSObject`] or `objc2_foundation::NSError`.
///
/// The success type `R` must be either `()` or `Retained<T>`.
///
/// At runtime, we create the temporary error variable for you on the stack
/// and send it as the out-parameter to the method. If the method then returns
/// `NO`/`false`, or in the case of an object pointer, `NULL`, the error
/// variable is loaded and returned in [`Err`].
///
/// [cocoa-error]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ErrorHandlingCocoa/ErrorHandling/ErrorHandling.html
/// [swift-error]: https://developer.apple.com/documentation/swift/about-imported-cocoa-error-parameters
/// [`NSObject`]: crate::runtime::NSObject
///
///
/// # Panics
///
/// Unwinds if the underlying method throws and exception. If the
/// `"catch-all"` Cargo feature is enabled, the Objective-C exception is
/// converted into a Rust panic, with potentially a bit better stack trace.
///
/// Finally, panics if the return type is specified as `Retained<_>`, but the
/// method actually returned NULL. If this happens, you should change the
/// signature to instead return `Option<Retained<_>>` to handle the error
/// yourself.
///
///
/// ## Type verification
///
/// To make message sending safer, all arguments and return values for
/// messages must implement [`encode::Encode`]. This allows the Rust compiler
/// to prevent you from passing e.g. a [`Vec`] into Objective-C, which would
/// both be UB and leak the vector.
///
/// When `debug_assertions` are enabled, this macro will check the encoding of
/// the given arguments and return every time you send a message, and will
/// panic if they are not equivalent.
///
/// This is not a perfect solution for ensuring safety (some Rust types have
/// the same Objective-C encoding, but are not equivalent, such as `&T` and
/// `*const T`), but it gets us much closer to it!
///
/// This behaviour can be tweaked with the `"relax-void-encoding"`,
/// `"relax-sign-encoding"` or `"disable-encoding-assertions"` Cargo feature
/// flags if it is causing you trouble.
///
/// [`encode::Encode`]: crate::encode::Encode
/// [`Vec`]: std::vec::Vec
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
///    (except if the variable is inside [`std::cell::UnsafeCell`] or
///    derivatives).
///
/// 5. If the receiver is a raw pointer it must be valid (aligned,
///    dereferenceable, initialized and so on). Messages to `null` pointers
///    are allowed (though heavily discouraged), but _only_ if the return type
///    itself is a pointer.
///
/// 6. You must uphold any additional safety requirements (explicit and
///    implicit) that the method has. For example:
///    - Methods that take pointers usually require that the pointer is valid,
///      and sometimes non-null.
///    - Sometimes, a method may only be called on the main thread.
///    - The lifetime of returned pointers usually follows certain rules, and
///      may not be valid outside of an [`autoreleasepool`] (returning
///      `Retained` usually helps with these cases).
///
/// 7. Each out-parameter must have the correct nullability, and the method
///    must not have any attributes that changes the how it handles memory
///    management for these.
///
/// 8. If using the automatic memory management facilities of this macro, the
///    method must not have any attributes such as `objc_method_family`,
///    `ns_returns_retained`, `ns_consumed` that changes the how it handles
///    memory management.
///
/// 8. TODO: Maybe more?
///
/// [`autoreleasepool`]: crate::rc::autoreleasepool
///
///
/// # Examples
///
/// Interacting with [`NSURLComponents`], [`NSString`] and [`NSNumber`].
///
/// [`NSURLComponents`]: https://developer.apple.com/documentation/foundation/nsurlcomponents?language=objc
/// [`NSString`]: https://developer.apple.com/documentation/foundation/nsstring?language=objc
/// [`NSNumber`]: https://developer.apple.com/documentation/foundation/nsnumber?language=objc
///
/// ```
/// use objc2::rc::Retained;
/// use objc2::{msg_send, ClassType};
/// use objc2::runtime::NSObject;
///
/// // Declare stub types for the three classes we'll be working with.
/// objc2::extern_class!(
///     #[unsafe(super(NSObject))] // + NSValue
///     struct NSNumber;
/// );
/// objc2::extern_class!(
///     #[unsafe(super(NSObject))]
///     struct NSURLComponents;
/// );
/// objc2::extern_class!(
///     #[unsafe(super(NSObject))]
///     struct NSString;
/// );
///
///
/// // Create an empty `NSURLComponents` by calling the class method `new`.
/// let components: Retained<NSURLComponents> = unsafe {
///     //          ^^^^^^^^^^^^^^^^^^^^^^^^^ the return type, a memory-managed
///     //                                    `NSURLComponents` instance
///     //
///     msg_send![NSURLComponents::class(), new]
///     //        ------------------------  ^^^ the selector `new`
///     //        |
///     //        the receiver, in this case the class itself
/// };
///
///
/// // Create a new `NSNumber` from an integer.
/// let port: Retained<NSNumber> = unsafe {
///     msg_send![NSNumber::class(), numberWithInt: 8080i32]
///     //                           -------------- ^^^^^^^ the argument to the method
///     //                           |
///     //                           the selector `numberWithInt:`
///     //
///     // Note how we must fully specify the argument as `8080i32` instead of just `8080`.
/// };
///
///
/// // Set the port property of the URL.
/// let _: () = unsafe { msg_send![&components, setPort: &*port] };
/// //     --                                   -------- ^^^^^^ the port is deref'd to
/// //     |                                    |               become the correct type
/// //     |                                    |
/// //     |                                    the selector `setPort:` is derived
/// //     |                                    from the property name `port`.
/// //     |
/// //     return type (i.e. nothing / void)
/// //
/// // Note that even return types of `void` must be explicitly specified as `()`.
///
///
/// // Set the `host` property of the URL.
/// let host: Retained<NSString> = unsafe {
///     msg_send![NSString::class(), stringWithUTF8String: c"example.com".as_ptr()]
/// };
/// let _: () = unsafe { msg_send![&components, setHost: &*host] };
///
///
/// // Set the `scheme` property of the URL.
/// let scheme: Retained<NSString> = unsafe {
///     msg_send![NSString::class(), stringWithUTF8String: c"http".as_ptr()]
/// };
/// let _: () = unsafe { msg_send![&components, setScheme: &*scheme] };
///
///
/// // Get the combined URL in string form.
/// let string: Option<Retained<NSString>> = unsafe { msg_send![&components, string] };
/// //          ^^^^^^ the method can return NULL, so we specify an option here
/// let string = string.unwrap();
///
/// // Convert the `NSString` to a Rust string.
/// // NOTE: This has issues with interior NUL characters.
/// // `objc2_foundation::NSString` handles this correctly.
/// let bytes: *const std::ffi::c_char = unsafe { msg_send![&string, UTF8String] };
/// let c_str = unsafe { std::ffi::CStr::from_ptr(bytes) };
///
/// assert_eq!(c_str, c"http://example.com:8080");
/// ```
///
/// The example above uses only `msg_send!` for demonstration purposes; note
/// that usually the interface you seek is already present in [the framework
/// crates] and then the equivalent code can be as simple as:
///
/// [the framework crates]: crate::topics::frameworks_list
///
/// ```ignore
/// use objc2_foundation::{NSNumber, NSString, NSURLComponents};
///
/// let components = NSURLComponents::new();
/// components.setPort(Some(&NSNumber::new_i32(8080)));
/// components.setHost(Some(&NSString::from_str("example.com")));
/// components.setScheme(Some(&NSString::from_str("http")));
/// let string = components.string();
///
/// assert_eq!(string.unwrap().to_string(), "http://example.com:8080");
/// ```
///
/// Sending messages to the superclass of an object.
///
/// ```no_run
/// use objc2::runtime::NSObject;
/// use objc2::{msg_send, ClassType};
/// #
/// # objc2::define_class!(
/// #     #[unsafe(super(NSObject))]
/// #     struct MyObject;
/// # );
/// #
/// # let obj: objc2::rc::Retained<MyObject> = todo!();
///
/// // Call `someMethod` on the direct super class.
/// let _: () = unsafe { msg_send![super(&obj), someMethod] };
///
/// // Or lower-level, a method on a specific superclass.
/// let superclass = NSObject::class();
/// let arg3: u32 = unsafe { msg_send![super(&obj, superclass), getArg3] };
/// ```
///
/// Sending a message with automatic error handling.
///
/// ```no_run
/// use objc2::msg_send;
/// use objc2::rc::Retained;
/// # #[cfg(requires_foundation)]
/// use objc2_foundation::{NSBundle, NSError};
/// # use objc2::runtime::NSObject as NSBundle;
/// # use objc2::runtime::NSObject as NSError;
///
/// # #[cfg(requires_foundation)]
/// let bundle = NSBundle::mainBundle();
/// # let bundle = NSBundle::new();
///
/// let res: Result<(), Retained<NSError>> = unsafe {
///     //          --  -------- ^^^^^^^ must be NSError or NSObject
///     //          |   |
///     //          |   always retained
///     //          |
///     //          `()` means that the method returns `bool`, we check
///     //          that and return success if `true`, an error if `false`
///     //
///     msg_send![&bundle, preflightAndReturnError: _]
///     //                                          ^ activate error handling
/// };
/// ```
///
/// Sending a message with an out parameter _and_ automatic error handling.
///
/// ```no_run
/// use objc2::msg_send;
/// use objc2::rc::Retained;
///
/// # type NSFileManager = objc2::runtime::NSObject;
/// # type NSURL = objc2::runtime::NSObject;
/// # type NSError = objc2::runtime::NSObject;
/// let obj: &NSFileManager;
/// # obj = todo!();
/// let url: &NSURL;
/// # url = todo!();
/// let mut result_url: Option<Retained<NSURL>> = None;
/// unsafe {
///     msg_send![
///         obj,
///         trashItemAtURL: url,
///         resultingItemURL: Some(&mut result_url),
///         error: _
///     ]?
/// //   ^ is possible on error-returning methods, if the return type is specified
/// };
///
/// // Use `result_url` here
///
/// # Ok::<(), Retained<NSError>>(())
/// ```
///
/// Attempt to do an invalid message send. This is undefined behaviour, but
/// will panic with `debug_assertions` enabled.
///
/// ```should_panic
/// use objc2::msg_send;
/// use objc2::runtime::NSObject;
///
/// let obj = NSObject::new();
///
/// // Wrong return type - this is UB!
/// //
/// // But it will be caught with `debug_assertions` enabled, stating that
/// // the return type's encoding is not correct.
/// let hash: f32 = unsafe { msg_send![&obj, hash] };
/// #
/// # panic!("does not panic in release mode, so for testing we make it!");
/// ```
#[macro_export]
macro_rules! msg_send {
    [super($obj:expr), $($selector_and_arguments:tt)+] => {
        $crate::__msg_send_parse! {
            ()
            ()
            ($($selector_and_arguments)+)

            (MsgSendSuperError::send_super_message_static_error)
            (MsgSendSuper::send_super_message_static)

            ($crate::__msg_send_helper)
            ($obj)
        }
    };
    [super($obj:expr, $superclass:expr), $($selector_and_arguments:tt)+] => {
        $crate::__msg_send_parse! {
            ()
            ()
            ($($selector_and_arguments)+)

            (MsgSendSuperError::send_super_message_error)
            (MsgSendSuper::send_super_message)

            ($crate::__msg_send_helper)
            ($obj, $superclass)
        }
    };
    [$obj:expr, $($selector_and_arguments:tt)+] => {
        $crate::__msg_send_parse! {
            ()
            ()
            ($($selector_and_arguments)+)

            (MsgSendError::send_message_error)
            (MsgSend::send_message)

            ($crate::__msg_send_helper)
            ($obj)
        }
    };
}

/// Use [`msg_send!`] instead, it now supports converting to/from `bool`.
#[macro_export]
#[deprecated = "use a normal msg_send! instead, it will perform the conversion for you"]
macro_rules! msg_send_bool {
    [$($msg_send_args:tt)+] => ({
        // Use old impl for backwards compat
        let result: $crate::runtime::Bool = $crate::msg_send![$($msg_send_args)+];
        result.as_bool()
    });
}

/// Use [`msg_send!`] instead, it now supports converting to/from
/// [`Retained`][crate::rc::Retained].
#[macro_export]
#[deprecated = "use a normal msg_send! instead, it will now perform the conversion to/from `Retained` for you"]
macro_rules! msg_send_id {
    [$($msg_send_args:tt)+] => {
        $crate::msg_send![$($msg_send_args)*]
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __msg_send_helper {
    {
        ($($fn_args:tt)+)
        ($trait:ident :: $fn:ident)
        ($($selector:tt)*)
        ($($argument:expr,)*)
    } => ({
        // Assign to intermediary variable for better UI, and to prevent
        // miscompilation on older Rust versions (TODO: Which ones?).
        //
        // Note: This can be accessed from any expression in `fn_args` and
        // `argument` - we won't (yet) bother with preventing that though.
        let result;

        // Always add trailing comma after each argument, so that we get a
        // 1-tuple if there is only one.
        //
        // And use `::<_, _>` for better UI.
        result = <$crate::__method_family!(() ($($selector)*)) as $crate::__macros::$trait<_, _>>::$fn(
            $($fn_args)+,
            $crate::sel!($($selector)*),
            ($($argument,)*),
        );
        result
    });
}
