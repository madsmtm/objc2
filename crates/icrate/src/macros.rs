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

macro_rules! extern_enum {
    (
        #[underlying($ty:path)]
        $(#[$m:meta])*
        $v:vis enum $name:ident {
            $(
                $(#[$field_m:meta])*
                $field:ident = $value:expr
            ),* $(,)?
        }
    ) => {
        $(#[$m])*
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        $v struct $name(pub $ty);

        // SAFETY: The type is `#[repr(transparent)]`
        impl_encode! {
            $name = <$ty as objc2::Encode>::ENCODING;
        }

        impl $name {
            $(
                $(#[$field_m])*
                #[allow(non_upper_case_globals)]
                pub const $field: Self = Self($value);
            )*
        }
    };
}

/// Corresponds to `NS_ENUM`
macro_rules! ns_enum {
    (
        #[underlying($ty:ty)]
        $(#[$m:meta])*
        $v:vis enum $name:ident {
            $(
                $(#[$field_m:meta])*
                $field:ident = $value:expr
            ),* $(,)?
        }
    ) => {
        // External enums can be safely constructed from the raw value, this
        // will not cause unsoundess in external libraries (but will likely
        // lead to an exception / a crash, if the invalid value is used).
        //
        // <https://developer.apple.com/documentation/swift/grouping-related-objective-c-constants#Declare-Simple-Enumerations>
        //
        // TODO: Once Rust gains support for more precisely specifying niches,
        // use that to convert this into a native enum with a hidden variant
        // that contains the remaining cases.
        extern_enum! {
            #[underlying($ty)]
            $(#[$m])*
            $v enum $name {
                $(
                    $(#[$field_m])*
                    $field = $value
                ),*
            }
        }
    };
}

/// Corresponds to `NS_OPTIONS`
macro_rules! ns_options {
    (
        #[underlying($ty:ty)]
        $(#[$m:meta])*
        $v:vis enum $name:ident {
            $(
                $(#[$field_m:meta])*
                $field:ident = $value:expr
            ),* $(,)?
        }
    ) => {
        // TODO: Handle this differently (e.g. as `bitflags`)
        extern_enum! {
            #[underlying($ty)]
            $(#[$m])*
            $v enum $name {
                $(
                    $(#[$field_m])*
                    $field = $value
                ),*
            }
        }
    };
}

/// Corresponds to `NS_CLOSED_ENUM`
macro_rules! ns_closed_enum {
    (
        #[underlying(NSUInteger)]
        $(#[$m:meta])*
        $v:vis enum $name:ident {
            $(
                $(#[$field_m:meta])*
                $field:ident = $value:expr
            ),* $(,)?
        }
    ) => {
        ns_closed_enum_inner! {
            #[repr(usize)]
            $(#[$m])*
            $v enum $name {
                $(
                    $(#[$field_m])*
                    $field = $value,
                )*
            }
        }
    };
    (
        #[underlying(NSInteger)]
        $(#[$m:meta])*
        $v:vis enum $name:ident {
            $(
                $(#[$field_m:meta])*
                $field:ident = $value:expr
            ),* $(,)?
        }
    ) => {
        ns_closed_enum_inner! {
            #[repr(isize)]
            $(#[$m])*
            $v enum $name {
                $(
                    $(#[$field_m])*
                    $field = $value,
                )*
            }
        }
    };
}

macro_rules! ns_closed_enum_inner {
    (
        #[repr($ty:ty)]
        $(#[$m:meta])*
        $v:vis enum $name:ident {
            $(
                $(#[$field_m:meta])*
                $field:ident = $value:expr,
            )*
        }
    ) => {
        // SAFETY: `NS_CLOSED_ENUM` is guaranteed to never gain additional
        // cases, so we are allowed to use a Rust enum (which in turn will
        // assume that the unused patterns are valid to use as a niche).
        #[repr($ty)]
        $(#[$m])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        $v enum $name {
            $(
                $(#[$field_m])*
                $field = $value,
            )*
        }

        // SAFETY: The enum is `#[repr($ty)]`.
        impl_encode! {
            $name = <$ty as objc2::Encode>::ENCODING;
        }
    }
}

/// Corresponds to `NS_ERROR_ENUM`
macro_rules! ns_error_enum {
    (
        #[underlying(NSInteger)]
        $(#[$m:meta])*
        $v:vis enum $name:ident {
            $(
                $(#[$field_m:meta])*
                $field:ident = $value:expr
            ),* $(,)?
        }
    ) => {
        // TODO: Handle this differently
        extern_enum! {
            #[underlying(NSInteger)]
            $(#[$m])*
            $v enum $name {
                $(
                    $(#[$field_m])*
                    $field = $value
                ),*
            }
        }
    };
}

macro_rules! typed_enum {
    ($v:vis type $name:ident = $ty:ty $(;)?) => {
        // TODO
        pub type $name = $ty;
    };
}

macro_rules! typed_extensible_enum {
    ($v:vis type $name:ident = $ty:ty $(;)?) => {
        // TODO
        pub type $name = $ty;
    };
}

macro_rules! extern_static {
    ($name:ident: $ty:ty) => {
        extern "C" {
            pub static $name: $ty;
        }
    };
    // Floats in statics are broken
    ($name:ident: NSAppKitVersion = $($value:tt)*) => {
        pub static $name: NSAppKitVersion = $($value)* as _;
    };
    ($name:ident: NSLayoutPriority = $($value:tt)*) => {
        pub static $name: NSLayoutPriority = $($value)* as _;
    };
    ($name:ident: NSStackViewVisibilityPriority = $($value:tt)*) => {
        pub static $name: NSStackViewVisibilityPriority = $($value)* as _;
    };
    ($name:ident: NSTouchBarItemPriority = $($value:tt)*) => {
        pub static $name: NSTouchBarItemPriority = $($value)* as _;
    };
    ($name:ident: MKFeatureDisplayPriority = $($value:tt)*) => {
        pub static $name: MKFeatureDisplayPriority = $($value)* as _;
    };
    ($name:ident: MKAnnotationViewZPriority = $($value:tt)*) => {
        pub static $name: MKAnnotationViewZPriority = $($value)* as _;
    };
    // Normal case
    ($name:ident: $ty:path = $value:expr) => {
        pub static $name: $ty = $value;
    };
}

macro_rules! extern_fn {
    (
        $(#[$m:meta])*
        $v:vis unsafe fn $name:ident($($params:tt)*) $(-> $res:ty)?;
    ) => {
        $(#[$m])*
        extern "C" {
            $v fn $name($($params)*) $(-> $res)?;
        }
    };
    (
        $(#[$m:meta])*
        $v:vis fn $name:ident($($arg:ident: $arg_ty:ty),* $(,)?) $(-> $res:ty)?;
    ) => {
        #[inline]
        $(#[$m])*
        $v extern "C" fn $name($($arg: $arg_ty),*) $(-> $res)? {
            extern "C" {
                fn $name($($arg: $arg_ty),*) $(-> $res)?;
            }
            unsafe {
                $name($($arg),*)
            }
        }
    };
}

macro_rules! inline_fn {
    (
        $(#[$m:meta])*
        $v:vis unsafe fn $name:ident($($params:tt)*) $(-> $res:ty)? $body:block
    ) => {
        // TODO
    };
}
