#![allow(deprecated, unused_imports, non_snake_case)]
use crate::*;

#[cfg(feature = "objc2")]
use objc2::encode::{Encode, Encoding, RefEncode};

/// [Apple's documentation](https://developer.apple.com/documentation/security/cssm_tp_apple_evidence_info?language=objc)
#[cfg(all(feature = "SecAsn1Types", feature = "cssmconfig", feature = "cssmtype"))]
#[deprecated]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CSSM_TP_APPLE_EVIDENCE_INFO {
    pub StatusBits: CSSM_TP_APPLE_CERT_STATUS,
    pub NumStatusCodes: uint32,
    pub StatusCodes: *mut CSSM_RETURN,
    pub Index: uint32,
    pub DlDbHandle: CSSM_DL_DB_HANDLE,
    pub UniqueRecord: CSSM_DB_UNIQUE_RECORD_PTR,
    #[cfg(all(target_vendor = "apple", not(target_os = "macos")))]
    pub CrlReason: sint32,
}

#[cfg(all(
    feature = "SecAsn1Types",
    feature = "cssmconfig",
    feature = "cssmtype",
    feature = "objc2"
))]
unsafe impl Encode for CSSM_TP_APPLE_EVIDENCE_INFO {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <CSSM_TP_APPLE_CERT_STATUS>::ENCODING,
            <uint32>::ENCODING,
            <*mut CSSM_RETURN>::ENCODING,
            <uint32>::ENCODING,
            <CSSM_DL_DB_HANDLE>::ENCODING,
            <CSSM_DB_UNIQUE_RECORD_PTR>::ENCODING,
            #[cfg(all(target_vendor = "apple", not(target_os = "macos")))]
            <sint32>::ENCODING,
        ],
    );
}

#[cfg(all(
    feature = "SecAsn1Types",
    feature = "cssmconfig",
    feature = "cssmtype",
    feature = "objc2"
))]
unsafe impl RefEncode for CSSM_TP_APPLE_EVIDENCE_INFO {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
