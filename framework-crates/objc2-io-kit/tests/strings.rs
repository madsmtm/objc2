#![cfg(feature = "libc")]

use std::ffi::{CStr, CString};

use libc::MACH_PORT_NULL;
use objc2_io_kit::{
    kIOReturnSuccess, kIOServicePlane, IOObjectConformsTo, IOObjectCopyClass, IOObjectGetClass,
    IOObjectRelease, IORegistryEntryFromPath, IORegistryEntryGetPath, IORegistryGetRootEntry,
};

#[cfg(not(target_os = "macos"))]
macro_rules! main_port {
    () => {
        #[allow(unused_unsafe)]
        unsafe {
            objc2_io_kit::kIOMainPortDefault
        }
    };
}

#[cfg(target_os = "macos")]
macro_rules! main_port {
    () => {{
        #[allow(deprecated, unused_unsafe)]
        unsafe {
            objc2_io_kit::kIOMasterPortDefault
        }
    }};
}

#[test]
fn out_pointer() {
    let obj = IORegistryGetRootEntry(main_port!());

    let mut name = [0; 128];
    assert_eq!(
        unsafe { IOObjectGetClass(obj, Some(&mut name)) },
        kIOReturnSuccess
    );
    let name = name.map(|c| c as u8);
    let name = CStr::from_bytes_until_nul(&name).unwrap();

    let cf_name = IOObjectCopyClass(obj).unwrap();

    assert_eq!(name.to_str().unwrap(), cf_name.to_string());
    assert_eq!(name, c"IORegistryEntry");

    assert_eq!(IOObjectRelease(obj), kIOReturnSuccess);
}

#[test]
fn in_pointer() {
    let obj = IORegistryGetRootEntry(main_port!());

    assert!(unsafe { IOObjectConformsTo(obj, Some(c"IORegistryEntry")) });
    assert!(!unsafe { IOObjectConformsTo(obj, Some(c"BogusClassName")) });

    assert_eq!(IOObjectRelease(obj), kIOReturnSuccess);
}

#[test]
fn entry_path() {
    let path = c"IOService:/";

    let obj = unsafe { IORegistryEntryFromPath(main_port!(), Some(path)) };
    assert_ne!(obj, MACH_PORT_NULL as u32);

    let mut out_path = [0; 512];
    assert_eq!(
        unsafe { IORegistryEntryGetPath(obj, Some(kIOServicePlane), Some(&mut out_path)) },
        kIOReturnSuccess
    );
    let out_path = out_path.map(|c| c as u8);
    let out_path = CStr::from_bytes_until_nul(&out_path).unwrap();
    assert_eq!(out_path, path);

    assert_eq!(IOObjectRelease(obj), kIOReturnSuccess);
}

#[test]
fn entry_path_too_long() {
    let path = CString::new([b'x'; 1000]).unwrap();
    let obj = unsafe { IORegistryEntryFromPath(main_port!(), Some(&path)) };
    assert_eq!(obj, MACH_PORT_NULL as u32);
}

#[test]
fn in_pointer_too_long() {
    let obj = IORegistryGetRootEntry(main_port!());

    let name = CString::new([b'x'; 1000]).unwrap();
    assert!(!unsafe { IOObjectConformsTo(obj, Some(&name)) });

    assert_eq!(IOObjectRelease(obj), kIOReturnSuccess);
}
