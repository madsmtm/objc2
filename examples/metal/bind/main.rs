// Copyright 2018 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

// Modified from <https://github.com/gfx-rs/metal-rs/tree/v0.33.0/examples/bind>

use objc2::{rc::autoreleasepool, runtime::ProtocolObject};
use objc2_foundation::NSRange;
use objc2_metal::{
    MTLBuffer, MTLCommandBuffer, MTLCommandEncoder, MTLCommandQueue, MTLComputeCommandEncoder,
    MTLCreateSystemDefaultDevice, MTLDevice, MTLResourceOptions, MTLSamplerDescriptor,
    MTLSamplerState,
};
use std::ptr::{self, NonNull};

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {}

fn main() {
    autoreleasepool(|_| {
        let device = MTLCreateSystemDefaultDevice().expect("no device found");

        let buffer = device
            .newBufferWithLength_options(8, MTLResourceOptions::empty())
            .unwrap();
        let sampler = {
            let descriptor = MTLSamplerDescriptor::new();
            device.newSamplerStateWithDescriptor(&descriptor).unwrap()
        };

        let queue = device.newCommandQueue().unwrap();
        let cmd_buf = queue.commandBuffer().unwrap();

        let encoder = cmd_buf.computeCommandEncoder().unwrap();

        let buffers = &[
            &*buffer as *const ProtocolObject<dyn MTLBuffer>,
            ptr::null(),
        ];
        let offsets = &[4, 0];
        unsafe {
            encoder.setBuffers_offsets_withRange(
                NonNull::new(buffers.as_ptr().cast_mut()).unwrap(),
                NonNull::new(offsets.as_ptr().cast_mut()).unwrap(),
                NSRange {
                    location: 2,
                    length: buffers.len() as _,
                },
            )
        };
        let states = &[
            &*sampler as *const ProtocolObject<dyn MTLSamplerState>,
            ptr::null(),
        ];
        unsafe {
            encoder.setSamplerStates_withRange(
                NonNull::new(states.as_ptr().cast_mut()).unwrap(),
                NSRange {
                    location: 1,
                    length: 2,
                },
            )
        };

        encoder.endEncoding();
        cmd_buf.commit();

        println!("Everything is bound");
    });
}
