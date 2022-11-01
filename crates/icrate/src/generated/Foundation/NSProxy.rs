//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

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
        pub unsafe fn alloc() -> Option<Allocated<Object>>;

        #[method_id(allocWithZone:)]
        pub unsafe fn allocWithZone(zone: *mut NSZone) -> Option<Allocated<Object>>;

        #[method(class)]
        pub unsafe fn class() -> &'static Class;

        #[method(forwardInvocation:)]
        pub unsafe fn forwardInvocation(&self, invocation: &NSInvocation);

        #[method_id(methodSignatureForSelector:)]
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

        #[method(respondsToSelector:)]
        pub unsafe fn respondsToSelector(aSelector: Sel) -> bool;

        #[method(allowsWeakReference)]
        pub unsafe fn allowsWeakReference(&self) -> bool;

        #[method(retainWeakReference)]
        pub unsafe fn retainWeakReference(&self) -> bool;
    }
);
