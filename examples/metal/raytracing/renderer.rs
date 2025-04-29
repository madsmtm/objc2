use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    ops::Index,
    ptr::NonNull,
    sync::{Arc, Condvar, Mutex},
};

use glam::{Vec3, Vec4, Vec4Swizzles};
use objc2::{rc::Retained, runtime::ProtocolObject, Message, ProtocolType};
use objc2_core_foundation::CGSize;
use objc2_foundation::{ns_string, NSArray, NSObjectProtocol, NSRange, NSString, NSUInteger};
use objc2_metal::{
    MTLAccelerationStructure, MTLAccelerationStructureCommandEncoder,
    MTLAccelerationStructureDescriptor, MTLAccelerationStructureInstanceDescriptor,
    MTLAccelerationStructureInstanceOptions, MTLBuffer, MTLClearColor, MTLCommandBuffer,
    MTLCommandEncoder, MTLCommandQueue, MTLComputeCommandEncoder, MTLComputePipelineDescriptor,
    MTLComputePipelineState, MTLDataType, MTLDevice, MTLFunction, MTLFunctionConstantValues,
    MTLInstanceAccelerationStructureDescriptor, MTLIntersectionFunctionTable,
    MTLIntersectionFunctionTableDescriptor, MTLLibrary, MTLLinkedFunctions, MTLLoadAction,
    MTLOrigin, MTLPipelineOption, MTLPixelFormat, MTLPrimitiveAccelerationStructureDescriptor,
    MTLPrimitiveType, MTLRegion, MTLRenderCommandEncoder, MTLRenderPassDescriptor,
    MTLRenderPipelineDescriptor, MTLRenderPipelineState, MTLResource, MTLResourceOptions,
    MTLResourceUsage, MTLSize, MTLStorageMode, MTLTexture, MTLTextureDescriptor, MTLTextureType,
    MTLTextureUsage,
};
use objc2_quartz_core::CAMetalDrawable;
use rand::RngCore;

use crate::{camera::Camera, geometry::get_managed_buffer_storage_mode, scene::Scene};

#[repr(C)]
struct Uniforms {
    pub width: u32,
    pub height: u32,
    pub frame_index: u32,
    pub light_count: u32,
    pub camera: Camera,
}

pub const MAX_FRAMES_IN_FLIGHT: NSUInteger = 3;
pub const ALIGNED_UNIFORMS_SIZE: NSUInteger = (size_of::<Uniforms>() + 255) & !255;
pub const UNIFORM_BUFFER_SIZE: NSUInteger = MAX_FRAMES_IN_FLIGHT * ALIGNED_UNIFORMS_SIZE;

#[derive(Clone)]
struct Semaphore {
    data: Arc<(Mutex<usize>, Condvar)>,
}

impl Semaphore {
    fn new(capacity: usize) -> Self {
        Self {
            data: Arc::new((Mutex::new(capacity), Condvar::new())),
        }
    }

    fn acquire(&self) {
        let mut value = self.data.0.lock().unwrap();
        while *value == 0 {
            value = self.data.1.wait(value).unwrap();
        }
        *value -= 1;
    }

    fn release(&self) {
        let mut value = self.data.0.lock().unwrap();
        *value += 1;
        self.data.1.notify_one();
    }
}

pub struct Renderer {
    pub device: Retained<ProtocolObject<dyn MTLDevice>>,
    pub scene: Scene,
    pub uniform_buffer: Retained<ProtocolObject<dyn MTLBuffer>>,
    pub resource_buffer: Retained<ProtocolObject<dyn MTLBuffer>>,
    pub instance_acceleration_structure: Retained<ProtocolObject<dyn MTLAccelerationStructure>>,
    pub accumulation_targets: [RefCell<Retained<ProtocolObject<dyn MTLTexture>>>; 2],
    pub random_texture: RefCell<Retained<ProtocolObject<dyn MTLTexture>>>,
    pub frame_index: Cell<NSUInteger>,
    pub uniform_buffer_index: Cell<NSUInteger>,
    pub uniform_buffer_offset: Cell<NSUInteger>,
    pub size: Cell<CGSize>,
    semaphore: Semaphore,
    pub queue: Retained<ProtocolObject<dyn MTLCommandQueue>>,
    instance_buffer: Retained<ProtocolObject<dyn MTLBuffer>>,
    intersection_function_table: Retained<ProtocolObject<dyn MTLIntersectionFunctionTable>>,
    primitive_acceleration_structures: Vec<Retained<ProtocolObject<dyn MTLAccelerationStructure>>>,
    raytracing_pipeline: Retained<ProtocolObject<dyn MTLComputePipelineState>>,
    copy_pipeline: Retained<ProtocolObject<dyn MTLRenderPipelineState>>,
}

