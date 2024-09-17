#![cfg_attr(not(feature = "afl"), no_main)]
use objc2::runtime::Sel;
use std::ffi::{CStr, CString};

#[allow(clippy::eq_op)]
fn run(s: &CStr) {
    #[cfg(not(target_vendor = "apple"))] // GNUstep
    let _cls = <objc2::runtime::NSObject as objc2::ClassType>::class();

    let sel = Sel::register(s);
    assert_eq!(s, sel.name());
    assert_eq!(sel, sel);
}

#[cfg(not(feature = "afl"))]
libfuzzer_sys::fuzz_target!(|s: CString| run(&s));

#[cfg(feature = "afl")]
fn main() {
    afl::fuzz!(|s: CString| {
        run(&s);
    });
}
