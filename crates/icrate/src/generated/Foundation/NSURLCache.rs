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
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSCachedURLResponse;
    unsafe impl ClassType for NSCachedURLResponse {
        type Super = NSObject;
    }
);
impl NSCachedURLResponse {
    pub unsafe fn initWithResponse_data(
        &self,
        response: &NSURLResponse,
        data: &NSData,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithResponse: response, data: data]
    }
    pub unsafe fn initWithResponse_data_userInfo_storagePolicy(
        &self,
        response: &NSURLResponse,
        data: &NSData,
        userInfo: Option<&NSDictionary>,
        storagePolicy: NSURLCacheStoragePolicy,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithResponse: response,
            data: data,
            userInfo: userInfo,
            storagePolicy: storagePolicy
        ]
    }
    pub unsafe fn response(&self) -> Id<NSURLResponse, Shared> {
        msg_send_id![self, response]
    }
    pub unsafe fn data(&self) -> Id<NSData, Shared> {
        msg_send_id![self, data]
    }
    pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary, Shared>> {
        msg_send_id![self, userInfo]
    }
    pub unsafe fn storagePolicy(&self) -> NSURLCacheStoragePolicy {
        msg_send![self, storagePolicy]
    }
}
use super::__exported::NSURLCacheInternal;
use super::__exported::NSURLRequest;
extern_class!(
    #[derive(Debug)]
    pub struct NSURLCache;
    unsafe impl ClassType for NSURLCache {
        type Super = NSObject;
    }
);
impl NSURLCache {
    pub unsafe fn initWithMemoryCapacity_diskCapacity_diskPath(
        &self,
        memoryCapacity: NSUInteger,
        diskCapacity: NSUInteger,
        path: Option<&NSString>,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithMemoryCapacity: memoryCapacity,
            diskCapacity: diskCapacity,
            diskPath: path
        ]
    }
    pub unsafe fn initWithMemoryCapacity_diskCapacity_directoryURL(
        &self,
        memoryCapacity: NSUInteger,
        diskCapacity: NSUInteger,
        directoryURL: Option<&NSURL>,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithMemoryCapacity: memoryCapacity,
            diskCapacity: diskCapacity,
            directoryURL: directoryURL
        ]
    }
    pub unsafe fn cachedResponseForRequest(
        &self,
        request: &NSURLRequest,
    ) -> Option<Id<NSCachedURLResponse, Shared>> {
        msg_send_id![self, cachedResponseForRequest: request]
    }
    pub unsafe fn storeCachedResponse_forRequest(
        &self,
        cachedResponse: &NSCachedURLResponse,
        request: &NSURLRequest,
    ) {
        msg_send![
            self,
            storeCachedResponse: cachedResponse,
            forRequest: request
        ]
    }
    pub unsafe fn removeCachedResponseForRequest(&self, request: &NSURLRequest) {
        msg_send![self, removeCachedResponseForRequest: request]
    }
    pub unsafe fn removeAllCachedResponses(&self) {
        msg_send![self, removeAllCachedResponses]
    }
    pub unsafe fn removeCachedResponsesSinceDate(&self, date: &NSDate) {
        msg_send![self, removeCachedResponsesSinceDate: date]
    }
    pub unsafe fn sharedURLCache() -> Id<NSURLCache, Shared> {
        msg_send_id![Self::class(), sharedURLCache]
    }
    pub unsafe fn setSharedURLCache(sharedURLCache: &NSURLCache) {
        msg_send![Self::class(), setSharedURLCache: sharedURLCache]
    }
    pub unsafe fn memoryCapacity(&self) -> NSUInteger {
        msg_send![self, memoryCapacity]
    }
    pub unsafe fn setMemoryCapacity(&self, memoryCapacity: NSUInteger) {
        msg_send![self, setMemoryCapacity: memoryCapacity]
    }
    pub unsafe fn diskCapacity(&self) -> NSUInteger {
        msg_send![self, diskCapacity]
    }
    pub unsafe fn setDiskCapacity(&self, diskCapacity: NSUInteger) {
        msg_send![self, setDiskCapacity: diskCapacity]
    }
    pub unsafe fn currentMemoryUsage(&self) -> NSUInteger {
        msg_send![self, currentMemoryUsage]
    }
    pub unsafe fn currentDiskUsage(&self) -> NSUInteger {
        msg_send![self, currentDiskUsage]
    }
}
#[doc = "NSURLSessionTaskAdditions"]
impl NSURLCache {
    pub unsafe fn storeCachedResponse_forDataTask(
        &self,
        cachedResponse: &NSCachedURLResponse,
        dataTask: &NSURLSessionDataTask,
    ) {
        msg_send![
            self,
            storeCachedResponse: cachedResponse,
            forDataTask: dataTask
        ]
    }
    pub unsafe fn getCachedResponseForDataTask_completionHandler(
        &self,
        dataTask: &NSURLSessionDataTask,
        completionHandler: TodoBlock,
    ) {
        msg_send![
            self,
            getCachedResponseForDataTask: dataTask,
            completionHandler: completionHandler
        ]
    }
    pub unsafe fn removeCachedResponseForDataTask(&self, dataTask: &NSURLSessionDataTask) {
        msg_send![self, removeCachedResponseForDataTask: dataTask]
    }
}
