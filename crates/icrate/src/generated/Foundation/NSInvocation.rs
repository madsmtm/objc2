use super::__exported::NSMethodSignature;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSInvocation;
    unsafe impl ClassType for NSInvocation {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSInvocation {
        pub unsafe fn invocationWithMethodSignature(
            sig: &NSMethodSignature,
        ) -> Id<NSInvocation, Shared> {
            msg_send_id![Self::class(), invocationWithMethodSignature: sig]
        }
        pub unsafe fn methodSignature(&self) -> Id<NSMethodSignature, Shared> {
            msg_send_id![self, methodSignature]
        }
        pub unsafe fn retainArguments(&self) {
            msg_send![self, retainArguments]
        }
        pub unsafe fn argumentsRetained(&self) -> bool {
            msg_send![self, argumentsRetained]
        }
        pub unsafe fn target(&self) -> Option<Id<Object, Shared>> {
            msg_send_id![self, target]
        }
        pub unsafe fn setTarget(&self, target: Option<&Object>) {
            msg_send![self, setTarget: target]
        }
        pub unsafe fn selector(&self) -> Sel {
            msg_send![self, selector]
        }
        pub unsafe fn setSelector(&self, selector: Sel) {
            msg_send![self, setSelector: selector]
        }
        pub unsafe fn getReturnValue(&self, retLoc: NonNull<c_void>) {
            msg_send![self, getReturnValue: retLoc]
        }
        pub unsafe fn setReturnValue(&self, retLoc: NonNull<c_void>) {
            msg_send![self, setReturnValue: retLoc]
        }
        pub unsafe fn getArgument_atIndex(
            &self,
            argumentLocation: NonNull<c_void>,
            idx: NSInteger,
        ) {
            msg_send![self, getArgument: argumentLocation, atIndex: idx]
        }
        pub unsafe fn setArgument_atIndex(
            &self,
            argumentLocation: NonNull<c_void>,
            idx: NSInteger,
        ) {
            msg_send![self, setArgument: argumentLocation, atIndex: idx]
        }
        pub unsafe fn invoke(&self) {
            msg_send![self, invoke]
        }
        pub unsafe fn invokeWithTarget(&self, target: &Object) {
            msg_send![self, invokeWithTarget: target]
        }
    }
);
