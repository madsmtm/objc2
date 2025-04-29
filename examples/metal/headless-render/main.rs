// Modified from <https://github.com/gfx-rs/metal-rs/tree/v0.33.0/examples/headless-render>
use std::io::BufWriter;
use std::path::PathBuf;
use std::{fs::File, ptr::NonNull};

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::ns_string;
use objc2_metal::{
    MTLBlitCommandEncoder, MTLBuffer, MTLClearColor, MTLCommandBuffer, MTLCommandEncoder,
    MTLCommandQueue, MTLCreateSystemDefaultDevice, MTLDevice, MTLLibrary, MTLLoadAction, MTLOrigin,
    MTLPixelFormat, MTLPrimitiveType, MTLRegion, MTLRenderCommandEncoder, MTLRenderPassDescriptor,
    MTLRenderPipelineDescriptor, MTLRenderPipelineState, MTLResourceOptions, MTLSize,
    MTLStoreAction, MTLTexture, MTLTextureDescriptor, MTLTextureUsage,
};

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {}

const VIEW_WIDTH: usize = 512;
const VIEW_HEIGHT: usize = 512;
const TOTAL_BYTES: usize = VIEW_WIDTH * VIEW_HEIGHT * 4;

const VERTEX_SHADER: &str = "triangle_vertex";
const FRAGMENT_SHADER: &str = "triangle_fragment";

// [2 bytes position, 3 bytes color] * 3
#[rustfmt::skip]
const VERTEX_ATTRIBS: [f32; 15] = [
    0.0, 0.5, 1.0, 0.0, 0.0,
    -0.5, -0.5, 0.0, 1.0, 0.0,
    0.5, -0.5, 0.0, 0.0, 1.0,
];

/// This example shows how to render headlessly by:
///
/// 1. Rendering a triangle to an MTLDrawable
///
/// 2. Waiting for the render to complete and the color texture to be synchronized with the CPU
///    by using a blit command encoder
///
/// 3. Reading the texture bytes from the MTLTexture
///
/// 4. Saving the texture to a PNG file
fn main() {
    let device = MTLCreateSystemDefaultDevice().expect("No device found");

    let texture = create_texture(&device);

    let library = device
        .newLibraryWithSource_options_error(ns_string!(include_str!("shaders.metal")), None)
        .unwrap_or_else(|e| panic!("{e}"));

    let pipeline_state = prepare_pipeline_state(&device, &library);

    let command_queue = device.newCommandQueue().unwrap();

    let vertex_buffer = create_vertex_buffer(&device);

    let render_pass_descriptor = MTLRenderPassDescriptor::new();
    initialize_color_attachment(&render_pass_descriptor, &texture);

    let command_buffer = command_queue.commandBuffer().unwrap();
    let rc_encoder = command_buffer
        .renderCommandEncoderWithDescriptor(&render_pass_descriptor)
        .unwrap();
    rc_encoder.setRenderPipelineState(&pipeline_state);
    unsafe { rc_encoder.setVertexBuffer_offset_atIndex(Some(&vertex_buffer), 0, 0) };
    unsafe { rc_encoder.drawPrimitives_vertexStart_vertexCount(MTLPrimitiveType::Triangle, 0, 3) };
    rc_encoder.endEncoding();

    unsafe {
        render_pass_descriptor
            .colorAttachments()
            .objectAtIndexedSubscript(0)
    }
    .setLoadAction(MTLLoadAction::DontCare);

    let blit_encoder = command_buffer.blitCommandEncoder().unwrap();
    blit_encoder.synchronizeResource(ProtocolObject::from_ref(&*texture));
    blit_encoder.endEncoding();

    command_buffer.commit();

    command_buffer.waitUntilCompleted();

    save_image(&texture);
}

fn save_image(texture: &ProtocolObject<dyn MTLTexture>) {
    let mut image = vec![0; TOTAL_BYTES];

    unsafe {
        texture.getBytes_bytesPerRow_fromRegion_mipmapLevel(
            NonNull::new(image.as_mut_ptr().cast()).unwrap(),
            VIEW_WIDTH * 4,
            MTLRegion {
                origin: MTLOrigin { x: 0, y: 0, z: 0 },
                size: MTLSize {
                    width: VIEW_WIDTH,
                    height: VIEW_HEIGHT,
                    depth: 1,
                },
            },
            0,
        )
    };

    let out_file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("headless-render/out.png");
    let file = File::create(&out_file).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, VIEW_WIDTH as u32, VIEW_HEIGHT as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&image).unwrap();

    println!("Image saved to {:?}", out_file);
}

fn create_texture(
    device: &ProtocolObject<dyn MTLDevice>,
) -> Retained<ProtocolObject<dyn MTLTexture>> {
    let texture = MTLTextureDescriptor::new();
    unsafe { texture.setWidth(VIEW_WIDTH) };
    unsafe { texture.setHeight(VIEW_HEIGHT) };
    texture.setPixelFormat(MTLPixelFormat::RGBA8Unorm);
    texture.setUsage(MTLTextureUsage::RenderTarget);

    device.newTextureWithDescriptor(&texture).unwrap()
}

fn prepare_pipeline_state(
    device: &ProtocolObject<dyn MTLDevice>,
    library: &ProtocolObject<dyn MTLLibrary>,
) -> Retained<ProtocolObject<dyn MTLRenderPipelineState>> {
    let vert = library
        .newFunctionWithName(ns_string!(VERTEX_SHADER))
        .unwrap();
    let frag = library
        .newFunctionWithName(ns_string!(FRAGMENT_SHADER))
        .unwrap();

    let pipeline_state_descriptor = MTLRenderPipelineDescriptor::new();

    pipeline_state_descriptor.setVertexFunction(Some(&vert));
    pipeline_state_descriptor.setFragmentFunction(Some(&frag));

    unsafe {
        pipeline_state_descriptor
            .colorAttachments()
            .objectAtIndexedSubscript(0)
    }
    .setPixelFormat(MTLPixelFormat::RGBA8Unorm);

    device
        .newRenderPipelineStateWithDescriptor_error(&pipeline_state_descriptor)
        .unwrap()
}

fn create_vertex_buffer(
    device: &ProtocolObject<dyn MTLDevice>,
) -> Retained<ProtocolObject<dyn MTLBuffer>> {
    unsafe {
        device.newBufferWithBytes_length_options(
            NonNull::new(VERTEX_ATTRIBS.as_ptr().cast_mut().cast()).unwrap(),
            size_of_val(&VERTEX_ATTRIBS),
            MTLResourceOptions::CPUCacheModeDefaultCache | MTLResourceOptions::StorageModeManaged,
        )
    }
    .unwrap()
}

fn initialize_color_attachment(
    descriptor: &MTLRenderPassDescriptor,
    texture: &ProtocolObject<dyn MTLTexture>,
) {
    let color_attachment = unsafe { descriptor.colorAttachments().objectAtIndexedSubscript(0) };

    color_attachment.setTexture(Some(texture));
    color_attachment.setLoadAction(MTLLoadAction::Clear);
    color_attachment.setClearColor(MTLClearColor {
        red: 0.5,
        green: 0.2,
        blue: 0.2,
        alpha: 1.0,
    });
    color_attachment.setStoreAction(MTLStoreAction::Store);
}
