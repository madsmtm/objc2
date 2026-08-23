/// Parse function parameters.
///
/// This calls `__parse_self!` to parse the `self` parameter, removes
/// `MainThreadMarker` from the parameter list, and splits the remaining
/// parameters into a parameter name and type.
#[doc(hidden)]
#[macro_export]
macro_rules! __parse_params {
    {
        // The function's parameters.
        ($($params:tt)*)

        // The output macro.
        ($out_macro:path)
        $($macro_args:tt)*

        // The following arguments will be appended to the output macro:
        //
        // 1. The receiver and receiver type if the method is an instance
        //    method. This does not include any `mut` keywords that there
        //    might have been in front of the parameter (`mut self: &Self`).
        //    ($($receiver:ident: $receiver_ty:ty)?)
        //
        // 2. The parameter names. This does not include any `mut` keywords
        //    that there might have been in front of the parameter, but does
        //    include any `_` patterns that the user might have set.
        //    ($($param:pat_param,)*)
        //
        // 3. The parameter types.
        //    ($($param_ty:ty,)*)
        //
        // 4. Any parameters that should be ignored (namely, those whose type
        //    is `MainThreadMarker`).
        //    ($($ignored_param:pat_param,)*)
    } => {
        $crate::__parse_self! {
            ($($params)*)

            ($crate::__parse_params_inner)
            ($out_macro)
            ($($macro_args)*)
            ()
            ()
            ()
            // receiver info from __parse_self
            // params_rest from __parse_self
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_params_inner {
    // Base case.
    {
        ($out_macro:path)
        ($($macro_args:tt)*)
        ($($param_parsed:tt)*)
        ($($param_ty_parsed:tt)*)
        ($($ignored_param:tt)*)

        ($($receiver:ident: $receiver_ty:ty)?)
        () // No more parameters to parse
    } => {
        $out_macro! {
            $($macro_args)*

            ($($receiver: $receiver_ty)?)
            ($($param_parsed)*)
            ($($param_ty_parsed)*)
            ($($ignored_param)*)
        }
    };

    // Skip using `MainThreadMarker` in the message send.
    //
    // This is a purely textual match, and using e.g.
    // `objc2::MainThreadMarker` would fail - but that would just be
    // detected as giving a wrong number of arguments, so it's fine for now.
    (
        ($out_macro:path)
        ($($macro_args:tt)*)
        ($($param_parsed:tt)*)
        ($($param_ty_parsed:tt)*)
        ($($ignored_param:tt)*)

        ($($receiver:ident: $receiver_ty:ty)?)
        ($param:ident : MainThreadMarker $(, $($params_rest:tt)*)?)
    ) => {
        $crate::__parse_params_inner! {
            ($out_macro)
            ($($macro_args)*)
            ($($param_parsed)*)
            ($($param_ty_parsed)*)
            ($($ignored_param)* $param,)
            ($($receiver: $receiver_ty)?)
            ($($($params_rest)*)?)
        }
    };

    // Parse various parameter patterns.
    //
    // We cannot just use `$param:pat_param`, since that cannot be followed
    // by a colon: <https://doc.rust-lang.org/reference/macros-by-example.html#r-macro.decl.follow-set.token-pat_param>
    //
    // Most of these are only useful in `define_class!`.
    {
        ($out_macro:path)
        ($($macro_args:tt)*)
        ($($param_parsed:tt)*)
        ($($param_ty_parsed:tt)*)
        ($($ignored_param:tt)*)

        ($($receiver:ident: $receiver_ty:ty)?)
        // `param` pattern.
        ($param:ident : $param_ty:ty $(, $($params_rest:tt)*)?)
    } => {
        $crate::__parse_params_inner! {
            ($out_macro)
            ($($macro_args)*)
            ($($param_parsed)* $param,)
            ($($param_ty_parsed)* $param_ty,)
            ($($ignored_param)*)
            ($($receiver: $receiver_ty)?)
            ($($($params_rest)*)?)
        }
    };
    {
        ($out_macro:path)
        ($($macro_args:tt)*)
        ($($param_parsed:tt)*)
        ($($param_ty_parsed:tt)*)
        ($($ignored_param:tt)*)

        ($($receiver:ident: $receiver_ty:ty)?)
        // `mut param` pattern.
        (mut $param:ident : $param_ty:ty $(, $($params_rest:tt)*)?)
    } => {
        $crate::__parse_params_inner! {
            ($out_macro)
            ($($macro_args)*)
            ($($param_parsed)* $param,) // Intentionally don't add `mut` here.
            ($($param_ty_parsed)* $param_ty,)
            ($($ignored_param)*)
            ($($receiver: $receiver_ty)?)
            ($($($params_rest)*)?)
        }
    };
    {
        ($out_macro:path)
        ($($macro_args:tt)*)
        ($($param_parsed:tt)*)
        ($($param_ty_parsed:tt)*)
        ($($ignored_param:tt)*)

        ($($receiver:ident: $receiver_ty:ty)?)
        // `_` pattern.
        (_ : $param_ty:ty $(, $($params_rest:tt)*)?)
    } => {
        $crate::__parse_params_inner! {
            ($out_macro)
            ($($macro_args)*)
            ($($param_parsed)* _,) // Using this in `extern_methods!` will fail.
            ($($param_ty_parsed)* $param_ty,)
            ($($ignored_param)*)
            ($($receiver: $receiver_ty)?)
            ($($($params_rest)*)?)
        }
    };

    // Variadic methods.
    {
        ($out_macro:path)
        ($($macro_args:tt)*)
        ($($param_parsed:tt)*)
        ($($param_ty_parsed:tt)*)
        ($($ignored_param:tt)*)

        ($($receiver:ident: $receiver_ty:ty)?)
        ($param:ident : ...)
    } => {
        $crate::__macros::compile_error!("variadic methods are not yet supported");
    };
    {
        ($out_macro:path)
        ($($macro_args:tt)*)
        ($($param_parsed:tt)*)
        ($($param_ty_parsed:tt)*)
        ($($ignored_param:tt)*)

        ($($receiver:ident: $receiver_ty:ty)?)
        (...)
    } => {
        $crate::__macros::compile_error!("variadic methods are not yet supported");
    };

    // Could not parse.
    {
        ($out_macro:path)
        ($($macro_args:tt)*)
        ($($param_parsed:tt)*)
        ($($param_ty_parsed:tt)*)
        ($($ignored_param:tt)*)

        ($($receiver:ident: $receiver_ty:ty)?)
        ($($params_rest:tt)*)
    } => {
        $crate::__macros::compile_error!(
            $crate::__macros::concat!(
                "failed parsing parameters `",
                $crate::__macros::stringify!($($params_rest)*),
                "`",
            )
        );
    };
}

#[cfg(test)]
mod tests {
    macro_rules! check {
        {
            ($($expected_receiver:ident : $expected_receiver_ty:ty)?)
            ($($expected_param:tt)*)
            ($($expected_param_ty:ty,)*)
            ($($expected_ignored_param:tt)*)

            ($($receiver:ident: $receiver_ty:ty)?)
            ($($param:tt)*)
            ($($param_ty:ty,)*)
            ($($ignored_param:tt)*)
        } => {
            assert_eq!(stringify!($($expected_receiver)?), stringify!($($receiver)?));
            assert_eq!(stringify!($($expected_receiver_ty)?), stringify!($($receiver_ty)?));
            assert_eq!(stringify!($($expected_param)*), stringify!($($param)*));
            assert_eq!(stringify!($($expected_param_ty,)*), stringify!($($param_ty,)*));
            assert_eq!(stringify!($($expected_ignored_param)*), stringify!($($ignored_param)*));
        };
    }

    #[test]
    fn parse_self() {
        __parse_params! {
            (&self, param: i32)

            (check)
            (self: &Self)
            (param,)
            (i32,)
            ()
        };
        __parse_params! {
            (&'static self, param: i32)

            (check)
            (self: &'static Self)
            (param,)
            (i32,)
            ()
        };
        __parse_params! {
            (&mut self, param: i32)

            (check)
            (self: &mut Self)
            (param,)
            (i32,)
            ()
        };
        __parse_params! {
            (&'s mut self, param: i32)

            (check)
            (self: &'s mut Self)
            (param,)
            (i32,)
            ()
        };

        __parse_params! {
            (self: Self, param: i32)

            (check)
            (self: Self)
            (param,)
            (i32,)
            ()
        };
        __parse_params! {
            (mut self: Self, param: i32)

            (check)
            (self: Self)
            (param,)
            (i32,)
            ()
        };

        __parse_params! {
            (this: Self, param: i32)

            (check)
            (this: Self)
            (param,)
            (i32,)
            ()
        };
        __parse_params! {
            (mut this: Self, param: i32)

            (check)
            (this: Self)
            (param,)
            (i32,)
            ()
        };

        __parse_params! {
            (_this: Self, param: i32)

            (check)
            (_this: Self)
            (param,)
            (i32,)
            ()
        };
        __parse_params! {
            (mut _this: Self, param: i32)

            (check)
            (_this: Self)
            (param,)
            (i32,)
            ()
        };

        __parse_params! {
            (param: i32,)

            (check)
            () // No receiver
            (param,)
            (i32,)
            ()
        };
    }

    #[test]
    fn parse_pattern() {
        __parse_params! {
            (x: u8, mut y: u16, _: u32)

            (check)
            ()
            (x, y, _,)
            (u8, u16, u32,)
            ()
        };
    }

    #[test]
    fn parse_main_thread_marker() {
        __parse_params! {
            (mtm1: MainThreadMarker, x: i32, mtm2: MainThreadMarker)

            (check)
            ()
            (x,)
            (i32,)
            (mtm1, mtm2,)
        };
    }
}
