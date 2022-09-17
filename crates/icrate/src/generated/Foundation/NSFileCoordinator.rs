#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    struct NSFileAccessIntent;
    unsafe impl ClassType for NSFileAccessIntent {
        type Super = NSObject;
    }
);
impl NSFileAccessIntent {
    pub unsafe fn readingIntentWithURL_options(
        url: &NSURL,
        options: NSFileCoordinatorReadingOptions,
    ) -> Id<Self, Shared> {
        msg_send_id![Self::class(), readingIntentWithURL: url, options: options]
    }
    pub unsafe fn writingIntentWithURL_options(
        url: &NSURL,
        options: NSFileCoordinatorWritingOptions,
    ) -> Id<Self, Shared> {
        msg_send_id![Self::class(), writingIntentWithURL: url, options: options]
    }
    pub unsafe fn URL(&self) -> Id<NSURL, Shared> {
        msg_send_id![self, URL]
    }
}
extern_class!(
    #[derive(Debug)]
    struct NSFileCoordinator;
    unsafe impl ClassType for NSFileCoordinator {
        type Super = NSObject;
    }
);
impl NSFileCoordinator {
    pub unsafe fn addFilePresenter(filePresenter: TodoGenerics) {
        msg_send![Self::class(), addFilePresenter: filePresenter]
    }
    pub unsafe fn removeFilePresenter(filePresenter: TodoGenerics) {
        msg_send![Self::class(), removeFilePresenter: filePresenter]
    }
    pub unsafe fn initWithFilePresenter(
        &self,
        filePresenterOrNil: TodoGenerics,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithFilePresenter: filePresenterOrNil]
    }
    pub unsafe fn coordinateAccessWithIntents_queue_byAccessor(
        &self,
        intents: TodoGenerics,
        queue: &NSOperationQueue,
        accessor: TodoBlock,
    ) {
        msg_send![
            self,
            coordinateAccessWithIntents: intents,
            queue: queue,
            byAccessor: accessor
        ]
    }
    pub unsafe fn coordinateReadingItemAtURL_options_error_byAccessor(
        &self,
        url: &NSURL,
        options: NSFileCoordinatorReadingOptions,
        outError: *mut Option<&NSError>,
        reader: TodoBlock,
    ) {
        msg_send![
            self,
            coordinateReadingItemAtURL: url,
            options: options,
            error: outError,
            byAccessor: reader
        ]
    }
    pub unsafe fn coordinateWritingItemAtURL_options_error_byAccessor(
        &self,
        url: &NSURL,
        options: NSFileCoordinatorWritingOptions,
        outError: *mut Option<&NSError>,
        writer: TodoBlock,
    ) {
        msg_send![
            self,
            coordinateWritingItemAtURL: url,
            options: options,
            error: outError,
            byAccessor: writer
        ]
    }
    pub unsafe fn coordinateReadingItemAtURL_options_writingItemAtURL_options_error_byAccessor(
        &self,
        readingURL: &NSURL,
        readingOptions: NSFileCoordinatorReadingOptions,
        writingURL: &NSURL,
        writingOptions: NSFileCoordinatorWritingOptions,
        outError: *mut Option<&NSError>,
        readerWriter: TodoBlock,
    ) {
        msg_send![
            self,
            coordinateReadingItemAtURL: readingURL,
            options: readingOptions,
            writingItemAtURL: writingURL,
            options: writingOptions,
            error: outError,
            byAccessor: readerWriter
        ]
    }
    pub unsafe fn coordinateWritingItemAtURL_options_writingItemAtURL_options_error_byAccessor(
        &self,
        url1: &NSURL,
        options1: NSFileCoordinatorWritingOptions,
        url2: &NSURL,
        options2: NSFileCoordinatorWritingOptions,
        outError: *mut Option<&NSError>,
        writer: TodoBlock,
    ) {
        msg_send![
            self,
            coordinateWritingItemAtURL: url1,
            options: options1,
            writingItemAtURL: url2,
            options: options2,
            error: outError,
            byAccessor: writer
        ]
    }
    pub unsafe fn prepareForReadingItemsAtURLs_options_writingItemsAtURLs_options_error_byAccessor(
        &self,
        readingURLs: TodoGenerics,
        readingOptions: NSFileCoordinatorReadingOptions,
        writingURLs: TodoGenerics,
        writingOptions: NSFileCoordinatorWritingOptions,
        outError: *mut Option<&NSError>,
        batchAccessor: TodoBlock,
    ) {
        msg_send![
            self,
            prepareForReadingItemsAtURLs: readingURLs,
            options: readingOptions,
            writingItemsAtURLs: writingURLs,
            options: writingOptions,
            error: outError,
            byAccessor: batchAccessor
        ]
    }
    pub unsafe fn itemAtURL_willMoveToURL(&self, oldURL: &NSURL, newURL: &NSURL) {
        msg_send![self, itemAtURL: oldURL, willMoveToURL: newURL]
    }
    pub unsafe fn itemAtURL_didMoveToURL(&self, oldURL: &NSURL, newURL: &NSURL) {
        msg_send![self, itemAtURL: oldURL, didMoveToURL: newURL]
    }
    pub unsafe fn itemAtURL_didChangeUbiquityAttributes(
        &self,
        url: &NSURL,
        attributes: TodoGenerics,
    ) {
        msg_send![
            self,
            itemAtURL: url,
            didChangeUbiquityAttributes: attributes
        ]
    }
    pub unsafe fn cancel(&self) {
        msg_send![self, cancel]
    }
    pub unsafe fn filePresenters() -> TodoGenerics {
        msg_send![Self::class(), filePresenters]
    }
    pub unsafe fn purposeIdentifier(&self) -> Id<NSString, Shared> {
        msg_send_id![self, purposeIdentifier]
    }
    pub unsafe fn setPurposeIdentifier(&self, purposeIdentifier: &NSString) {
        msg_send![self, setPurposeIdentifier: purposeIdentifier]
    }
}
