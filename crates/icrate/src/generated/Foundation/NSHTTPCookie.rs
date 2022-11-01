//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

pub type NSHTTPCookiePropertyKey = NSString;

pub type NSHTTPCookieStringPolicy = NSString;

extern "C" {
    pub static NSHTTPCookieName: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    pub static NSHTTPCookieValue: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    pub static NSHTTPCookieOriginURL: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    pub static NSHTTPCookieVersion: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    pub static NSHTTPCookieDomain: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    pub static NSHTTPCookiePath: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    pub static NSHTTPCookieSecure: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    pub static NSHTTPCookieExpires: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    pub static NSHTTPCookieComment: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    pub static NSHTTPCookieCommentURL: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    pub static NSHTTPCookieDiscard: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    pub static NSHTTPCookieMaximumAge: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    pub static NSHTTPCookiePort: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    pub static NSHTTPCookieSameSitePolicy: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    pub static NSHTTPCookieSameSiteLax: &'static NSHTTPCookieStringPolicy;
}

extern "C" {
    pub static NSHTTPCookieSameSiteStrict: &'static NSHTTPCookieStringPolicy;
}

extern_class!(
    #[derive(Debug)]
    pub struct NSHTTPCookie;

    unsafe impl ClassType for NSHTTPCookie {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSHTTPCookie {
        #[method_id(@__retain_semantics Init initWithProperties:)]
        pub unsafe fn initWithProperties(
            this: Option<Allocated<Self>>,
            properties: &NSDictionary<NSHTTPCookiePropertyKey, Object>,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other cookieWithProperties:)]
        pub unsafe fn cookieWithProperties(
            properties: &NSDictionary<NSHTTPCookiePropertyKey, Object>,
        ) -> Option<Id<NSHTTPCookie, Shared>>;

        #[method_id(@__retain_semantics Other requestHeaderFieldsWithCookies:)]
        pub unsafe fn requestHeaderFieldsWithCookies(
            cookies: &NSArray<NSHTTPCookie>,
        ) -> Id<NSDictionary<NSString, NSString>, Shared>;

        #[method_id(@__retain_semantics Other cookiesWithResponseHeaderFields:forURL:)]
        pub unsafe fn cookiesWithResponseHeaderFields_forURL(
            headerFields: &NSDictionary<NSString, NSString>,
            URL: &NSURL,
        ) -> Id<NSArray<NSHTTPCookie>, Shared>;

        #[method_id(@__retain_semantics Other properties)]
        pub unsafe fn properties(
            &self,
        ) -> Option<Id<NSDictionary<NSHTTPCookiePropertyKey, Object>, Shared>>;

        #[method(version)]
        pub unsafe fn version(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other expiresDate)]
        pub unsafe fn expiresDate(&self) -> Option<Id<NSDate, Shared>>;

        #[method(isSessionOnly)]
        pub unsafe fn isSessionOnly(&self) -> bool;

        #[method_id(@__retain_semantics Other domain)]
        pub unsafe fn domain(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other path)]
        pub unsafe fn path(&self) -> Id<NSString, Shared>;

        #[method(isSecure)]
        pub unsafe fn isSecure(&self) -> bool;

        #[method(isHTTPOnly)]
        pub unsafe fn isHTTPOnly(&self) -> bool;

        #[method_id(@__retain_semantics Other comment)]
        pub unsafe fn comment(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other commentURL)]
        pub unsafe fn commentURL(&self) -> Option<Id<NSURL, Shared>>;

        #[method_id(@__retain_semantics Other portList)]
        pub unsafe fn portList(&self) -> Option<Id<NSArray<NSNumber>, Shared>>;

        #[method_id(@__retain_semantics Other sameSitePolicy)]
        pub unsafe fn sameSitePolicy(&self) -> Option<Id<NSHTTPCookieStringPolicy, Shared>>;
    }
);
