// Copyright 2016 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

// Modified from <https://github.com/gfx-rs/metal-rs/tree/v0.33.0/examples/reflection>

use objc2::rc::autoreleasepool;
use objc2_foundation::ns_string;
use objc2_metal::{
    MTLCompileOptions, MTLCreateSystemDefaultDevice, MTLDevice, MTLLibrary, MTLPipelineOption,
    MTLRenderPipelineDescriptor, MTLVertexDescriptor,
};

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {}

const PROGRAM: &str = r"
    #include <metal_stdlib>

    using namespace metal;

    typedef struct {
        float2 position;
        float3 color;
    } vertex_t;

    struct ColorInOut {
        float4 position [[position]];
        float4 color;
    };

    vertex ColorInOut vs(const device vertex_t* vertex_array [[ buffer(0) ]],
                                      unsigned int vid [[ vertex_id ]])
    {
        ColorInOut out;

        out.position = float4(float2(vertex_array[vid].position), 0.0, 1.0);
        out.color = float4(float3(vertex_array[vid].color), 1.0);

        return out;
    }

    fragment float4 ps(ColorInOut in [[stage_in]])
    {
        return in.color;
    };
";

fn main() {
    autoreleasepool(|_| {
        let device = MTLCreateSystemDefaultDevice().expect("no device found");

        let options = MTLCompileOptions::new();
        let library = device
            .newLibraryWithSource_options_error(ns_string!(PROGRAM), Some(&options))
            .unwrap_or_else(|e| panic!("{e}"));
        let (vs, ps) = (
            library.newFunctionWithName(ns_string!("vs")).unwrap(),
            library.newFunctionWithName(ns_string!("ps")).unwrap(),
        );

        let vertex_desc = MTLVertexDescriptor::new();

        let desc = MTLRenderPipelineDescriptor::new();
        desc.setVertexFunction(Some(&vs));
        desc.setFragmentFunction(Some(&ps));
        desc.setVertexDescriptor(Some(&vertex_desc));

        println!("{:?}", desc);

        #[allow(deprecated)]
        let reflect_options = MTLPipelineOption::ArgumentInfo | MTLPipelineOption::BufferTypeInfo;
        let mut reflection = None;
        let _ = device
            .newRenderPipelineStateWithDescriptor_options_reflection_error(
                &desc,
                reflect_options,
                Some(&mut reflection),
            )
            .unwrap();

        let reflection = reflection.unwrap();

        println!("Vertex arguments: ");
        #[allow(deprecated)]
        let vertex_arguments = reflection.vertexArguments().unwrap();
        for argument in vertex_arguments {
            println!("{:?}", argument);
        }
    });
}
