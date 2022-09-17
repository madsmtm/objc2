#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    struct NSAppleScript;
    unsafe impl ClassType for NSAppleScript {
        type Super = NSObject;
    }
);
impl NSAppleScript {
    pub unsafe fn initWithContentsOfURL_error(
        &self,
        url: &NSURL,
        errorInfo: *mut TodoGenerics,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithContentsOfURL: url, error: errorInfo]
    }
    pub unsafe fn initWithSource(&self, source: &NSString) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithSource: source]
    }
    pub unsafe fn compileAndReturnError(&self, errorInfo: *mut TodoGenerics) -> bool {
        msg_send![self, compileAndReturnError: errorInfo]
    }
    pub unsafe fn executeAndReturnError(
        &self,
        errorInfo: *mut TodoGenerics,
    ) -> Id<NSAppleEventDescriptor, Shared> {
        msg_send_id![self, executeAndReturnError: errorInfo]
    }
    pub unsafe fn executeAppleEvent_error(
        &self,
        event: &NSAppleEventDescriptor,
        errorInfo: *mut TodoGenerics,
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
