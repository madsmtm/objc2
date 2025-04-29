// Copyright 2017 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

// Modified from <https://github.com/gfx-rs/metal-rs/tree/v0.33.0/examples/caps>

use objc2_metal::{MTLCreateSystemDefaultDevice, MTLDevice};

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {}

fn main() {
    let device = MTLCreateSystemDefaultDevice().expect("no device found");

    #[cfg(feature = "unstable-private")]
    {
        println!("Vendor: {:?}", unsafe { device.vendorName() });
        println!("Family: {:?}", unsafe { device.familyName() });
    }
    println!(
        "Max threads per threadgroup: {:?}",
        device.maxThreadsPerThreadgroup()
    );
    #[cfg(target_os = "macos")]
    {
        println!("Integrated GPU: {:?}", device.isLowPower());
        println!("Headless: {:?}", device.isHeadless());
        println!(
            "D24S8: {:?}",
            device.isDepth24Stencil8PixelFormatSupported()
        );
    }
    println!("maxBufferLength: {} Mb", device.maxBufferLength() >> 20);
    println!(
        "Indirect argument buffer: {:?}",
        device.argumentBuffersSupport()
    );
}
