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
        #[encoding_name = $encoding_name:expr]
        unsafe impl $ty:ident $(: $superclass:ty)? {}
    ) => {
        $crate::__cf_type_common!($ty);
        $crate::__cf_type_superclass!($ty $(: $superclass)?);

        // Objective-C interop
        $crate::__cf_type_objc2!(
            $ty,
            $crate::__cf_macro_helpers::Encoding::Struct($encoding_name, &[])
        );
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __cf_type_common {
    ($ty:ident) => {
        // Reflexive AsRef impl.
        impl $crate::__cf_macro_helpers::AsRef<Self> for $ty {
            #[inline]
            fn as_ref(&self) -> &Self {
                self
            }
        }

        impl $crate::__cf_macro_helpers::fmt::Debug for $ty {
            fn fmt(
                &self,
                f: &mut $crate::__cf_macro_helpers::fmt::Formatter<'_>,
            ) -> $crate::__cf_macro_helpers::fmt::Result {
                f.debug_struct($crate::__cf_macro_helpers::stringify!($ty))
                    .finish_non_exhaustive()
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __cf_type_superclass {
    // No superclass
    ($ty:ident) => {
        // NOTE: We intentionally don't implement `Deref` with
        // `Target = AnyObject` when there isn't a superclass, as we want
        // conversions to Objective-C types to be explicit.
        //
        // TODO: Maybe implement `Deref<Target = CFTypeRef>`?
    };
    // If has superclass.
    ($ty:ident: $superclass:ty) => {
        // Similar to `objc2::extern_class!`, we implement Deref for the
        // type to allow easy conversion to the super class.
        impl $crate::__cf_macro_helpers::Deref for $ty {
            type Target = $superclass;

            #[inline]
            fn deref(&self) -> &Self::Target {
                // SAFETY: It is valid to re-interpret a type as its superclass.
                unsafe { core::mem::transmute(self) }
            }
        }

        // Allow converting to superclasses.
        // Similar to `objc2::__extern_class_impl_as_ref_borrow!`.

        impl $crate::__cf_macro_helpers::AsRef<$superclass> for $ty {
            #[inline]
            fn as_ref(&self) -> &$superclass {
                self // Through Deref
            }
        }

        impl $crate::__cf_macro_helpers::Borrow<$superclass> for $ty {
            #[inline]
            fn borrow(&self) -> &$superclass {
                self // Through Deref
            }
        }
    };
}

#[cfg(feature = "objc2")]
#[doc(hidden)]
#[macro_export]
macro_rules! __cf_type_objc2 {
    ($ty:ty, $encoding:expr) => {
        // SAFETY: Caller upholds that the struct is a ZST type, and
        // represents a C struct with the given encoding.
        unsafe impl $crate::__cf_macro_helpers::RefEncode for $ty {
            const ENCODING_REF: $crate::__cf_macro_helpers::Encoding =
                $crate::__cf_macro_helpers::Encoding::Pointer(&$encoding);
        }

        // SAFETY: CF types are message-able in the Objective-C runtime.
        unsafe impl $crate::__cf_macro_helpers::Message for $ty {}

        // Allow converting to AnyObject.
        // Similar to objc2::__extern_class_impl_as_ref_borrow!
        impl $crate::__cf_macro_helpers::AsRef<$crate::__cf_macro_helpers::AnyObject> for $ty {
            #[inline]
            fn as_ref(&self) -> &$crate::__cf_macro_helpers::AnyObject {
                // SAFETY: CF types are valid to re-interpret as AnyObject.
                unsafe { $crate::__cf_macro_helpers::transmute(self) }
            }
        }

        impl $crate::__cf_macro_helpers::Borrow<$crate::__cf_macro_helpers::AnyObject> for $ty {
            #[inline]
            fn borrow(&self) -> &$crate::__cf_macro_helpers::AnyObject {
                <Self as $crate::__cf_macro_helpers::AsRef<$crate::__cf_macro_helpers::AnyObject>>::as_ref(self)
            }
        }

        // Do not implement `ClassType`, CoreFoundation objects are root
        // objects, and all inherit from the same (hidden) __NSCFType class.
        //
        // This also means that casting etc. must be implemented differently
        // for CoreFoundation objects (compare).
    };
}

#[cfg(not(feature = "objc2"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __cf_type_objc2 {
    ($($t:tt)*) => {};
}
