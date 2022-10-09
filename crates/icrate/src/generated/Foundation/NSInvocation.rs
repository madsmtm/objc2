use super::__exported::NSMethodSignature;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSInvocation;
    unsafe impl ClassType for NSInvocation {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSInvocation {
        #[method_id(invocationWithMethodSignature:)]
        pub unsafe fn invocationWithMethodSignature(
            sig: &NSMethodSignature,
        ) -> Id<NSInvocation, Shared>;
        #[method_id(methodSignature)]
        pub unsafe fn methodSignature(&self) -> Id<NSMethodSignature, Shared>;
        #[method(retainArguments)]
        pub unsafe fn retainArguments(&self);
        #[method(argumentsRetained)]
        pub unsafe fn argumentsRetained(&self) -> bool;
        #[method_id(target)]
        pub unsafe fn target(&self) -> Option<Id<Object, Shared>>;
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&Object>);
        #[method(selector)]
        pub unsafe fn selector(&self) -> Sel;
        #[method(setSelector:)]
        pub unsafe fn setSelector(&self, selector: Sel);
        #[method(getReturnValue:)]
        pub unsafe fn getReturnValue(&self, retLoc: NonNull<c_void>);
        #[method(setReturnValue:)]
        pub unsafe fn setReturnValue(&self, retLoc: NonNull<c_void>);
        #[method(getArgument:atIndex:)]
        pub unsafe fn getArgument_atIndex(&self, argumentLocation: NonNull<c_void>, idx: NSInteger);
        #[method(setArgument:atIndex:)]
        pub unsafe fn setArgument_atIndex(&self, argumentLocation: NonNull<c_void>, idx: NSInteger);
        #[method(invoke)]
        pub unsafe fn invoke(&self);
        #[method(invokeWithTarget:)]
        pub unsafe fn invokeWithTarget(&self, target: &Object);
    }
);
