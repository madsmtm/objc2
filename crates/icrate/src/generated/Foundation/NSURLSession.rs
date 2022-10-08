use super::__exported::NSArray;
use super::__exported::NSCachedURLResponse;
use super::__exported::NSData;
use super::__exported::NSDateInterval;
use super::__exported::NSDictionary;
use super::__exported::NSError;
use super::__exported::NSHTTPCookie;
use super::__exported::NSHTTPURLResponse;
use super::__exported::NSInputStream;
use super::__exported::NSNetService;
use super::__exported::NSOperationQueue;
use super::__exported::NSOutputStream;
use super::__exported::NSString;
use super::__exported::NSURLAuthenticationChallenge;
use super::__exported::NSURLCache;
use super::__exported::NSURLCredential;
use super::__exported::NSURLCredentialStorage;
use super::__exported::NSURLProtectionSpace;
use super::__exported::NSURLResponse;
use super::__exported::NSURL;
use crate::Foundation::generated::NSHTTPCookieStorage::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSProgress::*;
use crate::Foundation::generated::NSURLRequest::*;
use crate::Security::generated::SecureTransport::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSURLSession;
    unsafe impl ClassType for NSURLSession {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSURLSession {
        #[method_id(sharedSession)]
        pub unsafe fn sharedSession() -> Id<NSURLSession, Shared>;
        # [method_id (sessionWithConfiguration :)]
        pub unsafe fn sessionWithConfiguration(
            configuration: &NSURLSessionConfiguration,
        ) -> Id<NSURLSession, Shared>;
        # [method_id (sessionWithConfiguration : delegate : delegateQueue :)]
        pub unsafe fn sessionWithConfiguration_delegate_delegateQueue(
            configuration: &NSURLSessionConfiguration,
            delegate: Option<&NSURLSessionDelegate>,
            queue: Option<&NSOperationQueue>,
        ) -> Id<NSURLSession, Shared>;
        #[method_id(delegateQueue)]
        pub unsafe fn delegateQueue(&self) -> Id<NSOperationQueue, Shared>;
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSURLSessionDelegate, Shared>>;
        #[method_id(configuration)]
        pub unsafe fn configuration(&self) -> Id<NSURLSessionConfiguration, Shared>;
        #[method_id(sessionDescription)]
        pub unsafe fn sessionDescription(&self) -> Option<Id<NSString, Shared>>;
        # [method (setSessionDescription :)]
        pub unsafe fn setSessionDescription(&self, sessionDescription: Option<&NSString>);
        #[method(finishTasksAndInvalidate)]
        pub unsafe fn finishTasksAndInvalidate(&self);
        #[method(invalidateAndCancel)]
        pub unsafe fn invalidateAndCancel(&self);
        # [method (resetWithCompletionHandler :)]
        pub unsafe fn resetWithCompletionHandler(&self, completionHandler: TodoBlock);
        # [method (flushWithCompletionHandler :)]
        pub unsafe fn flushWithCompletionHandler(&self, completionHandler: TodoBlock);
        # [method (getTasksWithCompletionHandler :)]
        pub unsafe fn getTasksWithCompletionHandler(&self, completionHandler: TodoBlock);
        # [method (getAllTasksWithCompletionHandler :)]
        pub unsafe fn getAllTasksWithCompletionHandler(&self, completionHandler: TodoBlock);
        # [method_id (dataTaskWithRequest :)]
        pub unsafe fn dataTaskWithRequest(
            &self,
            request: &NSURLRequest,
        ) -> Id<NSURLSessionDataTask, Shared>;
        # [method_id (dataTaskWithURL :)]
        pub unsafe fn dataTaskWithURL(&self, url: &NSURL) -> Id<NSURLSessionDataTask, Shared>;
        # [method_id (uploadTaskWithRequest : fromFile :)]
        pub unsafe fn uploadTaskWithRequest_fromFile(
            &self,
            request: &NSURLRequest,
            fileURL: &NSURL,
        ) -> Id<NSURLSessionUploadTask, Shared>;
        # [method_id (uploadTaskWithRequest : fromData :)]
        pub unsafe fn uploadTaskWithRequest_fromData(
            &self,
            request: &NSURLRequest,
            bodyData: &NSData,
        ) -> Id<NSURLSessionUploadTask, Shared>;
        # [method_id (uploadTaskWithStreamedRequest :)]
        pub unsafe fn uploadTaskWithStreamedRequest(
            &self,
            request: &NSURLRequest,
        ) -> Id<NSURLSessionUploadTask, Shared>;
        # [method_id (downloadTaskWithRequest :)]
        pub unsafe fn downloadTaskWithRequest(
            &self,
            request: &NSURLRequest,
        ) -> Id<NSURLSessionDownloadTask, Shared>;
        # [method_id (downloadTaskWithURL :)]
        pub unsafe fn downloadTaskWithURL(
            &self,
            url: &NSURL,
        ) -> Id<NSURLSessionDownloadTask, Shared>;
        # [method_id (downloadTaskWithResumeData :)]
        pub unsafe fn downloadTaskWithResumeData(
            &self,
            resumeData: &NSData,
        ) -> Id<NSURLSessionDownloadTask, Shared>;
        # [method_id (streamTaskWithHostName : port :)]
        pub unsafe fn streamTaskWithHostName_port(
            &self,
            hostname: &NSString,
            port: NSInteger,
        ) -> Id<NSURLSessionStreamTask, Shared>;
        # [method_id (streamTaskWithNetService :)]
        pub unsafe fn streamTaskWithNetService(
            &self,
            service: &NSNetService,
        ) -> Id<NSURLSessionStreamTask, Shared>;
        # [method_id (webSocketTaskWithURL :)]
        pub unsafe fn webSocketTaskWithURL(
            &self,
            url: &NSURL,
        ) -> Id<NSURLSessionWebSocketTask, Shared>;
        # [method_id (webSocketTaskWithURL : protocols :)]
        pub unsafe fn webSocketTaskWithURL_protocols(
            &self,
            url: &NSURL,
            protocols: &NSArray<NSString>,
        ) -> Id<NSURLSessionWebSocketTask, Shared>;
        # [method_id (webSocketTaskWithRequest :)]
        pub unsafe fn webSocketTaskWithRequest(
            &self,
            request: &NSURLRequest,
        ) -> Id<NSURLSessionWebSocketTask, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
    }
);
extern_methods!(
    #[doc = "NSURLSessionAsynchronousConvenience"]
    unsafe impl NSURLSession {
        # [method_id (dataTaskWithRequest : completionHandler :)]
        pub unsafe fn dataTaskWithRequest_completionHandler(
            &self,
            request: &NSURLRequest,
            completionHandler: TodoBlock,
        ) -> Id<NSURLSessionDataTask, Shared>;
        # [method_id (dataTaskWithURL : completionHandler :)]
        pub unsafe fn dataTaskWithURL_completionHandler(
            &self,
            url: &NSURL,
            completionHandler: TodoBlock,
        ) -> Id<NSURLSessionDataTask, Shared>;
        # [method_id (uploadTaskWithRequest : fromFile : completionHandler :)]
        pub unsafe fn uploadTaskWithRequest_fromFile_completionHandler(
            &self,
            request: &NSURLRequest,
            fileURL: &NSURL,
            completionHandler: TodoBlock,
        ) -> Id<NSURLSessionUploadTask, Shared>;
        # [method_id (uploadTaskWithRequest : fromData : completionHandler :)]
        pub unsafe fn uploadTaskWithRequest_fromData_completionHandler(
            &self,
            request: &NSURLRequest,
            bodyData: Option<&NSData>,
            completionHandler: TodoBlock,
        ) -> Id<NSURLSessionUploadTask, Shared>;
        # [method_id (downloadTaskWithRequest : completionHandler :)]
        pub unsafe fn downloadTaskWithRequest_completionHandler(
            &self,
            request: &NSURLRequest,
            completionHandler: TodoBlock,
        ) -> Id<NSURLSessionDownloadTask, Shared>;
        # [method_id (downloadTaskWithURL : completionHandler :)]
        pub unsafe fn downloadTaskWithURL_completionHandler(
            &self,
            url: &NSURL,
            completionHandler: TodoBlock,
        ) -> Id<NSURLSessionDownloadTask, Shared>;
        # [method_id (downloadTaskWithResumeData : completionHandler :)]
        pub unsafe fn downloadTaskWithResumeData_completionHandler(
            &self,
            resumeData: &NSData,
            completionHandler: TodoBlock,
        ) -> Id<NSURLSessionDownloadTask, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSURLSessionTask;
    unsafe impl ClassType for NSURLSessionTask {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSURLSessionTask {
        #[method(taskIdentifier)]
        pub unsafe fn taskIdentifier(&self) -> NSUInteger;
        #[method_id(originalRequest)]
        pub unsafe fn originalRequest(&self) -> Option<Id<NSURLRequest, Shared>>;
        #[method_id(currentRequest)]
        pub unsafe fn currentRequest(&self) -> Option<Id<NSURLRequest, Shared>>;
        #[method_id(response)]
        pub unsafe fn response(&self) -> Option<Id<NSURLResponse, Shared>>;
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSURLSessionTaskDelegate, Shared>>;
        # [method (setDelegate :)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSURLSessionTaskDelegate>);
        #[method_id(progress)]
        pub unsafe fn progress(&self) -> Id<NSProgress, Shared>;
        #[method_id(earliestBeginDate)]
        pub unsafe fn earliestBeginDate(&self) -> Option<Id<NSDate, Shared>>;
        # [method (setEarliestBeginDate :)]
        pub unsafe fn setEarliestBeginDate(&self, earliestBeginDate: Option<&NSDate>);
        #[method(countOfBytesClientExpectsToSend)]
        pub unsafe fn countOfBytesClientExpectsToSend(&self) -> int64_t;
        # [method (setCountOfBytesClientExpectsToSend :)]
        pub unsafe fn setCountOfBytesClientExpectsToSend(
            &self,
            countOfBytesClientExpectsToSend: int64_t,
        );
        #[method(countOfBytesClientExpectsToReceive)]
        pub unsafe fn countOfBytesClientExpectsToReceive(&self) -> int64_t;
        # [method (setCountOfBytesClientExpectsToReceive :)]
        pub unsafe fn setCountOfBytesClientExpectsToReceive(
            &self,
            countOfBytesClientExpectsToReceive: int64_t,
        );
        #[method(countOfBytesSent)]
        pub unsafe fn countOfBytesSent(&self) -> int64_t;
        #[method(countOfBytesReceived)]
        pub unsafe fn countOfBytesReceived(&self) -> int64_t;
        #[method(countOfBytesExpectedToSend)]
        pub unsafe fn countOfBytesExpectedToSend(&self) -> int64_t;
        #[method(countOfBytesExpectedToReceive)]
        pub unsafe fn countOfBytesExpectedToReceive(&self) -> int64_t;
        #[method_id(taskDescription)]
        pub unsafe fn taskDescription(&self) -> Option<Id<NSString, Shared>>;
        # [method (setTaskDescription :)]
        pub unsafe fn setTaskDescription(&self, taskDescription: Option<&NSString>);
        #[method(cancel)]
        pub unsafe fn cancel(&self);
        #[method(state)]
        pub unsafe fn state(&self) -> NSURLSessionTaskState;
        #[method_id(error)]
        pub unsafe fn error(&self) -> Option<Id<NSError, Shared>>;
        #[method(suspend)]
        pub unsafe fn suspend(&self);
        #[method(resume)]
        pub unsafe fn resume(&self);
        #[method(priority)]
        pub unsafe fn priority(&self) -> c_float;
        # [method (setPriority :)]
        pub unsafe fn setPriority(&self, priority: c_float);
        #[method(prefersIncrementalDelivery)]
        pub unsafe fn prefersIncrementalDelivery(&self) -> bool;
        # [method (setPrefersIncrementalDelivery :)]
        pub unsafe fn setPrefersIncrementalDelivery(&self, prefersIncrementalDelivery: bool);
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSURLSessionDataTask;
    unsafe impl ClassType for NSURLSessionDataTask {
        type Super = NSURLSessionTask;
    }
);
extern_methods!(
    unsafe impl NSURLSessionDataTask {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSURLSessionUploadTask;
    unsafe impl ClassType for NSURLSessionUploadTask {
        type Super = NSURLSessionDataTask;
    }
);
extern_methods!(
    unsafe impl NSURLSessionUploadTask {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSURLSessionDownloadTask;
    unsafe impl ClassType for NSURLSessionDownloadTask {
        type Super = NSURLSessionTask;
    }
);
extern_methods!(
    unsafe impl NSURLSessionDownloadTask {
        # [method (cancelByProducingResumeData :)]
        pub unsafe fn cancelByProducingResumeData(&self, completionHandler: TodoBlock);
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSURLSessionStreamTask;
    unsafe impl ClassType for NSURLSessionStreamTask {
        type Super = NSURLSessionTask;
    }
);
extern_methods!(
    unsafe impl NSURLSessionStreamTask {
        # [method (readDataOfMinLength : maxLength : timeout : completionHandler :)]
        pub unsafe fn readDataOfMinLength_maxLength_timeout_completionHandler(
            &self,
            minBytes: NSUInteger,
            maxBytes: NSUInteger,
            timeout: NSTimeInterval,
            completionHandler: TodoBlock,
        );
        # [method (writeData : timeout : completionHandler :)]
        pub unsafe fn writeData_timeout_completionHandler(
            &self,
            data: &NSData,
            timeout: NSTimeInterval,
            completionHandler: TodoBlock,
        );
        #[method(captureStreams)]
        pub unsafe fn captureStreams(&self);
        #[method(closeWrite)]
        pub unsafe fn closeWrite(&self);
        #[method(closeRead)]
        pub unsafe fn closeRead(&self);
        #[method(startSecureConnection)]
        pub unsafe fn startSecureConnection(&self);
        #[method(stopSecureConnection)]
        pub unsafe fn stopSecureConnection(&self);
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSURLSessionWebSocketMessage;
    unsafe impl ClassType for NSURLSessionWebSocketMessage {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSURLSessionWebSocketMessage {
        # [method_id (initWithData :)]
        pub unsafe fn initWithData(&self, data: &NSData) -> Id<Self, Shared>;
        # [method_id (initWithString :)]
        pub unsafe fn initWithString(&self, string: &NSString) -> Id<Self, Shared>;
        #[method(type)]
        pub unsafe fn type_(&self) -> NSURLSessionWebSocketMessageType;
        #[method_id(data)]
        pub unsafe fn data(&self) -> Option<Id<NSData, Shared>>;
        #[method_id(string)]
        pub unsafe fn string(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSURLSessionWebSocketTask;
    unsafe impl ClassType for NSURLSessionWebSocketTask {
        type Super = NSURLSessionTask;
    }
);
extern_methods!(
    unsafe impl NSURLSessionWebSocketTask {
        # [method (sendMessage : completionHandler :)]
        pub unsafe fn sendMessage_completionHandler(
            &self,
            message: &NSURLSessionWebSocketMessage,
            completionHandler: TodoBlock,
        );
        # [method (receiveMessageWithCompletionHandler :)]
        pub unsafe fn receiveMessageWithCompletionHandler(&self, completionHandler: TodoBlock);
        # [method (sendPingWithPongReceiveHandler :)]
        pub unsafe fn sendPingWithPongReceiveHandler(&self, pongReceiveHandler: TodoBlock);
        # [method (cancelWithCloseCode : reason :)]
        pub unsafe fn cancelWithCloseCode_reason(
            &self,
            closeCode: NSURLSessionWebSocketCloseCode,
            reason: Option<&NSData>,
        );
        #[method(maximumMessageSize)]
        pub unsafe fn maximumMessageSize(&self) -> NSInteger;
        # [method (setMaximumMessageSize :)]
        pub unsafe fn setMaximumMessageSize(&self, maximumMessageSize: NSInteger);
        #[method(closeCode)]
        pub unsafe fn closeCode(&self) -> NSURLSessionWebSocketCloseCode;
        #[method_id(closeReason)]
        pub unsafe fn closeReason(&self) -> Option<Id<NSData, Shared>>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSURLSessionConfiguration;
    unsafe impl ClassType for NSURLSessionConfiguration {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSURLSessionConfiguration {
        #[method_id(defaultSessionConfiguration)]
        pub unsafe fn defaultSessionConfiguration() -> Id<NSURLSessionConfiguration, Shared>;
        #[method_id(ephemeralSessionConfiguration)]
        pub unsafe fn ephemeralSessionConfiguration() -> Id<NSURLSessionConfiguration, Shared>;
        # [method_id (backgroundSessionConfigurationWithIdentifier :)]
        pub unsafe fn backgroundSessionConfigurationWithIdentifier(
            identifier: &NSString,
        ) -> Id<NSURLSessionConfiguration, Shared>;
        #[method_id(identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString, Shared>>;
        #[method(requestCachePolicy)]
        pub unsafe fn requestCachePolicy(&self) -> NSURLRequestCachePolicy;
        # [method (setRequestCachePolicy :)]
        pub unsafe fn setRequestCachePolicy(&self, requestCachePolicy: NSURLRequestCachePolicy);
        #[method(timeoutIntervalForRequest)]
        pub unsafe fn timeoutIntervalForRequest(&self) -> NSTimeInterval;
        # [method (setTimeoutIntervalForRequest :)]
        pub unsafe fn setTimeoutIntervalForRequest(
            &self,
            timeoutIntervalForRequest: NSTimeInterval,
        );
        #[method(timeoutIntervalForResource)]
        pub unsafe fn timeoutIntervalForResource(&self) -> NSTimeInterval;
        # [method (setTimeoutIntervalForResource :)]
        pub unsafe fn setTimeoutIntervalForResource(
            &self,
            timeoutIntervalForResource: NSTimeInterval,
        );
        #[method(networkServiceType)]
        pub unsafe fn networkServiceType(&self) -> NSURLRequestNetworkServiceType;
        # [method (setNetworkServiceType :)]
        pub unsafe fn setNetworkServiceType(
            &self,
            networkServiceType: NSURLRequestNetworkServiceType,
        );
        #[method(allowsCellularAccess)]
        pub unsafe fn allowsCellularAccess(&self) -> bool;
        # [method (setAllowsCellularAccess :)]
        pub unsafe fn setAllowsCellularAccess(&self, allowsCellularAccess: bool);
        #[method(allowsExpensiveNetworkAccess)]
        pub unsafe fn allowsExpensiveNetworkAccess(&self) -> bool;
        # [method (setAllowsExpensiveNetworkAccess :)]
        pub unsafe fn setAllowsExpensiveNetworkAccess(&self, allowsExpensiveNetworkAccess: bool);
        #[method(allowsConstrainedNetworkAccess)]
        pub unsafe fn allowsConstrainedNetworkAccess(&self) -> bool;
        # [method (setAllowsConstrainedNetworkAccess :)]
        pub unsafe fn setAllowsConstrainedNetworkAccess(
            &self,
            allowsConstrainedNetworkAccess: bool,
        );
        #[method(waitsForConnectivity)]
        pub unsafe fn waitsForConnectivity(&self) -> bool;
        # [method (setWaitsForConnectivity :)]
        pub unsafe fn setWaitsForConnectivity(&self, waitsForConnectivity: bool);
        #[method(isDiscretionary)]
        pub unsafe fn isDiscretionary(&self) -> bool;
        # [method (setDiscretionary :)]
        pub unsafe fn setDiscretionary(&self, discretionary: bool);
        #[method_id(sharedContainerIdentifier)]
        pub unsafe fn sharedContainerIdentifier(&self) -> Option<Id<NSString, Shared>>;
        # [method (setSharedContainerIdentifier :)]
        pub unsafe fn setSharedContainerIdentifier(
            &self,
            sharedContainerIdentifier: Option<&NSString>,
        );
        #[method(sessionSendsLaunchEvents)]
        pub unsafe fn sessionSendsLaunchEvents(&self) -> bool;
        # [method (setSessionSendsLaunchEvents :)]
        pub unsafe fn setSessionSendsLaunchEvents(&self, sessionSendsLaunchEvents: bool);
        #[method_id(connectionProxyDictionary)]
        pub unsafe fn connectionProxyDictionary(&self) -> Option<Id<NSDictionary, Shared>>;
        # [method (setConnectionProxyDictionary :)]
        pub unsafe fn setConnectionProxyDictionary(
            &self,
            connectionProxyDictionary: Option<&NSDictionary>,
        );
        #[method(TLSMinimumSupportedProtocol)]
        pub unsafe fn TLSMinimumSupportedProtocol(&self) -> SSLProtocol;
        # [method (setTLSMinimumSupportedProtocol :)]
        pub unsafe fn setTLSMinimumSupportedProtocol(
            &self,
            TLSMinimumSupportedProtocol: SSLProtocol,
        );
        #[method(TLSMaximumSupportedProtocol)]
        pub unsafe fn TLSMaximumSupportedProtocol(&self) -> SSLProtocol;
        # [method (setTLSMaximumSupportedProtocol :)]
        pub unsafe fn setTLSMaximumSupportedProtocol(
            &self,
            TLSMaximumSupportedProtocol: SSLProtocol,
        );
        #[method(TLSMinimumSupportedProtocolVersion)]
        pub unsafe fn TLSMinimumSupportedProtocolVersion(&self) -> tls_protocol_version_t;
        # [method (setTLSMinimumSupportedProtocolVersion :)]
        pub unsafe fn setTLSMinimumSupportedProtocolVersion(
            &self,
            TLSMinimumSupportedProtocolVersion: tls_protocol_version_t,
        );
        #[method(TLSMaximumSupportedProtocolVersion)]
        pub unsafe fn TLSMaximumSupportedProtocolVersion(&self) -> tls_protocol_version_t;
        # [method (setTLSMaximumSupportedProtocolVersion :)]
        pub unsafe fn setTLSMaximumSupportedProtocolVersion(
            &self,
            TLSMaximumSupportedProtocolVersion: tls_protocol_version_t,
        );
        #[method(HTTPShouldUsePipelining)]
        pub unsafe fn HTTPShouldUsePipelining(&self) -> bool;
        # [method (setHTTPShouldUsePipelining :)]
        pub unsafe fn setHTTPShouldUsePipelining(&self, HTTPShouldUsePipelining: bool);
        #[method(HTTPShouldSetCookies)]
        pub unsafe fn HTTPShouldSetCookies(&self) -> bool;
        # [method (setHTTPShouldSetCookies :)]
        pub unsafe fn setHTTPShouldSetCookies(&self, HTTPShouldSetCookies: bool);
        #[method(HTTPCookieAcceptPolicy)]
        pub unsafe fn HTTPCookieAcceptPolicy(&self) -> NSHTTPCookieAcceptPolicy;
        # [method (setHTTPCookieAcceptPolicy :)]
        pub unsafe fn setHTTPCookieAcceptPolicy(
            &self,
            HTTPCookieAcceptPolicy: NSHTTPCookieAcceptPolicy,
        );
        #[method_id(HTTPAdditionalHeaders)]
        pub unsafe fn HTTPAdditionalHeaders(&self) -> Option<Id<NSDictionary, Shared>>;
        # [method (setHTTPAdditionalHeaders :)]
        pub unsafe fn setHTTPAdditionalHeaders(&self, HTTPAdditionalHeaders: Option<&NSDictionary>);
        #[method(HTTPMaximumConnectionsPerHost)]
        pub unsafe fn HTTPMaximumConnectionsPerHost(&self) -> NSInteger;
        # [method (setHTTPMaximumConnectionsPerHost :)]
        pub unsafe fn setHTTPMaximumConnectionsPerHost(
            &self,
            HTTPMaximumConnectionsPerHost: NSInteger,
        );
        #[method_id(HTTPCookieStorage)]
        pub unsafe fn HTTPCookieStorage(&self) -> Option<Id<NSHTTPCookieStorage, Shared>>;
        # [method (setHTTPCookieStorage :)]
        pub unsafe fn setHTTPCookieStorage(&self, HTTPCookieStorage: Option<&NSHTTPCookieStorage>);
        #[method_id(URLCredentialStorage)]
        pub unsafe fn URLCredentialStorage(&self) -> Option<Id<NSURLCredentialStorage, Shared>>;
        # [method (setURLCredentialStorage :)]
        pub unsafe fn setURLCredentialStorage(
            &self,
            URLCredentialStorage: Option<&NSURLCredentialStorage>,
        );
        #[method_id(URLCache)]
        pub unsafe fn URLCache(&self) -> Option<Id<NSURLCache, Shared>>;
        # [method (setURLCache :)]
        pub unsafe fn setURLCache(&self, URLCache: Option<&NSURLCache>);
        #[method(shouldUseExtendedBackgroundIdleMode)]
        pub unsafe fn shouldUseExtendedBackgroundIdleMode(&self) -> bool;
        # [method (setShouldUseExtendedBackgroundIdleMode :)]
        pub unsafe fn setShouldUseExtendedBackgroundIdleMode(
            &self,
            shouldUseExtendedBackgroundIdleMode: bool,
        );
        #[method_id(protocolClasses)]
        pub unsafe fn protocolClasses(&self) -> Option<Id<NSArray<TodoClass>, Shared>>;
        # [method (setProtocolClasses :)]
        pub unsafe fn setProtocolClasses(&self, protocolClasses: Option<&NSArray<TodoClass>>);
        #[method(multipathServiceType)]
        pub unsafe fn multipathServiceType(&self) -> NSURLSessionMultipathServiceType;
        # [method (setMultipathServiceType :)]
        pub unsafe fn setMultipathServiceType(
            &self,
            multipathServiceType: NSURLSessionMultipathServiceType,
        );
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
    }
);
pub type NSURLSessionDelegate = NSObject;
pub type NSURLSessionTaskDelegate = NSObject;
pub type NSURLSessionDataDelegate = NSObject;
pub type NSURLSessionDownloadDelegate = NSObject;
pub type NSURLSessionStreamDelegate = NSObject;
pub type NSURLSessionWebSocketDelegate = NSObject;
extern_methods!(
    #[doc = "NSURLSessionDeprecated"]
    unsafe impl NSURLSessionConfiguration {
        # [method_id (backgroundSessionConfiguration :)]
        pub unsafe fn backgroundSessionConfiguration(
            identifier: &NSString,
        ) -> Id<NSURLSessionConfiguration, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSURLSessionTaskTransactionMetrics;
    unsafe impl ClassType for NSURLSessionTaskTransactionMetrics {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSURLSessionTaskTransactionMetrics {
        #[method_id(request)]
        pub unsafe fn request(&self) -> Id<NSURLRequest, Shared>;
        #[method_id(response)]
        pub unsafe fn response(&self) -> Option<Id<NSURLResponse, Shared>>;
        #[method_id(fetchStartDate)]
        pub unsafe fn fetchStartDate(&self) -> Option<Id<NSDate, Shared>>;
        #[method_id(domainLookupStartDate)]
        pub unsafe fn domainLookupStartDate(&self) -> Option<Id<NSDate, Shared>>;
        #[method_id(domainLookupEndDate)]
        pub unsafe fn domainLookupEndDate(&self) -> Option<Id<NSDate, Shared>>;
        #[method_id(connectStartDate)]
        pub unsafe fn connectStartDate(&self) -> Option<Id<NSDate, Shared>>;
        #[method_id(secureConnectionStartDate)]
        pub unsafe fn secureConnectionStartDate(&self) -> Option<Id<NSDate, Shared>>;
        #[method_id(secureConnectionEndDate)]
        pub unsafe fn secureConnectionEndDate(&self) -> Option<Id<NSDate, Shared>>;
        #[method_id(connectEndDate)]
        pub unsafe fn connectEndDate(&self) -> Option<Id<NSDate, Shared>>;
        #[method_id(requestStartDate)]
        pub unsafe fn requestStartDate(&self) -> Option<Id<NSDate, Shared>>;
        #[method_id(requestEndDate)]
        pub unsafe fn requestEndDate(&self) -> Option<Id<NSDate, Shared>>;
        #[method_id(responseStartDate)]
        pub unsafe fn responseStartDate(&self) -> Option<Id<NSDate, Shared>>;
        #[method_id(responseEndDate)]
        pub unsafe fn responseEndDate(&self) -> Option<Id<NSDate, Shared>>;
        #[method_id(networkProtocolName)]
        pub unsafe fn networkProtocolName(&self) -> Option<Id<NSString, Shared>>;
        #[method(isProxyConnection)]
        pub unsafe fn isProxyConnection(&self) -> bool;
        #[method(isReusedConnection)]
        pub unsafe fn isReusedConnection(&self) -> bool;
        #[method(resourceFetchType)]
        pub unsafe fn resourceFetchType(&self) -> NSURLSessionTaskMetricsResourceFetchType;
        #[method(countOfRequestHeaderBytesSent)]
        pub unsafe fn countOfRequestHeaderBytesSent(&self) -> int64_t;
        #[method(countOfRequestBodyBytesSent)]
        pub unsafe fn countOfRequestBodyBytesSent(&self) -> int64_t;
        #[method(countOfRequestBodyBytesBeforeEncoding)]
        pub unsafe fn countOfRequestBodyBytesBeforeEncoding(&self) -> int64_t;
        #[method(countOfResponseHeaderBytesReceived)]
        pub unsafe fn countOfResponseHeaderBytesReceived(&self) -> int64_t;
        #[method(countOfResponseBodyBytesReceived)]
        pub unsafe fn countOfResponseBodyBytesReceived(&self) -> int64_t;
        #[method(countOfResponseBodyBytesAfterDecoding)]
        pub unsafe fn countOfResponseBodyBytesAfterDecoding(&self) -> int64_t;
        #[method_id(localAddress)]
        pub unsafe fn localAddress(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(localPort)]
        pub unsafe fn localPort(&self) -> Option<Id<NSNumber, Shared>>;
        #[method_id(remoteAddress)]
        pub unsafe fn remoteAddress(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(remotePort)]
        pub unsafe fn remotePort(&self) -> Option<Id<NSNumber, Shared>>;
        #[method_id(negotiatedTLSProtocolVersion)]
        pub unsafe fn negotiatedTLSProtocolVersion(&self) -> Option<Id<NSNumber, Shared>>;
        #[method_id(negotiatedTLSCipherSuite)]
        pub unsafe fn negotiatedTLSCipherSuite(&self) -> Option<Id<NSNumber, Shared>>;
        #[method(isCellular)]
        pub unsafe fn isCellular(&self) -> bool;
        #[method(isExpensive)]
        pub unsafe fn isExpensive(&self) -> bool;
        #[method(isConstrained)]
        pub unsafe fn isConstrained(&self) -> bool;
        #[method(isMultipath)]
        pub unsafe fn isMultipath(&self) -> bool;
        #[method(domainResolutionProtocol)]
        pub unsafe fn domainResolutionProtocol(
            &self,
        ) -> NSURLSessionTaskMetricsDomainResolutionProtocol;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSURLSessionTaskMetrics;
    unsafe impl ClassType for NSURLSessionTaskMetrics {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSURLSessionTaskMetrics {
        #[method_id(transactionMetrics)]
        pub unsafe fn transactionMetrics(
            &self,
        ) -> Id<NSArray<NSURLSessionTaskTransactionMetrics>, Shared>;
        #[method_id(taskInterval)]
        pub unsafe fn taskInterval(&self) -> Id<NSDateInterval, Shared>;
        #[method(redirectCount)]
        pub unsafe fn redirectCount(&self) -> NSUInteger;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
    }
);
