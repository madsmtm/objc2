//! Private functionality.
//!
//! The credit for finding these belong to the [metal-rs] project.
//!
//! [metal-rs]: https://github.com/gfx-rs/metal-rs

use crate::common::*;
use crate::Foundation;
use crate::Metal;

extern_methods!(
    unsafe impl Metal::MTLDevice {
        #[method_id(vendorName)]
        pub unsafe fn vendorName(&self) -> Id<Foundation::NSString, Shared>;

        #[method_id(familyName)]
        pub unsafe fn familyName(&self) -> Id<Foundation::NSString, Shared>;
    }
);

extern_methods!(
    unsafe impl Metal::MTLRenderPipelineReflection {
        #[method_id(initWithVertexData:fragmentData:serializedVertexDescriptor:device:options:flags:)]
        pub unsafe fn initWithVertexData(
            this: Option<Allocated<Self>>,
            vertex_data: *mut c_void,
            fragment_data: *mut c_void,
            vertex_desc: *mut c_void,
            device: &Metal::MTLDevice,
            options: u64,
            flags: u64,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(newSerializedVertexDataWithFlags:error:_)]
        pub unsafe fn newSerializedVertexDataWithFlags_error(
            &self,
            flags: u64,
        ) -> Result<Id<Object, Shared>, Id<Foundation::NSError, Shared>>;

        #[method(serializeFragmentData)]
        pub unsafe fn serializeFragmentData(&self) -> *mut c_void;
    }
);

extern_methods!(
    unsafe impl Metal::MTLSamplerDescriptor {
        #[method(setLodBias:)]
        pub unsafe fn setLodBias(&self, bias: f32);
    }
);

extern_methods!(
    unsafe impl Metal::MTLVertexDescriptor {
        #[method_id(newSerializedDescriptor)]
        pub unsafe fn newSerializedDescriptor(&self) -> Option<Id<Object, Shared>>;
    }
);
