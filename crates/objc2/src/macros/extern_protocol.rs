/// TODO
#[doc(alias = "@protocol")]
#[macro_export]
macro_rules! extern_protocol {
    (
        $(#[$m:meta])*
        $v:vis struct $name:ident;

        unsafe impl ProtocolType for $for:ty {
            $(const NAME: &'static str = $name_const:literal;)?

            $($methods:tt)*
        }
    ) => {
        $crate::__inner_extern_class!(
            @__inner

            $(#[$m])*
            $v struct ($name) {}

            unsafe impl () for $for {
                INHERITS = [$crate::runtime::Object];
            }
        );

        // SAFETY: TODO
        unsafe impl ProtocolType for $for {
            const NAME: &'static str = $crate::__select_name!($name; $($name_const)?);
            const __INNER: () = ();
        }

        impl $for {
            $crate::__extern_protocol_rewrite_methods! {
                $($methods)*
            }
        }

        const _: () = {
            if $crate::__macro_helpers::size_of::<$name>() != 0 {
                panic!(concat!(
                    "the struct ",
                    stringify!($name),
                    " is not zero-sized!",
                ))
            }
        };
    };
}

/// tt-munch each protocol method.
#[doc(hidden)]
#[macro_export]
macro_rules! __extern_protocol_rewrite_methods {
    // Base case
    {} => {};

    // Unsafe variant
    {
        $(#[$($m:tt)*])*
        $v:vis unsafe fn $name:ident($($args:tt)*) $(-> $ret:ty)?;

        $($rest:tt)*
    } => {
        $crate::__rewrite_self_arg! {
            ($crate::__extern_protocol_method_out)
            ($($args)*)
            @($(#[$($m)*])*)
            @($v unsafe fn $name($($args)*) $(-> $ret)?)
            // Macro will add:
            // @(kind)
            // @(args_start)
            // @(args_rest)
        }

        $crate::__extern_protocol_rewrite_methods! {
            $($rest)*
        }
    };

    // Safe variant
    {
        $(#[$($m:tt)*])*
        $v:vis fn $name:ident($($args:tt)*) $(-> $ret:ty)?;

        $($rest:tt)*
    } => {
        $crate::__rewrite_self_arg! {
            ($crate::__extern_protocol_method_out)
            ($($args)*)
            @($(#[$($m)*])*)
            @($v fn $name($($args)*) $(-> $ret)?)
            // Macro will add:
            // @(kind)
            // @(args_start)
            // @(args_rest)
        }

        $crate::__extern_protocol_rewrite_methods! {
            $($rest)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_protocol_method_out {
    {
        @($(#[$($m:tt)*])*)
        @($($function_start:tt)*)
        @(instance_method)
        @(
            $self_or_this:ident: $self_or_this_ty:ty,
            _: $sel_ty:ty,
        )
        @($($args_rest:tt)*)
    } => {
        $crate::__strip_custom_attributes! {
            @($(#[$($m)*])*)
            @($($function_start)* {
                #[allow(unused_unsafe)]
                unsafe {
                    $crate::__extract_custom_attributes! {
                        @($(#[$($m)*])*)
                        @($crate::__extern_protocol_method_body)
                        @(
                            @($self_or_this)
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
        @($(#[$($m:tt)*])*)
        @($($function_start:tt)*)
        @(class_method)
        @($($args_start:tt)*)
        @($($args_rest:tt)*)
    } => {
        compile_error!("class methods are not supported in `extern_protocol!`");
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_protocol_method_body {
    {
        @($($receiver:tt)*)
        @($($args_rest:tt)*)
        @(#[method($($sel:tt)*)])
        @($($m_optional:tt)*)
    } => {
        $crate::__collect_msg_send! {
            $crate::msg_send;
            $($receiver)*;
            ($($sel)*);
            ($($args_rest)*);
        }
    };
    {
        @($($receiver:tt)*)
        @($($args_rest:tt)*)
        @(#[method_id($($sel:tt)*)])
        @($($m_optional:tt)*)
    } => {
        $crate::__collect_msg_send! {
            $crate::msg_send_id;
            $($receiver)*;
            ($($sel)*);
            ($($args_rest)*);
        }
    };
}
