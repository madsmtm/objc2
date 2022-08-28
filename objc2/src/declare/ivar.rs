use core::fmt;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;

use crate::encode::{Encode, EncodeConvert};
use crate::runtime::Object;

/// Helper trait for defining instance variables.
///
/// This should be implemented for an empty marker type, which can then be
/// used within [`Ivar`] to refer to the instance variable.
///
///
/// # Safety
///
/// Really, [`Ivar`] should be marked as `unsafe`, but since we can't do that
/// we'll mark this trait as `unsafe` instead. See [`Ivar`] for safety
/// requirements.
///
///
/// # Examples
///
/// Create an instance variable `myCustomIvar` with type `i32`.
///
/// ```
/// use objc2::declare::IvarType;
///
/// // Helper type
/// struct MyCustomIvar;
///
/// unsafe impl IvarType for MyCustomIvar {
///     type Type = i32;
///     const NAME: &'static str = "myCustomIvar";
/// }
///
/// // `Ivar<MyCustomIvar>` can now be used
/// ```
pub unsafe trait IvarType {
    /// The type of the instance variable.
    type Type: EncodeConvert;
    /// The name of the instance variable.
    const NAME: &'static str;
}

unsafe impl<T: IvarType> IvarType for MaybeUninit<T>
where
    T::Type: Encode,
{
    type Type = MaybeUninit<T::Type>;
    const NAME: &'static str = T::NAME;
}

/// A wrapper type over a custom instance variable.
///
/// This type is not meant to be constructed by itself, it must reside within
/// another struct meant to represent an Objective-C object.
///
/// On [`Deref`] it then uses the [`IvarType::NAME`] string to access the ivar
/// of the containing object.
///
/// Note that this is not ([currently][zst-hack]) allowed by [stacked
/// borrows][sb], but due to [`Object`] being a zero-sized type such that we
/// don't have provenance over the ivars anyhow, this should be just as sound
/// as normal instance variable access.
///
/// [sb]: https://github.com/rust-lang/unsafe-code-guidelines/blob/e21202c60c7be03dd2ab016ada92fb5305d40438/wip/stacked-borrows.md
/// [zst-hack]: https://github.com/rust-lang/unsafe-code-guidelines/issues/305
///
///
/// # `bool` handling
///
/// This does _not_ perform a conversion step between [`bool`] and the
/// Objective-C `BOOL`; use [`runtime::Bool`][crate::runtime::Bool] when you
/// want your instance variable to be accessible from other Objective-C code.
///
///
/// # Safety
///
/// This must be used within a type that act as an Objective-C object. In
/// particular, this is never safe to have on the stack by itself.
///
/// Additionally, the instance variable described by `T` must be available on
/// the specific instance, and be of the exact same type. When declaring the
/// object yourself, you can ensure this using
/// [`ClassBuilder::add_static_ivar`].
///
/// Finally, two ivars with the same name must not be used on the same object.
///
/// [`ClassBuilder::add_static_ivar`]: crate::declare::ClassBuilder::add_static_ivar
///
///
/// # Examples
///
/// ```
/// use objc2::declare::{Ivar, IvarType};
/// use objc2::runtime::Object;
/// #
/// # #[cfg(feature = "gnustep-1-7")]
/// # unsafe { objc2::__gnustep_hack::get_class_to_force_linkage() };
///
/// // Declare ivar with given type and name
/// struct MyCustomIvar;
/// unsafe impl IvarType for MyCustomIvar {
///     type Type = i32;
///     const NAME: &'static str = "myCustomIvar";
/// }
///
/// // Custom object
/// #[repr(C)]
/// pub struct MyObject {
///     inner: Object,
///     // SAFETY: The instance variable is used within an object, and it is
///     // properly declared below.
///     my_ivar: Ivar<MyCustomIvar>,
/// }
///
/// # use objc2::class;
/// # use objc2::declare::ClassBuilder;
/// # let mut builder = ClassBuilder::new("MyObject", class!(NSObject)).unwrap();
/// // Declare the class and add the instance variable to it
/// builder.add_static_ivar::<MyCustomIvar>();
/// # let _cls = builder.register();
///
/// let obj: MyObject;
/// // You can now access `obj.my_ivar`
/// ```
///
/// See also the `declare_ivar.rs` example.
#[repr(C)]
// Must not be `Copy` nor `Clone`!
pub struct Ivar<T: IvarType> {
    /// Make this type allowed in `repr(C)`
    inner: [u8; 0],
    /// For proper variance and auto traits
    item: PhantomData<T::Type>,
}

