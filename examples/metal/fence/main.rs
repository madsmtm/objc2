// Copyright 2020 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

// Modified from <https://github.com/gfx-rs/metal-rs/tree/v0.33.0/examples/fence>

use objc2_metal::{
    MTLBlitCommandEncoder, MTLCommandBuffer, MTLCommandEncoder, MTLCommandQueue,
    MTLComputeCommandEncoder, MTLCreateSystemDefaultDevice, MTLDevice,
};

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {}

fn main() {
    let device = MTLCreateSystemDefaultDevice().expect("No device found");

    let command_queue = device.newCommandQueue().unwrap();
    let command_buffer = command_queue.commandBuffer().unwrap();

    let fence = device.newFence().unwrap();

    let blit_encoder = command_buffer.blitCommandEncoder().unwrap();
    blit_encoder.updateFence(&fence);
    blit_encoder.endEncoding();

    let compute_encoder = command_buffer.computeCommandEncoder().unwrap();
    compute_encoder.waitForFence(&fence);
    compute_encoder.endEncoding();

    command_buffer.commit();
    command_buffer.waitUntilCompleted();

    println!("Done");
}
