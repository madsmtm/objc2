#![allow(non_upper_case_globals)]

#[cfg(feature = "CTFont")]
impl crate::CTFontUIFontType {
    #[doc(alias = "kCTFontUIFontNone")]
    pub const None: Self = Self(u32::MAX);
}

#[cfg(feature = "CTRubyAnnotation")]
impl crate::CTRubyAlignment {
    #[doc(alias = "kCTRubyAlignmentInvalid")]
    pub const Invalid: Self = Self(u8::MAX);
}

#[cfg(feature = "CTRubyAnnotation")]
impl crate::CTRubyOverhang {
    #[doc(alias = "kCTRubyOverhangInvalid")]
    pub const Invalid: Self = Self(u8::MAX);
}

#[cfg(feature = "SFNTLayoutTypes")]
pub const kMORTLigLastAction: core::ffi::c_int = 0x80000000u32 as _;

#[cfg(feature = "SFNTLayoutTypes")]
pub const kMORXCoverVertical: core::ffi::c_int = 0x80000000u32 as _;

#[cfg(feature = "SFNTLayoutTypes")]
pub const kKERXVertical: core::ffi::c_int = 0x80000000u32 as _;
