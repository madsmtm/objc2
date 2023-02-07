use crate::Metal;

#[cfg(feature = "Metal_MTLDevice")]
impl Metal::MTLDevice {
    // pub fn system_default() -> Option<Self> {
    //     MTLCreateSystemDefaultDevice()
    // }
    //
    // pub fn all() -> Id<NSArray<Self>> {
    //     #[cfg(target_os = "macos")]
    //     MTLCopyAllDevices()
    //     #[cfg(not(target_os = "macos"))]
    //     NSArray::from(MTLCreateSystemDefaultDevice())
    // }
}
