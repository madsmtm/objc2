//! Test assembly output of `block2`.
// Limit to 64-bit Apple since we don't do anything target-specific.
#![cfg(all(target_vendor = "apple", target_pointer_width = "64"))]

use block2::{Block, RcBlock, StackBlock};

#[export_name = "fn1_stack_block_to_rc"]
fn stack_block_to_rc() -> RcBlock<dyn Fn(i32) -> i32> {
    StackBlock::new(|x| x + 2).copy()
}

#[export_name = "fn2_rc_block"]
fn rc_block() -> RcBlock<dyn Fn(i32) -> i32> {
    RcBlock::new(|x| x + 2)
}

#[export_name = "fn3_rc_block_drop"]
fn rc_block_drop(b: Box<i32>) -> RcBlock<dyn Fn(i32) -> i32> {
    RcBlock::new(move |x| x + *b)
}

extern "C" {
    fn needs_block(block: &Block<dyn Fn(i32) -> i32>);
}

#[export_name = "fn4_create_and_use_stack_block"]
fn create_and_use_stack_block() {
    let block = StackBlock::new(|x| x + 2);
    unsafe { needs_block(&block) };
}

#[export_name = "fn5_create_and_use_stack_block_drop"]
fn create_and_use_stack_block_drop(b: Box<i32>) {
    let block = StackBlock::new(move |x| x + *b);
    unsafe { needs_block(&block) };
}

#[export_name = "fn6_create_and_use_rc_block"]
fn create_and_use_rc_block() {
    let block = RcBlock::new(|x| x + 2);
    unsafe { needs_block(&block) };
}
