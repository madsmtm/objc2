#![cfg_attr(not(feature = "afl"), no_main)]
use objc2_core_foundation::CFString;

fn run(s: &str) {
    let obj = CFString::from_str(s);
    assert_eq!(obj.to_string(), s);
}

#[cfg(not(feature = "afl"))]
libfuzzer_sys::fuzz_target!(|s: &str| run(s));

#[cfg(feature = "afl")]
fn main() {
    afl::fuzz!(|s: &str| {
        run(s);
    });
}
