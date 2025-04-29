//! This example replicates `Synchronizing Events Between a GPU and the CPU` article:
//! <https://developer.apple.com/documentation/metal/synchronization/synchronizing_events_between_a_gpu_and_the_cpu>

use std::ptr::NonNull;

use block2::RcBlock;
use dispatch2::{DispatchQueue, DispatchQueueAttr};
use objc2::{runtime::ProtocolObject, AnyThread};
use objc2_metal::{
    MTLCommandBuffer, MTLCommandQueue, MTLCreateSystemDefaultDevice, MTLDevice, MTLSharedEvent,
    MTLSharedEventListener,
};

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {}

fn main() {
    let device = MTLCreateSystemDefaultDevice().expect("No device found");

    let command_queue = device.newCommandQueue().unwrap();
    let command_buffer = command_queue.commandBuffer().unwrap();

    // Shareable event
    let shared_event = device.newSharedEvent().unwrap();

    // Shareable event listener
    let my_queue = DispatchQueue::new(
        "com.example.apple-samplecode.MyQueue",
        DispatchQueueAttr::SERIAL,
    );

    // Enable `dispatch` feature to use dispatch queues,
    // otherwise unsafe `from_queue_handle` is available for use with native APIs.
    let shared_event_listener = unsafe {
        MTLSharedEventListener::initWithDispatchQueue(MTLSharedEventListener::alloc(), &my_queue)
    };

    // Register CPU work
    let notify_block = RcBlock::new(
        move |evt: NonNull<ProtocolObject<dyn MTLSharedEvent>>, val: u64| {
            let evt = unsafe { evt.as_ref() };
            println!("Got notification from GPU: {}", val);
            evt.setSignaledValue(3);
        },
    );

    unsafe {
        shared_event.notifyListener_atValue_block(
            &shared_event_listener,
            2,
            RcBlock::as_ptr(&notify_block),
        )
    };

    // Encode GPU work
    command_buffer.encodeSignalEvent_value(shared_event.as_ref(), 1);
    command_buffer.encodeSignalEvent_value(shared_event.as_ref(), 2);
    command_buffer.encodeWaitForEvent_value(shared_event.as_ref(), 3);

    command_buffer.commit();

    command_buffer.waitUntilCompleted();

    println!("Done");
}
