#[doc(hidden)]
#[macro_export]
macro_rules! __msg_send_parse {
    // No arguments
    {
        ($out_macro:path)
        @($error_fn:ident)
        // Intentionally empty
        @()
        @()
        @($selector:ident $(,)?)
        $($macro_args:tt)*
    } => {
        $crate::__msg_send_parse! {
            ($out_macro)
            @($error_fn)
            @($selector)
            @()
            @()
            $($macro_args)*
        }
    };

    // tt-munch remaining `selector: argument` pairs, looking for a pattern
    // that ends with `sel: _`.
    {
        ($out_macro:path)
        @($_error_fn:ident)
        @($($selector_output:tt)*)
        @($($argument_output:tt)*)
        @()
        $($macro_args:tt)*
    } => ({
        $out_macro! {
            $($macro_args)*
            @($($selector_output)*)
            @($($argument_output)*)
        }
    });
    {
        ($out_macro:path)
        @($error_fn:ident)
        @($($selector_output:tt)*)
        @($($argument_output:tt)*)
        @($selector:ident: _ $(,)?)
        @($fn:ident)
        $($macro_args:tt)*
    } => {
        $crate::__msg_send_parse! {
            ($out_macro)
            @($error_fn)
            @($($selector_output)* $selector:)
            // Don't pass an argument
            @($($argument_output)*)
            @()

            // Instead, we change the called function to the error function.
            @($error_fn)
            $($macro_args)*
        }
    };
    {
        ($out_macro:path)
        @($error_fn:ident)
        @($($selector_output:tt)*)
        @($($argument_output:tt)*)
        @($selector:ident : $argument:expr $(, $($rest:tt)*)?)
        $($macro_args:tt)*
    } => {
        $crate::__msg_send_parse! {
            ($out_macro)
            @($error_fn)
            @($($selector_output)* $selector:)
            @($($argument_output)* $argument,)
            @($($($rest)*)?)
            $($macro_args)*
        }
    };

    // Handle calls without comma between `selector: argument` pair.
    {
        ($out_macro:path)
        @($error_fn:ident)
        // Intentionally empty
        @()
        @()
        @($($selector:ident : $argument:expr)*)
        $($macro_args:tt)*
    } => {{
        $crate::__comma_between_args!(
            @($(
                ", ",
                stringify!($selector),
                ": ",
                stringify!($argument),
            )+)
            $($macro_args)*
        );

        $crate::__msg_send_parse! {
            ($out_macro)
            @($error_fn)
            @()
            @()
            @($($selector : $argument),*)
            $($macro_args)*
        }
    }};
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "unstable-msg-send-always-comma"))]
macro_rules! __comma_between_args {
    ($($args:tt)*) => {};
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "unstable-msg-send-always-comma")]
macro_rules! __comma_between_args {
    (
        @__output
        @($($args:tt)*)
        @($macro_name:literal)
    ) => {
        #[deprecated = concat!(
            "using ", $macro_name, "! without a comma between arguments is ",
            "technically not valid macro syntax, and may break in a future ",
            "version of Rust. You should use the following instead:\n",
            $macro_name, "![", $($args)* "]"
        )]
        #[inline]
        fn __msg_send_missing_comma() {}
        __msg_send_missing_comma();
    };
    (
        @($($args:tt)*)
        @(__send_super_message_static)
        @($obj:expr)
    ) => {
        $crate::__comma_between_args! {
            @__output
            @(stringify!(super($obj)), $($args)*)
            @("msg_send")
        }
    };
    (
        @($($args:tt)*)
        @(send_super_message)
        @($obj:expr, $superclass:expr)
    ) => {
        $crate::__comma_between_args! {
            @__output
            @(stringify!(super($obj, $superclass)), $($args)*)
            @("msg_send")
        }
    };
    (
        @($($args:tt)*)
        @(send_message)
        @($obj:expr)
    ) => {
        $crate::__comma_between_args! {
            @__output
            @(stringify!($obj), $($args)*)
            @("msg_send")
        }
    };
    // Catch-all for msg_send_id!
    (
        @($($args:tt)*)
        @($fn:ident)
        @($obj:expr)
        @()
    ) => {
        $crate::__comma_between_args! {
            @__output
            @(stringify!($obj), $($args)*)
            @("msg_send_id")
        }
    };
}
