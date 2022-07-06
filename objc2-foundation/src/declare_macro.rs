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
            $(
                $s:item
            )*
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

                // TODO: Fix shim
                extern "C" fn init_with(
                    this: &mut CustomAppDelegate,
                    _cmd: $crate::objc2::runtime::Sel,
                    ivar: u8,
                    another_ivar: Bool,
                ) -> *mut CustomAppDelegate {
                    CustomAppDelegate::init_with(this, ivar, another_ivar)
                }

                unsafe {
                    builder.add_method(
                        $crate::objc2::sel!(initWith:another:),
                        init_with as unsafe extern "C" fn(_, _, _, _) -> _,
                    );
                }

                builder.register()
            }
        }

        // Methods
        $(#[$impl_m])*
        impl $name {
            $(
                $s
            )*
        }
    };
}
