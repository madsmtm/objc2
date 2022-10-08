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
use objc2::{extern_class, extern_methods, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSHTTPCookieStorage;
    unsafe impl ClassType for NSHTTPCookieStorage {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSHTTPCookieStorage {
        pub unsafe fn sharedHTTPCookieStorage() -> Id<NSHTTPCookieStorage, Shared> {
            msg_send_id![Self::class(), sharedHTTPCookieStorage]
        }
        pub unsafe fn sharedCookieStorageForGroupContainerIdentifier(
            identifier: &NSString,
        ) -> Id<NSHTTPCookieStorage, Shared> {
            msg_send_id![
                Self::class(),
                sharedCookieStorageForGroupContainerIdentifier: identifier
            ]
        }
        pub unsafe fn cookies(&self) -> Option<Id<NSArray<NSHTTPCookie>, Shared>> {
            msg_send_id![self, cookies]
        }
        pub unsafe fn setCookie(&self, cookie: &NSHTTPCookie) {
            msg_send![self, setCookie: cookie]
        }
        pub unsafe fn deleteCookie(&self, cookie: &NSHTTPCookie) {
            msg_send![self, deleteCookie: cookie]
        }
        pub unsafe fn removeCookiesSinceDate(&self, date: &NSDate) {
            msg_send![self, removeCookiesSinceDate: date]
        }
        pub unsafe fn cookiesForURL(
            &self,
            URL: &NSURL,
        ) -> Option<Id<NSArray<NSHTTPCookie>, Shared>> {
            msg_send_id![self, cookiesForURL: URL]
        }
        pub unsafe fn setCookies_forURL_mainDocumentURL(
            &self,
            cookies: &NSArray<NSHTTPCookie>,
            URL: Option<&NSURL>,
            mainDocumentURL: Option<&NSURL>,
        ) {
            msg_send![
                self,
                setCookies: cookies,
                forURL: URL,
                mainDocumentURL: mainDocumentURL
            ]
        }
        pub unsafe fn cookieAcceptPolicy(&self) -> NSHTTPCookieAcceptPolicy {
            msg_send![self, cookieAcceptPolicy]
        }
        pub unsafe fn setCookieAcceptPolicy(&self, cookieAcceptPolicy: NSHTTPCookieAcceptPolicy) {
            msg_send![self, setCookieAcceptPolicy: cookieAcceptPolicy]
        }
        pub unsafe fn sortedCookiesUsingDescriptors(
            &self,
            sortOrder: &NSArray<NSSortDescriptor>,
        ) -> Id<NSArray<NSHTTPCookie>, Shared> {
            msg_send_id![self, sortedCookiesUsingDescriptors: sortOrder]
        }
    }
);
extern_methods!(
    #[doc = "NSURLSessionTaskAdditions"]
    unsafe impl NSHTTPCookieStorage {
        pub unsafe fn storeCookies_forTask(
            &self,
            cookies: &NSArray<NSHTTPCookie>,
            task: &NSURLSessionTask,
        ) {
            msg_send![self, storeCookies: cookies, forTask: task]
        }
        pub unsafe fn getCookiesForTask_completionHandler(
            &self,
            task: &NSURLSessionTask,
            completionHandler: TodoBlock,
        ) {
            msg_send![
                self,
                getCookiesForTask: task,
                completionHandler: completionHandler
            ]
        }
    }
);
