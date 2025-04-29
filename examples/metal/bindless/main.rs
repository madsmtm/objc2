// Copyright 2017 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

// Modified from <https://github.com/gfx-rs/metal-rs/tree/v0.33.0/examples/bindless>

use objc2::rc::autoreleasepool;
use objc2_foundation::{NSArray, NSUInteger};
#[allow(deprecated)]
use objc2_metal::MTLArgumentAccess;
use objc2_metal::{
    MTLArgumentBuffersTier, MTLArgumentDescriptor, MTLArgumentEncoder, MTLCommandBuffer,
    MTLCommandEncoder, MTLCommandQueue, MTLCreateSystemDefaultDevice, MTLDataType, MTLDevice,
    MTLHeap, MTLHeapDescriptor, MTLPixelFormat, MTLRenderCommandEncoder, MTLRenderPassDescriptor,
    MTLRenderStages, MTLResourceOptions, MTLStorageMode, MTLTextureDescriptor, MTLTextureType,
};

const BINDLESS_TEXTURE_COUNT: NSUInteger = 100_000; // ~25Mb

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {}

/// This example demonstrates:
/// - How to create a heap
/// - How to allocate textures from heap.
/// - How to create bindless resources via Metal's argument buffers.
/// - How to bind argument buffer to render encoder
fn main() {
    autoreleasepool(|_| {
        let device = MTLCreateSystemDefaultDevice().expect("no device found");

        /*

        MSL

        struct Textures {
            texture2d<float> texture;
        };
        struct BindlessTextures {
            device Textures *textures;
        };

         */

        // Tier 2 argument buffers are supported by macOS devices with a discrete GPU and by the A13 GPU.
        // The maximum per-app resources available at any given time are:
        // - 500,000 buffers or textures
        // - 2048 unique samplers
        let tier = device.argumentBuffersSupport();
        println!("Argument buffer support: {:?}", tier);
        assert_eq!(MTLArgumentBuffersTier::Tier2, tier);

        let texture_descriptor = MTLTextureDescriptor::new();
        unsafe { texture_descriptor.setWidth(1) };
        unsafe { texture_descriptor.setHeight(1) };
        unsafe { texture_descriptor.setDepth(1) };
        texture_descriptor.setTextureType(MTLTextureType::Type2D);
        texture_descriptor.setPixelFormat(MTLPixelFormat::R8Uint);
        texture_descriptor.setStorageMode(MTLStorageMode::Private); // GPU only.
        println!("Texture descriptor: {:?}", texture_descriptor);

        // Determine the size required for the heap for the given descriptor
        let size_and_align = device.heapTextureSizeAndAlignWithDescriptor(&texture_descriptor);

        // Align the size so that more resources will fit in the heap after this texture
        // See https://developer.apple.com/documentation/metal/buffers/using_argument_buffers_with_resource_heaps
        let texture_size =
            (size_and_align.size & (size_and_align.align - 1)) + size_and_align.align;
        let heap_size = texture_size * BINDLESS_TEXTURE_COUNT;

        let heap_descriptor = MTLHeapDescriptor::new();
        heap_descriptor.setStorageMode(texture_descriptor.storageMode()); // Must be compatible
        heap_descriptor.setSize(heap_size);
        println!("Heap descriptor: {:?}", heap_descriptor);

        let heap = device.newHeapWithDescriptor(&heap_descriptor).unwrap();
        println!("Heap: {:?}", heap);

        // Allocate textures from heap
        let textures = (0..BINDLESS_TEXTURE_COUNT)
            .map(|i| {
                heap.newTextureWithDescriptor(&texture_descriptor)
                    .unwrap_or_else(|| panic!("Failed to allocate texture {}", i))
            })
            .collect::<Vec<_>>();

        // Crate argument encoder that knows how to encode single texture
        let descriptor = MTLArgumentDescriptor::new();
        descriptor.setIndex(0);
        descriptor.setDataType(MTLDataType::Texture);
        descriptor.setTextureType(MTLTextureType::Type2D);
        #[allow(deprecated)]
        descriptor.setAccess(MTLArgumentAccess::ReadOnly);
        println!("Argument descriptor: {:?}", descriptor);

        let encoder = device
            .newArgumentEncoderWithArguments(&NSArray::from_retained_slice(&[descriptor]))
            .unwrap();
        println!("Encoder: {:?}", encoder);

        // Determinate argument buffer size to allocate.
        // Size needed to encode one texture * total number of bindless textures.
        let argument_buffer_size = encoder.encodedLength() * BINDLESS_TEXTURE_COUNT;
        let argument_buffer = device
            .newBufferWithLength_options(argument_buffer_size, MTLResourceOptions::empty())
            .unwrap();

        // Encode textures to the argument buffer.
        textures.iter().enumerate().for_each(|(index, texture)| {
            // Offset encoder to a proper texture slot
            let offset = index * encoder.encodedLength();
            unsafe { encoder.setArgumentBuffer_offset(Some(&argument_buffer), offset) };
            unsafe { encoder.setTexture_atIndex(Some(texture), 0) };
        });

        // How to use bindless argument buffer when drawing

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

        // Bind argument buffer.
        unsafe { encoder.setFragmentBuffer_offset_atIndex(Some(&argument_buffer), 0, 0) };
        // Make sure all textures are available to the pass.
        encoder.useHeap_stages(&heap, MTLRenderStages::Fragment);

        // Bind material buffer at index 1
        // Draw

        /*

        // Now instead of binding individual textures each draw call,
        // you can just bind material information instead:

        MSL

        struct Material {
            int diffuse_texture_index;
            int normal_texture_index;
            // ...
        }

        fragment float4 pixel(
            VertexOut v [[stage_in]],
            constant const BindlessTextures * textures [[buffer(0)]],
            constant Material * material [[buffer(1)]]
        ) {
            if (material->base_color_texture_index != -1) {
                textures[material->diffuse_texture_index].texture.sampler(...)
            }
            if (material->normal_texture_index != -1) {
                ...
            }
            ...
        }

         */

        encoder.endEncoding();
        command_buffer.commit();
    });
}
