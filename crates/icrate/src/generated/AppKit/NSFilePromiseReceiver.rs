use super::__exported::NSOperationQueue;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSPasteboard::*;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSGeometry::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSFilePromiseReceiver;
    unsafe impl ClassType for NSFilePromiseReceiver {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSFilePromiseReceiver {
        #[method_id(readableDraggedTypes)]
        pub unsafe fn readableDraggedTypes() -> Id<NSArray<NSString>, Shared>;
        #[method_id(fileTypes)]
        pub unsafe fn fileTypes(&self) -> Id<NSArray<NSString>, Shared>;
        #[method_id(fileNames)]
        pub unsafe fn fileNames(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(receivePromisedFilesAtDestination:options:operationQueue:reader:)]
        pub unsafe fn receivePromisedFilesAtDestination_options_operationQueue_reader(
            &self,
            destinationDir: &NSURL,
            options: &NSDictionary,
            operationQueue: &NSOperationQueue,
            reader: TodoBlock,
        );
    }
);
