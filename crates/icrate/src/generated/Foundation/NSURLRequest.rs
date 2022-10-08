use super::__exported::NSData;
use super::__exported::NSDictionary;
use super::__exported::NSInputStream;
use super::__exported::NSString;
use super::__exported::NSURLRequestInternal;
use super::__exported::NSURL;
use crate::Foundation::generated::NSDate::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSURLRequest;
    unsafe impl ClassType for NSURLRequest {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSURLRequest {
        pub unsafe fn requestWithURL(URL: &NSURL) -> Id<Self, Shared> {
            msg_send_id![Self::class(), requestWithURL: URL]
        }
        pub unsafe fn supportsSecureCoding() -> bool {
            msg_send![Self::class(), supportsSecureCoding]
        }
        pub unsafe fn requestWithURL_cachePolicy_timeoutInterval(
            URL: &NSURL,
            cachePolicy: NSURLRequestCachePolicy,
            timeoutInterval: NSTimeInterval,
        ) -> Id<Self, Shared> {
            msg_send_id![
                Self::class(),
                requestWithURL: URL,
                cachePolicy: cachePolicy,
                timeoutInterval: timeoutInterval
            ]
        }
        pub unsafe fn initWithURL(&self, URL: &NSURL) -> Id<Self, Shared> {
            msg_send_id![self, initWithURL: URL]
        }
        pub unsafe fn initWithURL_cachePolicy_timeoutInterval(
            &self,
            URL: &NSURL,
            cachePolicy: NSURLRequestCachePolicy,
            timeoutInterval: NSTimeInterval,
        ) -> Id<Self, Shared> {
            msg_send_id![
                self,
                initWithURL: URL,
                cachePolicy: cachePolicy,
                timeoutInterval: timeoutInterval
            ]
        }
        pub unsafe fn URL(&self) -> Option<Id<NSURL, Shared>> {
            msg_send_id![self, URL]
        }
        pub unsafe fn cachePolicy(&self) -> NSURLRequestCachePolicy {
            msg_send![self, cachePolicy]
        }
        pub unsafe fn timeoutInterval(&self) -> NSTimeInterval {
            msg_send![self, timeoutInterval]
        }
        pub unsafe fn mainDocumentURL(&self) -> Option<Id<NSURL, Shared>> {
            msg_send_id![self, mainDocumentURL]
        }
        pub unsafe fn networkServiceType(&self) -> NSURLRequestNetworkServiceType {
            msg_send![self, networkServiceType]
        }
        pub unsafe fn allowsCellularAccess(&self) -> bool {
            msg_send![self, allowsCellularAccess]
        }
        pub unsafe fn allowsExpensiveNetworkAccess(&self) -> bool {
            msg_send![self, allowsExpensiveNetworkAccess]
        }
        pub unsafe fn allowsConstrainedNetworkAccess(&self) -> bool {
            msg_send![self, allowsConstrainedNetworkAccess]
        }
        pub unsafe fn assumesHTTP3Capable(&self) -> bool {
            msg_send![self, assumesHTTP3Capable]
        }
        pub unsafe fn attribution(&self) -> NSURLRequestAttribution {
            msg_send![self, attribution]
        }
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
        pub unsafe fn URL(&self) -> Option<Id<NSURL, Shared>> {
            msg_send_id![self, URL]
        }
        pub unsafe fn setURL(&self, URL: Option<&NSURL>) {
            msg_send![self, setURL: URL]
        }
        pub unsafe fn cachePolicy(&self) -> NSURLRequestCachePolicy {
            msg_send![self, cachePolicy]
        }
        pub unsafe fn setCachePolicy(&self, cachePolicy: NSURLRequestCachePolicy) {
            msg_send![self, setCachePolicy: cachePolicy]
        }
        pub unsafe fn timeoutInterval(&self) -> NSTimeInterval {
            msg_send![self, timeoutInterval]
        }
        pub unsafe fn setTimeoutInterval(&self, timeoutInterval: NSTimeInterval) {
            msg_send![self, setTimeoutInterval: timeoutInterval]
        }
        pub unsafe fn mainDocumentURL(&self) -> Option<Id<NSURL, Shared>> {
            msg_send_id![self, mainDocumentURL]
        }
        pub unsafe fn setMainDocumentURL(&self, mainDocumentURL: Option<&NSURL>) {
            msg_send![self, setMainDocumentURL: mainDocumentURL]
        }
        pub unsafe fn networkServiceType(&self) -> NSURLRequestNetworkServiceType {
            msg_send![self, networkServiceType]
        }
        pub unsafe fn setNetworkServiceType(
            &self,
            networkServiceType: NSURLRequestNetworkServiceType,
        ) {
            msg_send![self, setNetworkServiceType: networkServiceType]
        }
        pub unsafe fn allowsCellularAccess(&self) -> bool {
            msg_send![self, allowsCellularAccess]
        }
        pub unsafe fn setAllowsCellularAccess(&self, allowsCellularAccess: bool) {
            msg_send![self, setAllowsCellularAccess: allowsCellularAccess]
        }
        pub unsafe fn allowsExpensiveNetworkAccess(&self) -> bool {
            msg_send![self, allowsExpensiveNetworkAccess]
        }
        pub unsafe fn setAllowsExpensiveNetworkAccess(&self, allowsExpensiveNetworkAccess: bool) {
            msg_send![
                self,
                setAllowsExpensiveNetworkAccess: allowsExpensiveNetworkAccess
            ]
        }
        pub unsafe fn allowsConstrainedNetworkAccess(&self) -> bool {
            msg_send![self, allowsConstrainedNetworkAccess]
        }
        pub unsafe fn setAllowsConstrainedNetworkAccess(
            &self,
            allowsConstrainedNetworkAccess: bool,
        ) {
            msg_send![
                self,
                setAllowsConstrainedNetworkAccess: allowsConstrainedNetworkAccess
            ]
        }
        pub unsafe fn assumesHTTP3Capable(&self) -> bool {
            msg_send![self, assumesHTTP3Capable]
        }
        pub unsafe fn setAssumesHTTP3Capable(&self, assumesHTTP3Capable: bool) {
            msg_send![self, setAssumesHTTP3Capable: assumesHTTP3Capable]
        }
        pub unsafe fn attribution(&self) -> NSURLRequestAttribution {
            msg_send![self, attribution]
        }
        pub unsafe fn setAttribution(&self, attribution: NSURLRequestAttribution) {
            msg_send![self, setAttribution: attribution]
        }
    }
);
extern_methods!(
    #[doc = "NSHTTPURLRequest"]
    unsafe impl NSURLRequest {
        pub unsafe fn HTTPMethod(&self) -> Option<Id<NSString, Shared>> {
            msg_send_id![self, HTTPMethod]
        }
        pub unsafe fn allHTTPHeaderFields(
            &self,
        ) -> Option<Id<NSDictionary<NSString, NSString>, Shared>> {
            msg_send_id![self, allHTTPHeaderFields]
        }
        pub unsafe fn valueForHTTPHeaderField(
            &self,
            field: &NSString,
        ) -> Option<Id<NSString, Shared>> {
            msg_send_id![self, valueForHTTPHeaderField: field]
        }
        pub unsafe fn HTTPBody(&self) -> Option<Id<NSData, Shared>> {
            msg_send_id![self, HTTPBody]
        }
        pub unsafe fn HTTPBodyStream(&self) -> Option<Id<NSInputStream, Shared>> {
            msg_send_id![self, HTTPBodyStream]
        }
        pub unsafe fn HTTPShouldHandleCookies(&self) -> bool {
            msg_send![self, HTTPShouldHandleCookies]
        }
        pub unsafe fn HTTPShouldUsePipelining(&self) -> bool {
            msg_send![self, HTTPShouldUsePipelining]
        }
    }
);
extern_methods!(
    #[doc = "NSMutableHTTPURLRequest"]
    unsafe impl NSMutableURLRequest {
        pub unsafe fn HTTPMethod(&self) -> Id<NSString, Shared> {
            msg_send_id![self, HTTPMethod]
        }
        pub unsafe fn setHTTPMethod(&self, HTTPMethod: &NSString) {
            msg_send![self, setHTTPMethod: HTTPMethod]
        }
        pub unsafe fn allHTTPHeaderFields(
            &self,
        ) -> Option<Id<NSDictionary<NSString, NSString>, Shared>> {
            msg_send_id![self, allHTTPHeaderFields]
        }
        pub unsafe fn setAllHTTPHeaderFields(
            &self,
            allHTTPHeaderFields: Option<&NSDictionary<NSString, NSString>>,
        ) {
            msg_send![self, setAllHTTPHeaderFields: allHTTPHeaderFields]
        }
        pub unsafe fn setValue_forHTTPHeaderField(
            &self,
            value: Option<&NSString>,
            field: &NSString,
        ) {
            msg_send![self, setValue: value, forHTTPHeaderField: field]
        }
        pub unsafe fn addValue_forHTTPHeaderField(&self, value: &NSString, field: &NSString) {
            msg_send![self, addValue: value, forHTTPHeaderField: field]
        }
        pub unsafe fn HTTPBody(&self) -> Option<Id<NSData, Shared>> {
            msg_send_id![self, HTTPBody]
        }
        pub unsafe fn setHTTPBody(&self, HTTPBody: Option<&NSData>) {
            msg_send![self, setHTTPBody: HTTPBody]
        }
        pub unsafe fn HTTPBodyStream(&self) -> Option<Id<NSInputStream, Shared>> {
            msg_send_id![self, HTTPBodyStream]
        }
        pub unsafe fn setHTTPBodyStream(&self, HTTPBodyStream: Option<&NSInputStream>) {
            msg_send![self, setHTTPBodyStream: HTTPBodyStream]
        }
        pub unsafe fn HTTPShouldHandleCookies(&self) -> bool {
            msg_send![self, HTTPShouldHandleCookies]
        }
        pub unsafe fn setHTTPShouldHandleCookies(&self, HTTPShouldHandleCookies: bool) {
            msg_send![self, setHTTPShouldHandleCookies: HTTPShouldHandleCookies]
        }
        pub unsafe fn HTTPShouldUsePipelining(&self) -> bool {
            msg_send![self, HTTPShouldUsePipelining]
        }
        pub unsafe fn setHTTPShouldUsePipelining(&self, HTTPShouldUsePipelining: bool) {
            msg_send![self, setHTTPShouldUsePipelining: HTTPShouldUsePipelining]
        }
    }
);
