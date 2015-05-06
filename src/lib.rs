extern crate libc;

use libc::{c_int, c_void};

extern {
    fn RustObjCExceptionThrow(exception: *mut c_void);
    fn RustObjCExceptionTryCatch(try: extern fn(*mut c_void),
            context: *mut c_void, error: *mut *mut c_void) -> c_int;
}
