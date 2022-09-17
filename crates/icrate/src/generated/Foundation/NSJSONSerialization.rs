#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSJSONSerialization;
    unsafe impl ClassType for NSJSONSerialization {
        type Super = NSObject;
    }
);
impl NSJSONSerialization {
    pub unsafe fn isValidJSONObject(obj: &Object) -> bool {
        msg_send![Self::class(), isValidJSONObject: obj]
    }
    pub unsafe fn dataWithJSONObject_options_error(
        obj: &Object,
        opt: NSJSONWritingOptions,
        error: *mut Option<&NSError>,
    ) -> Option<Id<NSData, Shared>> {
        msg_send_id![
            Self::class(),
            dataWithJSONObject: obj,
            options: opt,
            error: error
        ]
    }
    pub unsafe fn JSONObjectWithData_options_error(
        data: &NSData,
        opt: NSJSONReadingOptions,
        error: *mut Option<&NSError>,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![
            Self::class(),
            JSONObjectWithData: data,
            options: opt,
            error: error
        ]
    }
    pub unsafe fn writeJSONObject_toStream_options_error(
        obj: &Object,
        stream: &NSOutputStream,
        opt: NSJSONWritingOptions,
        error: *mut Option<&NSError>,
    ) -> NSInteger {
        msg_send![
            Self::class(),
            writeJSONObject: obj,
            toStream: stream,
            options: opt,
            error: error
        ]
    }
    pub unsafe fn JSONObjectWithStream_options_error(
        stream: &NSInputStream,
        opt: NSJSONReadingOptions,
        error: *mut Option<&NSError>,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![
            Self::class(),
            JSONObjectWithStream: stream,
            options: opt,
            error: error
        ]
    }
}
