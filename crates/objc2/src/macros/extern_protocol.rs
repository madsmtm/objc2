/// TODO
#[doc(alias = "@protocol")]
#[macro_export]
macro_rules! extern_protocol {
    (
        $(#[$m:meta])*
        $v:vis struct $name:ident;

        unsafe impl ProtocolType for $for:ty {
            $(const NAME: &'static str = $name_const:literal;)?
        }
    ) => {
        $crate::__inner_extern_class!(
            @__inner

            $(#[$m])*
            $v struct ($name) {}

            unsafe impl () for $for {
                INHERITS = [$crate::runtime::Object];
            }
        );

        // SAFETY: TODO
        unsafe impl ProtocolType for $for {
            const NAME: &'static str = $crate::__select_name!($name; $($name_const)?);
        }

        const _: () = {
            if $crate::__macro_helpers::size_of::<$name>() != 0 {
                panic!(concat!(
                    "the struct ",
                    stringify!($name),
                    " is not zero-sized!",
                ))
            }
        };
    };
}
