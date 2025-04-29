// Copyright 2017 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

// Modified from <https://github.com/gfx-rs/metal-rs/tree/v0.33.0/examples/argument-buffer>

use std::ptr::NonNull;

use objc2::{rc::autoreleasepool, runtime::ProtocolObject};
use objc2_foundation::NSArray;
use objc2_metal::{
    MTLArgumentDescriptor, MTLArgumentEncoder, MTLCommandBuffer, MTLCommandEncoder,
    MTLCommandQueue, MTLCreateSystemDefaultDevice, MTLDataType, MTLDevice, MTLRenderCommandEncoder,
    MTLRenderPassDescriptor, MTLRenderStages, MTLResource, MTLResourceOptions, MTLResourceUsage,
    MTLSamplerDescriptor, MTLTextureType,
};

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {}

fn main() {
    autoreleasepool(|_| {
        let device = MTLCreateSystemDefaultDevice().expect("no device found");

        // Build encoder for the following MSL argument buffer:
        //
        // struct ArgumentBuffer {
        //     texture2d<float> texture [[id(0)]];
        //     sampler sampler [[id(1)]];
        //     array<device float *, 2> buffers [[id(2)]];
        // }

        let desc1 = MTLArgumentDescriptor::new();
        desc1.setIndex(0);
        desc1.setDataType(MTLDataType::Texture);
        desc1.setTextureType(MTLTextureType::Type2D);

        let desc2 = MTLArgumentDescriptor::new();
        desc2.setIndex(1);
        desc2.setDataType(MTLDataType::Sampler);

        let desc3 = MTLArgumentDescriptor::new();
        desc3.setIndex(2);
        desc3.setDataType(MTLDataType::Pointer);
        desc3.setArrayLength(2);

        let encoder = device
            .newArgumentEncoderWithArguments(&NSArray::from_retained_slice(&[desc1, desc2, desc3]))
            .unwrap();
        println!("Encoder: {:?}", encoder);

        let argument_buffer = device
            .newBufferWithLength_options(encoder.encodedLength(), MTLResourceOptions::empty())
            .unwrap();
        unsafe { encoder.setArgumentBuffer_offset(Some(&argument_buffer), 0) };

        let sampler = {
            let descriptor = MTLSamplerDescriptor::new();
            descriptor.setSupportArgumentBuffers(true);
            device.newSamplerStateWithDescriptor(&descriptor).unwrap()
        };
        println!("{:?}", sampler);

        let buffer1 = device
            .newBufferWithLength_options(1024, MTLResourceOptions::empty())
            .unwrap();
        println!("Buffer1: {:?}", buffer1);
        let buffer2 = device
            .newBufferWithLength_options(1024, MTLResourceOptions::empty())
            .unwrap();
        println!("Buffer2: {:?}", buffer2);

        unsafe {
            encoder.setSamplerState_atIndex(Some(&sampler), 1);
            encoder.setBuffer_offset_atIndex(Some(&buffer1), 0, 2);
            encoder.setBuffer_offset_atIndex(Some(&buffer2), 0, 3);
        }

        // How to use argument buffer with render encoder.

        let queue = device.newCommandQueue().unwrap();
        let command_buffer = queue.commandBuffer().unwrap();

        // Use a targetless encoder for the example.
        let render_pass_descriptor = MTLRenderPassDescriptor::new();
        render_pass_descriptor.setRenderTargetWidth(256);
        render_pass_descriptor.setRenderTargetHeight(256);
        render_pass_descriptor.setDefaultRasterSampleCount(1);
        let encoder = command_buffer
            .renderCommandEncoderWithDescriptor(&render_pass_descriptor)
            .unwrap();

        // This method makes the array of resources resident for the selected stages of the render pass.
        // Call this method before issuing any draw calls that may access the array of resources.
        let resources: &[&ProtocolObject<dyn MTLResource>; 2] =
            &[buffer1.as_ref(), buffer2.as_ref()];
        unsafe {
            encoder.useResources_count_usage_stages(
                NonNull::new(resources.as_ptr().cast_mut()).unwrap().cast(),
                resources.len(),
                MTLResourceUsage::Read,
                MTLRenderStages::Vertex,
            )
        };
        // Bind argument buffer to vertex stage.
        unsafe { encoder.setVertexBuffer_offset_atIndex(Some(&argument_buffer), 0, 0) };

        // Render pass here...

        encoder.endEncoding();
        println!("Encoder: {:?}", encoder);

        command_buffer.commit();
    });
}
