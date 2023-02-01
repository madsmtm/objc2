#![allow(unused_macros)]

macro_rules! impl_encode {
    ($name:ident = $encoding:expr;) => {
        #[cfg(feature = "objective-c")]
        unsafe impl objc2::Encode for $name {
            const ENCODING: objc2::Encoding = $encoding;
        }

        #[cfg(feature = "objective-c")]
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
        #[underlying($ty:ty)]
        $(#[$m:meta])*
        $v:vis enum __anonymous__ {
            $(
                $(#[$field_m:meta])*
                $field:ident = $value:expr
            ),* $(,)?
        }
    ) => {
        extern_enum! {
            @__inner
            @($v)
            @($ty)
            @($(
                $(#[$field_m])*
                $field = $value,
            )*)
        }
    };
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
        // TODO: Improve type-safety
        $(#[$m])*
        $v type $name = $ty;

        extern_enum! {
            @__inner
            @($v)
            @($name)
            @($(
                $(#[$field_m])*
                $field = $value,
            )*)
        }
    };

    // tt-munch each field
    (
        @__inner
        @($v:vis)
        @($ty:ty)
        @()
    ) => {
        // Base case
    };
    (
        @__inner
        @($v:vis)
        @($ty:ty)
        @(
            $(#[$field_m:meta])*
            $field:ident = $value:expr,

            $($rest:tt)*
        )
    ) => {
        $(#[$field_m])*
        $v const $field: $ty = $value;

        extern_enum! {
            @__inner
            @($v)
            @($ty)
            @($($rest)*)
        }
    };
}

/// Corresponds to `NS_ENUM`
macro_rules! ns_enum {
    (
        #[underlying($ty:ty)]
        $(#[$m:meta])*
        $v:vis enum $($name:ident)? {
            $(
                $(#[$field_m:meta])*
                $field:ident = $value:expr
            ),* $(,)?
        }
    ) => {
        extern_enum! {
            #[underlying($ty)]
            $(#[$m])*
            $v enum $($name)? {
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
        // TODO: Handle this differently
        extern_enum! {
            #[underlying(NSUInteger)]
            $(#[$m])*
            $v enum $name {
                $(
                    $(#[$field_m])*
                    $field = $value
                ),*
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

/// Corresponds to `NS_ERROR_ENUM`
macro_rules! ns_error_enum {
    (
        #[underlying(NSInteger)]
        $(#[$m:meta])*
        $v:vis enum $($name:ident)? {
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
            $v enum $($name)? {
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
    ($name:ident: $ty:ty = $value:expr) => {
        pub static $name: $ty = $value;
    };
}

macro_rules! extern_fn {
    (
        $(#[$m:meta])*
        $v:vis unsafe fn $name:ident($($args:tt)*) $(-> $res:ty)?;
    ) => {
        $(#[$m])*
        extern "C" {
            $v fn $name($($args)*) $(-> $res)?;
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
        $v:vis unsafe fn $name:ident($($args:tt)*) $(-> $res:ty)? $body:block
    ) => {
        // TODO
    };
}
