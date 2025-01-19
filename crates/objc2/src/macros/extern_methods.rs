/// Define methods on an external class.
///
/// This is a convenience macro to generate associated functions and methods
/// that delegate to [`msg_send!`].
///
/// [`msg_send!`]: crate::msg_send
///
///
/// # Specification
///
/// Within the `impl` block you can define two types of functions without
/// bodies; ["associated functions"] and ["methods"]. These are then mapped to
/// the Objective-C equivalents "class methods" and "instance methods", and an
/// appropriate body is created for you. In particular, if you use `self` or
/// the special name `this` (or `_this`), your method will assumed to be an
/// instance method, and if you don't it will be assumed to be a class method.
///
/// If you specify a function/method with a body, the macro will output it
/// unchanged.
///
/// The name of the function will be used for the resulting function that the
/// user will use to access the functionality, but is otherwise not used by
/// the macro.
///
/// If you use `objc2::MainThreadMarker` as a parameter type, the macro will
/// ignore it, allowing you to neatly specify "this method must be run on the
/// main thread". Note that due to type-system limitations, this is currently
/// a textual match on `MainThreadMarker`; so you must use that exact
/// identifier.
///
/// ["associated functions"]: https://doc.rust-lang.org/reference/items/associated-items.html#methods
/// ["methods"]: https://doc.rust-lang.org/reference/items/associated-items.html#methods
///
///
/// ## Attributes
///
/// You can add most normal attributes to the methods, including
/// `#[cfg(...)]`, `#[allow(...)]`, `#[deprecated = ...]` and doc comments.
///
/// Exceptions and special attributes are noted below.
///
///
/// ### `#[method(...)]` (required)
///
/// Specify the desired selector using this attribute.
///
/// If the selector ends with "_", as in `#[method(my:error:_)]`, the method
/// is assumed to take an implicit `NSError**` parameter, which is
/// automatically converted to a [`Result`]. See the error section in
/// [`msg_send!`] for details.
///
///
/// ### `#[unsafe(method_family = ...)]` (optional)
///
/// The Cocoa memory management convention is figured out automatically based
/// on the name of the selector, but it can be overwritten with this `unsafe`
/// attribute.
///
/// This is commonly done in framework crates to improve compile-time
/// performance, as the logic to determine the family automatically can be
/// quite taxing at scale. That said, you should rarely need to use this
/// yourself.
///
/// The valid family names are:
/// - `alloc`.
/// - `new`.
/// - `init`.
/// - `copy`.
/// - `mutableCopy`.
///
/// As well as the special `none` family that opts-out of being in a family.
///
/// This corresponds to the `__attribute__((objc_method_family(family)))` C
/// attribute, see [Clang's documentation][clang-method-families].
///
/// [clang-method-families]: https://clang.llvm.org/docs/AutomaticReferenceCounting.html#method-families
///
///
/// #### Safety
///
/// You must ensure that the specified method family is correct.
///
///
/// ### `#[cfg_attr(..., ...)]`
///
/// This is only supported for attributes that apply to the method itself
/// (i.e. not supported for attributes that apply to any of the custom
/// attributes, due to implementation difficulty).
///
///
/// # Safety
///
/// You must ensure that any methods you declare with the `#[method(...)]`
/// attribute upholds the safety guarantees described in the [`msg_send!`]
/// macro, _or_ are marked `unsafe`.
///
///
/// # Examples
///
/// Let's create a quick custom class:
///
/// ```
/// use objc2::encode::{Encode, Encoding};
/// use objc2::ffi::NSUInteger;
/// use objc2::rc::{Allocated, Retained};
/// use objc2::runtime::NSObject;
/// use objc2::{define_class, extern_methods};
///
/// // Shim
/// type NSError = NSObject;
///
/// define_class!(
///     // SAFETY:
///     // - The superclass NSObject does not have any subclassing requirements.
///     // - `MyObject` does not implement `Drop`.
///     #[unsafe(super(NSObject))]
///     #[name = "MyObject"]
///     pub struct MyObject;
///
///     unsafe impl MyObject {
///         // ... Assume we've implemented all the methods used below
///     }
/// );
///
/// extern_methods!(
///     /// Creation methods.
///     unsafe impl MyObject {
///         #[method(new)]
///         pub fn new() -> Retained<Self>;
///
///         #[method(initWithVal:)]
///         // arbitrary self types are not stable, but we can work around it
///         // with the special name `this`.
///         pub fn init(this: Allocated<Self>, val: usize) -> Retained<Self>;
///     }
///
///     /// Instance accessor methods.
///     unsafe impl MyObject {
///         #[method(foo)]
///         pub fn foo(&self) -> NSUInteger;
///
///         #[method(fooObject)]
///         pub fn foo_object(&self) -> Retained<NSObject>;
///
///         #[method(withError:_)]
///         // Since the selector specifies "_", the return type is assumed to
///         // be `Result`.
///         pub fn with_error(&self) -> Result<(), Retained<NSError>>;
///     }
/// );
/// ```
///
/// The `extern_methods!` declaration then becomes:
///
/// ```
/// # use objc2::encode::{Encode, Encoding};
/// # use objc2::ffi::NSUInteger;
/// # use objc2::rc::{Allocated, Retained};
/// # use objc2::runtime::NSObject;
/// # use objc2::{define_class, extern_methods, ClassType};
/// #
/// # // Shim
/// # type NSError = NSObject;
/// #
/// # define_class!(
/// #     #[unsafe(super(NSObject))]
/// #     #[name = "MyObject2"]
/// #     pub struct MyObject;
/// #
/// #     unsafe impl MyObject {
/// #         // ... Assume we've implemented all the methods used below
/// #     }
/// # );
/// #
/// use objc2::msg_send;
///
/// /// Creation methods.
/// impl MyObject {
///     pub fn new() -> Retained<Self> {
///         unsafe { msg_send![Self::class(), new] }
///     }
///
///     pub fn init(this: Allocated<Self>, val: usize) -> Retained<Self> {
///         unsafe { msg_send![this, initWithVal: val] }
///     }
/// }
///
/// /// Instance accessor methods.
/// impl MyObject {
///     pub fn foo(&self) -> NSUInteger {
///         unsafe { msg_send![self, foo] }
///     }
///
///     pub fn foo_object(&self) -> Retained<NSObject> {
///         unsafe { msg_send![self, fooObject] }
///     }
///
///     // Since the selector specifies one more argument than we
///     // have, the return type is assumed to be `Result`.
///     pub fn with_error(&self) -> Result<(), Retained<NSError>> {
///         unsafe { msg_send![self, withError: _] }
///     }
/// }
/// ```
///
/// See the source code of `objc2-foundation` for many more examples.
#[macro_export]
macro_rules! extern_methods {
    (
        // Generic implementations.
        $(
            $(#[$impl_m:meta])*
            unsafe impl<$($t:ident $(: $b:ident $(+ $rest:ident)*)?),* $(,)?> $type:ty {
                $($methods:tt)*
            }
        )+
    ) => {
        $(
            $(#[$impl_m])*
            impl<$($t $(: $b $(+ $rest)*)?),*> $type {
                $crate::__extern_methods_rewrite_methods! {
                    $($methods)*
                }
            }
        )+
    };
    (
        // Non-generic implementations.
        $(
            $(#[$impl_m:meta])*
            unsafe impl $type:ty {
                // #[method($($selector)+)]
                // fn $fn_name($self, $($param: $Ty)*) -> $Return;
                $($methods:tt)*
            }
        )+
    ) => {
        $(
            $(#[$impl_m])*
            impl $type {
                $crate::__extern_methods_rewrite_methods! {
                    $($methods)*
                }
            }
        )+
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_methods_rewrite_methods {
    // Base case
    {} => {};

    // Unsafe variant
    {
        $(#[$($m:tt)*])*
        $v:vis unsafe fn $name:ident($($params:tt)*) $(-> $ret:ty)?
        // TODO: Handle where bounds better
        $(where $($where:ty : $bound:path),+ $(,)?)?;

        $($rest:tt)*
    } => {
        $crate::__rewrite_self_param! {
            ($($params)*)

            ($crate::__extract_method_attributes)
            ($(#[$($m)*])*)

            ($crate::__extern_methods_method_out)
            ($v unsafe fn $name($($params)*) $(-> $ret)?)
            ($($($where : $bound ,)+)?)
        }

        $crate::__extern_methods_rewrite_methods! {
            $($rest)*
        }
    };

    // Safe variant
    {
        $(#[$($m:tt)*])*
        $v:vis fn $name:ident($($params:tt)*) $(-> $ret:ty)?
        // TODO: Handle where bounds better
        $(where $($where:ty : $bound:path),+ $(,)?)?;

        $($rest:tt)*
    } => {
        $crate::__rewrite_self_param! {
            ($($params)*)

            ($crate::__extract_method_attributes)
            ($(#[$($m)*])*)

            ($crate::__extern_methods_method_out)
            ($v fn $name($($params)*) $(-> $ret)?)
            ($($($where : $bound ,)+)?)
        }

        $crate::__extern_methods_rewrite_methods! {
            $($rest)*
        }
    };

    // Other items that people might want to put here (e.g. functions with a
    // body).
    {
        $associated_item:item

        $($rest:tt)*
    } => {
        $associated_item

        $crate::__extern_methods_rewrite_methods! {
            $($rest)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_methods_method_out {
    {
        ($($function_start:tt)*)
        ($($where:ty : $bound:path ,)*)

        ($__builder_method:ident)
        ($receiver:expr)
        ($__receiver_ty:ty)
        ($($__params_prefix:tt)*)
        ($($params_rest:tt)*)

        (#[$method_or_method_id:ident($($sel:tt)*)])
        ($($method_family:tt)*)
        ($($optional:tt)*)
        ($($attr_method:tt)*)
        ($($attr_use:tt)*)
    } => {
        $($attr_method)*
        $($function_start)*
        where
            $($where : $bound,)*
        {
            $crate::__extern_methods_method_id_deprecated!($method_or_method_id($($sel)*));
            $crate::__extern_methods_no_optional!($($optional)*);

            #[allow(unused_unsafe)]
            unsafe {
                $crate::__method_msg_send! {
                    ($receiver)
                    ($($sel)*)
                    ($($params_rest)*)

                    ()
                    ()
                    ($($method_family)*)
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_methods_no_optional {
    () => {};
    (#[optional]) => {
        $crate::__macro_helpers::compile_error!(
            "`#[optional]` is only supported in `extern_protocol!`"
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_methods_method_id_deprecated {
    (method($($sel:tt)*)) => {};
    (method_id($($sel:tt)*)) => {{
        #[deprecated = $crate::__macro_helpers::concat!(
            "using #[method_id(",
            $crate::__macro_helpers::stringify!($($sel)*),
            ")] inside extern_methods! is deprecated.\nUse #[method(",
            $crate::__macro_helpers::stringify!($($sel)*),
            ")] instead",
        )]
        #[inline]
        fn method_id() {}
        method_id();
    }};
}
