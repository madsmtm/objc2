//! Test that the encoding string that we output is not full length.
use std::ffi::CStr;

use objc2_encode::Encode;

#[no_mangle]
static ENC: &CStr = i8::ENCODING_CSTR;
