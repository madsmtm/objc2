#![cfg_attr(not(feature = "afl"), no_main)]
use objc2::runtime::Sel;
use std::ffi::CString;

fn run(s: &str) {
    #[cfg(not(target_vendor = "apple"))] // GNUstep
    let _cls = <objc2::runtime::NSObject as objc2::ClassType>::class();

    #[allow(clippy::eq_op)]
    if CString::new(s).is_ok() {
        let sel = Sel::register(s);
        assert_eq!(s, sel.name());
        assert_eq!(sel, sel);
    }
}

#[cfg(not(feature = "afl"))]
libfuzzer_sys::fuzz_target!(|s: &str| run(s));

#[cfg(feature = "afl")]
fn main() {
    afl::fuzz!(|s: &str| {
        run(s);
    });
}
