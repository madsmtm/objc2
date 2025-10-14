//! Output code for registering the function with the Objective-C runtime.

#[doc(hidden)]
#[macro_export]
macro_rules! __define_class_register_impls {
    // Base-case
    (
        ($builder:ident)
    ) => {};

    // With protocol
    (
        ($builder:ident)

        $(#[$($m:tt)*])*
        unsafe impl $protocol:ident for $for:ty {
            $($methods:tt)*
        }

        $($rest:tt)*
    ) => {
        $crate::__extract_and_apply_cfg_attributes! {
            ($(#[$($m)*])*)

            // Implement protocol
            #[allow(unused_mut)]
            let mut __objc2_protocol_builder = $builder.add_protocol_methods::<dyn $protocol>();

            // In case the user's function is marked `deprecated`
            #[allow(deprecated)]
            // In case the user did not specify any methods
            #[allow(unused_unsafe)]
            // SAFETY: Upheld by caller
            unsafe {
                $crate::__define_class_register_methods! {
                    (__objc2_protocol_builder)

                    $($methods)*
                }
            }

            // Finished creating protocol; get error message if any
            __objc2_protocol_builder.finish();
        }

        $crate::__define_class_register_impls! {
            ($builder)
            $($rest)*
        }
    };

    // Without protocol
    (
        ($builder:ident)

        $(#[$($m:tt)*])*
        impl $for:ty {
            $($methods:tt)*
        }

        $($rest:tt)*
    ) => {
        $crate::__extract_and_apply_cfg_attributes! {
            ($(#[$($m)*])*)

            // In case the user's function is marked `deprecated`
            #[allow(deprecated)]
            // In case the user did not specify any methods
            #[allow(unused_unsafe)]
            // SAFETY: Upheld by caller
            unsafe {
                $crate::__define_class_register_methods! {
                    ($builder)

                    $($methods)*
                }
            }
        }

        $crate::__define_class_register_impls! {
            ($builder)
            $($rest)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_class_register_methods {
    // Base case
    {
        ($builder:ident)
    } => {};

    // Unsafe variant
    {
        ($builder:ident)

        $(#[$($m:tt)*])*
        unsafe fn $name:ident($($params:tt)*) $(-> $ret:ty)? $body:block

        $($rest:tt)*
    } => {
        $crate::__extract_method_attributes! {
            ($(#[$($m)*])*)

            ($crate::__rewrite_self_param)
            ($($params)*)

            ($crate::__define_class_register_out)
            ($builder)
            (unsafe)
            ($name)
        }

        $crate::__define_class_register_methods! {
            ($builder)

            $($rest)*
        }
    };

    // Safe variant
    {
        ($builder:ident)

        $(#[$($m:tt)*])*
        fn $name:ident($($params:tt)*) $(-> $ret:ty)? $body:block

        $($rest:tt)*
    } => {
        $crate::__extract_method_attributes! {
            ($(#[$($m)*])*)

            ($crate::__rewrite_self_param)
            ($($params)*)

            ($crate::__define_class_register_out)
            ($builder)
            ()
            ($name)
        }

        $crate::__define_class_register_methods! {
            ($builder)

            $($rest)*
        }
    };

    // Consume associated items for better UI.
    //
    // This will still fail inside __define_class_output_methods!
    {
        ($builder:ident)

        $_associated_item:item

        $($rest:tt)*
    } => {
        $crate::__define_class_register_methods! {
            ($builder)

            $($rest)*
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_class_register_out {
    {
        ($builder:ident)
        ($($qualifiers:tt)*)
        ($name:ident)

        ($method_or_method_id:ident($($sel:tt)*))
        ($($method_family:tt)*)
        ($($optional:tt)*)
        ($($attr_method:tt)*)
        ($($attr_use:tt)*)

        ($builder_method:ident)
        ($__receiver:expr)
        ($__receiver_ty:ty)
        ($($__params_prefix:tt)*)
        ($($params_rest:tt)*)
    } => {
        $($attr_use)*
        {
            $crate::__define_class_invalid_selectors!($method_or_method_id($($sel)*));
            $crate::__define_class_no_optional!($($optional)*);

            $builder.$builder_method(
                $crate::sel!($($sel)*),
                Self::$name as $crate::__fn_ptr! {
                    ($($qualifiers)*)
                    (_, _,)
                    $($params_rest)*
                },
            );
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_class_invalid_selectors {
    (method(dealloc)) => {
        $crate::__macros::compile_error!(
            "`#[unsafe(method(dealloc))]` is not supported. Implement `Drop` for the type instead"
        )
    };
    (method_id(dealloc)) => {
        $crate::__macros::compile_error!(
            "`#[unsafe(method_id(dealloc))]` is not supported. Implement `Drop` for the type instead"
        )
    };
    (method_id(alloc)) => {
        $crate::__macros::compile_error!($crate::__macros::concat!(
            "`#[unsafe(method_id(alloc))]` is not supported. ",
            "Use `#[unsafe(method(alloc))]` and do the memory management yourself",
        ))
    };
    (method_id(retain)) => {
        $crate::__macros::compile_error!($crate::__macros::concat!(
            "`#[unsafe(method_id(retain))]` is not supported. ",
            "Use `#[unsafe(method(retain))]` and do the memory management yourself",
        ))
    };
    (method_id(release)) => {
        $crate::__macros::compile_error!($crate::__macros::concat!(
            "`#[unsafe(method_id(release))]` is not supported. ",
            "Use `#[unsafe(method(release))]` and do the memory management yourself",
        ))
    };
    (method_id(autorelease)) => {
        $crate::__macros::compile_error!($crate::__macros::concat!(
            "`#[unsafe(method_id(autorelease))]` is not supported. ",
            "Use `#[unsafe(method(autorelease))]` and do the memory management yourself",
        ))
    };
    ($method_or_method_id:ident($($sel:tt)*)) => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_class_no_optional {
    () => {};
    (#[optional]) => {
        $crate::__macros::compile_error!("`#[optional]` is only supported in `extern_protocol!`")
    };
}

/// Create function pointer type with inferred parameters.
#[doc(hidden)]
#[macro_export]
macro_rules! __fn_ptr {
    (
        ($($qualifiers:tt)*)
        ($($output:tt)*)
        $(,)?
    ) => {
        $($qualifiers)* extern "C-unwind" fn($($output)*) -> _
    };
    (
        ($($qualifiers:tt)*)
        ($($output:tt)*)
        _ : $param_ty:ty $(, $($rest:tt)*)?
    ) => {
        $crate::__fn_ptr! {
            ($($qualifiers)*)
            ($($output)* _,)
            $($($rest)*)?
        }
    };
    (
        ($($qualifiers:tt)*)
        ($($output:tt)*)
        mut $param:ident : $param_ty:ty $(, $($rest:tt)*)?
    ) => {
        $crate::__fn_ptr! {
            ($($qualifiers)*)
            ($($output)* _,)
            $($($rest)*)?
        }
    };
    (
        ($($qualifiers:tt)*)
        ($($output:tt)*)
        $param:ident : $param_ty:ty $(, $($rest:tt)*)?
    ) => {
        $crate::__fn_ptr! {
            ($($qualifiers)*)
            ($($output)* _,)
            $($($rest)*)?
        }
    };
}
