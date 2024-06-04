#![cfg_attr(not(feature = "afl"), no_main)]
use objc2::runtime::AnyClass;
use std::ffi::CString;

fn run(s: &str) {
    #[cfg(not(target_vendor = "apple"))] // GNUstep
    let _cls = <objc2::runtime::NSObject as objc2::ClassType>::class();

    if CString::new(s).is_ok() {
        #[allow(clippy::eq_op)]
        if let Some(cls) = AnyClass::get(s) {
            assert_eq!(s, cls.name());
            assert_eq!(cls, cls);
        }
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
