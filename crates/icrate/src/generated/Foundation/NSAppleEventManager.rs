use super::NSAppleEventDescriptor;
use crate::CoreServices::generated::CoreServices::*;
use crate::Foundation::generated::NSNotification::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSAppleEventManager;
    unsafe impl ClassType for NSAppleEventManager {
        type Super = NSObject;
    }
);
impl NSAppleEventManager {
    pub unsafe fn sharedAppleEventManager() -> Id<NSAppleEventManager, Shared> {
        msg_send_id![Self::class(), sharedAppleEventManager]
    }
    pub unsafe fn setEventHandler_andSelector_forEventClass_andEventID(
        &self,
        handler: &Object,
        handleEventSelector: Sel,
        eventClass: AEEventClass,
        eventID: AEEventID,
    ) {
        msg_send![
            self,
            setEventHandler: handler,
            andSelector: handleEventSelector,
            forEventClass: eventClass,
            andEventID: eventID
        ]
    }
    pub unsafe fn removeEventHandlerForEventClass_andEventID(
        &self,
        eventClass: AEEventClass,
        eventID: AEEventID,
    ) {
        msg_send![
            self,
            removeEventHandlerForEventClass: eventClass,
            andEventID: eventID
        ]
    }
    pub unsafe fn dispatchRawAppleEvent_withRawReply_handlerRefCon(
        &self,
        theAppleEvent: NonNull<AppleEvent>,
        theReply: NonNull<AppleEvent>,
        handlerRefCon: SRefCon,
    ) -> OSErr {
        msg_send![
            self,
            dispatchRawAppleEvent: theAppleEvent,
            withRawReply: theReply,
            handlerRefCon: handlerRefCon
        ]
    }
    pub unsafe fn suspendCurrentAppleEvent(&self) -> NSAppleEventManagerSuspensionID {
        msg_send![self, suspendCurrentAppleEvent]
    }
    pub unsafe fn appleEventForSuspensionID(
        &self,
        suspensionID: NSAppleEventManagerSuspensionID,
    ) -> Id<NSAppleEventDescriptor, Shared> {
        msg_send_id![self, appleEventForSuspensionID: suspensionID]
    }
    pub unsafe fn replyAppleEventForSuspensionID(
        &self,
        suspensionID: NSAppleEventManagerSuspensionID,
    ) -> Id<NSAppleEventDescriptor, Shared> {
        msg_send_id![self, replyAppleEventForSuspensionID: suspensionID]
    }
    pub unsafe fn setCurrentAppleEventAndReplyEventWithSuspensionID(
        &self,
        suspensionID: NSAppleEventManagerSuspensionID,
    ) {
        msg_send![
            self,
            setCurrentAppleEventAndReplyEventWithSuspensionID: suspensionID
        ]
    }
    pub unsafe fn resumeWithSuspensionID(&self, suspensionID: NSAppleEventManagerSuspensionID) {
        msg_send![self, resumeWithSuspensionID: suspensionID]
    }
    pub unsafe fn currentAppleEvent(&self) -> Option<Id<NSAppleEventDescriptor, Shared>> {
        msg_send_id![self, currentAppleEvent]
    }
    pub unsafe fn currentReplyAppleEvent(&self) -> Option<Id<NSAppleEventDescriptor, Shared>> {
        msg_send_id![self, currentReplyAppleEvent]
    }
}
