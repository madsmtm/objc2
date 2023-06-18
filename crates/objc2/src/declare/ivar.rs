use core::fmt;
use core::marker::PhantomData;
use core::mem::{self, MaybeUninit};
use core::ops::{Deref, DerefMut};
use core::ptr::{self, NonNull};

use crate::encode::Encode;
use crate::runtime::{ivar_offset, AnyObject};

pub(crate) mod private {
    pub trait Sealed {}
}

/// Types that may be used in ivars.
///
/// This may be either:
/// - [`IvarBool`][super::IvarBool].
/// - [`IvarDrop<T>`][super::IvarDrop].
/// - [`IvarEncode<T>`][super::IvarEncode].
///
/// This is a sealed trait, and should not need to be implemented. Open an
/// issue if you know a use-case where this restrition should be lifted!
///
///
/// # Safety
///
/// You cannot rely on any safety guarantees from this.
//
// The type must have the same memory layout as the output type.
//
// Additionally, the type must be safe to drop even if zero-initialized.
//
// Ivars are documented to be zero-initialized in [this section of the
// Objective-C manual][obj-dynamically-created], and that has been true since
// at least [the Objective-C version shipped with Mac OS X 10.0][objc4-208].
//
// [obj-dynamically-created]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ProgrammingWithObjectiveC/WorkingwithObjects/WorkingwithObjects.html#//apple_ref/doc/uid/TP40011210-CH4-SW7
// [objc4-208]: https://github.com/apple-oss-distributions/objc4/blob/objc4-208/runtime/objc-class.m#L367
pub unsafe trait InnerIvarType: private::Sealed + Encode {
    /// The type that an `Ivar` containing this will dereference to.
    ///
    /// E.g. `Ivar<IvarDrop<Box<u8>>>` will deref to `Box<u8>`.
    type Output;

    /// # Safety
    ///
    /// The instance variable must have been initialized.
    #[doc(hidden)]
    unsafe fn __deref(&self) -> &Self::Output;

    /// # Safety
    ///
    /// The instance variable must have been initialized.
    #[doc(hidden)]
    unsafe fn __deref_mut(&mut self) -> &mut Self::Output;
}

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
/// use objc2::declare::{IvarEncode, IvarType};
///
/// // Helper type
/// struct MyCustomIvar;
///
/// unsafe impl IvarType for MyCustomIvar {
///     type Type = IvarEncode<i32>;
///     const NAME: &'static str = "myCustomIvar";
/// }
///
/// // `Ivar<MyCustomIvar>` can now be used
/// ```
pub unsafe trait IvarType {
    /// The type of the instance variable.
    type Type: InnerIvarType;
    /// The name of the instance variable.
    const NAME: &'static str;

    #[doc(hidden)]
    unsafe fn __offset(ptr: NonNull<AnyObject>) -> isize {
        let obj = unsafe { ptr.as_ref() };
        ivar_offset(obj.class(), Self::NAME, &Self::Type::ENCODING)
    }
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
/// borrows][sb], but due to objects being zero-sized types, we don't have
/// provenance over the ivars anyhow, this should be just as sound as normal
/// instance variable access.
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
/// use objc2::declare::{Ivar, IvarEncode, IvarType};
/// use objc2::runtime::NSObject;
///
/// // Declare ivar with given type and name
/// struct MyCustomIvar;
/// unsafe impl IvarType for MyCustomIvar {
///     type Type = IvarEncode<i32>;
///     const NAME: &'static str = "myCustomIvar";
/// }
///
/// // Custom object
/// #[repr(C)]
/// pub struct MyObject {
///     inner: NSObject,
///     // SAFETY: The instance variable is used within an object, and it is
///     // properly declared below.
///     my_ivar: Ivar<MyCustomIvar>,
/// }
///
/// # use objc2::ClassType;
/// # use objc2::declare::ClassBuilder;
/// # let mut builder = ClassBuilder::new("MyObject", NSObject::class()).unwrap();
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

impl<T: IvarType> Drop for Ivar<T> {
    #[inline]
    fn drop(&mut self) {
        if mem::needs_drop::<T::Type>() {
            unsafe { ptr::drop_in_place(self.as_inner_mut_ptr().as_ptr()) }
        }
    }
}

impl<T: IvarType> Ivar<T> {
    /// Get a pointer to the instance variable.
    ///
    /// Note that if the ivar has already been initialized, you can use the
    /// `Deref` implementation to get a reference.
    ///
    /// This is similar to [`MaybeUninit::as_ptr`], see that for usage
    /// instructions.
    pub fn as_ptr(this: &Self) -> *const <Self as Deref>::Target {
        this.as_inner_ptr().as_ptr().cast()
    }

