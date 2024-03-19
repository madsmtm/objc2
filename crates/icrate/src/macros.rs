#![allow(unused_macros)]

macro_rules! impl_encode {
    (
        $(#[$m:meta])*
        $name:ident = $encoding:expr;
    ) => {
        $(#[$m])*
        #[cfg(feature = "objc2")]
        unsafe impl objc2::Encode for $name {
            const ENCODING: objc2::Encoding = $encoding;
        }

        $(#[$m])*
        #[cfg(feature = "objc2")]
        unsafe impl objc2::RefEncode for $name {
            const ENCODING_REF: objc2::Encoding =
                objc2::Encoding::Pointer(&<Self as objc2::Encode>::ENCODING);
        }
    };
}

macro_rules! extern_struct {
    (
        #[encoding_name($encoding_name:literal)]
        $(#[$m:meta])*
        $v:vis struct $name:ident {
            $($field_v:vis $field:ident: $ty:ty),* $(,)?
        }
    ) => {
        #[repr(C)]
        #[derive(Clone, Copy, Debug, PartialEq)]
        $(#[$m])*
        $v struct $name {
            $($field_v $field: $ty,)*
        }

        $(#[$m])*
        impl_encode! {
            $name = objc2::Encoding::Struct(
                $encoding_name,
                &[$(<$ty as objc2::Encode>::ENCODING),*],
            );
        }
    };
    (
        $(#[$m:meta])*
        $v:vis struct $name:ident {
            $($field_v:vis $field:ident: $ty:ty),* $(,)?
        }
    ) => {
        #[repr(C)]
        #[derive(Clone, Copy, Debug, PartialEq)]
        $(#[$m])*
        $v struct $name {
            $($field_v $field: $ty,)*
        }

        $(#[$m])*
        impl_encode! {
            $name = objc2::Encoding::Struct(
                stringify!($name),
                &[$(<$ty as objc2::Encode>::ENCODING),*],
            );
        }
    };
}
