// Modified from <https://github.com/gfx-rs/metal-rs/tree/v0.33.0/examples/compute>
use std::ptr::NonNull;

use objc2::{
    rc::{autoreleasepool, Retained},
    runtime::ProtocolObject,
};
use objc2_foundation::{ns_string, NSRange, NSUInteger};
use objc2_metal::{
    MTLBlitCommandEncoder, MTLBuffer, MTLCommandBuffer, MTLCommandEncoder, MTLCommandQueue,
    MTLComputeCommandEncoder, MTLComputePassDescriptor, MTLComputePipelineDescriptor,
    MTLComputePipelineState, MTLCounterSampleBuffer, MTLCounterSampleBufferDescriptor,
    MTLCounterSamplingPoint, MTLCounterSet, MTLCreateSystemDefaultDevice, MTLDevice, MTLFunction,
    MTLFunctionType, MTLLibrary, MTLResourceOptions, MTLSize, MTLStorageMode,
};

const NUM_SAMPLES: usize = 2;

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {}

fn main() {
    let num_elements = std::env::args()
        .nth(1)
        .map(|s| s.parse::<u32>().unwrap())
        .unwrap_or(64 * 64);

    autoreleasepool(|_| {
        let device = MTLCreateSystemDefaultDevice().expect("No device found");
        let mut cpu_start = 0;
        let mut gpu_start = 0;
        device.sampleTimestamps_gpuTimestamp(&mut cpu_start, &mut gpu_start);

        let counter_sample_buffer = create_counter_sample_buffer(&device);
        let destination_buffer = device
            .newBufferWithLength_options(
                size_of::<u64>() * NUM_SAMPLES,
                MTLResourceOptions::StorageModeShared,
            )
            .unwrap();

        let counter_sampling_point = MTLCounterSamplingPoint::AtStageBoundary;
        assert!(device.supportsCounterSampling(counter_sampling_point));

        let command_queue = device.newCommandQueue().unwrap();
        let command_buffer = command_queue.commandBuffer().unwrap();

        let compute_pass_descriptor = MTLComputePassDescriptor::new();
        handle_compute_pass_sample_buffer_attachment(
            &compute_pass_descriptor,
            &counter_sample_buffer,
        );
        let encoder = command_buffer
            .computeCommandEncoderWithDescriptor(&compute_pass_descriptor)
            .unwrap();

        let pipeline_state = create_pipeline_state(&device);
        encoder.setComputePipelineState(&pipeline_state);

        let (buffer, sum) = create_input_and_output_buffers(&device, num_elements);
        unsafe { encoder.setBuffer_offset_atIndex(Some(&buffer), 0, 0) };
        unsafe { encoder.setBuffer_offset_atIndex(Some(&sum), 0, 1) };

        let num_threads = pipeline_state.threadExecutionWidth();

        let thread_group_count = MTLSize {
            width: ((num_elements as NSUInteger + num_threads) / num_threads),
            height: 1,
            depth: 1,
        };

        let thread_group_size = MTLSize {
            width: num_threads,
            height: 1,
            depth: 1,
        };

        encoder.dispatchThreadgroups_threadsPerThreadgroup(thread_group_count, thread_group_size);
        encoder.endEncoding();

        resolve_samples_into_buffer(&command_buffer, &counter_sample_buffer, &destination_buffer);

        command_buffer.commit();
        command_buffer.waitUntilCompleted();
        let mut cpu_end = 0;
        let mut gpu_end = 0;
        device.sampleTimestamps_gpuTimestamp(&mut cpu_end, &mut gpu_end);

        let sum = unsafe { sum.contents().cast::<u32>().read() };
        println!("Compute shader sum: {}", sum);

        assert_eq!(num_elements, sum);

        handle_timestamps(&destination_buffer, cpu_start, cpu_end, gpu_start, gpu_end);
    });
}

fn create_pipeline_state(
    device: &ProtocolObject<dyn MTLDevice>,
) -> Retained<ProtocolObject<dyn MTLComputePipelineState>> {
    let library = device
        .newLibraryWithSource_options_error(ns_string!(include_str!("shaders.metal")), None)
        .unwrap_or_else(|e| panic!("{e}"));
    let kernel = library.newFunctionWithName(ns_string!("sum")).unwrap();

    debug_assert_eq!(&*kernel.name(), ns_string!("sum"));
    debug_assert_eq!(kernel.functionType(), MTLFunctionType::Kernel);

    let pipeline_state_descriptor = MTLComputePipelineDescriptor::new();
    pipeline_state_descriptor.setComputeFunction(Some(&kernel));

    device
        .newComputePipelineStateWithFunction_error(
            &pipeline_state_descriptor.computeFunction().unwrap(),
        )
        .unwrap()
}

