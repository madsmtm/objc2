#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    struct NSPropertyListSerialization;
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
        error: *mut Option<&NSError>,
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
        error: *mut Option<&NSError>,
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
        error: *mut Option<&NSError>,
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
        error: *mut Option<&NSError>,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![
            Self::class(),
            propertyListWithStream: stream,
            options: opt,
            format: format,
            error: error
        ]
    }
    pub unsafe fn dataFromPropertyList_format_errorDescription(
        plist: &Object,
        format: NSPropertyListFormat,
        errorString: *mut Option<&NSString>,
    ) -> Option<Id<NSData, Shared>> {
        msg_send_id![
            Self::class(),
            dataFromPropertyList: plist,
            format: format,
            errorDescription: errorString
        ]
    }
    pub unsafe fn propertyListFromData_mutabilityOption_format_errorDescription(
        data: &NSData,
        opt: NSPropertyListMutabilityOptions,
        format: *mut NSPropertyListFormat,
        errorString: *mut Option<&NSString>,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![
            Self::class(),
            propertyListFromData: data,
            mutabilityOption: opt,
            format: format,
            errorDescription: errorString
        ]
    }
}
