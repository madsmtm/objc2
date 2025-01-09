use core::ptr::NonNull;

use crate::CFRetained;

/// A CoreFoundation-like type.
///
/// This trait is implemented for all CoreFoundation-like types (i.e. both
/// types in this crate like `CFString`, but also types from other frameworks
/// like `CGColor` from CoreGraphics).
///
/// This trait allows the type to be used in [`CFRetained`].
///
/// You should not need to implement this yourself.
///
///
/// # Safety
///
/// - The type must be a CoreFoundation-like type.
/// - The type must be safe to retain/release using `CFRetain` and
///   `CFRelease`.
pub unsafe trait Type {
    /// Increment the reference count of the receiver.
    ///
    /// This extends the duration in which the receiver is alive by detaching
    /// it from the lifetime information carried by the reference.
    ///
    /// This is similar to using [`Clone` on `CFRetained<Self>`][clone-id],
    /// with the addition that it can be used on a plain reference.
    ///
    /// [clone-id]: crate::CFRetained#impl-Clone-for-CFRetained<T>
    #[inline]
    #[doc(alias = "CFRetain")]
    fn retain(&self) -> CFRetained<Self>
    where
        Self: Sized,
    {
        let ptr = NonNull::from(self);

        // SAFETY: The pointer is valid, since it came from a Rust reference.
        unsafe { CFRetained::retain(ptr) }
    }
}
