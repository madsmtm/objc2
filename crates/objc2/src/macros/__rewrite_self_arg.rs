#[doc(hidden)]
#[macro_export]
macro_rules! __rewrite_self_arg {
    {
        ($out_macro:path)
        ($($args:tt)*)

        $($macro_args:tt)*
    } => {
        $crate::__rewrite_self_arg_inner! {
            ($out_macro)
            // Duplicate args out so that we can match on `self`, while still
            // using it as a function argument
            ($($args)*)
            ($($args)*)

            $($macro_args)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __rewrite_self_arg_inner {
    // Instance method
    {
        ($out_macro:path)
        (&self $($__rest_args:tt)*)
        (&$self:ident $(, $($rest:tt)*)?)

        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*
            @(instance_method)
            @(
                $self: &Self,
                _: $crate::runtime::Sel,
            )
            @($($($rest)*)?)
        }
    };
    {
        ($out_macro:path)
        (&mut self $($__rest_args:tt)*)
        (&mut $self:ident $(, $($rest:tt)*)?)

        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*
            @(instance_method)
            @(
                $self: &mut Self,
                _: $crate::runtime::Sel,
            )
            @($($($rest)*)?)
        }
    };
    {
        ($out_macro:path)
        (self: $__self_ty:ty $(, $($__rest_args:tt)*)?)
        ($self:ident: $self_ty:ty $(, $($rest:tt)*)?)

        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*
            @(instance_method)
            @(
                $self: $self_ty,
                _: $crate::runtime::Sel,
            )
            @($($($rest)*)?)
        }
    };
    {
        ($out_macro:path)
        (mut self: $__self_ty:ty $(, $($__rest_args:tt)*)?)
        (mut $self:ident: $self_ty:ty $(, $($rest:tt)*)?)

        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*
            @(instance_method)
            @(
                mut $self: $self_ty,
                _: $crate::runtime::Sel,
            )
            @($($($rest)*)?)
        }
    };

    // `this: Type` or `_this: Type` instance method
    // Workaround for arbitary self types being unstable
    // https://doc.rust-lang.org/nightly/unstable-book/language-features/arbitrary-self-types.html
    {
        ($out_macro:path)
        (mut this: $__self_ty:ty $(, $($__rest_args:tt)*)?)
        (mut $this:ident: $this_ty:ty $(, $($rest:tt)*)?)

        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*
            @(instance_method)
            @(
                mut $this: $this_ty,
                _: $crate::runtime::Sel,
            )
            @($($($rest)*)?)
        }
    };
    {
        ($out_macro:path)
        (this: $__self_ty:ty $(, $($__rest_args:tt)*)?)
        ($this:ident: $this_ty:ty $(, $($rest:tt)*)?)

        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*
            @(instance_method)
            @(
                $this: $this_ty,
                _: $crate::runtime::Sel,
            )
            @($($($rest)*)?)
        }
    };
    {
        ($out_macro:path)
        (mut _this: $__self_ty:ty $(, $($__rest_args:tt)*)?)
        (mut $this:ident: $this_ty:ty $(, $($rest:tt)*)?)

        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*
            @(instance_method)
            @(
                mut $this: $this_ty,
                _: $crate::runtime::Sel,
            )
            @($($($rest)*)?)
        }
    };
    {
        ($out_macro:path)
        (_this: $__self_ty:ty $(, $($__rest_args:tt)*)?)
        ($this:ident: $this_ty:ty $(, $($rest:tt)*)?)

        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*
            @(instance_method)
            @(
                $this: $this_ty,
                _: $crate::runtime::Sel,
            )
            @($($($rest)*)?)
        }
    };

    // Class method
    {
        ($out_macro:path)
        ($($__args:tt)*)
        ($($args:tt)*)

        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*
            @(class_method)
            @(
                _: &$crate::runtime::Class,
                _: $crate::runtime::Sel,
            )
            @($($args)*)
        }
    };
}
