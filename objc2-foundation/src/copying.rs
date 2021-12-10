use core::ptr::NonNull;

use objc2::msg_send;
use objc2::rc::{Id, Owned, Ownership};

use super::INSObject;

pub unsafe trait INSCopying: INSObject {
    /// Indicates whether the type is mutable or immutable.
    ///
    /// This can be [`Owned`] if and only if `copy` creates a new instance,
    /// see the following example:
    ///
    /// ```ignore
    /// let x: Id<MyObject, _> = MyObject::new();
    /// // This is valid only if `y` is a new instance. Otherwise `x` and `y`
    /// // would be able to create aliasing mutable references!
    /// let y: Id<MyObject, Owned> = x.copy();
    /// ```
    ///
    /// Note that for the same reason, you should be careful when defining
    /// `new` methods on your object; e.g. immutable types like [`NSString`]
    /// don't return `Id<NSString, Owned>`, because that would allow this
    /// trait to create an aliasing `Id<NSString, Shared>` (since sending the
    /// `copy` message (and others) does not create a new instance, but
    /// instead just retains the instance).
    ///
    /// [`NSString`]: crate::NSString
    type Ownership: Ownership;

    /// The output type.
    ///
    /// This is usually `Self`, but e.g. `NSMutableString` returns `NSString`.
    /// TODO: Verify???
    type Output: INSObject;

    fn copy(&self) -> Id<Self::Output, Self::Ownership> {
        unsafe {
            let obj: *mut Self::Output = msg_send![self, copy];
            Id::new(NonNull::new_unchecked(obj))
        }
    }
}

/// TODO
///
/// Note that the `mutableCopy` selector must return an owned object!
pub unsafe trait INSMutableCopying: INSObject {
    /// TODO
    type Output: INSObject;

    fn mutable_copy(&self) -> Id<Self::Output, Owned> {
        unsafe {
            let obj: *mut Self::Output = msg_send![self, mutableCopy];
            Id::new(NonNull::new_unchecked(obj))
        }
    }
}
