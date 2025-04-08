macro_rules! dispatch_object {
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

        // SAFETY: The object is a dispatch object.
        unsafe impl $crate::DispatchObject for $type {}

        // Reflexive impl
        impl core::convert::AsRef<Self> for $type {
            #[inline]
            fn as_ref(&self) -> &Self {
                self
            }
        }

        impl PartialEq for $type {
            /// Compare this [`$type`] with another using pointer equality.
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                // Dispatch objects use pointer equality.
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
        // SAFETY: Dispatch objects are internally objects.
        unsafe impl objc2::encode::RefEncode for $type {
            const ENCODING_REF: objc2::encode::Encoding
                = objc2::encode::Encoding::Object;
        }

        #[cfg(feature = "objc2")]
        // SAFETY: Dispatch objects can act as Objective-C objects
        // (and respond to e.g. retain/release messages).
        unsafe impl objc2::Message for $type {}

        #[cfg(feature = "objc2")]
        impl core::convert::AsRef<objc2::runtime::AnyObject> for $type {
            #[inline]
            fn as_ref(&self) -> &objc2::runtime::AnyObject {
                let ptr: *const Self = self;
                let ptr: *const objc2::runtime::AnyObject = ptr.cast();
                // SAFETY: Dispatch objects can act as Objective-C objects.
                unsafe { &*ptr }
            }
        }

        // TODO: Implement `ClassType` and `DowncastTarget` using
        // OS_dispatch_group classes etc.?
        //
        // They are available since macOS 10.12, so we could safely use
        // `objc2::class!`.
        //
        // They cannot be subclassed / cannot have categories defined for them
        // though (they're marked `objc_runtime_visible`).
    };
}

macro_rules! dispatch_object_not_data {
    (unsafe $type:ident) => {
        // SAFETY: All Dispatch objects, except for DispatchObject and
        // DispatchData, go through `DISPATCH_DECL` or `DISPATCH_DECL_SUBCLASS`,
        // which delegate to `OS_OBJECT_DECL_SENDABLE_SUBCLASS_SWIFT`, which
        // in turn marks the type as `OS_OBJECT_SWIFT_SENDABLE`.
        unsafe impl Send for $type {}
        // SAFETY: Same as above.
        unsafe impl Sync for $type {}
    };
}
