use std::fmt;
use std::hash;
use std::marker::{PhantomData, PhantomFn};
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr;

use {Message, ToMessage};
use runtime::Object;

#[link(name = "objc", kind = "dylib")]
extern {
    fn objc_retain(obj: *mut Object) -> *mut Object;
    fn objc_release(obj: *mut Object);
}

unsafe fn retain<T: Message>(ptr: *mut T) -> *mut T {
    objc_retain(ptr as *mut Object) as *mut T
}

unsafe fn release<T: Message>(ptr: *mut T) {
    objc_release(ptr as *mut Object);
}

/// A type used to mark that a struct owns the object(s) it contains,
/// so it has the sole references to them.
pub enum Owned { }
/// A type used to mark that the object(s) a struct contains are shared,
/// so there may be other references to them.
pub enum Shared { }

/// A type that marks what type of ownership a struct has over the object(s)
/// it contains; specifically, either `Owned` or `Shared`.
pub trait Ownership : 'static + PhantomFn<Self> { }
impl Ownership for Owned { }
impl Ownership for Shared { }

/// A pointer type for Objective-C's reference counted objects. The object of
/// an `Id` is retained and sent a `release` message when the `Id` is dropped.
///
/// An `Id` may be either `Owned` or `Shared`, represented by the types `Id`
/// and `ShareId`, respectively. If owned, there are no other references to the
/// object and the `Id` can be mutably dereferenced. `ShareId`, however, can
/// only be immutably dereferenced because there may be other references to the
/// object, but a `ShareId` can be cloned to provide more references to the
/// object. An owned `Id` can be "downgraded" freely to a `ShareId`, but there
/// is no way to safely upgrade back.
#[unsafe_no_drop_flag]
pub struct Id<T, O = Owned> {
    ptr: *mut T,
    own: PhantomData<O>,
}

impl<T, O> Id<T, O> where T: Message, O: Ownership {
    unsafe fn from_ptr_unchecked(ptr: *mut T) -> Id<T, O> {
        Id { ptr: ptr, own: PhantomData }
    }

    /// Constructs an `Id` from a pointer to an unretained object and
    /// retains it. Panics if the pointer is null.
    /// Unsafe because the pointer must be to a valid object and
    /// the caller must ensure the ownership is correct.
    pub unsafe fn from_ptr(ptr: *mut T) -> Id<T, O> {
        assert!(!ptr.is_null(), "Attempted to construct an Id from a null pointer");
        let ptr = retain(ptr);
        Id::from_ptr_unchecked(ptr)
    }

    /// Constructs an `Id` from a pointer to a retained object; this won't
    /// retain the pointer, so the caller must ensure the object has a +1
    /// retain count. Panics if the pointer is null.
    /// Unsafe because the pointer must be to a valid object and
    /// the caller must ensure the ownership is correct.
    pub unsafe fn from_retained_ptr(ptr: *mut T) -> Id<T, O> {
        assert!(!ptr.is_null(), "Attempted to construct an Id from a null pointer");
        Id::from_ptr_unchecked(ptr)
    }
}

impl<T> Id<T, Owned> where T: Message {
    /// "Downgrade" an owned `Id` to a `ShareId`, allowing it to be cloned.
    pub fn share(self) -> ShareId<T> {
        unsafe {
            let ptr = self.ptr;
            mem::forget(self);
            Id::from_ptr_unchecked(ptr)
        }
    }
}

impl<T, O> ToMessage for Id<T, O> where T: Message {
    fn as_id_ptr(&self) -> *mut Object {
        self.ptr as *mut Object
    }
}

impl<T> Clone for Id<T, Shared> where T: Message {
    fn clone(&self) -> ShareId<T> {
        unsafe {
            let ptr = retain(self.ptr);
            Id::from_ptr_unchecked(ptr)
        }
    }
}

#[unsafe_destructor]
impl<T, O> Drop for Id<T, O> where T: Message {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            let ptr = mem::replace(&mut self.ptr, ptr::null_mut());
            unsafe {
                release(ptr);
            }
        }
    }
}

unsafe impl<T, O> Sync for Id<T, O> where T: Sync { }

unsafe impl<T> Send for Id<T, Owned> where T: Send { }

unsafe impl<T> Send for Id<T, Shared> where T: Sync { }

impl<T, O> Deref for Id<T, O> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

impl<T> DerefMut for Id<T, Owned> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr }
    }
}

impl<T, O> PartialEq for Id<T, O> where T: PartialEq {
    fn eq(&self, other: &Id<T, O>) -> bool {
        self.deref() == other.deref()
    }

    fn ne(&self, other: &Id<T, O>) -> bool {
        self.deref() != other.deref()
    }
}

impl<T, O> Eq for Id<T, O> where T: Eq { }

impl<T, O> hash::Hash for Id<T, O> where T: hash::Hash {
    fn hash<H>(&self, state: &mut H) where H: hash::Hasher {
        self.deref().hash(state)
    }
}

impl<T, O> fmt::Debug for Id<T, O> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.deref().fmt(f)
    }
}

/// A convenient alias for a shared `Id`.
pub type ShareId<T> = Id<T, Shared>;

/// Extension methods for slices containing `Id`s.
pub trait IdSlice {
    /// The type of the items in the slice.
    type Item;

    /// Convert a slice of `Id`s into a slice of references
    fn as_refs_slice(&self) -> &[&Self::Item];
}

impl<T, O> IdSlice for [Id<T, O>] {
    type Item = T;

    fn as_refs_slice(&self) -> &[&T] {
        unsafe {
            mem::transmute(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem;

    use runtime::Object;
    use test_utils;
    use super::Id;

    fn retain_count(obj: &Object) -> usize {
        unsafe { msg_send![obj, retainCount] }
    }

    #[test]
    fn test_clone() {
        let obj = test_utils::sample_object();
        assert!(retain_count(&obj) == 1);

        let obj = obj.share();
        assert!(retain_count(&obj) == 1);

        let cloned = obj.clone();
        assert!(retain_count(&cloned) == 2);
        assert!(retain_count(&obj) == 2);

        drop(obj);
        assert!(retain_count(&cloned) == 1);
    }

    #[test]
    fn test_size() {
        let id_size = mem::size_of::<Id<Object>>();
        let ptr_size = mem::size_of::<*const Object>();
        assert!(id_size == ptr_size);
    }
}
