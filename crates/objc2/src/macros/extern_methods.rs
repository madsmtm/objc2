/// Define methods on an external class.
///
/// This is a convenience macro to easily generate associated functions and
/// methods that call [`msg_send!`] or [`msg_send_id!`] appropriately.
///
/// [`msg_send!`]: crate::msg_send
/// [`msg_send_id!`]: crate::msg_send_id
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
/// The desired selector can be specified using the `#[method(my:selector:)]`
/// or `#[method_id(my:selector:)]` attribute. The `method` attribute maps to
/// a call to [`msg_send!`], while the `method_id` maps to [`msg_send_id!`].
///
/// If the attribute ends with "_", as in `#[method(my:error:_)]` or
/// `#[method_id(my:error:_)]`, the method is assumed to take an
/// implicit `NSError**` parameter, which is automatically converted to a
/// [`Result`]. See the error section in [`msg_send!`] and [`msg_send_id!`]
/// for details.
///
/// Putting other attributes on the method such as `cfg`, `allow`, `doc`,
/// `deprecated` and so on is supported. However, note that `cfg_attr` may not
/// work correctly, due to implementation difficulty - if you have a concrete
/// use-case, please [open an issue], then we can discuss it.
///
/// The name of the function doesn't matter for out purposes, but is of course
/// what the user will use to access the functionality.
///
/// If you specify a function/method with a body, the macro will simply ignore
/// it.
///
/// ["associated functions"]: https://doc.rust-lang.org/reference/items/associated-items.html#methods
/// ["methods"]: https://doc.rust-lang.org/reference/items/associated-items.html#methods
/// [open an issue]: https://github.com/madsmtm/objc2/issues/new
///
///
/// # Safety
///
/// You must ensure that any methods you declare with the `#[method(...)]`
/// attribute upholds the safety guarantees decribed in the [`msg_send!`]
/// macro, _or_ are marked `unsafe`.
///
/// Likewise, you must ensure that any methods you declare with the
/// `#[method_id(...)]` attribute upholds the safety guarantees decribed in
/// the [`msg_send_id!`] macro, _or_ are marked `unsafe`.
///
///
/// # Examples
///
/// Let's create a quick custom class:
///
/// ```
/// use objc2::encode::{Encode, Encoding};
/// use objc2::ffi::NSUInteger;
/// use objc2::rc::{Allocated, Id};
/// use objc2::runtime::NSObject;
/// use objc2::{declare_class, extern_methods, mutability, ClassType};
///
/// // Shim
/// type NSError = NSObject;
///
/// declare_class!(
///     pub struct MyObject;
///
///     unsafe impl ClassType for MyObject {
///         type Super = NSObject;
///         type Mutability = mutability::Immutable;
///         const NAME: &'static str = "MyObject";
///     }
///
///     unsafe impl MyObject {
///         // ... Assume we've implemented all the methods used below
///     }
/// );
///
/// extern_methods!(
///     /// Creation methods.
///     unsafe impl MyObject {
///         #[method_id(new)]
///         pub fn new() -> Id<Self>;
///
///         #[method_id(initWithVal:)]
///         // Arbitary self types are not stable, but we can work around it
///         // with the special name `this`.
///         pub fn init(this: Option<Allocated<Self>>, val: usize) -> Id<Self>;
///     }
///
///     /// Instance accessor methods.
///     unsafe impl MyObject {
///         #[method(foo)]
///         pub fn foo(&self) -> NSUInteger;
///
///         #[method_id(fooObject)]
///         pub fn foo_object(&self) -> Id<NSObject>;
///
///         #[method(withError:_)]
///         // Since the selector specifies "_", the return type is assumed to
///         // be `Result`.
///         pub fn with_error(&self) -> Result<(), Id<NSError>>;
///     }
/// );
/// ```
///
/// The `extern_methods!` declaration then becomes:
///
/// ```
/// # use objc2::encode::{Encode, Encoding};
/// # use objc2::ffi::NSUInteger;
/// # use objc2::rc::{Allocated, Id};
/// # use objc2::runtime::NSObject;
/// # use objc2::{declare_class, extern_methods, mutability, ClassType};
/// #
/// # // Shim
/// # type NSError = NSObject;
/// #
/// # declare_class!(
/// #     pub struct MyObject;
/// #
/// #     unsafe impl ClassType for MyObject {
/// #         type Super = NSObject;
/// #         type Mutability = mutability::InteriorMutable;
/// #         const NAME: &'static str = "MyObject2";
/// #     }
/// #
/// #     unsafe impl MyObject {
/// #         // ... Assume we've implemented all the methods used below
/// #     }
/// # );
/// #
/// use objc2::{msg_send, msg_send_id};
///
/// /// Creation methods.
/// impl MyObject {
///     pub fn new() -> Id<Self> {
///         unsafe { msg_send_id![Self::class(), new] }
///     }
///
///     pub fn init(this: Option<Allocated<Self>>, val: usize) -> Id<Self> {
///         unsafe { msg_send_id![this, initWithVal: val] }
///     }
/// }
///
/// /// Instance accessor methods.
/// impl MyObject {
///     pub fn foo(&self) -> NSUInteger {
///         unsafe { msg_send![self, foo] }
///     }
///
///     pub fn foo_object(&self) -> Id<NSObject> {
///         unsafe { msg_send_id![self, fooObject] }
///     }
///
///     // Since the selector specifies one more argument than we
///     // have, the return type is assumed to be `Result`.
///     pub fn with_error(&self) -> Result<(), Id<NSError>> {
///         unsafe { msg_send![self, withError: _] }
///     }
/// }
/// ```
///
/// See the source code of `icrate` for many more examples.
#[macro_export]
macro_rules! extern_methods {
    // Generic impls
    (
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

    // Non-generic impls
    (
        $(
            $(#[$impl_m:meta])*
            unsafe impl $type:ty {
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
        $v:vis unsafe fn $name:ident($($args:tt)*) $(-> $ret:ty)?
        // TODO: Handle where bounds better
        $(where $($where:ty : $bound:path),+ $(,)?)?;

        $($rest:tt)*
    } => {
        $crate::__rewrite_self_arg! {
            ($($args)*)

            ($crate::__extract_custom_attributes)
            ($(#[$($m)*])*)
            ($name)

            ($crate::__extern_methods_method_out)
            ($v unsafe fn $name($($args)*) $(-> $ret)?)
            ($($($where : $bound ,)+)?)
        }

        $crate::__extern_methods_rewrite_methods! {
            $($rest)*
        }
    };

    // Safe variant
    {
        $(#[$($m:tt)*])*
        $v:vis fn $name:ident($($args:tt)*) $(-> $ret:ty)?
        // TODO: Handle where bounds better
        $(where $($where:ty : $bound:path),+ $(,)?)?;

        $($rest:tt)*
    } => {
        $crate::__rewrite_self_arg! {
            ($($args)*)

            ($crate::__extract_custom_attributes)
            ($(#[$($m)*])*)
            ($name)

            ($crate::__extern_methods_method_out)
            ($v fn $name($($args)*) $(-> $ret)?)
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
    // #[method(...)]
    {
        ($($function_start:tt)*)
        ($($where:ty : $bound:path ,)*)

        ($__builder_method:ident)
        ($receiver:expr)
        ($__receiver_ty:ty)
        ($($__args_prefix:tt)*)
        ($($args_rest:tt)*)

        (#[method($($sel:tt)*)])
        () // No `optional`
        ($($m_checked:tt)*)
    } => {
        $($m_checked)*
        $($function_start)*
        where
            $($where : $bound,)*
        {
            #[allow(unused_unsafe)]
            unsafe {
                $crate::__method_msg_send! {
                    ($receiver)
                    ($($sel)*)
                    ($($args_rest)*)

                    ()
                    ()
                }
            }
        }
    };

    // #[method_id(...)]
    {
        ($($function_start:tt)*)
        ($($where:ty : $bound:path ,)*)

        ($__builder_method:ident)
        ($receiver:expr)
        ($__receiver_ty:ty)
        ($($__args_prefix:tt)*)
        ($($args_rest:tt)*)

        (#[method_id($($sel:tt)*)])
        () // No `optional`
        ($($m_checked:tt)*)
    } => {
        $($m_checked)*
        $($function_start)*
        where
            $($where : $bound,)*
        {
            #[allow(unused_unsafe)]
            unsafe {
                $crate::__method_msg_send_id! {
                    ($receiver)
                    ($($sel)*)
                    ($($args_rest)*)

                    ()
                    ()
                    ()
                }
            }
        }
    };

    // #[optional]
    {
        ($($function_start:tt)*)
        ($($where:ty : $bound:path ,)*)

        ($__builder_method:ident)
        ($receiver:expr)
        ($__receiver_ty:ty)
        ($($__args_prefix:tt)*)
        ($($args_rest:tt)*)

        ($($m_method:tt)*)
        ($($m_optional:tt)*)
        ($($m_checked:tt)*)
    } => {
        $($m_checked)*
        $($function_start)* {
            compile_error!("`#[optional]` is only supported in `extern_protocol!`")
        }
    };
}
