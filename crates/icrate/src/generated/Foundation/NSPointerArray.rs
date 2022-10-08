use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSPointerFunctions::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPointerArray;
    unsafe impl ClassType for NSPointerArray {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSPointerArray {
        pub unsafe fn initWithOptions(
            &self,
            options: NSPointerFunctionsOptions,
        ) -> Id<Self, Shared> {
            msg_send_id![self, initWithOptions: options]
        }
        pub unsafe fn initWithPointerFunctions(
            &self,
            functions: &NSPointerFunctions,
        ) -> Id<Self, Shared> {
            msg_send_id![self, initWithPointerFunctions: functions]
        }
        pub unsafe fn pointerArrayWithOptions(
            options: NSPointerFunctionsOptions,
        ) -> Id<NSPointerArray, Shared> {
            msg_send_id![Self::class(), pointerArrayWithOptions: options]
        }
        pub unsafe fn pointerArrayWithPointerFunctions(
            functions: &NSPointerFunctions,
        ) -> Id<NSPointerArray, Shared> {
            msg_send_id![Self::class(), pointerArrayWithPointerFunctions: functions]
        }
        pub unsafe fn pointerFunctions(&self) -> Id<NSPointerFunctions, Shared> {
            msg_send_id![self, pointerFunctions]
        }
        pub unsafe fn pointerAtIndex(&self, index: NSUInteger) -> *mut c_void {
            msg_send![self, pointerAtIndex: index]
        }
        pub unsafe fn addPointer(&self, pointer: *mut c_void) {
            msg_send![self, addPointer: pointer]
        }
        pub unsafe fn removePointerAtIndex(&self, index: NSUInteger) {
            msg_send![self, removePointerAtIndex: index]
        }
        pub unsafe fn insertPointer_atIndex(&self, item: *mut c_void, index: NSUInteger) {
            msg_send![self, insertPointer: item, atIndex: index]
        }
        pub unsafe fn replacePointerAtIndex_withPointer(
            &self,
            index: NSUInteger,
            item: *mut c_void,
        ) {
            msg_send![self, replacePointerAtIndex: index, withPointer: item]
        }
        pub unsafe fn compact(&self) {
            msg_send![self, compact]
        }
        pub unsafe fn count(&self) -> NSUInteger {
            msg_send![self, count]
        }
        pub unsafe fn setCount(&self, count: NSUInteger) {
            msg_send![self, setCount: count]
        }
    }
);
extern_methods!(
    #[doc = "NSPointerArrayConveniences"]
    unsafe impl NSPointerArray {
        pub unsafe fn pointerArrayWithStrongObjects() -> Id<Object, Shared> {
            msg_send_id![Self::class(), pointerArrayWithStrongObjects]
        }
        pub unsafe fn pointerArrayWithWeakObjects() -> Id<Object, Shared> {
            msg_send_id![Self::class(), pointerArrayWithWeakObjects]
        }
        pub unsafe fn strongObjectsPointerArray() -> Id<NSPointerArray, Shared> {
            msg_send_id![Self::class(), strongObjectsPointerArray]
        }
        pub unsafe fn weakObjectsPointerArray() -> Id<NSPointerArray, Shared> {
            msg_send_id![Self::class(), weakObjectsPointerArray]
        }
        pub unsafe fn allObjects(&self) -> Id<NSArray, Shared> {
            msg_send_id![self, allObjects]
        }
    }
);
