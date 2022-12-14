use objc2::rc::{Id, Shared};
use objc2::{extern_methods, ClassType};

use crate::Foundation::{NSData, NSMutableData};

extern_methods!(
    unsafe impl NSMutableData {
        #[method_id(@__retain_semantics Other dataWithData:)]
        pub fn dataWithData(data: &NSData) -> Id<Self, Shared>;
    }
);
