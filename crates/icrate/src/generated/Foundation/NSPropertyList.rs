#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
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
        #[method(propertyList:isValidForFormat:)]
        pub unsafe fn propertyList_isValidForFormat(
            plist: &Object,
            format: NSPropertyListFormat,
        ) -> bool;
        #[method_id(dataWithPropertyList:format:options:error:)]
        pub unsafe fn dataWithPropertyList_format_options_error(
            plist: &Object,
            format: NSPropertyListFormat,
            opt: NSPropertyListWriteOptions,
        ) -> Result<Id<NSData, Shared>, Id<NSError, Shared>>;
        #[method_id(propertyListWithData:options:format:error:)]
        pub unsafe fn propertyListWithData_options_format_error(
            data: &NSData,
            opt: NSPropertyListReadOptions,
            format: *mut NSPropertyListFormat,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;
        #[method_id(propertyListWithStream:options:format:error:)]
        pub unsafe fn propertyListWithStream_options_format_error(
            stream: &NSInputStream,
            opt: NSPropertyListReadOptions,
            format: *mut NSPropertyListFormat,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;
    }
);
