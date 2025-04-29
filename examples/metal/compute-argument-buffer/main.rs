// Copyright 2017 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
// Modified from <https://github.com/gfx-rs/metal-rs/tree/v0.33.0/examples/compute-argument-buffer>

use std::{mem::size_of_val, ptr::NonNull};

use objc2::rc::autoreleasepool;
use objc2_foundation::ns_string;
use objc2_metal::{
    MTLArgumentEncoder, MTLBuffer, MTLCommandBuffer, MTLCommandEncoder, MTLCommandQueue,
    MTLCompileOptions, MTLComputeCommandEncoder, MTLComputePipelineDescriptor,
    MTLCreateSystemDefaultDevice, MTLDevice, MTLFunction, MTLLibrary, MTLResourceOptions,
    MTLResourceUsage, MTLSize,
};

fn main() {
    autoreleasepool(|_| {
        let device = MTLCreateSystemDefaultDevice().expect("no device found");
        let command_queue = device.newCommandQueue().unwrap();

        let data = [
            1u32, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30,
        ];

        let buffer = unsafe {
            device.newBufferWithBytes_length_options(
                NonNull::new(data.as_ptr().cast_mut().cast()).unwrap(),
                size_of_val(&data),
                MTLResourceOptions::CPUCacheModeDefaultCache,
            )
        }
        .unwrap();

        let sum = unsafe {
            let data = [0u32];
            device.newBufferWithBytes_length_options(
                NonNull::new(data.as_ptr().cast_mut().cast()).unwrap(),
                size_of_val(&data),
                MTLResourceOptions::CPUCacheModeDefaultCache,
            )
        }
        .unwrap();

        let command_buffer = command_queue.commandBuffer().unwrap();
        let encoder = command_buffer.computeCommandEncoder().unwrap();

        let library = device
            .newLibraryWithSource_options_error(
                ns_string!(include_str!("shaders.metal")),
                Some(&MTLCompileOptions::new()),
            )
            .unwrap_or_else(|e| panic!("{e}"));
        let kernel = library.newFunctionWithName(ns_string!("sum")).unwrap();

        let argument_encoder = unsafe { kernel.newArgumentEncoderWithBufferIndex(0) };
        let arg_buffer = device
            .newBufferWithLength_options(
                argument_encoder.encodedLength(),
                MTLResourceOptions::empty(),
            )
            .unwrap();
        unsafe { argument_encoder.setArgumentBuffer_offset(Some(&arg_buffer), 0) };
        unsafe { argument_encoder.setBuffer_offset_atIndex(Some(&buffer), 0, 0) };
        unsafe { argument_encoder.setBuffer_offset_atIndex(Some(&sum), 0, 1) };

        let pipeline_state_descriptor = MTLComputePipelineDescriptor::new();
        pipeline_state_descriptor.setComputeFunction(Some(&kernel));

        let pipeline_state = device
            .newComputePipelineStateWithFunction_error(
                &pipeline_state_descriptor.computeFunction().unwrap(),
            )
            .unwrap();

        encoder.setComputePipelineState(&pipeline_state);
        unsafe { encoder.setBuffer_offset_atIndex(Some(&arg_buffer), 0, 0) };

        encoder.useResource_usage(buffer.as_ref(), MTLResourceUsage::Read);
        encoder.useResource_usage(sum.as_ref(), MTLResourceUsage::Write);

        let width = 16;

        let thread_group_count = MTLSize {
            width,
            height: 1,
            depth: 1,
        };

        let thread_group_size = MTLSize {
            width: (data.len() + width) / width,
            height: 1,
            depth: 1,
        };

        encoder.dispatchThreadgroups_threadsPerThreadgroup(thread_group_count, thread_group_size);
        encoder.endEncoding();
        command_buffer.commit();
        command_buffer.waitUntilCompleted();

        let sum = unsafe { sum.contents().cast::<u32>().read() };
        assert_eq!(465, sum);
    });
}
