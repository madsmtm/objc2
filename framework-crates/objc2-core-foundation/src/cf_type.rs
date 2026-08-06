/// Implement necessary traits for the given type to act as a CoreFoundation
/// type.
///
///
/// # Stability
///
/// This is work in progress. The macro syntax will not change in
/// semver-incompatible versions (as other crates rely on this macro), but you
/// are not expected to use the macro yourself, and so error messages and
/// changelog notes may be absent.
///
///
/// # Safety
///
/// The type must be a type that represents a CoreFoundation type, and the
/// type must be declared as either an [`extern type`], or as a ZST with an
/// appropriate `#[repr(...)]`.
///
/// [`extern type`]: https://github.com/rust-lang/rust/issues/43467
///
///
/// # Generics
///
/// It is an explicit non-goal for this macro to support generic types, as
/// neither Swift nor Objective-C supports that for CoreFoundation types
/// either (and thus we wouldn't have any useful type-information in the
/// headers).
#[doc(hidden)] // For now, though still a breaking change to modify
#[macro_export]
macro_rules! cf_type {
    (
        unsafe impl $(<$($generic:ident : ?$sized:ident),* $(,)?>)? $ty:ident $(<$($generic_param:ident),* $(,)?>)? $(: $superclass:ident $(<$($superclass_generic_param:ident),* $(,)?>)?)? {}
    ) => {
        // Reflexive AsRef impl.
        impl $(<$($generic : ?$sized),*>)? $crate::__core::convert::AsRef<Self> for $ty $(<$($generic_param),*>)? {
            #[inline]
            fn as_ref(&self) -> &Self {
                self
            }
        }

        // SAFETY: The type is a CoreFoundation-like type.
        unsafe impl $(<$($generic : ?$sized),*>)? $crate::Type for $ty $(<$($generic_param),*>)? {}

        // Implement Deref-chain to CFType.
        $crate::__cf_type_superclass! {
            ($($($generic : ?$sized),*)?)
            ($ty ($($($generic_param),*)?))
            ($($superclass ($($($superclass_generic_param),*)?))?)
        }

        // Various trait impls.

        impl $(<$($generic : ?$sized + 'static),*>)? $crate::__core::convert::AsRef<$crate::CFType> for $ty $(<$($generic_param),*>)? {
            #[inline]
            fn as_ref(&self) -> &$crate::CFType {
                self // Through Deref of self or superclass
            }
        }

        impl $(<$($generic : ?$sized + 'static),*>)? $crate::__core::borrow::Borrow<$crate::CFType> for $ty $(<$($generic_param),*>)? {
            #[inline]
            fn borrow(&self) -> &$crate::CFType {
                self // Through Deref of self or superclass
            }
        }

        impl $(<$($generic : ?$sized),*>)? $crate::__core::cmp::PartialEq for $ty $(<$($generic_param),*>)? {
            #[inline]
            fn eq(&self, other: &Self) -> $crate::__core::primitive::bool {
                // SAFETY: The type is alive for as long as the `&CFType` is,
                // so it's fine that we've erased the generics' lifetimes.
                let this: &$crate::CFType = unsafe { $crate::__core::mem::transmute(self) };
                // SAFETY: Same as above.
                let other: &$crate::CFType = unsafe { $crate::__core::mem::transmute(other) };
                $crate::__core::cmp::PartialEq::eq(this, other)
            }
        }

        impl $(<$($generic : ?$sized),*>)? $crate::__core::cmp::Eq for $ty $(<$($generic_param),*>)? {}

        impl $(<$($generic : ?$sized),*>)? $crate::__core::hash::Hash for $ty $(<$($generic_param),*>)? {
            #[inline]
            fn hash<H: $crate::__core::hash::Hasher>(&self, state: &mut H) {
                // SAFETY: Same as in PartialEq.
                let this: &$crate::CFType = unsafe { $crate::__core::mem::transmute(self) };
                $crate::__core::hash::Hash::hash(this, state);
            }
        }

        impl $(<$($generic : ?$sized),*>)? $crate::__core::fmt::Debug for $ty $(<$($generic_param),*>)? {
            fn fmt(
                &self,
                f: &mut $crate::__core::fmt::Formatter<'_>,
            ) -> $crate::__core::fmt::Result {
                // SAFETY: Same as in PartialEq.
                let this: &$crate::CFType = unsafe { $crate::__core::mem::transmute(self) };
                $crate::__core::fmt::Debug::fmt(this, f)
            }
        }
    };
}

/// Add conversion methods to the superclass (or just `Deref` to `CFType` if
/// there is no superclass).
///
/// This requires `'static` generics when the superclass being converted to
/// erases those. So e.g. `CFArray<T> -> CFType` requires `'static`, but
/// `CFMutableArray<T> -> CFArray<T>` doesn't.
#[doc(hidden)]
#[macro_export]
macro_rules! __cf_type_superclass {
    // No superclass
    {
        ($($($generic:ident : ?$sized:ident),+)?)
        ($ty:ident ($($($generic_param:ident),+)?))
        ()
    } => {
        // NOTE: We intentionally don't implement `Deref` with
        // `Target = AnyObject` when there isn't a superclass, as we want
        // conversions to Objective-C types to be explicit.
        //
        // Instead, we prefer a `Deref` impl to `CFType`.
        impl $(<$($generic : ?$sized + 'static),+>)? $crate::__core::ops::Deref for $ty $(<$($generic_param),+>)? {
            type Target = $crate::CFType;

            #[inline]
            fn deref(&self) -> &Self::Target {
                // SAFETY: It is valid to re-interpret a type as CFType.
                unsafe { $crate::__core::mem::transmute(self) }
            }
        }
    };

    // If has superclass without generics.
    {
        ($($($generic:ident : ?$sized:ident),+)?)
        ($ty:ident ($($($generic_param:ident),+)?))
        ($superclass:ident ())
    } => {
        // Similar to `objc2::extern_class!`, we implement Deref for the
        // type to allow easy conversion to the super class.
        //
        // This requires `'static` because the generics are deleted.
        impl $(<$($generic : ?$sized + 'static),*>)? $crate::__core::ops::Deref for $ty $(<$($generic_param),+>)? {
            type Target = $superclass;

            #[inline]
            fn deref(&self) -> &Self::Target {
                // SAFETY: It is valid to re-interpret a type as its superclass.
                unsafe { $crate::__core::mem::transmute(self) }
            }
        }

        // Allow converting to superclasses.
        // Similar to `objc2::__extern_class_impl_as_ref_borrow!`.

        impl $(<$($generic : ?$sized + 'static),*>)? $crate::__core::convert::AsRef<$superclass> for $ty $(<$($generic_param),+>)? {
            #[inline]
            fn as_ref(&self) -> &$superclass {
                self // Through Deref
            }
        }

        impl $(<$($generic : ?$sized + 'static),*>)? $crate::__core::borrow::Borrow<$superclass> for $ty $(<$($generic_param),+>)? {
            #[inline]
            fn borrow(&self) -> &$superclass {
                self // Through Deref
            }
        }
    };

    // If has superclass with generics.
    // Does not require `'static`.
    {
        ($($($generic:ident : ?$sized:ident),+)?)
        ($ty:ident ($($($generic_param:ident),+)?))
        ($superclass:ident ($($superclass_generic_param:ident),+))
    } => {
        impl $(<$($generic : ?$sized),*>)? $crate::__core::ops::Deref for $ty $(<$($generic_param),+>)? {
            type Target = $superclass<$($superclass_generic_param),+>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                // SAFETY: It is valid to re-interpret a type as its superclass.
                unsafe { $crate::__core::mem::transmute(self) }
            }
        }

        // We require that $generic_param == $superclass_generic_param, check
        // that here by setting different params in the AsRef<T> and as the
        // return type of as_ref.
        impl $(<$($generic : ?$sized),*>)? $crate::__core::convert::AsRef<$superclass<$($superclass_generic_param),+>> for $ty $(<$($generic_param),+>)? {
            #[inline]
            fn as_ref(&self) -> &$superclass<$($($generic_param),+)?> {
                self // Through Deref
            }
        }

        impl $(<$($generic : ?$sized),*>)? $crate::__core::borrow::Borrow<$superclass<$($superclass_generic_param),+>> for $ty $(<$($generic_param),+>)? {
            #[inline]
            fn borrow(&self) -> &$superclass<$($($generic_param),+)?> {
                self // Through Deref
            }
        }
    };
}
