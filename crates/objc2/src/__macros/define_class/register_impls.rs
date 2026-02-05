//! Output code for registering the function with the Objective-C runtime.
//!
//! When defining Objective-C methods, we need to go from the defined type
//! `fn(args*) -> ret` to `extern "C-unwind" fn(receiver, sel, args*) -> ret`,
//! such that it matches the calling convention expected by Objective-C.
//!
//! Additionally, we would like to do various transformations to the arguments
//! and return type to make things safer and more "Rusty" to use.
//!
//! Previously, this was done by rewriting the defined function itself, but
//! there is value in keeping the original function definition and body
//! intact, since it:
//! - Improves IDE support.
//! - Makes errors in the body (if any) clearer for the user.
//! - Avoids lifetime issues around putting `self` in a closure (which would
//!   be needed for proper control flow semantics).
//! - Allows the user to call the function without calling into Objective-C.
//!
//! Instead, we create a "wrapper" function, see [thunk.rs][super::thunk].
//!
//!
//! ## A word of warning
//!
//! This is quite difficult to do, since we need the wrapper function to be
//! statically known to the compiler, while being significantly restricted in
//! how we can modify the types involved.
//!
//! Things that looks like it may work, but doesn't:
//! - Use a non-capturing closure, and let the compiler infer the types.
//!   Doesn't work because the closure has the Rust calling convention, and
//!   there's no way to change that.
//! - Generate the wrapper function inside `ClassType::class`. Doesn't work
//!   because the original function's types reference `Self`.
//! - Generate the wrapper function as an associated type on the class, but
//!   call it by another name. Doesn't work because we can't generate
//!   identifiers in a non-proc-macro, but could otherwise be a good solution
//!   if we switch to a proc-macro.
//! - Implement a trait for functions (not function pointers) that has an
//!   associated `const` of the correct `extern "C-unwind" fn(...)` type.
//!   Doesn't work because even though function types are first-class types,
//!   we can't actually name them or implement traits for them (yet).
//! - Replace instances of `Self` syntactically with another type inside a
//!   token tree. This means that we have to recursively apply a macro to
//!   every single token. We also have to do the replacement early, because as
//!   soon as we match e.g. `$ty`, the ability to inspect the nodes is gone,
//!   which in turn blows the recursion limit. See [`replace_self_ty`] for an
//!   implementation of this idea though.
//!
//! The (current) solution implements a trait and puts the function pointer in
//! an associated `const` on that trait, and then uses some tricks to generate
//! the desired `extern "C-unwind" fn` from that.
//!
//! The main difficulty in doing this is that we need have an associated type
//! for the type of the input function, and the type needs to be fully
//! specified, but we've found some tricks for doing so, see the code.
//!
//! [`replace_self_ty`]: https://docs.rs/replace_self_ty/

