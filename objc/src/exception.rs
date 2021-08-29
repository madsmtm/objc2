use objc_exception;

use crate::rc::StrongPtr;
use crate::runtime::Object;

pub unsafe fn catch_exception<F, R>(closure: F) -> Result<R, StrongPtr>
        where F: FnOnce() -> R {
    objc_exception::r#try(closure).map_err(|exception| {
        StrongPtr::new(exception as *mut Object)
    })
}
