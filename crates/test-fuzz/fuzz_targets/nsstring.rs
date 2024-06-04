#![cfg_attr(not(feature = "afl"), no_main)]
use objc2::rc::autoreleasepool;
use objc2_foundation::NSString;

fn run(s: &str) {
    autoreleasepool(|pool| {
        let obj = NSString::from_str(s);
        assert_eq!(obj.as_str(pool), s);
    });
}

#[cfg(not(feature = "afl"))]
libfuzzer_sys::fuzz_target!(|s: &str| run(s));

#[cfg(feature = "afl")]
fn main() {
    afl::fuzz!(|s: &str| {
        run(s);
    });
}
