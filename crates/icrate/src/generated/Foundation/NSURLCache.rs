use super::__exported::NSCachedURLResponseInternal;
use super::__exported::NSData;
use super::__exported::NSDate;
use super::__exported::NSDictionary;
use super::__exported::NSURLRequest;
use super::__exported::NSURLResponse;
use super::__exported::NSURLSessionDataTask;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSCachedURLResponse;
    unsafe impl ClassType for NSCachedURLResponse {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSCachedURLResponse {
        # [method_id (initWithResponse : data :)]
        pub unsafe fn initWithResponse_data(
            &self,
            response: &NSURLResponse,
            data: &NSData,
        ) -> Id<Self, Shared>;
        # [method_id (initWithResponse : data : userInfo : storagePolicy :)]
        pub unsafe fn initWithResponse_data_userInfo_storagePolicy(
            &self,
            response: &NSURLResponse,
            data: &NSData,
            userInfo: Option<&NSDictionary>,
            storagePolicy: NSURLCacheStoragePolicy,
        ) -> Id<Self, Shared>;
        #[method_id(response)]
        pub unsafe fn response(&self) -> Id<NSURLResponse, Shared>;
        #[method_id(data)]
        pub unsafe fn data(&self) -> Id<NSData, Shared>;
        #[method_id(userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary, Shared>>;
        #[method(storagePolicy)]
        pub unsafe fn storagePolicy(&self) -> NSURLCacheStoragePolicy;
    }
);
use super::__exported::NSURLCacheInternal;
use super::__exported::NSURLRequest;
extern_class!(
    #[derive(Debug)]
    pub struct NSURLCache;
    unsafe impl ClassType for NSURLCache {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSURLCache {
        #[method_id(sharedURLCache)]
        pub unsafe fn sharedURLCache() -> Id<NSURLCache, Shared>;
        # [method (setSharedURLCache :)]
        pub unsafe fn setSharedURLCache(sharedURLCache: &NSURLCache);
        # [method_id (initWithMemoryCapacity : diskCapacity : diskPath :)]
        pub unsafe fn initWithMemoryCapacity_diskCapacity_diskPath(
            &self,
            memoryCapacity: NSUInteger,
            diskCapacity: NSUInteger,
            path: Option<&NSString>,
        ) -> Id<Self, Shared>;
        # [method_id (initWithMemoryCapacity : diskCapacity : directoryURL :)]
        pub unsafe fn initWithMemoryCapacity_diskCapacity_directoryURL(
            &self,
            memoryCapacity: NSUInteger,
            diskCapacity: NSUInteger,
            directoryURL: Option<&NSURL>,
        ) -> Id<Self, Shared>;
        # [method_id (cachedResponseForRequest :)]
        pub unsafe fn cachedResponseForRequest(
            &self,
            request: &NSURLRequest,
        ) -> Option<Id<NSCachedURLResponse, Shared>>;
        # [method (storeCachedResponse : forRequest :)]
        pub unsafe fn storeCachedResponse_forRequest(
            &self,
            cachedResponse: &NSCachedURLResponse,
            request: &NSURLRequest,
        );
        # [method (removeCachedResponseForRequest :)]
        pub unsafe fn removeCachedResponseForRequest(&self, request: &NSURLRequest);
        #[method(removeAllCachedResponses)]
        pub unsafe fn removeAllCachedResponses(&self);
        # [method (removeCachedResponsesSinceDate :)]
        pub unsafe fn removeCachedResponsesSinceDate(&self, date: &NSDate);
        #[method(memoryCapacity)]
        pub unsafe fn memoryCapacity(&self) -> NSUInteger;
        # [method (setMemoryCapacity :)]
        pub unsafe fn setMemoryCapacity(&self, memoryCapacity: NSUInteger);
        #[method(diskCapacity)]
        pub unsafe fn diskCapacity(&self) -> NSUInteger;
        # [method (setDiskCapacity :)]
        pub unsafe fn setDiskCapacity(&self, diskCapacity: NSUInteger);
        #[method(currentMemoryUsage)]
        pub unsafe fn currentMemoryUsage(&self) -> NSUInteger;
        #[method(currentDiskUsage)]
        pub unsafe fn currentDiskUsage(&self) -> NSUInteger;
    }
);
extern_methods!(
    #[doc = "NSURLSessionTaskAdditions"]
    unsafe impl NSURLCache {
        # [method (storeCachedResponse : forDataTask :)]
        pub unsafe fn storeCachedResponse_forDataTask(
            &self,
            cachedResponse: &NSCachedURLResponse,
            dataTask: &NSURLSessionDataTask,
        );
        # [method (getCachedResponseForDataTask : completionHandler :)]
        pub unsafe fn getCachedResponseForDataTask_completionHandler(
            &self,
            dataTask: &NSURLSessionDataTask,
            completionHandler: TodoBlock,
        );
        # [method (removeCachedResponseForDataTask :)]
        pub unsafe fn removeCachedResponseForDataTask(&self, dataTask: &NSURLSessionDataTask);
    }
);
