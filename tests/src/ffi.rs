use objc2_encode::{Encode, Encoding};

/// A block that takes no arguments and returns an integer: `int32_t (^)()`.
#[repr(C)]
pub struct IntBlock {
    _priv: [u8; 0],
}

/// A block that takes one integer argument, adds to it, and returns the sum:
/// `int32_t (^)(int32_t)`.
#[repr(C)]
pub struct AddBlock {
    _priv: [u8; 0],
}

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
    const ENCODING: Encoding<'static> =
        Encoding::Struct("LargeStruct", &[f32::ENCODING, <[u8; 100]>::ENCODING]);
}

#[repr(C)]
pub struct LargeStructBlock {
    _priv: [u8; 0],
}

extern "C" {
    /// Returns a pointer to a global `IntBlock` that returns 7.
    pub fn get_int_block() -> *mut IntBlock;
    /// Returns a pointer to a copied `IntBlock` that returns `i`.
    pub fn get_int_block_with(i: i32) -> *mut IntBlock;
    /// Returns a pointer to a global `AddBlock` that returns its argument + 7.
    pub fn get_add_block() -> *mut AddBlock;
    /// Returns a pointer to a copied `AddBlock` that returns its argument + `i`.
    pub fn get_add_block_with(i: i32) -> *mut AddBlock;
    /// Invokes an `IntBlock` and returns its result.
    pub fn invoke_int_block(block: *mut IntBlock) -> i32;
    /// Invokes an `AddBlock` with `a` and returns the result.
    pub fn invoke_add_block(block: *mut AddBlock, a: i32) -> i32;

    pub fn get_large_struct_block() -> *mut LargeStructBlock;
    pub fn get_large_struct_block_with(i: LargeStruct) -> *mut LargeStructBlock;
    pub fn invoke_large_struct_block(block: *mut LargeStructBlock, s: LargeStruct) -> LargeStruct;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_block() {
        unsafe {
            assert_eq!(invoke_int_block(get_int_block()), 7);
            assert_eq!(invoke_int_block(get_int_block_with(13)), 13);
        }
    }

    #[test]
    fn test_add_block() {
        unsafe {
            assert_eq!(invoke_add_block(get_add_block(), 5), 12);
            assert_eq!(invoke_add_block(get_add_block_with(3), 5), 8);
        }
    }

    #[test]
    fn test_large_struct_block() {
        let data = LargeStruct::get();
        let mut expected = data.clone();
        expected.mutate();

        assert_eq!(
            unsafe { invoke_large_struct_block(get_large_struct_block(), data) },
            expected
        );
        assert_eq!(
            unsafe { invoke_large_struct_block(get_large_struct_block_with(expected), data) },
            expected
        );
    }
}
