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
use core::intrinsics;
use core::mem::ManuallyDrop;
use core::ptr;
use core::ptr::NonNull;

use crate::rc::{Id, Shared};
use crate::runtime::Object;

use objc_sys::{objc_begin_catch, objc_end_catch, objc_exception_throw, objc_object};

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
    unsafe { objc_exception_throw(exception) }
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
pub unsafe fn catch<R, F: FnOnce() -> R>(f: F) -> Result<R, Option<Id<Object, Shared>>> {
    // Our implementation is just a copy of `std::panicking::r#try`:
    // https://github.com/rust-lang/rust/blob/1.52.1/library/std/src/panicking.rs#L299-L408
    union Data<F, R> {
        f: ManuallyDrop<F>,
        r: ManuallyDrop<R>,
        p: ManuallyDrop<Option<Id<Object, Shared>>>,
    }

    let mut data = Data {
        f: ManuallyDrop::new(f),
    };

    let data_ptr = &mut data as *mut _ as *mut u8;

    return if unsafe { intrinsics::r#try(do_call::<F, R>, data_ptr, do_catch::<F, R>) } == 0 {
        Ok(ManuallyDrop::into_inner(unsafe { data.r }))
    } else {
        Err(ManuallyDrop::into_inner(unsafe { data.p }))
    };

    /// Only function that we've changed
    #[cold]
    unsafe fn cleanup(payload: *mut u8) -> Option<Id<Object, Shared>> {
        // We let Objective-C process the unwind payload, and hand us the
        // exception object. Everything between this and `objc_end_catch` is
        // treated as a `@catch` block.
        let obj = unsafe { objc_begin_catch(payload as *mut c_void) };
        // We retain the exception since it might have been autoreleased.
        // This cannot unwind, so we don't need extra guards here.
        let obj = NonNull::new(obj.cast::<Object>()).map(|ptr| unsafe { Id::retain(ptr) });
        // End the `@catch` block.
        unsafe { objc_end_catch() };
        obj
    }

    #[inline]
    fn do_call<F: FnOnce() -> R, R>(data: *mut u8) {
        unsafe {
            let data = data as *mut Data<F, R>;
            let data = &mut (*data);
            let f = ManuallyDrop::take(&mut data.f);
            data.r = ManuallyDrop::new(f());
        }
    }

    #[inline]
    fn do_catch<F: FnOnce() -> R, R>(data: *mut u8, payload: *mut u8) {
        unsafe {
            let data = data as *mut Data<F, R>;
            let data = &mut (*data);
            let obj = cleanup(payload);
            data.p = ManuallyDrop::new(obj);
        }
    }
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
