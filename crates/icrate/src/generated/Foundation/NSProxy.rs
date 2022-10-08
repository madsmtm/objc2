use super::__exported::NSInvocation;
use super::__exported::NSMethodSignature;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSProxy;
    unsafe impl ClassType for NSProxy {
        type Super = Object;
    }
);
extern_methods!(
    unsafe impl NSProxy {
        #[method_id(alloc)]
        pub unsafe fn alloc() -> Id<Object, Shared>;
        # [method_id (allocWithZone :)]
        pub unsafe fn allocWithZone(zone: *mut NSZone) -> Id<Object, Shared>;
        #[method(class)]
        pub unsafe fn class() -> &Class;
        # [method (forwardInvocation :)]
        pub unsafe fn forwardInvocation(&self, invocation: &NSInvocation);
        # [method_id (methodSignatureForSelector :)]
        pub unsafe fn methodSignatureForSelector(
            &self,
            sel: Sel,
        ) -> Option<Id<NSMethodSignature, Shared>>;
        #[method(dealloc)]
        pub unsafe fn dealloc(&self);
        #[method(finalize)]
        pub unsafe fn finalize(&self);
        #[method_id(description)]
        pub unsafe fn description(&self) -> Id<NSString, Shared>;
        #[method_id(debugDescription)]
        pub unsafe fn debugDescription(&self) -> Id<NSString, Shared>;
        # [method (respondsToSelector :)]
        pub unsafe fn respondsToSelector(aSelector: Sel) -> bool;
        #[method(allowsWeakReference)]
        pub unsafe fn allowsWeakReference(&self) -> bool;
        #[method(retainWeakReference)]
        pub unsafe fn retainWeakReference(&self) -> bool;
    }
);
