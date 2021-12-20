/// TODO
///
/// # Safety
///
/// The given name must be a valid Objective-C class that inherits NSObject
/// and it's instances must have the raw encoding `Encoding::Object` (an
/// example: `NSAutoreleasePool` does not have this). Finally the ownership
/// must be correct for this class.
macro_rules! object {
    (
        $(#[$m:meta])*
        unsafe $v:vis struct $name:ident
    ) => {
        object!($(#[$m])* unsafe $v struct $name<> {});
    };
    (
        $(#[$m:meta])*
        unsafe $v:vis struct $name:ident<$($t:ident $(: $b:ident)?),*> {
            $($p:ident: $pty:ty,)*
        }
    ) => {
        // TODO: `extern type`
        $(#[$m])*
        #[repr(C)]
        $v struct $name<$($t $(: $b)?),*> {
            _private: [u8; 0],
            $($p: $pty),*
        }

        unsafe impl<$($t $(: $b)?),*> ::objc2::Message for $name<$($t),*> { }

        unsafe impl<$($t $(: $b)?),*> ::objc2::RefEncode for $name<$($t),*> {
            const ENCODING_REF: ::objc2::Encoding<'static> = ::objc2::Encoding::Object;
        }

        unsafe impl<$($t $(: $b)?),*> $crate::INSObject for $name<$($t),*> {
            fn class() -> &'static ::objc2::runtime::Class {
                ::objc2::class!($name)
            }
        }

        impl<$($t $(: $b)?),*> ::core::cmp::PartialEq for $name<$($t),*> {
            fn eq(&self, other: &Self) -> bool {
                use $crate::INSObject;
                self.is_equal(other)
            }
        }

        impl<$($t $(: $b)?),*> ::core::cmp::Eq for $name<$($t),*> {}

        impl<$($t $(: $b)?),*> ::core::hash::Hash for $name<$($t),*> {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                use $crate::INSObject;
                self.hash_code().hash(state);
            }
        }

        impl<$($t $(: $b)?),*> ::core::fmt::Debug for $name<$($t),*> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                use $crate::{INSObject, INSString};
                ::objc2::rc::autoreleasepool(|pool| {
                    ::core::fmt::Debug::fmt(self.description().as_str(pool), f)
                })
            }
        }
    };
}

macro_rules! unsafe_def_fn {
    ($v:vis fn new -> $o:ty) => {
        $v fn new() -> Id<Self, $o> {
            let cls = <Self as INSObject>::class();
            unsafe { Id::new(NonNull::new_unchecked(msg_send![cls, new])) }
        }
    };
}
