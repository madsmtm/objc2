use core::f32;
use std::cell::Cell;
use std::ptr::NonNull;

use block2::RcBlock;
use dispatch2::{DispatchRetained, DispatchSemaphore, DispatchTime};
use objc2::rc::Retained;
use objc2::runtime::{Bool, ProtocolObject};
use objc2::{define_class, msg_send, AnyThread, ClassType, DeclaredClass, MainThreadOnly};
use objc2_core_foundation::CGSize;
use objc2_foundation::{
    ns_string, NSDictionary, NSNull, NSNumber, NSObject, NSObjectProtocol, NSURL,
};
use objc2_metal::{
    MTLBuffer, MTLCommandBuffer, MTLCommandEncoder, MTLCommandQueue, MTLCompareFunction,
    MTLCullMode, MTLDepthStencilDescriptor, MTLDepthStencilState, MTLDevice, MTLLibrary,
    MTLPixelFormat, MTLRenderCommandEncoder, MTLRenderPipelineDescriptor, MTLRenderPipelineState,
    MTLResource, MTLResourceOptions, MTLStorageMode, MTLTexture, MTLTextureUsage,
    MTLVertexDescriptor, MTLVertexFormat, MTLVertexStepFunction, MTLWinding,
};
use objc2_metal_kit::{
    MTKMesh, MTKMeshBufferAllocator, MTKModelIOVertexDescriptorFromMetal, MTKTextureLoader,
    MTKTextureLoaderOptionTextureStorageMode, MTKTextureLoaderOptionTextureUsage, MTKView,
    MTKViewDelegate,
};
use objc2_model_io::{
    MDLGeometryType, MDLMesh, MDLVertexAttributePosition, MDLVertexAttributeTextureCoordinate,
};

use crate::matrix::{
    matrix_multiply, matrix_perspective_right_hand, matrix_rotation, matrix_translation, Float3,
    Float4x4, UInt3,
};
use crate::shader_types::{
    BufferIndexMeshGenerics, BufferIndexMeshPositions, BufferIndexUniforms, TextureIndexColor,
    Uniforms, VertexAttributePosition, VertexAttributeTexcoord,
};

const MAX_BUFFERS_IN_FLIGHT: usize = 3;
const ALIGNED_UNIFORMS_SIZE: usize = (size_of::<Uniforms>() & !0xFF) + 0x100;

#[derive(Debug)]
pub struct Ivars {
    in_flight_semaphore: DispatchRetained<DispatchSemaphore>,
    command_queue: Retained<ProtocolObject<dyn MTLCommandQueue>>,

    dynamic_uniform_buffer: Retained<ProtocolObject<dyn MTLBuffer>>,
    pipeline_state: Retained<ProtocolObject<dyn MTLRenderPipelineState>>,
    depth_state: Retained<ProtocolObject<dyn MTLDepthStencilState>>,
    mesh: Retained<MTKMesh>,
    color_map: Retained<ProtocolObject<dyn MTLTexture>>,

    uniform_buffer_offset: Cell<usize>,
    uniform_buffer_index: Cell<u8>,
    uniform_buffer_address: Cell<Option<NonNull<Uniforms>>>,

    projection_matrix: Cell<Float4x4>,
    rotation: Cell<f32>,
}

define_class!(
    // SAFETY:
    // - The superclass NSObject does not have any subclassing requirements.
    // - `Renderer` does not implement `Drop`.
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[ivars = Ivars]
    #[derive(Debug)]
    pub struct Renderer;

    unsafe impl NSObjectProtocol for Renderer {}

    unsafe impl MTKViewDelegate for Renderer {
        #[unsafe(method(drawInMTKView:))]
        fn draw_in_mtk_view(&self, view: &MTKView) {
            self.draw(view);
        }

        #[unsafe(method(mtkView:drawableSizeWillChange:))]
        fn mtk_view_drawable_size_will_change(&self, _view: &MTKView, size: CGSize) {
            self.size_will_change(size);
        }
    }
);

