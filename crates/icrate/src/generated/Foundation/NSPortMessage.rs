use super::__exported::NSArray;
use super::__exported::NSDate;
use super::__exported::NSMutableArray;
use super::__exported::NSPort;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPortMessage;
    unsafe impl ClassType for NSPortMessage {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSPortMessage {
        # [method_id (initWithSendPort : receivePort : components :)]
        pub unsafe fn initWithSendPort_receivePort_components(
            &self,
            sendPort: Option<&NSPort>,
            replyPort: Option<&NSPort>,
            components: Option<&NSArray>,
        ) -> Id<Self, Shared>;
        #[method_id(components)]
        pub unsafe fn components(&self) -> Option<Id<NSArray, Shared>>;
        #[method_id(receivePort)]
        pub unsafe fn receivePort(&self) -> Option<Id<NSPort, Shared>>;
        #[method_id(sendPort)]
        pub unsafe fn sendPort(&self) -> Option<Id<NSPort, Shared>>;
        # [method (sendBeforeDate :)]
        pub unsafe fn sendBeforeDate(&self, date: &NSDate) -> bool;
        #[method(msgid)]
        pub unsafe fn msgid(&self) -> u32;
        # [method (setMsgid :)]
        pub unsafe fn setMsgid(&self, msgid: u32);
    }
);
