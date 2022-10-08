use super::__exported::NSData;
use super::__exported::NSError;
use super::__exported::NSInputStream;
use super::__exported::NSOutputStream;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSJSONSerialization;
    unsafe impl ClassType for NSJSONSerialization {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSJSONSerialization {
        pub unsafe fn isValidJSONObject(obj: &Object) -> bool {
            msg_send![Self::class(), isValidJSONObject: obj]
        }
        pub unsafe fn dataWithJSONObject_options_error(
            obj: &Object,
            opt: NSJSONWritingOptions,
        ) -> Result<Id<NSData, Shared>, Id<NSError, Shared>> {
            msg_send_id![
                Self::class(),
                dataWithJSONObject: obj,
                options: opt,
                error: _
            ]
        }
        pub unsafe fn JSONObjectWithData_options_error(
            data: &NSData,
            opt: NSJSONReadingOptions,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>> {
            msg_send_id![
                Self::class(),
                JSONObjectWithData: data,
                options: opt,
                error: _
            ]
        }
        pub unsafe fn JSONObjectWithStream_options_error(
            stream: &NSInputStream,
            opt: NSJSONReadingOptions,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>> {
            msg_send_id![
                Self::class(),
                JSONObjectWithStream: stream,
                options: opt,
                error: _
            ]
        }
    }
);
