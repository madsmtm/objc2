#![cfg(all(feature = "libc", feature = "network"))]
use std::ffi::CStr;

use objc2_core_foundation::{
    CFBoolean, CFDictionary, CFRetained, CFString, CFStringBuiltInEncodings, CFType,
};
use objc2_io_kit::{
    kIOEthernetInterfaceClass, kIOPrimaryInterface, kIOPropertyMatchKey, kIOReturnSuccess,
    IOObjectRelease, IOServiceGetMatchingService, IOServiceMatching,
};

#[test]
fn matching_ethernet_interface() {
    let matching_dict = unsafe { IOServiceMatching(kIOEthernetInterfaceClass.as_ptr()) }.unwrap();

    let property_dict = CFDictionary::<CFString, CFType>::from_slices(
        &[unsafe { &cstr(kIOPrimaryInterface) }],
        &[CFBoolean::new(true)],
    );
    matching_dict.set(unsafe { &cstr(kIOPropertyMatchKey) }, &property_dict);

    #[allow(deprecated)]
    let service =
        unsafe { IOServiceGetMatchingService(main_port(), Some((&matching_dict).into())) };

    assert_eq!(IOObjectRelease(service), kIOReturnSuccess);
}

/// A smaller helper function to construct a `CFString` from a UTF-8 `CStr`.
// TODO: Make this use-case easier in `objc2-core-foundation`?
unsafe fn cstr(s: &CStr) -> CFRetained<CFString> {
    unsafe { CFString::with_c_string(None, s, CFStringBuiltInEncodings::EncodingUTF8.0).unwrap() }
}

fn main_port() -> libc::mach_port_t {
    #[cfg(target_os = "macos")]
    #[allow(deprecated)]
    unsafe {
        objc2_io_kit::kIOMasterPortDefault
    }

    #[cfg(not(target_os = "macos"))]
    unsafe {
        objc2_io_kit::kIOMainPortDefault
    }
}
