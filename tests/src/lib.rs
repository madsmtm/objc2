//! Test of `block2`.
//!
//! This is used so that we don't need to add a `build.rs` script to `block2`.
#![no_std]
use block2::{Block, RcBlock};

extern crate alloc;
extern crate std;

pub mod ffi;
#[cfg(test)]
mod test_encode_utils;
#[cfg(test)]
mod test_object;

use crate::ffi::LargeStruct;

pub fn get_int_block_with(i: i32) -> RcBlock<(), i32> {
    unsafe {
        let ptr = ffi::get_int_block_with(i);
        RcBlock::new(ptr as *mut _)
    }
}

pub fn get_add_block_with(i: i32) -> RcBlock<(i32,), i32> {
    unsafe {
        let ptr = ffi::get_add_block_with(i);
        RcBlock::new(ptr as *mut _)
    }
}

pub fn invoke_int_block(block: &Block<(), i32>) -> i32 {
    let ptr = block as *const _;
    unsafe { ffi::invoke_int_block(ptr as *mut _) }
}

pub fn invoke_add_block(block: &Block<(i32,), i32>, a: i32) -> i32 {
    let ptr = block as *const _;
    unsafe { ffi::invoke_add_block(ptr as *mut _, a) }
}

pub fn invoke_large_struct_block(
    block: &Block<(LargeStruct,), LargeStruct>,
    x: LargeStruct,
) -> LargeStruct {
    let ptr = block as *const _;
    unsafe { ffi::invoke_large_struct_block(ptr as *mut _, x) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;
    use block2::{global_block, ConcreteBlock, RcBlock};

    global_block! {
        /// Test `global_block` in an external crate
        static MY_BLOCK = || -> i32 {
            42
        };
    }

    #[test]
    fn test_global_block() {
        assert_eq!(invoke_int_block(&MY_BLOCK), 42);
    }

    #[test]
    fn test_call_block() {
        let block = get_int_block_with(13);
        unsafe {
            assert_eq!(block.call(()), 13);
        }
    }

    #[test]
    fn test_call_block_args() {
        let block = get_add_block_with(13);
        unsafe {
            assert_eq!(block.call((2,)), 15);
        }
    }

    #[test]
    fn test_create_block() {
        let block = ConcreteBlock::new(|| 13);
        let result = invoke_int_block(&block);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_create_block_args() {
        let block = ConcreteBlock::new(|a: i32| a + 5);
        let result = invoke_add_block(&block, 6);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_concrete_block_copy() {
        let s = "Hello!".to_string();
        let expected_len = s.len() as i32;
        let block = ConcreteBlock::new(move || s.len() as i32);
        assert_eq!(invoke_int_block(&block), expected_len);

        let copied = block.copy();
        assert_eq!(invoke_int_block(&copied), expected_len);
    }

    #[test]
    fn test_concrete_block_stack_copy() {
        fn make_block() -> RcBlock<(), i32> {
            let x = 7;
            let block = ConcreteBlock::new(move || x);
            block.copy()
        }

        let block = make_block();
        assert_eq!(invoke_int_block(&block), 7);
    }

    #[test]
    fn test_large_struct_block() {
        global_block! {
            static BLOCK = |data: LargeStruct| -> LargeStruct {
                let mut data = data;
                data.mutate();
                data
            };
        }

        let data = LargeStruct::get();
        let mut new_data = data;
        new_data.mutate();

        assert_eq!(unsafe { BLOCK.call((data,)) }, new_data);
        assert_eq!(invoke_large_struct_block(&BLOCK, data), new_data);

        let block = ConcreteBlock::new(|mut x: LargeStruct| {
            x.mutate();
            x
        });
        assert_eq!(invoke_large_struct_block(&block, data), new_data);
        let block = block.copy();
        assert_eq!(invoke_large_struct_block(&block, data), new_data);
    }

    #[test]
    #[cfg(feature = "ui")]
    fn ui() {
        let t = trybuild::TestCases::new();
        t.compile_fail("ui/*.rs");
    }
}
