/// Create a new type to represent an Objective-C class.
///
/// This is similar to an `@interface` declaration in Objective-C.
///
/// The given struct name should correspond to a valid Objective-C class,
/// whose instances have the encoding [`Encoding::Object`]. (as an example:
/// `NSAutoreleasePool` does not have this!)
///
/// You must specify the superclass of this class, similar to how you would
/// in Objective-C.
///
/// Due to Rust trait limitations, specifying e.g. the superclass `NSData`
/// would not give you easy access to `NSObject`'s functionality. Therefore,
/// you may specify additional parts of the inheritance chain using the
/// `#[inherits(...)]` attribute.
///
/// [`Encoding::Object`]: crate::Encoding::Object
///
///
/// # Specification
///
/// The syntax is similar enough to Rust syntax that if you invoke the macro
/// with parentheses (as opposed to curly brackets), [`rustfmt` will be able to
/// format the contents][rustfmt-macros].
///
/// This creates an opaque struct containing the superclass (which means that
/// auto traits are inherited from the superclass), and implements the
/// following traits for it to allow easier usage as an Objective-C object:
///
/// - [`RefEncode`][crate::RefEncode]
/// - [`Message`][crate::Message]
/// - [`ClassType`][crate::ClassType]
/// - [`Deref<Target = $superclass>`][core::ops::Deref]
/// - [`DerefMut`][core::ops::DerefMut]
/// - [`AsRef<$inheritance_chain>`][AsRef]
/// - [`AsMut<$inheritance_chain>`][AsMut]
/// - [`Borrow<$inheritance_chain>`][core::borrow::Borrow]
/// - [`BorrowMut<$inheritance_chain>`][core::borrow::BorrowMut]
///
/// The macro allows specifying fields on the struct, but _only_ zero-sized
/// types like [`PhantomData`] and [`declare::Ivar`] are allowed here!
///
/// You may add a `#[cfg(...)]` attribute to the class and `ClassType` impl,
/// and then it will work as expected. Only `#[cfg(...)]` attributes are
/// supported on the `ClassType` impl!
///
/// [rustfmt-macros]: https://github.com/rust-lang/rustfmt/discussions/5437
/// [`PhantomData`]: core::marker::PhantomData
/// [`declare::Ivar`]: crate::declare::Ivar
///
///
/// # Safety
///
/// The specified superclass must be correct. The object must also respond to
/// standard memory management messages (this is upheld if [`NSObject`] is
/// part of its inheritance chain).
///
/// [`NSObject`]: crate::runtime::NSObject
///
///
/// # Examples
///
/// Create a new type to represent the `NSFormatter` class.
///
/// ```
/// use objc2::runtime::NSObject;
/// use objc2::rc::Id;
/// use objc2::{ClassType, extern_class, msg_send_id};
///
/// extern_class!(
///     /// An example description.
///     #[derive(PartialEq, Eq, Hash)] // Uses the superclass' implementation
///     // Specify the class and struct name to be used
///     pub struct NSFormatter;
///
///     // Specify the superclass, in this case `NSObject`
///     unsafe impl ClassType for NSFormatter {
///         type Super = NSObject;
///         // Optionally, specify the name of the class, if it differs from
///         // the struct name.
///         // const NAME: &'static str = "NSFormatter";
///     }
/// );
///
/// // We can specify the protocols that `NSFormatter` conforms to like this.
/// // (These should be created using the `extern_protocol!` macro).
/// //
/// // unsafe impl NSCoding for NSFormatter {}
/// // unsafe impl NSCopying for NSFormatter {}
///
/// // Provided by the implementation of `ClassType`
/// let cls = NSFormatter::class();
///
/// // `NSFormatter` implements `Message`:
/// let obj: Id<NSFormatter> = unsafe { msg_send_id![cls, new] };
/// ```
///
/// Represent the `NSDateFormatter` class, using the `NSFormatter` type we
/// declared previously to specify as its superclass.
///
/// ```
/// use objc2::runtime::NSObject;
/// use objc2::{extern_class, ClassType};
/// #
/// # extern_class!(
/// #     #[derive(PartialEq, Eq, Hash)]
/// #     pub struct NSFormatter;
/// #
/// #     unsafe impl ClassType for NSFormatter {
/// #         type Super = NSObject;
/// #     }
/// # );
///
/// extern_class!(
///     #[derive(PartialEq, Eq, Hash)]
///     pub struct NSDateFormatter;
///
///     unsafe impl ClassType for NSDateFormatter {
///         // Specify the correct inheritance chain
///         #[inherits(NSObject)]
///         type Super = NSFormatter;
///     }
/// );
///
/// // Similarly, we can specify the protocols that this implements here:
/// //
/// // unsafe impl NSCoding for NSFormatter {}
/// // unsafe impl NSCopying for NSFormatter {}
/// ```
///
/// See the source code of `icrate` for many more examples.
#[doc(alias = "@interface")]
#[macro_export]
macro_rules! extern_class {
    // No fields
    (
        $(#[$m:meta])*
        $v:vis struct $name:ident;

        $(#[$impl_m:meta])*
        unsafe impl ClassType for $for:ty {
            $(#[inherits($($inheritance_rest:ty),+)])?
            type Super = $superclass:ty;

            $(const NAME: &'static str = $name_const:literal;)?
        }
    ) => {
        // Just shorthand syntax for the following
        $crate::extern_class!(
            $(#[$m])*
            $v struct $name {}

            $(#[$impl_m])*
            unsafe impl ClassType for $for {
                $(#[inherits($($inheritance_rest),+)])?
                type Super = $superclass;

                $(const NAME: &'static str = $name_const;)?
            }
        );
    };
    (
        $(#[$m:meta])*
        $v:vis struct $name:ident {
            $($field_vis:vis $field:ident: $field_ty:ty,)*
        }

        $(#[$impl_m:meta])*
        unsafe impl ClassType for $for:ty {
            $(#[inherits($($inheritance_rest:ty),+)])?
            type Super = $superclass:ty;

            $(const NAME: &'static str = $name_const:literal;)?
        }
    ) => {
        $crate::__inner_extern_class!(
            $(#[$m])*
            $v struct $name<> {
                $($field_vis $field: $field_ty,)*
            }

            $(#[$impl_m])*
            unsafe impl<> ClassType for $for {
                $(#[inherits($($inheritance_rest),+)])?
                type Super = $superclass;

                $(const NAME: &'static str = $name_const;)?
            }
        );

        $(#[$impl_m])*
        const _: () = {
            if $crate::__macro_helpers::size_of::<$name>() != 0 {
                $crate::__macro_helpers::panic!($crate::__macro_helpers::concat!(
                    "the struct ",
                    $crate::__macro_helpers::stringify!($name),
                    " is not zero-sized!",
                ))
            }
        };
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_as_ref_borrow {
    {
        impl ($($t:tt)*) for $for:ty {
            fn as_ref($($self:tt)*) $ref:block
            fn as_mut($($self_mut:tt)*) $mut:block
        }

        @()
    } => {};
    {
        impl ($($t:tt)*) for $for:ty {
            fn as_ref($($self:tt)*) $ref:block
            fn as_mut($($self_mut:tt)*) $mut:block
        }

        @($item:ty, $($tail:ty,)*)
    } => {
        impl<$($t)*> $crate::__macro_helpers::AsRef<$item> for $for {
            #[inline]
            fn as_ref($($self)*) -> &$item $ref
        }

        impl<$($t)*> $crate::__macro_helpers::AsMut<$item> for $for {
            #[inline]
            fn as_mut($($self_mut)*) -> &mut $item $mut
        }

        // Borrow and BorrowMut are correct, since subclasses behaves
        // identical to the class they inherit (message sending doesn't care).
        //
        // In particular, `Eq`, `Ord` and `Hash` all give the same results
        // after borrow.

        impl<$($t)*> $crate::__macro_helpers::Borrow<$item> for $for {
            #[inline]
            fn borrow($($self)*) -> &$item $ref
        }

        impl<$($t)*> $crate::__macro_helpers::BorrowMut<$item> for $for {
            #[inline]
            fn borrow_mut($($self_mut)*) -> &mut $item $mut
        }

        $crate::__impl_as_ref_borrow! {
            impl ($($t)*) for $for {
                fn as_ref($($self)*) $ref
                fn as_mut($($self_mut)*) $mut
            }

            @($($tail,)*)
        }
    };
    // TODO: Expose a generic variant of the macro.
}

/// Note: While this is not public, it is still a breaking change to change
/// this, since `icrate` relies on it.
#[doc(hidden)]
#[macro_export]
macro_rules! __inner_extern_class {
    (
        $(#[$m:meta])*
        $v:vis struct $name:ident<$($t_struct:ident $(: $b_struct:ident $(= $default:ty)?)?),* $(,)?> {
            $($fields:tt)*
        }

        $(#[$impl_m:meta])*
        unsafe impl<$($t_for:ident $(: $b_for:ident)?),* $(,)?> ClassType for $for:ty {
            $(#[inherits($($inheritance_rest:ty),+ $(,)?)])?
            type Super = $superclass:ty;

            $(const NAME: &'static str = $name_const:literal;)?
        }
    ) => {
        $crate::__emit_struct! {
            ($(#[$m])*)
            ($v)
            ($name<$($t_struct $(: $b_struct $(= $default)?)?),*>)
            (
                __inner: $superclass,
                $($fields)*
            )
        }

        $crate::__extern_class_impl_traits! {
            $(#[$impl_m])*
            unsafe impl ($($t_for $(: $b_for)?),*) for $for {
                INHERITS = [$superclass, $($($inheritance_rest,)+)? $crate::runtime::Object];
            }
        }

        $(#[$impl_m])*
        unsafe impl<$($t_for $(: $b_for)?),*> ClassType for $for {
            type Super = $superclass;
            const NAME: &'static $crate::__macro_helpers::str = $crate::__select_name!($name; $($name_const)?);

            #[inline]
            fn class() -> &'static $crate::runtime::Class {
                $crate::__class_inner!(
                    $crate::__select_name!($name; $($name_const)?),
                    $crate::__hash_idents!($name),
                )
            }

            #[inline]
            fn as_super(&self) -> &Self::Super {
                &self.__inner
            }

            #[inline]
            fn as_super_mut(&mut self) -> &mut Self::Super {
                &mut self.__inner
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_class_impl_traits {
    (
        $(#[$impl_m:meta])*
        unsafe impl ($($t:tt)*) for $for:ty {
            INHERITS = [$superclass:ty $(, $inheritance_rest:ty)*];
        }
    ) => {
        // SAFETY:
        // - The item is FFI-safe with `#[repr(C)]`.
        // - The encoding is taken from the inner item, and caller verifies
        //   that it actually inherits said object.
        // - The rest of the struct's fields are ZSTs, so they don't influence
        //   the layout.
        $(#[$impl_m])*
        unsafe impl<$($t)*> $crate::RefEncode for $for {
            const ENCODING_REF: $crate::Encoding
                = <$superclass as $crate::RefEncode>::ENCODING_REF;
        }

        // SAFETY: This is essentially just a newtype wrapper over `Object`
        // (we even ensure that `Object` is always last in our inheritance
        // tree), so it is always safe to reinterpret as that.
        //
        // That the object must work with standard memory management is upheld
        // by the caller.
        $(#[$impl_m])*
        unsafe impl<$($t)*> $crate::Message for $for {}

        // SAFETY: An instance can always be _used_ in exactly the same way as
        // its superclasses (though not necessarily _constructed_ in the same
        // way, but `Deref` doesn't allow this).
        //
        // Remember; while we (the Rust side) may intentionally be forgetting
        // which instance we're holding, the Objective-C side will remember,
        // and will always dispatch to the correct method implementations.
        //
        // Any lifetime information that the object may have been holding is
        // safely kept in the returned reference.
        //
        // Generics are discarded (for example in the case of `&NSArray<T, O>`
        // to `&NSObject`), but if the generic contained a lifetime, that
        // lifetime is still included in the returned reference.
        //
        // Note that you can easily have two different variables pointing to
        // the same object, `x: &T` and `y: &T::Target`, and this would be
        // perfectly safe!
        $(#[$impl_m])*
        impl<$($t)*> $crate::__macro_helpers::Deref for $for {
            type Target = $superclass;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.__inner
            }
        }

        // SAFETY: Mutability does not change anything in the above
        // consideration, the lifetime of `&mut Self::Target` is still tied to
        // `&mut self`.
        //
        // Usually we don't want to allow `&mut` of immutable objects like
        // `NSString`, because their `NSCopying` implementation returns the
        // same object, and would violate aliasing rules.
        //
        // But `&mut NSMutableString` -> `&mut NSString` safe, since the
        // `NSCopying` implementation of `NSMutableString` is used, and that
        // is guaranteed to return a different object.
        $(#[$impl_m])*
        impl<$($t)*> $crate::__macro_helpers::DerefMut for $for {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.__inner
            }
        }

        $(#[$impl_m])*
        impl<$($t)*> $crate::__macro_helpers::AsRef<Self> for $for {
            #[inline]
            fn as_ref(&self) -> &Self {
                self
            }
        }

        $(#[$impl_m])*
        impl<$($t)*> $crate::__macro_helpers::AsMut<Self> for $for {
            #[inline]
            fn as_mut(&mut self) -> &mut Self {
                self
            }
        }

        // Assume the meta attributes are all `cfg` attributes
        $(#[$impl_m])*
        $crate::__impl_as_ref_borrow! {
            impl ($($t)*) for $for {
                fn as_ref(&self) {
                    // Triggers Deref coercion depending on return type
                    &*self
                }
                fn as_mut(&mut self) {
                    // Triggers Deref coercion depending on return type
                    &mut *self
                }
            }

            @($superclass, $($inheritance_rest,)*)
        }
    };
}
