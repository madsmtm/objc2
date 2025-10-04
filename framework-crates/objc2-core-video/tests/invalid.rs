#![cfg(feature = "CVBuffer")]
#![cfg(feature = "CVPixelBuffer")]
#![cfg(feature = "CVMetalBuffer")]
#![cfg(feature = "CVOpenGLBuffer")]
#![allow(deprecated)]
use std::ptr::{self, NonNull};

use objc2_core_foundation::CFRetained;
use objc2_core_video::{
    kCVPixelFormatType_32BGRA, kCVReturnSuccess, CVPixelBuffer, CVPixelBufferCreate,
    CVPixelBufferGetWidthOfPlane,
};

fn new_buffer() -> CFRetained<CVPixelBuffer> {
    let mut buffer = ptr::null_mut();
    let res = unsafe {
        CVPixelBufferCreate(
            None,
            100,
            100,
            kCVPixelFormatType_32BGRA,
            None,
            NonNull::from(&mut buffer),
        )
    };
    assert_eq!(res, kCVReturnSuccess);
    unsafe { CFRetained::from_raw(NonNull::new(buffer).unwrap()) }
}

/// Test that using a buffer method on a buffer not of that type is sound
/// (if still incorrect).
///
/// Ideally, this should be prevented at compile-time (e.g. by making
/// `CVMetalBuffer` a different type from `CVBuffer`).
//
// `CVOpenGLBufferGetAttributes` is only available on macOS.
#[cfg(target_os = "macos")]
#[test]
fn invalid_type() {
    use objc2_core_video::CVOpenGLBufferGetAttributes;

    let buffer = new_buffer();
    // Using a pixel buffer as a Metal / OpenGL buffer just returns `None`.
    //
    // `CVMetalBufferGetBuffer` was added in macOS 15.0, so we can't actually
    // test this yet.
    // assert_eq!(CVMetalBufferGetBuffer(&buffer), None);
    assert_eq!(CVOpenGLBufferGetAttributes(&buffer), None);
}

/// Test that passing an invalid index doesn't crash.
#[test]
fn invalid_plane_index() {
    let buffer = new_buffer();
    assert_eq!(CVPixelBufferGetWidthOfPlane(&buffer, 100), 0);
}
