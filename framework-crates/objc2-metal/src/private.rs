//! Private functionality.
//!
//! The credit for finding these belong to the [metal-rs] project.
//!
//! [metal-rs]: https://github.com/gfx-rs/metal-rs
#![allow(clippy::missing_safety_doc)]
#![allow(unused_imports)]
use core::ffi::c_void;

use crate::*;

use objc2::rc::{Allocated, Retained};
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2::{extern_methods, msg_send, Message};

pub unsafe trait MTLDevicePrivate: Message {
    unsafe fn vendor_name(&self) -> Retained<objc2_foundation::NSString> {
        unsafe { msg_send![self, vendorName] }
    }

    unsafe fn family_name(&self) -> Retained<objc2_foundation::NSString> {
        unsafe { msg_send![self, familyName] }
    }
}

#[cfg(feature = "MTLDevice")]
unsafe impl<P: MTLDevice + Message> MTLDevicePrivate for P {}

#[cfg(feature = "MTLRenderPipeline")]
impl MTLRenderPipelineReflection {
    extern_methods!(
        #[cfg(feature = "MTLDevice")]
        #[unsafe(method(initWithVertexData:fragmentData:serializedVertexDescriptor:device:options:flags:))]
        pub unsafe fn init_with_vertex_data(
            this: Allocated<Self>,
            vertex_data: *mut c_void,
            fragment_data: *mut c_void,
            vertex_desc: *mut c_void,
            device: &ProtocolObject<dyn MTLDevice>,
            options: u64,
            flags: u64,
        ) -> Option<Retained<Self>>;

        #[unsafe(method(newSerializedVertexDataWithFlags:error:_))]
        pub unsafe fn new_serialized_vertex_data_with_flags_error(
            &self,
            flags: u64,
        ) -> Result<Retained<AnyObject>, Retained<objc2_foundation::NSError>>;

        #[unsafe(method(serializeFragmentData))]
        pub unsafe fn serialize_fragment_data(&self) -> *mut c_void;
    );
}

#[cfg(feature = "MTLSampler")]
impl MTLSamplerDescriptor {
    extern_methods!(
        #[unsafe(method(setLodBias:))]
        pub unsafe fn set_lod_bias(&self, bias: f32);
    );
}

#[cfg(feature = "MTLVertexDescriptor")]
impl MTLVertexDescriptor {
    extern_methods!(
        #[unsafe(method(newSerializedDescriptor))]
        pub unsafe fn new_serialized_descriptor(&self) -> Option<Retained<AnyObject>>;
    );
}
