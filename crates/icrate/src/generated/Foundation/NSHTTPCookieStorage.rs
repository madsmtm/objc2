use super::NSArray;
use super::NSDate;
use super::NSHTTPCookie;
use super::NSHTTPCookieStorageInternal;
use super::NSSortDescriptor;
use super::NSURLSessionTask;
use super::NSURL;
use crate::Foundation::generated::NSNotification::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSHTTPCookieStorage;
    unsafe impl ClassType for NSHTTPCookieStorage {
        type Super = NSObject;
    }
);
impl NSHTTPCookieStorage {
    pub unsafe fn sharedCookieStorageForGroupContainerIdentifier(
        identifier: &NSString,
    ) -> Id<NSHTTPCookieStorage, Shared> {
        msg_send_id![
            Self::class(),
            sharedCookieStorageForGroupContainerIdentifier: identifier
        ]
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
    pub unsafe fn cookiesForURL(&self, URL: &NSURL) -> TodoGenerics {
        msg_send![self, cookiesForURL: URL]
    }
    pub unsafe fn setCookies_forURL_mainDocumentURL(
        &self,
        cookies: TodoGenerics,
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
    pub unsafe fn sortedCookiesUsingDescriptors(&self, sortOrder: TodoGenerics) -> TodoGenerics {
        msg_send![self, sortedCookiesUsingDescriptors: sortOrder]
    }
    pub unsafe fn sharedHTTPCookieStorage() -> Id<NSHTTPCookieStorage, Shared> {
        msg_send_id![Self::class(), sharedHTTPCookieStorage]
    }
    pub unsafe fn cookies(&self) -> TodoGenerics {
        msg_send![self, cookies]
    }
    pub unsafe fn cookieAcceptPolicy(&self) -> NSHTTPCookieAcceptPolicy {
        msg_send![self, cookieAcceptPolicy]
    }
    pub unsafe fn setCookieAcceptPolicy(&self, cookieAcceptPolicy: NSHTTPCookieAcceptPolicy) {
        msg_send![self, setCookieAcceptPolicy: cookieAcceptPolicy]
    }
}
#[doc = "NSURLSessionTaskAdditions"]
impl NSHTTPCookieStorage {
    pub unsafe fn storeCookies_forTask(&self, cookies: TodoGenerics, task: &NSURLSessionTask) {
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