impl Renderer {
    pub fn new(view: &MTKView) -> Retained<Self> {
        let in_flight_semaphore = DispatchSemaphore::new(MAX_BUFFERS_IN_FLIGHT as isize);

        unsafe {
            view.setDepthStencilPixelFormat(MTLPixelFormat::Depth32Float_Stencil8);
            view.setColorPixelFormat(MTLPixelFormat::BGRA8Unorm_sRGB);
            view.setSampleCount(1);
        }

        let device = unsafe { view.device() }.unwrap();

        let command_queue = device.newCommandQueue().unwrap();

        let vertex_descriptor = load_vertex_descriptor();
        let pipeline_state = load_pipeline_state(&device, view, &vertex_descriptor);
        let depth_state = load_depth_state(&device);
        let dynamic_uniform_buffer = load_dynamic_uniform_buffer(&device);

        // Load assets into metal objects.
        let mesh = load_mesh(&device, &vertex_descriptor);
        let color_map = load_color_map(&device);

        let this = Self::alloc(view.mtm());
        let this = this.set_ivars(Ivars {
            in_flight_semaphore,
            command_queue,
            pipeline_state,
            depth_state,
            dynamic_uniform_buffer,
            mesh,
            color_map,
            uniform_buffer_offset: Cell::new(0),
            uniform_buffer_index: Cell::new(0),
            uniform_buffer_address: Cell::new(None),
            projection_matrix: Cell::new(Float4x4::new([0.0; 4], [0.0; 4], [0.0; 4], [0.0; 4])),
            rotation: Cell::new(0.0),
        });
        unsafe { msg_send![super(this), init] }
    }
}

fn load_pipeline_state(
    device: &ProtocolObject<dyn MTLDevice>,
    view: &MTKView,
    vertex_descriptor: &MTLVertexDescriptor,
) -> Retained<ProtocolObject<dyn MTLRenderPipelineState>> {
    let default_library = device
        .newLibraryWithSource_options_error(
            &ns_string!(include_str!("shader_types.h"))
                .stringByAppendingString(ns_string!(include_str!("shaders.metal"))),
            None,
        )
        .unwrap();

    let vertex_function = default_library
        .newFunctionWithName(ns_string!("vertexShader"))
        .unwrap();

    let fragment_function = default_library
        .newFunctionWithName(ns_string!("fragmentShader"))
        .unwrap();

    let pipeline_state_descriptor = MTLRenderPipelineDescriptor::new();
    unsafe {
        pipeline_state_descriptor.setLabel(Some(ns_string!("MyPipeline")));
        pipeline_state_descriptor.setRasterSampleCount(view.sampleCount());
        pipeline_state_descriptor.setVertexFunction(Some(&vertex_function));
        pipeline_state_descriptor.setFragmentFunction(Some(&fragment_function));
        pipeline_state_descriptor.setVertexDescriptor(Some(vertex_descriptor));
        pipeline_state_descriptor
            .colorAttachments()
            .objectAtIndexedSubscript(0)
            .setPixelFormat(view.colorPixelFormat());
        pipeline_state_descriptor.setDepthAttachmentPixelFormat(view.depthStencilPixelFormat());
        pipeline_state_descriptor.setStencilAttachmentPixelFormat(view.depthStencilPixelFormat());
    }

    device
        .newRenderPipelineStateWithDescriptor_error(&pipeline_state_descriptor)
        .expect("failed creating pipeline state")
}

fn load_vertex_descriptor() -> Retained<MTLVertexDescriptor> {
    let vertex_descriptor = MTLVertexDescriptor::new();

    unsafe {
        let attributes = vertex_descriptor.attributes();

        let attr = attributes.objectAtIndexedSubscript(VertexAttributePosition);
        attr.setFormat(MTLVertexFormat::Float3);
        attr.setOffset(0);
        attr.setBufferIndex(BufferIndexMeshPositions);

        let attr = attributes.objectAtIndexedSubscript(VertexAttributeTexcoord);
        attr.setFormat(MTLVertexFormat::Float2);
        attr.setOffset(0);
        attr.setBufferIndex(BufferIndexMeshGenerics);

        let layouts = vertex_descriptor.layouts();

        let layout = layouts.objectAtIndexedSubscript(BufferIndexMeshPositions);
        layout.setStride(12);
        layout.setStepRate(1);
        layout.setStepFunction(MTLVertexStepFunction::PerVertex);

        let layout = layouts.objectAtIndexedSubscript(BufferIndexMeshGenerics);
        layout.setStride(8);
        layout.setStepRate(1);
        layout.setStepFunction(MTLVertexStepFunction::PerVertex);
    }

    vertex_descriptor
}

