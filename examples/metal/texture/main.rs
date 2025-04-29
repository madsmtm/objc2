//! Renders a textured quad to a window and adjusts the GPU buffer that contains the viewport's
//! height and width whenever the window is resized.
//!
//! This is a port of the Objective-C/Swift [Creating and Sampling Textures] tutorial.
//!
//! [Creating and Sampling Textures]: https://developer.apple.com/documentation/metal/creating_and_sampling_textures

// Modified from <https://github.com/gfx-rs/metal-rs/tree/v0.33.0/examples/texture>

#![cfg_attr(not(target_os = "macos"), allow(dead_code, unused))]

use std::{
    cell::OnceCell,
    io::{self},
    ptr::NonNull,
};

use objc2::{
    define_class, msg_send, rc::Retained, runtime::ProtocolObject, Ivars, MainThreadMarker,
    MainThreadOnly,
};
#[cfg(target_os = "macos")]
use objc2_app_kit::{
    NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate, NSBackingStoreType,
    NSWindow, NSWindowStyleMask,
};
use objc2_core_foundation::CGSize;
use objc2_foundation::{
    ns_string, NSNotification, NSObject, NSObjectProtocol, NSPoint, NSRange, NSRect, NSSize,
};
use objc2_metal::{
    MTLBuffer, MTLClearColor, MTLCommandBuffer, MTLCommandEncoder, MTLCommandQueue,
    MTLCreateSystemDefaultDevice, MTLDevice, MTLLibrary, MTLLoadAction, MTLOrigin, MTLPixelFormat,
    MTLPrimitiveType, MTLRegion, MTLRenderCommandEncoder, MTLRenderPassDescriptor,
    MTLRenderPipelineDescriptor, MTLRenderPipelineState, MTLResourceOptions, MTLSize,
    MTLStoreAction, MTLTexture, MTLTextureDescriptor, MTLTextureSwizzle, MTLTextureSwizzleChannels,
};
#[cfg(target_os = "macos")]
use objc2_metal_kit::{MTKView, MTKViewDelegate};
use objc2_quartz_core::CAMetalDrawable;

/// The state of our renderer.
#[derive(Debug)]
struct Renderer {
    viewport_size_buffer: Retained<ProtocolObject<dyn MTLBuffer>>,
    texture_to_render: Retained<ProtocolObject<dyn MTLTexture>>,
    vertex_buffer: Retained<ProtocolObject<dyn MTLBuffer>>,
    pipeline_state: Retained<ProtocolObject<dyn MTLRenderPipelineState>>,
    command_queue: Retained<ProtocolObject<dyn MTLCommandQueue>>,
}

