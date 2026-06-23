#![cfg(feature = "CVBuffer")]
#![cfg(feature = "CVPixelBuffer")]
#![cfg(feature = "CVMetalBuffer")]
#![cfg(feature = "CVOpenGLBuffer")]
#![allow(deprecated)]

use objc2_core_foundation::CFRetained;
use objc2_core_video::{kCVPixelFormatType_32BGRA, kCVReturnSuccess, CVPixelBuffer};

fn new_buffer() -> CFRetained<CVPixelBuffer> {
    let mut buffer = None;
    let res =
        unsafe { CVPixelBuffer::new(None, 100, 100, kCVPixelFormatType_32BGRA, None, &mut buffer) };
    assert_eq!(res, kCVReturnSuccess);
    buffer.unwrap()
}

/// Test that using a buffer method on a buffer not of that type is sound
/// (if still incorrect).
///
/// Nowadays, we prevent this at compile-time by making `CVOpenGLBuffer` a
/// different type from `CVPixelBuffer`.
//
// `CVOpenGLBufferGetAttributes` is only available on macOS.
#[cfg(target_os = "macos")]
#[test]
fn invalid_type() {
    use objc2_core_video::CVOpenGLBuffer;

    let buffer = new_buffer();
    // Using a pixel buffer as a Metal / OpenGL buffer just returns `None`.
    //
    // `CVMetalBufferGetBuffer` was added in macOS 15.0, so we can't actually
    // test this yet.
    // assert_eq!(CVMetalBufferGetBuffer(&buffer), None);

    let buffer = unsafe { CFRetained::cast_unchecked::<CVOpenGLBuffer>(buffer) };
    assert_eq!(buffer.attributes(), None);
}

/// Test that passing an invalid index doesn't crash.
#[test]
fn invalid_plane_index() {
    let buffer = new_buffer();
    assert_eq!(buffer.width_of_plane(100), 0);
}

#[test]
#[should_panic = "parameter `pixel_buffer_out` must point to `None` on entry"]
fn new_buffer_with_existing() {
    let mut buffer = Some(new_buffer());
    unsafe { CVPixelBuffer::new(None, 100, 100, kCVPixelFormatType_32BGRA, None, &mut buffer) };
}
