//! Ensure that passing NULL to cg_nullable methods doesn't crash.
//!
//! TODO(breaking): Make these take a non-null parameter instead.
#![cfg(feature = "CGContext")]
#![cfg(feature = "CGColorSpace")]
#![cfg(feature = "CGColorConversionInfo")]
#![cfg(feature = "CGPDFContext")]
#![cfg(feature = "CGBitmapContext")]
#![cfg(feature = "CGImage")]

use objc2_core_graphics::{
    CGBitmapContextCreate, CGBitmapInfo, CGColorConversionInfo, CGColorSpace, CGContext,
    CGImageAlphaInfo, CGImageByteOrderInfo, CGImageComponentInfo, CGImagePixelFormatInfo,
    CGPDFContextClose,
};

#[test]
fn null_context() {
    CGContext::save_g_state(None);
}

#[test]
fn null_colorspace() {
    assert_eq!(CGColorConversionInfo::new(None, None), None);
}

#[test]
fn non_pdf_context() {
    let color_space = CGColorSpace::new_device_rgb().unwrap();
    let context = unsafe {
        CGBitmapContextCreate(
            std::ptr::null_mut(),
            100,
            100,
            8,
            0,
            Some(&color_space),
            CGBitmapInfo::new(
                CGImageAlphaInfo::NoneSkipFirst,
                CGImageComponentInfo::Integer,
                CGImageByteOrderInfo::Order32Little,
                CGImagePixelFormatInfo::Packed,
            )
            .0,
        )
        .unwrap()
    };

    // Close a non-PDF context.
    CGPDFContextClose(Some(&context));
}
