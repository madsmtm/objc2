#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSURLRequest;
    unsafe impl ClassType for NSURLRequest {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSURLRequest {
        #[method_id(requestWithURL:)]
        pub unsafe fn requestWithURL(URL: &NSURL) -> Id<Self, Shared>;
        #[method(supportsSecureCoding)]
        pub unsafe fn supportsSecureCoding() -> bool;
        #[method_id(requestWithURL:cachePolicy:timeoutInterval:)]
        pub unsafe fn requestWithURL_cachePolicy_timeoutInterval(
            URL: &NSURL,
            cachePolicy: NSURLRequestCachePolicy,
            timeoutInterval: NSTimeInterval,
        ) -> Id<Self, Shared>;
        #[method_id(initWithURL:)]
        pub unsafe fn initWithURL(&self, URL: &NSURL) -> Id<Self, Shared>;
        #[method_id(initWithURL:cachePolicy:timeoutInterval:)]
        pub unsafe fn initWithURL_cachePolicy_timeoutInterval(
            &self,
            URL: &NSURL,
            cachePolicy: NSURLRequestCachePolicy,
            timeoutInterval: NSTimeInterval,
        ) -> Id<Self, Shared>;
        #[method_id(URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL, Shared>>;
        #[method(cachePolicy)]
        pub unsafe fn cachePolicy(&self) -> NSURLRequestCachePolicy;
        #[method(timeoutInterval)]
        pub unsafe fn timeoutInterval(&self) -> NSTimeInterval;
        #[method_id(mainDocumentURL)]
        pub unsafe fn mainDocumentURL(&self) -> Option<Id<NSURL, Shared>>;
        #[method(networkServiceType)]
        pub unsafe fn networkServiceType(&self) -> NSURLRequestNetworkServiceType;
        #[method(allowsCellularAccess)]
        pub unsafe fn allowsCellularAccess(&self) -> bool;
        #[method(allowsExpensiveNetworkAccess)]
        pub unsafe fn allowsExpensiveNetworkAccess(&self) -> bool;
        #[method(allowsConstrainedNetworkAccess)]
        pub unsafe fn allowsConstrainedNetworkAccess(&self) -> bool;
        #[method(assumesHTTP3Capable)]
        pub unsafe fn assumesHTTP3Capable(&self) -> bool;
        #[method(attribution)]
        pub unsafe fn attribution(&self) -> NSURLRequestAttribution;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSMutableURLRequest;
    unsafe impl ClassType for NSMutableURLRequest {
        type Super = NSURLRequest;
    }
);
extern_methods!(
    unsafe impl NSMutableURLRequest {
        #[method_id(URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL, Shared>>;
        #[method(setURL:)]
        pub unsafe fn setURL(&self, URL: Option<&NSURL>);
        #[method(cachePolicy)]
        pub unsafe fn cachePolicy(&self) -> NSURLRequestCachePolicy;
        #[method(setCachePolicy:)]
        pub unsafe fn setCachePolicy(&self, cachePolicy: NSURLRequestCachePolicy);
        #[method(timeoutInterval)]
        pub unsafe fn timeoutInterval(&self) -> NSTimeInterval;
        #[method(setTimeoutInterval:)]
        pub unsafe fn setTimeoutInterval(&self, timeoutInterval: NSTimeInterval);
        #[method_id(mainDocumentURL)]
        pub unsafe fn mainDocumentURL(&self) -> Option<Id<NSURL, Shared>>;
        #[method(setMainDocumentURL:)]
        pub unsafe fn setMainDocumentURL(&self, mainDocumentURL: Option<&NSURL>);
        #[method(networkServiceType)]
        pub unsafe fn networkServiceType(&self) -> NSURLRequestNetworkServiceType;
        #[method(setNetworkServiceType:)]
        pub unsafe fn setNetworkServiceType(
            &self,
            networkServiceType: NSURLRequestNetworkServiceType,
        );
        #[method(allowsCellularAccess)]
        pub unsafe fn allowsCellularAccess(&self) -> bool;
        #[method(setAllowsCellularAccess:)]
        pub unsafe fn setAllowsCellularAccess(&self, allowsCellularAccess: bool);
        #[method(allowsExpensiveNetworkAccess)]
        pub unsafe fn allowsExpensiveNetworkAccess(&self) -> bool;
        #[method(setAllowsExpensiveNetworkAccess:)]
        pub unsafe fn setAllowsExpensiveNetworkAccess(&self, allowsExpensiveNetworkAccess: bool);
        #[method(allowsConstrainedNetworkAccess)]
        pub unsafe fn allowsConstrainedNetworkAccess(&self) -> bool;
        #[method(setAllowsConstrainedNetworkAccess:)]
        pub unsafe fn setAllowsConstrainedNetworkAccess(
            &self,
            allowsConstrainedNetworkAccess: bool,
        );
        #[method(assumesHTTP3Capable)]
        pub unsafe fn assumesHTTP3Capable(&self) -> bool;
        #[method(setAssumesHTTP3Capable:)]
        pub unsafe fn setAssumesHTTP3Capable(&self, assumesHTTP3Capable: bool);
        #[method(attribution)]
        pub unsafe fn attribution(&self) -> NSURLRequestAttribution;
        #[method(setAttribution:)]
        pub unsafe fn setAttribution(&self, attribution: NSURLRequestAttribution);
    }
);
extern_methods!(
    #[doc = "NSHTTPURLRequest"]
    unsafe impl NSURLRequest {
        #[method_id(HTTPMethod)]
        pub unsafe fn HTTPMethod(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(allHTTPHeaderFields)]
        pub unsafe fn allHTTPHeaderFields(
            &self,
        ) -> Option<Id<NSDictionary<NSString, NSString>, Shared>>;
        #[method_id(valueForHTTPHeaderField:)]
        pub unsafe fn valueForHTTPHeaderField(
            &self,
            field: &NSString,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(HTTPBody)]
        pub unsafe fn HTTPBody(&self) -> Option<Id<NSData, Shared>>;
        #[method_id(HTTPBodyStream)]
        pub unsafe fn HTTPBodyStream(&self) -> Option<Id<NSInputStream, Shared>>;
        #[method(HTTPShouldHandleCookies)]
        pub unsafe fn HTTPShouldHandleCookies(&self) -> bool;
        #[method(HTTPShouldUsePipelining)]
        pub unsafe fn HTTPShouldUsePipelining(&self) -> bool;
    }
);
extern_methods!(
    #[doc = "NSMutableHTTPURLRequest"]
    unsafe impl NSMutableURLRequest {
        #[method_id(HTTPMethod)]
        pub unsafe fn HTTPMethod(&self) -> Id<NSString, Shared>;
        #[method(setHTTPMethod:)]
        pub unsafe fn setHTTPMethod(&self, HTTPMethod: &NSString);
        #[method_id(allHTTPHeaderFields)]
        pub unsafe fn allHTTPHeaderFields(
            &self,
        ) -> Option<Id<NSDictionary<NSString, NSString>, Shared>>;
        #[method(setAllHTTPHeaderFields:)]
        pub unsafe fn setAllHTTPHeaderFields(
            &self,
            allHTTPHeaderFields: Option<&NSDictionary<NSString, NSString>>,
        );
        #[method(setValue:forHTTPHeaderField:)]
        pub unsafe fn setValue_forHTTPHeaderField(
            &self,
            value: Option<&NSString>,
            field: &NSString,
        );
        #[method(addValue:forHTTPHeaderField:)]
        pub unsafe fn addValue_forHTTPHeaderField(&self, value: &NSString, field: &NSString);
        #[method_id(HTTPBody)]
        pub unsafe fn HTTPBody(&self) -> Option<Id<NSData, Shared>>;
        #[method(setHTTPBody:)]
        pub unsafe fn setHTTPBody(&self, HTTPBody: Option<&NSData>);
        #[method_id(HTTPBodyStream)]
        pub unsafe fn HTTPBodyStream(&self) -> Option<Id<NSInputStream, Shared>>;
        #[method(setHTTPBodyStream:)]
        pub unsafe fn setHTTPBodyStream(&self, HTTPBodyStream: Option<&NSInputStream>);
        #[method(HTTPShouldHandleCookies)]
        pub unsafe fn HTTPShouldHandleCookies(&self) -> bool;
        #[method(setHTTPShouldHandleCookies:)]
        pub unsafe fn setHTTPShouldHandleCookies(&self, HTTPShouldHandleCookies: bool);
        #[method(HTTPShouldUsePipelining)]
        pub unsafe fn HTTPShouldUsePipelining(&self) -> bool;
        #[method(setHTTPShouldUsePipelining:)]
        pub unsafe fn setHTTPShouldUsePipelining(&self, HTTPShouldUsePipelining: bool);
    }
);
