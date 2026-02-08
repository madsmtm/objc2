//! Renders a circle in a window.
//!
//! As metal primitive types are only limited to point, line and triangle
//! shape, this example shows how we can form complex structures out of
//! primitive types.

// Modified from <https://github.com/gfx-rs/metal-rs/tree/v0.33.0/examples/circle>
#![cfg_attr(not(target_os = "macos"), allow(dead_code, unused))]

use std::{cell::OnceCell, ptr::NonNull};

use objc2::{
    define_class, msg_send, rc::Retained, runtime::ProtocolObject, Ivars, MainThreadMarker,
    MainThreadOnly, Message,
};
#[cfg(target_os = "macos")]
use objc2_app_kit::{
    NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate, NSBackingStoreType,
    NSWindow, NSWindowStyleMask,
};
use objc2_foundation::{
    ns_string, NSNotification, NSObject, NSObjectProtocol, NSPoint, NSRange, NSRect, NSSize,
};
use objc2_metal::{
    MTLBlitCommandEncoder, MTLBuffer, MTLClearColor, MTLCommandBuffer, MTLCommandEncoder,
    MTLCommandQueue, MTLCounterSampleBuffer, MTLCounterSampleBufferDescriptor,
    MTLCounterSamplingPoint, MTLCounterSet, MTLCreateSystemDefaultDevice, MTLDevice, MTLLibrary,
    MTLLoadAction, MTLPixelFormat, MTLPrimitiveType, MTLRenderCommandEncoder,
    MTLRenderPassDescriptor, MTLRenderPipelineDescriptor, MTLRenderPipelineState,
    MTLResourceOptions, MTLStorageMode, MTLStoreAction, MTLTimestamp,
};
#[cfg(target_os = "macos")]
use objc2_metal_kit::{MTKView, MTKViewDelegate};
use objc2_quartz_core::CAMetalDrawable;

// Declare the data structures needed to carry vertex layout to
// metal shading language(MSL) program. Use #[repr(C)], to make
// the data structure compatible with C++ type data structure
// for vertex defined in MSL program as MSL program is broadly
// based on C++
#[repr(C)]
#[derive(Debug)]
pub struct Position(std::ffi::c_float, std::ffi::c_float);
#[repr(C)]
#[derive(Debug)]
pub struct Color(std::ffi::c_float, std::ffi::c_float, std::ffi::c_float);
#[repr(C)]
#[derive(Debug)]
pub struct AAPLVertex {
    p: Position,
    c: Color,
}

/// The state of our renderer.
#[derive(Debug)]
struct Renderer {
    pipeline_state: Retained<ProtocolObject<dyn MTLRenderPipelineState>>,
    command_queue: Retained<ProtocolObject<dyn MTLCommandQueue>>,
    vbuf: Retained<ProtocolObject<dyn MTLBuffer>>,
    sampler: TimeSampler,
}

impl Renderer {
    fn new(device: &ProtocolObject<dyn MTLDevice>, pixel_format: MTLPixelFormat) -> Self {
        let sampler = TimeSampler::new(device);

        let library = device
            .newLibraryWithSource_options_error(ns_string!(include_str!("shaders.metal")), None)
            .unwrap_or_else(|e| panic!("{e}"));

        // The render pipeline generated from the vertex and fragment shaders in the .metal shader file.
        let pipeline_state = {
            let vert = library.newFunctionWithName(ns_string!("vs")).unwrap();
            let frag = library.newFunctionWithName(ns_string!("ps")).unwrap();

            let pipeline_state_descriptor = MTLRenderPipelineDescriptor::new();
            pipeline_state_descriptor.setVertexFunction(Some(&vert));
            pipeline_state_descriptor.setFragmentFunction(Some(&frag));
            let color_attachment = unsafe {
                pipeline_state_descriptor
                    .colorAttachments()
                    .objectAtIndexedSubscript(0)
            };
            color_attachment.setPixelFormat(pixel_format);

            device
                .newRenderPipelineStateWithDescriptor_error(&pipeline_state_descriptor)
                .unwrap()
        };

        // Set the command queue used to pass commands to the device.
        let command_queue = device.newCommandQueue().unwrap();

        let vbuf = {
            let vertex_data = create_vertex_points_for_circle();

            unsafe {
                device.newBufferWithBytes_length_options(
                    NonNull::new(vertex_data.as_ptr().cast_mut().cast()).unwrap(),
                    size_of_val(vertex_data.as_slice()),
                    MTLResourceOptions::CPUCacheModeDefaultCache
                        | MTLResourceOptions::StorageModeManaged,
                )
            }
            .unwrap()
        };

        Self {
            pipeline_state,
            command_queue,
            vbuf,
            sampler,
        }
    }