impl Renderer {
    pub fn new(device: &ProtocolObject<dyn MTLDevice>) -> Self {
        let scene = Scene::new(device);

        let library = device
            .newLibraryWithSource_options_error(ns_string!(include_str!("shaders.metal")), None)
            .unwrap_or_else(|e| panic!("{e}"));
        let queue = device.newCommandQueue().unwrap();

        let buffer_data = [0u8; UNIFORM_BUFFER_SIZE];
        let uniform_buffer = unsafe {
            device.newBufferWithBytes_length_options(
                NonNull::new(buffer_data.as_ptr().cast_mut().cast()).unwrap(),
                UNIFORM_BUFFER_SIZE,
                get_managed_buffer_storage_mode(),
            )
        }
        .unwrap();
        uniform_buffer.setLabel(Some(ns_string!("uniform buffer")));
        let resources_stride = {
            let mut max = 0;
            for geometry in &scene.geometries {
                let s = geometry.get_resources().len();
                if s > max {
                    max = s;
                }
            }
            max
        };
        let mut resource_buffer_data = vec![0u64; resources_stride * scene.geometries.len()];
        for geometry_index in 0..scene.geometries.len() {
            let geometry = scene.geometries[geometry_index].as_ref();
            let resource_buffer_begin_index = resources_stride * geometry_index;
            let resources = geometry.get_resources();

            for (argument_index, resource) in resources.iter().enumerate() {
                let resource_buffer_index = resource_buffer_begin_index + argument_index;
                let resource_ptr: *const ProtocolObject<dyn MTLResource> = &**resource;
                let value = if resource.conformsToProtocol(<dyn MTLBuffer>::protocol().unwrap()) {
                    let buffer = unsafe { &*resource_ptr.cast::<ProtocolObject<dyn MTLBuffer>>() };
                    buffer.gpuAddress()
                } else if resource.conformsToProtocol(<dyn MTLTexture>::protocol().unwrap()) {
                    let texture =
                        unsafe { &*resource_ptr.cast::<ProtocolObject<dyn MTLTexture>>() };
                    texture.gpuResourceID().to_raw()
                } else {
                    panic!("Unexpected resource!")
                };
                resource_buffer_data[resource_buffer_index] = value;
            }
        }
        let resource_buffer = unsafe {
            device.newBufferWithBytes_length_options(
                NonNull::new(resource_buffer_data.as_ptr().cast_mut().cast()).unwrap(),
                size_of_val(resource_buffer_data.as_slice()),
                get_managed_buffer_storage_mode(),
            )
        }
        .unwrap();
        resource_buffer.setLabel(Some(ns_string!("resource buffer")));
        resource_buffer.didModifyRange(NSRange::new(0, resource_buffer.length()));

        let mut primitive_acceleration_structures = Vec::new();
        for i in 0..scene.geometries.len() {
            let mesh = scene.geometries[i].as_ref();
            let geometry_descriptor = mesh.get_geometry_descriptor();
            geometry_descriptor.setIntersectionFunctionTableOffset(i);
            let geometry_descriptors = NSArray::from_slice(&[&*geometry_descriptor]);
            let accel_descriptor = MTLPrimitiveAccelerationStructureDescriptor::descriptor();
            accel_descriptor.setGeometryDescriptors(Some(&geometry_descriptors));
            primitive_acceleration_structures.push(
                Self::new_acceleration_structure_with_descriptor(device, &queue, &accel_descriptor),
            );
        }

        let mut instance_descriptors = vec![
            MTLAccelerationStructureInstanceDescriptor::default();
            scene.geometry_instances.len()
        ];
        for (instance_index, instance) in scene.geometry_instances.iter().enumerate() {
            let geometry_index = instance.index_in_scene;
            instance_descriptors[instance_index].accelerationStructureIndex = geometry_index as u32;
            instance_descriptors[instance_index].options =
                if instance.geometry.get_intersection_function_name().is_none() {
                    MTLAccelerationStructureInstanceOptions::Opaque
                } else {
                    MTLAccelerationStructureInstanceOptions::None
                };
            instance_descriptors[instance_index].intersectionFunctionTableOffset = 0;
            instance_descriptors[instance_index].mask = instance.mask;
            for column in 0..4 {
                let packed = &mut instance_descriptors[instance_index]
                    .transformationMatrix
                    .columns[column];
                packed.x = *instance.transform.col(column).index(0);
                packed.y = *instance.transform.col(column).index(1);
                packed.z = *instance.transform.col(column).index(2);
            }
        }
        let instance_buffer = unsafe {
            device.newBufferWithBytes_length_options(
                NonNull::new(instance_descriptors.as_ptr().cast_mut().cast()).unwrap(),
                size_of_val(instance_descriptors.as_slice()),
                get_managed_buffer_storage_mode(),
            )
        }
        .unwrap();
        instance_buffer.setLabel(Some(ns_string!("instance buffer")));
        instance_buffer.didModifyRange(NSRange::new(0, instance_buffer.length()));

        let accel_descriptor = MTLInstanceAccelerationStructureDescriptor::descriptor();
        accel_descriptor.setInstancedAccelerationStructures(Some(&NSArray::from_retained_slice(
            &primitive_acceleration_structures,
        )));
        accel_descriptor.setInstanceCount(scene.geometry_instances.len());
        accel_descriptor.setInstanceDescriptorBuffer(Some(&instance_buffer));
        let instance_acceleration_structure =
            Self::new_acceleration_structure_with_descriptor(device, &queue, &accel_descriptor);

        let mut intersection_functions =
            BTreeMap::<Retained<NSString>, Retained<ProtocolObject<dyn MTLFunction>>>::new();
        for geometry in &scene.geometries {
            if let Some(name) = geometry.get_intersection_function_name() {
                if !intersection_functions.contains_key(name) {
                    let intersection_function = Self::new_specialised_function_with_name(
                        &library,
                        resources_stride as u32,
                        name,
                    );
                    intersection_functions.insert(name.retain(), intersection_function);
                }
            }
        }
        let raytracing_function = Self::new_specialised_function_with_name(
            &library,
            resources_stride as u32,
            ns_string!("raytracingKernel"),
        );
        let intersection_function_array: Vec<&ProtocolObject<dyn MTLFunction>> =
            intersection_functions.values().map(|f| &**f).collect();
        let intersection_function_array = NSArray::from_slice(&intersection_function_array);
        let raytracing_pipeline = Self::new_compute_pipeline_state_with_function(
            device,
            &raytracing_function,
            &intersection_function_array,
        );
        let intersection_function_table_descriptor = MTLIntersectionFunctionTableDescriptor::new();
        intersection_function_table_descriptor.setFunctionCount(scene.geometries.len());
        let intersection_function_table = raytracing_pipeline
            .newIntersectionFunctionTableWithDescriptor(&intersection_function_table_descriptor)
            .unwrap();
        for geometry_index in 0..scene.geometries.len() {
            let geometry = scene.geometries[geometry_index].as_ref();
            if let Some(intersection_function_name) = geometry.get_intersection_function_name() {
                let intersection_function = &intersection_functions[intersection_function_name];
                let handle = raytracing_pipeline
                    .functionHandleWithFunction(intersection_function)
                    .unwrap();
                intersection_function_table.setFunction_atIndex(Some(&handle), geometry_index);
            }
        }
        let render_descriptor = MTLRenderPipelineDescriptor::new();
        render_descriptor.setVertexFunction(Some(
            &library
                .newFunctionWithName(ns_string!("copyVertex"))
                .unwrap(),
        ));
        render_descriptor.setFragmentFunction(Some(
            &library
                .newFunctionWithName(ns_string!("copyFragment"))
                .unwrap(),
        ));
        unsafe {
            render_descriptor
                .colorAttachments()
                .objectAtIndexedSubscript(0)
        }
        .setPixelFormat(MTLPixelFormat::RGBA16Float);
        let copy_pipeline = device
            .newRenderPipelineStateWithDescriptor_error(&render_descriptor)
            .unwrap();

        let texture_descriptor = Self::create_target_descriptor(1024, 1024);
        let accumulation_targets = [
            RefCell::new(
                device
                    .newTextureWithDescriptor(&texture_descriptor)
                    .unwrap(),
            ),
            RefCell::new(
                device
                    .newTextureWithDescriptor(&texture_descriptor)
                    .unwrap(),
            ),
        ];
        let random_texture = RefCell::new(
            device
                .newTextureWithDescriptor(&texture_descriptor)
                .unwrap(),
        );

        Self {
            device: device.retain(),
            scene,
            uniform_buffer,
            resource_buffer,
            instance_acceleration_structure,
            accumulation_targets,
            random_texture,
            frame_index: Cell::new(0),
            uniform_buffer_index: Cell::new(0),
            uniform_buffer_offset: Cell::new(0),
            size: Cell::new(CGSize::new(1024.0, 1024.0)),
            semaphore: Semaphore::new(MAX_FRAMES_IN_FLIGHT - 2),
            instance_buffer,
            queue,
            intersection_function_table,
            primitive_acceleration_structures,
            raytracing_pipeline,
            copy_pipeline,
        }
    }