impl<T: IvarType> Ivar<T> {
    fn get_ref(&self) -> &T::Type {
        // SAFETY: The user ensures that this is placed in a struct that can
        // be reinterpreted as an `Object`. Since `Ivar` can never be
        // constructed by itself (and is neither Copy nor Clone), we know that
        // it is guaranteed to _stay_ in said struct.
        //
        // Even if the user were to do `mem::swap`, the `Ivar` has a unique
        // type (and does not hold any data), so that wouldn't break anything.
        //
        // Note: We technically don't have provenance over the object, nor the
        // ivar, but the object doesn't have provenance over the ivar either,
        // so that is fine.
        let ptr = NonNull::from(self).cast::<Object>();
        let obj = unsafe { ptr.as_ref() };

        // SAFETY: User ensures that the `Ivar<T>` is only used when the ivar
        // exists and has the correct type
        unsafe {
            obj.inner_ivar_ptr::<T::Type>(T::NAME)
                .as_ref()
                .unwrap_unchecked()
        }
    }

    fn get_mut_ptr(&mut self) -> *mut T::Type {
        let ptr = NonNull::from(self).cast::<Object>();
        // SAFETY: Same as `get_ref`.
        //
        // Note: We don't use `mut` because the user might have two mutable
        // references to different ivars, as such:
        //
        // ```
        // #[repr(C)]
        // struct X {
        //     inner: Object,
        //     ivar1: Ivar<Ivar1>,
        //     ivar2: Ivar<Ivar2>,
        // }
        //
        // let mut x: X;
        // let ivar1: &mut Ivar<Ivar1> = &mut x.ivar1;
        // let ivar2: &mut Ivar<Ivar2> = &mut x.ivar2;
        // ```
        //
        // And using `mut` would create aliasing mutable reference to the
        // object.
        //
        // TODO: Not entirely sure, it might be safe to just do `as_mut`, but
        // this is definitely safe.
        let obj = unsafe { ptr.as_ref() };

        // SAFETY: User ensures that the `Ivar<T>` is only used when the ivar
        // exists and has the correct type
        unsafe { obj.inner_ivar_ptr::<T::Type>(T::NAME) }
    }

    #[inline]
    fn get_mut(&mut self) -> &mut T::Type {
        // SAFETY: Safe as mutable because there is only one access to a
        // particular ivar at a time (since we have `&mut self`). `Object` is
        // `UnsafeCell`, so mutable access through `&Object` is allowed.
        unsafe { self.get_mut_ptr().as_mut().unwrap_unchecked() }
    }
}

impl<T: IvarType> Deref for Ivar<T> {
    type Target = T::Type;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.get_ref()
    }
}

impl<T: IvarType> DerefMut for Ivar<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

/// Format as a pointer to the instance variable.
impl<T: IvarType> fmt::Pointer for Ivar<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ptr: *const T::Type = &**self;
        fmt::Pointer::fmt(&ptr, f)
    }
}

#[cfg(test)]
mod tests {
    use core::mem;
    use core::panic::{RefUnwindSafe, UnwindSafe};

    use super::*;
    use crate::{msg_send, test_utils, MessageReceiver};

    struct TestIvar;

    unsafe impl IvarType for TestIvar {
        type Type = u32;
        const NAME: &'static str = "_foo";
    }

    #[repr(C)]
    struct IvarTestObject {
        inner: Object,
        foo: Ivar<TestIvar>,
    }

    #[test]
    fn auto_traits() {
        fn assert_auto_traits<T: Unpin + UnwindSafe + RefUnwindSafe + Sized + Send + Sync>() {}
        assert_auto_traits::<Ivar<TestIvar>>();

        // Ensure that `Ivar` is zero-sized
        assert_eq!(mem::size_of::<Ivar<TestIvar>>(), 0);
        assert_eq!(mem::align_of::<Ivar<TestIvar>>(), 1);
    }

    #[test]
    fn access_ivar() {
        let mut obj = test_utils::custom_object();
        let _: () = unsafe { msg_send![&mut obj, setFoo: 42u32] };

        let obj = unsafe {
            obj.__as_raw_receiver()
                .cast::<IvarTestObject>()
                .as_ref()
                .unwrap()
        };
        assert_eq!(*obj.foo, 42);
    }
}