fn load_depth_state(
    device: &ProtocolObject<dyn MTLDevice>,
) -> Retained<ProtocolObject<dyn MTLDepthStencilState>> {
    let depth_state_desc = MTLDepthStencilDescriptor::new();
    depth_state_desc.setDepthCompareFunction(MTLCompareFunction::Less);
    depth_state_desc.setDepthWriteEnabled(true);
    device
        .newDepthStencilStateWithDescriptor(&depth_state_desc)
        .unwrap()
}

fn load_dynamic_uniform_buffer(
    device: &ProtocolObject<dyn MTLDevice>,
) -> Retained<ProtocolObject<dyn MTLBuffer>> {
    let uniform_buffer_size = ALIGNED_UNIFORMS_SIZE * MAX_BUFFERS_IN_FLIGHT;

    let dynamic_uniform_buffer = device
        .newBufferWithLength_options(uniform_buffer_size, MTLResourceOptions::StorageModeShared)
        .unwrap();

    dynamic_uniform_buffer.setLabel(Some(ns_string!("UniformBuffer")));

    dynamic_uniform_buffer
}

fn load_mesh(
    device: &ProtocolObject<dyn MTLDevice>,
    vertex_descriptor: &MTLVertexDescriptor,
) -> Retained<MTKMesh> {
    let metal_allocator =
        unsafe { MTKMeshBufferAllocator::initWithDevice(MTKMeshBufferAllocator::alloc(), device) };

    let mdl_mesh: Retained<MDLMesh> = unsafe {
        msg_send![
            MDLMesh::class(),
            newBoxWithDimensions: Float3::splat(4.0),
            segments: UInt3::splat(2),
            geometryType: MDLGeometryType::Triangles,
            inwardNormals: Bool::NO,
            allocator: &*metal_allocator,
        ]
    };

    unsafe {
        let desc = MTKModelIOVertexDescriptorFromMetal(vertex_descriptor);

        let attributes = desc.attributes();
        attributes
            .objectAtIndex(VertexAttributePosition)
            .setName(MDLVertexAttributePosition);
        attributes
            .objectAtIndex(VertexAttributeTexcoord)
            .setName(MDLVertexAttributeTextureCoordinate);

        mdl_mesh.setVertexDescriptor(&desc);

        MTKMesh::initWithMesh_device_error(MTKMesh::alloc(), &mdl_mesh, device)
            .expect("failed MetalKit mesh")
    }
}

fn load_color_map(
    device: &ProtocolObject<dyn MTLDevice>,
) -> Retained<ProtocolObject<dyn MTLTexture>> {
    let texture_loader =
        unsafe { MTKTextureLoader::initWithDevice(MTKTextureLoader::alloc(), device) };
    let options = NSDictionary::from_slices(
        unsafe {
            &[
                MTKTextureLoaderOptionTextureUsage,
                MTKTextureLoaderOptionTextureStorageMode,
            ]
        },
        &[
            NSNumber::new_usize(MTLTextureUsage::ShaderRead.0).as_ref(),
            NSNumber::new_usize(MTLStorageMode::Private.0).as_ref(),
        ],
    );
    unsafe {
        texture_loader.newTextureWithContentsOfURL_options_error(
            &NSURL::fileURLWithPath(ns_string!(env!("CARGO_MANIFEST_DIR")))
                .URLByAppendingPathComponent(ns_string!("default_xcode_game/ColorMap.png"))
                .unwrap(),
            Some(&options),
        )
    }
    .expect("failed creating texture")
}

impl Renderer {
    /// Update the state of our uniform buffers before rendering.
    fn update_dynamic_buffer_state(&self) {
        self.ivars()
            .uniform_buffer_index
            .set((self.ivars().uniform_buffer_index.get() + 1) % MAX_BUFFERS_IN_FLIGHT as u8);

        self.ivars()
            .uniform_buffer_offset
            .set(ALIGNED_UNIFORMS_SIZE * self.ivars().uniform_buffer_index.get() as usize);

        self.ivars().uniform_buffer_address.set(Some(unsafe {
            self.ivars()
                .dynamic_uniform_buffer
                .contents()
                .cast::<Uniforms>()
                .byte_add(self.ivars().uniform_buffer_offset.get())
        }));
    }

