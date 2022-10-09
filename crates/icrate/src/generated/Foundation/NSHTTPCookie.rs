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
use objc2::{extern_class, extern_methods, ClassType};
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
extern_methods!(
    unsafe impl NSHTTPCookie {
        #[method_id(initWithProperties:)]
        pub unsafe fn initWithProperties(
            &self,
            properties: &NSDictionary<NSHTTPCookiePropertyKey, Object>,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(cookieWithProperties:)]
        pub unsafe fn cookieWithProperties(
            properties: &NSDictionary<NSHTTPCookiePropertyKey, Object>,
        ) -> Option<Id<NSHTTPCookie, Shared>>;
        #[method_id(requestHeaderFieldsWithCookies:)]
        pub unsafe fn requestHeaderFieldsWithCookies(
            cookies: &NSArray<NSHTTPCookie>,
        ) -> Id<NSDictionary<NSString, NSString>, Shared>;
        #[method_id(cookiesWithResponseHeaderFields:forURL:)]
        pub unsafe fn cookiesWithResponseHeaderFields_forURL(
            headerFields: &NSDictionary<NSString, NSString>,
            URL: &NSURL,
        ) -> Id<NSArray<NSHTTPCookie>, Shared>;
        #[method_id(properties)]
        pub unsafe fn properties(
            &self,
        ) -> Option<Id<NSDictionary<NSHTTPCookiePropertyKey, Object>, Shared>>;
        #[method(version)]
        pub unsafe fn version(&self) -> NSUInteger;
        #[method_id(name)]
        pub unsafe fn name(&self) -> Id<NSString, Shared>;
        #[method_id(value)]
        pub unsafe fn value(&self) -> Id<NSString, Shared>;
        #[method_id(expiresDate)]
        pub unsafe fn expiresDate(&self) -> Option<Id<NSDate, Shared>>;
        #[method(isSessionOnly)]
        pub unsafe fn isSessionOnly(&self) -> bool;
        #[method_id(domain)]
        pub unsafe fn domain(&self) -> Id<NSString, Shared>;
        #[method_id(path)]
        pub unsafe fn path(&self) -> Id<NSString, Shared>;
        #[method(isSecure)]
        pub unsafe fn isSecure(&self) -> bool;
        #[method(isHTTPOnly)]
        pub unsafe fn isHTTPOnly(&self) -> bool;
        #[method_id(comment)]
        pub unsafe fn comment(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(commentURL)]
        pub unsafe fn commentURL(&self) -> Option<Id<NSURL, Shared>>;
        #[method_id(portList)]
        pub unsafe fn portList(&self) -> Option<Id<NSArray<NSNumber>, Shared>>;
        #[method_id(sameSitePolicy)]
        pub unsafe fn sameSitePolicy(&self) -> Option<Id<NSHTTPCookieStringPolicy, Shared>>;
    }
);
