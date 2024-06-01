//! Test of `block2`.
//!
//! This is used so that we don't need to add a `build.rs` script to `block2`.
#![no_std]
#![cfg_attr(
    all(target_vendor = "apple", feature = "unstable-simd"),
    feature(repr_simd)
)]

use std::os::raw::c_void;

use block2::Block;
extern crate alloc;
extern crate std;

#[cfg(test)]
mod block;
#[cfg(all(test, feature = "exception"))]
mod exception;
mod rc_test_object;
#[cfg(test)]
mod test_declare_class_protocol;
#[cfg(test)]
mod test_encode_utils;
#[cfg(test)]
mod test_foundation_retain_semantics;
#[cfg(test)]
mod test_object;
#[cfg(test)]
#[cfg(all(target_vendor = "apple", feature = "unstable-simd"))]
mod test_simd_return;

#[no_mangle]
extern "C" fn debug_block(block: *mut c_void) {
    let block: &Block<dyn Fn()> = unsafe { &*(block as *const Block<dyn Fn()>) };
    std::println!("{block:#?}");
}

use objc2::*;
