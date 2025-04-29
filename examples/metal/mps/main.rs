// Original example taken from https://sergeyreznik.github.io/metal-ray-tracer/part-1/index.html
#![allow(deprecated)]
use std::ptr::NonNull;

use objc2::{rc::Retained, runtime::ProtocolObject, AnyThread};
use objc2_foundation::{ns_string, NSString};
use objc2_metal::{
    MTLCommandBuffer, MTLCommandEncoder, MTLCommandQueue, MTLComputeCommandEncoder,
    MTLComputePipelineDescriptor, MTLComputePipelineState, MTLCreateSystemDefaultDevice, MTLDevice,
    MTLLibrary, MTLPipelineOption, MTLResourceOptions, MTLSize,
};
use objc2_metal_performance_shaders::{
    MPSAccelerationStructureUsage, MPSDataType, MPSIntersectionDataType, MPSIntersectionType,
    MPSRayDataType, MPSRayIntersector, MPSRayOriginMinDistanceDirectionMaxDistance,
    MPSTriangleAccelerationStructure,
};

#[repr(C)]
struct Vertex {
    xyz: [f32; 3],
}

#[repr(C)]
pub struct MPSIntersectionDistancePrimitiveIndexCoordinates {
    pub distance: f32,
    pub primitive_index: u32,
    pub coordinates: [f32; 2], // FIXME: Should be a SIMD vector_float2
}

type Ray = MPSRayOriginMinDistanceDirectionMaxDistance;
type Intersection = MPSIntersectionDistancePrimitiveIndexCoordinates;

fn main() {
    let device = MTLCreateSystemDefaultDevice().expect("No device found");

    let library = device
        .newLibraryWithSource_options_error(ns_string!(include_str!("shaders.metal")), None)
        .unwrap_or_else(|e| panic!("Failed to load shader library: {e}"));

    let generate_rays_pipeline = create_pipeline("generateRays", &library, &device);

    let queue = device.newCommandQueue().unwrap();
    let command_buffer = queue.commandBuffer().unwrap();

    // Simple vertex/index buffer data

    let vertices = [
        Vertex {
            xyz: [0.25, 0.25, 0.0],
        },
        Vertex {
            xyz: [0.75, 0.25, 0.0],
        },
        Vertex {
            xyz: [0.50, 0.75, 0.0],
        },
    ];

    let indices = [0, 1, 2];

    // Vertex data should be stored in private or managed buffers on discrete GPU systems (AMD, NVIDIA).
    // Private buffers are stored entirely in GPU memory and cannot be accessed by the CPU. Managed
    // buffers maintain a copy in CPU memory and a copy in GPU memory.
    let buffer_opts = MTLResourceOptions::StorageModeManaged;

    let vertex_buffer = unsafe {
        device.newBufferWithBytes_length_options(
            NonNull::new(vertices.as_ptr().cast_mut().cast()).unwrap(),
            size_of_val(&vertices),
            buffer_opts,
        )
    }
    .unwrap();

    let index_buffer = unsafe {
        device.newBufferWithBytes_length_options(
            NonNull::new(indices.as_ptr().cast_mut().cast()).unwrap(),
            size_of_val(&indices),
            buffer_opts,
        )
    }
    .unwrap();

    // Build an acceleration structure using our vertex and index buffers containing the single triangle.
    let acceleration_structure = unsafe {
        MPSTriangleAccelerationStructure::initWithDevice(
            MPSTriangleAccelerationStructure::alloc(),
            &device,
        )
    };

    unsafe {
        acceleration_structure.setVertexBuffer(Some(&vertex_buffer));
        acceleration_structure.setVertexStride(size_of::<Vertex>());
        acceleration_structure.setIndexBuffer(Some(&index_buffer));
        acceleration_structure.setIndexType(MPSDataType::UInt32);
        acceleration_structure.setTriangleCount(1);
        acceleration_structure.setUsage(MPSAccelerationStructureUsage::None);
        acceleration_structure.rebuild();
    }

    let ray_intersector =
        unsafe { MPSRayIntersector::initWithDevice(MPSRayIntersector::alloc(), &device) };

    unsafe {
        ray_intersector.setRayStride(size_of::<Ray>());
        ray_intersector.setRayDataType(MPSRayDataType::OriginMinDistanceDirectionMaxDistance);
        ray_intersector.setIntersectionStride(size_of::<Intersection>());
        ray_intersector
            .setIntersectionDataType(MPSIntersectionDataType::DistancePrimitiveIndexCoordinates);
    }

    // Create a buffer to hold generated rays and intersection results
    let ray_count = 1024;
    let ray_buffer = device
        .newBufferWithLength_options(
            size_of::<Ray>() * ray_count,
            MTLResourceOptions::StorageModePrivate,
        )
        .unwrap();

    let intersection_buffer = device
        .newBufferWithLength_options(
            size_of::<Intersection>() * ray_count,
            MTLResourceOptions::StorageModePrivate,
        )
        .unwrap();

    // Run the compute shader to generate rays
    let encoder = command_buffer.computeCommandEncoder().unwrap();
    unsafe {
        encoder.setBuffer_offset_atIndex(Some(&ray_buffer), 0, 0);
        encoder.setComputePipelineState(&generate_rays_pipeline);
        encoder.dispatchThreadgroups_threadsPerThreadgroup(
            MTLSize {
                width: 4,
                height: 4,
                depth: 1,
            },
            MTLSize {
                width: 8,
                height: 8,
                depth: 1,
            },
        );
    }
    encoder.endEncoding();

    // Intersect rays with triangles inside acceleration structure
    unsafe {
        ray_intersector.encodeIntersectionToCommandBuffer_intersectionType_rayBuffer_rayBufferOffset_intersectionBuffer_intersectionBufferOffset_rayCount_accelerationStructure(
            &command_buffer,
            MPSIntersectionType::Nearest,
            &ray_buffer,
            0,
            &intersection_buffer,
            0,
            ray_count,
            &acceleration_structure,
        )
    };

    command_buffer.commit();
    command_buffer.waitUntilCompleted();

    println!("Done");
}

fn create_pipeline(
    func: &str,
    library: &ProtocolObject<dyn MTLLibrary>,
    device: &ProtocolObject<dyn MTLDevice>,
) -> Retained<ProtocolObject<dyn MTLComputePipelineState>> {
    // Create compute pipelines will will execute code on the GPU
    let compute_descriptor = MTLComputePipelineDescriptor::new();

    // Set to YES to allow compiler to make certain optimizations
    unsafe { compute_descriptor.setThreadGroupSizeIsMultipleOfThreadExecutionWidth(true) };

    let function = library
        .newFunctionWithName(&NSString::from_str(func))
        .unwrap();
    compute_descriptor.setComputeFunction(Some(&function));

    device
        .newComputePipelineStateWithDescriptor_options_reflection_error(
            &compute_descriptor,
            MTLPipelineOption::empty(),
            None,
        )
        .unwrap()
}
