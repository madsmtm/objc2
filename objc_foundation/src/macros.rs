#[macro_export]
macro_rules! object_struct {
    ($name:ident) => {
        pub struct $name {
            _private: (),
        }

        unsafe impl ::objc::Message for $name {}

        unsafe impl<'a> ::objc::Encode for &'a $name {
            const ENCODING: ::objc::Encoding<'static> = ::objc::Encoding::Object;
        }

        unsafe impl<'a> ::objc::Encode for &'a mut $name {
            const ENCODING: ::objc::Encoding<'static> = ::objc::Encoding::Object;
        }

        impl $crate::INSObject for $name {
            fn class() -> &'static ::objc::runtime::Class {
                ::objc::class!($name)
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
            fn hash<H>(&self, state: &mut H)
            where
                H: ::core::hash::Hasher,
            {
                use $crate::INSObject;
                self.hash_code().hash(state);
            }
        }

        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                use $crate::{INSObject, INSString};
                ::core::fmt::Debug::fmt(self.description().as_str(), f)
            }
        }
    };
}

macro_rules! object_impl {
    ($name:ident) => (
        object_impl!($name,);
    );
    ($name:ident<$($t:ident),+>) => (
        object_impl!($name, $($t),+);
    );
    ($name:ident, $($t:ident),*) => (
        unsafe impl<$($t),*> ::objc::Message for $name<$($t),*> { }

        unsafe impl<'a, $($t),*> ::objc::Encode for &'a $name<$($t),*> {
            const ENCODING: ::objc::Encoding<'static> = ::objc::Encoding::Object;
        }

        unsafe impl<'a, $($t),*> ::objc::Encode for &'a mut $name<$($t),*> {
            const ENCODING: ::objc::Encoding<'static> = ::objc::Encoding::Object;
        }
    );
}
