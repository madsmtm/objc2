use crate::common::*;
use crate::Foundation;

extern_methods!(
    #[cfg(feature = "Foundation_NSData")]
    unsafe impl Foundation::NSData {
        #[method(bytes)]
        pub(crate) fn bytes_raw(&self) -> Option<NonNull<c_void>>;
    }

    #[cfg(feature = "Foundation_NSMutableData")]
    unsafe impl Foundation::NSMutableData {
        #[method(mutableBytes)]
        pub(crate) fn mutable_bytes_raw(&mut self) -> Option<NonNull<c_void>>;
    }
);
