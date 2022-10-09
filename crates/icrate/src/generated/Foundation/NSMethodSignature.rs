use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSMethodSignature;
    unsafe impl ClassType for NSMethodSignature {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSMethodSignature {
        #[method_id(signatureWithObjCTypes:)]
        pub unsafe fn signatureWithObjCTypes(
            types: NonNull<c_char>,
        ) -> Option<Id<NSMethodSignature, Shared>>;
        #[method(numberOfArguments)]
        pub unsafe fn numberOfArguments(&self) -> NSUInteger;
        #[method(getArgumentTypeAtIndex:)]
        pub unsafe fn getArgumentTypeAtIndex(&self, idx: NSUInteger) -> NonNull<c_char>;
        #[method(frameLength)]
        pub unsafe fn frameLength(&self) -> NSUInteger;
        #[method(isOneway)]
        pub unsafe fn isOneway(&self) -> bool;
        #[method(methodReturnType)]
        pub unsafe fn methodReturnType(&self) -> NonNull<c_char>;
        #[method(methodReturnLength)]
        pub unsafe fn methodReturnLength(&self) -> NSUInteger;
    }
);
