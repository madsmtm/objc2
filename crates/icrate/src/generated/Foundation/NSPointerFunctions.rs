//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};

pub type NSPointerFunctionsOptions = NSUInteger;
pub const NSPointerFunctionsStrongMemory: NSPointerFunctionsOptions = (0 << 0);
pub const NSPointerFunctionsZeroingWeakMemory: NSPointerFunctionsOptions = (1 << 0);
pub const NSPointerFunctionsOpaqueMemory: NSPointerFunctionsOptions = (2 << 0);
pub const NSPointerFunctionsMallocMemory: NSPointerFunctionsOptions = (3 << 0);
pub const NSPointerFunctionsMachVirtualMemory: NSPointerFunctionsOptions = (4 << 0);
pub const NSPointerFunctionsWeakMemory: NSPointerFunctionsOptions = (5 << 0);
pub const NSPointerFunctionsObjectPersonality: NSPointerFunctionsOptions = (0 << 8);
pub const NSPointerFunctionsOpaquePersonality: NSPointerFunctionsOptions = (1 << 8);
pub const NSPointerFunctionsObjectPointerPersonality: NSPointerFunctionsOptions = (2 << 8);
pub const NSPointerFunctionsCStringPersonality: NSPointerFunctionsOptions = (3 << 8);
pub const NSPointerFunctionsStructPersonality: NSPointerFunctionsOptions = (4 << 8);
pub const NSPointerFunctionsIntegerPersonality: NSPointerFunctionsOptions = (5 << 8);
pub const NSPointerFunctionsCopyIn: NSPointerFunctionsOptions = (1 << 16);

extern_class!(
    #[derive(Debug)]
    pub struct NSPointerFunctions;

    unsafe impl ClassType for NSPointerFunctions {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSPointerFunctions {
        #[method_id(initWithOptions:)]
        pub unsafe fn initWithOptions(
            &self,
            options: NSPointerFunctionsOptions,
        ) -> Id<Self, Shared>;

        #[method_id(pointerFunctionsWithOptions:)]
        pub unsafe fn pointerFunctionsWithOptions(
            options: NSPointerFunctionsOptions,
        ) -> Id<NSPointerFunctions, Shared>;

        #[method(hashFunction)]
        pub unsafe fn hashFunction(&self) -> *mut TodoFunction;

        #[method(setHashFunction:)]
        pub unsafe fn setHashFunction(&self, hashFunction: *mut TodoFunction);

        #[method(isEqualFunction)]
        pub unsafe fn isEqualFunction(&self) -> *mut TodoFunction;

        #[method(setIsEqualFunction:)]
        pub unsafe fn setIsEqualFunction(&self, isEqualFunction: *mut TodoFunction);

        #[method(sizeFunction)]
        pub unsafe fn sizeFunction(&self) -> *mut TodoFunction;

        #[method(setSizeFunction:)]
        pub unsafe fn setSizeFunction(&self, sizeFunction: *mut TodoFunction);

        #[method(descriptionFunction)]
        pub unsafe fn descriptionFunction(&self) -> *mut TodoFunction;

        #[method(setDescriptionFunction:)]
        pub unsafe fn setDescriptionFunction(&self, descriptionFunction: *mut TodoFunction);

        #[method(relinquishFunction)]
        pub unsafe fn relinquishFunction(&self) -> *mut TodoFunction;

        #[method(setRelinquishFunction:)]
        pub unsafe fn setRelinquishFunction(&self, relinquishFunction: *mut TodoFunction);

        #[method(acquireFunction)]
        pub unsafe fn acquireFunction(&self) -> *mut TodoFunction;

        #[method(setAcquireFunction:)]
        pub unsafe fn setAcquireFunction(&self, acquireFunction: *mut TodoFunction);

        #[method(usesStrongWriteBarrier)]
        pub unsafe fn usesStrongWriteBarrier(&self) -> bool;

        #[method(setUsesStrongWriteBarrier:)]
        pub unsafe fn setUsesStrongWriteBarrier(&self, usesStrongWriteBarrier: bool);

        #[method(usesWeakReadAndWriteBarriers)]
        pub unsafe fn usesWeakReadAndWriteBarriers(&self) -> bool;

        #[method(setUsesWeakReadAndWriteBarriers:)]
        pub unsafe fn setUsesWeakReadAndWriteBarriers(&self, usesWeakReadAndWriteBarriers: bool);
    }
);
