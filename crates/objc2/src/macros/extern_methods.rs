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
/// use objc2::rc::{Allocated, Id, Shared};
/// use objc2::runtime::NSObject;
/// use objc2::{declare_class, extern_methods, ClassType};
///
/// // Shim
/// type NSError = NSObject;
///
/// declare_class!(
///     pub struct MyObject {}
///
///     unsafe impl ClassType for MyObject {
///         type Super = NSObject;
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
///         pub fn new() -> Id<Self, Shared>;
///
///         #[method_id(initWithVal:)]
///         // Arbitary self types are not stable, but we can work around it
///         // with the special name `this`.
///         pub fn init(this: Option<Allocated<Self>>, val: usize) -> Id<Self, Shared>;
///     }
///
///     /// Instance accessor methods.
///     unsafe impl MyObject {
///         #[method(foo)]
///         pub fn foo(&self) -> NSUInteger;
///
///         #[method_id(fooObject)]
///         pub fn foo_object(&self) -> Id<NSObject, Shared>;
///
///         #[method(withError:)]
///         // Since the selector specifies one more argument than we
///         // have, the return type is assumed to be `Result`.
///         pub fn with_error(&self) -> Result<(), Id<NSError, Shared>>;
///     }
/// );
/// ```
///
/// The `extern_methods!` declaration then becomes:
///
/// ```
/// # use objc2::encode::{Encode, Encoding};
/// # use objc2::ffi::NSUInteger;
/// # use objc2::rc::{Allocated, Id, Shared};
/// # use objc2::runtime::NSObject;
/// # use objc2::{declare_class, extern_methods, ClassType};
/// #
/// # // Shim
/// # type NSError = NSObject;
/// #
/// # declare_class!(
/// #     pub struct MyObject {}
/// #
/// #     unsafe impl ClassType for MyObject {
/// #         type Super = NSObject;
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
///     pub fn new() -> Id<Self, Shared> {
///         unsafe { msg_send_id![Self::class(), new] }
///     }
///
///     pub fn init(this: Option<Allocated<Self>>, val: usize) -> Id<Self, Shared> {
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
///     pub fn foo_object(&self) -> Id<NSObject, Shared> {
///         unsafe { msg_send_id![self, fooObject] }
///     }
///
///     // Since the selector specifies one more argument than we
///     // have, the return type is assumed to be `Result`.
///     pub fn with_error(&self) -> Result<(), Id<NSError, Shared>> {
///         unsafe { msg_send![self, withError: _] }
///     }
/// }
/// ```
///
/// See the source code of `icrate` for many more examples.
#[macro_export]
macro_rules! extern_methods {
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
                $crate::__inner_extern_methods! {
                    @rewrite_methods
                    $($methods)*
                }
            }
        )+
    };
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
                $crate::__inner_extern_methods! {
                    @rewrite_methods
                    $($methods)*
                }
            }
        )+
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __inner_extern_methods {
    {@rewrite_methods} => {};
    {
        @rewrite_methods
        // Unsafe variant
        $(#[$($m:tt)*])*
        $v:vis unsafe fn $name:ident($($args:tt)*) $(-> $ret:ty)?;

        $($rest:tt)*
    } => {
        // Detect instance vs. class method.
        $crate::__rewrite_self_arg! {
            ($crate::__inner_extern_methods)
            ($($args)*)
            @method_out
            @($(#[$($m)*])*)
            @($v unsafe fn $name($($args)*) $(-> $ret)?)
            // Will add @(kind)
            // Will add @(args_start)
            // Will add @(args_rest)
        }

        $crate::__inner_extern_methods! {
            @rewrite_methods
            $($rest)*
        }
    };
    {
        @rewrite_methods
        // Safe variant
        $(#[$($m:tt)*])*
        $v:vis fn $name:ident($($args:tt)*) $(-> $ret:ty)?;

        $($rest:tt)*
    } => {
        $crate::__rewrite_self_arg! {
            ($crate::__inner_extern_methods)
            ($($args)*)
            @method_out
            @($(#[$($m)*])*)
            @($v fn $name($($args)*) $(-> $ret)?)
        }

        $crate::__inner_extern_methods! {
            @rewrite_methods
            $($rest)*
        }
    };
    {
        @rewrite_methods
        // Other items that people might want to put here (e.g. functions with
        // a body).
        $associated_item:item

        $($rest:tt)*
    } => {
        $associated_item

        $crate::__inner_extern_methods! {
            @rewrite_methods
            $($rest)*
        }
    };
    {
        @method_out
        @($(#[$($m:tt)*])*)
        @($($function_start:tt)*)
        @($($kind:tt)*)
        @($($args_start:tt)*)
        @($($args_rest:tt)*)
    } => {
        $crate::__strip_custom_attributes! {
            @($(#[$($m)*])*)
            @($($function_start)* {
                #[allow(unused_unsafe)]
                unsafe {
                    $crate::__extract_custom_attributes! {
                        @($(#[$($m)*])*)
                        @($crate::__inner_extern_methods)
                        @(
                            @unsafe_method_body
                            @($($kind)*)
                            @($($args_start)*)
                            @($($args_rest)*)
                            // Macro will add:
                            // @(method attribute)
                            // @(optional attribute)
                        )
                        @()
                        @()
                    }
                }
            })
            @()
        }
    };

    {
        @unsafe_method_body
        @($kind:ident)
        @($($args_start:tt)*)
        @($($args_rest:tt)*)
        @(#[method($($sel:tt)*)])
        @() // No `optional`
    } => {
        $crate::__collect_msg_send! {
            $crate::msg_send;
            $crate::__inner_extern_methods!(
                @get_receiver
                @($kind)
                @($($args_start)*)
            );
            ($($sel)*);
            ($($args_rest)*);
        }
    };
    {
        @unsafe_method_body
        @($kind:ident)
        @($($args_start:tt)*)
        @($($args_rest:tt)*)
        @(#[method_id($($sel:tt)*)])
        @() // No `optional`
    } => {
        $crate::__collect_msg_send! {
            $crate::msg_send_id;
            $crate::__inner_extern_methods!(
                @get_receiver
                @($kind)
                @($($args_start)*)
            );
            ($($sel)*);
            ($($args_rest)*);
        }
    };
    {
        @unsafe_method_body
        @($kind:ident)
        @($($args_start:tt)*)
        @($($args_rest:tt)*)
        @($($m_method:tt)*)
        @($($m_optional:tt)*)
    } => {
        compile_error!("`#[optional]` is only supported in `extern_protocol!`")
    };

    {
        @get_receiver
        @(instance_method)
        @(
            $self_or_this:ident: $self_or_this_ty:ty,
            _: $sel_ty:ty,
        )
    } => {
        $self_or_this
    };
    {
        @get_receiver
        @(class_method)
        @(
            _: $cls_ty:ty,
            _: $sel_ty:ty,
        )
    } => {
        Self::class()
    };
}

/// Zip selector and arguments, and forward to macro.
#[doc(hidden)]
#[macro_export]
macro_rules! __collect_msg_send {
    // Selector with no arguments
    (
        $macro:path;
        $obj:expr;
        ($(@__retain_semantics $retain_semantics:ident )? $sel:ident);
        ();
    ) => {{
        $macro![$obj, $(@__retain_semantics $retain_semantics )? $sel]
    }};

    // Base case
    (
        $macro:path;
        $obj:expr;
        ();
        ();
        $($output:tt)+
    ) => {{
        $macro![$obj, $($output)+]
    }};

    // Allow trailing `sel:` without a corresponding argument (for errors)
    (
        $macro:path;
        $obj:expr;
        ($(@__retain_semantics $retain_semantics:ident )? $sel:ident:);
        ($(,)?);
        $($output:tt)*
    ) => {
        $crate::__collect_msg_send! {
            $macro;
            $obj;
            ();
            ();
            $($output)* $(@__retain_semantics $retain_semantics )? $sel: _,
        }
    };

    // tt-munch each argument
    (
        $macro:path;
        $obj:expr;
        ($(@__retain_semantics $retain_semantics:ident )? $sel:ident : $($sel_rest:tt)*);
        ($arg:ident: $arg_ty:ty $(, $($args_rest:tt)*)?);
        $($output:tt)*
    ) => {
        $crate::__collect_msg_send! {
            $macro;
            $obj;
            ($($sel_rest)*);
            ($($($args_rest)*)?);
            $($output)* $(@__retain_semantics $retain_semantics )? $sel: $arg,
        }
    };

    // If couldn't zip selector and arguments, show useful error message
    ($($_any:tt)*) => {{
        compile_error!("Number of arguments in function and selector did not match!")
    }};
}
