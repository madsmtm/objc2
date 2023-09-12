//! Test the output of the `ns_string!` macro.
#![cfg(feature = "Foundation_NSString")]

use icrate::Foundation::{ns_string, NSString};

// Temporary to allow testing putting string references in statics.
// This doesn't yet compile on other platforms, but could in the future!
#[cfg(all(feature = "apple", feature = "assembly-features"))]
#[no_mangle]
static EMPTY: &NSString = {
    const INPUT: &[u8] = b"";
    icrate::__ns_string_static!(INPUT);
    CFSTRING.as_nsstring_const()
};
#[cfg(all(feature = "apple", feature = "assembly-features"))]
#[no_mangle]
static XYZ: &NSString = {
    const INPUT: &[u8] = b"xyz";
    icrate::__ns_string_static!(INPUT);
    CFSTRING.as_nsstring_const()
};

#[no_mangle]
fn get_ascii() -> &'static NSString {
    ns_string!("abc")
}

#[no_mangle]
fn get_utf16() -> &'static NSString {
    ns_string!("ábć")
}

#[no_mangle]
fn get_with_nul() -> &'static NSString {
    ns_string!("a\0b\0c\0")
}
