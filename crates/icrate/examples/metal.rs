#![deny(unsafe_op_in_unsafe_fn)]

use core::ptr::NonNull;

use icrate::{
    ns_string,
    AppKit::{
        NSApplication, NSApplicationActivationPolicyRegular, NSApplicationDelegate,
        NSBackingStoreBuffered, NSWindow, NSWindowStyleMaskClosable, NSWindowStyleMaskResizable,
        NSWindowStyleMaskTitled,
    },
    Foundation::{NSNotification, NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize, NSString},
    Metal::{
        MTLCommandBuffer, MTLCommandEncoder, MTLCommandQueue, MTLCreateSystemDefaultDevice,
        MTLDevice, MTLDrawable, MTLLibrary, MTLPrimitiveTypeTriangle, MTLRenderCommandEncoder,
        MTLRenderPipelineDescriptor, MTLRenderPipelineState,
    },
    MetalKit::{MTKView, MTKViewDelegate},
};
use objc2::{
    declare::{Ivar, IvarDrop},
    declare_class, msg_send, msg_send_id,
    mutability::InteriorMutable,
    rc::Id,
    runtime::ProtocolObject,
    ClassType,
};

// declare the Objective-C class machinery
declare_class!(
    // declare the delegate class with our instance variables
    struct Delegate {
        device: IvarDrop<Id<ProtocolObject<dyn MTLDevice>>, "_device">,
        command_queue: IvarDrop<Id<ProtocolObject<dyn MTLCommandQueue>>, "_command_queue">,
        pipeline_state: IvarDrop<Id<ProtocolObject<dyn MTLRenderPipelineState>>, "_pipeline_state">,
        window: IvarDrop<Id<NSWindow>, "_window">,
        mtk_view: IvarDrop<Id<MTKView>, "_mtk_view">,
    }
    mod ivars;

    // declare the class type
    unsafe impl ClassType for Delegate {
        type Super = NSObject;
        type Mutability = InteriorMutable;
        const NAME: &'static str = "Delegate";
    }

    // define the Delegate methods (e.g., initializer)
    unsafe impl Delegate {
        #[method(initWithShaders:)]
        #[allow(non_snake_case)]
        unsafe fn __initWithShaders(this: *mut Self, shaders: &NSString) -> Option<NonNull<Self>> {
            let this: Option<&mut Self> = msg_send![super(this), init];

            // get the default device
            let device = {
                let ptr = unsafe { MTLCreateSystemDefaultDevice() };
                unsafe { Id::retain(ptr) }.expect("Failed to get default system device.")
            };

            // create the command queue
            let command_queue = device
                .newCommandQueue()
                .expect("Failed to create a command queue.");

            // create the app window
            let window = {
                let this = NSWindow::alloc();
                let content_rect = NSRect::new(NSPoint::new(0., 0.), NSSize::new(1024., 768.));
                let style = NSWindowStyleMaskClosable
                    | NSWindowStyleMaskResizable
                    | NSWindowStyleMaskTitled;
                let backing_store_type = NSBackingStoreBuffered;
                let flag = false;
                unsafe {
                    NSWindow::initWithContentRect_styleMask_backing_defer(
                        this,
                        content_rect,
                        style,
                        backing_store_type,
                        flag,
                    )
                }
            };

            // create the metal view
            let mtk_view = {
                let this = MTKView::alloc();
                let frame_rect = unsafe { window.frame() };
                unsafe { MTKView::initWithFrame_device(this, frame_rect, Some(&device)) }
            };

            // create the pipeline descriptor
            let pipeline_descriptor = MTLRenderPipelineDescriptor::new();

            unsafe {
                pipeline_descriptor
                    .colorAttachments()
                    .objectAtIndexedSubscript(0)
                    .setPixelFormat(mtk_view.colorPixelFormat());
            }

            // create the pipeline state
            let library = device
                .newLibraryWithSource_options_error(shaders, None)
                .expect("Failed to create a library.");

            let vertex_function = library.newFunctionWithName(ns_string!("vertex_main"));
            pipeline_descriptor.setVertexFunction(vertex_function.as_deref());

            let fragment_function = library.newFunctionWithName(ns_string!("fragment_main"));
            pipeline_descriptor.setFragmentFunction(fragment_function.as_deref());

            let pipeline_state = device
                .newRenderPipelineStateWithDescriptor_error(&pipeline_descriptor)
                .expect("Failed to create a pipeline state.");

            this.map(|this| {
                Ivar::write(&mut this.device, device);
                Ivar::write(&mut this.command_queue, command_queue);
                Ivar::write(&mut this.pipeline_state, pipeline_state);
                Ivar::write(&mut this.window, window);
                Ivar::write(&mut this.mtk_view, mtk_view);
                NonNull::from(this)
            })
        }
    }

    // define the delegate methods for the `NSApplicationDelegate` protocol
    unsafe impl NSApplicationDelegate for Delegate {
        #[method(applicationDidFinishLaunching:)]
        #[allow(non_snake_case)]
        unsafe fn applicationDidFinishLaunching(&self, _notification: &NSNotification) {
            // configure the metal view delegate
            unsafe {
                let object = ProtocolObject::from_ref(self);
                self.mtk_view.setDelegate(Some(object));
            }

            // configure the window
            unsafe {
                self.window.setContentView(Some(&self.mtk_view));
                self.window.center();
                self.window.setTitle(ns_string!("metal example"));
                self.window.makeKeyAndOrderFront(None);
            }
        }
    }

    // define the delegate methods for the `MTKViewDelegate` protocol
    unsafe impl MTKViewDelegate for Delegate {
        #[method(drawInMTKView:)]
        #[allow(non_snake_case)]
        unsafe fn drawInMTKView(&self, _view: &MTKView) {
            // FIXME: icrate `MTKView` doesn't have a generated binding for `currentDrawable` yet
            // (because it needs a definition of `CAMetalDrawable`, which we don't support yet) so
            // we have to use a raw `msg_send_id` call here instead.
            let current_drawable: Option<Id<ProtocolObject<dyn MTLDrawable>>> =
                msg_send_id![&*self.mtk_view, currentDrawable];

            let Some(current_drawable) = current_drawable else { return; };
            let Some(command_buffer) = self.command_queue.commandBuffer() else { return; };
            let Some(pass_descriptor) = (unsafe { self.mtk_view.currentRenderPassDescriptor() }) else { return; };
            let Some(encoder) = command_buffer.renderCommandEncoderWithDescriptor(&pass_descriptor) else { return; };

            #[rustfmt::skip]
            let vertex_data: &mut [f32] = &mut [
                -0.5, -0.5, 0. , 1. , 0. , 0. ,
                 0.5, -0.5, 0. , 0. , 1. , 0. ,
                 0. ,  0.5, 0. , 0. , 0. , 1. ,
            ];
            let vertex_bytes = unsafe {
                NonNull::new_unchecked(vertex_data.as_mut_ptr().cast::<core::ffi::c_void>())
            };
            unsafe {
                encoder.setVertexBytes_length_atIndex(
                    vertex_bytes,
                    vertex_data.len() * core::mem::size_of::<f32>(),
                    0,
                )
            };

            encoder.setRenderPipelineState(&self.pipeline_state);
            unsafe {
                encoder.drawPrimitives_vertexStart_vertexCount(MTLPrimitiveTypeTriangle, 0, 3)
            };
            encoder.endEncoding();

            command_buffer.presentDrawable(&current_drawable);

            command_buffer.commit();
        }

        #[method(mtkView:drawableSizeWillChange:)]
        #[allow(non_snake_case)]
        unsafe fn mtkView_drawableSizeWillChange(&self, _view: &MTKView, _size: NSSize) {
            println!("mtkView_drawableSizeWillChange");
        }
    }
);