    fn create_target_descriptor(
        width: NSUInteger,
        height: NSUInteger,
    ) -> Retained<MTLTextureDescriptor> {
        let texture_descriptor = MTLTextureDescriptor::new();
        texture_descriptor.setPixelFormat(MTLPixelFormat::RGBA32Float);
        texture_descriptor.setTextureType(MTLTextureType::Type2D);
        unsafe {
            texture_descriptor.setWidth(width);
            texture_descriptor.setHeight(height);
        }
        texture_descriptor.setStorageMode(MTLStorageMode::Private);
        texture_descriptor.setUsage(MTLTextureUsage::ShaderRead | MTLTextureUsage::ShaderWrite);
        texture_descriptor
    }

    pub fn resize(&self, size: CGSize) {
        self.size.set(size);
        let texture_descriptor =
            Self::create_target_descriptor(size.width as NSUInteger, size.height as NSUInteger);
        *self.accumulation_targets[0].borrow_mut() = self
            .device
            .newTextureWithDescriptor(&texture_descriptor)
            .unwrap();
        *self.accumulation_targets[1].borrow_mut() = self
            .device
            .newTextureWithDescriptor(&texture_descriptor)
            .unwrap();
        texture_descriptor.setPixelFormat(MTLPixelFormat::R32Uint);
        texture_descriptor.setUsage(MTLTextureUsage::ShaderRead);
        texture_descriptor.setStorageMode(MTLStorageMode::Managed);
        *self.random_texture.borrow_mut() = self
            .device
            .newTextureWithDescriptor(&texture_descriptor)
            .unwrap();
        let mut rng = rand::rng();
        let mut random_values = vec![0u32; (size.width * size.height) as usize];
        for v in &mut random_values {
            *v = rng.next_u32();
        }
        unsafe {
            self.random_texture
                .borrow()
                .replaceRegion_mipmapLevel_withBytes_bytesPerRow(
                    // TODO: new_2d
                    MTLRegion {
                        origin: MTLOrigin { x: 0, y: 0, z: 0 },
                        size: MTLSize {
                            width: size.width as NSUInteger,
                            height: size.height as NSUInteger,
                            depth: 1,
                        },
                    },
                    0,
                    NonNull::new(random_values.as_ptr().cast_mut().cast()).unwrap(),
                    size_of::<u32>() * size.width as NSUInteger,
                )
        };
        self.frame_index.set(0);
    }

