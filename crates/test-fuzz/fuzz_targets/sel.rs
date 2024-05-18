#![no_main]
use libfuzzer_sys::fuzz_target;
use objc2::runtime::Sel;
use std::ffi::CString;

fuzz_target!(|s: &str| {
    #[cfg(not(target_vendor = "apple"))] // GNUstep
    let _cls = <objc2::runtime::NSObject as objc2::ClassType>::class();

    #[allow(clippy::eq_op)]
    if CString::new(s).is_ok() {
        let sel = Sel::register(s);
        assert_eq!(s, sel.name());
        assert_eq!(sel, sel);
    }
});
