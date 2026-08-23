/// Detect error vs. non-error method.
#[doc(hidden)]
#[macro_export]
macro_rules! __parse_sel {
    {
        // The selector data from inside `#[method(...)]`.
        ($($sel:tt)*)
        // The function's parameter patterns, excluding `self`.
        ($($param:tt)*)

        // The output macro.
        ($out_macro:path)
        $($macro_args:tt)*

        // The following arguments will be appended to the output macro:
        //
        // 1. Whether the method is an error method. One of `true` or `false`.
        //    ($is_error:expr)
        //
        // 2. The selector, without the `_` from error methods and with `::`
        //    replaced by `: :`.
        //    ($($sel:tt)*)
    } => {
        $crate::__parse_sel_inner! {
            ($($sel)*)
            ($($param)*)
            ()

            ($out_macro)
            $($macro_args)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_sel_inner {
    // Simple selector with no parameters.
    {
        ($sel:ident)
        ()
        ()

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*

            (false)
            ($sel)
        }
    };

    // Parse each selector/parameter pair.
    {
        ($($sel:ident)? : $($sel_rest:tt)*)
        ($_param:pat_param, $($params_rest:tt)*)
        ($($sel_parsed:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__parse_sel_inner! {
            ($($sel_rest)*)
            ($($params_rest)*)
            ($($sel_parsed)* $($sel)? :)

            ($out_macro)
            $($macro_args)*
        }
    };

    // Handle path separator token (`::` to `: :`).
    {
        ($($sel:ident)? :: $($sel_rest:tt)*)
        ($_param1:pat_param, $_param2:pat_param, $($params_rest:tt)*)
        ($($sel_parsed:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__parse_sel_inner! {
            ($($sel_rest)*)
            ($($params_rest)*)
            ($($sel_parsed)* $($sel)? : : )

            ($out_macro)
            $($macro_args)*
        }
    };

    // Normal return.
    {
        () // No more selector left to parse.
        () // And no more parameters.
        // Notice the "+" here; we must make sure we actually _did_ parse
        // a selector, and haven't just gotten an empty `#[method()]`.
        ($($sel_parsed:tt)+)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*

            (false)
            ($($sel_parsed)+)
        }
    };

    // Error return.
    {
        // `sel:_` without a corresponding parameter.
        ($($sel:ident)? : _)
        ()
        ($($sel_parsed:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*

            (true)
            ($($sel_parsed)* $($sel)? :)
        }
    };

    // Too many selector components.
    {
        ($($sel_rest:tt)+)
        ()
        ($($sel_parsed:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__macros::compile_error!(
            "number of arguments in function and selector did not match"
        )
    };

    // Too few selector components.
    {
        ($($sel:ident)?)
        ($($param:tt)+) // +
        ($($sel_parsed:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__macros::compile_error!(
            "number of arguments in function and selector did not match"
        )
    };

    // Could not parse.
    {
        ($($sel_rest:tt)*)
        ($($param:tt)*)
        ($($sel_parsed:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__macros::compile_error!(
            $crate::__macros::concat!(
                "failed parsing selector `",
                $crate::__macros::stringify!($($sel_rest)*),
                "`",
            )
        )
    };
}
