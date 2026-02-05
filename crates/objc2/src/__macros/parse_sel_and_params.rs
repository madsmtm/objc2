/// Detect instance vs. class method and error vs. non-error method.
#[doc(hidden)]
#[macro_export]
macro_rules! __parse_sel_and_params {
    {
        // The selector data from inside `#[method(...)]`.
        ($($sel:tt)*)
        // The function's parameters.
        ($($params:tt)*)

        // The output macro.
        ($out_macro:path)
        $($macro_args:tt)*

        // The following arguments will be appended to the output macro:
        //
        // 1. One of:
        //    - `fn()`
        //    - `fn($receiver:ident: $receiver_ty:ty)`
        //    - `fn_result()`
        //    - `fn_result($receiver:ident: $receiver_ty:ty)`.
        //
        //    The receiver being set signifies that the method is an instance method.
        //
        //    Note that the receiver does not include any `mut` keywords that there
        //    might have been in front of the parameter (e.g. `mut self: &Self`).
        //    ($fn_or_fn_result:ident($($receiver:ident: $receiver_ty:ty)?))
        //
        // 2. The selector, without the `_` from error methods and with `::`
        //    replaced by `: :`.
        //    ($($sel:tt)*)
        //
        // 3. The parameter names.
        //
        //    This does not include any `mut` keywords that there might have been in
        //    front of the parameter, but does include any `_` patterns that the user
        //    might have set.
        //    ($($param:tt)*)
        //
        // 4. The parameter types.
        //    ($($param_ty:ty,)*)
        //
        // 5. Any parameters that should be ignored.
        //    ($($ignored_param:ty)*)
    } => {
        $crate::__parse_sel_and_params_self! {
            ($($sel)*)
            // Duplicate parameters so that we can match on `self`, while
            // still passing it on to another macro.
            ($($params)*)
            ($($params)*)

            ($out_macro)
            $($macro_args)*
        }
    };
}

