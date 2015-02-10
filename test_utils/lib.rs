#![crate_name = "objc_test_utils"]
#![crate_type = "lib"]

#[link(name = "Foundation", kind = "framework")]
extern { }

/// A block that takes no arguments and returns an integer: `int32_t (^)()`.
pub enum IntBlock { }

/// A block that takes one integer argument, adds to it, and returns the sum:
/// `int32_t (^)(int32_t)`.
pub enum AddBlock { }

#[link(name="block_utils", kind="static")]
extern {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_block() {
        unsafe {
            assert!(invoke_int_block(get_int_block()) == 7);
            assert!(invoke_int_block(get_int_block_with(13)) == 13);
        }
    }

    #[test]
    fn test_add_block() {
        unsafe {
            assert!(invoke_add_block(get_add_block(), 5) == 12);
            assert!(invoke_add_block(get_add_block_with(3), 5) == 8);
        }
    }
}
