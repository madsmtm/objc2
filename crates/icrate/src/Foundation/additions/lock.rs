#![cfg(feature = "Foundation_NSLock")]
use objc2::ConformsTo;

use crate::common::*;
use crate::Foundation::{NSLock, NSLocking};

// TODO: Proper Send/Sync impls here

unsafe impl ConformsTo<NSLocking> for NSLock {}

extern_methods!(
    unsafe impl NSLock {
        #[method_id(new)]
        pub fn new() -> Id<Self, Owned>;
    }
);