impl Renderer {
    fn new(device: &ProtocolObject<dyn MTLDevice>, view_pixel_format: MTLPixelFormat) -> Self {
        let library = device
            .newLibraryWithSource_options_error(ns_string!(include_str!("shaders.metal")), None)
            .unwrap_or_else(|e| panic!("{e}"));

        let viewport_size_buffer = device
            .newBufferWithLength_options(
                8,
                MTLResourceOptions::CPUCacheModeDefaultCache
                    | MTLResourceOptions::StorageModeManaged,
            )
            .unwrap();

        // Create texture to display.
        let texture_to_render = {
            let img = include_bytes!("../../../assets/logo.png");
            // let img = include_bytes!("../default_xcode_game/ColorMap.png");
            let decoder = png::Decoder::new(io::Cursor::new(img));
            let mut reader = decoder.read_info().unwrap();

            // Configure MTLTexture based on the input.
            let texture = MTLTextureDescriptor::new();
            let info = reader.info();
            let (width, height) = (info.width as usize, info.height as usize);
            unsafe { texture.setWidth(width) };
            unsafe { texture.setHeight(height) };
            match (info.bit_depth, info.color_type) {
                (png::BitDepth::Eight, png::ColorType::Rgba) => {
                    texture.setPixelFormat(MTLPixelFormat::RGBA8Unorm);
                }
                (png::BitDepth::Eight, png::ColorType::Grayscale) => {
                    texture.setPixelFormat(MTLPixelFormat::R8Unorm);
                    // Copy from the red channel to the green and blue channels.
                    texture.setSwizzle(MTLTextureSwizzleChannels {
                        red: MTLTextureSwizzle::Red,
                        green: MTLTextureSwizzle::Red,
                        blue: MTLTextureSwizzle::Red,
                        alpha: MTLTextureSwizzle::One,
                    });
                }
                _ => unimplemented!("unknown pixel format for: {info:#?}"),
            };
            let bytes_per_pixel = info.bytes_per_pixel();

            // Read png and write it into the MTLTexture.
            let texture = device.newTextureWithDescriptor(&texture).unwrap();

            let mut buf = vec![0; reader.output_buffer_size().unwrap()];
            reader.next_frame(&mut buf).unwrap();
            unsafe {
                texture.replaceRegion_mipmapLevel_withBytes_bytesPerRow(
                    MTLRegion {
                        origin: MTLOrigin { x: 0, y: 0, z: 0 },
                        size: MTLSize {
                            width,
                            height,
                            depth: 1,
                        },
                    },
                    0,
                    NonNull::new(buf.as_ptr().cast_mut().cast()).unwrap(),
                    width * bytes_per_pixel,
                )
            };

            texture
        };

        let aspect_ratio = (texture_to_render.width() / texture_to_render.height()) as f32;
        let quad_height = 200.0;
        let quad_width = quad_height * aspect_ratio;
        let vertex_data = [
            TexturedVertex::new([-quad_width, -quad_height], [0., 1.]),
            TexturedVertex::new([quad_width, -quad_height], [1., 1.]),
            TexturedVertex::new([quad_width, quad_height], [1., 0.]),
            TexturedVertex::new([-quad_width, -quad_height], [0., 1.]),
            TexturedVertex::new([quad_width, quad_height], [1., 0.]),
            TexturedVertex::new([-quad_width, quad_height], [0., 0.]),
        ];

        let vertex_buffer = unsafe {
            device.newBufferWithBytes_length_options(
                NonNull::new(vertex_data.as_ptr().cast_mut().cast()).unwrap(),
                size_of_val(&vertex_data),
                MTLResourceOptions::CPUCacheModeDefaultCache
                    | MTLResourceOptions::StorageModeManaged,
            )
        }
        .unwrap();

        let pipeline_state = {
            let pipeline_state_descriptor = MTLRenderPipelineDescriptor::new();

            let vertex_shader = library
                .newFunctionWithName(ns_string!("quad_vertex"))
                .unwrap();
            let fragment_shader = library
                .newFunctionWithName(ns_string!("sampling_shader"))
                .unwrap();

            pipeline_state_descriptor.setVertexFunction(Some(&vertex_shader));
            pipeline_state_descriptor.setFragmentFunction(Some(&fragment_shader));

            unsafe {
                pipeline_state_descriptor
                    .colorAttachments()
                    .objectAtIndexedSubscript(0)
            }
            .setPixelFormat(view_pixel_format);

            device
                .newRenderPipelineStateWithDescriptor_error(&pipeline_state_descriptor)
                .unwrap()
        };
        let command_queue = device.newCommandQueue().unwrap();

        Self {
            viewport_size_buffer,
            texture_to_render,
            vertex_buffer,
            command_queue,
            pipeline_state,
        }
    }

    fn draw(&self, drawable: &ProtocolObject<dyn CAMetalDrawable>) {
        let render_pass_descriptor = MTLRenderPassDescriptor::new();

        // Prepare render pass descriptor
        let color_attachment = unsafe {
            render_pass_descriptor
                .colorAttachments()
                .objectAtIndexedSubscript(0)
        };
        color_attachment.setTexture(Some(&drawable.texture()));
        color_attachment.setLoadAction(MTLLoadAction::Clear);
        color_attachment.setClearColor(MTLClearColor {
            red: 0.2,
            green: 0.5,
            blue: 0.8,
            alpha: 1.0,
        });
        color_attachment.setStoreAction(MTLStoreAction::Store);

        let command_buffer = self.command_queue.commandBuffer().unwrap();

        let encoder = command_buffer
            .renderCommandEncoderWithDescriptor(&render_pass_descriptor)
            .unwrap();
        encoder.setRenderPipelineState(&self.pipeline_state);

        unsafe {
            encoder.setVertexBuffer_offset_atIndex(Some(&self.vertex_buffer), 0, VerticesBufferIdx);
            encoder.setVertexBuffer_offset_atIndex(
                Some(&self.viewport_size_buffer),
                0,
                ViewportSizeBufferIdx,
            );
            encoder.setFragmentTexture_atIndex(Some(&self.texture_to_render), TextureBaseColorIdx);
        }

        unsafe { encoder.drawPrimitives_vertexStart_vertexCount(MTLPrimitiveType::Triangle, 0, 6) };
        encoder.endEncoding();

        command_buffer.presentDrawable(drawable.as_ref());
        command_buffer.commit();
    }

    fn resize(&self, size: CGSize) {
        let contents = self.viewport_size_buffer.contents().as_ptr();
        let viewport_size: [u32; 2] = [size.width as u32, size.height as u32];
        let byte_count = size_of_val(&viewport_size);

        unsafe {
            std::ptr::copy(viewport_size.as_ptr(), contents.cast(), byte_count);
        }
        self.viewport_size_buffer
            .didModifyRange(NSRange::new(0, byte_count));
    }
}

//
// Shader types. These must match what's in `shaders.metal`!
//

