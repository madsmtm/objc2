use crate::common::*;
use crate::CoreAnimation::*;

pub type CFTimeInterval = c_double;

extern_methods!(
    #[cfg(feature = "CoreAnimation_CATransaction")]
    unsafe impl CATransaction {
        #[method(batch)]
        #[deprecated = "removed in Xcode 15.1"]
        pub unsafe fn batch();
    }
);
