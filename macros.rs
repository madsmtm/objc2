#[macro_export]
macro_rules! object_struct {
    ($name:ident) => (
        object_struct!($name,);
    );
    ($name:ident<$($t:ident),+>) => (
        object_struct!($name, $($t),+);
    );
    ($name:ident, $($t:ident),*) => (
        pub enum $name<$($t),*> { }

        object_impl!($name $(,$t)*);
    );
}

#[macro_export]
macro_rules! object_impl {
    ($name:ident) => (
        object_impl!($name,);
    );
    ($name:ident<$($t:ident),+>) => (
        object_impl!($name, $($t),+);
    );
    ($name:ident, $($t:ident),*) => (
        unsafe impl<$($t),*> ::objc::Message for $name<$($t),*> { }

        impl<$($t),*> ::objc::EncodePtr for $name<$($t),*> {
            fn ptr_code() -> &'static str { "@" }
        }

        impl<$($t),*> $crate::INSObject for $name<$($t),*> {
            fn class() -> &'static ::objc::runtime::Class {
                let name = stringify!($name);
                match ::objc::runtime::Class::get(name) {
                    Some(cls) => cls,
                    None => panic!("Class {} not found", name),
                }
            }
        }

        impl<$($t),*> ::std::cmp::PartialEq for $name<$($t),*> {
            fn eq(&self, other: &$name<$($t),*>) -> bool {
                use $crate::INSObject;
                self.is_equal(other)
            }
        }

        impl<$($t),*> ::std::cmp::Eq for $name<$($t),*> { }

        impl<H, $($t),*> ::std::hash::Hash<H> for $name<$($t),*>
                where H: ::std::hash::Hasher + ::std::hash::Writer {
            fn hash(&self, state: &mut H) {
                use $crate::INSObject;
                self.hash_code().hash(state);
            }
        }

        impl<$($t),*> ::std::fmt::Debug for $name<$($t),*> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                use $crate::{INSObject, INSString};
                ::std::fmt::Debug::fmt(self.description().as_str(), f)
            }
        }
    );
}
