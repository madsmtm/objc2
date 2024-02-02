//! Test assembly output of `block2`.
// Limit to 64-bit Apple since we don't do anything target-specific.
#![cfg(all(feature = "apple", target_pointer_width = "64"))]

use block2::{Block, RcBlock, StackBlock};

#[no_mangle]
fn stack_block_to_rc() -> RcBlock<dyn Fn(i32) -> i32> {
    StackBlock::new(|x| x + 2).copy()
}

#[no_mangle]
fn rc_block() -> RcBlock<dyn Fn(i32) -> i32> {
    RcBlock::new(|x| x + 2)
}

#[no_mangle]
fn rc_block_drop(b: Box<i32>) -> RcBlock<dyn Fn(i32) -> i32> {
    RcBlock::new(move |x| x + *b)
}

extern "C" {
    fn needs_block(block: &Block<dyn Fn(i32) -> i32>);
}

#[no_mangle]
fn create_and_use_stack_block() {
    let block = StackBlock::new(|x| x + 2);
    unsafe { needs_block(&block) };
}

#[no_mangle]
fn create_and_use_stack_block_drop(b: Box<i32>) {
    let block = StackBlock::new(move |x| x + *b);
    unsafe { needs_block(&block) };
}

#[no_mangle]
fn create_and_use_rc_block() {
    let block = RcBlock::new(|x| x + 2);
    unsafe { needs_block(&block) };
}
