use crate::rc::StrongPtr;
use crate::runtime::Object;

// Comment copied from `objc2_exception`

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
pub unsafe fn catch_exception<R>(closure: impl FnOnce() -> R) -> Result<R, StrongPtr> {
    objc2_exception::r#try(closure).map_err(|exception| StrongPtr::new(exception as *mut Object))
}
