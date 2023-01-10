use crate::Metal;

// SAFETY: The documentation for captureObject specifies that the object
// may be one of these three:
// <https://developer.apple.com/documentation/metal/mtlcapturedescriptor/3237248-captureobject?language=objc>
#[cfg(feature = "Metal_MTLCaptureDescriptor")]
impl Metal::MTLCaptureDescriptor {
    #[doc(alias = "setCaptureObject")]
    #[cfg(feature = "Metal_MTLDevice")]
    pub fn set_capture_device(&self, device: &Metal::MTLDevice) {
        unsafe { self.setCaptureObject(Some(device)) }
    }

    #[doc(alias = "setCaptureObject")]
    #[cfg(feature = "Metal_MTLCaptureScope")]
    pub fn set_capture_scope(&self, scope: &Metal::MTLCaptureScope) {
        unsafe { self.setCaptureObject(Some(scope)) }
    }

    #[doc(alias = "setCaptureObject")]
    #[cfg(feature = "Metal_MTLCommandQueue")]
    pub fn set_capture_command_queue(&self, command_queue: &Metal::MTLCommandQueue) {
        unsafe { self.setCaptureObject(Some(command_queue)) }
    }
}
