//! Smart pointers for Objective-C reference counting.

// This crate is, but its dependencies are not
#![no_std]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2_id/0.1.1")]

extern crate alloc;

pub use id::{Id, Owned, Ownership, ShareId, Shared, WeakId};

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
extern "C" {}

mod id;

// TODO: Remove the need for this hack

#[cfg(not(target_vendor = "apple"))]
use objc2::runtime::Class;

#[cfg(not(target_vendor = "apple"))]
#[link(name = "gnustep-base", kind = "dylib")]
extern "C" {}

#[cfg(not(target_vendor = "apple"))]
extern "C" {
    static _OBJC_CLASS_NSObject: Class;
}

#[cfg(not(target_vendor = "apple"))]
#[allow(dead_code)]
unsafe fn get_class_to_force_linkage() -> &'static Class {
    &_OBJC_CLASS_NSObject
}