    /// Update any game state before encoding rendering commands to our drawable.
    fn update_game_state(&self) {
        // SAFETY: TODO.
        let uniforms = unsafe { self.ivars().uniform_buffer_address.get().unwrap().as_mut() };

        uniforms.projection_matrix = self.ivars().projection_matrix.get();

        let rotation_axis = (1.0, 1.0, 0.0);
        let model_matrix = matrix_rotation(self.ivars().rotation.get(), rotation_axis);
        let view_matrix = matrix_translation(0.0, 0.0, -8.0);

        uniforms.model_view_matrix = matrix_multiply(model_matrix, view_matrix);

        self.ivars()
            .rotation
            .set(self.ivars().rotation.get() + 0.01);
    }

    /// Per frame updates.
    fn draw(&self, view: &MTKView) {
        self.ivars().in_flight_semaphore.wait(DispatchTime::FOREVER);

        let command_buffer = self.ivars().command_queue.commandBuffer().unwrap();
        command_buffer.setLabel(Some(ns_string!("MyCommand")));

        let block_sema = self.ivars().in_flight_semaphore.clone();
        unsafe {
            command_buffer.addCompletedHandler(RcBlock::as_ptr(&RcBlock::new(move |_buffer| {
                block_sema.signal();
            })))
        };

        self.update_dynamic_buffer_state();

        self.update_game_state();

        // Delay getting the currentRenderPassDescriptor until we absolutely
        // need it to avoid holding onto the drawable and blocking the display
        // pipeline any longer than necessary.
        let render_pass_descriptor = unsafe { view.currentRenderPassDescriptor() };

        if let Some(render_pass_descriptor) = render_pass_descriptor {
            // Final pass rendering code here.

            let render_encoder = command_buffer
                .renderCommandEncoderWithDescriptor(&render_pass_descriptor)
                .unwrap();
            render_encoder.setLabel(Some(ns_string!("MyRenderEncoder")));

            render_encoder.pushDebugGroup(ns_string!("DrawBox"));

            render_encoder.setFrontFacingWinding(MTLWinding::CounterClockwise);
            render_encoder.setCullMode(MTLCullMode::Back);
            render_encoder.setRenderPipelineState(&self.ivars().pipeline_state);
            render_encoder.setDepthStencilState(Some(&self.ivars().depth_state));

            unsafe {
                render_encoder.setVertexBuffer_offset_atIndex(
                    Some(&self.ivars().dynamic_uniform_buffer),
                    self.ivars().uniform_buffer_offset.get(),
                    BufferIndexUniforms,
                )
            };

            unsafe {
                render_encoder.setFragmentBuffer_offset_atIndex(
                    Some(&self.ivars().dynamic_uniform_buffer),
                    self.ivars().uniform_buffer_offset.get(),
                    BufferIndexUniforms,
                )
            };

            let vertex_buffers = unsafe { self.ivars().mesh.vertexBuffers() };
            for (i, vertex_buffer) in vertex_buffers.into_iter().enumerate() {
                if **vertex_buffer == **NSNull::null() {
                    eprintln!("got null vertex_buffer");
                    continue;
                }
                unsafe {
                    render_encoder.setVertexBuffer_offset_atIndex(
                        Some(&vertex_buffer.buffer()),
                        vertex_buffer.offset(),
                        i,
                    )
                };
            }

            unsafe {
                render_encoder
                    .setFragmentTexture_atIndex(Some(&self.ivars().color_map), TextureIndexColor)
            };

            for submesh in unsafe { self.ivars().mesh.submeshes() } {
                unsafe {
                    render_encoder
                        .drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferOffset(
                            submesh.primitiveType(),
                            submesh.indexCount(),
                            submesh.indexType(),
                            &submesh.indexBuffer().buffer(),
                            submesh.indexBuffer().offset(),
                        )
                };
            }

            render_encoder.popDebugGroup();

            render_encoder.endEncoding();

            command_buffer.presentDrawable(ProtocolObject::from_ref(
                &*unsafe { view.currentDrawable() }.unwrap(),
            ));
        } else {
            eprintln!("no render_pass_descriptor");
        }

        command_buffer.commit();
    }

    /// Respond to drawable size or orientation changes.
    fn size_will_change(&self, size: CGSize) {
        let aspect = size.width / size.height;
        self.ivars()
            .projection_matrix
            .set(matrix_perspective_right_hand(
                65.0 * (f32::consts::PI / 180.0),
                aspect as f32,
                0.1,
                100.0,
            ));
    }
}
