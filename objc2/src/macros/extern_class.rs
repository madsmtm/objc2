/// Create a new type to represent an Objective-C class.
///
/// This is similar to an `@interface` declaration in Objective-C.
///
/// The given struct name should correspond to a valid Objective-C class,
/// whose instances have the encoding [`Encoding::Object`]. (as an example:
/// `NSAutoreleasePool` does not have this!)
///
/// You must specify the superclass of this class, similar to how you would
/// in Objective-C. Due to Rust trait limitations, specifying e.g. the
/// superclass `NSData` would not give you easy access to `NSObject`'s
/// functionality, therefore you may specify additional parts of the
/// inheritance chain.
///
/// [`Encoding::Object`]: crate::Encoding::Object
///
///
/// # Specification
///
/// This creates an opaque struct containing the superclass (which means that
/// auto traits are inherited from the superclass), and implements the
/// following traits for it to allow easier usage as an Objective-C object:
///
/// - [`RefEncode`][crate::RefEncode]
/// - [`Message`][crate::Message]
/// - [`Deref<Target = $superclass>`][core::ops::Deref]
/// - [`DerefMut`][core::ops::DerefMut]
/// - [`AsRef<$inheritance_chain>`][AsRef]
/// - [`AsMut<$inheritance_chain>`][AsMut]
/// - [`Borrow<$inheritance_chain>`][core::borrow::Borrow]
/// - [`BorrowMut<$inheritance_chain>`][core::borrow::BorrowMut]
///
/// An associated function `class` is created on the object as a convenient
/// shorthand so that you can do `MyObject::class()` instead of
/// `class!(MyObject)`.
///
/// The macro allows specifying fields on the struct, but _only_ zero-sized
/// types like [`PhantomData`] and [`declare::Ivar`] are allowed here!
///
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
/// [`NSObject`]: crate::foundation::NSObject
///
///
/// # Example
///
/// Create a new type to represent the `NSFormatter` class.
///
/// ```
/// use objc2::foundation::NSObject;
/// use objc2::rc::{Id, Shared};
/// use objc2::{extern_class, msg_send_id};
/// #
/// # #[cfg(feature = "gnustep-1-7")]
/// # unsafe { objc2::__gnustep_hack::get_class_to_force_linkage() };
///
/// extern_class! {
///     /// An example description.
///     #[derive(PartialEq, Eq, Hash)] // Uses `NSObject`'s implementation
///     // Specify class and superclass
///     // In this case the class `NSFormatter`, which subclasses `NSObject`
///     unsafe pub struct NSFormatter: NSObject;
/// }
///
/// // Provided by the macro
/// let cls = NSFormatter::class();
///
/// // `NSFormatter` implements `Message`:
/// let obj: Id<NSFormatter, Shared> = unsafe { msg_send_id![cls, new].unwrap() };
/// ```
///
/// Represent the `NSDateFormatter` class, using the `NSFormatter` type we
/// declared previously to specify as its superclass.
///
/// ```
/// use objc2::extern_class;
/// use objc2::foundation::NSObject;
/// #
/// # extern_class! {
/// #     #[derive(PartialEq, Eq, Hash)]
/// #     unsafe pub struct NSFormatter: NSObject;
/// # }
///
/// extern_class! {
///     #[derive(PartialEq, Eq, Hash)]
///     // Specify the correct inheritance chain
///     // `NSDateFormatter` subclasses `NSFormatter` which subclasses `NSObject`
///     unsafe pub struct NSDateFormatter: NSFormatter, NSObject;
/// }
/// ```
///
/// See the source code of `objc2::foundation` in general for more examples.
#[doc(alias = "@interface")]
#[macro_export]
macro_rules! extern_class {
    (
        $(#[$m:meta])*
        unsafe $v:vis struct $name:ident: $superclass:ty $(, $inheritance_rest:ty)*;
    ) => {
        // Just shorthand syntax for the following
        $crate::extern_class! {
            $(#[$m])*
            unsafe $v struct $name: $superclass $(, $inheritance_rest)* {}
        }
    };
    (
        $(#[$m:meta])*
        unsafe $v:vis struct $name:ident: $superclass:ty $(, $inheritance_rest:ty)* {
            $($field_vis:vis $field:ident: $field_ty:ty,)*
        }
    ) => {
        $crate::__inner_extern_class! {
            $(#[$m])*
            unsafe $v struct $name<>: $superclass $(, $inheritance_rest)* {
                $($field_vis $field: $field_ty,)*
            }
        }

        const _: () = {
            if $crate::__macro_helpers::size_of::<$name>() != 0 {
                panic!(concat!(
                    "the struct ",
                    stringify!($name),
                    " is not zero-sized!",
                ))
            }
        };
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_as_ref_borrow {
    ($name:ident<$($t:ident $(: $b:ident)?),*>,) => {};
    ($name:ident<$($t:ident $(: $b:ident)?),*>, $item:ty, $($tail:ty,)*) => {
        impl<$($t $(: $b)?),*> $crate::__macro_helpers::AsRef<$item> for $name<$($t),*> {
            #[inline]
            fn as_ref(&self) -> &$item {
                // Triggers Deref coercion depending on return type
                &*self
            }
        }

        impl<$($t $(: $b)?),*> $crate::__macro_helpers::AsMut<$item> for $name<$($t),*> {
            #[inline]
            fn as_mut(&mut self) -> &mut $item {
                // Triggers DerefMut coercion depending on return type
                &mut *self
            }
        }

        // Borrow and BorrowMut are correct, since subclasses behaves
        // identical to the class they inherit (message sending doesn't care).
        //
        // In particular, `Eq`, `Ord` and `Hash` all give the same results
        // after borrow.

        impl<$($t $(: $b)?),*> $crate::__macro_helpers::Borrow<$item> for $name<$($t),*> {
            #[inline]
            fn borrow(&self) -> &$item {
                // Triggers Deref coercion depending on return type
                &*self
            }
        }

        impl<$($t $(: $b)?),*> $crate::__macro_helpers::BorrowMut<$item> for $name<$($t),*> {
            #[inline]
            fn borrow_mut(&mut self) -> &mut $item {
                // Triggers Deref coercion depending on return type
                &mut *self
            }
        }

        $crate::__impl_as_ref_borrow!($name<$($t $(: $b)?),*>, $($tail,)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __inner_extern_class {
    // TODO: Expose this variant in the `object` macro.
    (
        $(#[$m:meta])*
        unsafe $v:vis struct $name:ident<$($t:ident $(: $b:ident $(= $default:ty)?)?),*>: $($inheritance_chain:ty),+ {
            $($field_vis:vis $field:ident: $field_ty:ty,)*
        }
    ) => {
        $crate::__inner_extern_class! {
            @__inner
            $(#[$m])*
            unsafe $v struct $name<$($t $(: $b $(= $default)?)?),*>: $($inheritance_chain,)+ $crate::runtime::Object {
                $($field_vis $field: $field_ty,)*
            }
        }

        impl<$($t $(: $b)?),*> $name<$($t),*> {
            #[doc = concat!(
                "Get a reference to the Objective-C class `",
                stringify!($name),
                "`.",
            )]
            #[inline]
            // TODO: Allow users to configure this?
            $v fn class() -> &'static $crate::runtime::Class {
                $crate::class!($name)
            }
        }
    };
    (
        @__inner
        $(#[$m:meta])*
        unsafe $v:vis struct $name:ident<$($t:ident $(: $b:ident $(= $default:ty)?)?),*>: $superclass:ty $(, $inheritance_rest:ty)* {
            $($field_vis:vis $field:ident: $field_ty:ty,)*
        }
    ) => {
        $(#[$m])*
        // TODO: repr(transparent) when the inner pointer is no longer a ZST.
        #[repr(C)]
        $v struct $name<$($t $(: $b $(= $default)?)?),*> {
            __inner: $superclass,
            // Additional fields (should only be zero-sized PhantomData or ivars).
            $($field_vis $field: $field_ty,)*
        }

        // SAFETY:
        // - The item is FFI-safe with `#[repr(C)]`.
        // - The encoding is taken from the inner item, and caller verifies
        //   that it actually inherits said object.
        // - The rest of the struct's fields are ZSTs, so they don't influence
        //   the layout.
        unsafe impl<$($t $(: $b)?),*> $crate::RefEncode for $name<$($t),*> {
            const ENCODING_REF: $crate::Encoding<'static>
                = <$superclass as $crate::RefEncode>::ENCODING_REF;
        }

        // SAFETY: This is essentially just a newtype wrapper over `Object`
        // (we even ensure that `Object` is always last in our inheritance
        // tree), so it is always safe to reinterpret as that.
        //
        // That the object must work with standard memory management is upheld
        // by the caller.
        unsafe impl<$($t $(: $b)?),*> $crate::Message for $name<$($t),*> {}

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
        impl<$($t $(: $b)?),*> $crate::__macro_helpers::Deref for $name<$($t),*> {
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
        impl<$($t $(: $b)?),*> $crate::__macro_helpers::DerefMut for $name<$($t),*> {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.__inner
            }
        }

        impl<$($t $(: $b)?),*> $crate::__macro_helpers::AsRef<Self> for $name<$($t),*> {
            #[inline]
            fn as_ref(&self) -> &Self {
                self
            }
        }

        impl<$($t $(: $b)?),*> $crate::__macro_helpers::AsMut<Self> for $name<$($t),*> {
            #[inline]
            fn as_mut(&mut self) -> &mut Self {
                self
            }
        }

        $crate::__impl_as_ref_borrow!($name<$($t $(: $b)?),*>, $superclass, $($inheritance_rest,)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __attribute_helper {
    // Convert a set of attributes described with `@[...]` to `#[...]`, while
    // parsing out the `sel(...)` attribute.
    {
        @strip_sel
        @[sel($($_sel_args:tt)*)]
        $(@[$($m_rest:tt)*])*

        $(#[$($m:tt)*])*
        ($($fn:tt)*)
    } => {
        $crate::__attribute_helper! {
            @strip_sel
            $(@[$($m_rest)*])*

            $(#[$($m)*])*
            ($($fn)*)
        }
    };
    {
        @strip_sel
        @[$($m_checked:tt)*]
        $(@[$($m_rest:tt)*])*

        $(#[$($m:tt)*])*
        ($($fn:tt)*)
    } => {
        $crate::__attribute_helper! {
            @strip_sel
            $(@[$($m_rest)*])*

            $(#[$($m)*])*
            #[$($m_checked)*]
            ($($fn)*)
        }
    };
    {
        @strip_sel
        $(#[$($m:tt)*])*
        ($($fn:tt)*)
    } => {
        $(#[$($m)*])*
        $($fn)*
    };

    // Extract and convert the `#[sel(...)]` attribute to a `sel!` invocation.
    {
        @extract_sel
        #[sel($($sel:tt)*)]
        $($rest:tt)*
    } => {{
        $crate::__attribute_helper! {
            @extract_sel_duplicate
            $($rest)*
        }

        $crate::sel!($($sel)*)
    }};
    {
        @extract_sel
        #[$($m_checked:tt)*]
        $($rest:tt)*
    } => {{
        $crate::__attribute_helper! {
            @extract_sel
            $($rest)*
        }
    }};
    {@extract_sel} => {{
        compile_error!("Must specify the desired selector using `#[sel(...)]`");
    }};

    {
        @extract_sel_duplicate
        #[sel($($_sel_args:tt)*)]
        $($rest:tt)*
    } => {{
        compile_error!("Cannot not specify a selector twice!");
    }};
    {
        @extract_sel_duplicate
        #[$($m_checked:tt)*]
        $($rest:tt)*
    } => {{
        $crate::__attribute_helper! {
            @extract_sel_duplicate
            $($rest)*
        }
    }};
    {@extract_sel_duplicate} => {};

}

/// Create function pointer type with inferred arguments.
///
/// TODO: Investigate if there's a better way of doing this.
#[doc(hidden)]
#[macro_export]
macro_rules! __fn_ptr {
    {
        @($($qualifiers:tt)*)
        $($param1:ident)? $(_)?: $param1_ty:ty,
        $($param2:ident)? $(_)?: $param2_ty:ty $(,)?
    } => {
        $($qualifiers)* fn(_, _) -> _
    };
    {
        @($($qualifiers:tt)*)
        $($param1:ident)? $(_)?: $param1_ty:ty,
        $($param2:ident)? $(_)?: $param2_ty:ty,
        $($param3:ident)? $(_)?: $param3_ty:ty $(,)?
    } => {
        $($qualifiers)* fn(_, _, _) -> _
    };
    {
        @($($qualifiers:tt)*)
        $($param1:ident)? $(_)?: $param1_ty:ty,
        $($param2:ident)? $(_)?: $param2_ty:ty,
        $($param3:ident)? $(_)?: $param3_ty:ty,
        $($param4:ident)? $(_)?: $param4_ty:ty $(,)?
    } => {
        $($qualifiers)* fn(_, _, _, _) -> _
    };
    {
        @($($qualifiers:tt)*)
        $($param1:ident)? $(_)?: $param1_ty:ty,
        $($param2:ident)? $(_)?: $param2_ty:ty,
        $($param3:ident)? $(_)?: $param3_ty:ty,
        $($param4:ident)? $(_)?: $param4_ty:ty,
        $($param5:ident)? $(_)?: $param5_ty:ty $(,)?
    } => {
        $($qualifiers)* fn(_, _, _, _, _) -> _
    };
    {
        @($($qualifiers:tt)*)
        $($param1:ident)? $(_)?: $param1_ty:ty,
        $($param2:ident)? $(_)?: $param2_ty:ty,
        $($param3:ident)? $(_)?: $param3_ty:ty,
        $($param4:ident)? $(_)?: $param4_ty:ty,
        $($param5:ident)? $(_)?: $param5_ty:ty,
        $($param6:ident)? $(_)?: $param6_ty:ty $(,)?
    } => {
        $($qualifiers)* fn(_, _, _, _, _, _) -> _
    };
    {
        @($($qualifiers:tt)*)
        $($param1:ident)? $(_)?: $param1_ty:ty,
        $($param2:ident)? $(_)?: $param2_ty:ty,
        $($param3:ident)? $(_)?: $param3_ty:ty,
        $($param4:ident)? $(_)?: $param4_ty:ty,
        $($param5:ident)? $(_)?: $param5_ty:ty,
        $($param6:ident)? $(_)?: $param6_ty:ty,
        $($param7:ident)? $(_)?: $param7_ty:ty $(,)?
    } => {
        $($qualifiers)* fn(_, _, _, _, _, _, _) -> _
    };
    {
        @($($qualifiers:tt)*)
        $($param1:ident)? $(_)?: $param1_ty:ty,
        $($param2:ident)? $(_)?: $param2_ty:ty,
        $($param3:ident)? $(_)?: $param3_ty:ty,
        $($param4:ident)? $(_)?: $param4_ty:ty,
        $($param5:ident)? $(_)?: $param5_ty:ty,
        $($param6:ident)? $(_)?: $param6_ty:ty,
        $($param7:ident)? $(_)?: $param7_ty:ty,
        $($param8:ident)? $(_)?: $param8_ty:ty $(,)?
    } => {
        $($qualifiers)* fn(_, _, _, _, _, _, _, _) -> _
    };
    {
        @($($qualifiers:tt)*)
        $($param1:ident)? $(_)?: $param1_ty:ty,
        $($param2:ident)? $(_)?: $param2_ty:ty,
        $($param3:ident)? $(_)?: $param3_ty:ty,
        $($param4:ident)? $(_)?: $param4_ty:ty,
        $($param5:ident)? $(_)?: $param5_ty:ty,
        $($param6:ident)? $(_)?: $param6_ty:ty,
        $($param7:ident)? $(_)?: $param7_ty:ty,
        $($param8:ident)? $(_)?: $param8_ty:ty,
        $($param9:ident)? $(_)?: $param9_ty:ty $(,)?
    } => {
        $($qualifiers)* fn(_, _, _, _, _, _, _, _, _) -> _
    };
    {
        @($($qualifiers:tt)*)
        $($param1:ident)? $(_)?: $param1_ty:ty,
        $($param2:ident)? $(_)?: $param2_ty:ty,
        $($param3:ident)? $(_)?: $param3_ty:ty,
        $($param4:ident)? $(_)?: $param4_ty:ty,
        $($param5:ident)? $(_)?: $param5_ty:ty,
        $($param6:ident)? $(_)?: $param6_ty:ty,
        $($param7:ident)? $(_)?: $param7_ty:ty,
        $($param8:ident)? $(_)?: $param8_ty:ty,
        $($param9:ident)? $(_)?: $param9_ty:ty,
        $($param10:ident)? $(_)?: $param10_ty:ty $(,)?
    } => {
        $($qualifiers)* fn(_, _, _, _, _, _, _, _, _, _) -> _
    };
    {
        @($($qualifiers:tt)*)
        $($param1:ident)? $(_)?: $param1_ty:ty,
        $($param2:ident)? $(_)?: $param2_ty:ty,
        $($param3:ident)? $(_)?: $param3_ty:ty,
        $($param4:ident)? $(_)?: $param4_ty:ty,
        $($param5:ident)? $(_)?: $param5_ty:ty,
        $($param6:ident)? $(_)?: $param6_ty:ty,
        $($param7:ident)? $(_)?: $param7_ty:ty,
        $($param8:ident)? $(_)?: $param8_ty:ty,
        $($param9:ident)? $(_)?: $param9_ty:ty,
        $($param10:ident)? $(_)?: $param10_ty:ty,
        $($param11:ident)? $(_)?: $param11_ty:ty $(,)?
    } => {
        $($qualifiers)* fn(_, _, _, _, _, _, _, _, _, _, _) -> _
    };
    {
        @($($qualifiers:tt)*)
        $($param1:ident)? $(_)?: $param1_ty:ty,
        $($param2:ident)? $(_)?: $param2_ty:ty,
        $($param3:ident)? $(_)?: $param3_ty:ty,
        $($param4:ident)? $(_)?: $param4_ty:ty,
        $($param5:ident)? $(_)?: $param5_ty:ty,
        $($param6:ident)? $(_)?: $param6_ty:ty,
        $($param7:ident)? $(_)?: $param7_ty:ty,
        $($param8:ident)? $(_)?: $param8_ty:ty,
        $($param9:ident)? $(_)?: $param9_ty:ty,
        $($param10:ident)? $(_)?: $param10_ty:ty,
        $($param11:ident)? $(_)?: $param11_ty:ty,
        $($param12:ident)? $(_)?: $param12_ty:ty $(,)?
    } => {
        $($qualifiers)* fn(_, _, _, _, _, _, _, _, _, _, _, _) -> _
    };
    {
        @($($qualifiers:tt)*)
        $($param1:ident)? $(_)?: $param1_ty:ty,
        $($param2:ident)? $(_)?: $param2_ty:ty,
        $($param3:ident)? $(_)?: $param3_ty:ty,
        $($param4:ident)? $(_)?: $param4_ty:ty,
        $($param5:ident)? $(_)?: $param5_ty:ty,
        $($param6:ident)? $(_)?: $param6_ty:ty,
        $($param7:ident)? $(_)?: $param7_ty:ty,
        $($param8:ident)? $(_)?: $param8_ty:ty,
        $($param9:ident)? $(_)?: $param9_ty:ty,
        $($param10:ident)? $(_)?: $param10_ty:ty,
        $($param11:ident)? $(_)?: $param11_ty:ty,
        $($param12:ident)? $(_)?: $param12_ty:ty,
        $($param13:ident)? $(_)?: $param13_ty:ty $(,)?
    } => {
        $($qualifiers)* fn(_, _, _, _, _, _, _, _, _, _, _, _, _) -> _
    };
    {
        @($($qualifiers:tt)*)
        $($param1:ident)? $(_)?: $param1_ty:ty,
        $($param2:ident)? $(_)?: $param2_ty:ty,
        $($param3:ident)? $(_)?: $param3_ty:ty,
        $($param4:ident)? $(_)?: $param4_ty:ty,
        $($param5:ident)? $(_)?: $param5_ty:ty,
        $($param6:ident)? $(_)?: $param6_ty:ty,
        $($param7:ident)? $(_)?: $param7_ty:ty,
        $($param8:ident)? $(_)?: $param8_ty:ty,
        $($param9:ident)? $(_)?: $param9_ty:ty,
        $($param10:ident)? $(_)?: $param10_ty:ty,
        $($param11:ident)? $(_)?: $param11_ty:ty,
        $($param12:ident)? $(_)?: $param12_ty:ty,
        $($param13:ident)? $(_)?: $param13_ty:ty,
        $($param14:ident)? $(_)?: $param14_ty:ty $(,)?
    } => {
        $($qualifiers)* fn(_, _, _, _, _, _, _, _, _, _, _, _, _, _) -> _
    };
}
