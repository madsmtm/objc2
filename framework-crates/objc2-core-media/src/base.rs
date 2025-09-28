// See CMBase.h
#![allow(unexpected_cfgs)]

// (TARGET_OS_OSX || TARGET_OS_MACCATALYST || TARGET_OS_WINDOWS) && TARGET_CPU_X86_64
#[cfg(all(
    any(target_os = "macos", target_env = "macabi", target_os = "windows"),
    target_arch = "x86_64"
))]
type Inner = u32; // uint32_t
#[cfg(not(all(
    any(target_os = "macos", target_env = "macabi", target_os = "windows"),
    target_arch = "x86_64"
)))]
type Inner = usize; // uintptr_t

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/cmbaseclassversion?language=objc)
pub type CMBaseClassVersion = Inner;

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/cmstructversion?language=objc)
pub type CMStructVersion = Inner;
