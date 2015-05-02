use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::ptr;

use objc::Message;
use objc::runtime::Object;

use {Id, ShareId};

#[link(name = "objc", kind = "dylib")]
extern {
    fn objc_storeWeak(location: *mut *mut Object, obj: *mut Object) -> *mut Object;
    fn objc_loadWeakRetained(location: *mut *mut Object) -> *mut Object;
    fn objc_destroyWeak(addr: *mut *mut Object);
}

// Our pointer must have the same address even if we are moved, so Box it.
// Although loading the WeakPtr may modify the pointer, it is thread safe,
// so we must use an UnsafeCell to get a *mut without self being mutable.
struct WeakPtr(Box<UnsafeCell<*mut Object>>);

impl WeakPtr {
    fn new(obj: *mut Object) -> WeakPtr {
        let ptr = Box::new(UnsafeCell::new(ptr::null_mut()));
        unsafe {
            objc_storeWeak(ptr.get(), obj);
        }
        WeakPtr(ptr)
    }

    fn load(&self) -> *mut Object {
        unsafe {
            objc_loadWeakRetained(self.0.get())
        }
    }
}

impl Drop for WeakPtr {
    fn drop(&mut self) {
        unsafe {
            objc_destroyWeak(self.0.get());
        }
    }
}

/// A pointer type for a weak reference to an Objective-C reference counted
/// object.
pub struct WeakId<T> {
    ptr: WeakPtr,
    item: PhantomData<T>,
}

impl<T> WeakId<T> where T: Message {
    /// Construct a new `WeakId` referencing the given `ShareId`.
    pub fn new(obj: &ShareId<T>) -> WeakId<T> {
        let obj_ptr: *const T = &**obj;
        WeakId {
            ptr: WeakPtr::new(obj_ptr as *mut Object),
            item: PhantomData,
        }
    }

    /// Load a `ShareId` from the `WeakId` if the object still exists.
    /// Returns `None` if the object has been deallocated.
    pub fn load(&self) -> Option<ShareId<T>> {
        let obj = self.ptr.load();
        if obj.is_null() {
            None
        } else {
            unsafe { Some(Id::from_retained_ptr(obj as *mut T)) }
        }
    }
}

unsafe impl<T> Sync for WeakId<T> where T: Sync { }

unsafe impl<T> Send for WeakId<T> where T: Sync { }

#[cfg(test)]
mod tests {
    use objc::runtime::{Class, Object};
    use {Id, ShareId};
    use super::WeakId;

    #[test]
    fn test_weak() {
        let cls = Class::get("NSObject").unwrap();
        let obj: ShareId<Object> = unsafe {
            let obj: *mut Object = msg_send![cls, alloc];
            let obj: *mut Object = msg_send![obj, init];
            Id::from_retained_ptr(obj)
        };

        let weak = WeakId::new(&obj);
        let strong = weak.load().unwrap();
        let strong_ptr: *const Object = &*strong;
        let obj_ptr: *const Object = &*obj;
        assert!(strong_ptr == obj_ptr);
        drop(strong);

        drop(obj);
        assert!(weak.load().is_none());
    }
}
