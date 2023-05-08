#![cfg(feature = "Foundation_NSData")]
use crate::common::*;
use crate::Foundation::NSData;

extern_methods!(
    unsafe impl NSData {
        #[method(bytes)]
        pub(crate) fn bytes_raw(&self) -> *const c_void;
    }
);
