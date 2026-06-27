use core::ffi::CStr;

/// Convert a class name with a trailing NUL byte to a `CStr`, at `const`.
#[track_caller]
pub const fn class_c_name(name: &str) -> &CStr {
    let bytes = name.as_bytes();
    // Workaround for `from_bytes_with_nul` not being `const` in MSRV.
    let mut i = 0;
    while i < bytes.len() - 1 {
        if bytes[i] == 0 {
            panic!("name must not contain interior NUL bytes");
        }
        i += 1;
    }
    if let Ok(c_name) = CStr::from_bytes_until_nul(bytes) {
        c_name
    } else {
        unreachable!()
    }
}
