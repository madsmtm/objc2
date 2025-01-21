use core::ffi::{c_char, c_uint, c_void};
use core::num::NonZeroU32;
use core::ptr;
use core::sync::atomic::{AtomicU32, Ordering};
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

use super::OSVersion;
use crate::rc::{autoreleasepool, Allocated, Retained};
use crate::runtime::__nsstring::{nsstring_to_str, UTF8_ENCODING};
use crate::runtime::{NSObject, NSObjectProtocol};
use crate::{class, msg_send};

/// The deployment target for the current OS.
pub(crate) const DEPLOYMENT_TARGET: OSVersion = {
    // Intentionally use `#[cfg]` guards instead of `cfg!` here, to avoid
    // recompiling when unrelated environment variables change.
    #[cfg(target_os = "macos")]
    let var = option_env!("MACOSX_DEPLOYMENT_TARGET");
    #[cfg(target_os = "ios")] // Also used on Mac Catalyst.
    let var = option_env!("IPHONEOS_DEPLOYMENT_TARGET");
    #[cfg(target_os = "tvos")]
    let var = option_env!("TVOS_DEPLOYMENT_TARGET");
    #[cfg(target_os = "watchos")]
    let var = option_env!("WATCHOS_DEPLOYMENT_TARGET");
    #[cfg(target_os = "visionos")]
    let var = option_env!("XROS_DEPLOYMENT_TARGET");

    if let Some(var) = var {
        OSVersion::from_str(var)
    } else {
        // Default operating system version.
        // See <https://github.com/rust-lang/rust/blob/1e5719bdc40bb553089ce83525f07dfe0b2e71e9/compiler/rustc_target/src/spec/base/apple/mod.rs#L207-L215>
        //
        // Note that we cannot do as they suggest, and use
        // `rustc --print=deployment-target`, as this has to work at `const`
        // time.
        #[allow(clippy::if_same_then_else)]
        let os_min = if cfg!(target_os = "macos") {
            (10, 12, 0)
        } else if cfg!(target_os = "ios") {
            (10, 0, 0)
        } else if cfg!(target_os = "tvos") {
            (10, 0, 0)
        } else if cfg!(target_os = "watchos") {
            (5, 0, 0)
        } else if cfg!(target_os = "visionos") {
            (1, 0, 0)
        } else {
            panic!("unknown Apple OS")
        };

        // On certain targets it makes sense to raise the minimum OS version.
        //
        // See <https://github.com/rust-lang/rust/blob/1e5719bdc40bb553089ce83525f07dfe0b2e71e9/compiler/rustc_target/src/spec/base/apple/mod.rs#L217-L231>
        //
        // Note that we cannot do all the same checks as `rustc` does, because
        // we have no way of knowing if the architecture is `arm64e` without
        // reading the target triple itself (and we want to get rid of build
        // scripts).
        #[allow(clippy::if_same_then_else)]
        let min = if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
            (11, 0, 0)
        } else if cfg!(all(
            target_os = "ios",
            target_arch = "aarch64",
            target_abi_macabi
        )) {
            (14, 0, 0)
        } else if cfg!(all(
            target_os = "ios",
            target_arch = "aarch64",
            target_simulator
        )) {
            (14, 0, 0)
        } else if cfg!(all(target_os = "tvos", target_arch = "aarch64")) {
            (14, 0, 0)
        } else if cfg!(all(target_os = "watchos", target_arch = "aarch64")) {
            (7, 0, 0)
        } else {
            os_min
        };

        OSVersion {
            major: min.0,
            minor: min.1,
            patch: min.2,
        }
    }
};

/// Look up the current version at runtime.
///
/// Note that this doesn't work with "zippered" `dylib`s yet, though
/// that's probably fine, `rustc` doesn't support those either:
/// <https://github.com/rust-lang/rust/issues/131216>
#[inline]
pub(crate) fn current_version() -> OSVersion {
    // Cache the lookup for performance.
    //
    // We assume that 0.0.0 is never gonna be a valid version,
    // and use that as our sentinel value.
    static CURRENT_VERSION: AtomicU32 = AtomicU32::new(0);

    // We use relaxed atomics, it doesn't matter if two threads end up racing
    // to read or write the version.
    let version = CURRENT_VERSION.load(Ordering::Relaxed);
    OSVersion::from_u32(if version == 0 {
        // TODO: Consider using `std::panic::abort_unwind` here for code-size?
        let version = lookup_version().get();
        CURRENT_VERSION.store(version, Ordering::Relaxed);
        version
    } else {
        version
    })
}

#[cold]
fn lookup_version() -> NonZeroU32 {
    // Since macOS 10.15, libSystem has provided the undocumented
    // `_availability_version_check` via `libxpc` for doing this version
    // lookup, though it's usage may be a bit dangerous, see:
    // - https://reviews.llvm.org/D150397
    // - https://github.com/llvm/llvm-project/issues/64227
    //
    // So instead, we use the safer approach of reading from `sysctl`, and
    // if that fails, we fall back to the property list (this is what
    // `_availability_version_check` does internally).
    let version = version_from_sysctl().unwrap_or_else(version_from_plist);
    // Use `NonZeroU32` to try to make it clearer to the optimizer that this
    // will never return 0.
    NonZeroU32::new(version.to_u32()).expect("version cannot be 0.0.0")
}

