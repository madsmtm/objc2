/// See `README.md` for details on this macro.
macro_rules! data {
    // Parse module declarations
    (
        @($($parsed_module:ident)*)

        mod $module:ident;

        $($rest:tt)*
    ) => {
        data! {
            @($($parsed_module)* $module)

            $($rest)*
        }
    };
    (
        @($($module:ident)*)

        $($rest:tt)*
    ) => {
        $(
            #[allow(non_snake_case)]
            mod $module;
        )*

        #[allow(unused_variables)]
        pub(crate) fn apply_tweaks(config: &mut $crate::config::Config) {
            $(
                $module::apply_tweaks(config);
            )*

            __data_inner! {
                @(config)
                $($rest)*
            }
        }
    };
    ($($rest:tt)*) => {
        data! {
            @()

            $($rest)*
        }
    };
}

macro_rules! __set_mutability {
    ($data:expr;) => {};
    ($data:expr; ImmutableWithMutableSubclass<$framework:ident::$subclass:ident>) => {
        $data.mutability = $crate::stmt::Mutability::ImmutableWithMutableSubclass(
            $crate::ItemIdentifier::from_raw(
                stringify!($subclass).to_string(),
                stringify!($framework).to_string(),
            ),
        );
    };
    ($data:expr; MutableWithImmutableSuperclass<$framework:ident::$superclass:ident>) => {
        $data.mutability = $crate::stmt::Mutability::MutableWithImmutableSuperclass(
            $crate::ItemIdentifier::from_raw(
                stringify!($superclass).to_string(),
                stringify!($framework).to_string(),
            ),
        );
    };
    ($data:expr; Immutable) => {
        $data.mutability = $crate::stmt::Mutability::Immutable;
    };
    ($data:expr; Mutable) => {
        $data.mutability = $crate::stmt::Mutability::Mutable;
    };
}

macro_rules! __data_inner {
    // Base case
    (
        @($config:expr)
    ) => {};
    // Class
    (
        @($config:expr)

        class $class:ident $(: $mutability:ident $(<$framework:ident::$ty:ident>)?)? {
            $($methods:tt)*
        }

        $($rest:tt)*
    ) => {
        #[allow(unused_mut)]
        let mut data = $config.class_data.entry(stringify!($class).to_string()).or_default();

        __set_mutability! {
            data;
            $($mutability $(<$framework::$ty>)?)?
        }

        __data_methods! {
            @(data)
            $($methods)*
        }

        __data_inner! {
            @($config)
            $($rest)*
        }
    };
    // Protocol
    (
        @($config:expr)

        protocol $protocol:ident {
            $($methods:tt)*
        }

        $($rest:tt)*
    ) => {
        #[allow(unused_mut)]
        let mut data = $config.protocol_data.entry(stringify!($protocol).to_string()).or_default();

        __data_methods! {
            @(data)
            $($methods)*
        }

        __data_inner! {
            @($config)
            $($rest)*
        }
    };
    // Function
    (
        @($config:expr)

        unsafe fn $function:ident;

        $($rest:tt)*
    ) => {
        #[allow(unused_mut)]
        let mut data = $config.fns.entry(stringify!($function).to_string()).or_default();

        data.unsafe_ = false;

        __data_inner! {
            @($config)
            $($rest)*
        }
    }
}

macro_rules! __data_methods {
    // Base case
    (
        @($data:expr)
    ) => {};
    // Mark method as safe
    (
        @($data:expr)

        unsafe -$name:ident;

        $($rest:tt)*
    ) => {
        let mut method_data = $data.methods.entry(stringify!($name).to_string()).or_default();

        method_data.unsafe_ = false;

        __data_methods! {
            @($data)
            $($rest)*
        }
    };
    // Mark class method as safe
    (
        @($data:expr)

        unsafe +$name:ident;

        $($rest:tt)*
    ) => {
        // TODO
        __data_methods! {
            @($data)
            unsafe -$name;

            $($rest)*
        }
    };
    // Mark method as mutable
    (
        @($data:expr)

        mut -$name:ident;

        $($rest:tt)*
    ) => {
        let mut method_data = $data.methods.entry(stringify!($name).to_string()).or_default();

        method_data.mutating = true;

        __data_methods! {
            @($data)
            $($rest)*
        }
    };
    // Mark class method as mutable
    (
        @($data:expr)

        mut +$name:ident;

        $($rest:tt)*
    ) => {
        // TODO
        __data_methods! {
            @($data)
            mut -$name;

            $($rest)*
        }
    };
    // Mark method as safe and mutable
    (
        @($data:expr)

        unsafe mut -$name:ident;

        $($rest:tt)*
    ) => {
        let mut method_data = $data.methods.entry(stringify!($name).to_string()).or_default();

        method_data.unsafe_ = false;
        method_data.mutating = true;

        __data_methods! {
            @($data)
            $($rest)*
        }
    };
    // Mark class method as safe and mutable
    (
        @($data:expr)

        unsafe mut +$name:ident;

        $($rest:tt)*
    ) => {
        // TODO
        __data_methods! {
            @($data)
            unsafe mut -$name;

            $($rest)*
        }
    };
}
