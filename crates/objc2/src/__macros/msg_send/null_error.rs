use core::ffi::CStr;
use std::sync::OnceLock;

use crate::ffi::NSInteger;
use crate::rc::{autoreleasepool, Retained};
use crate::runtime::{AnyClass, NSObject};
use crate::{msg_send, ClassType};

// Marked `#[cold]` to tell the optimizer that errors are comparatively rare.
//
// And intentionally not `#[inline]`, we'll let the optimizer figure out if it
// wants to do that or not.
#[cold]
pub(crate) unsafe fn encountered_error<E: ClassType>(err: *mut E) -> Retained<E> {
    // SAFETY: Caller ensures that the pointer is valid or null.
    unsafe { Retained::retain(err) }.unwrap_or_else(|| {
        let err = null_error();
        assert!(E::IS_NSERROR_COMPATIBLE);
        // SAFETY: Just checked (via `const` assertion) that the `E` type is
        // either `NSError` or `NSObject`, and hence it is valid to cast the
        // `NSObject` that we have here to that.
        unsafe { Retained::cast_unchecked(err) }
    })
}

/// Poor mans string equality in `const`. Implements `a == b`.
///
/// FIXME(MSRV): Use newer APIs for implementing this.
const fn is_eq(a: &CStr, b: &[u8]) -> bool {
    let ptr = a.as_ptr();

    let mut i = 0;
    while i < b.len() {
        let c = unsafe { *ptr.add(i) };
        if (c as u8) != b[i] {
            return false;
        }
        if c == 0 {
            return false;
        }
        i += 1;
    }

    unsafe { *ptr.add(i) == 0 }
}

// TODO: Use inline `const` once in MSRV (or add proper trait bounds).
trait IsNSError {
    const IS_NSERROR_COMPATIBLE: bool;
}

impl<T: ClassType> IsNSError for T {
    const IS_NSERROR_COMPATIBLE: bool = {
        if is_eq(T::NAME, b"NSError") || is_eq(T::NAME, b"NSObject") {
            true
        } else {
            // The post monomorphization error here is not nice, but it's
            // better than UB because the user used a type that cannot be
            // converted to NSError.
            //
            // TODO: Add a trait bound or similar instead.
            panic!("error parameter must be either `NSError` or `NSObject`")
        }
    };
}

#[cold] // Mark the NULL error branch as cold
fn null_error() -> Retained<NSObject> {
    static CACHED_NULL_ERROR: OnceLock<NSErrorWrapper> = OnceLock::new();

    // We use a OnceLock here, since performance doesn't really matter, and
    // using an AtomicPtr would leak under (very) high initialization
    // contention.
    CACHED_NULL_ERROR.get_or_init(create_null_error).0.clone()
}

struct NSErrorWrapper(Retained<NSObject>);

// SAFETY: NSError is immutable and thread safe.
unsafe impl Send for NSErrorWrapper {}
unsafe impl Sync for NSErrorWrapper {}

#[cold] // Mark the error creation branch as cold
fn create_null_error() -> NSErrorWrapper {
    // Wrap creation in an autoreleasepool, since we don't know anything about
    // the outside world, and we don't want to appear to leak.
    autoreleasepool(|_| {
        // TODO: Replace with c string literals once in MSRV.

        // SAFETY: The string is NUL terminated.
        let cls = unsafe { CStr::from_bytes_with_nul_unchecked(b"NSString\0") };
        // Intentional dynamic lookup, we don't know if Foundation is linked.
        let cls = AnyClass::get(cls).unwrap_or_else(foundation_not_linked);

        // SAFETY: The string is NUL terminated.
        let domain = unsafe { CStr::from_bytes_with_nul_unchecked(b"__objc2.missingError\0") };
        // SAFETY: The signate is correct, and the string is UTF-8 encoded and
        // NUL terminated.
        let domain: Retained<NSObject> =
            unsafe { msg_send![cls, stringWithUTF8String: domain.as_ptr()] };

        // SAFETY: The string is valid.
        let cls = unsafe { CStr::from_bytes_with_nul_unchecked(b"NSError\0") };
        // Intentional dynamic lookup, we don't know if Foundation is linked.
        let cls = AnyClass::get(cls).unwrap_or_else(foundation_not_linked);

        let domain: &NSObject = &domain;
        let code: NSInteger = 0;
        let user_info: Option<&NSObject> = None;
        // SAFETY: The signature is correct.
        let err: Retained<NSObject> =
            unsafe { msg_send![cls, errorWithDomain: domain, code: code, userInfo: user_info] };
        NSErrorWrapper(err)
    })
}

fn foundation_not_linked() -> &'static AnyClass {
    panic!("Foundation must be linked to get a proper error message on NULL errors")
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use super::*;

    fn c(s: &str) -> CString {
        CString::new(s).unwrap()
    }

    #[test]
    fn test_is_eq() {
        assert!(is_eq(&c("NSError"), b"NSError"));
        assert!(!is_eq(&c("nserror"), b"NSError"));
        assert!(!is_eq(&c("CFError"), b"NSError"));
        assert!(!is_eq(&c("NSErr"), b"NSError"));
        assert!(!is_eq(&c("NSErrorrrr"), b"NSError"));
    }

    #[test]
    fn test_create() {
        let _ = create_null_error().0;
    }
}
