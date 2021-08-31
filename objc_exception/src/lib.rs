//! Rust interface for Objective-C's `@throw` and `@try`/`@catch` statements.

#![no_std]

#[cfg(test)]
extern crate alloc;

use core::ffi::c_void;
use core::mem;
use core::ptr;

#[link(name = "objc", kind = "dylib")]
// TODO: "C-unwind"
extern "C" {
    /// See [`objc-exception.h`][objc-exception].
    ///
    /// [objc-exception]: https://opensource.apple.com/source/objc4/objc4-818.2/runtime/objc-exception.h.auto.html
    // Header marks this with _Nonnull, but LLVM output shows otherwise
    fn objc_exception_throw(exception: *mut c_void) -> !;
    // fn objc_exception_rethrow();
}

extern "C" {
    fn RustObjCExceptionTryCatch(
        r#try: extern "C" fn(*mut c_void),
        context: *mut c_void,
        error: *mut *mut c_void,
    ) -> u8; // std::os::raw::c_uchar
}

/// An opaque type representing any Objective-C object thrown as an exception.
///
/// In the future this will be an [`extern type`][rfc-1861], if that gets
/// stabilized.
///
/// [rfc-1861]: https://rust-lang.github.io/rfcs/1861-extern-types.html
#[repr(C)]
pub struct Exception {
    /// See the [Nomicon] for details on representing opaque structs.
    ///
    /// [Nomicon]: https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs
    _priv: [u8; 0],
}

/// Throws an Objective-C exception.
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
    objc_exception_throw(exception as *mut _)
}

unsafe fn try_no_ret<F>(closure: F) -> Result<(), *mut Exception>
where
    F: FnOnce(),
{
    extern "C" fn try_objc_execute_closure<F>(closure: &mut Option<F>)
    where
        F: FnOnce(),
    {
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
    let success = RustObjCExceptionTryCatch(f, context, &mut exception);

    if success == 0 {
        Ok(())
    } else {
        Err(exception as *mut _)
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
/// This encourages unwinding through the closure from
/// Objective-C, which is not safe.
pub unsafe fn r#try<F, R>(closure: F) -> Result<R, *mut Exception>
where
    F: FnOnce() -> R,
{
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
