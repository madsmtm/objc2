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

        unsafe impl<$($t $(: $b)?),*> ::objc2::encode::RefEncode for $name<$($t),*> {
            const ENCODING_REF: ::objc2::encode::Encoding<'static> = ::objc2::encode::Encoding::Object;
        }

        unsafe impl<$($t $(: $b)?),*> $crate::INSObject for $name<$($t),*> {
            fn class() -> &'static ::objc2::runtime::Class {
                ::objc2::class!($name)
            }
        }

        // Objective-C equality has approximately the same semantics as Rust
        // equality (although less aptly specified).
        //
        // At the very least, equality is _expected_ to be symmetric and
        // transitive, and that's about the best we can do.
        //
        // `T: PartialEq` bound added because e.g. `NSArray` does deep
        // (instead of shallow) equality comparisons.
        //
        // See also https://nshipster.com/equality/
        impl<$($t: ::core::cmp::PartialEq $(+ $b)?),*> ::core::cmp::PartialEq for $name<$($t),*> {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                use $crate::INSObject;
                self.is_equal(other)
            }
        }

        // Most types' equality is reflexive.
        //
        // `T: Eq` bound added to prevent e.g. `NSValue<f32>` from being `Eq`
        // (even though `[NAN isEqual: NAN]` is true in Objective-C).
        impl<$($t: ::core::cmp::Eq $(+ $b)?),*> ::core::cmp::Eq for $name<$($t),*> {}

        // Hashing in Objective-C has the exact same requirement as in Rust:
        //
        // > If two objects are equal (as determined by the isEqual: method),
        // > they must have the same hash value.
        //
        // See https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418859-hash
        impl<$($t: ::core::hash::Hash $(+ $b)?),*> ::core::hash::Hash for $name<$($t),*> {
            #[inline]
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                use $crate::INSObject;
                self.hash_code().hash(state);
            }
        }

        // TODO: Consider T: Debug bound
        impl<$($t $(: $b)?),*> ::core::fmt::Debug for $name<$($t),*> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                use ::objc2::MessageReceiver;
                use ::alloc::borrow::ToOwned;
                use $crate::{INSObject, INSString, NSObject};
                // "downgrading" to  NSObject and calling `to_owned` to work
                // around `f` and Self not being AutoreleaseSafe.
                // TODO: Fix this!
                let this: &NSObject = unsafe { &*self.as_raw_receiver().cast() };
                let s = ::objc2::rc::autoreleasepool(|pool| {
                    this.description().as_str(pool).to_owned()
                });
                ::core::fmt::Debug::fmt(&s, f)
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
