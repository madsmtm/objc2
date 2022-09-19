use super::__exported::NSAppleEventDescriptor;
use super::__exported::NSDictionary;
use super::__exported::NSString;
use super::__exported::NSURL;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSAppleScript;
    unsafe impl ClassType for NSAppleScript {
        type Super = NSObject;
    }
);
impl NSAppleScript {
    pub unsafe fn initWithContentsOfURL_error(
        &self,
        url: &NSURL,
        errorInfo: *mut *mut NSDictionary,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithContentsOfURL: url, error: errorInfo]
    }
    pub unsafe fn initWithSource(&self, source: &NSString) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithSource: source]
    }
    pub unsafe fn compileAndReturnError(&self, errorInfo: *mut *mut NSDictionary) -> bool {
        msg_send![self, compileAndReturnError: errorInfo]
    }
    pub unsafe fn executeAndReturnError(
        &self,
        errorInfo: *mut *mut NSDictionary,
    ) -> Id<NSAppleEventDescriptor, Shared> {
        msg_send_id![self, executeAndReturnError: errorInfo]
    }
    pub unsafe fn executeAppleEvent_error(
        &self,
        event: &NSAppleEventDescriptor,
        errorInfo: *mut *mut NSDictionary,
    ) -> Id<NSAppleEventDescriptor, Shared> {
        msg_send_id![self, executeAppleEvent: event, error: errorInfo]
    }
    pub unsafe fn source(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, source]
    }
    pub unsafe fn isCompiled(&self) -> bool {
        msg_send![self, isCompiled]
    }
}
