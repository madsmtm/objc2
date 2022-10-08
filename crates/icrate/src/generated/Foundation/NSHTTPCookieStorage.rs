use super::__exported::NSArray;
use super::__exported::NSDate;
use super::__exported::NSHTTPCookie;
use super::__exported::NSHTTPCookieStorageInternal;
use super::__exported::NSSortDescriptor;
use super::__exported::NSURLSessionTask;
use super::__exported::NSURL;
use crate::Foundation::generated::NSNotification::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSHTTPCookieStorage;
    unsafe impl ClassType for NSHTTPCookieStorage {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSHTTPCookieStorage {
        #[method_id(sharedHTTPCookieStorage)]
        pub unsafe fn sharedHTTPCookieStorage() -> Id<NSHTTPCookieStorage, Shared>;
        # [method_id (sharedCookieStorageForGroupContainerIdentifier :)]
        pub unsafe fn sharedCookieStorageForGroupContainerIdentifier(
            identifier: &NSString,
        ) -> Id<NSHTTPCookieStorage, Shared>;
        #[method_id(cookies)]
        pub unsafe fn cookies(&self) -> Option<Id<NSArray<NSHTTPCookie>, Shared>>;
        # [method (setCookie :)]
        pub unsafe fn setCookie(&self, cookie: &NSHTTPCookie);
        # [method (deleteCookie :)]
        pub unsafe fn deleteCookie(&self, cookie: &NSHTTPCookie);
        # [method (removeCookiesSinceDate :)]
        pub unsafe fn removeCookiesSinceDate(&self, date: &NSDate);
        # [method_id (cookiesForURL :)]
        pub unsafe fn cookiesForURL(
            &self,
            URL: &NSURL,
        ) -> Option<Id<NSArray<NSHTTPCookie>, Shared>>;
        # [method (setCookies : forURL : mainDocumentURL :)]
        pub unsafe fn setCookies_forURL_mainDocumentURL(
            &self,
            cookies: &NSArray<NSHTTPCookie>,
            URL: Option<&NSURL>,
            mainDocumentURL: Option<&NSURL>,
        );
        #[method(cookieAcceptPolicy)]
        pub unsafe fn cookieAcceptPolicy(&self) -> NSHTTPCookieAcceptPolicy;
        # [method (setCookieAcceptPolicy :)]
        pub unsafe fn setCookieAcceptPolicy(&self, cookieAcceptPolicy: NSHTTPCookieAcceptPolicy);
        # [method_id (sortedCookiesUsingDescriptors :)]
        pub unsafe fn sortedCookiesUsingDescriptors(
            &self,
            sortOrder: &NSArray<NSSortDescriptor>,
        ) -> Id<NSArray<NSHTTPCookie>, Shared>;
    }
);
extern_methods!(
    #[doc = "NSURLSessionTaskAdditions"]
    unsafe impl NSHTTPCookieStorage {
        # [method (storeCookies : forTask :)]
        pub unsafe fn storeCookies_forTask(
            &self,
            cookies: &NSArray<NSHTTPCookie>,
            task: &NSURLSessionTask,
        );
        # [method (getCookiesForTask : completionHandler :)]
        pub unsafe fn getCookiesForTask_completionHandler(
            &self,
            task: &NSURLSessionTask,
            completionHandler: TodoBlock,
        );
    }
);