    fn as_inner_ptr(&self) -> NonNull<T::Type> {
        let ptr: NonNull<AnyObject> = NonNull::from(self).cast();

        // SAFETY: The user ensures that this is placed in a struct that can
        // be reinterpreted as an `AnyObject`. Since `Ivar` can never be
        // constructed by itself (and is neither Copy nor Clone), we know that
        // it is guaranteed to _stay_ in said struct.
        //
        // Even if the user were to do `mem::swap`, the `Ivar` has a unique
        // type (and does not hold any data), so that wouldn't break anything.
        //
        // Note: We technically don't have provenance over the object, nor the
        // ivar, but the object doesn't have provenance over the ivar either,
        // so that is fine.
        let offset = unsafe { T::__offset(ptr) };
        // SAFETY: The offset is valid
        unsafe { AnyObject::ivar_at_offset::<T::Type>(ptr, offset) }
    }

    /// Get a mutable pointer to the instance variable.
    ///
    /// This is useful when you want to initialize the ivar inside an `init`
    /// method (where it may otherwise not have been safely initialized yet).
    ///
    /// Note that if the ivar has already been initialized, you can use the
    /// `DerefMut` implementation to get a mutable reference.
    ///
    /// This is similar to [`MaybeUninit::as_mut_ptr`], see that for usage
    /// instructions.
    pub fn as_mut_ptr(this: &mut Self) -> *mut <Self as Deref>::Target {
        this.as_inner_mut_ptr().as_ptr().cast()
    }

    fn as_inner_mut_ptr(&mut self) -> NonNull<T::Type> {
        let ptr: NonNull<AnyObject> = NonNull::from(self).cast();

        // SAFETY: Same as `as_inner_ptr`
        let offset = unsafe { T::__offset(ptr) };
        // SAFETY: The offset is valid
        unsafe { AnyObject::ivar_at_offset::<T::Type>(ptr, offset) }
    }

    /// Sets the value of the instance variable.
    ///
    /// This is useful when you want to initialize the ivar inside an `init`
    /// method (where it may otherwise not have been safely initialized yet).
    ///
    /// This is similar to [`MaybeUninit::write`], see that for usage
    /// instructions.
    pub fn write(this: &mut Self, val: <Self as Deref>::Target) -> &mut <Self as Deref>::Target {
        let ptr: *mut <T::Type as InnerIvarType>::Output = Self::as_mut_ptr(this);
        let ptr: *mut MaybeUninit<<T::Type as InnerIvarType>::Output> = ptr.cast();
        let ivar = unsafe { ptr.as_mut().unwrap_unchecked() };
        ivar.write(val)
    }
}

impl<T: IvarType> Deref for Ivar<T> {
    type Target = <T::Type as InnerIvarType>::Output;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: User ensures that the `Ivar<T>` is only used when the ivar
        // exists, has the correct type, and has been properly initialized.
        //
        // Since all accesses to a particular ivar only goes through one
        // `Ivar`, if we have `&Ivar` we know that `&T` is safe.
        unsafe { self.as_inner_ptr().as_ref().__deref() }
    }
}

impl<T: IvarType> DerefMut for Ivar<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: User ensures that the `Ivar<T>` is only used when the ivar
        // exists, has the correct type, and has been properly initialized.
        //
        // Safe as mutable because there is only one access to a
        // particular ivar at a time (since we have `&mut self`).

        // Note: We're careful not to create `&mut AnyObject` because the user
        // might have two mutable references to different ivars, as such:
        //
        // ```
        // #[repr(C)]
        // struct X {
        //     inner: AnyObject,
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
        unsafe { self.as_inner_mut_ptr().as_mut().__deref_mut() }
    }
}

/// Format as a pointer to the instance variable.
impl<T: IvarType> fmt::Pointer for Ivar<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&Self::as_ptr(self), f)
    }
}

#[cfg(test)]
mod tests {
    use core::mem;
    use core::panic::{RefUnwindSafe, UnwindSafe};
    use std::sync::atomic::{AtomicBool, Ordering};

    use super::*;
    use crate::declare::{IvarBool, IvarEncode};
    use crate::mutability::Mutable;
    use crate::rc::Id;
    use crate::runtime::NSObject;
    use crate::{declare_class, msg_send, msg_send_id, test_utils, ClassType, MessageReceiver};

    struct TestIvar;

    unsafe impl IvarType for TestIvar {
        type Type = IvarEncode<u32>;
        const NAME: &'static str = "_foo";
    }

    #[repr(C)]
    struct IvarTestObject {
        inner: NSObject,
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

    #[test]
    fn ensure_custom_drop_is_possible() {
        static HAS_RUN_DEALLOC: AtomicBool = AtomicBool::new(false);

        declare_class!(
            #[derive(Debug, PartialEq, Eq)]
            struct CustomDrop {
                ivar: IvarEncode<u8, "_ivar">,
                ivar_bool: IvarBool<"_ivar_bool">,
            }

            mod customdrop;

            unsafe impl ClassType for CustomDrop {
                type Super = NSObject;
                type Mutability = Mutable;
                const NAME: &'static str = "CustomDrop";
            }
        );

        impl Drop for CustomDrop {
            fn drop(&mut self) {
                HAS_RUN_DEALLOC.store(true, Ordering::Relaxed);
            }
        }

        let _: Id<CustomDrop> = unsafe { msg_send_id![CustomDrop::class(), new] };

        assert!(HAS_RUN_DEALLOC.load(Ordering::Relaxed));
    }
}
