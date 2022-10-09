use super::__exported::NSAppleEventDescriptor;
use crate::CoreServices::generated::CoreServices::*;
use crate::Foundation::generated::NSNotification::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSAppleEventManager;
    unsafe impl ClassType for NSAppleEventManager {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSAppleEventManager {
        #[method_id(sharedAppleEventManager)]
        pub unsafe fn sharedAppleEventManager() -> Id<NSAppleEventManager, Shared>;
        #[method(setEventHandler:andSelector:forEventClass:andEventID:)]
        pub unsafe fn setEventHandler_andSelector_forEventClass_andEventID(
            &self,
            handler: &Object,
            handleEventSelector: Sel,
            eventClass: AEEventClass,
            eventID: AEEventID,
        );
        #[method(removeEventHandlerForEventClass:andEventID:)]
        pub unsafe fn removeEventHandlerForEventClass_andEventID(
            &self,
            eventClass: AEEventClass,
            eventID: AEEventID,
        );
        #[method(dispatchRawAppleEvent:withRawReply:handlerRefCon:)]
        pub unsafe fn dispatchRawAppleEvent_withRawReply_handlerRefCon(
            &self,
            theAppleEvent: NonNull<AppleEvent>,
            theReply: NonNull<AppleEvent>,
            handlerRefCon: SRefCon,
        ) -> OSErr;
        #[method_id(currentAppleEvent)]
        pub unsafe fn currentAppleEvent(&self) -> Option<Id<NSAppleEventDescriptor, Shared>>;
        #[method_id(currentReplyAppleEvent)]
        pub unsafe fn currentReplyAppleEvent(&self) -> Option<Id<NSAppleEventDescriptor, Shared>>;
        #[method(suspendCurrentAppleEvent)]
        pub unsafe fn suspendCurrentAppleEvent(&self) -> NSAppleEventManagerSuspensionID;
        #[method_id(appleEventForSuspensionID:)]
        pub unsafe fn appleEventForSuspensionID(
            &self,
            suspensionID: NSAppleEventManagerSuspensionID,
        ) -> Id<NSAppleEventDescriptor, Shared>;
        #[method_id(replyAppleEventForSuspensionID:)]
        pub unsafe fn replyAppleEventForSuspensionID(
            &self,
            suspensionID: NSAppleEventManagerSuspensionID,
        ) -> Id<NSAppleEventDescriptor, Shared>;
        #[method(setCurrentAppleEventAndReplyEventWithSuspensionID:)]
        pub unsafe fn setCurrentAppleEventAndReplyEventWithSuspensionID(
            &self,
            suspensionID: NSAppleEventManagerSuspensionID,
        );
        #[method(resumeWithSuspensionID:)]
        pub unsafe fn resumeWithSuspensionID(&self, suspensionID: NSAppleEventManagerSuspensionID);
    }
);
