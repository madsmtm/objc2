#[doc(hidden)]
#[macro_export]
macro_rules! __inner_declare_class {
    {@rewrite_methods @$output_type:ident} => {};
    {
        @rewrite_methods
        @$output_type:ident

        $(#[$m:meta])*
        @sel($($sel:tt)+)
        $v:vis fn $name:ident($($args:tt)*) $(-> $ret:ty)? $body:block

        $($rest:tt)*
    } => {
        $crate::__inner_declare_class! {
            @rewrite_methods_inner
            @$output_type
            // Split args out so that we can match on `self`, while still use
            // it as a function argument
            ($($args)*)

            $(#[$m])*
            @sel($($sel)+)
            $v fn $name($($args)*) $(-> $ret)? $body
        }

        $crate::__inner_declare_class! {
            @rewrite_methods
            @$output_type

            $($rest)*
        }
    };

    // Instance method
    {
        @rewrite_methods_inner
        @$output_type:ident
        (&mut self $($__rest_args:tt)*)

        $(#[$m:meta])*
        @sel($($sel:tt)+)
        $v:vis fn $name:ident(
            &mut $self:ident
            $($rest_args:tt)*
        ) $(-> $ret:ty)? $body:block
    } => {
        $crate::__inner_declare_class! {
            @$output_type

            $(#[$m])*
            $v extern "C" fn $name(
                &mut $self,
                _cmd: $crate::objc2::runtime::Sel
                $($rest_args)*
            ) $(-> $ret)? $body
        }
    };
    {
        @rewrite_methods_inner
        @$output_type:ident
        (&self $($__rest_args:tt)*)

        $(#[$m:meta])*
        @sel($($sel:tt)+)
        $v:vis fn $name:ident(
            &$self:ident
            $($rest_args:tt)*
        ) $(-> $ret:ty)? $body:block
    } => {
        $crate::__inner_declare_class! {
            @$output_type

            $(#[$m])*
            $v extern "C" fn $name(
                &$self,
                _cmd: $crate::objc2::runtime::Sel
                $($rest_args)*
            ) $(-> $ret)? $body
        }
    };
    {
        @rewrite_methods_inner
        @$output_type:ident
        (
            mut self: $__self_ty:ty
            $(, $($__rest_args:tt)*)?
        )

        $(#[$m:meta])*
        @sel($($sel:tt)+)
        $v:vis fn $name:ident(
            mut $self:ident: $self_ty:ty
            $(, $($rest_args:tt)*)?
        ) $(-> $ret:ty)? $body:block
    } => {
        $crate::__inner_declare_class! {
            @$output_type

            $(#[$m])*
            $v extern "C" fn $name(
                mut $self: $self_ty,
                _cmd: $crate::objc2::runtime::Sel
                $(, $($rest_args)*)?
            ) $(-> $ret)? $body
        }
    };
    {
        @rewrite_methods_inner
        @$output_type:ident
        (
            self: $__self_ty:ty
            $(, $($__rest_args:tt)*)?
        )

        $(#[$m:meta])*
        @sel($($sel:tt)+)
        $v:vis fn $name:ident(
            $self:ident: $self_ty:ty
            $(, $($rest_args:tt)*)?
        ) $(-> $ret:ty)? $body:block
    } => {
        $crate::__inner_declare_class! {
            @$output_type

            $(#[$m])*
            $v extern "C" fn $name(
                $self: $self_ty,
                _cmd: $crate::objc2::runtime::Sel
                $(, $($rest_args)*)?
            ) $(-> $ret)? $body
        }
    };

    // Class method
    {
        @rewrite_methods_inner
        @$output_type:ident
        ($($__args:tt)*)

        $(#[$m:meta])*
        @sel($($sel:tt)+)
        $v:vis fn $name:ident(
            $($args:tt)*
        ) $(-> $ret:ty)? $body:block
    } => {
        $crate::__inner_declare_class! {
            @$output_type

            $(#[$m])*
            $v extern "C" fn $name(
                _cls: &$crate::objc2::runtime::Class,
                _cmd: $crate::objc2::runtime::Sel,
                $($args)*
            ) $(-> $ret)? $body
        }
    };

    {
        @method_out

        $($method:item)*
    } => {
        $($method)*
    };
}

/// TODO
#[macro_export]
macro_rules! declare_class {
    {
        $(#[$m:meta])*
        unsafe $v:vis struct $name:ident: $inherits:ident $(, $inheritance_rest:ident)* $(<$($protocols:ident),+ $(,)?>)? {
            $($ivar_v:vis $ivar:ident: $ivar_ty:ty,)*
        }

        $(#[$impl_m:meta])*
        unsafe impl {
            $($methods:tt)*
        }
    } => {
        $(
            #[allow(non_camel_case_types)]
            $ivar_v struct $ivar {
                __priv: (),
            }

            unsafe impl $crate::objc2::declare::IvarType for $ivar {
                type Type = $ivar_ty;
                const NAME: &'static str = stringify!($ivar);
            }
        )*

        $crate::__inner_extern_class! {
            @__inner
            $(#[$m])*
            unsafe $v struct $name<>: $inherits, $($inheritance_rest,)* $crate::objc2::runtime::Object {
                $($ivar_v $ivar: $crate::objc2::declare::Ivar<$ivar>,)*
            }
        }

        // Creation
        impl $name {
            fn create_class() -> &'static $crate::objc2::runtime::Class {
                let superclass = <$inherits>::class();
                let mut builder = $crate::objc2::declare::ClassBuilder::new(stringify!($name), superclass).unwrap();

                // Implement protocols
                $(
                    $(
                        builder.add_protocol($crate::objc2::runtime::Protocol::get(stringify!($protocols)).unwrap());
                    )+
                )?

                $(
                    builder.add_ivar::<$ivar_ty>(stringify!($ivar));
                )*

                unsafe {
                    builder.add_method(
                        $crate::objc2::sel!(initWith:another:),
                        $name::init_with as unsafe extern "C" fn(_, _, _, _) -> _,
                    );
                }

                builder.register()
            }
        }

        // Methods
        $(#[$impl_m])*
        impl $name {
            $crate::__inner_declare_class! {
                @rewrite_methods
                @method_out

                $($methods)*
            }
        }
    };
}
