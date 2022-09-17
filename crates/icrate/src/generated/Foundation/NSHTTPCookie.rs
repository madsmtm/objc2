#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    struct NSHTTPCookie;
    unsafe impl ClassType for NSHTTPCookie {
        type Super = NSObject;
    }
);
impl NSHTTPCookie {
    pub unsafe fn initWithProperties(&self, properties: TodoGenerics) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithProperties: properties]
    }
    pub unsafe fn cookieWithProperties(
        properties: TodoGenerics,
    ) -> Option<Id<NSHTTPCookie, Shared>> {
        msg_send_id![Self::class(), cookieWithProperties: properties]
    }
    pub unsafe fn requestHeaderFieldsWithCookies(cookies: TodoGenerics) -> TodoGenerics {
        msg_send![Self::class(), requestHeaderFieldsWithCookies: cookies]
    }
    pub unsafe fn cookiesWithResponseHeaderFields_forURL(
        headerFields: TodoGenerics,
        URL: &NSURL,
    ) -> TodoGenerics {
        msg_send![
            Self::class(),
            cookiesWithResponseHeaderFields: headerFields,
            forURL: URL
        ]
    }
    pub unsafe fn properties(&self) -> TodoGenerics {
        msg_send![self, properties]
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
    pub unsafe fn portList(&self) -> TodoGenerics {
        msg_send![self, portList]
    }
    pub unsafe fn sameSitePolicy(&self) -> NSHTTPCookieStringPolicy {
        msg_send![self, sameSitePolicy]
    }
}
