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
    (
        $(#[$m:meta])*
        $v:vis unsafe trait $name:ident $(: $conforms_to:ident $(+ $conforms_to_rest:ident)*)? {
            $($methods:tt)*
        }

        unsafe impl ProtocolType for dyn $for:ident {
            $(const NAME: &'static str = $name_const:literal;)?
        }
    ) => {
        $(#[$m])*
        $v unsafe trait $name $(: $conforms_to $(+ $conforms_to_rest)*)? {
            $crate::__extern_protocol_rewrite_methods! {
                $($methods)*
            }
        }

        unsafe impl<T> $name for $crate::ProtocolObject<T>
        where
            T: ?$crate::__macro_helpers::Sized + $crate::ProtocolType + $name
        {}

        // SAFETY: The specified name is ensured by caller to be a protocol,
        // and is correctly defined.
        unsafe impl ProtocolType for dyn $for {
            const NAME: &'static $crate::__macro_helpers::str = $crate::__select_name!($name; $($name_const)?);
            const __INNER: () = ();
        }

        // SAFETY: Anything that implements the protocol `$name` is valid to
        // convert to `ProtocolObject<dyn $name>`.
        unsafe impl<T> $crate::ImplementedBy<T> for dyn $for
        where
            T: ?$crate::__macro_helpers::Sized + $crate::Message + $name
        {
            const __INNER: () = ();
        }
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
            ($($args)*)

            ($crate::__extract_custom_attributes)
            ($(#[$($m)*])*)
            ($name)

            ($crate::__extern_protocol_method_out)
            ($v unsafe fn $name($($args)*) $(-> $ret)?)
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
            ($($args)*)

            ($crate::__extract_custom_attributes)
            ($(#[$($m)*])*)
            ($name)

            ($crate::__extern_protocol_method_out)
            ($v fn $name($($args)*) $(-> $ret)?)
        }

        $crate::__extern_protocol_rewrite_methods! {
            $($rest)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_protocol_method_out {
    // #[method(...)]
    {
        ($($function_start:tt)*)

        ($__builder_method:ident)
        ($receiver:expr)
        ($__receiver_ty:ty)
        ($($__args_prefix:tt)*)
        ($($args_rest:tt)*)

        (#[method($($sel:tt)*)])
        ($($m_optional:tt)*)
        ($($m_checked:tt)*)
    } => {
        $($m_checked)*
        $($function_start)*
        where
            Self: $crate::__macro_helpers::Sized + $crate::ClassType {
            #[allow(unused_unsafe)]
            unsafe {
                $crate::__method_msg_send! {
                    ($receiver)
                    ($($sel)*)
                    ($($args_rest)*)

                    ()
                    ()
                }
            }
        }
    };

    // #[method_id(...)]
    {
        ($($function_start:tt)*)

        ($__builder_method:ident)
        ($receiver:expr)
        ($__receiver_ty:ty)
        ($($__args_prefix:tt)*)
        ($($args_rest:tt)*)

        (#[method_id($($sel:tt)*)])
        ($($m_optional:tt)*)
        ($($m_checked:tt)*)
    } => {
        $($m_checked)*
        $($function_start)*
        where
            Self: $crate::__macro_helpers::Sized + $crate::ClassType {
            #[allow(unused_unsafe)]
            unsafe {
                $crate::__method_msg_send_id! {
                    ($receiver)
                    ($($sel)*)
                    ($($args_rest)*)

                    ()
                    ()
                    ()
                }
            }
        }
    };
}
