use super::NSArray;
use super::NSCachedURLResponse;
use super::NSData;
use super::NSError;
use super::NSInputStream;
use super::NSOperationQueue;
use super::NSRunLoop;
use super::NSURLAuthenticationChallenge;
use super::NSURLConnectionInternal;
use super::NSURLProtectionSpace;
use super::NSURLRequest;
use super::NSURLResponse;
use super::NSURL;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSRunLoop::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSURLConnection;
    unsafe impl ClassType for NSURLConnection {
        type Super = NSObject;
    }
);
impl NSURLConnection {
    pub unsafe fn initWithRequest_delegate_startImmediately(
        &self,
        request: &NSURLRequest,
        delegate: Option<&Object>,
        startImmediately: bool,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            self,
            initWithRequest: request,
            delegate: delegate,
            startImmediately: startImmediately
        ]
    }
    pub unsafe fn initWithRequest_delegate(
        &self,
        request: &NSURLRequest,
        delegate: Option<&Object>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithRequest: request, delegate: delegate]
    }
    pub unsafe fn connectionWithRequest_delegate(
        request: &NSURLRequest,
        delegate: Option<&Object>,
    ) -> Option<Id<NSURLConnection, Shared>> {
        msg_send_id![
            Self::class(),
            connectionWithRequest: request,
            delegate: delegate
        ]
    }
    pub unsafe fn start(&self) {
        msg_send![self, start]
    }
    pub unsafe fn cancel(&self) {
        msg_send![self, cancel]
    }
    pub unsafe fn scheduleInRunLoop_forMode(&self, aRunLoop: &NSRunLoop, mode: NSRunLoopMode) {
        msg_send![self, scheduleInRunLoop: aRunLoop, forMode: mode]
    }
    pub unsafe fn unscheduleFromRunLoop_forMode(&self, aRunLoop: &NSRunLoop, mode: NSRunLoopMode) {
        msg_send![self, unscheduleFromRunLoop: aRunLoop, forMode: mode]
    }
    pub unsafe fn setDelegateQueue(&self, queue: Option<&NSOperationQueue>) {
        msg_send![self, setDelegateQueue: queue]
    }
    pub unsafe fn canHandleRequest(request: &NSURLRequest) -> bool {
        msg_send![Self::class(), canHandleRequest: request]
    }
    pub unsafe fn originalRequest(&self) -> Id<NSURLRequest, Shared> {
        msg_send_id![self, originalRequest]
    }
    pub unsafe fn currentRequest(&self) -> Id<NSURLRequest, Shared> {
        msg_send_id![self, currentRequest]
    }
}
pub type NSURLConnectionDelegate = NSObject;
pub type NSURLConnectionDataDelegate = NSObject;
pub type NSURLConnectionDownloadDelegate = NSObject;
#[doc = "NSURLConnectionSynchronousLoading"]
impl NSURLConnection {
    pub unsafe fn sendSynchronousRequest_returningResponse_error(
        request: &NSURLRequest,
        response: *mut Option<&NSURLResponse>,
        error: *mut Option<&NSError>,
    ) -> Option<Id<NSData, Shared>> {
        msg_send_id![
            Self::class(),
            sendSynchronousRequest: request,
            returningResponse: response,
            error: error
        ]
    }
}
#[doc = "NSURLConnectionQueuedLoading"]
impl NSURLConnection {
    pub unsafe fn sendAsynchronousRequest_queue_completionHandler(
        request: &NSURLRequest,
        queue: &NSOperationQueue,
        handler: TodoBlock,
    ) {
        msg_send![
            Self::class(),
            sendAsynchronousRequest: request,
            queue: queue,
            completionHandler: handler
        ]
    }
}
