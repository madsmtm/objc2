#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSURLHandle;
    unsafe impl ClassType for NSURLHandle {
        type Super = NSObject;
    }
);
impl NSURLHandle {
    pub unsafe fn registerURLHandleClass(anURLHandleSubclass: Option<&Class>) {
        msg_send![Self::class(), registerURLHandleClass: anURLHandleSubclass]
    }
    pub unsafe fn URLHandleClassForURL(anURL: Option<&NSURL>) -> Option<&Class> {
        msg_send![Self::class(), URLHandleClassForURL: anURL]
    }
    pub unsafe fn status(&self) -> NSURLHandleStatus {
        msg_send![self, status]
    }
    pub unsafe fn failureReason(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, failureReason]
    }
    pub unsafe fn addClient(&self, client: TodoGenerics) {
        msg_send![self, addClient: client]
    }
    pub unsafe fn removeClient(&self, client: TodoGenerics) {
        msg_send![self, removeClient: client]
    }
    pub unsafe fn loadInBackground(&self) {
        msg_send![self, loadInBackground]
    }
    pub unsafe fn cancelLoadInBackground(&self) {
        msg_send![self, cancelLoadInBackground]
    }
    pub unsafe fn resourceData(&self) -> Option<Id<NSData, Shared>> {
        msg_send_id![self, resourceData]
    }
    pub unsafe fn availableResourceData(&self) -> Option<Id<NSData, Shared>> {
        msg_send_id![self, availableResourceData]
    }
    pub unsafe fn expectedResourceDataSize(&self) -> c_longlong {
        msg_send![self, expectedResourceDataSize]
    }
    pub unsafe fn flushCachedData(&self) {
        msg_send![self, flushCachedData]
    }
    pub unsafe fn backgroundLoadDidFailWithReason(&self, reason: Option<&NSString>) {
        msg_send![self, backgroundLoadDidFailWithReason: reason]
    }
    pub unsafe fn didLoadBytes_loadComplete(&self, newBytes: Option<&NSData>, yorn: bool) {
        msg_send![self, didLoadBytes: newBytes, loadComplete: yorn]
    }
    pub unsafe fn canInitWithURL(anURL: Option<&NSURL>) -> bool {
        msg_send![Self::class(), canInitWithURL: anURL]
    }
    pub unsafe fn cachedHandleForURL(anURL: Option<&NSURL>) -> Option<Id<NSURLHandle, Shared>> {
        msg_send_id![Self::class(), cachedHandleForURL: anURL]
    }
    pub unsafe fn initWithURL_cached(
        &self,
        anURL: Option<&NSURL>,
        willCache: bool,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![self, initWithURL: anURL, cached: willCache]
    }
    pub unsafe fn propertyForKey(
        &self,
        propertyKey: Option<&NSString>,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![self, propertyForKey: propertyKey]
    }
    pub unsafe fn propertyForKeyIfAvailable(
        &self,
        propertyKey: Option<&NSString>,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![self, propertyForKeyIfAvailable: propertyKey]
    }
    pub unsafe fn writeProperty_forKey(
        &self,
        propertyValue: Option<&Object>,
        propertyKey: Option<&NSString>,
    ) -> bool {
        msg_send![self, writeProperty: propertyValue, forKey: propertyKey]
    }
    pub unsafe fn writeData(&self, data: Option<&NSData>) -> bool {
        msg_send![self, writeData: data]
    }
    pub unsafe fn loadInForeground(&self) -> Option<Id<NSData, Shared>> {
        msg_send_id![self, loadInForeground]
    }
    pub unsafe fn beginLoadInBackground(&self) {
        msg_send![self, beginLoadInBackground]
    }
    pub unsafe fn endLoadInBackground(&self) {
        msg_send![self, endLoadInBackground]
    }
}