/// Parse the `self` parameter.
#[doc(hidden)]
#[macro_export]
macro_rules! __parse_sel_and_params_self {
    // Instance method
    {
        ($($sel:tt)*)
        (&self $($_params_rest:tt)*)
        (&$self:ident $(, $($params_rest:tt)*)?)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__parse_sel_and_params_inner! {
            ($($sel)*)
            ($($($params_rest)*)?)

            ($self: &Self)
            ()
            ()
            ()
            ()

            ($out_macro)
            $($macro_args)*
        }
    };
    {
        ($($sel:tt)*)
        (&mut self $($_params_rest:tt)*)
        (&mut $self:ident $(, $($params_rest:tt)*)?)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__parse_sel_and_params_inner! {
            ($($sel)*)
            ($($($params_rest)*)?)

            ($self: &mut Self)
            ()
            ()
            ()
            ()

            ($out_macro)
            $($macro_args)*
        }
    };
    {
        ($($sel:tt)*)
        (self: $_self_ty:ty $(, $($_params_rest:tt)*)?)
        ($self:ident: $self_ty:ty $(, $($params_rest:tt)*)?)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__parse_sel_and_params_inner! {
            ($($sel)*)
            ($($($params_rest)*)?)

            ($self: $self_ty)
            ()
            ()
            ()
            ()

            ($out_macro)
            $($macro_args)*
        }
    };
    {
        ($($sel:tt)*)
        (mut self: $_self_ty:ty $(, $($_params_rest:tt)*)?)
        ($_mut:ident $self:ident: $self_ty:ty $(, $($params_rest:tt)*)?)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__parse_sel_and_params_inner! {
            ($($sel)*)
            ($($($params_rest)*)?)

            ($self: $self_ty)
            ()
            ()
            ()
            ()

            ($out_macro)
            $($macro_args)*
        }
    };

    // `this: Type` or `_this: Type` instance method
    // Workaround for arbitrary self types being unstable
    // https://doc.rust-lang.org/nightly/unstable-book/language-features/arbitrary-self-types.html
    {
        ($($sel:tt)*)
        (this: $_this_ty:ty $(, $($_params_rest:tt)*)?)
        ($this:ident: $this_ty:ty $(, $($params_rest:tt)*)?)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__parse_sel_and_params_inner! {
            ($($sel)*)
            ($($($params_rest)*)?)

            ($this: $this_ty)
            ()
            ()
            ()
            ()

            ($out_macro)
            $($macro_args)*
        }
    };
    {
        ($($sel:tt)*)
        (mut this: $_this_ty:ty $(, $($_params_rest:tt)*)?)
        ($_mut:ident $this:ident: $this_ty:ty $(, $($params_rest:tt)*)?)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__parse_sel_and_params_inner! {
            ($($sel)*)
            ($($($params_rest)*)?)

            ($this: $this_ty)
            ()
            ()
            ()
            ()

            ($out_macro)
            $($macro_args)*
        }
    };
    {
        ($($sel:tt)*)
        (_this: $_this_ty:ty $(, $($_params_rest:tt)*)?)
        ($this:ident: $this_ty:ty $(, $($params_rest:tt)*)?)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__parse_sel_and_params_inner! {
            ($($sel)*)
            ($($($params_rest)*)?)

            ($this: $this_ty)
            ()
            ()
            ()
            ()

            ($out_macro)
            $($macro_args)*
        }
    };
    {
        ($($sel:tt)*)
        (mut _this: $_this_ty:ty $(, $($_params_rest:tt)*)?)
        ($_mut:ident $this:ident: $this_ty:ty $(, $($params_rest:tt)*)?)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__parse_sel_and_params_inner! {
            ($($sel)*)
            ($($($params_rest)*)?)

            ($this: $this_ty)
            ()
            ()
            ()
            ()

            ($out_macro)
            $($macro_args)*
        }
    };

    // Class method
    //
    // This is intentionally placed last, since we only want to assume a class
    // method if none of the above succeeded.
    {
        ($($sel:tt)*)
        ($($_params_rest:tt)*)
        ($($params_rest:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__parse_sel_and_params_inner! {
            ($($sel)*)
            ($($params_rest)*)

            ()
            ()
            ()
            ()
            ()

            ($out_macro)
            $($macro_args)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_sel_and_params_inner {
    // Simple selector with no parameters.
    {
        ($sel:ident)
        ()

        ($($receiver_info:tt)*)
        ()
        ()
        ()
        ($($ignored_param:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*

            (fn($($receiver_info)*))
            ($sel)
            ()
            ()
            ($($ignored_param)*)
        }
    };

    // Skip using `MainThreadMarker` in the message send.
    //
    // This is a purely textual match, and using e.g.
    // `Foundation::MainThreadMarker` would fail - but that would just be
    // detected as giving a wrong number of arguments, so it's fine for now.
    (
        ($($sel_rest:tt)*)
        ($param:ident: MainThreadMarker $(, $($params_rest:tt)*)?)

        ($($receiver_info:tt)*)
        ($($sel_parsed:tt)*)
        ($($param_parsed:tt)*)
        ($($param_ty_parsed:tt)*)
        ($($ignored_param:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    ) => {
        $crate::__parse_sel_and_params_inner! {
            ($($sel_rest)*)
            ($($($params_rest)*)?)

            ($($receiver_info)*)
            ($($sel_parsed)*)
            ($($param_parsed)*)
            ($($param_ty_parsed)*)
            ($($ignored_param)* $param,)

            ($out_macro)
            $($macro_args)*
        }
    };

    // Parse each selector/parameter pair.
    //
    // We cannot just use `$param:pat_param`, since that cannot be followed
    // by a `:`, which we would need it to.
    {
        ($($sel:ident)? : $($sel_rest:tt)*)
        // `_` pattern.
        (_ : $param_ty:ty $(, $($params_rest:tt)*)?)

        ($($receiver_info:tt)*)
        ($($sel_parsed:tt)*)
        ($($param_parsed:tt)*)
        ($($param_ty_parsed:tt)*)
        ($($ignored_param:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__parse_sel_and_params_inner! {
            ($($sel_rest)*)
            ($($($params_rest)*)?)

            ($($receiver_info)*)
            ($($sel_parsed)* $($sel)? :)
            ($($param_parsed)* _,)
            ($($param_ty_parsed)* $param_ty,)
            ($($ignored_param)*)

            ($out_macro)
            $($macro_args)*
        }
    };
    {
        ($($sel:ident)? : $($sel_rest:tt)*)
        // `param` pattern.
        ($param:ident : $param_ty:ty $(, $($params_rest:tt)*)?)

        ($($receiver_info:tt)*)
        ($($sel_parsed:tt)*)
        ($($param_parsed:tt)*)
        ($($param_ty_parsed:tt)*)
        ($($ignored_param:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__parse_sel_and_params_inner! {
            ($($sel_rest)*)
            ($($($params_rest)*)?)

            ($($receiver_info)*)
            ($($sel_parsed)* $($sel)? :)
            ($($param_parsed)* $param,)
            ($($param_ty_parsed)* $param_ty,)
            ($($ignored_param)*)

            ($out_macro)
            $($macro_args)*
        }
    };
    {
        ($($sel:ident)? : $($sel_rest:tt)*)
        // `mut param` pattern.
        (mut $param:ident : $param_ty:ty $(, $($params_rest:tt)*)?)

        ($($receiver_info:tt)*)
        ($($sel_parsed:tt)*)
        ($($param_parsed:tt)*)
        ($($param_ty_parsed:tt)*)
        ($($ignored_param:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__parse_sel_and_params_inner! {
            ($($sel_rest)*)
            ($($($params_rest)*)?)

            ($($receiver_info)*)
            ($($sel_parsed)* $($sel)? :)
            ($($param_parsed)* $param,) // Intentionally don't add `mut` here.
            ($($param_ty_parsed)* $param_ty,)
            ($($ignored_param)*)

            ($out_macro)
            $($macro_args)*
        }
    };

    // Convert path separator token (`::` to `: :`).
    {
        ($($sel:ident)? :: $($sel_rest:tt)*)
        ($($params_rest:tt)*)

        ($($receiver_info:tt)*)
        ($($sel_parsed:tt)*)
        ($($param_parsed:tt)*)
        ($($param_ty_parsed:tt)*)
        ($($ignored_param:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__parse_sel_and_params_inner! {
            ($($sel)? : : $($sel_rest)*)
            ($($params_rest)*)

            ($($receiver_info)*)
            ($($sel_parsed)*)
            ($($param_parsed)*)
            ($($param_ty_parsed)*)
            ($($ignored_param)*)

            ($out_macro)
            $($macro_args)*
        }
    };

    // Normal return.
    {
        () // No more selector left to parse.
        () // And no more parameters.

        ($($receiver_info:tt)*)
        // Notice the "+" here; we must make sure we actually _did_ parse
        // a selector, and haven't just gotten an empty `#[method()]`.
        ($($sel_parsed:tt)+)
        ($($param_parsed:tt)*)
        ($($param_ty_parsed:tt)*)
        ($($ignored_param:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*

            (fn($($receiver_info)*))
            ($($sel_parsed)+)
            ($($param_parsed)*)
            ($($param_ty_parsed)*)
            ($($ignored_param)*)
        }
    };

    // Error return.
    {
        // `sel:_` without a corresponding parameter.
        ($($sel:ident)? : _)
        ()

        ($($receiver_info:tt)*)
        ($($sel_parsed:tt)*)
        ($($param_parsed:tt)*)
        ($($param_ty_parsed:tt)*)
        ($($ignored_param:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*

            (fn_result($($receiver_info)*))
            ($($sel_parsed)* $($sel)? :)
            ($($param_parsed)*)
            ($($param_ty_parsed)*)
            ($($ignored_param)*)
        }
    };

    // Variadic methods.
    {
        ($($sel_rest:tt)*)
        ($($param:ident :)? ...)

        ($($receiver_info:tt)*)
        ($($sel_parsed:tt)*)
        ($($param_parsed:tt)*)
        ($($param_ty_parsed:tt)*)
        ($($ignored_param:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__macros::compile_error!(
            "variadic methods are not yet supported"
        )
    };

    // Too many selector components.
    {
        ($($sel_rest:tt)+)
        ()

        ($($receiver_info:tt)*)
        ($($sel_parsed:tt)*)
        ($($param_parsed:tt)*)
        ($($param_ty_parsed:tt)*)
        ($($ignored_param:tt)*)

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
        ($($params_rest:tt)+)

        ($($receiver_info:tt)*)
        ($($sel_parsed:tt)*)
        ($($param_parsed:tt)*)
        ($($param_ty_parsed:tt)*)
        ($($ignored_param:tt)*)

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
        ($($params_rest:tt)*)

        ($($receiver_info:tt)*)
        ($($sel_parsed:tt)*)
        ($($param_parsed:tt)*)
        ($($param_ty_parsed:tt)*)
        ($($ignored_param:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__macros::compile_error!(
            $crate::__macros::concat!(
                "failed parsing selector `",
                $crate::__macros::stringify!($($sel_rest)*),
                "` or parameters `",
                $crate::__macros::stringify!($($params_rest)*),
                "`",
            )
        )
    };
}
