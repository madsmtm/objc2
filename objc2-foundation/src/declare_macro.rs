#[doc(hidden)]
#[macro_export]
macro_rules! __inner_declare_class {
    {@rewrite_methods @$output_type:ident @$builder:ident} => {};
    {
        @rewrite_methods
        @$output_type:ident
        @$builder:ident

        $(#[$m:meta])*
        @sel($($sel:tt)+)
        fn $name:ident($($args:tt)*) $(-> $ret:ty)? $body:block

        $($rest:tt)*
    } => {
        $crate::__inner_declare_class! {
            @rewrite_methods_inner
            @$output_type
            @$builder
            // Split args out so that we can match on `self`, while still use
            // it as a function argument
            ($($args)*)

            $(#[$m])*
            @sel($($sel)+)
            fn $name($($args)*) $(-> $ret)? $body
        }

        $crate::__inner_declare_class! {
            @rewrite_methods
            @$output_type
            @$builder

            $($rest)*
        }
    };

    // Instance method
    {
        @rewrite_methods_inner
        @$output_type:ident
        @$builder:ident
        (&mut self $($__rest_args:tt)*)

        $(#[$m:meta])*
        @sel($($sel:tt)+)
        fn $name:ident(
            &mut $self:ident
            $(, $($rest_args:tt)*)?
        ) $(-> $ret:ty)? $body:block
    } => {
        $crate::__inner_declare_class! {
            @$output_type
            @instance_method
            @sel($($sel)*)
            @$name
            @$builder
            @($($($rest_args)*)?)

            $(#[$m])*
            extern "C" fn $name(
                &mut $self,
                _cmd: $crate::objc2::runtime::Sel,
                $($($rest_args)*)?
            ) $(-> $ret)? $body
        }
    };
    {
        @rewrite_methods_inner
        @$output_type:ident
        @$builder:ident
        (&self $($__rest_args:tt)*)

        $(#[$m:meta])*
        @sel($($sel:tt)+)
        fn $name:ident(
            &$self:ident
            $(, $($rest_args:tt)*)?
        ) $(-> $ret:ty)? $body:block
    } => {
        $crate::__inner_declare_class! {
            @$output_type
            @instance_method
            @sel($($sel)*)
            @$name
            @$builder
            @($($($rest_args)*)?)

            $(#[$m])*
            extern "C" fn $name(
                &$self,
                _cmd: $crate::objc2::runtime::Sel,
                $($($rest_args)*)?
            ) $(-> $ret)? $body
        }
    };
    {
        @rewrite_methods_inner
        @$output_type:ident
        @$builder:ident
        (
            mut self: $__self_ty:ty
            $(, $($__rest_args:tt)*)?
        )

        $(#[$m:meta])*
        @sel($($sel:tt)+)
        fn $name:ident(
            mut $self:ident: $self_ty:ty
            $(, $($rest_args:tt)*)?
        ) $(-> $ret:ty)? $body:block
    } => {
        $crate::__inner_declare_class! {
            @$output_type
            @instance_method
            @sel($($sel)*)
            @$name
            @$builder
            @($($($rest_args)*)?)

            $(#[$m])*
            extern "C" fn $name(
                mut $self: $self_ty,
                _cmd: $crate::objc2::runtime::Sel,
                $($($rest_args)*)?
            ) $(-> $ret)? $body
        }
    };
    {
        @rewrite_methods_inner
        @$output_type:ident
        @$builder:ident
        (
            self: $__self_ty:ty
            $(, $($__rest_args:tt)*)?
        )

        $(#[$m:meta])*
        @sel($($sel:tt)+)
        fn $name:ident(
            $self:ident: $self_ty:ty
            $(, $($rest_args:tt)*)?
        ) $(-> $ret:ty)? $body:block
    } => {
        $crate::__inner_declare_class! {
            @$output_type
            @instance_method
            @sel($($sel)*)
            @$name
            @$builder
            @($($($rest_args)*)?)

            $(#[$m])*
            extern "C" fn $name(
                $self: $self_ty,
                _cmd: $crate::objc2::runtime::Sel,
                $($($rest_args)*)?
            ) $(-> $ret)? $body
        }
    };

    // Class method
    {
        @rewrite_methods_inner
        @$output_type:ident
        @$builder:ident
        ($($__args:tt)*)

        $(#[$m:meta])*
        @sel($($sel:tt)+)
        fn $name:ident(
            $($args:tt)*
        ) $(-> $ret:ty)? $body:block
    } => {
        $crate::__inner_declare_class! {
            @$output_type
            @class_method
            @sel($($sel)*)
            @$name
            @$builder
            @($($args)*)

            $(#[$m])*
            extern "C" fn $name(
                _cls: &$crate::objc2::runtime::Class,
                _cmd: $crate::objc2::runtime::Sel,
                $($args)*
            ) $(-> $ret)? $body
        }
    };

    {
        @method_out
        @$method_type:ident
        @sel($($sel:tt)*)
        @$name:ident
        @$builder:ident
        @($($builder_args:tt)*)

        $method:item
    } => {
        $method
    };
    {
        @register_out
        @class_method
        @sel($($sel:tt)*)
        @$name:ident
        @$builder:ident
        @($($builder_args:tt)*)

        $method:item
    } => {
        $builder.add_class_method(
            $crate::objc2::sel!($($sel)*),
            $crate::__inner_declare_class! {
                @cast_extern_fn
                @$name
                $($builder_args)*
            },
        );
    };
    {
        @register_out
        @instance_method
        @sel($($sel:tt)*)
        @$name:ident
        @$builder:ident
        @($($builder_args:tt)*)

        $method:item
    } => {
        $builder.add_method(
            $crate::objc2::sel!($($sel)*),
            $crate::__inner_declare_class! {
                @cast_extern_fn
                @$name
                $($builder_args)*
            },
        );
    };

    // Create the `as extern "C" fn(...) -> _` cast
    //
    // TODO: Investigate if there's a better way of doing this
    {
        @cast_extern_fn
        @$name:ident

        $(,)?
    } => {
        Self::$name as extern "C" fn(_, _) -> _
    };
    {
        @cast_extern_fn
        @$name:ident

        $param1:ident: $param1_ty:ty $(,)?
    } => {
        Self::$name as extern "C" fn(_, _, _) -> _
    };
    {
        @cast_extern_fn
        @$name:ident

        $param1:ident: $param1_ty:ty,
        $param2:ident: $param2_ty:ty $(,)?
    } => {
        Self::$name as extern "C" fn(_, _, _, _) -> _
    };
    {
        @cast_extern_fn
        @$name:ident

        $param1:ident: $param1_ty:ty,
        $param2:ident: $param2_ty:ty,
        $param3:ident: $param3_ty:ty $(,)?
    } => {
        Self::$name as extern "C" fn(_, _, _, _, _) -> _
    };
    {
        @cast_extern_fn
        @$name:ident

        $param1:ident: $param1_ty:ty,
        $param2:ident: $param2_ty:ty,
        $param3:ident: $param3_ty:ty,
        $param4:ident: $param4_ty:ty $(,)?
    } => {
        Self::$name as extern "C" fn(_, _, _, _, _, _) -> _
    };
    {
        @cast_extern_fn
        @$name:ident

        $param1:ident: $param1_ty:ty,
        $param2:ident: $param2_ty:ty,
        $param3:ident: $param3_ty:ty,
        $param4:ident: $param4_ty:ty,
        $param5:ident: $param5_ty:ty $(,)?
    } => {
        Self::$name as extern "C" fn(_, _, _, _, _, _, _) -> _
    };
    {
        @cast_extern_fn
        @$name:ident

        $param1:ident: $param1_ty:ty,
        $param2:ident: $param2_ty:ty,
        $param3:ident: $param3_ty:ty,
        $param4:ident: $param4_ty:ty,
        $param5:ident: $param5_ty:ty,
        $param6:ident: $param6_ty:ty $(,)?
    } => {
        Self::$name as extern "C" fn(_, _, _, _, _, _, _, _) -> _
    };
    {
        @cast_extern_fn
        @$name:ident

        $param1:ident: $param1_ty:ty,
        $param2:ident: $param2_ty:ty,
        $param3:ident: $param3_ty:ty,
        $param4:ident: $param4_ty:ty,
        $param5:ident: $param5_ty:ty,
        $param6:ident: $param6_ty:ty,
        $param7:ident: $param7_ty:ty $(,)?
    } => {
        Self::$name as extern "C" fn(_, _, _, _, _, _, _, _, _) -> _
    };
    {
        @cast_extern_fn
        @$name:ident

        $param1:ident: $param1_ty:ty,
        $param2:ident: $param2_ty:ty,
        $param3:ident: $param3_ty:ty,
        $param4:ident: $param4_ty:ty,
        $param5:ident: $param5_ty:ty,
        $param6:ident: $param6_ty:ty,
        $param7:ident: $param7_ty:ty,
        $param8:ident: $param8_ty:ty $(,)?
    } => {
        Self::$name as extern "C" fn(_, _, _, _, _, _, _, _, _, _) -> _
    };
    {
        @cast_extern_fn
        @$name:ident

        $param1:ident: $param1_ty:ty,
        $param2:ident: $param2_ty:ty,
        $param3:ident: $param3_ty:ty,
        $param4:ident: $param4_ty:ty,
        $param5:ident: $param5_ty:ty,
        $param6:ident: $param6_ty:ty,
        $param7:ident: $param7_ty:ty,
        $param8:ident: $param8_ty:ty,
        $param9:ident: $param9_ty:ty $(,)?
    } => {
        Self::$name as extern "C" fn(_, _, _, _, _, _, _, _, _, _, _) -> _
    };
    {
        @cast_extern_fn
        @$name:ident

        $param1:ident: $param1_ty:ty,
        $param2:ident: $param2_ty:ty,
        $param3:ident: $param3_ty:ty,
        $param4:ident: $param4_ty:ty,
        $param5:ident: $param5_ty:ty,
        $param6:ident: $param6_ty:ty,
        $param7:ident: $param7_ty:ty,
        $param8:ident: $param8_ty:ty,
        $param9:ident: $param9_ty:ty,
        $param10:ident: $param10_ty:ty $(,)?
    } => {
        Self::$name as extern "C" fn(_, _, _, _, _, _, _, _, _, _, _, _) -> _
    };
    {
        @cast_extern_fn
        @$name:ident

        $param1:ident: $param1_ty:ty,
        $param2:ident: $param2_ty:ty,
        $param3:ident: $param3_ty:ty,
        $param4:ident: $param4_ty:ty,
        $param5:ident: $param5_ty:ty,
        $param6:ident: $param6_ty:ty,
        $param7:ident: $param7_ty:ty,
        $param8:ident: $param8_ty:ty,
        $param9:ident: $param9_ty:ty,
        $param10:ident: $param10_ty:ty,
        $param11:ident: $param11_ty:ty $(,)?
    } => {
        Self::$name as extern "C" fn(_, _, _, _, _, _, _, _, _, _, _, _, _) -> _
    };
    {
        @cast_extern_fn
        @$name:ident

        $param1:ident: $param1_ty:ty,
        $param2:ident: $param2_ty:ty,
        $param3:ident: $param3_ty:ty,
        $param4:ident: $param4_ty:ty,
        $param5:ident: $param5_ty:ty,
        $param6:ident: $param6_ty:ty,
        $param7:ident: $param7_ty:ty,
        $param8:ident: $param8_ty:ty,
        $param9:ident: $param9_ty:ty,
        $param10:ident: $param10_ty:ty,
        $param11:ident: $param11_ty:ty,
        $param12:ident: $param12_ty:ty $(,)?
    } => {
        Self::$name as extern "C" fn(_, _, _, _, _, _, _, _, _, _, _, _, _, _) -> _
    };
}

/// TODO
///
/// This macro is limited in a few spots, in particular:
/// - A transformation step is performed on the methods, and hence they should
///   not be called manually, only through a `msg_send!` (they can't be marked
///   `pub` nor `unsafe` for the same reason).
/// - ...
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
            fn class() -> &'static $crate::objc2::runtime::Class {
                // TODO: Use `core::cell::LazyCell`
                use $crate::__std::sync::Once;

                use $crate::objc2::declare::ClassBuilder;
                use $crate::objc2::runtime::Protocol;
                static REGISTER_CLASS: Once = Once::new();

                REGISTER_CLASS.call_once(|| {
                    let superclass = <$inherits>::class();
                    let mut builder = ClassBuilder::new(stringify!($name), superclass).unwrap();

                    // Implement protocols
                    $(
                        $(
                            builder.add_protocol(Protocol::get(stringify!($protocols)).unwrap());
                        )+
                    )?

                    $(
                        builder.add_ivar::<$ivar_ty>(stringify!($ivar));
                    )*

                    unsafe {
                        $crate::__inner_declare_class! {
                            @rewrite_methods
                            @register_out
                            @builder

                            $($methods)*
                        }
                    }

                    let _cls = builder.register();
                });

                $crate::objc2::class!($name)
            }
        }

        // Methods
        $(#[$impl_m])*
        impl $name {
            $crate::__inner_declare_class! {
                @rewrite_methods
                @method_out
                @__builder

                $($methods)*
            }
        }
    };
}
