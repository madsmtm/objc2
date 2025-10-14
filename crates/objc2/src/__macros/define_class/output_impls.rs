//! Output code for the function itself.

#[doc(hidden)]
#[macro_export]
macro_rules! __define_class_output_impls {
    // Base-case
    () => {};

    // With protocol
    (
        $(#[$m:meta])*
        unsafe impl $protocol:ident for $for:ty {
            $($methods:tt)*
        }

        $($rest:tt)*
    ) => {
        // SAFETY: Upheld by caller
        $(#[$m])*
        unsafe impl $protocol for $for {}

        $(#[$m])*
        impl $for {
            $crate::__define_class_output_methods! {
                $($methods)*
            }
        }

        $crate::__define_class_output_impls!{
            $($rest)*
        }
    };

    // Without protocol
    (
        $(#[$m:meta])*
        impl $for:ty {
            $($methods:tt)*
        }

        $($rest:tt)*
    ) => {
        $(#[$m])*
        impl $for {
            $crate::__define_class_output_methods! {
                $($methods)*
            }
        }

        $crate::__define_class_output_impls! {
            $($rest)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_class_output_methods {
    // Base case
    {} => {};

    // Unsafe variant
    {
        $(#[$($m:tt)*])*
        unsafe fn $name:ident($($params:tt)*) $(-> $ret:ty)? $body:block

        $($rest:tt)*
    } => {
        $crate::__extract_method_attributes! {
            ($(#[$($m)*])*)

            ($crate::__rewrite_self_param)
            ($($params)*)

            ($crate::__define_class_method_out)
            (unsafe)
            ($name)
            ($($ret)?)
            ($body)
        }

        $crate::__define_class_output_methods! {
            $($rest)*
        }
    };

    // Safe variant
    {
        $(#[$($m:tt)*])*
        fn $name:ident($($params:tt)*) $(-> $ret:ty)? $body:block

        $($rest:tt)*
    } => {
        $crate::__extract_method_attributes! {
            ($(#[$($m)*])*)

            ($crate::__rewrite_self_param)
            ($($params)*)

            ($crate::__define_class_method_out)
            ()
            ($name)
            ($($ret)?)
            ($body)
        }

        $crate::__define_class_output_methods! {
            $($rest)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_class_method_out {
    {
        ($($qualifiers:tt)*)
        ($name:ident)
        ($($ret:ty)?)
        ($body:block)

        ($($m_method:tt)*)
        ($($method_family:tt)*)
        ($($optional:tt)*)
        ($($attr_method:tt)*)
        ($($attr_use:tt)*)

        ($builder_method:ident)
        ($receiver:expr)
        ($receiver_ty:ty)
        ($($params_prefix:tt)*)
        ($($params_rest:tt)*)
    } => {
        $crate::__define_class_rewrite_params! {
            ($($params_rest)*)
            ()
            ()

            ($crate::__define_class_method_out_inner)

            ($($qualifiers)*)
            ($name)
            ($($ret)?)
            ($body)

            ($builder_method)
            ($receiver)
            ($receiver_ty)
            ($($params_prefix)*)

            ($($m_method)*)
            ($($method_family)*)
            ($($optional)*)
            ($($attr_method)*)
            ($($attr_use)*)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_class_rewrite_params {
    // Convert _
    {
        (_ : $param_ty:ty $(, $($params_rest:tt)*)?)
        ($($params_converted:tt)*)
        ($($body_prefix:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__define_class_rewrite_params! {
            ($($($params_rest)*)?)
            ($($params_converted)* _ : <$param_ty as $crate::__macros::ConvertArgument>::__Inner,)
            ($($body_prefix)*)

            ($out_macro)
            $($macro_args)*
        }
    };
    // Convert mut
    {
        (mut $param:ident : $param_ty:ty $(, $($params_rest:tt)*)?)
        ($($params_converted:tt)*)
        ($($body_prefix:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__define_class_rewrite_params! {
            ($($($params_rest)*)?)
            ($($params_converted)* $param : <$param_ty as $crate::__macros::ConvertArgument>::__Inner,)
            (
                $($body_prefix)*
                let mut $param = <$param_ty as $crate::__macros::ConvertArgument>::__from_defined_param($param);
            )

            ($out_macro)
            $($macro_args)*
        }
    };
    // Convert
    {
        ($param:ident : $param_ty:ty $(, $($params_rest:tt)*)?)
        ($($params_converted:tt)*)
        ($($body_prefix:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__define_class_rewrite_params! {
            ($($($params_rest)*)?)
            ($($params_converted)* $param : <$param_ty as $crate::__macros::ConvertArgument>::__Inner,)
            (
                $($body_prefix)*
                let $param = <$param_ty as $crate::__macros::ConvertArgument>::__from_defined_param($param);
            )

            ($out_macro)
            $($macro_args)*
        }
    };
    // Output result
    {
        ()
        ($($params_converted:tt)*)
        ($($body_prefix:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*

            ($($params_converted)*)
            ($($body_prefix)*)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_class_method_out_inner {
    // #[unsafe(method(...))]
    {
        ($($qualifiers:tt)*)
        ($name:ident)
        ($($ret:ty)?)
        ($body:block)

        ($__builder_method:ident)
        ($__receiver:expr)
        ($__receiver_ty:ty)
        ($($params_prefix:tt)*)

        (method($($__sel:tt)*))
        ($($method_family:tt)*)
        ($($optional:tt)*)
        ($($attr_method:tt)*)
        ($($attr_use:tt)*)

        ($($params_converted:tt)*)
        ($($body_prefix:tt)*)
    } => {
        $($attr_method)*
        #[allow(clippy::diverging_sub_expression)]
        $($qualifiers)* extern "C-unwind" fn $name(
            $($params_prefix)*
            $($params_converted)*
        ) $(-> <$ret as $crate::__macros::ConvertReturn<()>>::Inner)? {
            $crate::__define_class_no_method_family!($($method_family)*);
            $($body_prefix)*
            $crate::__convert_result! {
                $body $(; $ret)?
            }
        }
    };

    // #[unsafe(method_id(...))]
    {
        ($($qualifiers:tt)*)
        ($name:ident)
        ($ret:ty)
        ($body:block)

        ($__builder_method:ident)
        ($__receiver:expr)
        ($receiver_ty:ty)
        ($($params_prefix:tt)*)

        (method_id($($sel:tt)*))
        ($($method_family:tt)*)
        ($($optional:tt)*)
        ($($attr_method:tt)*)
        ($($attr_use:tt)*)

        ($($params_converted:tt)*)
        ($($body_prefix:tt)*)
    } => {
        $($attr_method)*
        #[allow(clippy::diverging_sub_expression)]
        $($qualifiers)* extern "C-unwind" fn $name(
            $($params_prefix)*
            $($params_converted)*
        ) -> $crate::__macros::RetainedReturnValue {
            // TODO: Somehow tell the compiler that `this: Allocated<Self>` is non-null.

            $($body_prefix)*

            let __objc2_result = $body;

            #[allow(unreachable_code)]
            <$crate::__method_family!(($($method_family)*) ($($sel)*)) as $crate::__macros::MessageReceiveRetained<
                $receiver_ty,
                $ret,
            >>::into_return(__objc2_result)
        }
    };

    {
        ($($qualifiers:tt)*)
        ($name:ident)
        ()
        ($body:block)

        ($__builder_method:ident)
        ($__receiver:expr)
        ($__receiver_ty:ty)
        ($($params_prefix:tt)*)

        (method_id($($sel:tt)*))
        ($($method_family:tt)*)
        ($($optional:tt)*)
        ($($attr_method:tt)*)
        ($($attr_use:tt)*)

        ($($params_converted:tt)*)
        ($($body_prefix:tt)*)
    } => {
        $($attr_method)*
        $($qualifiers)* extern "C-unwind" fn $name() {
            $crate::__macros::compile_error!("`#[unsafe(method_id(...))]` must have a return type")
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __convert_result {
    ($body:block) => {
        $body
    };
    ($body:block; $ret:ty) => {
        let __objc2_result = $body;
        #[allow(unreachable_code)]
        <$ret as $crate::__macros::ConvertReturn<()>>::convert_defined_return(__objc2_result)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_class_no_method_family {
    () => {};
    ($($t:tt)+) => {
        $crate::__macros::compile_error!(
            "`#[unsafe(method_family = ...)]` is not yet supported in `define_class!` together with `#[unsafe(method(...))]`"
        )
    };
}
