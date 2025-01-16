//! Test the output of the `ns_string!` macro.
use objc2_foundation::{ns_string, NSString};

// Temporary to allow testing putting string references in statics.
// This doesn't yet compile on other platforms, but could in the future!
#[cfg(all(target_vendor = "apple", feature = "assembly-features"))]
#[repr(transparent)]
struct StaticString(&'static NSString);

#[cfg(all(target_vendor = "apple", feature = "assembly-features"))]
unsafe impl Sync for StaticString {}

#[cfg(all(target_vendor = "apple", feature = "assembly-features"))]
#[no_mangle]
static EMPTY: StaticString = {
    const INPUT: &[u8] = b"";
    objc2_foundation::__ns_string_static!(INPUT);
    StaticString(CFSTRING.as_nsstring_const())
};

#[cfg(all(target_vendor = "apple", feature = "assembly-features"))]
#[no_mangle]
static XYZ: StaticString = {
    const INPUT: &[u8] = b"xyz";
    objc2_foundation::__ns_string_static!(INPUT);
    StaticString(CFSTRING.as_nsstring_const())
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
