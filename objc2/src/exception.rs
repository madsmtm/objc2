//! Objective-C's @throw and @try/@catch.
//!
//! By default, if the [`msg_send!`] macro causes an exception to be thrown,
//! this will unwind into Rust, resulting in undefined behavior. However, this
//! crate has an `"catch-all"` feature which, when enabled, wraps each
//! [`msg_send!`] in a `@catch` and panics if an exception is caught,
//! preventing Objective-C from unwinding into Rust.
//!
//! The `@try`/`@catch` functionality in this module is only available when
//! the `"exception"` feature is enabled.
//!
//! See the following links for more information:
//! - [Exception Programming Topics for Cocoa](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Exceptions/Exceptions.html)
//! - [The Objective-C Programming Language - Exception Handling](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjectiveC/Chapters/ocExceptionHandling.html)
//! - [Exception Handling in LLVM](https://llvm.org/docs/ExceptionHandling.html)

// TODO: Test this with panic=abort, and ensure that the code-size is
// reasonable in that case.

use alloc::string::String;
use alloc::string::ToString;
#[cfg(feature = "exception")]
use core::ffi::c_void;
use core::fmt;
#[cfg(feature = "exception")]
use core::mem;
use core::ops::Deref;
use core::panic::RefUnwindSafe;
use core::panic::UnwindSafe;
#[cfg(feature = "exception")]
use core::ptr;
use core::slice;
use objc2_encode::Encoding;
use objc2_encode::RefEncode;
use std::error::Error;
use std::os::raw::c_char;

#[cfg(feature = "exception")]
use crate::ffi;
use crate::rc::autoreleasepool;
use crate::rc::{Id, Shared};
use crate::runtime::Class;
use crate::runtime::Object;
use crate::Message;
use crate::{msg_send, msg_send_bool, msg_send_id, sel};

/// Unfortunate reimplementation of `objc2_foundation::NSString`.
///
/// I guess this is the price of wanting to do things "right"...
unsafe fn to_string_hack(obj: Id<Object, Shared>) -> String {
    #[cfg(feature = "apple")]
    const UTF8_ENCODING: usize = 4;
    #[cfg(feature = "gnustep-1-7")]
    const UTF8_ENCODING: i32 = 4;

    autoreleasepool(|_| {
        let len: usize = unsafe { msg_send![&obj, lengthOfBytesUsingEncoding: UTF8_ENCODING] };

        let bytes: *const c_char = unsafe { msg_send![&obj, UTF8String] };
        let bytes: *const u8 = bytes.cast();
        let bytes: &[u8] = unsafe { slice::from_raw_parts(bytes, len) };

        // Use lossy to avoid panic in error situations
        String::from_utf8_lossy(bytes).to_string()
    })
}

/// An Objective-C exception.
///
/// While highly recommended that any exceptions you intend to throw are
/// subclasses of `NSException`, this is not required by the runtime (similar
/// to how Rust can panic with arbitary payloads using [`panic_any`]).
///
/// [`panic_any`]: std::panic::panic_any
#[repr(transparent)]
pub struct Exception(Object);

unsafe impl RefEncode for Exception {
    const ENCODING_REF: Encoding<'static> = Encoding::Object;
}

unsafe impl Message for Exception {}

impl Deref for Exception {
    type Target = Object;

    #[inline]
    fn deref(&self) -> &Object {
        &self.0
    }
}

impl AsRef<Object> for Exception {
    #[inline]
    fn as_ref(&self) -> &Object {
        self
    }
}

// Note: We can't implement `Send` nor `Sync` since the exception could be
// anything!

impl Exception {
    /// Checks whether this is an instance of `NSException`.
    ///
    /// This should be considered a hint; it may return `false` in very, very
    /// few cases where it is actually `true`, but if it returns `true`, then
    /// it is definitely an instance of `NSException`.
    fn is_nsexception(&self) -> bool {
        // If `NSException` class is present
        if let Some(cls) = Class::get("NSException") {
            if self.0.class().responds_to(sel!(isKindOfClass:)) {
                unsafe { msg_send_bool![self, isKindOfClass: cls] }
            } else {
                false
            }
        } else {
            false
        }
    }

    // SAFETY: Must ensure that self is NSException
    unsafe fn name(&self) -> Option<String> {
        let obj: Option<Id<Object, Shared>> = unsafe { msg_send_id![self, name] };
        obj.map(|obj| unsafe { to_string_hack(obj) })
    }

    // SAFETY: Must ensure that self is NSException
    unsafe fn reason(&self) -> Option<String> {
        let obj: Option<Id<Object, Shared>> = unsafe { msg_send_id![self, reason] };
        obj.map(|obj| unsafe { to_string_hack(obj) })
    }
}

// This is not in any way efficient, but that's not really the point!
//
// We mostly just want to present a somewhat usable error message when the
// `catch-all` feature is enabled!
impl fmt::Debug for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "exception {:?}", self.0)?;

        // Attempt to provide better error message
        if self.is_nsexception() {
            // SAFETY: We know that these are safe to call since this is an
            // instance of `NSException`.
            let name = unsafe { self.name() };
            let reason = unsafe { self.reason() };

            if let Some(name) = name {
                write!(f, " '{}'", name)?;
            } else {
                write!(f, " (NULL)")?;
            }

            if let Some(reason) = reason {
                write!(f, " reason:{}", reason)?;
            } else {
                write!(f, " reason:(NULL)")?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_nsexception() {
            // SAFETY: Just checked that this is NSException.
            if let Some(reason) = unsafe { self.reason() } {
                return write!(f, "{}", reason);
            }
        }
        write!(f, "unknown exception")
    }
}

