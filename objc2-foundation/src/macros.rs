/// Create a new type to represent an Objective-C class.
///
/// The given name should correspond to a valid Objective-C class, whose
/// instances have the encoding `Encoding::Object` (as an example:
/// `NSAutoreleasePool` does not have this).
///
///
/// # Specification
///
/// This creates an opaque struct, and implements traits for it to allow
/// easier usage as an Objective-C object.
///
/// The traits [`objc2::RefEncode`] and [`objc2::Message`] are implemented to
/// allow sending messages to the object and using it in [`objc2::rc::Id`].
///
/// [`Deref`] and [`DerefMut`] are implemented and delegate to the first
/// superclass (direct parent). Auto traits are inherited from this superclass
/// as well (this macro effectively just creates a newtype wrapper around the
/// superclass).
///
/// Finally, [`AsRef`], [`AsMut`], [`Borrow`] and [`BorrowMut`] are
/// implemented to allow conversion to an arbitary superclasses in the
/// inheritance chain (since an instance of a class can always be interpreted
/// as it's superclasses).
///
/// [`Deref`]: core::ops::Deref
/// [`DerefMut`]: core::ops::DerefMut
/// [`Borrow`]: core::borrow::Borrow
/// [`BorrowMut`]: core::borrow::BorrowMut
///
///
/// # Safety
///
/// The specified inheritance chain must be correct (including in the correct
/// order), and the types in said chain must be valid as Objective-C objects -
/// this is easiest to ensure by also creating those using this macro.
///
///
/// # Example
///
/// Create a new type to represent the `NSFormatter` class.
///
/// ```
/// use objc2::msg_send_id;
/// use objc2::rc::{Id, Shared};
/// use objc2_foundation::{extern_class, NSObject};
/// #
/// # #[cfg(feature = "gnustep-1-7")]
/// # unsafe { objc2::__gnustep_hack::get_class_to_force_linkage() };
///
/// extern_class! {
///     /// An example description.
///     #[derive(PartialEq, Eq, Hash)] // Uses `NSObject`'s implementation
///     // Specify class and superclass
///     // In this case the class `NSFormatter`, which subclasses `NSObject`
///     unsafe pub struct NSFormatter: NSObject;
/// }
///
/// // Provided by the macro
/// let cls = NSFormatter::class();
///
/// // `NSFormatter` implements `Message`:
/// let obj: Id<NSFormatter, Shared> = unsafe { msg_send_id![cls, new].unwrap() };
/// ```
///
/// Represent the `NSDateFormatter` class, using the `NSFormatter` type we
/// declared previously to specify as its superclass.
///
/// ```
/// use objc2_foundation::{extern_class, NSObject};
/// #
/// # extern_class! {
/// #     #[derive(PartialEq, Eq, Hash)]
/// #     unsafe pub struct NSFormatter: NSObject;
/// # }
///
/// extern_class! {
///     #[derive(PartialEq, Eq, Hash)]
///     // Specify the correct inheritance chain
///     // `NSDateFormatter` subclasses `NSFormatter` which subclasses `NSObject`
///     unsafe pub struct NSDateFormatter: NSFormatter, NSObject;
/// }
/// ```
///
/// See the source code of `objc2_foundation` in general for more examples.
#[macro_export]
macro_rules! extern_class {
    (
        $(#[$m:meta])*
        unsafe $v:vis struct $name:ident: $($inheritance_chain:ty),+;

        $(
            unsafe impl {
                $($methods:tt)*
            }
        )?
    ) => {
        $crate::__inner_extern_class! {
            @__inner
            $(#[$m])*
            unsafe $v struct $name<>: $($inheritance_chain,)+ $crate::objc2::runtime::Object {}

            unsafe impl {
                $($($methods)*)?
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_as_ref_borrow {
    ($name:ident<$($t:ident $(: $b:ident)?),*>,) => {};
    ($name:ident<$($t:ident $(: $b:ident)?),*>, $item:ty, $($tail:ty,)*) => {
        impl<$($t $(: $b)?),*> $crate::__core::convert::AsRef<$item> for $name<$($t),*> {
            #[inline]
            fn as_ref(&self) -> &$item {
                // Triggers Deref coercion depending on return type
                &*self
            }
        }

        impl<$($t $(: $b)?),*> $crate::__core::convert::AsMut<$item> for $name<$($t),*> {
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

        impl<$($t $(: $b)?),*> $crate::__core::borrow::Borrow<$item> for $name<$($t),*> {
            #[inline]
            fn borrow(&self) -> &$item {
                // Triggers Deref coercion depending on return type
                &*self
            }
        }

        impl<$($t $(: $b)?),*> $crate::__core::borrow::BorrowMut<$item> for $name<$($t),*> {
            #[inline]
            fn borrow_mut(&mut self) -> &mut $item {
                // Triggers Deref coercion depending on return type
                &mut *self
            }
        }

        $crate::__impl_as_ref_borrow!($name<$($t $(: $b)?),*>, $($tail,)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __inner_extern_class {
    // TODO: Expose this variant in the `object` macro.
    (
        $(#[$m:meta])*
        unsafe $v:vis struct $name:ident<$($t:ident $(: $b:ident)?),*>: $($inheritance_chain:ty),+ {
            $($p:ident: $pty:ty,)*
        }

        $(
            unsafe impl {
                $($methods:tt)*
            }
        )?
    ) => {
        $crate::__inner_extern_class! {
            @__inner
            $(#[$m])*
            unsafe $v struct $name<$($t $(: $b)?),*>: $($inheritance_chain,)+ $crate::objc2::runtime::Object {
                $($p: $pty,)*
            }

            unsafe impl {
                $($($methods)*)?
            }
        }
    };
    (
        @__inner
        $(#[$m:meta])*
        unsafe $v:vis struct $name:ident<$($t:ident $(: $b:ident)?),*>: $inherits:ty $(, $inheritance_rest:ty)* {
            $($p:ident: $pty:ty,)*
        }

        unsafe impl {
            $($methods:tt)*
        }
    ) => {
        $(#[$m])*
        // TODO: repr(transparent) when the inner pointer is no longer a ZST.
        #[repr(C)]
        $v struct $name<$($t $(: $b)?),*> {
            __inner: $inherits,
            // Additional fields (should only be zero-sized PhantomData).
            $($p: $pty),*
        }

        unsafe impl<$($t $(: $b)?),*> $crate::objc2::Message for $name<$($t),*> { }

        unsafe impl<$($t $(: $b)?),*> $crate::objc2::RefEncode for $name<$($t),*> {
            const ENCODING_REF: $crate::objc2::Encoding<'static>
                = <$inherits as $crate::objc2::RefEncode>::ENCODING_REF;
        }

        impl<$($t $(: $b)?),*> $name<$($t),*> {
            $v fn class() -> &'static $crate::objc2::runtime::Class {
                $crate::objc2::class!($name)
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
        impl<$($t $(: $b)?),*> $crate::__core::ops::Deref for $name<$($t),*> {
            type Target = $inherits;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.__inner
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
        impl<$($t $(: $b)?),*> $crate::__core::ops::DerefMut for $name<$($t),*> {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.__inner
            }
        }

        impl<$($t $(: $b)?),*> $crate::__core::convert::AsRef<Self> for $name<$($t),*> {
            #[inline]
            fn as_ref(&self) -> &Self {
                self
            }
        }

        impl<$($t $(: $b)?),*> $crate::__core::convert::AsMut<Self> for $name<$($t),*> {
            #[inline]
            fn as_mut(&mut self) -> &mut Self {
                self
            }
        }

        $crate::__impl_as_ref_borrow!($name<$($t $(: $b)?),*>, $inherits, $($inheritance_rest,)*);

        impl<$($t $(: $b)?),*> $name<$($t),*> {
            $crate::__def_fn!{
                $($methods)*
            }
        }
    };
}

#[macro_export]
macro_rules! __def_fn {
    // Choosing the correct type of msg_send! based on the return type
    //
    // We have to do it like this, since otherwise we won't be able to capture
    // the return type in the way we want.
    //
    // Yup, this is _ugly_, but hey, it works for now. Would really like
    // some of that specialization though!
    {} => {};
    {
        #[sel($($sel:tt)+)]
        $(#[$m:meta])*
        $v:vis fn $name:ident(
            $(&$self:ident)?
            $(&mut $mut_self:ident)?
            $($first_arg:ident: $first_arg_ty:ty)?
            $(, $rest_arg:ident: $rest_arg_ty:ty)* $(,)?
        ) -> bool;

        $($rest:tt)*
    } => {
        $(#[$m])*
        $v fn $name(
            $(&$self)?
            $(&mut $mut_self)?
            $($first_arg: $first_arg_ty)?
            $(, $rest_arg: $rest_arg_ty)*
        ) -> bool {
            unsafe {
                $crate::__def_fn!(
                    @instance_or_class;
                    msg_send_bool;
                    ($($sel)+);
                    ($($self)? $($mut_self)?);
                    ($($self)? $($mut_self)? $($first_arg)? $(, $rest_arg)*);
                )
            }
        }

        $crate::__def_fn!{
            $($rest)*
        }
    };
    {
        #[sel($($sel:tt)+)]
        $(#[$m:meta])*
        $v:vis fn $name:ident(
            $(&$self:ident)?
            $(&mut $mut_self:ident)?
            $($first_arg:ident: $first_arg_ty:ty)?
            $(, $rest_arg:ident: $rest_arg_ty:ty)* $(,)?
        ) -> Id<$obj:ty, $o:ty>;

        $($rest:tt)*
    } => {
        $(#[$m])*
        $v fn $name(
            $(&$self)?
            $(&mut $mut_self)?
            $($first_arg: $first_arg_ty)?
            $(, $rest_arg: $rest_arg_ty)*
        ) -> Id<$obj, $o> {
            unsafe {
                $crate::__def_fn!(
                    @instance_or_class;
                    msg_send_id;
                    ($($sel)+);
                    ($($self)? $($mut_self)?);
                    ($($self)? $($mut_self)? $($first_arg)? $(, $rest_arg)*);
                ).unwrap()
            }
        }

        $crate::__def_fn!{
            $($rest)*
        }
    };
    {
        #[sel($($sel:tt)+)]
        $(#[$m:meta])*
        $v:vis fn $name:ident(
            $(&$self:ident)?
            $(&mut $mut_self:ident)?
            $($first_arg:ident: $first_arg_ty:ty)?
            $(, $rest_arg:ident: $rest_arg_ty:ty)* $(,)?
        ) -> Option<Id<$obj:ty, $o:ty>>;

        $($rest:tt)*
    } => {
        $(#[$m])*
        $v fn $name(
            $(&$self)?
            $(&mut $mut_self)?
            $($first_arg: $first_arg_ty)?
            $(, $rest_arg: $rest_arg_ty)*
        ) -> Option<Id<$obj, $o>> {
            unsafe {
                $crate::__def_fn!(
                    @instance_or_class;
                    msg_send_id;
                    ($($sel)+);
                    ($($self)? $($mut_self)?);
                    ($($self)? $($mut_self)? $($first_arg)? $(, $rest_arg)*);
                )
            }
        }

        $crate::__def_fn!{
            $($rest)*
        }
    };
    {
        #[sel($($sel:tt)+)]
        $(#[$m:meta])*
        $v:vis fn $name:ident(
            $(&$self:ident)?
            $(&mut $mut_self:ident)?
            $($first_arg:ident: $first_arg_ty:ty)?
            $(, $rest_arg:ident: $rest_arg_ty:ty)* $(,)?
        ) $(-> $ret:ty)?;

        $($rest:tt)*
    } => {
        $(#[$m])*
        $v fn $name(
            $(&$self)?
            $(&mut $mut_self)?
            $($first_arg: $first_arg_ty)?
            $(, $rest_arg: $rest_arg_ty)*
        ) $(-> $ret)? {
            // TODO: Verify that the return type is not `bool` accidentally
            // (we match on the literal `bool`, not the actual type, so people
            // could have written `core::primitive::bool`).
            unsafe {
                $crate::__def_fn!(
                    @instance_or_class;
                    msg_send;
                    ($($sel)+);
                    ($($self)? $($mut_self)? $($first_arg)?);
                    ($($self)? $($mut_self)? $($first_arg)? $(, $rest_arg)*);
                )
            }
        }

        $crate::__def_fn!{
            $($rest)*
        }
    };

    // Do things differently depending on instance or class method

    (
        @instance_or_class;
        $macro:ident;
        ($($sel:tt)+);
        (self);
        ($self:ident $(, $rest_arg:ident)*);
    ) => {
        $crate::__def_fn!(
            @collect_msg_send;
            $macro;
            $self;
            ($($sel)+);
            ($($rest_arg)*);
        )
    };
    (
        @instance_or_class;
        $macro:ident;
        ($($sel:tt)+);
        (this);
        ($this:ident $(, $rest_arg:ident)*);
    ) => {
        $crate::__def_fn!(
            @collect_msg_send;
            $macro;
            $this;
            ($($sel)+);
            ($($rest_arg)*);
        )
    };
    (
        @instance_or_class;
        $macro:ident;
        ($($sel:tt)+);
        ();
        ($($rest_arg:ident)*);
    ) => {
        $crate::__def_fn!(
            @collect_msg_send;
            $macro;
            Self::class();
            ($($sel)+);
            ($($rest_arg)*);
        )
    };

    // Collecting selector and argument streams
    // TODO: More of these

    (
        @collect_msg_send;
        $macro:ident;
        $obj:expr;
        ($sel1:ident);
        ();
    ) => { ::objc2::$macro![$obj, $sel1] };
    (
        @collect_msg_send;
        $macro:ident;
        $obj:expr;
        ($sel1:ident:);
        ($arg1:expr);
    ) => { ::objc2::$macro![$obj, $sel1: $arg1] };
    (
        @collect_msg_send;
        $macro:ident;
        $obj:expr;
        ($sel1:ident: $sel2:ident:);
        ($arg1:expr, $arg2:expr);
    ) => { ::objc2::$macro![$obj, $sel1: $arg1, $sel2: $arg2] };
    (
        @collect_msg_send;
        $macro:ident;
        $obj:expr;
        ($sel1:ident: $sel2:ident: $sel3:ident:);
        ($arg1:expr, $arg2:expr, $arg3:expr);
    ) => { ::objc2::$macro![$obj, $sel1: $arg1, $sel2: $arg2, $sel3: $arg3] };
}

macro_rules! unsafe_def_fn {
    (
        $(#[$m:meta])*
        $v:vis fn new -> $o:ty $(;)?
    ) => {
        $(#[$m])*
        $v fn new() -> Id<Self, $o> {
            let cls = Self::class();
            unsafe { $crate::objc2::msg_send_id![cls, new].unwrap() }
        }
    };
}
