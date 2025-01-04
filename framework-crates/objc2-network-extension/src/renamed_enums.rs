#![allow(non_upper_case_globals)]

use objc2::encode::{Encode, Encoding, RefEncode};
use objc2::ffi::NSInteger;

/// TLS version to use during TLS handshake.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/networkextension/nehotspotconfigurationeaptlsversion?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NEHotspotConfigurationEAPTLSVersion(pub NSInteger);
impl NEHotspotConfigurationEAPTLSVersion {
    #[doc(alias = "NEHotspotConfigurationEAPTLSVersion_1_0")]
    pub const Version_1_0: Self = Self(0);
    #[doc(alias = "NEHotspotConfigurationEAPTLSVersion_1_1")]
    pub const Version_1_1: Self = Self(1);
    #[doc(alias = "NEHotspotConfigurationEAPTLSVersion_1_2")]
    pub const Version_1_2: Self = Self(2);
}

unsafe impl Encode for NEHotspotConfigurationEAPTLSVersion {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NEHotspotConfigurationEAPTLSVersion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// IKEv2 Encryption Algorithms.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/networkextension/nevpnikev2encryptionalgorithm?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NEVPNIKEv2EncryptionAlgorithm(pub NSInteger);
impl NEVPNIKEv2EncryptionAlgorithm {
    #[deprecated = "Use an encryption algorithm with 256-bit keys instead"]
    #[doc(alias = "NEVPNIKEv2EncryptionAlgorithmDES")]
    pub const AlgorithmDES: Self = Self(1);
    #[deprecated = "Use an encryption algorithm with 256-bit keys instead"]
    #[doc(alias = "NEVPNIKEv2EncryptionAlgorithm3DES")]
    pub const Algorithm3DES: Self = Self(2);
    #[deprecated = "Use an encryption algorithm with 256-bit keys instead"]
    #[doc(alias = "NEVPNIKEv2EncryptionAlgorithmAES128")]
    pub const AlgorithmAES128: Self = Self(3);
    #[doc(alias = "NEVPNIKEv2EncryptionAlgorithmAES256")]
    pub const AlgorithmAES256: Self = Self(4);
    #[deprecated = "Use an encryption algorithm with 256-bit keys instead"]
    #[doc(alias = "NEVPNIKEv2EncryptionAlgorithmAES128GCM")]
    pub const AlgorithmAES128GCM: Self = Self(5);
    #[doc(alias = "NEVPNIKEv2EncryptionAlgorithmAES256GCM")]
    pub const AlgorithmAES256GCM: Self = Self(6);
    #[doc(alias = "NEVPNIKEv2EncryptionAlgorithmChaCha20Poly1305")]
    pub const AlgorithmChaCha20Poly1305: Self = Self(7);
}

unsafe impl Encode for NEVPNIKEv2EncryptionAlgorithm {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NEVPNIKEv2EncryptionAlgorithm {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
