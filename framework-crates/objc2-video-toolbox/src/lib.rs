//! # Bindings to the `VideoToolbox` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/videotoolbox/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(feature = "unstable-darwin-objc", feature(darwin_objc))]
#![cfg_attr(docsrs, feature(doc_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-video-toolbox/0.3.2")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod generated;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;

// MacTypes.h
#[allow(dead_code)]
pub(crate) type Boolean = u8;
#[allow(dead_code)]
pub(crate) type OSStatus = i32;
#[allow(dead_code)]
pub(crate) type OSType = u32;

// NOTE: `VTRAWProcessingSessionRef` is marked as `CM_SWIFT_NONSENDABLE`, but
// `$(xcrun --show-sdk-path)/usr/lib/swift/VideoToolbox.swiftmodule/*`
// includes an `@unchecked Swift.Sendable` extension, so might be safe to
// mark it as sendable?

/// Prototype for callback invoked when frame compression is complete.
///
/// When you create a compression session, you pass in a callback function to be called
/// for compressed frames.  This function will be called in decode order (which is not
/// necessarily the same as display order).
///
/// Parameter `outputCallbackRefCon`: The callback's reference value.
///
/// Parameter `sourceFrameRefCon`: The frame's reference value, copied from the sourceFrameRefCon argument to
/// VTCompressionSessionEncodeFrame.
///
/// Parameter `status`: noErr if compression was successful; an error code if compression was not successful.
///
/// Parameter `infoFlags`: Contains information about the encode operation.
/// The kVTEncodeInfo_Asynchronous bit may be set if the encode ran asynchronously.
/// The kVTEncodeInfo_FrameDropped bit may be set if the frame was dropped.
///
/// Parameter `sampleBuffer`: Contains the compressed frame, if compression was successful and the frame was not dropped;
/// otherwise, NULL.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/videotoolbox/vtcompressionoutputcallback?language=objc)
#[cfg(all(
    feature = "VTCompressionSession",
    feature = "VTErrors",
    feature = "objc2-core-media"
))]
pub type VTCompressionOutputCallback = unsafe extern "C-unwind" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    OSStatus,
    VTEncodeInfoFlags,
    *mut objc2_core_media::CMSampleBuffer,
);
