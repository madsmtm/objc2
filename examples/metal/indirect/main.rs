// Copyright 2017 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate metal_rs as metal;
extern crate cocoa;
#[macro_use] extern crate objc;
extern crate objc_id;
extern crate objc_foundation;

use metal::*;

use cocoa::foundation::NSAutoreleasePool;
use objc_foundation::{NSArray, INSArray};
use objc_id::Id;

fn main() {
    let mut pool = unsafe { NSAutoreleasePool::new(cocoa::base::nil) };

    let device = Device::system_default();

    let desc1 = ArgumentDescriptor::new();
    desc1.set_index(1);
    desc1.set_data_type(MTLDataType::Sampler);
    let desc2 = ArgumentDescriptor::new();
    desc2.set_data_type(MTLDataType::Texture);

    let encoder = device.new_argument_encoder(&Array::from_slice(&[desc1, desc2]));
    println!("{:?}", encoder);

    let buffer = device.new_buffer(encoder.encoded_length(), MTLResourceOptions::empty());
    encoder.set_argument_buffer(&buffer, 0);

    let sampler = {
        let descriptor = SamplerDescriptor::new();
        device.new_sampler(&descriptor)
    };
    encoder.set_sampler_states(&[&sampler], 0);
    println!("{:?}", sampler);

    unsafe {
        msg_send![pool, release];
    }
}
