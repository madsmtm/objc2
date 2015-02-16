use std::cell::UnsafeCell;
use std::ptr;

use {Id, ShareId, Message, ToMessage};
use runtime::Object;

#[link(name = "objc", kind = "dylib")]
extern {
    fn objc_storeWeak(location: *mut *mut Object, obj: *mut Object) -> *mut Object;
    fn objc_loadWeakRetained(location: *mut *mut Object) -> *mut Object;
    fn objc_destroyWeak(addr: *mut *mut Object);
}

/// A pointer type for a weak reference to an Objective-C reference counted
/// object.
pub struct WeakId<T> {
    // Our pointer must have the same address even if we are moved, so Box it.
    // Although loading the WeakId may modify the pointer, it is thread safe,
    // so we must use an UnsafeCell to get a *mut without self being mutable.
    ptr: Box<UnsafeCell<*const T>>,
}

impl<T> WeakId<T> where T: Message {
    /// Construct a new `WeakId` referencing the given `ShareId`.
    pub fn new(obj: &ShareId<T>) -> WeakId<T> {
        let ptr = Box::new(UnsafeCell::new(ptr::null()));
        unsafe {
            let loc = ptr.get() as *mut *mut Object;
            objc_storeWeak(loc, obj.as_ptr() as *mut Object);
        }
        WeakId { ptr: ptr }
    }

    /// Load a `ShareId` from the `WeakId` if the object still exists.
    /// Returns `None` if the object has been deallocated.
    pub fn load(&self) -> Option<ShareId<T>> {
        unsafe {
            let loc = self.ptr.get() as *mut *mut Object;
            let obj = objc_loadWeakRetained(loc) as *mut T;
            if obj.is_null() {
                None
            } else {
                Some(Id::from_retained_ptr(obj))
            }
        }
    }
}

#[unsafe_destructor]
impl<T> Drop for WeakId<T> {
    fn drop(&mut self) {
        unsafe {
            let loc = self.ptr.get() as *mut *mut Object;
            objc_destroyWeak(loc);
        }
    }
}

#[cfg(test)]
mod tests {
    use runtime::Object;
    use test_utils;
    use super::WeakId;

    #[test]
    fn test_weak() {
        let obj = test_utils::sample_object().share();

        let weak = WeakId::new(&obj);
        let strong = weak.load().unwrap();
        assert!(&*strong as *const Object == &*obj as *const Object);
        drop(strong);

        drop(obj);
        assert!(weak.load().is_none());
    }
}
