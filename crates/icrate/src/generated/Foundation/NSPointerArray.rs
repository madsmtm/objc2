use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSPointerFunctions::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPointerArray;
    unsafe impl ClassType for NSPointerArray {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSPointerArray {
        #[method_id(initWithOptions:)]
        pub unsafe fn initWithOptions(
            &self,
            options: NSPointerFunctionsOptions,
        ) -> Id<Self, Shared>;
        #[method_id(initWithPointerFunctions:)]
        pub unsafe fn initWithPointerFunctions(
            &self,
            functions: &NSPointerFunctions,
        ) -> Id<Self, Shared>;
        #[method_id(pointerArrayWithOptions:)]
        pub unsafe fn pointerArrayWithOptions(
            options: NSPointerFunctionsOptions,
        ) -> Id<NSPointerArray, Shared>;
        #[method_id(pointerArrayWithPointerFunctions:)]
        pub unsafe fn pointerArrayWithPointerFunctions(
            functions: &NSPointerFunctions,
        ) -> Id<NSPointerArray, Shared>;
        #[method_id(pointerFunctions)]
        pub unsafe fn pointerFunctions(&self) -> Id<NSPointerFunctions, Shared>;
        #[method(pointerAtIndex:)]
        pub unsafe fn pointerAtIndex(&self, index: NSUInteger) -> *mut c_void;
        #[method(addPointer:)]
        pub unsafe fn addPointer(&self, pointer: *mut c_void);
        #[method(removePointerAtIndex:)]
        pub unsafe fn removePointerAtIndex(&self, index: NSUInteger);
        #[method(insertPointer:atIndex:)]
        pub unsafe fn insertPointer_atIndex(&self, item: *mut c_void, index: NSUInteger);
        #[method(replacePointerAtIndex:withPointer:)]
        pub unsafe fn replacePointerAtIndex_withPointer(
            &self,
            index: NSUInteger,
            item: *mut c_void,
        );
        #[method(compact)]
        pub unsafe fn compact(&self);
        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;
        #[method(setCount:)]
        pub unsafe fn setCount(&self, count: NSUInteger);
    }
);
extern_methods!(
    #[doc = "NSPointerArrayConveniences"]
    unsafe impl NSPointerArray {
        #[method_id(pointerArrayWithStrongObjects)]
        pub unsafe fn pointerArrayWithStrongObjects() -> Id<Object, Shared>;
        #[method_id(pointerArrayWithWeakObjects)]
        pub unsafe fn pointerArrayWithWeakObjects() -> Id<Object, Shared>;
        #[method_id(strongObjectsPointerArray)]
        pub unsafe fn strongObjectsPointerArray() -> Id<NSPointerArray, Shared>;
        #[method_id(weakObjectsPointerArray)]
        pub unsafe fn weakObjectsPointerArray() -> Id<NSPointerArray, Shared>;
        #[method_id(allObjects)]
        pub unsafe fn allObjects(&self) -> Id<NSArray, Shared>;
    }
);
