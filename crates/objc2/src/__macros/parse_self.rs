/// Parse and extract the `self` parameter.
///
/// Allows differentiating between instance methods and class methods.
#[doc(hidden)]
#[macro_export]
macro_rules! __parse_self {
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
        // 2. The remaining parameters.
        //    ($($params_rest:tt)*)
    } => {
        $crate::__parse_self_inner! {
            // Duplicate parameters so that we can match on `self`, while
            // still passing it on to another macro.
            //
            // (This is required because `self` is kinda special, and it isn't
            // allowed for macros to produce it in certain contexts).
            ($($params)*)
            ($($params)*)

            ($out_macro)
            $($macro_args)*
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_self_inner {
    // Instance method.
    //
    // Basically anything that is `SelfParam`:
    // <https://doc.rust-lang.org/nightly/reference/items/functions.html#grammar-SelfParam>
    {
        (&       $($_lifetime:lifetime)? self        $(, $($_params_rest:tt)*)?)
        ($ref:tt $($lifetime:lifetime)?  $self:ident $(, $($params_rest:tt)*)? )

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*

            ($self: $ref $($lifetime)? Self)
            ($($($params_rest)*)?)
        }
    };
    {
        (&       $($_lifetime:lifetime)? mut        self        $(, $($_params_rest:tt)*)?)
        ($ref:tt $($lifetime:lifetime)?  $mut:ident $self:ident $(, $($params_rest:tt)*)? )

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*

            ($self: $ref $($lifetime)? $mut Self)
            ($($($params_rest)*)?)
        }
    };
    {
        (self        : $_self_ty:ty $(, $($_params_rest:tt)*)?)
        ($self:ident : $self_ty:ty  $(, $($params_rest:tt)*)? )

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*

            ($self: $self_ty)
            ($($($params_rest)*)?)
        }
    };
    {
        (mut         self        : $_self_ty:ty $(, $($_params_rest:tt)*)?)
        ($_mut:ident $self:ident : $self_ty:ty  $(, $($params_rest:tt)*)? )

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*

            ($self: $self_ty)
            ($($($params_rest)*)?)
        }
    };

    // `this: Type` or `_this: Type` instance method.
    //
    // Workaround for arbitrary self types being unstable:
    // <https://doc.rust-lang.org/nightly/unstable-book/language-features/arbitrary-self-types.html>
    {
        (this        : $_this_ty:ty $(, $($_params_rest:tt)*)?)
        ($this:ident : $this_ty:ty  $(, $($params_rest:tt)*)? )

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*

            ($this: $this_ty)
            ($($($params_rest)*)?)
        }
    };
    {
        (mut         this        : $_this_ty:ty $(, $($_params_rest:tt)*)?)
        ($_mut:ident $this:ident : $this_ty:ty  $(, $($params_rest:tt)*)? )

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*

            ($this: $this_ty)
            ($($($params_rest)*)?)
        }
    };
    {
        (_this       : $_this_ty:ty $(, $($_params_rest:tt)*)?)
        ($this:ident : $this_ty:ty  $(, $($params_rest:tt)*)? )

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*

            ($this: $this_ty)
            ($($($params_rest)*)?)
        }
    };
    {
        (mut         _this       : $_this_ty:ty $(, $($_params_rest:tt)*)?)
        ($_mut:ident $this:ident : $this_ty:ty  $(, $($params_rest:tt)*)? )

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*

            ($this: $this_ty)
            ($($($params_rest)*)?)
        }
    };

    // Class method.
    //
    // This is intentionally placed last, since we only want to assume a class
    // method if none of the above succeeded.
    {
        ($($_params_rest:tt)*)
        ($($params_rest:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*

            () // No receiver
            ($($params_rest)*)
        }
    };
}
