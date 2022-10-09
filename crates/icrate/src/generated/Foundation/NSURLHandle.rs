use super::__exported::NSData;
use super::__exported::NSMutableArray;
use super::__exported::NSMutableData;
use super::__exported::NSURL;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSURLHandleClient = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSURLHandle;
    unsafe impl ClassType for NSURLHandle {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSURLHandle {
        #[method(registerURLHandleClass:)]
        pub unsafe fn registerURLHandleClass(anURLHandleSubclass: Option<&Class>);
        #[method(URLHandleClassForURL:)]
        pub unsafe fn URLHandleClassForURL(anURL: Option<&NSURL>) -> Option<&Class>;
        #[method(status)]
        pub unsafe fn status(&self) -> NSURLHandleStatus;
        #[method_id(failureReason)]
        pub unsafe fn failureReason(&self) -> Option<Id<NSString, Shared>>;
        #[method(addClient:)]
        pub unsafe fn addClient(&self, client: Option<&NSURLHandleClient>);
        #[method(removeClient:)]
        pub unsafe fn removeClient(&self, client: Option<&NSURLHandleClient>);
        #[method(loadInBackground)]
        pub unsafe fn loadInBackground(&self);
        #[method(cancelLoadInBackground)]
        pub unsafe fn cancelLoadInBackground(&self);
        #[method_id(resourceData)]
        pub unsafe fn resourceData(&self) -> Option<Id<NSData, Shared>>;
        #[method_id(availableResourceData)]
        pub unsafe fn availableResourceData(&self) -> Option<Id<NSData, Shared>>;
        #[method(expectedResourceDataSize)]
        pub unsafe fn expectedResourceDataSize(&self) -> c_longlong;
        #[method(flushCachedData)]
        pub unsafe fn flushCachedData(&self);
        #[method(backgroundLoadDidFailWithReason:)]
        pub unsafe fn backgroundLoadDidFailWithReason(&self, reason: Option<&NSString>);
        #[method(didLoadBytes:loadComplete:)]
        pub unsafe fn didLoadBytes_loadComplete(&self, newBytes: Option<&NSData>, yorn: bool);
        #[method(canInitWithURL:)]
        pub unsafe fn canInitWithURL(anURL: Option<&NSURL>) -> bool;
        #[method_id(cachedHandleForURL:)]
        pub unsafe fn cachedHandleForURL(anURL: Option<&NSURL>) -> Option<Id<NSURLHandle, Shared>>;
        #[method_id(initWithURL:cached:)]
        pub unsafe fn initWithURL_cached(
            &self,
            anURL: Option<&NSURL>,
            willCache: bool,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(propertyForKey:)]
        pub unsafe fn propertyForKey(
            &self,
            propertyKey: Option<&NSString>,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(propertyForKeyIfAvailable:)]
        pub unsafe fn propertyForKeyIfAvailable(
            &self,
            propertyKey: Option<&NSString>,
        ) -> Option<Id<Object, Shared>>;
        #[method(writeProperty:forKey:)]
        pub unsafe fn writeProperty_forKey(
            &self,
            propertyValue: Option<&Object>,
            propertyKey: Option<&NSString>,
        ) -> bool;
        #[method(writeData:)]
        pub unsafe fn writeData(&self, data: Option<&NSData>) -> bool;
        #[method_id(loadInForeground)]
        pub unsafe fn loadInForeground(&self) -> Option<Id<NSData, Shared>>;
        #[method(beginLoadInBackground)]
        pub unsafe fn beginLoadInBackground(&self);
        #[method(endLoadInBackground)]
        pub unsafe fn endLoadInBackground(&self);
    }
);
