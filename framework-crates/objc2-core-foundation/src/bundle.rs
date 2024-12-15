#[cfg(target_pointer_width = "64")] // #if TARGET_RT_64_BIT
type Inner = core::ffi::c_int;
#[cfg(not(target_pointer_width = "64"))]
type Inner = i16; // SInt16

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfbundlerefnum?language=objc)
pub type CFBundleRefNum = Inner;
