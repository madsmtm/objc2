use std::os::raw::c_void;

use block2::Block;
use objc2::{Encode, Encoding};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LargeStruct {
    pub x: f32,
    pub y: [u8; 100],
}

impl LargeStruct {
    pub fn get() -> Self {
        let mut y = [10; 100];
        y[42] = 123;
        Self { x: 5.0, y }
    }

    pub fn mutate(&mut self) {
        self.x -= 1.0;
        self.y[12] += 1;
        self.y[99] = 123;
    }
}

unsafe impl Encode for LargeStruct {
    const ENCODING: Encoding =
        Encoding::Struct("LargeStruct", &[f32::ENCODING, <[u8; 100]>::ENCODING]);
}

extern "C" {
    /// Returns a pointer to a global block that returns 7.
    pub fn get_int_block() -> *mut Block<(), i32>;
    /// Returns a pointer to a copied block that returns `i`.
    pub fn get_int_block_with(i: i32) -> *mut Block<(), i32>;
    /// Returns a pointer to a global block that returns its argument + 7.
    pub fn get_add_block() -> *mut Block<(i32,), i32>;
    /// Returns a pointer to a copied block that returns its argument + `i`.
    pub fn get_add_block_with(i: i32) -> *mut Block<(i32,), i32>;
    /// Invokes a block and returns its result.
    pub fn invoke_int_block(block: &Block<(), i32>) -> i32;
    /// Invokes a block with `a` and returns the result.
    pub fn invoke_add_block(block: &Block<(i32,), i32>, a: i32) -> i32;

    pub fn get_large_struct_block() -> *mut Block<(LargeStruct,), LargeStruct>;
    pub fn get_large_struct_block_with(i: LargeStruct) -> *mut Block<(LargeStruct,), LargeStruct>;
    pub fn invoke_large_struct_block(
        block: &Block<(LargeStruct,), LargeStruct>,
        s: LargeStruct,
    ) -> LargeStruct;

    pub fn try_block_debugging(x: i32);
}

#[no_mangle]
extern "C" fn debug_block(block: *mut c_void) {
    let block: &Block<(), ()> = unsafe { &*(block as *const Block<(), ()>) };
    std::println!("{block:#?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_debugging() {
        unsafe { try_block_debugging(5) };
        // Uncomment to see debug output
        // panic!();
    }

    #[test]
    fn test_int_block() {
        unsafe {
            assert_eq!(invoke_int_block(&*get_int_block()), 7);
            assert_eq!(invoke_int_block(&*get_int_block_with(13)), 13);
        }
    }

    #[test]
    fn test_add_block() {
        unsafe {
            assert_eq!(invoke_add_block(&*get_add_block(), 5), 12);
            assert_eq!(invoke_add_block(&*get_add_block_with(3), 5), 8);
        }
    }

    #[test]
    fn test_large_struct_block() {
        let data = LargeStruct::get();
        let mut expected = data;
        expected.mutate();

        assert_eq!(
            unsafe { invoke_large_struct_block(&*get_large_struct_block(), data) },
            expected
        );
        assert_eq!(
            unsafe { invoke_large_struct_block(&*get_large_struct_block_with(expected), data) },
            expected
        );
    }
}
