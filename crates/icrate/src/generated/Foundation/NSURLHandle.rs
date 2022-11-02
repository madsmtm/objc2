//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern "C" {
    pub static NSHTTPPropertyStatusCodeKey: Option<&'static NSString>;
}

extern "C" {
    pub static NSHTTPPropertyStatusReasonKey: Option<&'static NSString>;
}

extern "C" {
    pub static NSHTTPPropertyServerHTTPVersionKey: Option<&'static NSString>;
}

extern "C" {
    pub static NSHTTPPropertyRedirectionHeadersKey: Option<&'static NSString>;
}

extern "C" {
    pub static NSHTTPPropertyErrorPageDataKey: Option<&'static NSString>;
}

extern "C" {
    pub static NSHTTPPropertyHTTPProxy: Option<&'static NSString>;
}

extern "C" {
    pub static NSFTPPropertyUserLoginKey: Option<&'static NSString>;
}

extern "C" {
    pub static NSFTPPropertyUserPasswordKey: Option<&'static NSString>;
}

extern "C" {
    pub static NSFTPPropertyActiveTransferModeKey: Option<&'static NSString>;
}

extern "C" {
    pub static NSFTPPropertyFileOffsetKey: Option<&'static NSString>;
}

extern "C" {
    pub static NSFTPPropertyFTPProxy: Option<&'static NSString>;
}

pub type NSURLHandleStatus = NSUInteger;
pub const NSURLHandleNotLoaded: NSURLHandleStatus = 0;
pub const NSURLHandleLoadSucceeded: NSURLHandleStatus = 1;
pub const NSURLHandleLoadInProgress: NSURLHandleStatus = 2;
pub const NSURLHandleLoadFailed: NSURLHandleStatus = 3;

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
        pub unsafe fn URLHandleClassForURL(anURL: Option<&NSURL>) -> Option<&'static Class>;

        #[method(status)]
        pub unsafe fn status(&self) -> NSURLHandleStatus;

        #[method_id(@__retain_semantics Other failureReason)]
        pub unsafe fn failureReason(&self) -> Option<Id<NSString, Shared>>;

        #[method(addClient:)]
        pub unsafe fn addClient(&self, client: Option<&NSURLHandleClient>);

        #[method(removeClient:)]
        pub unsafe fn removeClient(&self, client: Option<&NSURLHandleClient>);

        #[method(loadInBackground)]
        pub unsafe fn loadInBackground(&self);

        #[method(cancelLoadInBackground)]
        pub unsafe fn cancelLoadInBackground(&self);

        #[method_id(@__retain_semantics Other resourceData)]
        pub unsafe fn resourceData(&self) -> Option<Id<NSData, Shared>>;

        #[method_id(@__retain_semantics Other availableResourceData)]
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

        #[method_id(@__retain_semantics Other cachedHandleForURL:)]
        pub unsafe fn cachedHandleForURL(anURL: Option<&NSURL>) -> Option<Id<NSURLHandle, Shared>>;

        #[method_id(@__retain_semantics Init initWithURL:cached:)]
        pub unsafe fn initWithURL_cached(
            this: Option<Allocated<Self>>,
            anURL: Option<&NSURL>,
            willCache: bool,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other propertyForKey:)]
        pub unsafe fn propertyForKey(
            &self,
            propertyKey: Option<&NSString>,
        ) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other propertyForKeyIfAvailable:)]
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

        #[method_id(@__retain_semantics Other loadInForeground)]
        pub unsafe fn loadInForeground(&self) -> Option<Id<NSData, Shared>>;

        #[method(beginLoadInBackground)]
        pub unsafe fn beginLoadInBackground(&self);

        #[method(endLoadInBackground)]
        pub unsafe fn endLoadInBackground(&self);
    }
);
