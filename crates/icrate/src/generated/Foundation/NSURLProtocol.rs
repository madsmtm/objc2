use super::__exported::NSCachedURLResponse;
use super::__exported::NSError;
use super::__exported::NSMutableURLRequest;
use super::__exported::NSURLAuthenticationChallenge;
use super::__exported::NSURLConnection;
use super::__exported::NSURLProtocolInternal;
use super::__exported::NSURLRequest;
use super::__exported::NSURLResponse;
use super::__exported::NSURLSessionTask;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSURLCache::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSURLProtocolClient = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSURLProtocol;
    unsafe impl ClassType for NSURLProtocol {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSURLProtocol {
        # [method_id (initWithRequest : cachedResponse : client :)]
        pub unsafe fn initWithRequest_cachedResponse_client(
            &self,
            request: &NSURLRequest,
            cachedResponse: Option<&NSCachedURLResponse>,
            client: Option<&NSURLProtocolClient>,
        ) -> Id<Self, Shared>;
        #[method_id(client)]
        pub unsafe fn client(&self) -> Option<Id<NSURLProtocolClient, Shared>>;
        #[method_id(request)]
        pub unsafe fn request(&self) -> Id<NSURLRequest, Shared>;
        #[method_id(cachedResponse)]
        pub unsafe fn cachedResponse(&self) -> Option<Id<NSCachedURLResponse, Shared>>;
        # [method (canInitWithRequest :)]
        pub unsafe fn canInitWithRequest(request: &NSURLRequest) -> bool;
        # [method_id (canonicalRequestForRequest :)]
        pub unsafe fn canonicalRequestForRequest(
            request: &NSURLRequest,
        ) -> Id<NSURLRequest, Shared>;
        # [method (requestIsCacheEquivalent : toRequest :)]
        pub unsafe fn requestIsCacheEquivalent_toRequest(
            a: &NSURLRequest,
            b: &NSURLRequest,
        ) -> bool;
        #[method(startLoading)]
        pub unsafe fn startLoading(&self);
        #[method(stopLoading)]
        pub unsafe fn stopLoading(&self);
        # [method_id (propertyForKey : inRequest :)]
        pub unsafe fn propertyForKey_inRequest(
            key: &NSString,
            request: &NSURLRequest,
        ) -> Option<Id<Object, Shared>>;
        # [method (setProperty : forKey : inRequest :)]
        pub unsafe fn setProperty_forKey_inRequest(
            value: &Object,
            key: &NSString,
            request: &NSMutableURLRequest,
        );
        # [method (removePropertyForKey : inRequest :)]
        pub unsafe fn removePropertyForKey_inRequest(key: &NSString, request: &NSMutableURLRequest);
        # [method (registerClass :)]
        pub unsafe fn registerClass(protocolClass: &Class) -> bool;
        # [method (unregisterClass :)]
        pub unsafe fn unregisterClass(protocolClass: &Class);
    }
);
extern_methods!(
    #[doc = "NSURLSessionTaskAdditions"]
    unsafe impl NSURLProtocol {
        # [method (canInitWithTask :)]
        pub unsafe fn canInitWithTask(task: &NSURLSessionTask) -> bool;
        # [method_id (initWithTask : cachedResponse : client :)]
        pub unsafe fn initWithTask_cachedResponse_client(
            &self,
            task: &NSURLSessionTask,
            cachedResponse: Option<&NSCachedURLResponse>,
            client: Option<&NSURLProtocolClient>,
        ) -> Id<Self, Shared>;
        #[method_id(task)]
        pub unsafe fn task(&self) -> Option<Id<NSURLSessionTask, Shared>>;
    }
);
