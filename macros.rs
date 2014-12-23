#![macro_escape]

#[macro_export]
macro_rules! object_struct(
    ($name:ident<$($t:ident),+>) => (
        object_struct!($name, $($t),+);
    );
    ($name:ident $(,$t:ident)*) => (
        #[allow(missing_copy_implementations)]
        pub enum $name<$($t),*> { }

        object_impl!($name $(,$t)*);
    );
);

#[macro_export]
macro_rules! object_impl(
    ($name:ident<$($t:ident),+>) => (
        object_impl!($name, $($t),+);
    );
    ($name:ident $(,$t:ident)*) => (
        impl<$($t),*> ::objc::Message for $name<$($t),*> { }

        encode_message_impl!("@", $name $(, $t)*);

        impl<$($t),*> ::objc_foundation::INSObject for $name<$($t),*> {
            fn class_name() -> ::objc_foundation::ClassName<$name<$($t),*>> {
                ::objc_foundation::ClassName(stringify!($name))
            }
        }

        impl<$($t),*> ::std::cmp::PartialEq for $name<$($t),*> {
            fn eq(&self, other: &$name<$($t),*>) -> bool {
                use objc_foundation::INSObject;
                self.is_equal(other)
            }
        }

        impl<$($t),*> ::std::cmp::Eq for $name<$($t),*> { }

        impl<$($t),*> ::std::hash::Hash for $name<$($t),*> {
            fn hash(&self, state: &mut ::std::hash::sip::SipState) {
                use objc_foundation::INSObject;
                self.hash_code().hash(state);
            }
        }

        impl<$($t),*> ::std::fmt::Show for $name<$($t),*> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                use objc_foundation::{INSObject, INSString};
                self.description().as_str().fmt(f)
            }
        }
    );
);
