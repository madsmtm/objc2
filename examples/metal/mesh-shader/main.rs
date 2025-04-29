// Modified from <https://github.com/gfx-rs/metal-rs/tree/v0.33.0/examples/mesh-shader>
#![cfg_attr(not(target_os = "macos"), allow(dead_code, unused))]

use std::cell::OnceCell;

use objc2::{
    define_class, msg_send, rc::Retained, runtime::ProtocolObject, Ivars, MainThreadMarker,
    MainThreadOnly,
};
#[cfg(target_os = "macos")]
use objc2_app_kit::{
    NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate, NSBackingStoreType,
    NSWindow, NSWindowStyleMask,
};
use objc2_foundation::{
    ns_string, NSNotification, NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize,
};
use objc2_metal::{
    MTLClearColor, MTLCommandBuffer, MTLCommandEncoder, MTLCommandQueue,
    MTLCreateSystemDefaultDevice, MTLDevice, MTLLibrary, MTLLoadAction,
    MTLMeshRenderPipelineDescriptor, MTLPipelineOption, MTLPixelFormat, MTLRenderCommandEncoder,
    MTLRenderPassDescriptor, MTLRenderPipelineState, MTLSize, MTLStoreAction,
};
#[cfg(target_os = "macos")]
use objc2_metal_kit::{MTKView, MTKViewDelegate};
use objc2_quartz_core::CAMetalDrawable;

/// The state of our renderer.
#[derive(Debug)]
struct Renderer {
    pipeline_state: Retained<ProtocolObject<dyn MTLRenderPipelineState>>,
    command_queue: Retained<ProtocolObject<dyn MTLCommandQueue>>,
}

impl Renderer {
    fn new(device: &ProtocolObject<dyn MTLDevice>, pixel_format: MTLPixelFormat) -> Self {
        let library = device
            .newLibraryWithSource_options_error(ns_string!(include_str!("shaders.metal")), None)
            .unwrap_or_else(|e| panic!("{e}"));

        let mesh = library
            .newFunctionWithName(ns_string!("mesh_function"))
            .unwrap();
        let frag = library
            .newFunctionWithName(ns_string!("fragment_function"))
            .unwrap();

        let pipeline_state_desc = MTLMeshRenderPipelineDescriptor::new();
        let color_attachment = unsafe {
            pipeline_state_desc
                .colorAttachments()
                .objectAtIndexedSubscript(0)
        };
        color_attachment.setPixelFormat(pixel_format);
        unsafe { pipeline_state_desc.setMeshFunction(Some(&mesh)) };
        unsafe { pipeline_state_desc.setFragmentFunction(Some(&frag)) };

        let pipeline_state = device
            .newRenderPipelineStateWithMeshDescriptor_options_reflection_error(
                &pipeline_state_desc,
                MTLPipelineOption::None,
                None,
            )
            .unwrap();

        let command_queue = device.newCommandQueue().unwrap();

        Self {
            pipeline_state,
            command_queue,
        }
    }

    fn draw(&self, drawable: &ProtocolObject<dyn CAMetalDrawable>) {
        let render_pass_descriptor = MTLRenderPassDescriptor::new();

        let color_attachment = unsafe {
            render_pass_descriptor
                .colorAttachments()
                .objectAtIndexedSubscript(0)
        };

        color_attachment.setTexture(Some(&drawable.texture()));
        color_attachment.setLoadAction(MTLLoadAction::Clear);
        color_attachment.setClearColor(MTLClearColor {
            red: 0.2,
            green: 0.2,
            blue: 0.25,
            alpha: 1.0,
        });
        color_attachment.setStoreAction(MTLStoreAction::Store);

        let command_buffer = self.command_queue.commandBuffer().unwrap();
        let encoder = command_buffer
            .renderCommandEncoderWithDescriptor(&render_pass_descriptor)
            .unwrap();

        encoder.setRenderPipelineState(&self.pipeline_state);
        encoder.drawMeshThreads_threadsPerObjectThreadgroup_threadsPerMeshThreadgroup(
            MTLSize {
                width: 1,
                height: 1,
                depth: 1,
            },
            MTLSize {
                width: 1,
                height: 1,
                depth: 1,
            },
            MTLSize {
                width: 1,
                height: 1,
                depth: 1,
            },
        );

        encoder.endEncoding();

        command_buffer.presentDrawable(drawable.as_ref());
        command_buffer.commit();
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
