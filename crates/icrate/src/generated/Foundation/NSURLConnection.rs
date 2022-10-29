#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSURLConnection;
    unsafe impl ClassType for NSURLConnection {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSURLConnection {
        #[method_id(initWithRequest:delegate:startImmediately:)]
        pub unsafe fn initWithRequest_delegate_startImmediately(
            &self,
            request: &NSURLRequest,
            delegate: Option<&Object>,
            startImmediately: bool,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithRequest:delegate:)]
        pub unsafe fn initWithRequest_delegate(
            &self,
            request: &NSURLRequest,
            delegate: Option<&Object>,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(connectionWithRequest:delegate:)]
        pub unsafe fn connectionWithRequest_delegate(
            request: &NSURLRequest,
            delegate: Option<&Object>,
        ) -> Option<Id<NSURLConnection, Shared>>;
        #[method_id(originalRequest)]
        pub unsafe fn originalRequest(&self) -> Id<NSURLRequest, Shared>;
        #[method_id(currentRequest)]
        pub unsafe fn currentRequest(&self) -> Id<NSURLRequest, Shared>;
        #[method(start)]
        pub unsafe fn start(&self);
        #[method(cancel)]
        pub unsafe fn cancel(&self);
        #[method(scheduleInRunLoop:forMode:)]
        pub unsafe fn scheduleInRunLoop_forMode(&self, aRunLoop: &NSRunLoop, mode: &NSRunLoopMode);
        #[method(unscheduleFromRunLoop:forMode:)]
        pub unsafe fn unscheduleFromRunLoop_forMode(
            &self,
            aRunLoop: &NSRunLoop,
            mode: &NSRunLoopMode,
        );
        #[method(setDelegateQueue:)]
        pub unsafe fn setDelegateQueue(&self, queue: Option<&NSOperationQueue>);
        #[method(canHandleRequest:)]
        pub unsafe fn canHandleRequest(request: &NSURLRequest) -> bool;
    }
);
pub type NSURLConnectionDelegate = NSObject;
pub type NSURLConnectionDataDelegate = NSObject;
pub type NSURLConnectionDownloadDelegate = NSObject;
extern_methods!(
    #[doc = "NSURLConnectionSynchronousLoading"]
    unsafe impl NSURLConnection {
        #[method_id(sendSynchronousRequest:returningResponse:error:)]
        pub unsafe fn sendSynchronousRequest_returningResponse_error(
            request: &NSURLRequest,
            response: Option<&mut Option<Id<NSURLResponse, Shared>>>,
        ) -> Result<Id<NSData, Shared>, Id<NSError, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSURLConnectionQueuedLoading"]
    unsafe impl NSURLConnection {
        #[method(sendAsynchronousRequest:queue:completionHandler:)]
        pub unsafe fn sendAsynchronousRequest_queue_completionHandler(
            request: &NSURLRequest,
            queue: &NSOperationQueue,
            handler: TodoBlock,
        );
    }
);
