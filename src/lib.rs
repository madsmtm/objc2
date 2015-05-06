extern crate libc;

use std::mem;
use std::ptr;
use libc::{c_int, c_void};

#[link(name = "objc", kind = "dylib")]
extern { }

extern {
    fn RustObjCExceptionThrow(exception: *mut c_void);
    fn RustObjCExceptionTryCatch(try: extern fn(*mut c_void),
            context: *mut c_void, error: *mut *mut c_void) -> c_int;
}

pub unsafe fn throw(exception: *mut c_void) -> ! {
    RustObjCExceptionThrow(exception);
    unreachable!();
}

unsafe fn function_and_context<F>(closure: *mut F) ->
        (extern fn(*mut c_void), *mut c_void)
        where F: FnOnce() {
    unsafe extern fn try_objc_execute_closure<F>(closure: *mut F)
            where F: FnOnce() {
        (ptr::read(closure))();
    }

    (mem::transmute(try_objc_execute_closure::<F>), closure as *mut c_void)
}

pub unsafe fn try<F, R>(closure: F) -> Result<R, *mut c_void>
        where F: FnOnce() -> R {
    let mut result = mem::uninitialized();
    let result_ptr: *mut R = &mut result;
    let mut closure = move || {
        ptr::write(result_ptr, closure());
    };

    let mut exception = ptr::null_mut();
    let (f, context) = function_and_context(&mut closure);
    let success = RustObjCExceptionTryCatch(f, context, &mut exception);
    mem::forget(closure);

    if success == 0 {
        Ok(result)
    } else {
        mem::forget(result);
        Err(exception)
    }
}

#[cfg(test)]
mod tests {
    use std::ptr;
    use super::{throw, try};

    #[test]
    fn test_try() {
        unsafe {
            let s = "Hello".to_string();
            let result = try(move || {
                if s.len() > 0 {
                    throw(ptr::null_mut());
                }
                s.len()
            });
            assert!(result.unwrap_err() == ptr::null_mut());

            let mut s = "Hello".to_string();
            let result = try(move || {
                s.push_str(", World!");
                s
            });
            assert!(result.unwrap() == "Hello, World!");
        }
    }
}
