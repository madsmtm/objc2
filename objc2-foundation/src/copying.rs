use core::ptr::NonNull;

use objc2::msg_send;
use objc2::rc::{Id, Owned};

use super::INSObject;

pub unsafe trait INSCopying: INSObject {
    /// This can be an [`Owned`] [`INSObject`] if and only if `copy` creates a
    /// new instance, see the following example:
    ///
    /// ```ignore
    /// let x: Id<MyObject, Owned> = MyObject::new();
    /// // This is valid only if `y` is a new instance. Otherwise `x` and `y`
    /// // would be able to create aliasing mutable references!
    /// let y: Id<MyObject, Owned> = x.copy();
    /// ```
    type Output: INSObject;

    fn copy(&self) -> Id<Self::Output, <Self::Output as INSObject>::Ownership> {
        unsafe {
            let obj: *mut Self::Output = msg_send![self, copy];
            Id::new(NonNull::new_unchecked(obj))
        }
    }
}

pub unsafe trait INSMutableCopying: INSObject {
    /// An [`Owned`] [`INSObject`] is required to be able to return an owned
    /// [`Id`].
    type Output: INSObject<Ownership = Owned>;

    fn mutable_copy(&self) -> Id<Self::Output, Owned> {
        unsafe {
            let obj: *mut Self::Output = msg_send![self, mutableCopy];
            Id::new(NonNull::new_unchecked(obj))
        }
    }
}
