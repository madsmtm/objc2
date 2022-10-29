#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSJSONSerialization;
    unsafe impl ClassType for NSJSONSerialization {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSJSONSerialization {
        #[method(isValidJSONObject:)]
        pub unsafe fn isValidJSONObject(obj: &Object) -> bool;
        #[method_id(dataWithJSONObject:options:error:)]
        pub unsafe fn dataWithJSONObject_options_error(
            obj: &Object,
            opt: NSJSONWritingOptions,
        ) -> Result<Id<NSData, Shared>, Id<NSError, Shared>>;
        #[method_id(JSONObjectWithData:options:error:)]
        pub unsafe fn JSONObjectWithData_options_error(
            data: &NSData,
            opt: NSJSONReadingOptions,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;
        #[method_id(JSONObjectWithStream:options:error:)]
        pub unsafe fn JSONObjectWithStream_options_error(
            stream: &NSInputStream,
            opt: NSJSONReadingOptions,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;
    }
);
