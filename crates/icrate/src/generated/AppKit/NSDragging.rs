#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSDraggingInfo = NSObject;
pub type NSDraggingDestination = NSObject;
pub type NSDraggingSource = NSObject;
pub type NSSpringLoadingDestination = NSObject;
extern_methods!(
    #[doc = "NSDraggingSourceDeprecated"]
    unsafe impl NSObject {
        #[method_id(namesOfPromisedFilesDroppedAtDestination:)]
        pub unsafe fn namesOfPromisedFilesDroppedAtDestination(
            &self,
            dropDestination: &NSURL,
        ) -> Option<Id<NSArray<NSString>, Shared>>;
        #[method(draggingSourceOperationMaskForLocal:)]
        pub unsafe fn draggingSourceOperationMaskForLocal(&self, flag: bool) -> NSDragOperation;
        #[method(draggedImage:beganAt:)]
        pub unsafe fn draggedImage_beganAt(&self, image: Option<&NSImage>, screenPoint: NSPoint);
        #[method(draggedImage:endedAt:operation:)]
        pub unsafe fn draggedImage_endedAt_operation(
            &self,
            image: Option<&NSImage>,
            screenPoint: NSPoint,
            operation: NSDragOperation,
        );
        #[method(draggedImage:movedTo:)]
        pub unsafe fn draggedImage_movedTo(&self, image: Option<&NSImage>, screenPoint: NSPoint);
        #[method(ignoreModifierKeysWhileDragging)]
        pub unsafe fn ignoreModifierKeysWhileDragging(&self) -> bool;
        #[method(draggedImage:endedAt:deposited:)]
        pub unsafe fn draggedImage_endedAt_deposited(
            &self,
            image: Option<&NSImage>,
            screenPoint: NSPoint,
            flag: bool,
        );
    }
);