fn handle_compute_pass_sample_buffer_attachment(
    compute_pass_descriptor: &MTLComputePassDescriptor,
    counter_sample_buffer: &ProtocolObject<dyn MTLCounterSampleBuffer>,
) {
    let sample_buffer_attachment_descriptor = unsafe {
        compute_pass_descriptor
            .sampleBufferAttachments()
            .objectAtIndexedSubscript(0)
    };

    sample_buffer_attachment_descriptor.setSampleBuffer(Some(counter_sample_buffer));
    unsafe { sample_buffer_attachment_descriptor.setStartOfEncoderSampleIndex(0) };
    unsafe { sample_buffer_attachment_descriptor.setEndOfEncoderSampleIndex(1) };
}

fn resolve_samples_into_buffer(
    command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
    counter_sample_buffer: &ProtocolObject<dyn MTLCounterSampleBuffer>,
    destination_buffer: &ProtocolObject<dyn MTLBuffer>,
) {
    let blit_encoder = command_buffer.blitCommandEncoder().unwrap();
    unsafe {
        blit_encoder.resolveCounters_inRange_destinationBuffer_destinationOffset(
            counter_sample_buffer,
            NSRange::new(0, NUM_SAMPLES),
            destination_buffer,
            0,
        )
    };
    blit_encoder.endEncoding();
}

fn handle_timestamps(
    resolved_sample_buffer: &ProtocolObject<dyn MTLBuffer>,
    cpu_start: u64,
    cpu_end: u64,
    gpu_start: u64,
    gpu_end: u64,
) {
    let samples = unsafe {
        std::slice::from_raw_parts(
            resolved_sample_buffer.contents().as_ptr().cast::<u64>(),
            NUM_SAMPLES,
        )
    };
    let pass_start = samples[0];
    let pass_end = samples[1];

    let cpu_time_span = cpu_end - cpu_start;
    let gpu_time_span = gpu_end - gpu_start;

    let micros = microseconds_between_begin(pass_start, pass_end, gpu_time_span, cpu_time_span);
    println!("Compute pass duration: {} µs", micros);
}

fn create_counter_sample_buffer(
    device: &ProtocolObject<dyn MTLDevice>,
) -> Retained<ProtocolObject<dyn MTLCounterSampleBuffer>> {
    let counter_sample_buffer_desc = MTLCounterSampleBufferDescriptor::new();
    counter_sample_buffer_desc.setStorageMode(MTLStorageMode::Shared);
    unsafe { counter_sample_buffer_desc.setSampleCount(NUM_SAMPLES) };
    let counter_sets = device.counterSets().unwrap();

    let timestamp_counter = counter_sets
        .iter()
        .find(|cs| *cs.name() == *ns_string!("timestamp"))
        .expect("No timestamp counter found");

    counter_sample_buffer_desc.setCounterSet(Some(&timestamp_counter));

    device
        .newCounterSampleBufferWithDescriptor_error(&counter_sample_buffer_desc)
        .unwrap()
}

#[allow(clippy::type_complexity)]
fn create_input_and_output_buffers(
    device: &ProtocolObject<dyn MTLDevice>,
    num_elements: u32,
) -> (
    Retained<ProtocolObject<dyn MTLBuffer>>,
    Retained<ProtocolObject<dyn MTLBuffer>>,
) {
    let data = vec![1u32; num_elements as usize];

    let buffer = unsafe {
        device.newBufferWithBytes_length_options(
            NonNull::new(data.as_ptr().cast_mut().cast()).unwrap(),
            size_of_val(data.as_slice()),
            MTLResourceOptions::CPUCacheModeDefaultCache,
        )
    }
    .unwrap();

    let data = [0u32];
    let sum = unsafe {
        device.newBufferWithBytes_length_options(
            NonNull::new(data.as_ptr().cast_mut().cast()).unwrap(),
            size_of_val(&data),
            MTLResourceOptions::CPUCacheModeDefaultCache,
        )
    }
    .unwrap();
    (buffer, sum)
}

/// <https://developer.apple.com/documentation/metal/gpu_counters_and_counter_sample_buffers/converting_gpu_timestamps_into_cpu_time>
fn microseconds_between_begin(begin: u64, end: u64, gpu_time_span: u64, cpu_time_span: u64) -> f64 {
    let time_span = (end as f64) - (begin as f64);
    let nanoseconds = time_span / (gpu_time_span as f64) * (cpu_time_span as f64);
    nanoseconds / 1000.0
}
