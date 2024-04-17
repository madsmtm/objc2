//! Private functionality.
//!
//! The credit for finding these belong to the [metal-rs] project.
//!
//! [metal-rs]: https://github.com/gfx-rs/metal-rs
#![allow(clippy::missing_safety_doc)]
#![allow(unused_imports)]
use std::ffi::c_void;

use crate::*;

use objc2::rc::{Allocated, Id};
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2::{extern_methods, msg_send_id, Message};

pub unsafe trait MTLDevicePrivate: Message {
    unsafe fn vendorName(&self) -> Id<objc2_foundation::NSString> {
        unsafe { msg_send_id![self, vendorName] }
    }

    unsafe fn familyName(&self) -> Id<objc2_foundation::NSString> {
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

        #[method_id(newSerializedVertexDataWithFlags:error:_)]
        pub unsafe fn newSerializedVertexDataWithFlags_error(
            &self,
            flags: u64,
        ) -> Result<Id<AnyObject>, Id<objc2_foundation::NSError>>;

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