    fn update_uniforms(&self) {
        self.uniform_buffer_offset
            .set(ALIGNED_UNIFORMS_SIZE * self.uniform_buffer_index.get());

        let uniforms = unsafe {
            self.uniform_buffer
                .contents()
                .add(self.uniform_buffer_offset.get())
                .cast::<Uniforms>()
                .as_mut()
        };

        let position = self.scene.camera.position;
        let target = self.scene.camera.forward;
        let up = self.scene.camera.up;

        let forward = Vec3::normalize(target.xyz() - position.xyz());
        let right = Vec3::normalize(Vec3::cross(forward, up.xyz()));
        let up = Vec3::normalize(Vec3::cross(right, forward));

        uniforms.camera.position = position;
        uniforms.camera.forward = Vec4::from((forward, 0.0));
        uniforms.camera.right = Vec4::from((right, 0.0));
        uniforms.camera.up = Vec4::from((up, 0.0));

        let field_of_view = 45.0 * (std::f32::consts::PI / 180.0);
        #[allow(clippy::unnecessary_cast)]
        let aspect_ratio = self.size.get().width as f32 / self.size.get().height as f32;
        let image_plane_height = f32::tan(field_of_view / 2.0);
        let image_plane_width = aspect_ratio * image_plane_height;

        uniforms.camera.right *= image_plane_width;
        uniforms.camera.up *= image_plane_height;

        uniforms.width = self.size.get().width as u32;
        uniforms.height = self.size.get().height as u32;

        uniforms.frame_index = self.frame_index.get() as u32;
        self.frame_index.set(self.frame_index.get() + 1);

        uniforms.light_count = self.scene.lights.len() as u32;

        self.uniform_buffer.didModifyRange(NSRange {
            location: self.uniform_buffer_offset.get(),
            length: ALIGNED_UNIFORMS_SIZE,
        });

        self.uniform_buffer_index
            .set((self.uniform_buffer_index.get() + 1) % MAX_FRAMES_IN_FLIGHT);
    }

