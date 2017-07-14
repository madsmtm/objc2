// Copyright 2017 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate metal_rs as metal;

use metal::*;

fn main() {
    let device = create_system_default_device();

    let desc1 = MTLArgumentDescriptor::new();
    desc1.set_index(1);
    desc1.set_data_type(MTLDataType::Sampler);
    let desc2 = MTLArgumentDescriptor::new();
    desc2.set_data_type(MTLDataType::Texture);

    let arguments = NSArray::array_with_objects(&[desc1, desc2]);
    let encoder = device.new_argument_encoder(arguments);
    println!("{:?}", encoder);

    let buffer = device.new_buffer(encoder.encoded_length(), MTLResourceOptions::empty());
    encoder.set_argument_buffer(buffer, 0);

    let sampler = {
        let descriptor = MTLSamplerDescriptor::new();
        device.new_sampler(descriptor)
    };
    encoder.set_sampler_states(&[sampler], 0);
    println!("{:?}", sampler);
}
