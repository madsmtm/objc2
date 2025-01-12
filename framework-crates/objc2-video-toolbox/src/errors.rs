#[cfg(feature = "objc2")]
use objc2::encode::{Encode, Encoding, RefEncode};

/// Directives for the decompression session and the video decoder, passed into
/// decodeFlags parameter of VTDecompressionSessionDecodeFrame.
///
///
/// With the kVTDecodeFrame_EnableAsynchronousDecompression bit clear, the video decoder
/// is compelled to emit every frame before it returns.  With the bit set, the decoder may
/// process frames asynchronously, but it is not compelled to do so.
///
/// A hint to the decompression session and video decoder that a CVImageBuffer should not
/// be emitted for this frame.  NULL will be returned instead.
///
/// A hint to the video decoder that it would be OK to use a low-power mode that can not decode faster than 1x realtime.
///
/// With the kVTDecodeFrame_EnableTemporalProcessing bit clear, the video decoder should emit
/// every frame once that frame's decoding is done -- frames may not be delayed indefinitely.  With
/// the bit set, it is legal for the decoder to delay frames indefinitely -- at least
/// until VTDecompressionSessionFinishDelayedFrames or VTDecompressionSessionInvalidate is called.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/videotoolbox/vtdecodeframeflags?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VTDecodeFrameFlags(pub u32);
bitflags::bitflags! {
    impl VTDecodeFrameFlags: u32 {
        #[doc(alias = "kVTDecodeFrame_EnableAsynchronousDecompression")]
        const Frame_EnableAsynchronousDecompression = 1<<0;
        #[doc(alias = "kVTDecodeFrame_DoNotOutputFrame")]
        const Frame_DoNotOutputFrame = 1<<1;
        #[doc(alias = "kVTDecodeFrame_1xRealTimePlayback")]
        const Frame_1xRealTimePlayback = 1<<2;
        #[doc(alias = "kVTDecodeFrame_EnableTemporalProcessing")]
        const Frame_EnableTemporalProcessing = 1<<3;
    }
}

#[cfg(feature = "objc2")]
unsafe impl Encode for VTDecodeFrameFlags {
    const ENCODING: Encoding = u32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for VTDecodeFrameFlags {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
