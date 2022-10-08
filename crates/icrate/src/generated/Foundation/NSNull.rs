use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSNull;
    unsafe impl ClassType for NSNull {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSNull {
        pub unsafe fn null() -> Id<NSNull, Shared> {
            msg_send_id![Self::class(), null]
        }
    }
);
