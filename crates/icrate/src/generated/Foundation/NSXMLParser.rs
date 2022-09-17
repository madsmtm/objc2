use super::__exported::NSData;
use super::__exported::NSDictionary;
use super::__exported::NSError;
use super::__exported::NSInputStream;
use super::__exported::NSSet;
use super::__exported::NSString;
use super::__exported::NSURL;
use crate::Foundation::generated::NSError::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSXMLParser;
    unsafe impl ClassType for NSXMLParser {
        type Super = NSObject;
    }
);
impl NSXMLParser {
    pub unsafe fn initWithContentsOfURL(&self, url: &NSURL) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithContentsOfURL: url]
    }
    pub unsafe fn initWithData(&self, data: &NSData) -> Id<Self, Shared> {
        msg_send_id![self, initWithData: data]
    }
    pub unsafe fn initWithStream(&self, stream: &NSInputStream) -> Id<Self, Shared> {
        msg_send_id![self, initWithStream: stream]
    }
    pub unsafe fn parse(&self) -> bool {
        msg_send![self, parse]
    }
    pub unsafe fn abortParsing(&self) {
        msg_send![self, abortParsing]
    }
    pub unsafe fn delegate(&self) -> TodoGenerics {
        msg_send![self, delegate]
    }
    pub unsafe fn setDelegate(&self, delegate: TodoGenerics) {
        msg_send![self, setDelegate: delegate]
    }
    pub unsafe fn shouldProcessNamespaces(&self) -> bool {
        msg_send![self, shouldProcessNamespaces]
    }
    pub unsafe fn setShouldProcessNamespaces(&self, shouldProcessNamespaces: bool) {
        msg_send![self, setShouldProcessNamespaces: shouldProcessNamespaces]
    }
    pub unsafe fn shouldReportNamespacePrefixes(&self) -> bool {
        msg_send![self, shouldReportNamespacePrefixes]
    }
    pub unsafe fn setShouldReportNamespacePrefixes(&self, shouldReportNamespacePrefixes: bool) {
        msg_send![
            self,
            setShouldReportNamespacePrefixes: shouldReportNamespacePrefixes
        ]
    }
    pub unsafe fn externalEntityResolvingPolicy(&self) -> NSXMLParserExternalEntityResolvingPolicy {
        msg_send![self, externalEntityResolvingPolicy]
    }
    pub unsafe fn setExternalEntityResolvingPolicy(
        &self,
        externalEntityResolvingPolicy: NSXMLParserExternalEntityResolvingPolicy,
    ) {
        msg_send![
            self,
            setExternalEntityResolvingPolicy: externalEntityResolvingPolicy
        ]
    }
    pub unsafe fn allowedExternalEntityURLs(&self) -> TodoGenerics {
        msg_send![self, allowedExternalEntityURLs]
    }
    pub unsafe fn setAllowedExternalEntityURLs(&self, allowedExternalEntityURLs: TodoGenerics) {
        msg_send![
            self,
            setAllowedExternalEntityURLs: allowedExternalEntityURLs
        ]
    }
    pub unsafe fn parserError(&self) -> Option<Id<NSError, Shared>> {
        msg_send_id![self, parserError]
    }
    pub unsafe fn shouldResolveExternalEntities(&self) -> bool {
        msg_send![self, shouldResolveExternalEntities]
    }
    pub unsafe fn setShouldResolveExternalEntities(&self, shouldResolveExternalEntities: bool) {
        msg_send![
            self,
            setShouldResolveExternalEntities: shouldResolveExternalEntities
        ]
    }
}
#[doc = "NSXMLParserLocatorAdditions"]
impl NSXMLParser {
    pub unsafe fn publicID(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, publicID]
    }
    pub unsafe fn systemID(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, systemID]
    }
    pub unsafe fn lineNumber(&self) -> NSInteger {
        msg_send![self, lineNumber]
    }
    pub unsafe fn columnNumber(&self) -> NSInteger {
        msg_send![self, columnNumber]
    }
}
pub type NSXMLParserDelegate = NSObject;
