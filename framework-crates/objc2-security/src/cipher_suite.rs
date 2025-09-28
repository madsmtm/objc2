#![allow(unexpected_cfgs)]
// ((TARGET_OS_IPHONE && !TARGET_OS_MACCATALYST) || (TARGET_OS_OSX && TARGET_CPU_ARM64))
#[cfg(any(
    all(
        all(target_vendor = "apple", not(target_os = "macos")), // TARGET_OS_IPHONE
        not(target_env = "macabi"), // !TARGET_OS_MACCATALYST
    ),
    // (TARGET_OS_OSX && TARGET_CPU_ARM64)
    all(target_os = "macos", target_arch = "aarch64"),
))]
type Inner = u16; // uint16_t

#[cfg(not(any(
    all(
        all(target_vendor = "apple", not(target_os = "macos")),
        not(target_env = "macabi"),
    ),
    all(target_os = "macos", target_arch = "aarch64"),
)))]
type Inner = u32; // uint32_t

/// [Apple's documentation](https://developer.apple.com/documentation/security/sslciphersuite?language=objc)
pub type SSLCipherSuite = Inner;
