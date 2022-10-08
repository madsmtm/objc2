use super::__exported::NSInvocation;
use super::__exported::NSMethodSignature;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSProxy;
    unsafe impl ClassType for NSProxy {
        type Super = Object;
    }
);
extern_methods!(
    unsafe impl NSProxy {
        pub unsafe fn alloc() -> Id<Object, Shared> {
            msg_send_id![Self::class(), alloc]
        }
        pub unsafe fn allocWithZone(zone: *mut NSZone) -> Id<Object, Shared> {
            msg_send_id![Self::class(), allocWithZone: zone]
        }
        pub unsafe fn class() -> &Class {
            msg_send![Self::class(), class]
        }
        pub unsafe fn forwardInvocation(&self, invocation: &NSInvocation) {
            msg_send![self, forwardInvocation: invocation]
        }
        pub unsafe fn methodSignatureForSelector(
            &self,
            sel: Sel,
        ) -> Option<Id<NSMethodSignature, Shared>> {
            msg_send_id![self, methodSignatureForSelector: sel]
        }
        pub unsafe fn dealloc(&self) {
            msg_send![self, dealloc]
        }
        pub unsafe fn finalize(&self) {
            msg_send![self, finalize]
        }
        pub unsafe fn description(&self) -> Id<NSString, Shared> {
            msg_send_id![self, description]
        }
        pub unsafe fn debugDescription(&self) -> Id<NSString, Shared> {
            msg_send_id![self, debugDescription]
        }
        pub unsafe fn respondsToSelector(aSelector: Sel) -> bool {
            msg_send![Self::class(), respondsToSelector: aSelector]
        }
        pub unsafe fn allowsWeakReference(&self) -> bool {
            msg_send![self, allowsWeakReference]
        }
        pub unsafe fn retainWeakReference(&self) -> bool {
            msg_send![self, retainWeakReference]
        }
    }
);
