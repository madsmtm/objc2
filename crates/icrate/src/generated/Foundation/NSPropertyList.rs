use super::__exported::NSData;
use super::__exported::NSError;
use super::__exported::NSInputStream;
use super::__exported::NSOutputStream;
use super::__exported::NSString;
use crate::CoreFoundation::generated::CFPropertyList::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
pub type NSPropertyListReadOptions = NSPropertyListMutabilityOptions;
pub type NSPropertyListWriteOptions = NSUInteger;
extern_class!(
    #[derive(Debug)]
    pub struct NSPropertyListSerialization;
    unsafe impl ClassType for NSPropertyListSerialization {
        type Super = NSObject;
    }
);
impl NSPropertyListSerialization {
    pub unsafe fn propertyList_isValidForFormat(
        plist: &Object,
        format: NSPropertyListFormat,
    ) -> bool {
        msg_send![Self::class(), propertyList: plist, isValidForFormat: format]
    }
    pub unsafe fn dataWithPropertyList_format_options_error(
        plist: &Object,
        format: NSPropertyListFormat,
        opt: NSPropertyListWriteOptions,
        error: *mut *mut NSError,
    ) -> Option<Id<NSData, Shared>> {
        msg_send_id![
            Self::class(),
            dataWithPropertyList: plist,
            format: format,
            options: opt,
            error: error
        ]
    }
    pub unsafe fn writePropertyList_toStream_format_options_error(
        plist: &Object,
        stream: &NSOutputStream,
        format: NSPropertyListFormat,
        opt: NSPropertyListWriteOptions,
        error: *mut *mut NSError,
    ) -> NSInteger {
        msg_send![
            Self::class(),
            writePropertyList: plist,
            toStream: stream,
            format: format,
            options: opt,
            error: error
        ]
    }
    pub unsafe fn propertyListWithData_options_format_error(
        data: &NSData,
        opt: NSPropertyListReadOptions,
        format: *mut NSPropertyListFormat,
        error: *mut *mut NSError,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![
            Self::class(),
            propertyListWithData: data,
            options: opt,
            format: format,
            error: error
        ]
    }
    pub unsafe fn propertyListWithStream_options_format_error(
        stream: &NSInputStream,
        opt: NSPropertyListReadOptions,
        format: *mut NSPropertyListFormat,
        error: *mut *mut NSError,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![
            Self::class(),
            propertyListWithStream: stream,
            options: opt,
            format: format,
            error: error
        ]
    }
}
