#![cfg(feature = "apple")]
//! Test the output of the `ns_string!` macro.
use objc2_foundation::{ns_string, NSString};

#[no_mangle]
static EMPTY: &NSString = ns_string!("");

#[no_mangle]
static XYZ: &NSString = ns_string!("xyz");

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
