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
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSXMLParser;
    unsafe impl ClassType for NSXMLParser {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSXMLParser {
        #[method_id(initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(&self, url: &NSURL) -> Option<Id<Self, Shared>>;
        #[method_id(initWithData:)]
        pub unsafe fn initWithData(&self, data: &NSData) -> Id<Self, Shared>;
        #[method_id(initWithStream:)]
        pub unsafe fn initWithStream(&self, stream: &NSInputStream) -> Id<Self, Shared>;
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSXMLParserDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSXMLParserDelegate>);
        #[method(shouldProcessNamespaces)]
        pub unsafe fn shouldProcessNamespaces(&self) -> bool;
        #[method(setShouldProcessNamespaces:)]
        pub unsafe fn setShouldProcessNamespaces(&self, shouldProcessNamespaces: bool);
        #[method(shouldReportNamespacePrefixes)]
        pub unsafe fn shouldReportNamespacePrefixes(&self) -> bool;
        #[method(setShouldReportNamespacePrefixes:)]
        pub unsafe fn setShouldReportNamespacePrefixes(&self, shouldReportNamespacePrefixes: bool);
        #[method(externalEntityResolvingPolicy)]
        pub unsafe fn externalEntityResolvingPolicy(
            &self,
        ) -> NSXMLParserExternalEntityResolvingPolicy;
        #[method(setExternalEntityResolvingPolicy:)]
        pub unsafe fn setExternalEntityResolvingPolicy(
            &self,
            externalEntityResolvingPolicy: NSXMLParserExternalEntityResolvingPolicy,
        );
        #[method_id(allowedExternalEntityURLs)]
        pub unsafe fn allowedExternalEntityURLs(&self) -> Option<Id<NSSet<NSURL>, Shared>>;
        #[method(setAllowedExternalEntityURLs:)]
        pub unsafe fn setAllowedExternalEntityURLs(
            &self,
            allowedExternalEntityURLs: Option<&NSSet<NSURL>>,
        );
        #[method(parse)]
        pub unsafe fn parse(&self) -> bool;
        #[method(abortParsing)]
        pub unsafe fn abortParsing(&self);
        #[method_id(parserError)]
        pub unsafe fn parserError(&self) -> Option<Id<NSError, Shared>>;
        #[method(shouldResolveExternalEntities)]
        pub unsafe fn shouldResolveExternalEntities(&self) -> bool;
        #[method(setShouldResolveExternalEntities:)]
        pub unsafe fn setShouldResolveExternalEntities(&self, shouldResolveExternalEntities: bool);
    }
);
extern_methods!(
    #[doc = "NSXMLParserLocatorAdditions"]
    unsafe impl NSXMLParser {
        #[method_id(publicID)]
        pub unsafe fn publicID(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(systemID)]
        pub unsafe fn systemID(&self) -> Option<Id<NSString, Shared>>;
        #[method(lineNumber)]
        pub unsafe fn lineNumber(&self) -> NSInteger;
        #[method(columnNumber)]
        pub unsafe fn columnNumber(&self) -> NSInteger;
    }
);
pub type NSXMLParserDelegate = NSObject;
