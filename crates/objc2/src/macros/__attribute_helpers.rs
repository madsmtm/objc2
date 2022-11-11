/// Remove the `method(...)`, `method_id(...)` and `optional` attributes from
/// a given set of attributes.
///
/// This is implemented as a tt-muncher, taking the following arguments:
/// - The attributes to be processed
/// - The output that the attributes will be attached to
/// - The attributes that have already been processed (mostly internal)
#[doc(hidden)]
#[macro_export]
macro_rules! __strip_custom_attributes {
    // Base case
    {
        @() // No attributes left to process
        @($($output:tt)*)
        @($($m_done:tt)*)
    } => {
        // Output
        $($m_done)*
        $($output)*
    };
    // `method` attribute
    {
        @(
            #[method $($args:tt)*]
            $($m_rest:tt)*
        )
        @($($output:tt)*)
        @($($m_done:tt)*)
    } => {
        $crate::__strip_custom_attributes! {
            @($($m_rest)*)
            @($($output)*)
            @($($m_done)*)
        }
    };
    // `method_id` attribute
    {
        @(
            #[method_id $($args:tt)*]
            $($m_rest:tt)*
        )
        @($($output:tt)*)
        @($($m_done:tt)*)
    } => {
        $crate::__strip_custom_attributes! {
            @($($m_rest)*)
            @($($output)*)
            @($($m_done)*)
        }
    };
    // `optional` attribute
    {
        @(
            #[optional $($args:tt)*]
            $($m_rest:tt)*
        )
        @($($output:tt)*)
        @($($m_done:tt)*)
    } => {
        $crate::__strip_custom_attributes! {
            @($($m_rest)*)
            @($($output)*)
            @($($m_done)*)
        }
    };
    // Other attributes
    {
        @(
            #[$($m_checked:tt)*]
            $($m_rest:tt)*
        )
        @($($output:tt)*)
        @($($m_done:tt)*)
    } => {
        $crate::__strip_custom_attributes! {
            @($($m_rest)*)
            @($($output)*)
            @(
                $($m_done)*
                // The attribute is appended to the current set, since we've
                // been consuming the attributes from the front.
                #[$($m_checked)*]
            )
        }
    };
}

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
        // Simply ignore the attribute, and continue parsing the rest
        $crate::__extract_and_apply_cfg_attributes! {
            @($($m_rest)*)
            @($($output)*)
        }
    };
}

/// Extract a `#[method(...)]` or `#[method_id(...)]` and the `#[optional]`
/// attribute, and send it to another macro.
///
/// This will ensure that there is one and only one of the method attributes
/// present.
///
/// This is implemented as a tt-muncher, taking the following arguments:
/// - The attributes to be processed
/// - The output macro that will be called
/// - Any extra arguments given to the output macro
/// - The `method`/`method_id` attribute that has already been processed, if
///   any (mostly internal)
/// - The `optional` attribute that has already been processed, if any (mostly
///   internal)
///
/// And will call the output macro with the given arguments, along with the
/// following extra arguments:
/// - The `method` or `method_id` attribute.
/// - The `optional` attribute, if any.
#[doc(hidden)]
#[macro_export]
macro_rules! __extract_custom_attributes {
    // No method/method_id attribute found
    {
        // No attributes left to process
        @()
        @($out_macro:path)
        @($($macro_args:tt)*)
        // And we found no `method` or `method_id` attributes
        @()
        @($($m_optional:tt)*)
    } => {
        compile_error!("must specify the desired selector using `#[method(...)]` or `#[method_id(...)]`")
    };
    // Base case
    {
        // No attributes left to process
        @()
        @($out_macro:path)
        @($($macro_args:tt)*)
        // And we found a `method` or `method_id` attribute
        @($($m_method:tt)*)
        @($($m_optional:tt)*)
    } => {{
        // Output
        $out_macro! {
            $($macro_args)*
            // Append attributes to the end of the macro arguments
            @($($m_method)*)
            @($($m_optional)*)
        }
    }};

    // `method` attribute
    {
        @(
            #[method($($args:tt)*)]
            $($rest:tt)*
        )
        @($out_macro:path)
        @($($macro_args:tt)*)
        // If no existing `method` nor `method_id` attributes exist
        @()
        @($($m_optional:tt)*)
    } => {
        $crate::__extract_custom_attributes! {
            @($($rest)*)
            @($out_macro)
            @($($macro_args)*)
            // Add method attribute
            @(#[method($($args)*)])
            @($($m_optional)*)
        }
    };
    // Duplicate `method` attributes
    {
        @(
            #[method($($args:tt)*)]
            $($rest:tt)*
        )
        @($out_macro:path)
        @($($macro_args:tt)*)
        @($($m_method:tt)*)
        @($($m_optional:tt)*)
    } => {
        compile_error!("cannot specify the `method`/`method_id` attribute twice")
    };

    // `method_id` attribute
    {
        @(
            #[method_id($($args:tt)*)]
            $($rest:tt)*
        )
        @($out_macro:path)
        @($($macro_args:tt)*)
        // If no existing `method` nor `method_id` attributes exist
        @()
        @($($m_optional:tt)*)
    } => {
        $crate::__extract_custom_attributes! {
            @($($rest)*)
            @($out_macro)
            @($($macro_args)*)
            // Add method_id attribute
            @(#[method_id($($args)*)])
            @($($m_optional)*)
        }
    };
    // Duplicate `method` attributes
    {
        @(
            #[method_id($($args:tt)*)]
            $($rest:tt)*
        )
        @($out_macro:path)
        @($($macro_args:tt)*)
        @($($m_method:tt)*)
        @($($m_optional:tt)*)
    } => {
        compile_error!("cannot specify the `method`/`method_id` attribute twice")
    };

    // `optional` attribute
    {
        @(
            #[optional]
            $($rest:tt)*
        )
        @($out_macro:path)
        @($($macro_args:tt)*)
        @($($m_method:tt)*)
        // If no existing `optional` attributes exist
        @()
    } => {
        $crate::__extract_custom_attributes! {
            @($($rest)*)
            @($out_macro)
            @($($macro_args)*)
            @($($m_method)*)
            // Add optional attribute
            @(#[optional])
        }
    };
    // Duplicate `optional` attributes
    {
        @(
            #[optional]
            $($rest:tt)*
        )
        @($out_macro:path)
        @($($macro_args:tt)*)
        @($($m_method:tt)*)
        @($($m_optional:tt)*)
    } => {
        compile_error!("cannot specify the `optional` attribute twice")
    };

    // Other attributes
    {
        @(
            #[$($m_checked:tt)*]
            $($rest:tt)*
        )
        @($out_macro:path)
        @($($macro_args:tt)*)
        @($($m_method:tt)*)
        @($($m_optional:tt)*)
    } => {
        $crate::__extract_custom_attributes! {
            @($($rest)*)
            @($out_macro)
            @($($macro_args)*)
            @($($m_method)*)
            @($($m_optional)*)
        }
    };
}