unsafe impl NSObjectProtocol for Delegate {}

#[cfg(target_os = "macos")]
impl Delegate {
    pub fn init_with_shaders(shaders: &NSString) -> Id<Self> {
        unsafe { msg_send_id![Self::alloc(), initWithShaders: shaders] }
    }
}

fn main() {
    let app = unsafe { NSApplication::sharedApplication() };
    unsafe { app.setActivationPolicy(NSApplicationActivationPolicyRegular) };

    let shaders = ns_string!(
        r#"
        #include <metal_stdlib>
        using namespace metal;
        struct VertexIn {
            packed_float3 position;
            packed_float3 color;
        };
        struct VertexOut {
            float4 position [[position]];
            float4 color;
        };
        vertex VertexOut vertex_main(device const VertexIn *vertices [[buffer(0)]],
                                    uint vertexId [[vertex_id]]) {
            VertexOut out;
            out.position = float4(vertices[vertexId].position, 1);
            out.color = float4(vertices[vertexId].color, 1);
            return out;
        }
        fragment float4 fragment_main(VertexOut in [[stage_in]]) {
            return in.color;
        }
    "#
    );

    // initialize the delegate
    let delegate = Delegate::init_with_shaders(shaders);

    // configure the application delegate
    unsafe {
        let object = ProtocolObject::from_ref(&*delegate);
        app.setDelegate(Some(object))
    };

    // run the app
    unsafe { app.run() };
}