    pub fn draw(&self, drawable: &ProtocolObject<dyn CAMetalDrawable>) {
        self.semaphore.acquire();
        self.update_uniforms();
        let command_buffer = self.queue.commandBuffer().unwrap();
        let sem = self.semaphore.clone();
        let block = block2::RcBlock::new(move |_| {
            sem.release();
        });
        unsafe { command_buffer.addCompletedHandler(block2::RcBlock::as_ptr(&block)) };
        let width = self.size.get().width as NSUInteger;
        let height = self.size.get().height as NSUInteger;
        let threads_per_thread_group = MTLSize {
            width: 8,
            height: 8,
            depth: 1,
        };
        let thread_groups = MTLSize {
            width: width.div_ceil(threads_per_thread_group.width),
            height: height.div_ceil(threads_per_thread_group.height),
            depth: 1,
        };
        let compute_encoder = command_buffer.computeCommandEncoder().unwrap();
        unsafe {
            compute_encoder.setBuffer_offset_atIndex(
                Some(&self.uniform_buffer),
                self.uniform_buffer_offset.get(),
                0,
            );
            compute_encoder.setBuffer_offset_atIndex(Some(&self.instance_buffer), 0, 2);
            compute_encoder.setBuffer_offset_atIndex(Some(&self.scene.lights_buffer), 0, 3);
            compute_encoder.setAccelerationStructure_atBufferIndex(
                Some(&self.instance_acceleration_structure),
                4,
            );
            compute_encoder.setIntersectionFunctionTable_atBufferIndex(
                Some(&self.intersection_function_table),
                5,
            );
            compute_encoder.setTexture_atIndex(Some(&self.random_texture.borrow()), 0);
            compute_encoder.setTexture_atIndex(Some(&self.accumulation_targets[0].borrow()), 1);
            compute_encoder.setTexture_atIndex(Some(&self.accumulation_targets[1].borrow()), 2);
        }
        for geometry in &self.scene.geometries {
            for resource in geometry.get_resources() {
                compute_encoder.useResource_usage(resource, MTLResourceUsage::Read);
            }
        }
        for primitive_acceleration_structure in &self.primitive_acceleration_structures {
            compute_encoder.useResource_usage(
                primitive_acceleration_structure.as_ref(),
                MTLResourceUsage::Read,
            );
        }
        compute_encoder.setComputePipelineState(&self.raytracing_pipeline);
        compute_encoder
            .dispatchThreadgroups_threadsPerThreadgroup(thread_groups, threads_per_thread_group);
        compute_encoder.endEncoding();
        std::mem::swap(
            &mut *self.accumulation_targets[0].borrow_mut(),
            &mut *self.accumulation_targets[1].borrow_mut(),
        );

        let render_pass_descriptor = MTLRenderPassDescriptor::new();
        let colour_attachment = unsafe {
            render_pass_descriptor
                .colorAttachments()
                .objectAtIndexedSubscript(0)
        };
        colour_attachment.setTexture(Some(&drawable.texture()));
        colour_attachment.setLoadAction(MTLLoadAction::Clear);
        colour_attachment.setClearColor(MTLClearColor {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        });
        let render_encoder = command_buffer
            .renderCommandEncoderWithDescriptor(&render_pass_descriptor)
            .unwrap();
        render_encoder.setRenderPipelineState(&self.copy_pipeline);
        unsafe {
            render_encoder
                .setFragmentTexture_atIndex(Some(&self.accumulation_targets[0].borrow()), 0);
            render_encoder.drawPrimitives_vertexStart_vertexCount(MTLPrimitiveType::Triangle, 0, 6);
        }
        render_encoder.endEncoding();
        command_buffer.presentDrawable(drawable.as_ref());

        command_buffer.commit();
    }

