#![cfg(feature = "CGContext")]
#![cfg(feature = "CGColorSpace")]
#![cfg(feature = "CGPDFContext")]
#![cfg(feature = "CGBitmapContext")]
#![cfg(feature = "CGImage")]

use objc2_core_graphics::{
    CGBitmapContextCreate, CGBitmapInfo, CGColorSpace, CGImageAlphaInfo, CGImageByteOrderInfo,
    CGImageComponentInfo, CGImagePixelFormatInfo, CGPDFContextClose,
};

#[test]
fn non_pdf_context() {
    let color_space = CGColorSpace::new_device_rgb();
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
            ),
        )
        .unwrap()
    };

    // Close a non-PDF context.
    CGPDFContextClose(&context);
}
