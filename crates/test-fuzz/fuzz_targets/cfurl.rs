//! Test parsing of URL strings.
#![cfg_attr(not(feature = "afl"), no_main)]
use objc2_core_foundation::{CFString, CFURL};

fn run(str: &str) {
    if let Some(url) = CFURL::from_string(&CFString::from_str(str)) {
        let url_str = url.to_string().to_string();
        // Sometimes the URL returned from `.to_string` may be overly
        // percentage-escaped. But in all other cases, they should be
        // equal.
        if !url_str.contains("%") {
            assert_eq!(url_str, str);
        }
    }

    // Test that the same holds for unchecked URL creation.
    if let Some(url) = unsafe { CFURL::from_str_unchecked(str) } {
        let url_str = url.to_string().to_string();
        if !url_str.contains("%") {
            assert_eq!(url_str, str);
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
