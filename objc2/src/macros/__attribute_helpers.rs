/// Remove the `method(...)` and `method_id(...)` attributes from a given set
/// of attributes.
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

/// Extract a `#[method(...)]` or `#[method_id(...)]` attribute and send
/// it to another macro.
///
/// This will ensure that there is one and only one of the attributes present.
///
/// This is implemented as a tt-muncher, taking the following arguments:
/// - The attributes to be processed
/// - The output macro that will be called
/// - Any extra arguments given to the output macro
/// - The `method`/`method_id` attribute that has already been processed, if
///   any (mostly internal)
///
/// And will call the output macro with the given arguments, along with the
/// following extra arguments:
/// - The `method` or `method_id` attribute.
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
    } => {{
        // Output
        $out_macro! {
            $($macro_args)*
            // Append attribute to the end of the macro arguments
            @($($m_method)*)
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
    } => {
        $crate::__extract_custom_attributes! {
            @($($rest)*)
            @($out_macro)
            @($($macro_args)*)
            // Add method attribute
            @(#[method($($args)*)])
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
    } => {
        compile_error!("cannot specify the `method`/`method_id` attribute twice!")
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
    } => {
        $crate::__extract_custom_attributes! {
            @($($rest)*)
            @($out_macro)
            @($($macro_args)*)
            // Add method_id attribute
            @(#[method_id($($args)*)])
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
    } => {
        compile_error!("cannot specify the `method`/`method_id` attribute twice!")
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
    } => {
        $crate::__extract_custom_attributes! {
            @($($rest)*)
            @($out_macro)
            @($($macro_args)*)
            @($($m_method)*)
        }
    };
}
