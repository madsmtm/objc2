/// Create a new type to represent a class.
///
/// This is similar to an `@interface` declaration in Objective-C.
///
/// It is useful for things like `objc2-foundation`, which needs to create
/// interfaces to existing, externally defined classes like `NSString`,
/// `NSURL` and so on, but can also be useful for users that have custom
/// classes written in Objective-C that they want to access from Rust.
///
///
/// # Specification
///
/// The syntax is similar enough to Rust syntax that if you invoke the macro
/// with parentheses (as opposed to curly brackets), [`rustfmt` will be able to
/// format the contents][rustfmt-macros] (so e.g. as `extern_class!( ... );`).
///
/// The macro creates an opaque struct containing the superclass (which means
/// that auto traits are inherited from the superclass), and implements the
/// following traits for it to allow easier usage as an Objective-C object:
///
/// - [`RefEncode`][crate::RefEncode]
/// - [`Message`][crate::Message]
/// - [`ClassType`][crate::ClassType]
/// - [`Deref<Target = $superclass>`][core::ops::Deref]
/// - [`AsRef<$inheritance_chain>`][AsRef]
/// - [`Borrow<$inheritance_chain>`][core::borrow::Borrow]
///
/// The macro allows specifying zero-sized fields like [`PhantomData`] on the
/// struct.
///
/// You can add most attributes to the class, including `#[cfg(...)]`,
/// `#[derive(...)]` and doc comments (but not ABI-modifying attributes like
/// `#[repr(...)]`).
///
/// [rustfmt-macros]: https://github.com/rust-lang/rustfmt/discussions/5437
/// [`PhantomData`]: core::marker::PhantomData
///
///
/// ## `ClassType` implementation
///
/// The syntax of this macro neatly documents that it implements the
/// [`ClassType`] trait for you, though to do so you need to provide it the
/// following:
/// - The [`Super`] class.
///
///   Due to Rust trait limitations, specifying e.g. the superclass `NSData`
///   would not give you the ability to convert via. `AsRef` to `NSObject`.
///   Therefore, you may optionally specify additional parts of the
///   inheritance chain using an `#[inherits(...)]` attribute.
/// - Optionally, the class' [`ThreadKind`], if the object is only usable on
///   the main thread.
/// - Optionally, the class' [`NAME`] - if not specified, this will default to
///   the struct name.
///
/// You may add `#[cfg(...)]` attributes to the `ClassType` impl, and then it
/// will work as expected. No other attributes are supported.
///
/// [`ClassType`]: crate::ClassType
/// [`Super`]: crate::ClassType::Super
/// [`ThreadKind`]: crate::ClassType::ThreadKind
/// [`NAME`]: crate::ClassType::NAME
///
///
/// # Safety
///
/// This macro implements the three unsafe traits [`RefEncode`], [`Message`]
/// and [`ClassType`] for you, and while it can ensure most of the required
/// properties in those, it cannot ensure all of them.
///
/// In particular, when writing `unsafe` on `impl ClassType`, you must ensure
/// that:
/// 1. [`ClassType::Super`] is correct.
/// 2. [`ClassType::ThreadKind`] is correctly set to `MainThreadOnly` if the
///    class can only be used from the main thread.
///
/// [`RefEncode`]: crate::encode::RefEncode
/// [`Message`]: crate::Message
/// [`ClassType::Super`]: crate::ClassType::Super
/// [`ClassType::ThreadKind`]: crate::ClassType::ThreadKind
/// [ClassType#safety]: crate::ClassType#safety
///
///
/// # Examples
///
/// Create a new type to represent the `NSFormatter` class (for demonstration,
/// `objc2_foundation::NSFormatter` exist for exactly this purpose).
///
/// ```
/// # #[cfg(not_available)]
/// use objc2_foundation::{NSCoding, NSCopying, NSObjectProtocol};
/// # use objc2::runtime::NSObjectProtocol;
/// use objc2::rc::Retained;
/// use objc2::runtime::NSObject;
/// use objc2::{extern_class, msg_send_id, ClassType};
///
/// extern_class!(
///     /// An example description.
///     #[derive(PartialEq, Eq, Hash)] // Uses the superclass' implementation
///     // Specify the class and struct name to be used
///     pub struct NSFormatter;
///
///     unsafe impl ClassType for NSFormatter {
///         // Specify the superclass, in this case `NSObject`
///         type Super = NSObject;
///
///         // Optionally, specify that the class is only usable on the main thread.
///         // type ThreadKind = dyn MainThreadOnly;
///
///         // Optionally, specify the name of the class, if it differs from
///         // the struct name.
///         // const NAME: &'static str = "NSFormatter";
///     }
/// );
///
/// // Note: We have to specify the protocols for the superclasses as well,
/// // since Rust doesn't do inheritance.
/// unsafe impl NSObjectProtocol for NSFormatter {}
/// # #[cfg(not_available)]
/// unsafe impl NSCopying for NSFormatter {}
/// # #[cfg(not_available)]
/// unsafe impl NSCoding for NSFormatter {}
///
/// fn main() {
///     // Provided by the implementation of `ClassType`
///     let cls = NSFormatter::class();
///
///     // `NSFormatter` implements `Message`:
///     let obj: Retained<NSFormatter> = unsafe { msg_send_id![cls, new] };
/// }
/// ```
///
/// Represent the `NSDateFormatter` class, using the `NSFormatter` type we
/// declared previously to specify as its superclass.
///
/// ```
/// # #[cfg(not_available)]
/// use objc2_foundation::{NSCoding, NSCopying, NSObjectProtocol};
/// # use objc2::runtime::NSObjectProtocol;
/// use objc2::runtime::NSObject;
/// use objc2::{extern_class, ClassType};
/// #
/// # extern_class!(
/// #     #[derive(PartialEq, Eq, Hash)]
/// #     pub struct NSFormatter;
/// #
/// #     unsafe impl ClassType for NSFormatter {
/// #         type Super = NSObject;
/// #     }
/// # );
///
/// extern_class!(
///     #[derive(PartialEq, Eq, Hash)]
///     pub struct NSDateFormatter;
///
///     unsafe impl ClassType for NSDateFormatter {
///         // Specify the correct inheritance chain
///         #[inherits(NSObject)]
///         type Super = NSFormatter;
///     }
/// );
///
/// // Similarly, we can specify the protocols that this implements here:
/// unsafe impl NSObjectProtocol for NSFormatter {}
/// # #[cfg(not_available)]
/// unsafe impl NSCopying for NSDateFormatter {}
/// # #[cfg(not_available)]
/// unsafe impl NSCoding for NSDateFormatter {}
/// ```
///
/// See the source code of `objc2-foundation` for many more examples.
#[doc(alias = "@interface")]
#[macro_export]
macro_rules! extern_class {
    // No fields
    (
        $(#[$m:meta])*
        $v:vis struct $name:ident;

        $(#[$impl_m:meta])*
        unsafe impl ClassType for $for:ty {
            $(#[inherits($($inheritance_rest:ty),+)])?
            type Super = $superclass:ty;
            $(type ThreadKind = $thread_kind:ty;)?
            $(const NAME: &'static str = $name_const:expr;)?
        }
    ) => {
        // Shorthand syntax for the following
        $crate::extern_class!(
            $(#[$m])*
            $v struct $name {}

            $(#[$impl_m])*
            unsafe impl ClassType for $for {
                $(#[inherits($($inheritance_rest),+)])?
                type Super = $superclass;
                $(type ThreadKind = $thread_kind;)?
                $(const NAME: &'static str = $name_const;)?
            }
        );
    };
    (
        $(#[$m:meta])*
        $v:vis struct $name:ident {
            $($field_vis:vis $field:ident: $field_ty:ty,)*
        }

        $(#[$impl_m:meta])*
        unsafe impl ClassType for $for:ty {
            $(#[inherits($($inheritance_rest:ty),+)])?
            type Super = $superclass:ty;
            $(type ThreadKind = $thread_kind:ty;)?
            $(const NAME: &'static str = $name_const:expr;)?
        }
    ) => {
        $crate::__inner_extern_class!(
            $(#[$m])*
            $v struct $name<> {
                __superclass: $superclass,
                $($field_vis $field: $field_ty,)*
            }

            // SAFETY:
            // 1. We define the type, and ensure that it represents a specific
            //    class (for example with the check below that ensures it is
            //    zero-sized).
            // 2. Caller upholds that the superclass is correct.
            // 3. [`Self::ThreadKind`] is ensured correct by `ValidThreadKind`
            //    and `MainThreadOnlyDoesNotImplSendSync` in `class`.
            // 4. While the user controls the name, they control it in a way
            //    that...
            // 5. [`class`] still returns the actual class.
            $(#[$impl_m])*
            unsafe impl<> ClassType for $for {
                $(#[inherits($($inheritance_rest),+)])?
                type Super = $superclass;
                $(type ThreadKind = $thread_kind;)?
                $(const NAME: &'static str = $name_const;)?

                fn as_super(&self) -> &Self::Super {
                    &self.__superclass
                }
            }
        );

        $(#[$impl_m])*
        const _: () = {
            if $crate::__macro_helpers::size_of::<$name>() != 0 {
                $crate::__macro_helpers::panic!($crate::__macro_helpers::concat!(
                    "the struct ",
                    $crate::__macro_helpers::stringify!($name),
                    " is not zero-sized!",
                ))
            }
        };
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_as_ref_borrow {
    {
        impl ($($t:tt)*) for $for:ty {
            fn as_ref($($self:tt)*) $ref:block
        }

        ()
    } => {};
    {
        impl ($($t:tt)*) for $for:ty {
            fn as_ref($($self:tt)*) $ref:block
        }

        ($item:ty, $($tail:ty,)*)
    } => {
        impl<$($t)*> $crate::__macro_helpers::AsRef<$item> for $for {
            #[inline]
            fn as_ref($($self)*) -> &$item $ref
        }

        // Borrow is correct, since subclasses behaves identical to the class
        // they inherit (message sending doesn't care).
        //
        // In particular, `Eq`, `Ord` and `Hash` all give the same results
        // after borrow.

        impl<$($t)*> $crate::__macro_helpers::Borrow<$item> for $for {
            #[inline]
            fn borrow($($self)*) -> &$item $ref
        }

        $crate::__impl_as_ref_borrow! {
            impl ($($t)*) for $for {
                fn as_ref($($self)*) $ref
            }

            ($($tail,)*)
        }
    };
    // TODO: Expose a generic variant of the macro.
}

/// Note: While this is not public, it is still a breaking change to change
/// this, since framework crates rely on it.
#[doc(hidden)]
#[macro_export]
macro_rules! __inner_extern_class {
    (
        $(#[$m:meta])*
        $v:vis struct $name:ident<$($t_struct:ident $(: $(?$b_sized_struct:ident)? $($b_struct:ident)? $(= $default:ty)?)?),* $(,)?> {
            $superclass_field:ident: $superclass_field_ty:ty,
            $($fields:tt)*
        }

        $(#[$impl_m:meta])*
        unsafe impl<$($t_for:ident $(: $(?$b_sized_for:ident +)? $b_for:ident)?),* $(,)?> ClassType for $for:ty {
            $(#[inherits($($inheritance_rest:ty),+ $(,)?)])?
            type Super = $superclass:ty;
            $(type ThreadKind = $thread_kind:ty;)?
            $(const NAME: &'static str = $name_const:expr;)?

            fn as_super(&$as_super_self:ident) -> &Self::Super $as_super:block
        }
    ) => {
        $(#[$m])*
        #[repr(C)]
        $v struct $name<$($t_struct $(: $(?$b_sized_struct)? $($b_struct)? $(= $default)?)?),*> {
            $superclass_field: $superclass_field_ty,
            $($fields)*
        }

        $crate::__extern_class_impl_traits! {
            $(#[$impl_m])*
            unsafe impl ($($t_for $(: $(?$b_sized_for +)? $b_for)?),*) for $for {
                INHERITS = [$superclass, $($($inheritance_rest,)+)? $crate::runtime::AnyObject];

                fn as_super(&$as_super_self) $as_super
            }
        }

        // SAFETY: This maps `SomeClass<T, ...>` to a single `SomeClass<AnyObject, ...>` type and
        // implements `DowncastTarget` on that type. This is safe because the "base container" class
        // is the same and each generic argument is replaced with `AnyObject`, which can represent
        // any Objective-C class instance.
        $(#[$impl_m])*
        unsafe impl $crate::DowncastTarget for $name<$($crate::__extern_class_map_anyobject!($t_for)),*> {}

        $(#[$impl_m])*
        unsafe impl<$($t_for $(: $(?$b_sized_for +)? $b_for)?),*> ClassType for $for {
            type Super = $superclass;
            type ThreadKind = $crate::__select_thread_kind!($($thread_kind)?);
            const NAME: &'static $crate::__macro_helpers::str = $crate::__select_name!($name; $($name_const)?);

            #[inline]
            fn class() -> &'static $crate::runtime::AnyClass {
                let _ = <Self as $crate::__macro_helpers::ValidThreadKind<Self::ThreadKind>>::check;
                let _ = <Self as $crate::__macro_helpers::MainThreadOnlyDoesNotImplSendSync<_>>::check;

                $crate::__class_inner!(
                    $crate::__select_name!($name; $($name_const)?),
                    $crate::__hash_idents!($name)
                )
            }

            #[inline]
            fn as_super(&$as_super_self) -> &Self::Super $as_super
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_class_map_anyobject {
    ($t:ident) => {
        $crate::runtime::AnyObject
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_class_impl_traits {
    (
        $(#[$impl_m:meta])*
        unsafe impl ($($t:tt)*) for $for:ty {
            INHERITS = [$superclass:ty $(, $inheritance_rest:ty)*];

            fn as_super(&$as_super_self:ident) $as_super:block
        }
    ) => {
        // SAFETY:
        // - The item is FFI-safe with `#[repr(C)]`.
        // - The encoding is taken from the inner item, and caller verifies
        //   that it actually inherits said object.
        // - The rest of the struct's fields are ZSTs, so they don't influence
        //   the layout.
        //
        // Be aware that very rarely, this implementation is wrong because the
        // class' instances do not have the encoding `Encoding::Object`.
        //
        // A known case is that `NSAutoreleasePool` has a different encoding.
        // This should be fairly problem-free though, since that is still
        // valid in Objective-C to represent that class' instances as
        // `NSObject*`.
        $(#[$impl_m])*
        unsafe impl<$($t)*> $crate::RefEncode for $for {
            const ENCODING_REF: $crate::Encoding
                = <$superclass as $crate::RefEncode>::ENCODING_REF;
        }

        // SAFETY: This is a newtype wrapper over `AnyObject` (we even ensure
        // that `AnyObject` is always last in our inheritance tree), so it is
        // always safe to reinterpret as that.
        //
        // That the object must work with standard memory management is
        // properly upheld by the fact that the superclass is required by
        // `ValidThreadKind` to implement `ClassType`, and hence must also be
        // a subclass of one of `NSObject`, `NSProxy` or some other class that
        // ensures this (e.g. the object itself is not a root class).
        $(#[$impl_m])*
        unsafe impl<$($t)*> $crate::Message for $for {}

        // SAFETY: An instance can always be _used_ in exactly the same way as
        // its superclasses (though not necessarily _constructed_ in the same
        // way, but `Deref` doesn't allow this).
        //
        // Remember; while we (the Rust side) may intentionally be forgetting
        // which instance we're holding, the Objective-C side will remember,
        // and will always dispatch to the correct method implementations.
        //
        // TODO: If the object has a lifetime, we must keep that lifetime
        // information, since all objects can be retained using
        // `Message::retain`, and that could possibly make it unsound to allow
        // non-`'static` here.
        //
        // `&NSMutableArray<T>` -> `&NSArray<T>` -> `Retained<NSArray<T>>` is
        // fine, but `&UserClass<'a>` -> `&NSObject` -> `Retained<NSObject>`
        // is not, and hence `&NSArray<UserClass<'a>>` -> `&NSObject` ->
        // `Retained<NSObject>` isn't either.
        $(#[$impl_m])*
        impl<$($t)*> $crate::__macro_helpers::Deref for $for {
            type Target = $superclass;

            #[inline]
            fn deref(&$as_super_self) -> &Self::Target $as_super
        }

        $(#[$impl_m])*
        impl<$($t)*> $crate::__macro_helpers::AsRef<Self> for $for {
            #[inline]
            fn as_ref(&self) -> &Self {
                self
            }
        }

        // Assume the meta attributes are all `cfg` attributes
        $(#[$impl_m])*
        $crate::__impl_as_ref_borrow! {
            impl ($($t)*) for $for {
                fn as_ref(&self) {
                    // Triggers Deref coercion depending on return type
                    &*self
                }
            }

            ($superclass, $($inheritance_rest,)*)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __select_thread_kind {
    () => {
        // Default to the super class' thread kind
        <Self::Super as $crate::ClassType>::ThreadKind
    };
    ($thread_kind:ty) => {
        $thread_kind
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __select_name {
    ($_name:ident; $name_const:expr) => {
        $name_const
    };
    ($name:ident;) => {
        $crate::__macro_helpers::stringify!($name)
    };
}