/// Create and register a thunk for each method.
#[doc(hidden)]
#[macro_export]
macro_rules! __define_class_register_impls {
    // Base-case
    (
        ($builder:ident)
    ) => {};

    // With protocol
    (
        ($builder:ident)

        $(#[$($m:tt)*])*
        unsafe impl $protocol:ident for $for:ty {
            $($methods:tt)*
        }

        $($rest:tt)*
    ) => {
        $crate::__extract_and_apply_cfg_attributes! {
            ($(#[$($m)*])*)

            // Implement protocol
            #[allow(unused_mut)]
            let mut __objc2_protocol_builder = $builder.add_protocol_methods::<dyn $protocol>();

            $crate::__define_class_register_methods! {
                (__objc2_protocol_builder)
                ($for)
                ($protocol)

                $($methods)*
            }

            // Finished creating protocol; get error message if any
            __objc2_protocol_builder.finish();
        }

        $crate::__define_class_register_impls! {
            ($builder)
            $($rest)*
        }
    };

    // Without protocol
    (
        ($builder:ident)

        $(#[$($m:tt)*])*
        impl $for:ty {
            $($methods:tt)*
        }

        $($rest:tt)*
    ) => {
        $crate::__extract_and_apply_cfg_attributes! {
            ($(#[$($m)*])*)

            $crate::__define_class_register_methods! {
                ($builder)
                ($for)
                ()

                $($methods)*
            }
        }

        $crate::__define_class_register_impls! {
            ($builder)
            $($rest)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_class_register_methods {
    // Base case
    {
        ($builder:ident)
        ($for:ty)
        ($($protocol:ty)?)
    } => {};

    // Unsafe variant
    {
        ($builder:ident)
        ($for:ty)
        ($($protocol:ty)?)

        $(#[$($m:tt)*])*
        unsafe fn $name:ident($($params:tt)*) $(-> $ret:ty)? $_body:block

        $($rest:tt)*
    } => {
        $crate::__extract_method_attributes! {
            ($(#[$($m)*])*)

            ($crate::__define_class_register_method)
            ($builder)
            ($for)
            ($($protocol)?)
            (unsafe)
            ($name)
            ($($params)*)
            ($($ret)?)
        }

        $crate::__define_class_register_methods! {
            ($builder)
            ($for)
            ($($protocol)?)

            $($rest)*
        }
    };

    // Safe variant
    {
        ($builder:ident)
        ($for:ty)
        ($($protocol:ty)?)

        $(#[$($m:tt)*])*
        fn $name:ident($($params:tt)*) $(-> $ret:ty)? $_body:block

        $($rest:tt)*
    } => {
        $crate::__extract_method_attributes! {
            ($(#[$($m)*])*)

            ($crate::__define_class_register_method)
            ($builder)
            ($for)
            ($($protocol)?)
            ()
            ($name)
            ($($params)*)
            ($($ret)?)
        }

        $crate::__define_class_register_methods! {
            ($builder)
            ($for)
            ($($protocol)?)

            $($rest)*
        }
    };

    // Invalid method.
    {
        ($builder:ident)
        ($for:ty)
        ($($protocol:ty)?)

        $item:item

        $($rest:tt)*
    } => {
        $crate::__macros::compile_error!($crate::__macros::concat!(
            "not a supported method:\n",
            $crate::__macros::stringify!($item),
        ));

        $crate::__define_class_register_methods! {
            ($builder)
            ($for)
            ($($protocol)?)

            $($rest)*
        }
    };

    // Invalid item.
    {
        ($builder:ident)
        ($for:ty)
        ($($protocol:ty)?)

        $($rest:tt)*
    } => {
        $crate::__macros::compile_error!($crate::__macros::concat!(
            "invalid method:\n",
            $crate::__macros::stringify!($($rest)*),
        ));

        $($rest)*
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_class_register_method {
    {
        ($builder:ident)
        ($for:ty)
        ($($protocol:ty)?)
        ($($qualifiers:tt)*)
        ($name:ident)
        ($($params:tt)*)
        ($($ret:ty)?)

        ($method_or_method_id:ident($($sel:tt)*))
        ($($method_family:tt)*)
        ($($optional:tt)*)
        ($($__attr_method:tt)*)
        ($($attr_use:tt)*)
    } => {
        $($attr_use)*
        {
            $crate::__extern_methods_method_id_deprecated!($method_or_method_id($($sel)*));
            $crate::__define_class_invalid_selectors!($($sel)*);
            $crate::__define_class_no_optional!($($optional)*);

            $crate::__parse_sel_and_params! {
                ($($sel)*)
                ($($params)*)

                ($crate::__declare_class_register_thunk)

                ($builder)
                ($for)
                ($($protocol)?)
                ($($qualifiers)*)
                ($name)
                ($crate::__fallback_if_not_set! {
                    ($($ret)?)
                    (()) // unit return
                })
                ($($method_family)*)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_class_register_thunk {
    {
        ($builder:ident)
        ($for:ty)
        ($($protocol:ty)?)
        ($($qualifiers:tt)*)
        ($name:ident)
        ($ret:ty)
        ($($method_family:tt)*)

        ($fn_or_fn_result:ident($($_receiver:ident: $receiver_ty:ty)?))
        ($($sel:tt)*)
        ($($_param:tt)*)
        ($($param_ty:ty,)*)
        ($($ignored_param:tt)*)
    } => (
        // Helper to make a unique `ConvertDefinedFn` for each function.
        //
        // TODO: Replace with the function's type when we can do so:
        // https://github.com/rust-lang/impl-trait-initiative/issues/17
        struct __FnMarker;

        type __RetainSemantics = $crate::__method_family!(($($method_family)*) ($($sel)*));

        type __Kind<'cls> = $crate::__declare_class_get_kind!(
            $fn_or_fn_result($($_receiver: $receiver_ty)?)
        );

        impl<'__function> $crate::__macros::ConvertDefinedFn<
            '__function,
            __FnMarker,
            __RetainSemantics,
            __Kind<'_>,
        > for $for {
            // Make the input lifetimes free (but require that the output
            // lifetime is `'static`).
            //
            // See `LifetimeAssign` for details on this.
            type Func = <
                (
                    $($qualifiers)* fn(&'__function ()) -> ($($receiver_ty,)? $($param_ty,)*),
                    $ret,
                ) as $crate::__macros::LifetimeAssign
            >::OutputFn;

            // Allow in case the user's function is marked `deprecated`
            #[allow(deprecated)]
            // We made the input lifetimes smaller above, but that's fine,
            // subtyping will take care of converting the signature for us.
            const FN: Self::Func = Self::$name;
            // TODO: `<Self $(as $protocol)?>` once we emit on protocols.
        }

        unsafe {
            $crate::__declare_class_call_builder_method! {
                ($builder)
                ($crate::sel!($($sel)*))
                (<Self as $crate::__macros::ConvertDefinedFn<'_, __FnMarker, __RetainSemantics, __Kind<'_>>>::THUNK)
                ($($receiver_ty)?)
            }
        };
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_class_get_kind {
    (fn()) => {
        $crate::__macros::ClassFnKind<&'cls $crate::runtime::AnyClass>
    };
    (fn($_receiver:ident: $_receiver_ty:ty)) => {
        $crate::__macros::MethodKind
    };
    (fn_result()) => {
        $crate::__macros::ClassFnResultKind<&'cls $crate::runtime::AnyClass>
    };
    (fn_result($_receiver:ident: $_receiver_ty:ty)) => {
        $crate::__macros::MethodResultKind
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_class_call_builder_method {
    // Class method
    {
        ($builder:ident)
        ($sel:expr)
        ($fnptr:expr)
        ()
    } => {
        $builder.add_class_method($sel, $fnptr)
    };

    // Instance method
    {
        ($builder:ident)
        ($sel:expr)
        ($fnptr:expr)
        ($_receiver_ty:ty)
    } => {
        $builder.add_method($sel, $fnptr)
    };
}

// TODO: Concept of "unsafe selectors"
// - alloc
// - retain
// - release
// - autorelease

#[doc(hidden)]
#[macro_export]
macro_rules! __define_class_invalid_selectors {
    (dealloc) => {
        $crate::__macros::compile_error!(
            "`#[unsafe(method(dealloc))]` is not supported. Implement `Drop` for the type instead"
        )
    };
    ($($sel:tt)*) => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_class_no_optional {
    () => {};
    (#[optional]) => {
        $crate::__macros::compile_error!("`#[optional]` is only supported in `extern_protocol!`")
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_class_no_main_thread_marker {
    () => {};
    ($($t:tt)*) => {
        $crate::__macros::compile_error!($crate::__macros::concat!(
            "`",
            $crate::__macros::stringify!($($t)*),
            ": MainThreadMarker` is not supported in `define_class!`",
        ))
    };
}
