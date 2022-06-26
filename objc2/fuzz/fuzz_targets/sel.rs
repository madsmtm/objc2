#![no_main]
use libfuzzer_sys::fuzz_target;
use objc2::runtime::Sel;
use std::ffi::CString;
use std::str;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = str::from_utf8(data) {
        if CString::new(s).is_ok() {
            let sel = Sel::register(&s);
            assert_eq!(&*s, sel.name());
            assert_eq!(&sel, &sel);
        }
    }
});
