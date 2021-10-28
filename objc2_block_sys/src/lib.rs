//!
//! See:
//! - https://github.com/apple/swift-corelibs-libdispatch/tree/main/src/BlocksRuntime
//! - https://clang.llvm.org/docs/BlockLanguageSpec.html
//! - https://clang.llvm.org/docs/Block-ABI-Apple.html

// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2_block_sys/0.0.0")]

use core::ffi::c_void;

#[repr(C)]
pub struct Class {
    #[cfg(any(feature = "apple", feature = "compiler-rt"))]
    _priv: [*mut c_void; 32],
    #[cfg(any(feature = "gnustep-1-7", feature = "objfw"))]
    // The size of this is unknown
    _priv: [u8; 0],
}

extern "C" {
    // the raw data space for runtime classes for blocks
    // class+meta used for stack, malloc, and collectable based blocks

    pub static _NSConcreteGlobalBlock: Class;
    pub static _NSConcreteStackBlock: Class;
    pub static _NSConcreteMallocBlock: Class;

    pub fn _Block_copy(block: *const c_void) -> *mut c_void;
    pub fn _Block_release(block: *const c_void);

    /// Runtime entry point called by compiler when assigning objects inside
    /// copy helper routines
    pub fn _Block_object_assign(dest_addr: *mut c_void, object: *const c_void, flags: i32);

    /// runtime entry point called by the compiler when disposing of objects
    /// inside dispose helper routine
    pub fn _Block_object_dispose(object: *const c_void, flags: i32);
}
