//! Regression test for <https://github.com/madsmtm/objc2/issues/758>.
#![cfg(feature = "CMBlockBuffer")]
use objc2_core_foundation::{kCFAllocatorDefault, kCFAllocatorNull, CFRetained};
use objc2_core_media::{
    kCMBlockBufferCustomBlockSourceVersion, kCMBlockBufferNoErr, CMBlockBuffer,
    CMBlockBufferCustomBlockSource,
};
use std::{ffi::c_void, ptr::NonNull};

unsafe extern "C-unwind" fn free_block(
    ref_context: *mut c_void,
    block: NonNull<c_void>,
    size: usize,
) {
    let data = unsafe { Vec::from_raw_parts(block.as_ptr().cast::<u8>(), 2, 2) };
    println!("Freeing block of size: {size}, {data:?}");
    let ref_value = unsafe { *(ref_context.cast::<i32>()) };
    println!("Getting RefCon: {ref_value}");
}

#[test]
fn main() {
    let data_size = 2;
    let data = vec![b'A'; data_size];
    let data = data.leak(); // we free this at `free_block`

    let mut ref_data = 42;
    let mut source: CMBlockBufferCustomBlockSource = unsafe { std::mem::zeroed() };
    source.version = kCMBlockBufferCustomBlockSourceVersion;
    source.FreeBlock = Some(free_block);
    let ref_data: *mut i32 = &mut ref_data;
    source.refCon = ref_data.cast::<c_void>();

    let mut block_buffer: *mut CMBlockBuffer = std::ptr::null_mut();
    let status = unsafe {
        CMBlockBuffer::create_with_memory_block(
            kCFAllocatorDefault,
            data.as_mut_ptr().cast(),
            data_size,
            kCFAllocatorNull,
            &source,
            0,
            data_size,
            0,
            NonNull::from(&mut block_buffer),
        )
    };
    assert_eq!(status, kCMBlockBufferNoErr); // fail to create block buffer
    let block_buffer = unsafe { CFRetained::from_raw(NonNull::new(block_buffer).unwrap()) };
    assert_eq!(block_buffer.retain_count(), 1);
}
