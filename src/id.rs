use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr;

use {Encode, EncodePtr, Message, ToMessage};

/// A type used to mark that a struct owns the object(s) it contains,
/// so it has the sole references to them.
#[allow(missing_copy_implementations)]
pub enum Owned { }
/// A type used to mark that the object(s) a struct contains are shared,
/// so there may be other references to them.
#[allow(missing_copy_implementations)]
pub enum Shared { }

/// A type that marks what type of ownership a struct has over the object(s)
/// it contains; specifically, either `Owned` or `Shared`.
pub trait Ownership { }
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
pub struct Id<T, O = Owned> where T: Message, O: Ownership {
    ptr: *mut T,
}

impl<T, O> Id<T, O> where T: Message, O: Ownership {
    /// Constructs an `Id` from a pointer to an unretained object and
    /// retains it. Panics if the pointer is null.
    /// Unsafe because the pointer must be to a valid object and
    /// the caller must ensure the ownership is correct.
    pub unsafe fn from_ptr(ptr: *mut T) -> Id<T, O> {
        match Id::maybe_from_ptr(ptr) {
            Some(id) => id,
            None => panic!("Attempted to construct an Id from a null pointer"),
        }
    }

    /// Constructs an `Id` from a pointer to a retained object; this won't
    /// retain the pointer, so the caller must ensure the object has a +1
    /// retain count. Panics if the pointer is null.
    /// Unsafe because the pointer must be to a valid object and
    /// the caller must ensure the ownership is correct.
    pub unsafe fn from_retained_ptr(ptr: *mut T) -> Id<T, O> {
        match Id::maybe_from_retained_ptr(ptr) {
            Some(id) => id,
            None => panic!("Attempted to construct an Id from a null pointer"),
        }
    }

    /// Constructs an `Id` from a pointer to an unretained object and
    /// retains it if the pointer isn't null, otherwise returns None.
    /// Unsafe because the pointer must be to a valid object and
    /// the caller must ensure the ownership is correct.
    pub unsafe fn maybe_from_ptr(ptr: *mut T) -> Option<Id<T, O>> {
        // objc_msgSend is a no-op on null pointers
        let _: () = msg_send![ptr, retain];
        Id::maybe_from_retained_ptr(ptr)
    }

    /// Constructs an `Id` from a pointer to a retained object if the pointer
    /// isn't null, otherwise returns None. This won't retain the pointer,
    /// so the caller must ensure the object has a +1 retain count.
    /// Unsafe because the pointer must be to a valid object and
    /// the caller must ensure the ownership is correct.
    pub unsafe fn maybe_from_retained_ptr(ptr: *mut T) -> Option<Id<T, O>> {
        if ptr.is_null() {
            None
        } else {
            Some(Id { ptr: ptr })
        }
    }
}

impl<T> Id<T, Owned> where T: Message {
    /// "Downgrade" an owned `Id` to a `ShareId`, allowing it to be cloned.
    pub fn share(self) -> ShareId<T> {
        let ptr = self.ptr;
        unsafe {
            mem::forget(self);
        }
        Id { ptr: ptr }
    }
}

impl<T, O> Encode for Id<T, O> where T: Message, O: Ownership {
    fn code() -> &'static str {
        <T as EncodePtr>::ptr_code()
    }
}

impl<T, O> ToMessage for Id<T, O> where T: Message, O: Ownership {
    type Target = T;

    fn as_ptr(&self) -> *mut T {
        self.ptr
    }
}

impl<T> Clone for Id<T, Shared> where T: Message {
    fn clone(&self) -> ShareId<T> {
        let ptr = self.ptr;
        unsafe {
            let _: () = msg_send![ptr, retain];
        }
        Id { ptr: ptr }
    }
}

#[unsafe_destructor]
impl<T, O> Drop for Id<T, O> where T: Message, O: Ownership {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            let ptr = mem::replace(&mut self.ptr, ptr::null_mut());
            unsafe {
                let _: () = msg_send![ptr, release];
            }
        }
    }
}

impl<T, O> Deref for Id<T, O> where T: Message, O: Ownership {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

impl<T> DerefMut for Id<T, Owned> where T: Message {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr }
    }
}

impl<T, O> PartialEq for Id<T, O> where T: Message + PartialEq, O: Ownership {
    fn eq(&self, other: &Id<T, O>) -> bool {
        self.deref() == other.deref()
    }

    fn ne(&self, other: &Id<T, O>) -> bool {
        self.deref() != other.deref()
    }
}

impl<T, O> Eq for Id<T, O> where T: Message + Eq, O: Ownership { }

impl<H, T, O> Hash<H> for Id<T, O>
        where H: Hasher, T: Message + Hash<H>, O: Ownership {
    fn hash(&self, state: &mut H) {
        self.deref().hash(state)
    }
}

impl<T, O> fmt::Show for Id<T, O> where T: Message + fmt::Show, O: Ownership {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.deref().fmt(f)
    }
}

/// A convenient alias for a shared `Id`.
pub type ShareId<T> = Id<T, Shared>;

/// Extension methods for slices containing `Id`s.
pub trait IdSlice {
    type Item;

    /// Convert a slice of `Id`s into a slice of references
    fn as_refs_slice(&self) -> &[&Self::Item];
}

impl<T, O> IdSlice for [Id<T, O>] where T: Message, O: Ownership {
    type Item = T;

    fn as_refs_slice(&self) -> &[&T] {
        unsafe {
            mem::transmute(self)
        }
    }
}

/// Trait to convert to a vector of `Id`s by consuming self.
pub trait IntoIdVector {
    type Item;

    /// Converts to a vector of `Id`s by consuming self, retaining each object
    /// contained in self.
    /// Unsafe because the caller must ensure the `Id`s are constructed from
    /// valid objects and the ownership of the resulting `Id`s is correct.
    unsafe fn into_id_vec<O>(self) -> Vec<Id<Self::Item, O>> where O: Ownership;
}

impl<R: ToMessage> IntoIdVector for Vec<R> {
    type Item = R::Target;

    unsafe fn into_id_vec<O>(self) -> Vec<Id<R::Target, O>> where O: Ownership {
        self.map_in_place(|obj| Id::from_ptr(obj.as_ptr()))
    }
}
