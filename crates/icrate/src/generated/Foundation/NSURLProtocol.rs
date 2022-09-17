#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSURLProtocol;
    unsafe impl ClassType for NSURLProtocol {
        type Super = NSObject;
    }
);
impl NSURLProtocol {
    pub unsafe fn initWithRequest_cachedResponse_client(
        &self,
        request: &NSURLRequest,
        cachedResponse: Option<&NSCachedURLResponse>,
        client: TodoGenerics,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithRequest: request,
            cachedResponse: cachedResponse,
            client: client
        ]
    }
    pub unsafe fn canInitWithRequest(request: &NSURLRequest) -> bool {
        msg_send![Self::class(), canInitWithRequest: request]
    }
    pub unsafe fn canonicalRequestForRequest(request: &NSURLRequest) -> Id<NSURLRequest, Shared> {
        msg_send_id![Self::class(), canonicalRequestForRequest: request]
    }
    pub unsafe fn requestIsCacheEquivalent_toRequest(a: &NSURLRequest, b: &NSURLRequest) -> bool {
        msg_send![Self::class(), requestIsCacheEquivalent: a, toRequest: b]
    }
    pub unsafe fn startLoading(&self) {
        msg_send![self, startLoading]
    }
    pub unsafe fn stopLoading(&self) {
        msg_send![self, stopLoading]
    }
    pub unsafe fn propertyForKey_inRequest(
        key: &NSString,
        request: &NSURLRequest,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![Self::class(), propertyForKey: key, inRequest: request]
    }
    pub unsafe fn setProperty_forKey_inRequest(
        value: &Object,
        key: &NSString,
        request: &NSMutableURLRequest,
    ) {
        msg_send![
            Self::class(),
            setProperty: value,
            forKey: key,
            inRequest: request
        ]
    }
    pub unsafe fn removePropertyForKey_inRequest(key: &NSString, request: &NSMutableURLRequest) {
        msg_send![Self::class(), removePropertyForKey: key, inRequest: request]
    }
    pub unsafe fn registerClass(protocolClass: &Class) -> bool {
        msg_send![Self::class(), registerClass: protocolClass]
    }
    pub unsafe fn unregisterClass(protocolClass: &Class) {
        msg_send![Self::class(), unregisterClass: protocolClass]
    }
    pub unsafe fn client(&self) -> TodoGenerics {
        msg_send![self, client]
    }
    pub unsafe fn request(&self) -> Id<NSURLRequest, Shared> {
        msg_send_id![self, request]
    }
    pub unsafe fn cachedResponse(&self) -> Option<Id<NSCachedURLResponse, Shared>> {
        msg_send_id![self, cachedResponse]
    }
}
#[doc = "NSURLSessionTaskAdditions"]
impl NSURLProtocol {
    pub unsafe fn canInitWithTask(task: &NSURLSessionTask) -> bool {
        msg_send![Self::class(), canInitWithTask: task]
    }
    pub unsafe fn initWithTask_cachedResponse_client(
        &self,
        task: &NSURLSessionTask,
        cachedResponse: Option<&NSCachedURLResponse>,
        client: TodoGenerics,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithTask: task,
            cachedResponse: cachedResponse,
            client: client
        ]
    }
    pub unsafe fn task(&self) -> Option<Id<NSURLSessionTask, Shared>> {
        msg_send_id![self, task]
    }
}
