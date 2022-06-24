use core::ffi::c_void;

use crate::ffi;

/// A type used to mark that a struct owns the object(s) it contains,
/// so it has the sole references to them.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Owned {}

/// A type used to mark that the object(s) a struct contains are shared,
/// so there may be other references to them.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Shared {}

mod private {
    pub trait Sealed {}

    impl Sealed for super::Owned {}
    impl Sealed for super::Shared {}
}

/// A type that marks what type of ownership a struct has over the object(s)
/// it contains; specifically, either [`Owned`] or [`Shared`].
///
/// This trait is sealed and not meant to be implemented outside of the this
/// crate.
pub trait Ownership: private::Sealed + 'static {
    #[doc(hidden)]
    #[inline]
    unsafe fn __ensure_unique_if_owned(_: *mut ffi::objc_object) {}
    #[doc(hidden)]
    #[inline]
    unsafe fn __relinquish_ownership(_: *mut ffi::objc_object) {}
}

/// The value of this doesn't matter, it's only the address that we use.
static OBJC2_ID_OWNED_UNIQUE_KEY: i32 = 0;

#[inline]
#[allow(unused)]
fn key() -> *const c_void {
    &OBJC2_ID_OWNED_UNIQUE_KEY as *const i32 as *const _
}

#[cfg(feature = "unstable-verify-ownership")]
impl Ownership for Owned {
    #[track_caller]
    unsafe fn __ensure_unique_if_owned(obj: *mut ffi::objc_object) {
        std::println!("\n__ensure_unique_if_owned: {:?} / {:?}", obj, key());
        let associated = unsafe { ffi::objc_getAssociatedObject(obj, key()) };
        std::println!("associated: {:?}", associated);
        if associated.is_null() {
            // Set the associated object to something (it can be anything, so
            // we just set it to the current object).
            //
            // We use `assign` because we don't want to retain or copy the
            // object, since we just use it as a marker.
            unsafe { ffi::objc_setAssociatedObject(obj, key(), obj, ffi::OBJC_ASSOCIATION_ASSIGN) };
        } else {
            panic!("Another `Id<T, Owned>` has already been created from that object!");
        }
        let associated = unsafe { ffi::objc_getAssociatedObject(obj, key()) };
        std::println!("associated: {:?}", associated);
    }

    #[track_caller]
    unsafe fn __relinquish_ownership(obj: *mut ffi::objc_object) {
        std::println!("\n__relinquish_ownership: {:?} / {:?}", obj, key());
        let associated = unsafe { ffi::objc_getAssociatedObject(obj, key()) };
        std::println!("associated: {:?}", associated);
        if associated.is_null() {
            panic!("Tried to give up ownership of `Id<T, Owned>` that wasn't owned!");
        } else {
            // Clear the associated object
            unsafe {
                ffi::objc_setAssociatedObject(obj, key(), ffi::nil, ffi::OBJC_ASSOCIATION_ASSIGN)
            };
        }
        let associated = unsafe { ffi::objc_getAssociatedObject(obj, key()) };
        std::println!("associated: {:?}", associated);
    }
}

#[cfg(not(feature = "unstable-verify-ownership"))]
impl Ownership for Owned {}

#[cfg(feature = "unstable-verify-ownership")]
impl Ownership for Shared {
    #[track_caller]
    unsafe fn __ensure_unique_if_owned(obj: *mut ffi::objc_object) {
        std::println!("\n__ensure_unique_if_owned: {:?} / {:?}", obj, key());
        let associated = unsafe { ffi::objc_getAssociatedObject(obj, key()) };
        std::println!("associated: {:?}", associated);
        if !associated.is_null() {
            panic!("An `Id<T, Owned>` exists while trying to create `Id<T, Shared>`!");
        }
    }
    #[track_caller]
    unsafe fn __relinquish_ownership(obj: *mut ffi::objc_object) {
        std::println!("\n__relinquish_ownership: {:?} / {:?}", obj, key());
        let associated = unsafe { ffi::objc_getAssociatedObject(obj, key()) };
        std::println!("associated: {:?}", associated);
        if !associated.is_null() {
            panic!("Tried to give up ownership of `Id<T, Shared>`!");
        }
    }
}

#[cfg(not(feature = "unstable-verify-ownership"))]
impl Ownership for Shared {}