    fn draw(&self, drawable: &ProtocolObject<dyn CAMetalDrawable>) {
        // Create a new command buffer for each render pass to the current drawable
        let command_buffer = self.command_queue.commandBuffer().unwrap();

        // Obtain a renderPassDescriptor generated from the view's drawable textures.
        let render_pass_descriptor = MTLRenderPassDescriptor::new();

        // Set up the color attachment.
        let color_attachment = unsafe {
            render_pass_descriptor
                .colorAttachments()
                .objectAtIndexedSubscript(0)
        };
        color_attachment.setTexture(Some(&drawable.texture()));
        color_attachment.setLoadAction(MTLLoadAction::Clear);
        // Setting a background color
        color_attachment.setClearColor(MTLClearColor {
            red: 0.5,
            green: 0.5,
            blue: 0.8,
            alpha: 1.0,
        });
        color_attachment.setStoreAction(MTLStoreAction::Store);

        self.sampler.add_to_render_pass(&render_pass_descriptor);

        // Create a render command encoder.
        let encoder = command_buffer
            .renderCommandEncoderWithDescriptor(&render_pass_descriptor)
            .unwrap();
        encoder.setRenderPipelineState(&self.pipeline_state);
        // Pass in the parameter data.
        unsafe { encoder.setVertexBuffer_offset_atIndex(Some(&self.vbuf), 0, 0) };
        // Draw the triangles which will eventually form the circle.
        unsafe {
            encoder.drawPrimitives_vertexStart_vertexCount(MTLPrimitiveType::TriangleStrip, 0, 1080)
        };
        encoder.endEncoding();

        self.sampler.resolve(&command_buffer);

        // Schedule a present once the framebuffer is complete using the current drawable.
        command_buffer.presentDrawable(drawable.as_ref());

        // Finalize rendering here & push the command buffer to the GPU.
        command_buffer.commit();
        command_buffer.waitUntilCompleted();

        self.sampler.print_timings();
    }
}

