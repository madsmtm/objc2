//! Test of `block2`.
//!
//! This is used so that we don't need to add a `build.rs` script to `block2`.
#![no_std]
#![cfg_attr(
    all(target_vendor = "apple", feature = "unstable-simd"),
    feature(repr_simd)
)]

use core::ffi::c_void;

use block2::Block;
extern crate alloc;
extern crate std;

#[cfg(test)]
mod backtrace;
#[cfg(test)]
mod block;
#[cfg(all(test, feature = "exception"))]
mod exception;
mod rc_test_object;
#[cfg(test)]
mod test_define_class_protocol;
#[cfg(test)]
mod test_encode_utils;
#[cfg(test)]
mod test_foundation_retain_semantics;
#[cfg(test)]
mod test_object;
#[cfg(test)]
#[cfg(all(target_vendor = "apple", feature = "unstable-simd"))]
mod test_simd_return;

// Run some `objc2` doctests that require extra crates.
#[cfg(target_vendor = "apple")]
#[allow(rustdoc::broken_intra_doc_links)]
#[path = "../../objc2/src/topics/mod.rs"]
pub mod objc2_topics;

#[no_mangle]
extern "C-unwind" fn debug_block(block: *mut c_void) {
    let block: &Block<dyn Fn()> = unsafe { &*(block as *const Block<dyn Fn()>) };
    std::println!("{block:#?}");
}

use objc2::*;
