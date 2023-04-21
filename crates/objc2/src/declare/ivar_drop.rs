use alloc::boxed::Box;
use core::ffi::c_void;

use crate::encode::{Encode, Encoding};
use crate::rc::Id;
use crate::Message;

use super::InnerIvarType;

mod private {
    /// # Safety
    ///
    /// The inner type must be safe to zero-initialize.
    pub unsafe trait IvarDropHelper {
        type Inner;
    }
}

/// Ivar types that may drop.
///
/// This currently works with the following types:
/// - `Box<T>`
/// - `Option<Box<T>>`
/// - `Id<T>`
/// - `Option<Id<T>>`
///
/// Further may be added when the standard library guarantee their layout.
#[repr(transparent)]
pub struct IvarDrop<T: private::IvarDropHelper>(<T as private::IvarDropHelper>::Inner);

impl<T: private::IvarDropHelper> super::ivar::private::Sealed for IvarDrop<T> {}

// Note that we use `*const c_void` and not `*const T` to allow _any_ type,
// not just types that can be encoded by Objective-C
unsafe impl<T: Sized> Encode for IvarDrop<Box<T>> {
    const ENCODING: Encoding = <*const c_void>::ENCODING;
}

// SAFETY: `Option<Box<T>>` is safe to zero-initialize
unsafe impl<T: Sized> private::IvarDropHelper for Box<T> {
    type Inner = Option<Box<T>>;
}

// SAFETY: The memory layout of `Box<T: Sized>` is guaranteed to be a pointer:
// <https://doc.rust-lang.org/1.62.1/std/boxed/index.html#memory-layout>
//
// The user ensures that the Box has been initialized in an `init` method
// before being used.
unsafe impl<T: Sized> InnerIvarType for IvarDrop<Box<T>> {
    type Output = Box<T>;

    #[inline]
    unsafe fn __deref(&self) -> &Self::Output {
        match &self.0 {
            Some(inner) => inner,
            None => unsafe { box_unreachable() },
        }
    }

    #[inline]
    unsafe fn __deref_mut(&mut self) -> &mut Self::Output {
        match &mut self.0 {
            Some(inner) => inner,
            None => unsafe { box_unreachable() },
        }
    }
}

unsafe impl<T: Sized> Encode for IvarDrop<Option<Box<T>>> {
    const ENCODING: Encoding = <*const c_void>::ENCODING;
}

// SAFETY: `Option<Box<T>>` is safe to zero-initialize
unsafe impl<T: Sized> private::IvarDropHelper for Option<Box<T>> {
    type Inner = Option<Box<T>>;
}

// SAFETY: `Option<Box<T>>` guarantees the null-pointer optimization, so for
// `T: Sized` the layout is just a pointer:
// <https://doc.rust-lang.org/1.62.1/std/option/index.html#representation>
//
// This is valid to initialize as all-zeroes, so the user doesn't have to do
// anything to initialize it.
unsafe impl<T: Sized> InnerIvarType for IvarDrop<Option<Box<T>>> {
    type Output = Option<Box<T>>;

    #[inline]
    unsafe fn __deref(&self) -> &Self::Output {
        &self.0
    }

    #[inline]
    unsafe fn __deref_mut(&mut self) -> &mut Self::Output {
        &mut self.0
    }
}

unsafe impl<T: Message> Encode for IvarDrop<Id<T>> {
    const ENCODING: Encoding = <*const T>::ENCODING;
}

// SAFETY: `Option<Id<T>>` is safe to zero-initialize
unsafe impl<T: Message> private::IvarDropHelper for Id<T> {
    type Inner = Option<Id<T>>;
}

// SAFETY: `Id` is `NonNull<T>`, and hence safe to store as a pointer.
//
// The user ensures that the Id has been initialized in an `init` method
// before being used.
//
// Note: We could technically do `impl InnerIvarType for Ivar<Id<T>>`
// directly today, but since we can't do so for `Box` (because that is
// `#[fundamental]`), I think it makes sense to handle them similarly.
unsafe impl<T: Message> InnerIvarType for IvarDrop<Id<T>> {
    type Output = Id<T>;

