#![no_main]
use libfuzzer_sys::fuzz_target;
use objc2::runtime::Class;
use std::ffi::CString;

fuzz_target!(|s: &str| {
    #[cfg(feature = "gnustep-1-7")]
    unsafe {
        objc2::__gnustep_hack::get_class_to_force_linkage()
    };

    if CString::new(s).is_ok() {
        #[allow(clippy::eq_op)]
        if let Some(cls) = Class::get(s) {
            assert_eq!(s, cls.name());
            assert_eq!(cls, cls);
        }
    }
});
