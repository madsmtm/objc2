#![no_main]
use libfuzzer_sys::fuzz_target;
use objc2::runtime::Class;
use std::ffi::CString;
use std::str;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = str::from_utf8(data) {
        if CString::new(s).is_ok() {
            if let Some(cls) = Class::get(&s) {
                assert_eq!(&*s, cls.name());
                assert_eq!(&cls, &cls);
            }
        }
    }
});
