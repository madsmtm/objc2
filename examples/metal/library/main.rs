// Copyright 2016 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

// Modified from <https://github.com/gfx-rs/metal-rs/tree/v0.33.0/examples/library

use objc2_foundation::ns_string;
use objc2_metal::{MTLCompileOptions, MTLCreateSystemDefaultDevice, MTLDevice};

const PROGRAM: &str = "";

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {}

fn main() {
    let device = MTLCreateSystemDefaultDevice().expect("no device found");

    let options = MTLCompileOptions::new();
    let _library = device
        .newLibraryWithSource_options_error(ns_string!(PROGRAM), Some(&options))
        .unwrap_or_else(|e| panic!("{e}"));
}