/// Read the version from `kern.osproductversion` or `kern.iossupportversion`.
fn version_from_sysctl() -> Option<OSVersion> {
    // This won't work in the simulator, `kern.osproductversion` will return
    // the host macOS version.
    if cfg!(target_simulator) {
        return None;
    }

    // SAFETY: Same signature as in `libc`
    extern "C" {
        fn sysctlbyname(
            name: *const c_char,
            oldp: *mut c_void,
            oldlenp: *mut usize,
            newp: *mut c_void,
            newlen: usize,
        ) -> c_uint;
    }

    let name = if cfg!(target_abi_macabi) {
        b"kern.iossupportversion\0".as_ptr().cast()
    } else {
        // Introduced in macOS 10.13.4.
        b"kern.osproductversion\0".as_ptr().cast()
    };

    let mut buf: [u8; 32] = [0; 32];
    let mut size = buf.len();
    let ret = unsafe { sysctlbyname(name, buf.as_mut_ptr().cast(), &mut size, ptr::null_mut(), 0) };
    if ret != 0 {
        // `sysctlbyname` is not available.
        return None;
    }

    Some(OSVersion::from_bytes(&buf[..(size - 1)]))
}

/// Look up the current OS version from the `ProductVersion` or
/// `iOSSupportVersion` in `/System/Library/CoreServices/SystemVersion.plist`.
/// This file was introduced in macOS 10.3.0.
///
/// This is also what is done in `compiler-rt`:
/// <https://github.com/llvm/llvm-project/blob/llvmorg-19.1.1/compiler-rt/lib/builtins/os_version_check.c>
///
/// NOTE: I don't _think_ we need to do a similar thing as what Zig does to
/// handle the fake 10.16 versions returned when the SDK version of the binary
/// is less than 11.0:
/// <https://github.com/ziglang/zig/blob/0.13.0/lib/std/zig/system/darwin/macos.zig>
///
/// My reasoning is that we _want_ to follow Apple's behaviour here, and
/// return 10.16 when compiled with an older SDK; the user should upgrade
/// their tooling.
///
/// NOTE: `rustc` currently doesn't set the right SDK version when linking
/// with ld64, so this will usually have the wrong behaviour on x86_64. But
/// that's a `rustc` bug, and is tracked in:
/// <https://github.com/rust-lang/rust/issues/129432>
///
///
/// # Panics
///
/// Panics if reading or parsing the PList fails (or if the system was out of
/// memory).
///
/// We deliberately choose to panic, as having this lookup silently return
/// an empty OS version would be impossible for a user to debug.
fn version_from_plist() -> OSVersion {
    // Use Foundation's mechanisms for reading the PList.
    autoreleasepool(|pool| {
        let path: Retained<NSObject> = if cfg!(target_simulator) {
            let root = std::env::var_os("IPHONE_SIMULATOR_ROOT")
                    .expect("environment variable `IPHONE_SIMULATOR_ROOT` must be set when executing under simulator");
            let path = PathBuf::from(root).join("System/Library/CoreServices/SystemVersion.plist");
            let path = path.as_os_str().as_bytes();

            // SAFETY: Allocating a string is valid on all threads.
            let alloc: Allocated<NSObject> = unsafe { Allocated::alloc(class!(NSString)) };
            // SAFETY: The bytes are valid, and the length is correct.
            unsafe {
                let bytes_ptr: *const c_void = path.as_ptr().cast();
                msg_send![
                    alloc,
                    initWithBytes: bytes_ptr,
                    length: path.len(),
                    // OsStr is a superset of UTF-8 on unix platforms
                    encoding: UTF8_ENCODING,
                ]
            }
        } else {
            let path: *const c_char = b"/System/Library/CoreServices/SystemVersion.plist\0"
                .as_ptr()
                .cast();
            // SAFETY: The path is NULL terminated.
            unsafe { msg_send![class!(NSString), stringWithUTF8String: path] }
        };

        // SAFETY: dictionaryWithContentsOfFile: is safe to call.
        let data: Option<Retained<NSObject>> =
            unsafe { msg_send![class!(NSDictionary), dictionaryWithContentsOfFile: &*path] };

        let data = data.expect(
            "`/System/Library/CoreServices/SystemVersion.plist` must be readable, and contain a valid PList",
        );

        // Read `ProductVersion`, except when running on Mac Catalyst, then we
        // read `iOSSupportVersion` instead.
        let lookup_key: *const c_char = if cfg!(target_abi_macabi) {
            b"iOSSupportVersion\0".as_ptr().cast()
        } else {
            b"ProductVersion\0".as_ptr().cast()
        };
        // SAFETY: The lookup key is NULL terminated.
        let lookup_key: Retained<NSObject> =
            unsafe { msg_send![class!(NSString), stringWithUTF8String: lookup_key] };

        let version: Retained<NSObject> = unsafe { msg_send![&data, objectForKey: &*lookup_key] };

        assert!(
            version.isKindOfClass(class!(NSString)),
            "`ProductVersion` key in `/System/Library/CoreServices/SystemVersion.plist` must be a string"
        );

        // SAFETY: The given object is an NSString, and the returned string
        // slice is not used outside of the current pool.
        let version = unsafe { nsstring_to_str(&version, pool) };

        OSVersion::from_str(version)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::string::String;
    use std::process::Command;

    #[test]
    fn sysctl_same_as_in_plist() {
        if let Some(version) = version_from_sysctl() {
            assert_eq!(version, version_from_plist());
        }
    }

    #[test]
    fn read_version() {
        assert!(OSVersion::MIN < current_version(), "version cannot be min");
        assert!(current_version() < OSVersion::MAX, "version cannot be max");
    }

    #[test]
    #[cfg_attr(
        not(target_os = "macos"),
        ignore = "`sw_vers` is only available on macOS"
    )]
    fn compare_against_sw_vers() {
        let expected = Command::new("sw_vers")
            .arg("-productVersion")
            .output()
            .unwrap()
            .stdout;
        let expected = String::from_utf8(expected).unwrap();
        let expected = OSVersion::from_str(expected.trim());

        let actual = current_version();
        assert_eq!(expected, actual);
    }
}
