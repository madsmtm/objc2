#![allow(non_snake_case)]
use objc2::extern_methods;
use objc2::rc::Retained;
use objc2_foundation::{NSError, NSRange};

use crate::{VNRecognizedText, VNRectangleObservation};

impl VNRecognizedText {
    extern_methods!(
        /// Calculate the bounding box around the characters in the range of
        /// the string.
        ///
        /// The bounding boxes are not guaranteed to be an exact fit around
        /// the characters and are purely meant for UI purposes and not for
        /// image processing.
        #[unsafe(method(boundingBoxForRange:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn boundingBoxForRange_error(
            &self,
            range: NSRange,
        ) -> Result<Retained<VNRectangleObservation>, Retained<NSError>>;
    );
}