    #[inline]
    unsafe fn __deref(&self) -> &Self::Output {
        match &self.0 {
            Some(inner) => inner,
            None => unsafe { id_unreachable() },
        }
    }

    #[inline]
    unsafe fn __deref_mut(&mut self) -> &mut Self::Output {
        match &mut self.0 {
            Some(inner) => inner,
            None => unsafe { id_unreachable() },
        }
    }
}

unsafe impl<T: Message> Encode for IvarDrop<Option<Id<T>>> {
    const ENCODING: Encoding = <*const T>::ENCODING;
}

// SAFETY: `Option<Id<T>>` is safe to zero-initialize
unsafe impl<T: Message> private::IvarDropHelper for Option<Id<T>> {
    type Inner = Option<Id<T>>;
}

// SAFETY: `Id<T>` guarantees the null-pointer optimization.
//
// This is valid to initialize as all-zeroes, so the user doesn't have to do
// anything to initialize it.
unsafe impl<T: Message> InnerIvarType for IvarDrop<Option<Id<T>>> {
    type Output = Option<Id<T>>;

    #[inline]
    unsafe fn __deref(&self) -> &Self::Output {
        &self.0
    }

    #[inline]
    unsafe fn __deref_mut(&mut self) -> &mut Self::Output {
        &mut self.0
    }
}

// TODO: Allow the following once their layout is guaranteed by `std`:
// - Arc<T>
// - Option<Arc<T>>
// - sync::Weak<T>
// - Rc<T>
// - Option<Rc<T>>
// - rc::Weak<T>
// - Vec<T>
// - String

// TODO: Allow `WeakId` once we figure out how to allow it being initialized
// by default.

#[inline]
#[track_caller]
unsafe fn id_unreachable() -> ! {
    #[cfg(debug_assertions)]
    {
        unreachable!("an Id in instance variables must always be initialized before use!")
    }
    // SAFETY: Checked by caller
    #[cfg(not(debug_assertions))]
    unsafe {
        core::hint::unreachable_unchecked()
    }
}

