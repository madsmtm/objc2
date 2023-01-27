#![cfg(feature = "Foundation_NSLock")]
use crate::common::*;
use crate::Foundation::NSLock;

// TODO: Proper Send/Sync impls here

extern_methods!(
    unsafe impl NSLock {
        #[method_id(new)]
        pub fn new() -> Id<Self, Owned>;
    }
);
