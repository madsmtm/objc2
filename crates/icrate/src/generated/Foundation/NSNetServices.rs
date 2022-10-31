//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};

pub type NSNetServicesError = NSInteger;
pub const NSNetServicesUnknownError: NSNetServicesError = -72000;
pub const NSNetServicesCollisionError: NSNetServicesError = -72001;
pub const NSNetServicesNotFoundError: NSNetServicesError = -72002;
pub const NSNetServicesActivityInProgress: NSNetServicesError = -72003;
pub const NSNetServicesBadArgumentError: NSNetServicesError = -72004;
pub const NSNetServicesCancelledError: NSNetServicesError = -72005;
pub const NSNetServicesInvalidError: NSNetServicesError = -72006;
pub const NSNetServicesTimeoutError: NSNetServicesError = -72007;
pub const NSNetServicesMissingRequiredConfigurationError: NSNetServicesError = -72008;

pub type NSNetServiceOptions = NSUInteger;
pub const NSNetServiceNoAutoRename: NSNetServiceOptions = 1;
pub const NSNetServiceListenForConnections: NSNetServiceOptions = 2;

extern_class!(
    #[derive(Debug)]
    pub struct NSNetService;

    unsafe impl ClassType for NSNetService {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSNetService {
        #[method_id(initWithDomain:type:name:port:)]
        pub unsafe fn initWithDomain_type_name_port(
            &self,
            domain: &NSString,
            type_: &NSString,
            name: &NSString,
            port: c_int,
        ) -> Id<Self, Shared>;

        #[method_id(initWithDomain:type:name:)]
        pub unsafe fn initWithDomain_type_name(
            &self,
            domain: &NSString,
            type_: &NSString,
            name: &NSString,
        ) -> Id<Self, Shared>;

        #[method(scheduleInRunLoop:forMode:)]
        pub unsafe fn scheduleInRunLoop_forMode(&self, aRunLoop: &NSRunLoop, mode: &NSRunLoopMode);

        #[method(removeFromRunLoop:forMode:)]
        pub unsafe fn removeFromRunLoop_forMode(&self, aRunLoop: &NSRunLoop, mode: &NSRunLoopMode);

        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSNetServiceDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSNetServiceDelegate>);

        #[method(includesPeerToPeer)]
        pub unsafe fn includesPeerToPeer(&self) -> bool;

        #[method(setIncludesPeerToPeer:)]
        pub unsafe fn setIncludesPeerToPeer(&self, includesPeerToPeer: bool);

        #[method_id(name)]
        pub unsafe fn name(&self) -> Id<NSString, Shared>;

        #[method_id(type)]
        pub unsafe fn type_(&self) -> Id<NSString, Shared>;

        #[method_id(domain)]
        pub unsafe fn domain(&self) -> Id<NSString, Shared>;

        #[method_id(hostName)]
        pub unsafe fn hostName(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(addresses)]
        pub unsafe fn addresses(&self) -> Option<Id<NSArray<NSData>, Shared>>;

        #[method(port)]
        pub unsafe fn port(&self) -> NSInteger;

        #[method(publish)]
        pub unsafe fn publish(&self);

        #[method(publishWithOptions:)]
        pub unsafe fn publishWithOptions(&self, options: NSNetServiceOptions);

        #[method(resolve)]
        pub unsafe fn resolve(&self);

        #[method(stop)]
        pub unsafe fn stop(&self);

        #[method_id(dictionaryFromTXTRecordData:)]
        pub unsafe fn dictionaryFromTXTRecordData(
            txtData: &NSData,
        ) -> Id<NSDictionary<NSString, NSData>, Shared>;

        #[method_id(dataFromTXTRecordDictionary:)]
        pub unsafe fn dataFromTXTRecordDictionary(
            txtDictionary: &NSDictionary<NSString, NSData>,
        ) -> Id<NSData, Shared>;

        #[method(resolveWithTimeout:)]
        pub unsafe fn resolveWithTimeout(&self, timeout: NSTimeInterval);

        #[method(setTXTRecordData:)]
        pub unsafe fn setTXTRecordData(&self, recordData: Option<&NSData>) -> bool;

        #[method_id(TXTRecordData)]
        pub unsafe fn TXTRecordData(&self) -> Option<Id<NSData, Shared>>;

        #[method(startMonitoring)]
        pub unsafe fn startMonitoring(&self);

        #[method(stopMonitoring)]
        pub unsafe fn stopMonitoring(&self);
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSNetServiceBrowser;

    unsafe impl ClassType for NSNetServiceBrowser {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSNetServiceBrowser {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;

        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSNetServiceBrowserDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSNetServiceBrowserDelegate>);

        #[method(includesPeerToPeer)]
        pub unsafe fn includesPeerToPeer(&self) -> bool;

        #[method(setIncludesPeerToPeer:)]
        pub unsafe fn setIncludesPeerToPeer(&self, includesPeerToPeer: bool);

        #[method(scheduleInRunLoop:forMode:)]
        pub unsafe fn scheduleInRunLoop_forMode(&self, aRunLoop: &NSRunLoop, mode: &NSRunLoopMode);

        #[method(removeFromRunLoop:forMode:)]
        pub unsafe fn removeFromRunLoop_forMode(&self, aRunLoop: &NSRunLoop, mode: &NSRunLoopMode);

        #[method(searchForBrowsableDomains)]
        pub unsafe fn searchForBrowsableDomains(&self);

        #[method(searchForRegistrationDomains)]
        pub unsafe fn searchForRegistrationDomains(&self);

        #[method(searchForServicesOfType:inDomain:)]
        pub unsafe fn searchForServicesOfType_inDomain(
            &self,
            type_: &NSString,
            domainString: &NSString,
        );

        #[method(stop)]
        pub unsafe fn stop(&self);
    }
);

pub type NSNetServiceDelegate = NSObject;

pub type NSNetServiceBrowserDelegate = NSObject;
