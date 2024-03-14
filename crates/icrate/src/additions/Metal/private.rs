//! Private functionality.
//!
//! The credit for finding these belong to the [metal-rs] project.
//!
//! [metal-rs]: https://github.com/gfx-rs/metal-rs
#![allow(clippy::missing_safety_doc)]

use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

use objc2::msg_send_id;

pub unsafe trait MTLDevicePrivate: MTLDevice + Message {
    #[cfg(feature = "Foundation_NSString")]
    unsafe fn vendorName(&self) -> Id<NSString> {
        unsafe { msg_send_id![self, vendorName] }
    }

    #[cfg(feature = "Foundation_NSString")]
    unsafe fn familyName(&self) -> Id<NSString> {
        unsafe { msg_send_id![self, familyName] }
    }
}

#[cfg(feature = "Metal_MTLDevice")]
unsafe impl<P: MTLDevice + Message> MTLDevicePrivate for P {}

extern_methods!(
    #[cfg(feature = "Metal_MTLRenderPipeline")]
    unsafe impl MTLRenderPipelineReflection {
        #[cfg(feature = "Metal_MTLDevice")]
        #[method_id(initWithVertexData:fragmentData:serializedVertexDescriptor:device:options:flags:)]
        pub unsafe fn initWithVertexData(
            this: Allocated<Self>,
            vertex_data: *mut c_void,
            fragment_data: *mut c_void,
            vertex_desc: *mut c_void,
            device: &ProtocolObject<dyn MTLDevice>,
            options: u64,
            flags: u64,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(newSerializedVertexDataWithFlags:error:_)]
        pub unsafe fn newSerializedVertexDataWithFlags_error(
            &self,
            flags: u64,
        ) -> Result<Id<AnyObject>, Id<NSError>>;

        #[method(serializeFragmentData)]
        pub unsafe fn serializeFragmentData(&self) -> *mut c_void;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLSamplerDescriptor")]
    unsafe impl MTLSamplerDescriptor {
        #[method(setLodBias:)]
        pub unsafe fn setLodBias(&self, bias: f32);
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLVertexDescriptor")]
    unsafe impl MTLVertexDescriptor {
        #[method_id(newSerializedDescriptor)]
        pub unsafe fn newSerializedDescriptor(&self) -> Option<Id<AnyObject>>;
    }
);
