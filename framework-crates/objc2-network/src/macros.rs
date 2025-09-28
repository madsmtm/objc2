#[macro_export]
macro_rules! nw_object {
    (
        $(#[$m:meta])*
        pub struct $type:ident;
    ) => {
        $(#[$m])*
        #[repr(C)]
        pub struct $type {
            inner: [u8; 0],
            _p: $crate::OpaqueData,
        }

        // SAFETY: The object is a network object.
        unsafe impl $crate::NWObject for $type {}

        // Reflexive impl
        impl core::convert::AsRef<Self> for $type {
            #[inline]
            fn as_ref(&self) -> &Self {
                self
            }
        }

        impl PartialEq for $type {
            #[doc = concat!("Compare this [`", stringify!($type), "`] with another using pointer equality.")]
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                // Network objects use pointer equality.
                // Some objects like Path provide individual method for equality check.
                core::ptr::eq(self, other)
            }
        }

        // Pointer equality is reflexive.
        impl Eq for $type {}

        // Hash based on pointer.
        impl core::hash::Hash for $type {
            #[inline]
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                let ptr: *const Self = self;
                ptr.hash(state);
            }
        }

        impl core::fmt::Debug for $type {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let ptr: *const Self = self;
                f.debug_struct(core::stringify!($type))
                    .field("ptr", &ptr)
                    .finish_non_exhaustive()
            }
        }

        #[cfg(feature = "objc2")]
        // SAFETY: Network objects are internally objects.
        unsafe impl objc2::encode::RefEncode for $type {
            const ENCODING_REF: objc2::encode::Encoding
                = objc2::encode::Encoding::Object;
        }

        #[cfg(feature = "objc2")]
        // SAFETY: Network objects can act as Objective-C objects
        // (and respond to e.g. retain/release messages).
        unsafe impl objc2::Message for $type {}

        #[cfg(feature = "objc2")]
        impl core::convert::AsRef<objc2::runtime::AnyObject> for $type {
            #[inline]
            fn as_ref(&self) -> &objc2::runtime::AnyObject {
                let ptr: *const Self = self;
                let ptr: *const objc2::runtime::AnyObject = ptr.cast();
                // SAFETY: Network objects can act as Objective-C objects.
                unsafe { &*ptr }
            }
        }
    };
}
