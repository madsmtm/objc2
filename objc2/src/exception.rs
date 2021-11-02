//! Objective-C's @throw and @try/@catch.
//!
//! This is only available when the `exception` feature is enabled.
//!
//! See the following links for more information:
//! - <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Exceptions/Tasks/HandlingExceptions.html>
//! - <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjectiveC/Chapters/ocExceptionHandling.html>
//! - <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Exceptions/Exceptions.html>
//! - <https://llvm.org/docs/ExceptionHandling.html>

use core::ffi::c_void;
use core::mem;
use core::ptr;
use core::ptr::NonNull;
use std::os::raw::c_uchar;

use crate::rc::{Id, Shared};
use crate::runtime::Object;

use objc2_sys::{objc_exception_throw, objc_object};

extern "C" {
    fn rust_objc_try_catch_exception(
        f: extern "C" fn(*mut c_void),
        context: *mut c_void,
        error: *mut *mut objc_object,
    ) -> c_uchar;
}

/// Throws an Objective-C exception.
///
/// The argument must be a pointer to an Objective-C object.
///
/// # Safety
///
/// This unwinds from Objective-C, and the exception must be caught using an
/// Objective-C exception handler like [`catch`] (and specifically not
/// [`catch_unwind`]).
///
/// This also invokes undefined behaviour until `C-unwind` is stabilized, see
/// [RFC-2945].
///
/// [`catch_unwind`]: std::panic::catch_unwind
/// [RFC-2945]: https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html
#[inline]
pub unsafe fn throw(exception: Option<&Id<Object, Shared>>) -> ! {
    let exception = match exception {
        Some(id) => &**id as *const Object as *mut objc_object,
        None => ptr::null_mut(),
    };
    objc_exception_throw(exception)
}

unsafe fn try_no_ret<F: FnOnce()>(closure: F) -> Result<(), Option<Id<Object, Shared>>> {
    extern "C" fn try_objc_execute_closure<F: FnOnce()>(closure: &mut Option<F>) {
        // This is always passed Some, so it's safe to unwrap
        let closure = closure.take().unwrap();
        closure();
    }

    let f: extern "C" fn(&mut Option<F>) = try_objc_execute_closure;
    let f: extern "C" fn(*mut c_void) = mem::transmute(f);
    // Wrap the closure in an Option so it can be taken
    let mut closure = Some(closure);
    let context = &mut closure as *mut _ as *mut c_void;

    let mut exception = ptr::null_mut();
    let success = rust_objc_try_catch_exception(f, context, &mut exception);

    if success == 0 {
        Ok(())
    } else {
        // SAFETY:
        // The exception is always a valid object (or NULL, but that has been
        // checked).
        //
        // The ownership is safe as Shared; Objective-C code throwing an
        // exception knows that they don't hold sole access to that exception
        // instance any more, and Rust code is forbidden by requiring a Shared
        // Id in `throw` (instead of just a shared reference, which could have
        // come from an Owned Id).
        Err(NonNull::new(exception as *mut Object).map(|e| Id::new(e)))
    }
}

/// Tries to execute the given closure and catches an Objective-C exception
/// if one is thrown.
///
/// Returns a `Result` that is either `Ok` if the closure succeeded without an
/// exception being thrown, or an `Err` with a pointer to an exception if one
/// was thrown. The exception is retained and so must be released.
///
/// # Safety
///
/// The given closure must not panic (e.g. normal Rust unwinding into this
/// causes undefined behaviour).
///
/// Additionally, this unwinds through the closure from Objective-C, which is
/// undefined behaviour until `C-unwind` is stabilized, see [RFC-2945].
///
/// [RFC-2945]: https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html
pub unsafe fn catch<R>(closure: impl FnOnce() -> R) -> Result<R, Option<Id<Object, Shared>>> {
    let mut value = None;
    let result = {
        let value_ref = &mut value;
        try_no_ret(move || {
            *value_ref = Some(closure());
        })
    };
    // If the try succeeded, this was set so it's safe to unwrap
    result.map(|_| value.unwrap())
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_catch() {
        let mut s = "Hello".to_string();
        let result = unsafe {
            catch(move || {
                s.push_str(", World!");
                s
            })
        };
        assert_eq!(result.unwrap(), "Hello, World!");
    }

    #[test]
    fn test_throw_catch_none() {
        let s = "Hello".to_string();
        let result = unsafe {
            catch(move || {
                if !s.is_empty() {
                    throw(None);
                }
                s.len()
            })
        };
        assert!(result.unwrap_err().is_none());
    }

    #[test]
    fn test_throw_catch_object() {
        let obj: Id<Object, Shared> = unsafe { Id::new(msg_send![class!(NSObject), new]) };

        let result = unsafe { catch(|| throw(Some(&obj))) };
        let e = result.unwrap_err().unwrap();
        // Compare pointers
        assert_eq!(&*e as *const Object, &*obj as *const Object);
    }
}
