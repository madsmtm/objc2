/// Create a new trait to represent a protocol.
///
/// This is similar to a `@protocol` declaration in Objective-C.
///
/// See [Protocols - The Objective-C Programming Language][protocols] and
/// [Working with Protocols - Programming with Objective-C][working-with] for
/// general information about protocols in Objective-C.
///
/// This macro will create an `unsafe` trait with methods that provide access
/// to the functionality exposed by the protocol.
///
/// Conforming to the protocol can be done in two ways:
/// - For external classes, use the [`extern_conformance!`] macro.
/// - For custom classes created with the [`define_class!`] macro, implement
///   the trait inside the macro.
///
/// Objective-C has a smart feature where you can write `id<MyProtocol>`, and
/// then work with the protocol as-if it was an object; this is very similar
/// to `dyn` traits in Rust, and we implement it in a similar way, see
/// [`ProtocolObject`] for details.
///
/// [protocols]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjectiveC/Chapters/ocProtocols.html
/// [working-with]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ProgrammingWithObjectiveC/WorkingwithProtocols/WorkingwithProtocols.html
/// [`extern_conformance!`]: crate::extern_conformance
/// [`ProtocolObject`]: crate::runtime::ProtocolObject
///
///
/// # Specification
///
/// The syntax is similar enough to Rust syntax that if you invoke the macro
/// with parentheses (as opposed to curly brackets), `rustfmt` will be able to
/// format the contents.
///
/// This macro creates an `unsafe` trait with the specified methods. A default
/// implementation of the method is generated based on the selector specified
/// with `#[unsafe(method(a:selector:))]`. Similar to [`extern_methods!`], you
/// can use the `#[unsafe(method_family = ...)]` attribute to override the
/// inferred method family.
///
/// Other protocols that this protocol conforms to / inherits can be specified
/// as supertraits.
///
/// The new trait `T` is automatically implemented for
/// [`ProtocolObject<dyn T>`], which also means that [`ProtocolType`] is
/// implemented for `dyn T`.
///
/// Finally, you can use the `#[optional]` attribute to mark optional methods.
/// This currently doesn't have any effect, but probably will have one in the
/// future when implementing protocols in [`define_class!`].
///
/// This macro otherwise shares similarities with [`extern_class!`] and
/// [`extern_methods!`].
///
/// [`ProtocolObject<dyn T>`]: crate::runtime::ProtocolObject
/// [`ProtocolType`]: crate::ProtocolType
/// [`define_class!`]: crate::define_class
/// [`extern_class!`]: crate::extern_class
/// [`extern_methods!`]: crate::extern_methods
///
///
/// # Safety
///
/// The following are required for using the macro itself:
/// - The specified name must be an existing Objective-C protocol.
/// - The protocol must actually inherit/conform to the protocols specified
///   as supertraits.
///
/// Each method is annotated with `#[unsafe(method(...))]`, where you are
/// responsible for ensuring that the declaration is correct.
///
/// While the following are required when implementing the `unsafe` trait for
/// a new type:
/// - The type must represent an object that implements the protocol.
///
///
/// # Examples
///
/// Create a trait to represent the `NSItemProviderWriting` protocol (in
/// practice, you would import this from `objc2-foundation`, this is just for
/// demonstration purposes).
///
/// ```
/// use std::ffi::c_void;
/// use objc2::ffi::NSInteger;
/// use objc2::rc::Retained;
/// use objc2::runtime::{NSObject, NSObjectProtocol};
/// use objc2::extern_protocol;
/// # type NSArray<T> = T;
/// # type NSString = NSObject;
/// # type NSProgress = NSObject;
/// # type NSItemProviderRepresentationVisibility = NSInteger;
/// # #[cfg(defined_in_foundation)]
/// use objc2_foundation::{NSArray, NSString, NSProgress, NSItemProviderRepresentationVisibility};
///
/// extern_protocol!(
///     /// This comment will appear on the trait as expected.
///     //
///     // We could have named the trait something else on the Rust-side, and
///     // then used this to keep it correct from Objective-C.
///     // #[name = "NSItemProviderWriting"]
///     //
///     // SAFETY:
///     // - The name is correct.
///     // - The protocol does inherit from `NSObjectProtocol`.
///     pub unsafe trait NSItemProviderWriting: NSObjectProtocol {
///         //                                  ^^^^^^^^^^^^^^^^
///         // This protocol inherits from the `NSObject` protocol
///
///         // This method we mark as `unsafe`, since we aren't using the
///         // correct type for the completion handler.
///         #[unsafe(method(loadDataWithTypeIdentifier:forItemProviderCompletionHandler:))]
///         unsafe fn loadData(
///             &self,
///             type_identifier: &NSString,
///             completion_handler: *mut c_void,
///         ) -> Option<Retained<NSProgress>>;
///
///         // SAFETY: The method is correctly specified.
///         #[unsafe(method(writableTypeIdentifiersForItemProvider))]
///         fn writableTypeIdentifiersForItemProvider_class()
///             -> Retained<NSArray<NSString>>;
///
///         // The rest of these are optional, which means that a user of
///         // `define_class!` would not need to implement them.
///
///         // SAFETY: The method is correctly specified.
///         #[unsafe(method(writableTypeIdentifiersForItemProvider))]
///         #[optional]
///         fn writableTypeIdentifiersForItemProvider(&self)
///             -> Retained<NSArray<NSString>>;
///
///         // SAFETY: The method is correctly specified.
///         #[unsafe(method(itemProviderVisibilityForRepresentationWithTypeIdentifier:))]
///         #[optional]
///         fn itemProviderVisibilityForRepresentation_class(
///             type_identifier: &NSString,
///         ) -> NSItemProviderRepresentationVisibility;
///
///         // SAFETY: The method is correctly specified.
///         #[unsafe(method(itemProviderVisibilityForRepresentationWithTypeIdentifier:))]
///         #[optional]
///         fn itemProviderVisibilityForRepresentation(
///             &self,
///             type_identifier: &NSString,
///         ) -> NSItemProviderRepresentationVisibility;
///     }
/// );
///
/// // Types can now implement `NSItemProviderWriting`, and use the methods
/// // from it as we specified.
/// ```
///
/// See the source code of `objc2-foundation` for many more examples.
#[doc(alias = "@protocol")]
#[macro_export]
macro_rules! extern_protocol {
    (
        // The special #[name = $name:literal] attribute is supported here.
        $(#[$($attrs:tt)*])*
        $v:vis unsafe trait $protocol:ident $(: $conforms_to:ident $(+ $conforms_to_rest:ident)*)? {
            $($methods:tt)*
        }
    ) => {
        $crate::__extract_attributes! {
            ($(#[$($attrs)*])*)

            ($crate::__inner_extern_protocol)
            ($protocol)
            ($v unsafe trait $protocol $(: $conforms_to $(+ $conforms_to_rest)*)? {
                $crate::__extern_protocol_rewrite_methods! {
                    $($methods)*
                }
            })
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __inner_extern_protocol {
    (
        ($protocol:ident)
        ($protocol_definition:item)

        ($($superclasses:tt)*)
        ($($thread_kind:tt)*)
        ($($name:tt)*)
        ($($derives:tt)*)
        ($($attr_protocol:tt)*)
        ($($attr_impl:tt)*)
    ) => {
        $($attr_protocol)*
        $protocol_definition

        $($attr_impl)*
        unsafe impl<T> $protocol for $crate::runtime::ProtocolObject<T>
        where
            T: ?$crate::__macros::Sized + $protocol
        {}

        // SAFETY: The specified name is ensured by caller to be a protocol,
        // and is correctly defined.
        $($attr_impl)*
        unsafe impl $crate::ProtocolType for dyn $protocol {
            const NAME: &'static $crate::__macros::str = $crate::__fallback_if_not_set! {
                ($($name)*)
                ($crate::__macros::stringify!($protocol))
            };
            const __INNER: () = ();
        }

        // SAFETY: Anything that implements the protocol is valid to convert
        // to `ProtocolObject<dyn [PROTO]>`.
        $($attr_impl)*
        unsafe impl<T> $crate::runtime::ImplementedBy<T> for dyn $protocol
        where
            T: ?$crate::__macros::Sized + $crate::Message + $protocol
        {
            const __INNER: () = ();
        }

        // TODO: Should we also implement `ImplementedBy` for `Send + Sync`
        // types, as is done for `NSObjectProtocol`?

        $crate::__extern_protocol_check_no_super!($($superclasses)*);

        $crate::__extern_protocol_check_no_thread_kind!($($thread_kind)*);

        $crate::__extern_protocol_check_no_derives!($($derives)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_protocol_check_no_super {
    () => {};
    ($($ivars:tt)*) => {
        $crate::__macros::compile_error!("#[super] is not supported in extern_protocol!");
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_protocol_check_no_thread_kind {
    () => {};
    ($($ivars:tt)*) => {
        $crate::__macros::compile_error!(
            "#[thread_kind = ...] is not supported in extern_protocol!. Add MainThreadOnly or AnyThread bound instead"
        );
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_protocol_check_no_derives {
    () => {};
    ($($ivars:tt)*) => {
        $crate::__macros::compile_error!("#[derive(...)] is not supported in extern_protocol!");
    };
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
        $v:vis unsafe fn $name:ident($($params:tt)*) $(-> $ret:ty)?
        // TODO: Handle where bounds better
        $(where $($where:ty : $bound:path),+ $(,)?)?;

        $($rest:tt)*
    } => {
        $crate::__extract_method_attributes! {
            ($(#[$($m)*])*)

            ($crate::__extern_protocol_inner)
            ($v unsafe fn $name($($params)*) $(-> $ret)?)
            ($($($where: $bound,)+)?)
            ($($params)*)
        }

        $crate::__extern_protocol_rewrite_methods! {
            $($rest)*
        }
    };

    // Safe variant
    {
        $(#[$($m:tt)*])*
        $v:vis fn $name:ident($($params:tt)*) $(-> $ret:ty)?
        // TODO: Handle where bounds better
        $(where $($where:ty : $bound:path),+ $(,)?)?;

        $($rest:tt)*
    } => {
        $crate::__extract_method_attributes! {
            ($(#[$($m)*])*)

            ($crate::__extern_protocol_inner)
            ($v fn $name($($params)*) $(-> $ret)?)
            ($($($where: $bound,)+)?)
            ($($params)*)
        }

        $crate::__extern_protocol_rewrite_methods! {
            $($rest)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_protocol_inner {
    {
        ($($function_start:tt)*)
        ($($where:ty: $bound:path,)*)
        ($($params:tt)*)

        ($method_or_method_id:ident($($sel:tt)*))
        ($($method_family:tt)*)
        ($($optional:tt)*) // TODO: Use this?
        ($($attr_method:tt)*)
        ($($__attr_use:tt)*)
    } => {
        $crate::__parse_sel_and_params! {
            ($($sel)*)
            ($($params)*)

            ($crate::__extern_protocol_method_out)
            ($($attr_method)*)
            ($($function_start)*)
            ($($where: $bound,)*)
            ($($method_family)*)
            ($method_or_method_id($($sel)*))
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_protocol_method_out {
    {
        ($($attr_method:tt)*)
        ($($function_start:tt)*)
        ($($where:ty: $bound:path,)*)
        ($($method_family:tt)*)
        ($method_or_method_id:ident($($sel_unparsed:tt)*))

        ($fn_or_fn_result:ident($($receiver:ident: $receiver_ty:ty)?))
        ($($sel:tt)*)
        ($($param:ident,)*)
        ($($param_ty:tt)*)
        ($($ignored_param:tt)*)
    } => {
        $crate::__extern_protocol_apply_bounds! {
            ($($receiver)?)
            ($($attr_method)*)
            ($($function_start)*)
            ($($where: $bound,)*)
            ({
                $crate::__extern_methods_method_id_deprecated!($method_or_method_id($($sel_unparsed)*));

                // SAFETY: Upheld by writer of `#[unsafe(method(...))]`.
                #[allow(unused_unsafe)]
                unsafe {
                    $crate::__extern_methods_call_method! {
                        ($($method_family)*)

                        ($fn_or_fn_result($($receiver: $receiver_ty)?))
                        ($($sel)*)
                        ($($param,)*)
                        ($($param_ty)*)
                        ($($ignored_param)*)
                    }
                }
            })
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_protocol_apply_bounds {
    // Instance method.
    {
        ($receiver:ident)
        ($($attr_method:tt)*)
        ($($function_start:tt)*)
        ($($where:ty : $bound:path ,)*)
        ($body:block)
    } => {
        $($attr_method)*
        $($function_start)*
        where
            Self: $crate::__macros::Sized + $crate::Message,
            $($where : $bound,)*
        $body
    };
    // Class method.
    {
        ()
        ($($attr_method:tt)*)
        ($($function_start:tt)*)
        ($($where:ty : $bound:path ,)*)
        ($body:block)
    } => {
        $($attr_method)*
        $($function_start)*
        where
            Self: $crate::__macros::Sized + $crate::ClassType,
            $($where : $bound,)*
        $body
    };
}

#[cfg(test)]
mod tests {
    use crate::ProtocolType;

    #[test]
    fn explicit_name() {
        extern_protocol!(
            #[allow(clippy::missing_safety_doc)]
            #[name = "NSObject"]
            unsafe trait Foo {}
        );

        let proto = <dyn Foo>::protocol().unwrap();
        assert_eq!(proto.name().to_str().unwrap(), "NSObject");
        assert_eq!(<dyn Foo>::NAME, "NSObject");
    }
}
