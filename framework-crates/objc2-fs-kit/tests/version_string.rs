use std::ffi::{c_char, c_uchar, CStr};

use objc2_fs_kit::FSKitVersionString;

#[test]
fn can_read_version() {
    let version: *const [c_uchar; 0] = &raw const FSKitVersionString;
    let version: *const c_char = version.cast();
    let s = unsafe { CStr::from_ptr(version) };
    assert!(s.count_bytes() > 1);
}
