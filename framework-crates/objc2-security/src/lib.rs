//! # Bindings to the `Security` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/security/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(feature = "unstable-darwin-objc", feature(darwin_objc))]
#![cfg_attr(docsrs, feature(doc_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-security/0.3.2")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod generated;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;

// Manual fixes.
#[cfg(all(feature = "libc", feature = "Authorization"))]
mod authorization;
#[cfg(feature = "CipherSuite")]
mod cipher_suite;
#[cfg(feature = "cssmapple")]
mod cssmapple;
#[cfg(all(feature = "libc", feature = "Authorization"))]
#[allow(unused_imports, unreachable_pub)]
pub use self::authorization::*;
#[cfg(feature = "CipherSuite")]
pub use self::cipher_suite::*;
#[cfg(feature = "cssmapple")]
#[allow(unused_imports, unreachable_pub)]
pub use self::cssmapple::*;

#[cfg(all(feature = "cssmtype", feature = "cssmconfig", feature = "objc2"))]
use objc2::encode::{Encode, Encoding, RefEncode};

// MacTypes.h
#[allow(dead_code)]
pub(crate) type Boolean = u8; // unsigned char
#[allow(dead_code)]
pub(crate) type FourCharCode = u32;
#[allow(dead_code)]
pub(crate) type OSType = FourCharCode;
#[allow(dead_code)]
pub(crate) type OSStatus = i32;

// Fixes
#[allow(non_camel_case_types, dead_code)]
pub(crate) type kr_policy_list_item = core::ffi::c_void;

#[allow(non_camel_case_types)]
#[cfg(feature = "cssmtype")]
pub type CSSM_STRING = [core::ffi::c_char; 68];

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(deprecated)]
#[cfg(all(
    feature = "cssmtype",
    feature = "SecAsn1Types",
    feature = "cssmconfig",
    feature = "objc2"
))]
#[derive(Copy, Clone)]
#[repr(C)]
pub union cssm_list_element_Element {
    pub Sublist: CSSM_LIST,
    pub Word: SecAsn1Item,
}

#[allow(deprecated)]
#[cfg(all(
    feature = "cssmtype",
    feature = "SecAsn1Types",
    feature = "cssmconfig",
    feature = "objc2"
))]
unsafe impl Encode for cssm_list_element_Element {
    const ENCODING: Encoding =
        Encoding::Union("?", &[<CSSM_LIST>::ENCODING, <SecAsn1Item>::ENCODING]);
}

#[allow(deprecated)]
#[cfg(all(
    feature = "cssmtype",
    feature = "SecAsn1Types",
    feature = "cssmconfig",
    feature = "objc2"
))]
unsafe impl RefEncode for cssm_list_element_Element {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[cfg(all(feature = "cssmtype", feature = "SecAsn1Types", feature = "cssmconfig"))]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cssm_list_element {
    pub NextElement: *mut cssm_list_element,
    pub WordID: CSSM_WORDID_TYPE,
    pub ElementType: CSSM_LIST_ELEMENT_TYPE,
    pub Element: cssm_list_element_Element,
}

#[cfg(all(
    feature = "cssmtype",
    feature = "SecAsn1Types",
    feature = "cssmconfig",
    feature = "objc2"
))]
unsafe impl Encode for cssm_list_element {
    // Empty to avoid constant cycle
    const ENCODING: Encoding = Encoding::Struct("cssm_list_element", &[]);
}

#[cfg(all(
    feature = "cssmtype",
    feature = "SecAsn1Types",
    feature = "cssmconfig",
    feature = "objc2"
))]
unsafe impl RefEncode for cssm_list_element {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// Used by `SecAuthenticationType` for endianness-dependent constants.
#[cfg(feature = "SecKeychain")]
#[allow(non_snake_case)]
macro_rules! AUTH_TYPE_FIX_ {
    ($code:expr) => {
        $crate::FourCharCode::from_be($code)
    };
}
#[cfg(feature = "SecKeychain")]
pub(crate) use AUTH_TYPE_FIX_;
