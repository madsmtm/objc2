/// Create a new type to represent an Objective-C protocol.
///
/// This is similar to a `@protocol` declaration in Objective-C.
///
/// See [Protocols - The Objective-C Programming Language][protocols] and
/// [Working with Protocols - Programming with Objective-C][working-with] for
/// general information about protocols in Objective-C.
///
/// [protocols]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjectiveC/Chapters/ocProtocols.html
/// [working-with]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ProgrammingWithObjectiveC/WorkingwithProtocols/WorkingwithProtocols.html
///
///
/// # Tradeoffs
///
/// It should come as no surprise that Objective-C and Rust are not the same
/// language! This is in particular very prominent in the way that we map
/// protocols to Rust; one could choose to map them as traits, which has some
/// upsides, but also prevents using them as objects.
///
/// If we could customize how `dyn Trait` works in Rust, then it may have been
/// beneficial to map them as traits, but as the sitation stands, we choose
/// to map them as structs that implement [`Message`], similar to how we map
/// classes. This means that you can use protocols inside [`rc::Id`], which is
/// very important for a lot of use-cases.
///
/// If you have ideas for how to improve this situation, please help us out in
/// [#291]!
///
/// [`Message`]: crate::Message
/// [`rc::Id`]: crate::rc::Id
/// [#291]: https://github.com/madsmtm/objc2/issues/291
///
///
/// # Specification
///
/// This macro shares many similarities with [`extern_class!`] and
/// [`extern_methods!`], if you are familiar with those, it should be fairly
/// straightforward to use.
///
/// It differs in a few aspects though:
/// - You can use the `#[optional]` attribute to mark optional methods. This
///   currently doesn't have any effect, but will probably in the future.
/// - Class methods are not (yet) supported.
/// - Protocols do not have a direct parent/child relationship, so specifying
///   a parent is not required. However, to make usage easier if the protocol
///   only directly conforms to one protocol, you may specify a "parent"
///   protocol that this protocol will `Deref` to.
/// - Other protocols that this protocol conforms to can be specified using
///   the `#[conforms_to(...)]` attribute, similar to `#[inherits(...)]` in
///   [`extern_class!`].
///
/// [`extern_class!`]: crate::extern_class
/// [`extern_methods!`]: crate::extern_methods
/// [`ConformsTo`]: crate::ConformsTo
///
///
/// # Safety
///
/// - The specified name must be an exisiting Objective-C protocol.
/// - The protocol must actually inherit/conform to the protocols specified in
///   `#[conforms_to(...)]`.
/// - If a parent protocol is specified, the protocol must inherit/conform to
///   said protocol.
/// - The protocol's methods must be correctly specified.
#[doc(alias = "@protocol")]
#[macro_export]
macro_rules! extern_protocol {
    // Note: We're not using `$(...)?`, that causes a "local ambiguity error".
    (
        $(#[$m:meta])*
        $v:vis struct $name:ident;

        $(#[conforms_to($($conforms_to:ty),+)])?
        unsafe impl ProtocolType for $for:ty {
            type Parent = $parent:ty;

            const NAME: &'static str = $name_const:literal;

            $($methods:tt)*
        }
    ) => {
        $crate::__extern_protocol_inner! {
            $(#[$m])*
            $v struct $name;

            #[conforms_to($($($conforms_to),+)?)]
            unsafe impl ProtocolType for $for {
                type Parent = $parent;

                const NAME: &'static str = $name_const;
            }

            impl {
                $($methods)*
            }
        }

        // SAFETY: Specifying a protocol as the parent protocol signifies that
        // it conforms to it.
        unsafe impl $crate::ConformsTo<$parent> for $for {}
    };
    (
        $(#[$m:meta])*
        $v:vis struct $name:ident;

        $(#[conforms_to($($conforms_to:ty),+)])?
        unsafe impl ProtocolType for $for:ty {
            type Parent = $parent:ty;

            $($methods:tt)*
        }
    ) => {
        $crate::__extern_protocol_inner! {
            $(#[$m])*
            $v struct $name;

            #[conforms_to($($($conforms_to),+)?)]
            unsafe impl ProtocolType for $for {
                type Parent = $parent;

                const NAME: &'static str = $crate::__macro_helpers::stringify!($name);
            }

            impl {
                $($methods)*
            }
        }

        // SAFETY: Specifying a protocol as the parent protocol signifies that
        // it conforms to it.
        unsafe impl $crate::ConformsTo<$parent> for $for {}
    };
    (
        $(#[$m:meta])*
        $v:vis struct $name:ident;

        $(#[conforms_to($($conforms_to:ty),+)])?
        unsafe impl ProtocolType for $for:ty {
            const NAME: &'static str = $name_const:literal;

            $($methods:tt)*
        }
    ) => {
        $crate::__extern_protocol_inner! {
            $(#[$m])*
            $v struct $name;

            #[conforms_to($($($conforms_to),+)?)]
            unsafe impl ProtocolType for $for {
                type Parent = $crate::runtime::Object;

                const NAME: &'static str = $name_const;
            }

            impl {
                $($methods)*
            }
        }
    };
    (
        $(#[$m:meta])*
        $v:vis struct $name:ident;

        $(#[conforms_to($($conforms_to:ty),+)])?
        unsafe impl ProtocolType for $for:ty {
            $($methods:tt)*
        }
    ) => {
        $crate::__extern_protocol_inner! {
            $(#[$m])*
            $v struct $name;

            #[conforms_to($($($conforms_to),+)?)]
            unsafe impl ProtocolType for $for {
                type Parent = $crate::runtime::Object;

                const NAME: &'static str = $crate::__macro_helpers::stringify!($name);
            }

            impl {
                $($methods)*
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_protocol_inner {
    {
        $(#[$m:meta])*
        $v:vis struct $name:ident;

        #[conforms_to($($conforms_to:ty),* $(,)?)]
        unsafe impl ProtocolType for $for:ty {
            type Parent = $parent:ty;
            const NAME: &'static str = $name_const:expr;
        }

        impl {
            $($methods:tt)*
        }
    } => {
        // Create struct that derefs to the parent
        $crate::__inner_extern_class!(
            @__inner

            $(#[$m])*
            $v struct ($name) {}

            unsafe impl () for $for {
                INHERITS = [$parent];
            }
        );

        // SAFETY: The specified name is ensured by caller to be a protocol,
        // and is correctly defined.
        unsafe impl ProtocolType for $for {
            const NAME: &'static $crate::__macro_helpers::str = $name_const;
            const __INNER: () = ();
        }

        $(
            // SAFETY: Caller ensures that the protocol actually conforms to
            // these protocols.
            unsafe impl $crate::ConformsTo<$conforms_to> for $for {}
        )*

        $crate::__impl_as_ref_borrow! {
            impl () for $for {
                fn as_ref(&self) {
                    <Self as $crate::ConformsTo<_>>::as_protocol(self)
                }
                fn as_mut(&mut self) {
                    <Self as $crate::ConformsTo<_>>::as_protocol_mut(self)
                }
            }

            @($($conforms_to,)*)
        }

        impl $for {
            $crate::__extern_protocol_rewrite_methods! {
                $($methods)*
            }
        }

        const _: () = {
            if $crate::__macro_helpers::size_of::<$name>() != 0 {
                $crate::__macro_helpers::panic!($crate::__macro_helpers::concat!(
                    "the struct ",
                    $crate::__macro_helpers::stringify!($name),
                    " is not zero-sized!",
                ))
            }
        };
    }
}

/// tt-munch each protocol method.
#[doc(hidden)]
#[macro_export]
macro_rules! __extern_protocol_rewrite_methods {
    // Base case
    {} => {};

    // Unsafe variant
    {
        $(#[$($m:tt)*])*
        $v:vis unsafe fn $name:ident($($args:tt)*) $(-> $ret:ty)?;

        $($rest:tt)*
    } => {
        $crate::__rewrite_self_arg! {
            ($crate::__extern_protocol_method_out)
            ($($args)*)
            @($(#[$($m)*])*)
            @($v unsafe fn $name($($args)*) $(-> $ret)?)
            // Macro will add:
            // @(kind)
            // @(args_start)
            // @(args_rest)
        }

        $crate::__extern_protocol_rewrite_methods! {
            $($rest)*
        }
    };

    // Safe variant
    {
        $(#[$($m:tt)*])*
        $v:vis fn $name:ident($($args:tt)*) $(-> $ret:ty)?;

        $($rest:tt)*
    } => {
        $crate::__rewrite_self_arg! {
            ($crate::__extern_protocol_method_out)
            ($($args)*)
            @($(#[$($m)*])*)
            @($v fn $name($($args)*) $(-> $ret)?)
            // Macro will add:
            // @(kind)
            // @(args_start)
            // @(args_rest)
        }

        $crate::__extern_protocol_rewrite_methods! {
            $($rest)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_protocol_method_out {
    {
        @($(#[$($m:tt)*])*)
        @($($function_start:tt)*)
        @(instance_method)
        @(
            $self_or_this:ident: $self_or_this_ty:ty,
            _: $sel_ty:ty,
        )
        @($($args_rest:tt)*)
    } => {
        $crate::__strip_custom_attributes! {
            @($(#[$($m)*])*)
            @($($function_start)* {
                #[allow(unused_unsafe)]
                unsafe {
                    $crate::__extract_custom_attributes! {
                        @($(#[$($m)*])*)
                        @($crate::__extern_protocol_method_body)
                        @(
                            @($self_or_this)
                            @($($args_rest)*)
                            // Macro will add:
                            // @(method attribute)
                            // @(optional attribute)
                        )
                        @()
                        @()
                    }
                }
            })
            @()
        }
    };
    {
        @($(#[$($m:tt)*])*)
        @($($function_start:tt)*)
        @(class_method)
        @($($args_start:tt)*)
        @($($args_rest:tt)*)
    } => {
        compile_error!("class methods are not supported in `extern_protocol!`");
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_protocol_method_body {
    {
        @($($receiver:tt)*)
        @($($args_rest:tt)*)
        @(#[method($($sel:tt)*)])
        @($($m_optional:tt)*)
    } => {
        $crate::__collect_msg_send! {
            $crate::msg_send;
            $($receiver)*;
            ($($sel)*);
            ($($args_rest)*);
        }
    };
    {
        @($($receiver:tt)*)
        @($($args_rest:tt)*)
        @(#[method_id($($sel:tt)*)])
        @($($m_optional:tt)*)
    } => {
        $crate::__collect_msg_send! {
            $crate::msg_send_id;
            $($receiver)*;
            ($($sel)*);
            ($($args_rest)*);
        }
    };
}
