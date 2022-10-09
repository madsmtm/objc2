use super::__exported::NSArray;
use super::__exported::NSError;
use super::__exported::NSFilePresenter;
use super::__exported::NSMutableDictionary;
use super::__exported::NSOperationQueue;
use super::__exported::NSSet;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSURL::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSFileAccessIntent;
    unsafe impl ClassType for NSFileAccessIntent {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSFileAccessIntent {
        #[method_id(readingIntentWithURL:options:)]
        pub unsafe fn readingIntentWithURL_options(
            url: &NSURL,
            options: NSFileCoordinatorReadingOptions,
        ) -> Id<Self, Shared>;
        #[method_id(writingIntentWithURL:options:)]
        pub unsafe fn writingIntentWithURL_options(
            url: &NSURL,
            options: NSFileCoordinatorWritingOptions,
        ) -> Id<Self, Shared>;
        #[method_id(URL)]
        pub unsafe fn URL(&self) -> Id<NSURL, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSFileCoordinator;
    unsafe impl ClassType for NSFileCoordinator {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSFileCoordinator {
        #[method(addFilePresenter:)]
        pub unsafe fn addFilePresenter(filePresenter: &NSFilePresenter);
        #[method(removeFilePresenter:)]
        pub unsafe fn removeFilePresenter(filePresenter: &NSFilePresenter);
        #[method_id(filePresenters)]
        pub unsafe fn filePresenters() -> Id<NSArray<NSFilePresenter>, Shared>;
        #[method_id(initWithFilePresenter:)]
        pub unsafe fn initWithFilePresenter(
            &self,
            filePresenterOrNil: Option<&NSFilePresenter>,
        ) -> Id<Self, Shared>;
        #[method_id(purposeIdentifier)]
        pub unsafe fn purposeIdentifier(&self) -> Id<NSString, Shared>;
        #[method(setPurposeIdentifier:)]
        pub unsafe fn setPurposeIdentifier(&self, purposeIdentifier: &NSString);
        #[method(coordinateAccessWithIntents:queue:byAccessor:)]
        pub unsafe fn coordinateAccessWithIntents_queue_byAccessor(
            &self,
            intents: &NSArray<NSFileAccessIntent>,
            queue: &NSOperationQueue,
            accessor: TodoBlock,
        );
        #[method(coordinateReadingItemAtURL:options:error:byAccessor:)]
        pub unsafe fn coordinateReadingItemAtURL_options_error_byAccessor(
            &self,
            url: &NSURL,
            options: NSFileCoordinatorReadingOptions,
            outError: *mut *mut NSError,
            reader: TodoBlock,
        );
        #[method(coordinateWritingItemAtURL:options:error:byAccessor:)]
        pub unsafe fn coordinateWritingItemAtURL_options_error_byAccessor(
            &self,
            url: &NSURL,
            options: NSFileCoordinatorWritingOptions,
            outError: *mut *mut NSError,
            writer: TodoBlock,
        );
        #[method(coordinateReadingItemAtURL:options:writingItemAtURL:options:error:byAccessor:)]
        pub unsafe fn coordinateReadingItemAtURL_options_writingItemAtURL_options_error_byAccessor(
            &self,
            readingURL: &NSURL,
            readingOptions: NSFileCoordinatorReadingOptions,
            writingURL: &NSURL,
            writingOptions: NSFileCoordinatorWritingOptions,
            outError: *mut *mut NSError,
            readerWriter: TodoBlock,
        );
        #[method(coordinateWritingItemAtURL:options:writingItemAtURL:options:error:byAccessor:)]
        pub unsafe fn coordinateWritingItemAtURL_options_writingItemAtURL_options_error_byAccessor(
            &self,
            url1: &NSURL,
            options1: NSFileCoordinatorWritingOptions,
            url2: &NSURL,
            options2: NSFileCoordinatorWritingOptions,
            outError: *mut *mut NSError,
            writer: TodoBlock,
        );
        #[method(prepareForReadingItemsAtURLs:options:writingItemsAtURLs:options:error:byAccessor:)]
        pub unsafe fn prepareForReadingItemsAtURLs_options_writingItemsAtURLs_options_error_byAccessor(
            &self,
            readingURLs: &NSArray<NSURL>,
            readingOptions: NSFileCoordinatorReadingOptions,
            writingURLs: &NSArray<NSURL>,
            writingOptions: NSFileCoordinatorWritingOptions,
            outError: *mut *mut NSError,
            batchAccessor: TodoBlock,
        );
        #[method(itemAtURL:willMoveToURL:)]
        pub unsafe fn itemAtURL_willMoveToURL(&self, oldURL: &NSURL, newURL: &NSURL);
        #[method(itemAtURL:didMoveToURL:)]
        pub unsafe fn itemAtURL_didMoveToURL(&self, oldURL: &NSURL, newURL: &NSURL);
        #[method(itemAtURL:didChangeUbiquityAttributes:)]
        pub unsafe fn itemAtURL_didChangeUbiquityAttributes(
            &self,
            url: &NSURL,
            attributes: &NSSet<NSURLResourceKey>,
        );
        #[method(cancel)]
        pub unsafe fn cancel(&self);
    }
);
