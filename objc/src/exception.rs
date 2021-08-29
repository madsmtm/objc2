use objc_exception;

use crate::rc::StrongPtr;
use crate::runtime::Object;

// Comment copied from `objc_exception`

/// Tries to execute the given closure and catches an Objective-C exception
/// if one is thrown.
///
/// Returns a `Result` that is either `Ok` if the closure succeeded without an
/// exception being thrown, or an `Err` with a pointer to an exception if one
/// was thrown. The exception is retained and so must be released.
///
/// # Safety
///
/// This encourages unwinding through the closure from Objective-C, which is
/// not safe.
pub unsafe fn catch_exception<F, R>(closure: F) -> Result<R, StrongPtr>
where
    F: FnOnce() -> R,
{
    objc_exception::r#try(closure).map_err(|exception| StrongPtr::new(exception as *mut Object))
}
