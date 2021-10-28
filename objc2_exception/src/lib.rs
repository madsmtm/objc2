//! Rust interface for Objective-C's `@throw` and `@try`/`@catch` statements.
//!
//! See the following links for more information:
//! - <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Exceptions/Tasks/HandlingExceptions.html>
//! - <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjectiveC/Chapters/ocExceptionHandling.html>
//! - <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Exceptions/Exceptions.html>
//! - <https://llvm.org/docs/ExceptionHandling.html>

#![no_std]
#![warn(missing_docs)]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2_exception/0.1.2")]

#[cfg(test)]
extern crate alloc;

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
extern "C" {}

use core::ffi::c_void;
use core::mem;
use core::ptr;

use objc2_sys::{objc_exception_throw, objc_object};

extern "C" {
    fn rust_objc_try_catch_exception(
        f: extern "C" fn(*mut c_void),
        context: *mut c_void,
        error: *mut *mut objc_object,
    ) -> u8; // std::os::raw::c_uchar
}

/// An opaque type representing any Objective-C object thrown as an exception.
///
/// In the future this will be an [`extern type`][rfc-1861], if that gets
/// stabilized.
///
/// [rfc-1861]: https://rust-lang.github.io/rfcs/1861-extern-types.html
pub type Exception = objc_object;

/// Throws an Objective-C exception.
///
/// The argument must be a pointer to an Objective-C object.
///
/// # Safety
///
/// This unwinds from Objective-C, and the exception must be caught using an
/// Objective-C exception handler.
///
/// This also invokes undefined behaviour until `C-unwind` is stabilized, see
/// [RFC-2945].
///
/// [RFC-2945]: https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html
#[inline]
pub unsafe fn throw(exception: *mut Exception) -> ! {
    objc_exception_throw(exception)
}

unsafe fn try_no_ret<F: FnOnce()>(closure: F) -> Result<(), *mut Exception> {
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
        Err(exception)
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
/// The given closure must not panic.
///
/// Additionally, this unwinds through the closure from Objective-C, which is
/// undefined behaviour until `C-unwind` is stabilized, see [RFC-2945].
///
/// [RFC-2945]: https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html
pub unsafe fn r#try<R>(closure: impl FnOnce() -> R) -> Result<R, *mut Exception> {
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
    use core::ptr;

    use super::{r#try, throw};

    #[test]
    fn test_try() {
        unsafe {
            let s = "Hello".to_string();
            let result = r#try(move || {
                if !s.is_empty() {
                    throw(ptr::null_mut());
                }
                s.len()
            });
            assert!(result.unwrap_err() == ptr::null_mut());

            let mut s = "Hello".to_string();
            let result = r#try(move || {
                s.push_str(", World!");
                s
            });
            assert!(result.unwrap() == "Hello, World!");
        }
    }
}
