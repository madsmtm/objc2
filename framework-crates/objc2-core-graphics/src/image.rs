use crate::{
    CGBitmapInfo, CGImageAlphaInfo, CGImageByteOrderInfo, CGImageComponentInfo,
    CGImagePixelFormatInfo,
};

#[allow(non_upper_case_globals, deprecated)]
impl CGBitmapInfo {
    #[doc(alias = "kCGBitmapByteOrder16Host")]
    pub const ByteOrder16Host: Self = if cfg!(target_endian = "big") {
        Self::ByteOrder16Big
    } else {
        Self::ByteOrder16Little
    };

    #[doc(alias = "kCGBitmapByteOrder32Host")]
    pub const ByteOrder32Host: Self = if cfg!(target_endian = "big") {
        Self::ByteOrder32Big
    } else {
        Self::ByteOrder32Little
    };
}

#[allow(non_upper_case_globals, deprecated)]
impl CGImageByteOrderInfo {
    #[doc(alias = "kCGImageByteOrder16Host")]
    pub const Order16Host: Self = if cfg!(target_endian = "big") {
        Self::Order16Big
    } else {
        Self::Order16Little
    };

    #[doc(alias = "kCGImageByteOrder32Host")]
    pub const Order32Host: Self = if cfg!(target_endian = "big") {
        Self::Order32Big
    } else {
        Self::Order32Little
    };
}

impl CGBitmapInfo {
    /// Create a bitmap info structure from its four component parts.
    #[doc(alias = "CGBitmapInfoMake")]
    pub fn new(
        alpha: CGImageAlphaInfo,
        component: CGImageComponentInfo,
        byte_order: CGImageByteOrderInfo,
        pixel_format: CGImagePixelFormatInfo,
    ) -> Self {
        Self(alpha.0 | component.0 | byte_order.0 | pixel_format.0)
    }
}
