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
        unsafe $v:vis struct $name:ident: $inherits:ty $(;)?
    ) => {
        object!($(#[$m])* unsafe $v struct $name<>: $inherits {});
    };
    (
        $(#[$m:meta])*
        unsafe $v:vis struct $name:ident<$($t:ident $(: $b:ident)?),*>: $inherits:ty {
            $($p:ident: $pty:ty,)*
        }
    ) => {
        $(#[$m])*
        // TODO: repr(transparent) when the inner pointer is no longer a ZST.
        #[repr(C)]
        $v struct $name<$($t $(: $b)?),*> {
            inner: $inherits,
            // Additional fields (should only be zero-sized PhantomData).
            $($p: $pty),*
        }

        unsafe impl<$($t $(: $b)?),*> ::objc2::Message for $name<$($t),*> { }

        unsafe impl<$($t $(: $b)?),*> ::objc2::RefEncode for $name<$($t),*> {
            const ENCODING_REF: ::objc2::Encoding<'static> = ::objc2::Encoding::Object;
        }

        impl<$($t $(: $b)?),*> $name<$($t),*> {
            pub fn class() -> &'static ::objc2::runtime::Class {
                ::objc2::class!($name)
            }
        }

        // SAFETY: An instance can always be _used_ in exactly the same way as
        // it's superclasses (though not necessarily _constructed_ in the same
        // way, but `Deref` doesn't allow this).
        //
        // Remember; while we (the Rust side) may intentionally be forgetting
        // which instance we're holding, the Objective-C side will remember,
        // and will always dispatch to the correct method implementations.
        //
        // Any lifetime information that the object may have been holding is
        // safely kept in the returned reference.
        //
        // Generics are discarded (for example in the case of `&NSValue<T>` to
        // `&NSObject`), but if the generic contained a lifetime, that
        // lifetime is still included in the returned reference.
        //
        // Note that you can easily have two different variables pointing to
        // the same object, `x: &T` and `y: &T::Target`, and this would be
        // perfectly safe!
        impl<$($t $(: $b)?),*> ::core::ops::Deref for $name<$($t),*> {
            type Target = $inherits;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        // SAFETY: Mutability does not change anything in the above
        // consideration, the lifetime of `&mut Self::Target` is still tied to
        // `&mut self`.
        //
        // Usually we don't want to allow `&mut` of immutable objects like
        // `NSString`, because their `NSCopying` implementation returns the
        // same object, and would violate aliasing rules.
        //
        // But `&mut NSMutableString` -> `&mut NSString` safe, since the
        // `NSCopying` implementation of `NSMutableString` is used, and that
        // is guaranteed to return a different object.
        impl<$($t $(: $b)?),*> ::core::ops::DerefMut for $name<$($t),*> {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }

        // TODO: AsRef and AsMut

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
                use ::objc2::MessageReceiver;
                use $crate::NSObject;
                // "downgrading" to  NSObject to work around generic
                // downgrading not having been set up yet.
                // TODO: Fix this.
                let other: &NSObject = unsafe { &*other.as_raw_receiver().cast() };
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
                self.hash_code().hash(state);
            }
        }

        // TODO: Consider T: Debug bound
        impl<$($t $(: $b)?),*> ::core::fmt::Debug for $name<$($t),*> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                use ::objc2::MessageReceiver;
                use ::alloc::borrow::ToOwned;
                use $crate::NSObject;
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
    (
        $(#[$m:meta])*
        $v:vis fn new -> $o:ty $(;)?
    ) => {
        $(#[$m])*
        $v fn new() -> Id<Self, $o> {
            let cls = Self::class();
            unsafe { Id::new(NonNull::new_unchecked(msg_send![cls, new])) }
        }
    };
}
