// See CMBase.h

// TODO: Or target_abi = "macabi"
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
type Inner = u32; // uint32_t
#[cfg(not(all(target_os = "macos", target_arch = "x86_64")))]
type Inner = usize; // uintptr_t

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/cmbaseclassversion?language=objc)
pub type CMBaseClassVersion = Inner;

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/cmstructversion?language=objc)
pub type CMStructVersion = Inner;
