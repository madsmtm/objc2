use crate::MTLRasterizationRateLayerDescriptor;
use objc2::extern_methods;

impl MTLRasterizationRateLayerDescriptor {
    extern_methods!(
        /// The number of quality samples that this descriptor uses to
        /// describe its current function, for the horizontal and vertical
        /// axis. The depth component of the returned `MTLSize` is always 0.
        #[cfg(feature = "MTLTypes")]
        #[unsafe(method(sampleCount))]
        pub fn sampleCount(&self) -> crate::MTLSize;
    );
}
