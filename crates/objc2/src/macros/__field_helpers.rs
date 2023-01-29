#[doc(hidden)]
#[macro_export]
macro_rules! __emit_struct_and_ivars {
    (
        ($(#[$m:meta])*)
        ($v:vis)
        ($($struct:tt)*)
        ($($ivar_helper_module_v:vis mod $ivar_helper_module:ident)?)
        ($($fields:tt)*)
        ($($parsed_fields:tt)*)
    ) => {
        $crate::__parse_fields! {
            ($($fields)*)
            ($($ivar_helper_module_v mod $ivar_helper_module)?)
            () () // No parsed ivars
            ($($parsed_fields)*)

            ($crate::__emit_struct)
            ($(#[$m])*)
            ($v)
            ($($struct)*)
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __emit_struct {
    (
        ($(#[$m:meta])*)
        ($v:vis)
        ($($struct:tt)*)

        ($($fields:tt)*)
    ) => {
        $(#[$m])*
        #[repr(C)]
        $v struct $($struct)* {
            // These are at this point all zero-sized.
            $($fields)*
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_fields {
    // Base-case, no ivars, no module
    (
        () // No more fields left
        () // No module
        () () // No ivars
        ($($parsed_fields:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    ) => {
        $out_macro! {
            $($macro_args)*

            ($($parsed_fields)*)
        }
    };

    // Base-case, has ivars, no module
    (
        () // No more fields left
        () // No module
        ($($ivar_output:tt)+) ($($ivar_type_name:tt)+)
        ($($parsed_fields:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    ) => {
        $crate::__macro_helpers::compile_error!(
            "must specify an ivar module when the type has ivars"
        );

        $($ivar_output)+

        $out_macro! {
            $($macro_args)*

            ($($parsed_fields)*)
        }
    };

    // Base-case, no ivars, has module
    (
        () // No more fields left
        ($ivar_helper_module_v:vis mod $ivar_helper_module:ident)
        () () // No ivars
        ($($parsed_fields:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    ) => {
        $ivar_helper_module_v mod $ivar_helper_module {
            $crate::__macro_helpers::compile_error!(
                "no need to specify an ivar module when the type has no ivars"
            );

            pub(super) fn __objc2_declare_ivars(__objc2_builder: &mut $crate::declare::ClassBuilder) {}
        }

        $out_macro! {
            $($macro_args)*

            ($($parsed_fields)*)
        }
    };

    // Base-case, has ivars, has module
    (
        () // No more fields left
        ($ivar_helper_module_v:vis mod $ivar_helper_module:ident)
        ($($ivar_output:tt)+) ($($ivar_type_name:ident)+)
        ($($parsed_fields:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    ) => {
        $ivar_helper_module_v mod $ivar_helper_module {
            use super::*;

            $($ivar_output)+

            pub(super) fn __objc2_declare_ivars(__objc2_builder: &mut $crate::declare::ClassBuilder) {
                // Ivars
                $(
                    __objc2_builder.add_static_ivar::<$ivar_type_name>();
                )+
            }
        }

        $out_macro! {
            $($macro_args)*

            ($($parsed_fields)*)
        }
    };

    // PhantomData
    (
        (
            $(#[$m:meta])*
            $vis:vis $field_name:ident: PhantomData<$ty:ty>
            $(, $($rest_fields:tt)*)?
        )
        ($($ivar_helper_module_v:vis mod $ivar_helper_module:ident)?)
        ($($ivar_output:tt)*) ($($ivar_type_name:ident)*)
        ($($parsed_fields:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    ) => {
        $crate::__parse_fields! {
            ($($($rest_fields)*)?)
            ($($ivar_helper_module_v mod $ivar_helper_module)?)
            ($($ivar_output)*) ($($ivar_type_name)*)
            (
                $($parsed_fields)*

                // A user could have defined their own PhantomData-like, type,
                // and then tried to use it here, which we would accept, but
                // which wouldn't necessarily be zero-sized!
                //
                // Hence we wrap it in an extra PhantomData, to ensure it is
                // (while still not generating "unused imports" for the user).
                $(#[$m])*
                $vis $field_name: $crate::__macro_helpers::PhantomData<PhantomData<$ty>>,
            )

            ($out_macro)
            $($macro_args)*
        }
    };

    // IvarDrop
    (
        (
            $(#[$m:meta])*
            $vis:vis $field_name:ident: IvarDrop<$ty:ty, $ivar_name:literal>
            $(, $($rest_fields:tt)*)?
        )
        ($($ivar_helper_module_v:vis mod $ivar_helper_module:ident)?)
        ($($ivar_output:tt)*) ($($ivar_type_name:ident)*)
        ($($parsed_fields:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    ) => {
        $crate::__parse_fields! {
            ($($($rest_fields)*)?)
            ($($ivar_helper_module_v mod $ivar_helper_module)?)
            (
                $($ivar_output)*

                #[allow(non_camel_case_types)]
                #[allow(unreachable_pub)]
                pub struct $field_name {
                    __priv: (),
                }

                // SAFETY:
                // - The ivars are in a type used as an Objective-C object.
                // - The ivar is added to the class in `__objc2_declare_ivars`.
                // - Caller upholds that the ivars are properly initialized.
                unsafe impl $crate::declare::IvarType for $field_name {
                    type Type = IvarDrop<$ty>;
                    const NAME: &'static $crate::__macro_helpers::str = $ivar_name;
                }
            ) ($($ivar_type_name)* $field_name)
            (
                $($parsed_fields)*

                $(#[$m])*
                $vis $field_name: $crate::declare::Ivar<$($ivar_helper_module ::)? $field_name>,
            )

            ($out_macro)
            $($macro_args)*
        }
    };

    // IvarEncode
    (
        (
            $(#[$m:meta])*
            $vis:vis $field_name:ident: IvarEncode<$ty:ty, $ivar_name:literal>
            $(, $($rest_fields:tt)*)?
        )
        ($($ivar_helper_module_v:vis mod $ivar_helper_module:ident)?)
        ($($ivar_output:tt)*) ($($ivar_type_name:ident)*)
        ($($parsed_fields:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    ) => {
        $crate::__parse_fields! {
            ($($($rest_fields)*)?)
            ($($ivar_helper_module_v mod $ivar_helper_module)?)
            (
                $($ivar_output)*

                #[allow(non_camel_case_types)]
                #[allow(unreachable_pub)]
                pub struct $field_name {
                    __priv: (),
                }

                // SAFETY: See above
                unsafe impl $crate::declare::IvarType for $field_name {
                    type Type = IvarEncode<$ty>;
                    const NAME: &'static $crate::__macro_helpers::str = $ivar_name;
                }
            ) ($($ivar_type_name)* $field_name)
            (
                $($parsed_fields)*

                $(#[$m])*
                $vis $field_name: $crate::declare::Ivar<$($ivar_helper_module ::)? $field_name>,
            )

            ($out_macro)
            $($macro_args)*
        }
    };

    // IvarBool
    (
        (
            $(#[$m:meta])*
            $vis:vis $field_name:ident: IvarBool<$ivar_name:literal>
            $(, $($rest_fields:tt)*)?
        )
        ($($ivar_helper_module_v:vis mod $ivar_helper_module:ident)?)
        ($($ivar_output:tt)*) ($($ivar_type_name:ident)*)
        ($($parsed_fields:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    ) => {
        $crate::__parse_fields! {
            ($($($rest_fields)*)?)
            ($($ivar_helper_module_v mod $ivar_helper_module)?)
            (
                $($ivar_output)*

                #[allow(non_camel_case_types)]
                #[allow(unreachable_pub)]
                pub struct $field_name {
                    __priv: (),
                }

                // SAFETY: See above
                unsafe impl $crate::declare::IvarType for $field_name {
                    type Type = IvarBool;
                    const NAME: &'static $crate::__macro_helpers::str = $ivar_name;
                }
            ) ($($ivar_type_name)* $field_name)
            (
                $($parsed_fields)*

                $(#[$m])*
                $vis $field_name: $crate::declare::Ivar<$($ivar_helper_module ::)? $field_name>,
            )

            ($out_macro)
            $($macro_args)*
        }
    };

    // Invalid type
    (
        (
            $(#[$m:meta])*
            $vis:vis $field_name:ident: $ty:ty
            $(, $($rest_fields:tt)*)?
        )
        ($($ivar_helper_module_v:vis mod $ivar_helper_module:ident)?)
        ($($ivar_output:tt)*) ($($ivar_type_name:ident)*)
        ($($parsed_fields:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    ) => {
        $crate::__macro_helpers::compile_error!($crate::__macro_helpers::concat!(
            "invalid type ",
            $crate::__macro_helpers::stringify!($ty),
            " in field ",
            $crate::__macro_helpers::stringify!($field_name),
            ". Type must be either `PhantomData`, `IvarDrop`, `IvarBool` or `IvarEncode`."
        ));

        $crate::__parse_fields! {
            ($($($rest_fields)*)?)
            ($($ivar_helper_module_v mod $ivar_helper_module)?)
            ($($ivar_output)*) ($($ivar_type_name)*)
            (
                $($parsed_fields)*

                $(#[$m])*
                $vis $field_name: $ty,
            )

            ($out_macro)
            $($macro_args)*
        }
    }
}