    fn new_acceleration_structure_with_descriptor(
        device: &ProtocolObject<dyn MTLDevice>,
        queue: &ProtocolObject<dyn MTLCommandQueue>,
        descriptor: &MTLAccelerationStructureDescriptor,
    ) -> Retained<ProtocolObject<dyn MTLAccelerationStructure>> {
        let accel_sizes = device.accelerationStructureSizesWithDescriptor(descriptor);
        let acceleration_structure = device
            .newAccelerationStructureWithSize(accel_sizes.accelerationStructureSize)
            .unwrap();
        let scratch_buffer = device
            .newBufferWithLength_options(
                accel_sizes.buildScratchBufferSize,
                MTLResourceOptions::StorageModePrivate,
            )
            .unwrap();
        let command_buffer = queue.commandBuffer().unwrap();
        let command_encoder = command_buffer
            .accelerationStructureCommandEncoder()
            .unwrap();
        let compacted_size_buffer = device
            .newBufferWithLength_options(size_of::<u32>(), MTLResourceOptions::StorageModeShared)
            .unwrap();
        command_encoder.buildAccelerationStructure_descriptor_scratchBuffer_scratchBufferOffset(
            &acceleration_structure,
            descriptor,
            &scratch_buffer,
            0,
        );
        command_encoder.writeCompactedAccelerationStructureSize_toBuffer_offset(
            &acceleration_structure,
            &compacted_size_buffer,
            0,
        );
        command_encoder.endEncoding();
        command_buffer.commit();
        command_buffer.waitUntilCompleted();
        let compacted_size = unsafe { compacted_size_buffer.contents().cast::<u32>().read() };
        let compacted_acceleration_structure = device
            .newAccelerationStructureWithSize(compacted_size as NSUInteger)
            .unwrap();
        let command_buffer = queue.commandBuffer().unwrap();
        let command_encoder = command_buffer
            .accelerationStructureCommandEncoder()
            .unwrap();
        command_encoder.copyAndCompactAccelerationStructure_toAccelerationStructure(
            &acceleration_structure,
            &compacted_acceleration_structure,
        );
        command_encoder.endEncoding();
        command_buffer.commit();
        compacted_acceleration_structure
    }

    fn new_specialised_function_with_name(
        library: &ProtocolObject<dyn MTLLibrary>,
        resources_stride: u32,
        name: &NSString,
    ) -> Retained<ProtocolObject<dyn MTLFunction>> {
        let constants = MTLFunctionConstantValues::new();
        let resources_stride = resources_stride * size_of::<u64>() as u32;
        unsafe {
            constants.setConstantValue_type_atIndex(
                NonNull::from(&resources_stride).cast(),
                MTLDataType::UInt,
                0,
            )
        };
        let v = true;
        unsafe {
            constants.setConstantValue_type_atIndex(NonNull::from(&v).cast(), MTLDataType::Bool, 1)
        };
        unsafe {
            constants.setConstantValue_type_atIndex(NonNull::from(&v).cast(), MTLDataType::Bool, 2)
        };
        library
            .newFunctionWithName_constantValues_error(name, &constants)
            .unwrap()
    }

    fn new_compute_pipeline_state_with_function(
        device: &ProtocolObject<dyn MTLDevice>,
        function: &ProtocolObject<dyn MTLFunction>,
        linked_functions: &NSArray<ProtocolObject<dyn MTLFunction>>,
    ) -> Retained<ProtocolObject<dyn MTLComputePipelineState>> {
        let linked_functions = {
            let lf = MTLLinkedFunctions::new();
            lf.setFunctions(Some(linked_functions));
            lf
        };
        let descriptor = MTLComputePipelineDescriptor::new();
        descriptor.setComputeFunction(Some(function));
        descriptor.setLinkedFunctions(Some(&linked_functions));
        unsafe { descriptor.setThreadGroupSizeIsMultipleOfThreadExecutionWidth(true) };
        device
            .newComputePipelineStateWithDescriptor_options_reflection_error(
                &descriptor,
                MTLPipelineOption::None,
                None,
            )
            .unwrap()
    }
}
