//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug)]
    pub struct NSFilePromiseReceiver;

    unsafe impl ClassType for NSFilePromiseReceiver {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSFilePromiseReceiver {
        #[method_id(@__retain_semantics Other readableDraggedTypes)]
        pub unsafe fn readableDraggedTypes() -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other fileTypes)]
        pub unsafe fn fileTypes(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other fileNames)]
        pub unsafe fn fileNames(&self) -> Id<NSArray<NSString>, Shared>;

        #[method(receivePromisedFilesAtDestination:options:operationQueue:reader:)]
        pub unsafe fn receivePromisedFilesAtDestination_options_operationQueue_reader(
            &self,
            destinationDir: &NSURL,
            options: &NSDictionary,
            operationQueue: &NSOperationQueue,
            reader: &Block<(NonNull<NSURL>, *mut NSError), ()>,
        );
    }
);
