#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSAutoreleasePool;
    unsafe impl ClassType for NSAutoreleasePool {
        type Super = NSObject;
    }
);
impl NSAutoreleasePool {
    pub unsafe fn addObject(anObject: &Object) {
        msg_send![Self::class(), addObject: anObject]
    }
    pub unsafe fn addObject(&self, anObject: &Object) {
        msg_send![self, addObject: anObject]
    }
    pub unsafe fn drain(&self) {
        msg_send![self, drain]
    }
}
