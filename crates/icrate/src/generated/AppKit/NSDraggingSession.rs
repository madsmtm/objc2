#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSDraggingSession;
    unsafe impl ClassType for NSDraggingSession {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSDraggingSession {
        #[method(draggingFormation)]
        pub unsafe fn draggingFormation(&self) -> NSDraggingFormation;
        #[method(setDraggingFormation:)]
        pub unsafe fn setDraggingFormation(&self, draggingFormation: NSDraggingFormation);
        #[method(animatesToStartingPositionsOnCancelOrFail)]
        pub unsafe fn animatesToStartingPositionsOnCancelOrFail(&self) -> bool;
        #[method(setAnimatesToStartingPositionsOnCancelOrFail:)]
        pub unsafe fn setAnimatesToStartingPositionsOnCancelOrFail(
            &self,
            animatesToStartingPositionsOnCancelOrFail: bool,
        );
        #[method(draggingLeaderIndex)]
        pub unsafe fn draggingLeaderIndex(&self) -> NSInteger;
        #[method(setDraggingLeaderIndex:)]
        pub unsafe fn setDraggingLeaderIndex(&self, draggingLeaderIndex: NSInteger);
        #[method_id(draggingPasteboard)]
        pub unsafe fn draggingPasteboard(&self) -> Id<NSPasteboard, Shared>;
        #[method(draggingSequenceNumber)]
        pub unsafe fn draggingSequenceNumber(&self) -> NSInteger;
        #[method(draggingLocation)]
        pub unsafe fn draggingLocation(&self) -> NSPoint;
        #[method(enumerateDraggingItemsWithOptions:forView:classes:searchOptions:usingBlock:)]
        pub unsafe fn enumerateDraggingItemsWithOptions_forView_classes_searchOptions_usingBlock(
            &self,
            enumOpts: NSDraggingItemEnumerationOptions,
            view: Option<&NSView>,
            classArray: &NSArray<TodoClass>,
            searchOptions: &NSDictionary<NSPasteboardReadingOptionKey, Object>,
            block: TodoBlock,
        );
    }
);
