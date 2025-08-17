use core::ptr;

use super::Object;
use crate::rc::{Id, Shared};
use crate::ffi;

/// Associated object support.
///
/// These are associated functions, since they are very rarely used, and will
/// mostly just clutter up the `Deref` chain and documentation of all other
/// classes in `icrate`.
///
/// See [Apple's documentation](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjectiveC/Chapters/ocAssociativeReferences.html).
impl Object {
    ///
    ///
    /// # Panics
    ///
    /// This may panic or abort the process if the specified object does not
    /// support associated objects.
    unsafe fn set_associated_ptr<K>(this: &Self, key: &K, value: *const ()) {
        let key: *const K = key;
        // SAFETY: The object and key is non-null
        //
        // Caller ensures that the key is uniquely used for the expected
        // operation.
        unsafe {
            ffi::objc_setAssociatedObject(this.as_ptr(), key.cast(), value as *mut _, ffi::OBJC_ASSOCIATION_ASSIGN)
        }
    }

    unsafe fn set_associated_id<K, T>(this: &Self, key: &K, value: Option<&Id<T, Shared>>) {
        let key: *const K = key;
        let ptr: *const T = value.map(|value| Id::as_ptr(value)).unwrap_or(ptr::null());
        // SAFETY: The object and key is non-null, and the value came from
        // a shared `Id`, so it is safe to retain.
        //
        // Caller ensures that the key is uniquely used for the expected
        // operation.
        unsafe {
            ffi::objc_setAssociatedObject(
                this.as_ptr(),
                key.cast(),
                ptr.cast(),
                ffi::OBJC_ASSOCIATION_RETAIN,
            )
        }
    }

    unsafe fn get_associated_ptr<K>(this: &Self, key: &K) -> *const () {
        let key: *const K = key;
        // SAFETY:
        unsafe { ffi::objc_getAssociatedObject(this.as_ptr(), key.cast()).cast() }
    }

    unsafe fn get_associated_id<K, T>(this: &Self, key: &K) -> Id<T, Shared> {
        let ptr = this.get_associated_ptr(key) as *mut T;
        // SAFETY: Caller upholds that the associated object stores an `Id`,
        // and that the `Id` was originally `Shared`.
        unsafe { Id::retain_autoreleased(ptr) }
    }

    unsafe fn remove_associated<K, T>(this: &Self, key: &K) {
        let key: *const K = key;
        // SAFETY: The object and key is non-null, and the associated is being
        // broken, so the policy doesn't matter.
        //
        // Caller ensures that the key is uniquely used for the expected
        // operation.
        unsafe {
            ffi::objc_setAssociatedObject(
                this.as_ptr(),
                key.cast(),
                ptr::null(),
                ffi::OBJC_ASSOCIATION_ASSIGN,
            )
        }
    }

    fn remove_all_associated(this: &Self) {
        // SAFETY:
        unsafe { ffi::objc_removeAssociatedObjects(this.as_ptr()) }
    }

    // objc_setAssociatedObject
    // objc_getAssociatedObject
    // objc_removeAssociatedObjects

    // https://nshipster.com/associated-objects/
}