#[inline]
#[track_caller]
unsafe fn box_unreachable() -> ! {
    #[cfg(debug_assertions)]
    {
        unreachable!("a Box in instance variables must always be initialized before use!")
    }
    // SAFETY: Checked by caller
    #[cfg(not(debug_assertions))]
    unsafe {
        core::hint::unreachable_unchecked()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::declare::{Ivar, IvarType};
    use crate::mutability::Mutable;
    use crate::rc::{Allocated, __RcTestObject, __ThreadTestData};
    use crate::runtime::NSObject;
    use crate::{declare_class, msg_send, msg_send_id, ClassType};

    struct TestIvar1;
    unsafe impl IvarType for TestIvar1 {
        type Type = IvarDrop<Box<u8>>;
        const NAME: &'static str = "_abc";
    }

    struct TestIvar2;
    unsafe impl IvarType for TestIvar2 {
        type Type = IvarDrop<Option<Box<u8>>>;
        const NAME: &'static str = "_abc";
    }

    struct TestIvar3;
    unsafe impl IvarType for TestIvar3 {
        type Type = IvarDrop<Id<NSObject>>;
        const NAME: &'static str = "_abc";
    }

    struct TestIvar4;
    unsafe impl IvarType for TestIvar4 {
        type Type = IvarDrop<Option<Id<NSObject>>>;
        const NAME: &'static str = "_abc";
    }

    declare_class!(
        #[derive(Debug, PartialEq, Eq)]
        struct IvarTester {
            ivar1: IvarDrop<Id<__RcTestObject>, "_ivar1">,
            ivar2: IvarDrop<Option<Id<__RcTestObject>>, "_ivar2">,
            ivar3: IvarDrop<Box<Id<__RcTestObject>>, "_ivar3">,
            ivar4: IvarDrop<Option<Box<Id<__RcTestObject>>>, "_ivar4">,
        }

        mod ivartester;

        unsafe impl ClassType for IvarTester {
            type Super = NSObject;
            type Mutability = Mutable;
            const NAME: &'static str = "IvarTester";
        }

        unsafe impl IvarTester {
            #[method(init)]
            fn init(&mut self) -> Option<&mut Self> {
                let this: Option<&mut Self> = unsafe { msg_send![super(self), init] };
                this.map(|this| {
                    Ivar::write(&mut this.ivar1, __RcTestObject::new());
                    *this.ivar2 = Some(__RcTestObject::new());
                    Ivar::write(&mut this.ivar3, Box::new(__RcTestObject::new()));
                    *this.ivar4 = Some(Box::new(__RcTestObject::new()));
                    this
                })
            }

            #[method(initInvalid)]
            fn init_invalid(&mut self) -> Option<&mut Self> {
                // Don't actually initialize anything here; this creates an
                // invalid instance, where accessing the two ivars `ivar1`
                // and `ivar3` is UB
                unsafe { msg_send![super(self), init] }
            }
        }
    );

    declare_class!(
        #[derive(Debug, PartialEq, Eq)]
        struct IvarTesterSubclass {
            ivar5: IvarDrop<Id<__RcTestObject>, "_ivar5">,
        }

        mod ivartestersubclass;

        unsafe impl ClassType for IvarTesterSubclass {
            type Super = IvarTester;
            type Mutability = Mutable;
            const NAME: &'static str = "IvarTesterSubclass";
        }

        unsafe impl IvarTesterSubclass {
            #[method(init)]
            fn init(&mut self) -> Option<&mut Self> {
                let this: Option<&mut Self> = unsafe { msg_send![super(self), init] };
                this.map(|this| {
                    Ivar::write(&mut this.ivar5, __RcTestObject::new());
                    this
                })
            }
        }
    );

    #[test]
    fn test_alloc_dealloc() {
        let expected = __ThreadTestData::current();

        let obj: Allocated<IvarTester> = unsafe { msg_send_id![IvarTester::class(), alloc] };
        expected.assert_current();

        drop(obj);
        expected.assert_current();
    }

    #[test]
    fn test_init_drop() {
        let mut expected = __ThreadTestData::current();

        let mut obj: Id<IvarTester> = unsafe { msg_send_id![IvarTester::class(), new] };
        expected.alloc += 4;
        expected.init += 4;
        expected.assert_current();

        *obj.ivar1 = (*obj.ivar1).clone();
        expected.retain += 1;
        expected.release += 1;
        expected.assert_current();

        *obj.ivar2 = None;
        expected.release += 1;
        expected.dealloc += 1;
        expected.assert_current();

        drop(obj);
        expected.release += 3;
        expected.dealloc += 3;
        expected.assert_current();
    }

    #[test]
    fn test_subclass() {
        let mut expected = __ThreadTestData::current();

        let mut obj: Id<IvarTesterSubclass> =
            unsafe { msg_send_id![IvarTesterSubclass::class(), new] };
        expected.alloc += 5;
        expected.init += 5;
        expected.assert_current();

        *obj.ivar5 = (*obj.ivar1).clone();
        expected.retain += 1;
        expected.release += 1;
        expected.dealloc += 1;
        expected.assert_current();

        drop(obj);
        expected.release += 5;
        expected.dealloc += 4;
        expected.assert_current();
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only panics in debug mode")]
    #[should_panic = "an Id in instance variables must always be initialized before use"]
    fn test_init_invalid_ref() {
        let obj: Id<IvarTester> = unsafe { msg_send_id![IvarTester::alloc(), initInvalid] };

        std::println!("{:?}", obj.ivar1);
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only panics in debug mode")]
    #[should_panic = "an Id in instance variables must always be initialized before use"]
    fn test_init_invalid_mut() {
        let mut obj: Id<IvarTester> = unsafe { msg_send_id![IvarTester::alloc(), initInvalid] };

        *obj.ivar1 = __RcTestObject::new();
    }
}
