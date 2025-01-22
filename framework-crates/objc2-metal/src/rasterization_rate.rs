use crate::MTLRasterizationRateLayerDescriptor;
use objc2::extern_methods;

extern_methods!(
    unsafe impl MTLRasterizationRateLayerDescriptor {
        /// The number of quality samples that this descriptor uses to
        /// describe its current function, for the horizontal and vertical
        /// axis. The depth component of the returned `MTLSize` is always 0.
        #[cfg(feature = "MTLTypes")]
        #[unsafe(method(sampleCount))]
        pub fn sampleCount(&self) -> crate::MTLSize;
    }
);
