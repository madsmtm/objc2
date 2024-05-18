#![no_main]
use libfuzzer_sys::fuzz_target;
use objc2::runtime::AnyClass;
use std::ffi::CString;

fuzz_target!(|s: &str| {
    #[cfg(not(target_vendor = "apple"))] // GNUstep
    let _cls = <objc2::runtime::NSObject as objc2::ClassType>::class();

    if CString::new(s).is_ok() {
        #[allow(clippy::eq_op)]
        if let Some(cls) = AnyClass::get(s) {
            assert_eq!(s, cls.name());
            assert_eq!(cls, cls);
        }
    }
});