// HACK: Workaround for cross-architecture SIMD not being stable in Rust (yet).
#[allow(non_camel_case_types)]
pub type simd_float2 = u64;
#[allow(non_camel_case_types)]
pub type vector_float2 = simd_float2;
#[allow(non_upper_case_globals)]
pub const VerticesBufferIdx: VertexInputIndex = 0;
#[allow(non_upper_case_globals)]
pub const ViewportSizeBufferIdx: VertexInputIndex = 1;
pub type VertexInputIndex = usize;
#[allow(non_upper_case_globals)]
pub const TextureBaseColorIdx: TextureIndex = 0;
pub type TextureIndex = usize;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TexturedVertex {
    pub position: vector_float2,
    pub texture_coord: vector_float2,
}

impl TexturedVertex {
    fn new(position: [f32; 2], texture_coord: [f32; 2]) -> Self {
        // The metal shader is expecting two floats, but the rust-bindgen generated
        // type is a u64.
        //
        // So, we transmute the 2 floats into a u64 so that when the shader receives
        // these 64 bits they'll be interpreted as a `vector_float2`.
        Self {
            position: unsafe { std::mem::transmute::<[f32; 2], u64>(position) },
            texture_coord: unsafe { std::mem::transmute::<[f32; 2], u64>(texture_coord) },
        }
    }
}

//
// Boilerplate for setting up a MTKView in a window.
//

define_class!(
    /// The state of our application.
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    struct Delegate {
        renderer: OnceCell<Renderer>,
        #[cfg(target_os = "macos")]
        window: OnceCell<Retained<NSWindow>>,
    }

    unsafe impl NSObjectProtocol for Delegate {}

    // define the delegate methods for the `NSApplicationDelegate` protocol
    #[cfg(target_os = "macos")]
    unsafe impl NSApplicationDelegate for Delegate {
        #[unsafe(method(applicationDidFinishLaunching:))]
        #[allow(non_snake_case)]
        unsafe fn applicationDidFinishLaunching(&self, _notification: &NSNotification) {
            let mtm = self.mtm();
            // create the app window
            let window = {
                let content_rect = NSRect::new(NSPoint::new(0., 0.), NSSize::new(768., 768.));
                let style = NSWindowStyleMask::Closable
                    | NSWindowStyleMask::Resizable
                    | NSWindowStyleMask::Titled;
                let backing_store_type = NSBackingStoreType::Buffered;
                let flag = false;
                unsafe {
                    NSWindow::initWithContentRect_styleMask_backing_defer(
                        NSWindow::alloc(mtm),
                        content_rect,
                        style,
                        backing_store_type,
                        flag,
                    )
                }
            };

            // get the default device
            let device =
                MTLCreateSystemDefaultDevice().expect("failed to get default system device");

            // create the metal view
            let mtk_view = {
                let frame_rect = window.frame();
                MTKView::initWithFrame_device(MTKView::alloc(mtm), frame_rect, Some(&device))
            };

            // initialize the renderer
            let renderer = Renderer::new(&device, mtk_view.colorPixelFormat());
            self.renderer().set(renderer).unwrap();

            // configure the metal view delegate
            let object = ProtocolObject::from_ref(self);
            mtk_view.setDelegate(Some(object));

            // configure the window
            window.setContentView(Some(&mtk_view));
            window.center();
            window.setTitle(ns_string!("metal example"));
            window.makeKeyAndOrderFront(None);

            self.window().set(window).unwrap();
        }
    }

    // define the delegate methods for the `MTKViewDelegate` protocol
    #[cfg(target_os = "macos")] // TODO: Support iOS
    unsafe impl MTKViewDelegate for Delegate {
        #[unsafe(method(drawInMTKView:))]
        #[allow(non_snake_case)]
        unsafe fn drawInMTKView(&self, mtk_view: &MTKView) {
            let renderer = self.renderer().get().unwrap();
            let drawable = mtk_view.currentDrawable().unwrap();
            renderer.draw(&drawable);
        }

        #[unsafe(method(mtkView:drawableSizeWillChange:))]
        #[allow(non_snake_case)]
        unsafe fn mtkView_drawableSizeWillChange(&self, _view: &MTKView, size: NSSize) {
            let renderer = self.renderer().get().unwrap();
            renderer.resize(size);
        }
    }
);

impl Delegate {
    fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(mtm);
        let this = this.set_ivars(Ivars::<Self> {
            renderer: OnceCell::default(),
            #[cfg(target_os = "macos")]
            window: OnceCell::default(),
        });
        unsafe { msg_send![super(this), init] }
    }
}

#[cfg(target_os = "macos")]
fn main() {
    let mtm = MainThreadMarker::new().unwrap();
    // configure the app
    let app = NSApplication::sharedApplication(mtm);
    app.setActivationPolicy(NSApplicationActivationPolicy::Regular);

    // configure the application delegate
    let delegate = Delegate::new(mtm);
    let object = ProtocolObject::from_ref(&*delegate);
    app.setDelegate(Some(object));

    // run the app
    app.run();
}

#[cfg(not(target_os = "macos"))]
fn main() {
    panic!("This example is currently only supported on macOS");
}
