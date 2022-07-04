macro_rules! __impl_as_ref_borrow {
    ($name:ident<$($t:ident $(: $b:ident)?),*>,) => {};
    ($name:ident<$($t:ident $(: $b:ident)?),*>, $item:ty, $($tail:ty,)*) => {
        impl<$($t $(: $b)?),*> AsRef<$item> for $name<$($t),*> {
            #[inline]
            fn as_ref(&self) -> &$item {
                // Triggers Deref coercion depending on return type
                &*self
            }
        }

        impl<$($t $(: $b)?),*> AsMut<$item> for $name<$($t),*> {
            #[inline]
            fn as_mut(&mut self) -> &mut $item {
                // Triggers DerefMut coercion depending on return type
                &mut *self
            }
        }

        // Borrow and BorrowMut are correct, since subclasses behaves
        // identical to the class they inherit (message sending doesn't care).
        //
        // In particular, `Eq`, `Ord` and `Hash` all give the same results
        // after borrow.

        impl<$($t $(: $b)?),*> ::core::borrow::Borrow<$item> for $name<$($t),*> {
            #[inline]
            fn borrow(&self) -> &$item {
                // Triggers Deref coercion depending on return type
                &*self
            }
        }

        impl<$($t $(: $b)?),*> ::core::borrow::BorrowMut<$item> for $name<$($t),*> {
            #[inline]
            fn borrow_mut(&mut self) -> &mut $item {
                // Triggers Deref coercion depending on return type
                &mut *self
            }
        }

        __impl_as_ref_borrow!($name<$($t $(: $b)?),*>, $($tail,)*);
    };
}

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
        unsafe $v:vis struct $name:ident: $($inheritance_chain:ty),+ $(;)?
    ) => {
        object! {
            @__inner
            $(#[$m])*
            unsafe $v struct $name<>: $($inheritance_chain,)+ ::objc2::runtime::Object {}
        }
    };
    (
        $(#[$m:meta])*
        unsafe $v:vis struct $name:ident<$($t:ident $(: $b:ident)?),*>: $($inheritance_chain:ty),+ {
            $($p:ident: $pty:ty,)*
        }
    ) => {
        object! {
            @__inner
            $(#[$m])*
            unsafe $v struct $name<$($t $(: $b)?),*>: $($inheritance_chain,)+ ::objc2::runtime::Object {
                $($p: $pty,)*
            }
        }
    };
    (
        @__inner
        $(#[$m:meta])*
        unsafe $v:vis struct $name:ident<$($t:ident $(: $b:ident)?),*>: $inherits:ty $(, $inheritance_rest:ty)* {
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

        impl<$($t $(: $b)?),*> AsRef<Self> for $name<$($t),*> {
            #[inline]
            fn as_ref(&self) -> &Self {
                self
            }
        }

        impl<$($t $(: $b)?),*> AsMut<Self> for $name<$($t),*> {
            #[inline]
            fn as_mut(&mut self) -> &mut Self {
                self
            }
        }

        __impl_as_ref_borrow!($name<$($t $(: $b)?),*>, $inherits, $($inheritance_rest,)*);

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
                self.is_equal(other.as_ref())
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
                let description = self.description();
                ::objc2::rc::autoreleasepool(|pool| {
                    ::core::fmt::Debug::fmt(description.as_str(pool), f)
                })
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
            unsafe { ::objc2::msg_send_id![cls, new].unwrap() }
        }
    };
}
