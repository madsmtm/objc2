//! Output code for the function itself.

/// Handle implementing protocols and stripping custom attributes from
/// methods (as these are only used when registering the method).
#[doc(hidden)]
#[macro_export]
macro_rules! __define_class_output_impls {
    // This is implemented as an incremental tt-muncher.

    // Base-case.
    () => {};

    // With protocol.
    (
        $(#[$m:meta])*
        unsafe impl $protocol:ident for $for:ty {
            $($methods:tt)*
        }

        $($rest:tt)*
    ) => {
        // SAFETY: Upheld by caller.
        $(#[$m])*
        unsafe impl $protocol for $for {}

        // TODO: Move this into `impl $protocol` in the future? That would
        // allow much better checking that the signature etc. is correct, but
        // would also prevent the user from doing more complex things, and
        // would make changing a protocol method to be safe a breaking change.
        $(#[$m])*
        impl $for {
            $crate::__define_class_strip_custom_method_attributes! {
                ()
                $($methods)*
            }
        }

        $crate::__define_class_output_impls!{
            $($rest)*
        }
    };

    // Without protocol.
    (
        $(#[$m:meta])*
        impl $for:ty {
            $($methods:tt)*
        }

        $($rest:tt)*
    ) => {
        $(#[$m])*
        impl $for {
            $crate::__define_class_strip_custom_method_attributes! {
                ()
                $($methods)*
            }
        }

        $crate::__define_class_output_impls! {
            $($rest)*
        }
    };
}

/// Strip custom attributes from methods.
///
/// Things like `const` or functions with crazy signatures are not checked in
/// here, but instead checked when registering the method. This is
/// intentionally very lenient, which should give us better diagnostics and
/// Rust-Analyzer support.
///
/// This does not support custom attributes inside `#[cfg_attr(...)]`.
#[doc(hidden)]
#[macro_export]
macro_rules! __define_class_strip_custom_method_attributes {
    // This is implemented as an incremental tt-muncher that inspects all
    // attributes, strips the custom ones, and then emits the remaining
    // content.
    //
    // TODO: This can be implemented more cleanly with [RFC 3697], with that,
    // we'd wrap the contents in a `const _: () = ...`, and `use` an empty
    // attribute macro as the custom attributes:
    //
    // [RFC 3697]: https://github.com/rust-lang/rfcs/pull/3697

    // Base-case.
    (
        ($($parsed:tt)*)
    ) => {
        // With any well-formed program, there shouldn't be any "floating"
        // syntax left over here, but in case there are, emit it here for
        // better diagnostics.
        $($parsed)*
    };

    // Skip custom attributes.
    {
        ($($parsed:tt)*)
        #[unsafe(method $($_args:tt)*)]
        $($rest:tt)*
    } => {
        $crate::__define_class_strip_custom_method_attributes! {
            ($($parsed)*)
            $($rest)*
        }
    };
    {
        ($($parsed:tt)*)
        #[method $($_args:tt)*]
        $($rest:tt)*
    } => {
        $crate::__define_class_strip_custom_method_attributes! {
            ($($parsed)*)
            $($rest)*
        }
    };
    {
        ($($parsed:tt)*)
        #[unsafe(method_id $($_args:tt)*)]
        $($rest:tt)*
    } => {
        $crate::__define_class_strip_custom_method_attributes! {
            ($($parsed)*)
            $($rest)*
        }
    };
    {
        ($($parsed:tt)*)
        #[method_id $($_args:tt)*]
        $($rest:tt)*
    } => {
        $crate::__define_class_strip_custom_method_attributes! {
            ($($parsed)*)
            $($rest)*
        }
    };
    {
        ($($parsed:tt)*)
        #[unsafe(method_family $($_args:tt)*)]
        $($rest:tt)*
    } => {
        $crate::__define_class_strip_custom_method_attributes! {
            ($($parsed)*)
            $($rest)*
        }
    };
    {
        ($($parsed:tt)*)
        #[method_family $($_args:tt)*]
        $($rest:tt)*
    } => {
        $crate::__define_class_strip_custom_method_attributes! {
            ($($parsed)*)
            $($rest)*
        }
    };
    {
        ($($parsed:tt)*)
        #[optional $($_args:tt)*]
        $($rest:tt)*
    } => {
        $crate::__define_class_strip_custom_method_attributes! {
            ($($parsed)*)
            $($rest)*
        }
    };

    // Bundle up other attributes.
    {
        ($($parsed:tt)*)
        // Note: we effectively replace the `#[]` here which slightly worsens
        // some diagnostics, for example some for `#[test]`. Could be worked
        // around but it's probably not something we need to worry about.
        #[$($attr:tt)*]
        $($rest:tt)*
    } => {
        $crate::__define_class_strip_custom_method_attributes! {
            ($($parsed)* #[$($attr)*])
            $($rest)*
        }
    };

    // Output item (together with currently parsed attributes).
    {
        ($($parsed:tt)*)
        $item:item

        $($rest:tt)*
    } => {
        $($parsed)*
        $item

        // Munch the rest.
        $crate::__define_class_strip_custom_method_attributes! {
            ()
            $($rest)*
        }
    };

    // Fallback, in case the syntax could not be parsed as an `$item`.
    //
    // In theory, we could avoid the two branches above and only have this,
    // but in practice, don't want to rely on this, since it requires a lot
    // of recursion (1 for each top-level token). In practice, that'd limit us
    // to around 15-25 methods without the user raising the default recursion
    // limit (which for example is too few for Winit).
    (
        ($($parsed:tt)*)
        $extra:tt

        $($rest:tt)*
    ) => {
        $crate::__define_class_strip_custom_method_attributes! {
            // Bundle it up, and rely on it being emitted in the base case.
            ($($parsed)* $extra)
            $($rest)*
        }
    }
}
