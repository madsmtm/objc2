//! Drawing a rotating triangle.
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(clippy::incompatible_msrv)]
#![cfg_attr(not(target_os = "macos"), allow(dead_code, unused))]
#![cfg_attr(feature = "unstable-darwin-objc", feature(darwin_objc))]

use core::{cell::OnceCell, ptr::NonNull};

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{define_class, msg_send, Ivars, MainThreadMarker, MainThreadOnly};
#[cfg(target_os = "macos")]
use objc2_app_kit::{
    NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate, NSBackingStoreType,
    NSWindow, NSWindowStyleMask,
};
use objc2_foundation::{
    ns_string, NSDate, NSNotification, NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize,
};
use objc2_metal::{
    MTLCommandBuffer, MTLCommandEncoder, MTLCommandQueue, MTLCreateSystemDefaultDevice, MTLDevice,
    MTLLibrary, MTLLoadAction, MTLPackedFloat3, MTLPixelFormat, MTLPrimitiveType,
    MTLRenderCommandEncoder, MTLRenderPassDescriptor, MTLRenderPipelineDescriptor,
    MTLRenderPipelineState, MTLStoreAction,
};
#[cfg(target_os = "macos")]
use objc2_metal_kit::{MTKView, MTKViewDelegate};
use objc2_quartz_core::CAMetalDrawable;

#[derive(Copy, Clone)]
#[repr(C)]
struct SceneProperties {
    time: f32,
}

#[derive(Copy, Clone)]
#[repr(C)]
struct VertexInput {
    position: MTLPackedFloat3,
    color: MTLPackedFloat3,
}

/// The state of our renderer.
#[derive(Debug)]
struct Renderer {
    start_date: Retained<NSDate>,
    command_queue: Retained<ProtocolObject<dyn MTLCommandQueue>>,
    pipeline_state: Retained<ProtocolObject<dyn MTLRenderPipelineState>>,
}

impl Renderer {
    fn new(device: &ProtocolObject<dyn MTLDevice>, pixel_format: MTLPixelFormat) -> Self {
        // create the command queue
        let command_queue = device
            .newCommandQueue()
            .expect("Failed to create a command queue.");

        // create the pipeline descriptor
        let pipeline_descriptor = MTLRenderPipelineDescriptor::new();

        unsafe {
            pipeline_descriptor
                .colorAttachments()
                .objectAtIndexedSubscript(0)
                .setPixelFormat(pixel_format);
        }

        // compile the shaders
        let library = device
            .newLibraryWithSource_options_error(ns_string!(include_str!("triangle.metal")), None)
            .unwrap_or_else(|e| panic!("Failed to create a library: {e}"));

        // configure the vertex shader
        let vertex_function = library.newFunctionWithName(ns_string!("vertex_main"));
        pipeline_descriptor.setVertexFunction(vertex_function.as_deref());

        // configure the fragment shader
        let fragment_function = library.newFunctionWithName(ns_string!("fragment_main"));
        pipeline_descriptor.setFragmentFunction(fragment_function.as_deref());

        // create the pipeline state
        let pipeline_state = device
            .newRenderPipelineStateWithDescriptor_error(&pipeline_descriptor)
            .expect("Failed to create a pipeline state.");

        let start_date = NSDate::now();

        Self {
            start_date,
            command_queue,
            pipeline_state,
        }
    }

    fn draw(&self, drawable: &ProtocolObject<dyn CAMetalDrawable>) {
        let command_buffer = self.command_queue.commandBuffer().unwrap();

        let pass_descriptor = MTLRenderPassDescriptor::new();
        let color_attachment = unsafe {
            pass_descriptor
                .colorAttachments()
                .objectAtIndexedSubscript(0)
        };

        color_attachment.setTexture(Some(&drawable.texture()));
        color_attachment.setLoadAction(MTLLoadAction::Clear);
        color_attachment.setStoreAction(MTLStoreAction::Store);
        let encoder = command_buffer
            .renderCommandEncoderWithDescriptor(&pass_descriptor)
            .unwrap();

        // compute the scene properties
        let scene_properties_data = &SceneProperties {
            time: self.start_date.timeIntervalSinceNow() as f32,
        };
        // write the scene properties to the vertex shader argument buffer at index 0
        let scene_properties_bytes = NonNull::from(scene_properties_data);
        unsafe {
            encoder.setVertexBytes_length_atIndex(
                scene_properties_bytes.cast::<core::ffi::c_void>(),
                core::mem::size_of_val(scene_properties_data),
                0,
            )
        };

        // compute the triangle geometry
        let vertex_input_data: &[VertexInput] = &[
            VertexInput {
                position: MTLPackedFloat3 {
                    x: -f32::sqrt(3.0) / 4.0,
                    y: -0.25,
                    z: 0.,
                },
                color: MTLPackedFloat3 {
                    x: 1.,
                    y: 0.,
                    z: 0.,
                },
            },
            VertexInput {
                position: MTLPackedFloat3 {
                    x: f32::sqrt(3.0) / 4.0,
                    y: -0.25,
                    z: 0.,
                },
                color: MTLPackedFloat3 {
                    x: 0.,
                    y: 1.,
                    z: 0.,
                },
            },
            VertexInput {
                position: MTLPackedFloat3 {
                    x: 0.,
                    y: 0.5,
                    z: 0.,
                },
                color: MTLPackedFloat3 {
                    x: 0.,
                    y: 0.,
                    z: 1.,
                },
            },
        ];
        // write the triangle geometry to the vertex shader argument buffer at index 1
        let vertex_input_bytes = NonNull::from(vertex_input_data);
        unsafe {
            encoder.setVertexBytes_length_atIndex(
                vertex_input_bytes.cast::<core::ffi::c_void>(),
                core::mem::size_of_val(vertex_input_data),
                1,
            )
        };

        // configure the encoder with the pipeline and draw the triangle
        encoder.setRenderPipelineState(&self.pipeline_state);
        unsafe { encoder.drawPrimitives_vertexStart_vertexCount(MTLPrimitiveType::Triangle, 0, 3) };
        encoder.endEncoding();

        // schedule the command buffer for display and commit
        command_buffer.presentDrawable(drawable.as_ref());
        command_buffer.commit();
    }
}

//
// Boilerplate for setting up a MTKView in a window.
//

define_class!(
    /// The state of our application.
    //
    // SAFETY:
    // - The superclass NSObject does not have any subclassing requirements.
    // - `MainThreadOnly` is correct, since this is an application delegate.
    // - `Delegate` does not implement `Drop`.
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
