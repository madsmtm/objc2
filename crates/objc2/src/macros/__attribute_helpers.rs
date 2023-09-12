/// Parse the given attributes, and gate the output on any `cfg` attributes
/// that were present in the set.
///
/// This is implemented as a tt-muncher, taking the following arguments:
/// - The attributes to be processed
/// - The output that the `cfg` attributes will be attached to
#[doc(hidden)]
#[macro_export]
macro_rules! __extract_and_apply_cfg_attributes {
    // Base case
    {
        @() // No attributes left to process
        @($($output:tt)*)
    } => {
        $($output)*
    };
    // `cfg` attribute
    {
        @(
            #[cfg $($args:tt)*]
            $($m_rest:tt)*
        )
        @($($output:tt)*)
    } => {
        // Apply the attribute and continue
        #[cfg $($args)*]
        {
            $crate::__extract_and_apply_cfg_attributes! {
                @($($m_rest)*)
                @($($output)*)
            }
        }
    };
    // Other attributes
    {
        @(
            #[$($m_ignored:tt)*]
            $($m_rest:tt)*
        )
        @($($output:tt)*)
    } => {
        // Ignore the attribute, and continue parsing the rest
        $crate::__extract_and_apply_cfg_attributes! {
            @($($m_rest)*)
            @($($output)*)
        }
    };
}

/// Extract `#[method(...)]` or `#[method_id(...)]` and the `#[optional]`
/// attribute, and send it to another macro.
///
/// This will ensure that there is one and only one of the method attributes
/// present.
///
/// This takes the following arguments:
/// 1. The attributes to parse.
///    ($($m:tt)*)
///
/// 2. The output macro.
///    ($out_macro:path)
///
/// Further arguments are passed on to the output macro, with the following
/// arguments appended to it:
/// 1. The `method` or `method_id` attribute.
///    (#[$method_or_method_id:ident($($sel:tt)*)])
///
/// 2. The `optional` attribute, if any.
///    ($(#[optional])?)
///
/// 3. The rest of the attributes.
///    ($(#[$($m:tt)*])*)
#[doc(hidden)]
#[macro_export]
macro_rules! __extract_custom_attributes {
    {
        ($($m:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__extract_custom_attributes_inner! {
            ($($m)*)
            // No already parsed attributes
            ()
            ()
            ()

            ($out_macro)
            $($macro_args)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extract_custom_attributes_inner {
    // No method/method_id attribute found
    {
        // No attributes left to process
        ()
        // And we found no `method` or `method_id` attributes
        ()
        ($($m_optional:tt)*)
        ($($m_checked:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        compile_error!("must specify the desired selector using `#[method(...)]` or `#[method_id(...)]`");
    };

    // Base case
    {
        // No attributes left to process
        ()
        // And we found a `method` or `method_id` attribute
        ($($m_method:tt)*)
        ($($m_optional:tt)*)
        ($($m_checked:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        // Output
        $out_macro! {
            $($macro_args)*
            // Append attributes to the end of the macro arguments
            ($($m_method)*)
            ($($m_optional)*)
            ($($m_checked)*)
        }
    };

    // `method` attribute
    {
        (
            #[method($($args:tt)*)]
            $($rest:tt)*
        )
        // If no existing `method` nor `method_id` attributes exist
        ()
        ($($m_optional:tt)*)
        ($($m_checked:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__extract_custom_attributes_inner! {
            ($($rest)*)
            // Add method attribute
            (#[method($($args)*)])
            ($($m_optional)*)
            ($($m_checked)*)

            ($out_macro)
            $($macro_args)*
        }
    };
    // Duplicate `method` attributes
    {
        (
            #[method($($args:tt)*)]
            $($rest:tt)*
        )
        ($($m_method:tt)*)
        ($($m_optional:tt)*)
        ($($m_checked:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        compile_error!("cannot specify the `method`/`method_id` attribute twice");
    };

    // `method_id` attribute
    {
        (
            #[method_id($($args:tt)*)]
            $($rest:tt)*
        )
        // If no existing `method` nor `method_id` attributes exist
        ()
        ($($m_optional:tt)*)
        ($($m_checked:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__extract_custom_attributes_inner! {
            ($($rest)*)
            // Add method_id attribute
            (#[method_id($($args)*)])
            ($($m_optional)*)
            ($($m_checked)*)

            ($out_macro)
            $($macro_args)*
        }
    };
    // Duplicate `method` attributes
    {
        (
            #[method_id($($args:tt)*)]
            $($rest:tt)*
        )
        ($($m_method:tt)*)
        ($($m_optional:tt)*)
        ($($m_checked:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        compile_error!("cannot specify the `method`/`method_id` attribute twice");
    };

    // `optional` attribute
    {
        (
            #[optional]
            $($rest:tt)*
        )
        ($($m_method:tt)*)
        // If no existing `optional` attributes exist
        ()
        ($($m_checked:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__extract_custom_attributes_inner! {
            ($($rest)*)
            ($($m_method)*)
            // Add optional attribute
            (#[optional])
            ($($m_checked)*)

            ($out_macro)
            $($macro_args)*
        }
    };
    // Duplicate `optional` attributes
    {
        (
            #[optional]
            $($rest:tt)*
        )
        ($($m_method:tt)*)
        ($($m_optional:tt)*)
        ($($m_checked:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        compile_error!("cannot specify the `optional` attribute twice");
    };

    // Other attributes
    {
        (
            #[$($checked:tt)*]
            $($rest:tt)*
        )
        ($($m_method:tt)*)
        ($($m_optional:tt)*)
        ($($m_checked:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__extract_custom_attributes_inner! {
            ($($rest)*)
            ($($m_method)*)
            ($($m_optional)*)
            (
                $($m_checked)*
                // The attribute is appended to the current set, since we've
                // been consuming the attributes from the front.
                #[$($checked)*]
            )

            ($out_macro)
            $($macro_args)*
        }
    };
}
