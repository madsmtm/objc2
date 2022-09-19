use super::__exported::NSArray;
use super::__exported::NSDate;
use super::__exported::NSDictionary;
use super::__exported::NSNumber;
use super::__exported::NSString;
use super::__exported::NSURL;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
pub type NSHTTPCookiePropertyKey = NSString;
pub type NSHTTPCookieStringPolicy = NSString;
use super::__exported::NSHTTPCookieInternal;
extern_class!(
    #[derive(Debug)]
    pub struct NSHTTPCookie;
    unsafe impl ClassType for NSHTTPCookie {
        type Super = NSObject;
    }
);
impl NSHTTPCookie {
    pub unsafe fn initWithProperties(
        &self,
        properties: &NSDictionary<NSHTTPCookiePropertyKey, Object>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithProperties: properties]
    }
    pub unsafe fn cookieWithProperties(
        properties: &NSDictionary<NSHTTPCookiePropertyKey, Object>,
    ) -> Option<Id<NSHTTPCookie, Shared>> {
        msg_send_id![Self::class(), cookieWithProperties: properties]
    }
    pub unsafe fn requestHeaderFieldsWithCookies(
        cookies: &NSArray<NSHTTPCookie>,
    ) -> Id<NSDictionary<NSString, NSString>, Shared> {
        msg_send_id![Self::class(), requestHeaderFieldsWithCookies: cookies]
    }
    pub unsafe fn cookiesWithResponseHeaderFields_forURL(
        headerFields: &NSDictionary<NSString, NSString>,
        URL: &NSURL,
    ) -> Id<NSArray<NSHTTPCookie>, Shared> {
        msg_send_id![
            Self::class(),
            cookiesWithResponseHeaderFields: headerFields,
            forURL: URL
        ]
    }
    pub unsafe fn properties(
        &self,
    ) -> Option<Id<NSDictionary<NSHTTPCookiePropertyKey, Object>, Shared>> {
        msg_send_id![self, properties]
    }
    pub unsafe fn version(&self) -> NSUInteger {
        msg_send![self, version]
    }
    pub unsafe fn name(&self) -> Id<NSString, Shared> {
        msg_send_id![self, name]
    }
    pub unsafe fn value(&self) -> Id<NSString, Shared> {
        msg_send_id![self, value]
    }
    pub unsafe fn expiresDate(&self) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, expiresDate]
    }
    pub unsafe fn isSessionOnly(&self) -> bool {
        msg_send![self, isSessionOnly]
    }
    pub unsafe fn domain(&self) -> Id<NSString, Shared> {
        msg_send_id![self, domain]
    }
    pub unsafe fn path(&self) -> Id<NSString, Shared> {
        msg_send_id![self, path]
    }
    pub unsafe fn isSecure(&self) -> bool {
        msg_send![self, isSecure]
    }
    pub unsafe fn isHTTPOnly(&self) -> bool {
        msg_send![self, isHTTPOnly]
    }
    pub unsafe fn comment(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, comment]
    }
    pub unsafe fn commentURL(&self) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, commentURL]
    }
    pub unsafe fn portList(&self) -> Option<Id<NSArray<NSNumber>, Shared>> {
        msg_send_id![self, portList]
    }
    pub unsafe fn sameSitePolicy(&self) -> Option<Id<NSHTTPCookieStringPolicy, Shared>> {
        msg_send_id![self, sameSitePolicy]
    }
}
