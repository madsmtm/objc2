// Modified from <https://github.com/gfx-rs/metal-rs/tree/v0.33.0/examples/shader-dylib>

#![cfg_attr(not(target_os = "macos"), allow(dead_code, unused))]
#![cfg_attr(feature = "unstable-darwin-objc", feature(darwin_objc))]

use core::cell::OnceCell;

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{define_class, msg_send, Ivars, MainThreadMarker, MainThreadOnly};
#[cfg(target_os = "macos")]
use objc2_app_kit::{
    NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate, NSBackingStoreType,
    NSWindow, NSWindowStyleMask,
};
use objc2_foundation::{
    ns_string, NSArray, NSNotification, NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize,
    NSString, NSURL,
};
use objc2_metal::{
    MTLCommandBuffer, MTLCommandEncoder, MTLCommandQueue, MTLCompileOptions,
    MTLComputeCommandEncoder, MTLComputePipelineState, MTLCopyAllDevices, MTLDevice,
    MTLDynamicLibrary, MTLLibrary, MTLLibraryType, MTLPixelFormat, MTLSize,
};
#[cfg(target_os = "macos")]
use objc2_metal_kit::{MTKView, MTKViewDelegate};
use objc2_quartz_core::CAMetalDrawable;

#[derive(Debug)]
struct Renderer {
    command_queue: Retained<ProtocolObject<dyn MTLCommandQueue>>,
    image_fill_cps: Retained<ProtocolObject<dyn MTLComputePipelineState>>,
}

fn select_device() -> Retained<ProtocolObject<dyn MTLDevice>> {
    let devices = MTLCopyAllDevices();
    devices
        .into_iter()
        .find(|d| d.supportsDynamicLibraries())
        .expect("could not find Metal device that supports dynamic libraries")
}

#[cfg(target_os = "macos")]
fn configure_view(mtk_view: &MTKView) {
    // Allow filling the layer's texture with a compute shader.
    //
    // You could also set these properties on the CAMetalLayer.
    mtk_view.setFramebufferOnly(false);
    mtk_view.setColorPixelFormat(MTLPixelFormat::BGRA8Unorm);
}

impl Renderer {
    fn new(device: &ProtocolObject<dyn MTLDevice>) -> Self {
        let command_queue = device.newCommandQueue().unwrap();

        // compile dynamic lib shader
        let dylib_src = ns_string!(include_str!("test_dylib.metal"));

        let install_path = std::env::temp_dir().join("test_dylib.metallib");

        let opts = MTLCompileOptions::new();
        opts.setLibraryType(MTLLibraryType::Dynamic);
        opts.setInstallName(Some(&NSString::from_str(install_path.to_str().unwrap())));

        let lib = device
            .newLibraryWithSource_options_error(dylib_src, Some(&opts))
            .unwrap_or_else(|e| panic!("{e}"));

        // create dylib
        let dylib = device.newDynamicLibrary_error(&lib).unwrap();
        dylib.setLabel(Some(ns_string!("test_dylib")));

        // optional: serialize binary blob that can be loaded later
        let url = NSURL::from_file_path(&install_path).unwrap();
        dylib.serializeToURL_error(&url).unwrap();

        // create shader that links with dylib
        let shader_src = ns_string!(include_str!("test_shader.metal"));

        let opts = MTLCompileOptions::new();
        // add dynamic library to link with
        let libraries = NSArray::from_slice(&[dylib.as_ref()]);
        opts.setLibraries(Some(&libraries));

        // compile
        let shader_lib = device
            .newLibraryWithSource_options_error(shader_src, Some(&opts))
            .unwrap_or_else(|e| panic!("{e}"));

        let func = shader_lib
            .newFunctionWithName(ns_string!("test_kernel"))
            .unwrap();

        // create pipeline state
        // linking occurs here
        let image_fill_cps = device
            .newComputePipelineStateWithFunction_error(&func)
            .unwrap();

        Self {
            command_queue,
            image_fill_cps,
        }
    }

    fn draw(&self, drawable: &ProtocolObject<dyn CAMetalDrawable>) {
        let size = drawable.layer().drawableSize();

        let w = self.image_fill_cps.threadExecutionWidth();
        let h = self.image_fill_cps.maxTotalThreadsPerThreadgroup() / w;
        let threads_per_threadgroup = MTLSize {
            width: w,
            height: h,
            depth: 1,
        };
        let threads_per_grid = MTLSize {
            width: size.width as _,
            height: size.height as _,
            depth: 1,
        };

        let command_buffer = self.command_queue.commandBuffer().unwrap();

        {
            let encoder = command_buffer.computeCommandEncoder().unwrap();
            encoder.setComputePipelineState(&self.image_fill_cps);
            unsafe { encoder.setTexture_atIndex(Some(&drawable.texture()), 0) };
            encoder
                .dispatchThreads_threadsPerThreadgroup(threads_per_grid, threads_per_threadgroup);
            encoder.endEncoding();
        }

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

            let device = select_device();

            // create the metal view
            let mtk_view = {
                let frame_rect = window.frame();
                MTKView::initWithFrame_device(MTKView::alloc(mtm), frame_rect, Some(&device))
            };

            // initialize the renderer
            let renderer = Renderer::new(&device);
            self.renderer().set(renderer).unwrap();

            // configure the metal view delegate
            let object = ProtocolObject::from_ref(self);
            mtk_view.setDelegate(Some(object));

            configure_view(&mtk_view);

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
