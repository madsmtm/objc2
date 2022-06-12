//! Objective-C's @throw and @try/@catch.
//!
//! By default, if the [`msg_send!`] macro causes an exception to be thrown,
//! this will unwind into Rust, resulting in undefined behavior. However, this
//! crate has an `"catch_all"` feature which, when enabled, wraps each
//! [`msg_send!`] in a [`catch`] and panics if an exception is caught,
//! preventing Objective-C from unwinding into Rust.
//!
//! This module is only available when the `"exception"` feature is enabled.
//!
//! See the following links for more information:
//! - <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Exceptions/Tasks/HandlingExceptions.html>
//! - <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjectiveC/Chapters/ocExceptionHandling.html>
//! - <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Exceptions/Exceptions.html>
//! - <https://llvm.org/docs/ExceptionHandling.html>

// TODO: Test this with panic=abort, and ensure that the code-size is
// reasonable in that case.

use core::ffi::c_void;
use core::mem;
use core::ptr;

use crate::ffi;
use crate::rc::{Id, Shared};
use crate::runtime::Object;

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
        Some(id) => id.as_ptr() as *mut ffi::objc_object,
        None => ptr::null_mut(),
    };
    unsafe { ffi::objc_exception_throw(exception) }
}

unsafe fn try_no_ret<F: FnOnce()>(closure: F) -> Result<(), Option<Id<Object, Shared>>> {
    extern "C" fn try_objc_execute_closure<F: FnOnce()>(closure: &mut Option<F>) {
        // This is always passed Some, so it's safe to unwrap
        let closure = closure.take().unwrap();
        closure();
    }

    let f: extern "C" fn(&mut Option<F>) = try_objc_execute_closure;
    let f: extern "C" fn(*mut c_void) = unsafe { mem::transmute(f) };
    // Wrap the closure in an Option so it can be taken
    let mut closure = Some(closure);
    let context: *mut Option<F> = &mut closure;
    let context = context.cast();

    let mut exception = ptr::null_mut();
    let success = unsafe { ffi::rust_objc_sys_0_2_try_catch_exception(f, context, &mut exception) };

    if success == 0 {
        Ok(())
    } else {
        // SAFETY:
        // The exception is always a valid object or NULL.
        //
        // The ownership is safe as Shared; Objective-C code throwing an
        // exception knows that they don't hold sole access to that exception
        // instance any more, and Rust code is forbidden by requiring a Shared
        // Id in `throw` (instead of just a shared reference, which could have
        // come from an Owned Id).
        Err(unsafe { Id::new(exception.cast()) })
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
    let value_ref = &mut value;
    let closure = move || {
        *value_ref = Some(closure());
    };
    let result = unsafe { try_no_ret(closure) };
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
    // TODO: `NULL` exceptions are invalid on 32-bit / w. fragile runtime
    #[cfg(target_pointer_width = "64")]
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
        let obj: Id<Object, Shared> = unsafe { Id::new(msg_send![class!(NSObject), new]).unwrap() };

        let result = unsafe { catch(|| throw(Some(&obj))) };
        let exception = result.unwrap_err().unwrap();
        assert!(ptr::eq(&*exception, &*obj));
    }
}
