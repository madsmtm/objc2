/// Detect instance vs. class method.
#[doc(hidden)]
#[macro_export]
macro_rules! __rewrite_self_param {
    {
        // The parameters and parameter types to parse.
        ($($params:tt)*)

        // The output macro.
        ($out_macro:path)

        // Further arguments to passed to the output macro.
        $($out_args:tt)*

        // The following arguments will be appended to the output macro:
        //
        // The builder that needs to be called to register this function with
        // the runtime. One of `add_method` or `add_class_method`.
        // ($builder_method:ident)
        //
        // The receiver that we need call to call the method. Roughly either
        // `self` or `Self::class()`.
        // ($receiver:expr)
        //
        // The type of the receiver.
        // ($receiver_ty:ty)
        //
        // The ABI-required method prefix for functions in `define_class!`.
        // ($($params_prefix:tt)*)
        //
        // The rest of the function's parameters and parameter types.
        // ($($params_rest:tt)*)
    } => {
        $crate::__rewrite_self_param_inner! {
            // Duplicate params out so that we can match on `self`, while still
            // using it as a function parameter
            ($($params)*)
            ($($params)*)

            ($out_macro)
            $($out_args)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __rewrite_self_param_inner {
    // Instance method
    {
        (&self $($__params_rest:tt)*)
        (&$self:ident $(, $($params_rest:tt)*)?)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $out_macro! {
            $($out_args)*

            (add_method)
            ($self)
            (&Self)
            (
                &$self,
                _: $crate::runtime::Sel,
            )
            ($($($params_rest)*)?)
        }
    };
    {
        (&mut self $($__params_rest:tt)*)
        (&mut $self:ident $(, $($params_rest:tt)*)?)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $out_macro! {
            $($out_args)*

            (add_method)
            ($self)
            (&mut Self)
            (
                &mut $self,
                _: $crate::runtime::Sel,
            )
            ($($($params_rest)*)?)
        }
    };
    {
        (self: $__self_ty:ty $(, $($__params_rest:tt)*)?)
        ($self:ident: $self_ty:ty $(, $($params_rest:tt)*)?)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $out_macro! {
            $($out_args)*

            (add_method)
            ($self)
            ($self_ty)
            (
                $self: $self_ty,
                _: $crate::runtime::Sel,
            )
            ($($($params_rest)*)?)
        }
    };
    {
        (mut self: $__self_ty:ty $(, $($__params_rest:tt)*)?)
        ($mut:ident $self:ident: $self_ty:ty $(, $($params_rest:tt)*)?)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $out_macro! {
            $($out_args)*

            (add_method)
            ($self)
            ($self_ty)
            (
                $mut $self: $self_ty,
                _: $crate::runtime::Sel,
            )
            ($($($params_rest)*)?)
        }
    };

    // `this: Type` or `_this: Type` instance method
    // Workaround for arbitrary self types being unstable
    // https://doc.rust-lang.org/nightly/unstable-book/language-features/arbitrary-self-types.html
    {
        (mut this: $__self_ty:ty $(, $($__params_rest:tt)*)?)
        ($mut:ident $this:ident: $this_ty:ty $(, $($params_rest:tt)*)?)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $out_macro! {
            $($out_args)*

            (add_method)
            ($this)
            ($this_ty)
            (
                $mut $this: $this_ty,
                _: $crate::runtime::Sel,
            )
            ($($($params_rest)*)?)
        }
    };
    {
        (this: $__self_ty:ty $(, $($__params_rest:tt)*)?)
        ($this:ident: $this_ty:ty $(, $($params_rest:tt)*)?)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $out_macro! {
            $($out_args)*

            (add_method)
            ($this)
            ($this_ty)
            (
                $this: $this_ty,
                _: $crate::runtime::Sel,
            )
            ($($($params_rest)*)?)
        }
    };
    {
        (mut _this: $__self_ty:ty $(, $($__params_rest:tt)*)?)
        ($mut:ident $this:ident: $this_ty:ty $(, $($params_rest:tt)*)?)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $out_macro! {
            $($out_args)*

            (add_method)
            ($this)
            ($this_ty)
            (
                $mut $this: $this_ty,
                _: $crate::runtime::Sel,
            )
            ($($($params_rest)*)?)
        }
    };
    {
        (_this: $__self_ty:ty $(, $($__params_rest:tt)*)?)
        ($this:ident: $this_ty:ty $(, $($params_rest:tt)*)?)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $out_macro! {
            $($out_args)*

            (add_method)
            ($this)
            ($this_ty)
            (
                $this: $this_ty,
                _: $crate::runtime::Sel,
            )
            ($($($params_rest)*)?)
        }
    };

    // Class method
    {
        ($($__params:tt)*)
        ($($params_rest:tt)*)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $out_macro! {
            $($out_args)*

            (add_class_method)
            (<Self as $crate::ClassType>::class())
            (&$crate::runtime::AnyClass)
            (
                _: &$crate::runtime::AnyClass,
                _: $crate::runtime::Sel,
            )
            ($($params_rest)*)
        }
    };
}
