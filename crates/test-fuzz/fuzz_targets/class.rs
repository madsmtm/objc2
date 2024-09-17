#![cfg_attr(not(feature = "afl"), no_main)]
use objc2::runtime::AnyClass;
use std::ffi::{CStr, CString};

fn run(s: &CStr) {
    #[cfg(not(target_vendor = "apple"))] // GNUstep
    let _cls = <objc2::runtime::NSObject as objc2::ClassType>::class();

    #[allow(clippy::eq_op)]
    if let Some(cls) = AnyClass::get(s) {
        assert_eq!(s, cls.name());
        assert_eq!(cls, cls);
    }
}

#[cfg(not(feature = "afl"))]
libfuzzer_sys::fuzz_target!(|s: CString| run(&s));

#[cfg(feature = "afl")]
fn main() {
    afl::fuzz!(|s: CString| {
        run(&s);
    });
}
