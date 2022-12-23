use core::ffi::c_void;
use core::ptr::NonNull;

use objc2::extern_methods;

use crate::Foundation::NSMutableData;

extern_methods!(
    unsafe impl NSMutableData {
        #[method(mutableBytes)]
        pub fn mutableBytes(&mut self) -> Option<NonNull<c_void>>;
    }
);
