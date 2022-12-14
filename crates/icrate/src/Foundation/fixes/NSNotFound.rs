use objc2::ffi::{NSInteger, NSIntegerMax};

/// A value indicating that a requested item couldn’t be found or doesn’t exist.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nsnotfound?language=objc).
pub const NSNotFound: NSInteger = NSIntegerMax;
