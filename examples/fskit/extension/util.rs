use objc2::rc::Retained;
use objc2_foundation::{NSError, NSPOSIXErrorDomain};
use std::ffi::c_int;

pub(crate) fn posix_err(code: c_int) -> *mut NSError {
    Retained::autorelease_ptr(NSError::new(code as _, unsafe { NSPOSIXErrorDomain }))
}
