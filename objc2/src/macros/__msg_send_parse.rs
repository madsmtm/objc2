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
    // TODO: Deprecate this
    {
        ($out_macro:path)
        @($error_fn:ident)
        // Intentionally empty
        @()
        @()
        @($($selector:ident : $argument:expr)*)
        $($macro_args:tt)*
    } => {
        $crate::__msg_send_parse! {
            ($out_macro)
            @($error_fn)
            @()
            @()
            @($($selector : $argument),*)
            $($macro_args)*
        }
    };
}
