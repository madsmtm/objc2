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
        () // No attributes left to process
        $($output:tt)*
    } => {
        $($output)*
    };
    // `cfg` attribute
    {
        (
            #[cfg $($args:tt)*]
            $($rest:tt)*
        )
        $($output:tt)*
    } => {
        // Apply the attribute and continue
        #[cfg $($args)*]
        {
            $crate::__extract_and_apply_cfg_attributes! {
                ($($rest)*)
                $($output)*
            }
        }
    };
    // Other attributes
    {
        (
            #[$($m_ignored:tt)*]
            $($rest:tt)*
        )
        $($output:tt)*
    } => {
        // Ignore the attribute, and continue parsing the rest
        $crate::__extract_and_apply_cfg_attributes! {
            ($($rest)*)
            $($output)*
        }
    };
}

/// Extract our custom method attributes, and send it to another macro.
///
/// Handles:
/// - `#[unsafe(method(...))]` or `#[unsafe(method_id(...))]`.
/// - `#[unsafe(method_family(...))]`.
/// - `#[optional]`.
///
/// This will ensure that there is one and only one of the `method` attributes
/// present.
#[doc(hidden)]
#[macro_export]
macro_rules! __extract_method_attributes {
    {
        // The attributes to parse.
        ($($m:tt)*)

        // The output macro.
        ($out_macro:path)

        // Further arguments to passed to the output macro.
        $($out_args:tt)*

        // The following arguments will be appended to the output macro:
        //
        // The `method` or `method_id` attribute.
        // ($method_or_method_id:ident($($sel:tt)*))
        //
        // The requested method family, if any was present. One of `new`,
        // `alloc`, `init`, `copy`, `mutableCopy` or `none`.
        // ($($method_family:tt)*)
        //
        // The `optional` attribute, if any.
        // ($(#[optional])?)
        //
        // The remaining attributes that should be placed on the method
        // definition itself.
        // ($(#[$($attr_method:tt)*])*)
        //
        // Attributes like `cfg` and `allow` that should be placed on the
        // usage site of the method.
        // ($(#[$($attr_use:tt)*])*)
    } => {
        $crate::__extract_method_attributes_inner! {
            ($($m)*)
            // No already parsed attributes
            () // method/method_id
            () // method family
            () // optional
            () // attr_method
            () // attr_use

            ($out_macro)
            $($out_args)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extract_method_attributes_inner {
    // No method/method_id attribute found
    {
        // No attributes left to process
        ()

        // And we found no `method` or `method_id` attributes
        ()
        ($($method_family:tt)*)
        ($($optional:tt)*)
        ($($attr_method:tt)*)
        ($($attr_use:tt)*)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $crate::__macros::compile_error!("must specify the desired selector using `#[unsafe(method(...))]` or `#[unsafe(method_id(...))]`");

        // Try to output anyhow, for better UI.
        $out_macro! {
            $($out_args)*
            // Append attributes to the end of the macro arguments
            (method(invalidSelector))
            ($($method_family)*)
            ($($optional)*)
            ($($attr_method)*)
            ($($attr_use)*)
        }
    };

    // Base case
    {
        // No attributes left to process
        ()

        // And we found a `method` or `method_id` attribute
        ($($method:tt)*)
        ($($method_family:tt)*)
        ($($optional:tt)*)
        ($($attr_method:tt)*)
        ($($attr_use:tt)*)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        // Output
        $out_macro! {
            $($out_args)*
            // Append attributes to the end of the macro arguments
            ($($method)*)
            ($($method_family)*)
            ($($optional)*)
            ($($attr_method)*)
            ($($attr_use)*)
        }
    };

    // `unsafe(method)` attribute
    {
        (
            #[unsafe(method($($parsed:tt)*))]
            $($rest:tt)*
        )

        ($($method:tt)*)
        ($($method_family:tt)*)
        ($($optional:tt)*)
        ($($attr_method:tt)*)
        ($($attr_use:tt)*)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $crate::__handle_duplicate!("method`/`method_id"; $($method)*);
        $crate::__extract_method_attributes_inner! {
            ($($rest)*)

            // Add method attribute
            (method($($parsed)*))
            ($($method_family)*)
            ($($optional)*)
            ($($attr_method)*)
            ($($attr_use)*)

            ($out_macro)
            $($out_args)*
        }
    };

    // `method` attribute
    {
        (
            #[method($($parsed:tt)*)]
            $($rest:tt)*
        )

        ($($method:tt)*)
        ($($method_family:tt)*)
        ($($optional:tt)*)
        ($($attr_method:tt)*)
        ($($attr_use:tt)*)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $crate::__macros::compile_error!($crate::__macros::concat!(
            "The #[method] attribute is now unsafe, and must be used as #[unsafe(method(",
            $crate::__macros::stringify!($($parsed)*),
            "))]",
        ));

        // Continue for better UI.
        $crate::__handle_duplicate!("method`/`method_id"; $($method)*);
        $crate::__extract_method_attributes_inner! {
            ($($rest)*)

            (method($($parsed)*))
            ($($method_family)*)
            ($($optional)*)
            ($($attr_method)*)
            ($($attr_use)*)

            ($out_macro)
            $($out_args)*
        }
    };

    // `unsafe(method_id)` attribute
    {
        (
            #[unsafe(method_id($($parsed:tt)*))]
            $($rest:tt)*
        )

        ($($method:tt)*)
        ($($method_family:tt)*)
        ($($optional:tt)*)
        ($($attr_method:tt)*)
        ($($attr_use:tt)*)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $crate::__handle_duplicate!("method`/`method_id"; $($method)*);
        $crate::__extract_method_attributes_inner! {
            ($($rest)*)

            // Add method_id attribute
            (method_id($($parsed)*))
            ($($method_family)*)
            ($($optional)*)
            ($($attr_method)*)
            ($($attr_use)*)

            ($out_macro)
            $($out_args)*
        }
    };

    // `method_id` attribute
    {
        (
            #[method_id($($parsed:tt)*)]
            $($rest:tt)*
        )

        ($($method:tt)*)
        ($($method_family:tt)*)
        ($($optional:tt)*)
        ($($attr_method:tt)*)
        ($($attr_use:tt)*)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $crate::__macros::compile_error!($crate::__macros::concat!(
            "The #[method_id] attribute is now unsafe, and must be used as #[unsafe(method_id(",
            $crate::__macros::stringify!($($parsed)*),
            "))]",
        ));

        // Continue for better UI.
        $crate::__handle_duplicate!("method`/`method_id"; $($method)*);
        $crate::__extract_method_attributes_inner! {
            ($($rest)*)

            (method_id($($parsed)*))
            ($($method_family)*)
            ($($optional)*)
            ($($attr_method)*)
            ($($attr_use)*)

            ($out_macro)
            $($out_args)*
        }
    };

    // `unsafe(method_family)` attribute
    {
        (
            #[unsafe(method_family = $($parsed:tt)+)]
            $($rest:tt)*
        )
        ($($method:tt)*)
        ($($method_family:tt)*)
        ($($optional:tt)*)
        ($($attr_method:tt)*)
        ($($attr_use:tt)*)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $crate::__handle_duplicate!("method_family"; $($method_family)*);
        $crate::__extract_method_attributes_inner! {
            ($($rest)*)

            ($($method)*)
            // Add method_family attribute
            ($($parsed)+)
            ($($optional)*)
            ($($attr_method)*)
            ($($attr_use)*)

            ($out_macro)
            $($out_args)*
        }
    };

    // `method_family` attribute
    {
        (
            #[method_family = $($parsed:tt)+]
            $($rest:tt)*
        )
        ($($method:tt)*)
        ($($method_family:tt)*)
        ($($optional:tt)*)
        ($($attr_method:tt)*)
        ($($attr_use:tt)*)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $crate::__macros::compile_error!($crate::__macros::concat!(
            "The #[method_family] attribute is unsafe, and must be used as #[unsafe(method_family = ",
            $crate::__macros::stringify!($($parsed)*),
            ")]",
        ));

        // Continue for better UI.
        $crate::__handle_duplicate!("method_family"; $($method_family)*);
        $crate::__extract_method_attributes_inner! {
            ($($rest)*)

            ($($method)*)
            ($($parsed)+)
            ($($optional)*)
            ($($attr_method)*)
            ($($attr_use)*)

            ($out_macro)
            $($out_args)*
        }
    };

    // `optional` attribute
    {
        (
            #[optional]
            $($rest:tt)*
        )

        ($($method:tt)*)
        ($($method_family:tt)*)
        ($($optional:tt)*)
        ($($attr_method:tt)*)
        ($($attr_use:tt)*)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $crate::__handle_duplicate!("optional"; $($optional)*);
        $crate::__extract_method_attributes_inner! {
            ($($rest)*)

            ($($method)*)
            ($($method_family)*)
            // Add optional attribute
            (#[optional])
            ($($attr_method)*)
            ($($attr_use)*)

            ($out_macro)
            $($out_args)*
        }
    };

    // Special-case `cfg(...)`
    {
        (
            #[cfg $($parsed:tt)*]
            $($rest:tt)*
        )

        ($($method:tt)*)
        ($($method_family:tt)*)
        ($($optional:tt)*)
        ($($attr_method:tt)*)
        ($($attr_use:tt)*)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $crate::__extract_method_attributes_inner! {
            ($($rest)*)

            ($($method)*)
            ($($method_family)*)
            ($($optional)*)
            (
                $($attr_method)*
                #[cfg $($parsed)*]
            )
            // Add `cfg` attributes to use site as well.
            (
                $($attr_use)*
                #[cfg $($parsed)*]
            )

            ($out_macro)
            $($out_args)*
        }
    };

    // Special-case `allow(...)`
    {
        (
            #[allow $($parsed:tt)*]
            $($rest:tt)*
        )

        ($($method:tt)*)
        ($($method_family:tt)*)
        ($($optional:tt)*)
        ($($attr_method:tt)*)
        ($($attr_use:tt)*)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $crate::__extract_method_attributes_inner! {
            ($($rest)*)

            ($($method)*)
            ($($method_family)*)
            ($($optional)*)
            (
                $($attr_method)*
                #[allow $($parsed)*]
            )
            // Add `allow` attributes to use site as well.
            (
                $($attr_use)*
                #[allow $($parsed)*]
            )

            ($out_macro)
            $($out_args)*
        }
    };

    // Other attributes.
    // (`doc`, `deprecated`, `expect/warn/deny/forbid`, `cfg_attr`,
    // `no_mangle`, `inline`, `cold`, `track_caller`, etc.)
    {
        (
            #[$($parsed:tt)*]
            $($rest:tt)*
        )

        ($($method:tt)*)
        ($($method_family:tt)*)
        ($($optional:tt)*)
        ($($attr_method:tt)*)
        ($($attr_use:tt)*)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $crate::__extract_method_attributes_inner! {
            ($($rest)*)

            ($($method)*)
            ($($method_family)*)
            ($($optional)*)
            (
                $($attr_method)*
                // The attribute is appended to the current set, since we've
                // been consuming the attributes from the front.
                #[$($parsed)*]
            )
            ($($attr_use)*)

            ($out_macro)
            $($out_args)*
        }
    };
}

/// Extract custom attributes, and forward them to another macro.
///
/// Used by `define_class!`, `extern_class!` and `extern_protocol!`.
///
/// This will ensure that there is only one of our custom attributes present.
///
/// Custom attributes support both invocation forms `#[a = b]` and `#[a(b)]`,
/// and if they are wrapped in `unsafe`, that token will be passed onwards.
#[doc(hidden)]
#[macro_export]
macro_rules! __extract_attributes {
    {
        // The attributes to parse.
        ($($attrs:tt)*)

        // The output macro.
        ($out_macro:path)

        // Further arguments to passed to the output macro.
        $($out_args:tt)*

        // The following arguments will be appended to the output macro:
        //
        // The contents of the `super` attribute + optionally the token `unsafe`.
        // (unsafe($($superclasses:path),*))
        //
        // The contents of the `thread_kind` attribute, if any.
        // ($($thread_kind:path)?)
        //
        // The contents of the `name` attribute, if any.
        // ($($name:expr)?)
        //
        // The list of paths in all `derive` attributes.
        // ($($derives:path),*)
        //
        // Attributes that should be applied to the item.
        // ($($attr_item:tt)*)
        //
        // Attributes that should be applied to item implementations.
        // ($($attr_impl:tt)*)
    } => {
        $crate::__extract_attributes_inner! {
            ($($attrs)*)

            // No already parsed attributes
            () // superclasses
            () // thread_kind
            () // name
            () // derive
            () // attr_item
            () // attr_impl

            ($out_macro)
            $($out_args)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extract_attributes_inner {
    // Base case
    {
        // No attributes left to process
        ()

        ($($superclasses:tt)*)
        ($($thread_kind:tt)*)
        ($($name:tt)*)
        ($($derives:tt)*)
        ($($attr_item:tt)*)
        ($($attr_impl:tt)*)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        // Output
        $out_macro! {
            $($out_args)*

            // Append attributes to the end of the macro arguments
            ($($superclasses)*)
            ($($thread_kind)*)
            ($($name)*)
            ($($derives)*)
            ($($attr_item)*)
            ($($attr_impl)*)
        }
    };

    // `unsafe(super(...))`
    {
        (
            #[unsafe(super($($parsed:tt)*))]
            $($rest:tt)*
        )

        ($($superclasses:tt)*)
        ($($thread_kind:tt)*)
        ($($name:tt)*)
        ($($derives:tt)*)
        ($($attr_item:tt)*)
        ($($attr_impl:tt)*)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $crate::__handle_duplicate!("super"; $($superclasses)*);
        $crate::__extract_attributes_inner! {
            ($($rest)*)

            (unsafe $($parsed)*)
            ($($thread_kind)*)
            ($($name)*)
            ($($derives)*)
            ($($attr_item)*)
            ($($attr_impl)*)

            ($out_macro)
            $($out_args)*
        }
    };
    // `unsafe(super = ...)`
    {
        (
            #[unsafe(super = $parsed:path)]
            $($rest:tt)*
        )

        ($($superclasses:tt)*)
        ($($thread_kind:tt)*)
        ($($name:tt)*)
        ($($derives:tt)*)
        ($($attr_item:tt)*)
        ($($attr_impl:tt)*)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $crate::__handle_duplicate!("super"; $($superclasses)*);
        $crate::__extract_attributes_inner! {
            ($($rest)*)

            (unsafe $parsed)
            ($($thread_kind)*)
            ($($name)*)
            ($($derives)*)
            ($($attr_item)*)
            ($($attr_impl)*)

            ($out_macro)
            $($out_args)*
        }
    };

    // `super(...)`
    {
        (
            #[super($($parsed:tt)*)]
            $($rest:tt)*
        )

        ($($superclasses:tt)*)
        ($($thread_kind:tt)*)
        ($($name:tt)*)
        ($($derives:tt)*)
        ($($attr_item:tt)*)
        ($($attr_impl:tt)*)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $crate::__handle_duplicate!("super"; $($superclasses)*);
        $crate::__extract_attributes_inner! {
            ($($rest)*)

            (safe $($parsed)*)
            ($($thread_kind)*)
            ($($name)*)
            ($($derives)*)
            ($($attr_item)*)
            ($($attr_impl)*)

            ($out_macro)
            $($out_args)*
        }
    };
    // `super = ...`
    {
        (
            #[super = $parsed:path]
            $($rest:tt)*
        )

        ($($superclasses:tt)*)
        ($($thread_kind:tt)*)
        ($($name:tt)*)
        ($($derives:tt)*)
        ($($attr_item:tt)*)
        ($($attr_impl:tt)*)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $crate::__handle_duplicate!("super"; $($superclasses)*);
        $crate::__extract_attributes_inner! {
            ($($rest)*)

            (safe $parsed)
            ($($thread_kind)*)
            ($($name)*)
            ($($derives)*)
            ($($attr_item)*)
            ($($attr_impl)*)

            ($out_macro)
            $($out_args)*
        }
    };

    // `thread_kind = ...`
    {
        (
            #[thread_kind = $($parsed:tt)+]
            $($rest:tt)*
        )

        ($($superclasses:tt)*)
        ($($thread_kind:tt)*)
        ($($name:tt)*)
        ($($derives:tt)*)
        ($($attr_item:tt)*)
        ($($attr_impl:tt)*)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $crate::__handle_duplicate!("thread_kind"; $($thread_kind)*);
        $crate::__extract_attributes_inner! {
            ($($rest)*)

            ($($superclasses)*)
            ($($parsed)+)
            ($($name)*)
            ($($derives)*)
            ($($attr_item)*)
            ($($attr_impl)*)

            ($out_macro)
            $($out_args)*
        }
    };

    // `name = ...`
    {
        (
            #[name = $($parsed:tt)+]
            $($rest:tt)*
        )

        ($($superclasses:tt)*)
        ($($thread_kind:tt)*)
        ($($name:tt)*)
        ($($derives:tt)*)
        ($($attr_item:tt)*)
        ($($attr_impl:tt)*)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $crate::__handle_duplicate!("name"; $($name)*);
        $crate::__extract_attributes_inner! {
            ($($rest)*)

            ($($superclasses)*)
            ($($thread_kind)*)
            ($($parsed)+)
            ($($derives)*)
            ($($attr_item)*)
            ($($attr_impl)*)

            ($out_macro)
            $($out_args)*
        }
    };

    // `ivars = ...` (old syntax, removed)
    {
        (
            #[ivars = $($parsed:tt)+]
            $($rest:tt)*
        )

        ($($superclasses:tt)*)
        ($($thread_kind:tt)*)
        ($($name:tt)*)
        ($($derives:tt)*)
        ($($attr_item:tt)*)
        ($($attr_impl:tt)*)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $crate::__macros::compile_error!(
            $crate::__macros::concat!(
                "the syntax for specifying instance variables has changed, move the ivars to a field in the struct instead\n",
                // For better support in rust-analyzer.
                "#[ivars = ",
                $crate::__macros::stringify!($($parsed)+),
                "] -> ivars: ",
                $crate::__macros::stringify!($($parsed)+),
            )
        );
        $crate::__extract_attributes_inner! {
            ($($rest)*)

            ($($superclasses)*)
            ($($thread_kind)*)
            ($($name)*)
            ($($derives)*)
            ($($attr_item)*)
            ($($attr_impl)*)

            ($out_macro)
            $($out_args)*
        }
    };

    // Special-case `derive(...)`
    {
        (
            #[derive($($parsed:tt)*)]
            $($rest:tt)*
        )

        ($($superclasses:tt)*)
        ($($thread_kind:tt)*)
        ($($name:tt)*)
        ($($derives:tt)*)
        ($($attr_item:tt)*)
        ($($attr_impl:tt)*)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $crate::__extract_attributes_inner! {
            ($($rest)*)

            ($($superclasses)*)
            ($($thread_kind)*)
            ($($name)*)
            // Combine all #[derive(...)] into one list.
            ($($derives)*, $($parsed)*)
            ($($attr_item)*)
            ($($attr_impl)*)

            ($out_macro)
            $($out_args)*
        }
    };

    // Special-case `cfg(...)`
    {
        (
            #[cfg $($parsed:tt)*]
            $($rest:tt)*
        )

        ($($superclasses:tt)*)
        ($($thread_kind:tt)*)
        ($($name:tt)*)
        ($($derives:tt)*)
        ($($attr_item:tt)*)
        ($($attr_impl:tt)*)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $crate::__extract_attributes_inner! {
            ($($rest)*)

            ($($superclasses)*)
            ($($thread_kind)*)
            ($($name)*)
            ($($derives)*)
            (
                $($attr_item)*
                #[cfg $($parsed)*]
            )
            // Add `cfg` attributes to implementations as well.
            (
                $($attr_impl)*
                #[cfg $($parsed)*]
            )

            ($out_macro)
            $($out_args)*
        }
    };

    // Special-case `allow(...)`
    {
        (
            #[allow $($parsed:tt)*]
            $($rest:tt)*
        )

        ($($superclasses:tt)*)
        ($($thread_kind:tt)*)
        ($($name:tt)*)
        ($($derives:tt)*)
        ($($attr_item:tt)*)
        ($($attr_impl:tt)*)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $crate::__extract_attributes_inner! {
            ($($rest)*)

            ($($superclasses)*)
            ($($thread_kind)*)
            ($($name)*)
            ($($derives)*)
            (
                $($attr_item)*
                #[allow $($parsed)*]
            )
            // Add `allow` attributes to implementations as well.
            (
                $($attr_impl)*
                #[allow $($parsed)*]
            )

            ($out_macro)
            $($out_args)*
        }
    };

    // Other attributes.
    // (`doc`, `deprecated`, `repr`, `non_exhaustive`, `expect/warn/deny/forbid`, `cfg_attr`, etc.)
    {
        (
            #[$($parsed:tt)*]
            $($rest:tt)*
        )

        ($($superclasses:tt)*)
        ($($thread_kind:tt)*)
        ($($name:tt)*)
        ($($derives:tt)*)
        ($($attr_item:tt)*)
        ($($attr_impl:tt)*)

        ($out_macro:path)
        $($out_args:tt)*
    } => {
        $crate::__extract_attributes_inner! {
            ($($rest)*)

            ($($superclasses)*)
            ($($thread_kind)*)
            ($($name)*)
            ($($derives)*)
            // Pass all other attributes onwards to the struct.
            (
                $($attr_item)*
                #[$($parsed)*]
            )
            ($($attr_impl)*)

            ($out_macro)
            $($out_args)*
        }
    };
}

/// Ensure that custom attributes do not appear twice.
///
/// NOTE: This intentionally only results in a `compile_error!`, to allow
/// subsequent macros to still output something (better for rust-analyzer).
#[doc(hidden)]
#[macro_export]
macro_rules! __handle_duplicate {
    (
        $name:literal;
        // No existing value
    ) => {
        // Success
    };
    (
        $name:literal;
        $($existing:tt)+ // Has existing value
    ) => {
        $crate::__macros::compile_error!($crate::__macros::concat!(
            "cannot specify the `",
            $name,
            "` attribute twice",
        ));
    };
}