impl Error for Exception {}

impl UnwindSafe for Exception {}
impl RefUnwindSafe for Exception {}

/// Throws an Objective-C exception.
///
/// This is the Objective-C equivalent of Rust's [`panic!`].
///
///
/// # Safety
///
/// This unwinds from Objective-C, and the exception must be caught using an
/// Objective-C exception handler like [`catch`] (and specifically not
/// [`catch_unwind`]).
///
/// This also invokes undefined behaviour until `C-unwind` is stabilized, see
/// [RFC-2945] - you can try this out on nightly using the `unstable-c-unwind`
/// feature flag.
///
/// [`catch_unwind`]: std::panic::catch_unwind
/// [RFC-2945]: https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html
#[inline]
#[cfg(feature = "exception")] // For consistency, not strictly required
pub unsafe fn throw(exception: Id<Exception, Shared>) -> ! {
    let ptr = exception.0.as_ptr() as *mut ffi::objc_object;
    // SAFETY: Object is valid and non-null (nil exceptions are not valid in
    // the old runtime).
    unsafe { ffi::objc_exception_throw(ptr) }
}

#[cfg(feature = "exception")]
unsafe fn try_no_ret<F: FnOnce()>(closure: F) -> Result<(), Option<Id<Exception, Shared>>> {
    #[cfg(not(feature = "unstable-c-unwind"))]
    let f = {
        extern "C" fn try_objc_execute_closure<F>(closure: &mut Option<F>)
        where
            F: FnOnce(),
        {
            // This is always passed Some, so it's safe to unwrap
            let closure = closure.take().unwrap();
            closure();
        }

        let f: extern "C" fn(&mut Option<F>) = try_objc_execute_closure;
        let f: extern "C" fn(*mut c_void) = unsafe { mem::transmute(f) };
        f
    };

    #[cfg(feature = "unstable-c-unwind")]
    let f = {
        extern "C-unwind" fn try_objc_execute_closure<F>(closure: &mut Option<F>)
        where
            F: FnOnce(),
        {
            // This is always passed Some, so it's safe to unwrap
            let closure = closure.take().unwrap();
            closure();
        }

        let f: extern "C-unwind" fn(&mut Option<F>) = try_objc_execute_closure;
        let f: extern "C-unwind" fn(*mut c_void) = unsafe { mem::transmute(f) };
        f
    };

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
/// This is the Objective-C equivalent of Rust's [`catch_unwind`].
/// Accordingly, if your Rust code is compiled with `panic=abort` this cannot
/// catch the exception.
///
/// Returns a `Result` that is either `Ok` if the closure succeeded without an
/// exception being thrown, or an `Err` with the exception. The exception is
/// automatically released.
///
/// The exception is `None` in the extremely exceptional case that the
/// exception object is `nil`. This should basically never happen, but is
/// technically possible on some systems with `@throw nil`.
///
/// [`catch_unwind`]: std::panic::catch_unwind
///
///
/// # Safety
///
/// The given closure must not panic (e.g. normal Rust unwinding into this
/// causes undefined behaviour).
///
/// Additionally, this unwinds through the closure from Objective-C, which is
/// undefined behaviour until `C-unwind` is stabilized, see [RFC-2945] - you
/// can try this out on nightly using the `unstable-c-unwind` feature flag.
///
/// [RFC-2945]: https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html
#[cfg(feature = "exception")]
pub unsafe fn catch<R>(
    closure: impl FnOnce() -> R + UnwindSafe,
) -> Result<R, Option<Id<Exception, Shared>>> {
    let mut value = None;
    let value_ref = &mut value;
    let closure = move || {
        *value_ref = Some(closure());
    };
    let result = unsafe { try_no_ret(closure) };
    // If the try succeeded, this was set so it's safe to unwrap
    result.map(|()| value.unwrap())
}

#[cfg(test)]
#[cfg(feature = "exception")]
mod tests {
    use alloc::format;
    use alloc::string::ToString;

    use super::*;
    use crate::{class, msg_send_id};

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
    #[cfg_attr(
        all(feature = "apple", target_os = "macos", target_arch = "x86"),
        ignore = "`NULL` exceptions are invalid on 32-bit / w. fragile runtime"
    )]
    fn test_catch_null() {
        let s = "Hello".to_string();
        let result = unsafe {
            catch(move || {
                if !s.is_empty() {
                    ffi::objc_exception_throw(ptr::null_mut())
                }
                s.len()
            })
        };
        assert!(result.unwrap_err().is_none());
    }

    #[test]
    fn test_throw_catch_object() {
        let obj: Id<Exception, Shared> = unsafe { msg_send_id![class!(NSObject), new].unwrap() };
        // TODO: Investigate why this is required on GNUStep!
        let _obj2 = obj.clone();
        let ptr: *const Exception = &*obj;

        let result = unsafe { catch(|| throw(obj)) };
        let obj = result.unwrap_err().unwrap();

        assert_eq!(
            format!("{:?}", obj),
            format!("exception <NSObject: {:p}>", ptr)
        );

        assert!(ptr::eq(&*obj, ptr));
    }
}
