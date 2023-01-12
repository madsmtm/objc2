use crate::common::*;
use crate::Foundation;

extern_methods!(
    #[cfg(feature = "Foundation_NSMutableData")]
    unsafe impl Foundation::NSMutableData {
        #[method(mutableBytes)]
        pub fn mutableBytes(&mut self) -> Option<NonNull<c_void>>;
    }
);