// If we want to draw a circle, we need to draw it out of the three primitive
// types available with metal framework. Triangle is used in this case to form
// the circle. If we consider a circle to be total of 360 degree at center, we
// can form small triangle with one point at origin and two points at the
// perimeter of the circle for each degree. Eventually, if we can take enough
// triangle virtices for total of 360 degree, the triangles together will
// form a circle. This function captures the triangle vertices for each degree
// and push the co-ordinates of the vertices to a rust vector
fn create_vertex_points_for_circle() -> Vec<AAPLVertex> {
    let mut v: Vec<AAPLVertex> = Vec::new();
    let origin_x: f32 = 0.0;
    let origin_y: f32 = 0.0;

    // Size of the circle
    let circle_size = 0.8f32;

    for i in 0..720 {
        let y = i as f32;
        // Get the X co-ordinate of each point on the perimeter of circle
        let position_x: f32 = y.to_radians().cos() * 100.0;
        let position_x: f32 = position_x.trunc() / 100.0;
        // Set the size of the circle
        let position_x: f32 = position_x * circle_size;
        // Get the Y co-ordinate of each point on the perimeter of circle
        let position_y: f32 = y.to_radians().sin() * 100.0;
        let position_y: f32 = position_y.trunc() / 100.0;
        // Set the size of the circle
        let position_y: f32 = position_y * circle_size;

        v.push(AAPLVertex {
            p: Position(position_x, position_y),
            c: Color(0.7, 0.3, 0.5),
        });

        if (i + 1) % 2 == 0 {
            // For each two points on perimeter, push one point of origin
            v.push(AAPLVertex {
                p: Position(origin_x, origin_y),
                c: Color(0.2, 0.7, 0.4),
            });
        }
    }

    v
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
        unsafe fn mtkView_drawableSizeWillChange(&self, _view: &MTKView, _size: NSSize) {
            // println!("mtkView_drawableSizeWillChange");
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

//
// Boilerplate for setting up CPU and GPU timestamp sampling.
//

#[derive(Debug)]
struct TimeSampler {
    device: Retained<ProtocolObject<dyn MTLDevice>>,
    cpu_start: MTLTimestamp,
    gpu_start: MTLTimestamp,
    counter_sample_buffer: Retained<ProtocolObject<dyn MTLCounterSampleBuffer>>,
    destination_buffer: Retained<ProtocolObject<dyn MTLBuffer>>,
}

impl TimeSampler {
    fn new(device: &ProtocolObject<dyn MTLDevice>) -> Self {
        let mut cpu_start = 0;
        let mut gpu_start = 0;
        device.sampleTimestamps_gpuTimestamp(&mut cpu_start, &mut gpu_start);

        let counter_sample_buffer_desc = MTLCounterSampleBufferDescriptor::new();
        counter_sample_buffer_desc.setStorageMode(MTLStorageMode::Shared);
        unsafe { counter_sample_buffer_desc.setSampleCount(4) };
        let counter_sets = device.counterSets().unwrap();
        let timestamp_counter = counter_sets
            .iter()
            .find(|cs| *cs.name() == *ns_string!("timestamp"))
            .expect("No timestamp counter found");
        counter_sample_buffer_desc.setCounterSet(Some(&timestamp_counter));

        let counter_sample_buffer = device
            .newCounterSampleBufferWithDescriptor_error(&counter_sample_buffer_desc)
            .unwrap();

        let destination_buffer = device
            .newBufferWithLength_options(
                size_of::<u64>() * 4,
                MTLResourceOptions::StorageModeShared,
            )
            .unwrap();
        let counter_sampling_point = MTLCounterSamplingPoint::AtStageBoundary;
        assert!(device.supportsCounterSampling(counter_sampling_point));

        Self {
            device: device.retain(),
            cpu_start,
            gpu_start,
            counter_sample_buffer,
            destination_buffer,
        }
    }

    fn add_to_render_pass(&self, render_pass_descriptor: &MTLRenderPassDescriptor) {
        let sample_buffer_attachment = unsafe {
            render_pass_descriptor
                .sampleBufferAttachments()
                .objectAtIndexedSubscript(0)
        };
        sample_buffer_attachment.setSampleBuffer(Some(&self.counter_sample_buffer));
        unsafe {
            sample_buffer_attachment.setStartOfVertexSampleIndex(0);
            sample_buffer_attachment.setEndOfVertexSampleIndex(1);
            sample_buffer_attachment.setStartOfFragmentSampleIndex(2);
            sample_buffer_attachment.setEndOfFragmentSampleIndex(3);
        }
    }

    /// Resolve samples into buffer.
    fn resolve(&self, command_buffer: &ProtocolObject<dyn MTLCommandBuffer>) {
        let blit_encoder = command_buffer.blitCommandEncoder().unwrap();
        unsafe {
            blit_encoder.resolveCounters_inRange_destinationBuffer_destinationOffset(
                &self.counter_sample_buffer,
                NSRange::new(0, 4),
                &self.destination_buffer,
                0,
            )
        };
        blit_encoder.endEncoding();
    }

    fn print_timings(&self) {
        let mut cpu_end = 0;
        let mut gpu_end = 0;
        self.device
            .sampleTimestamps_gpuTimestamp(&mut cpu_end, &mut gpu_end);

        let samples = unsafe {
            std::slice::from_raw_parts(self.destination_buffer.contents().as_ptr().cast::<u64>(), 4)
        };
        let vertex_pass_start = samples[0];
        let vertex_pass_end = samples[1];
        let fragment_pass_start = samples[2];
        let fragment_pass_end = samples[3];

        let cpu_time_span = cpu_end - self.cpu_start;
        let gpu_time_span = gpu_end - self.gpu_start;

        let vertex_micros = microseconds_between_begin(
            vertex_pass_start,
            vertex_pass_end,
            gpu_time_span,
            cpu_time_span,
        );
        let fragment_micros = microseconds_between_begin(
            fragment_pass_start,
            fragment_pass_end,
            gpu_time_span,
            cpu_time_span,
        );

        println!("Vertex pass duration:   {:.2} µs", vertex_micros);
        println!("Fragment pass duration: {:.2} µs\n", fragment_micros);
    }
}

/// <https://developer.apple.com/documentation/metal/gpu_counters_and_counter_sample_buffers/converting_gpu_timestamps_into_cpu_time>
fn microseconds_between_begin(begin: u64, end: u64, gpu_time_span: u64, cpu_time_span: u64) -> f64 {
    let time_span = (end as f64) - (begin as f64);
    let nanoseconds = time_span / (gpu_time_span as f64) * (cpu_time_span as f64);
    nanoseconds / 1000.0
}
