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
use objc2::{extern_class, extern_methods, msg_send, msg_send_id, ClassType};
pub type NSPropertyListReadOptions = NSPropertyListMutabilityOptions;
pub type NSPropertyListWriteOptions = NSUInteger;
extern_class!(
    #[derive(Debug)]
    pub struct NSPropertyListSerialization;
    unsafe impl ClassType for NSPropertyListSerialization {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSPropertyListSerialization {
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
        ) -> Result<Id<NSData, Shared>, Id<NSError, Shared>> {
            msg_send_id![
                Self::class(),
                dataWithPropertyList: plist,
                format: format,
                options: opt,
                error: _
            ]
        }
        pub unsafe fn propertyListWithData_options_format_error(
            data: &NSData,
            opt: NSPropertyListReadOptions,
            format: *mut NSPropertyListFormat,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>> {
            msg_send_id![
                Self::class(),
                propertyListWithData: data,
                options: opt,
                format: format,
                error: _
            ]
        }
        pub unsafe fn propertyListWithStream_options_format_error(
            stream: &NSInputStream,
            opt: NSPropertyListReadOptions,
            format: *mut NSPropertyListFormat,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>> {
            msg_send_id![
                Self::class(),
                propertyListWithStream: stream,
                options: opt,
                format: format,
                error: _
            ]
        }
    }
);
