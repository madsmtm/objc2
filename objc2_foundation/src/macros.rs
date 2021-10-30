macro_rules! object_struct {
    ($name:ident, $ownership:ty) => {
        // TODO: `extern type`
        #[repr(C)]
        pub struct $name {
            _private: [u8; 0],
        }

        unsafe impl ::objc2::Message for $name {}

        unsafe impl ::objc2::RefEncode for $name {
            const ENCODING_REF: ::objc2::Encoding<'static> = ::objc2::Encoding::Object;
        }

        impl $crate::INSObject for $name {
            type Ownership = $ownership;

            fn class() -> &'static ::objc2::runtime::Class {
                ::objc2::class!($name)
            }
        }

        impl ::core::cmp::PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                use $crate::INSObject;
                self.is_equal(other)
            }
        }

        impl ::core::cmp::Eq for $name {}

        impl ::core::hash::Hash for $name {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                use $crate::INSObject;
                self.hash_code().hash(state);
            }
        }

        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                use $crate::{INSObject, INSString};
                ::objc2::rc::autoreleasepool(|pool| {
                    ::core::fmt::Debug::fmt(self.description().as_str(pool), f)
                })
            }
        }
    };
}

macro_rules! object_impl {
    ($name:ident) => (
        object_impl!($name,);
    );
    ($name:ident<$($t:ident$(: $b:ident)?),+>) => (
        object_impl!($name, $($t$(: $b)?),+);
    );
    ($name:ident, $($t:ident$(: $b:ident)?),*) => (
        unsafe impl<$($t$(:($b))?),*> ::objc2::Message for $name<$($t),*> { }

        unsafe impl<$($t$(: $b)?),*> ::objc2::RefEncode for $name<$($t),*> {
            const ENCODING_REF: ::objc2::Encoding<'static> = ::objc2::Encoding::Object;
        }
    );
}
