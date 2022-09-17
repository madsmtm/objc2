use super::NSArray;
use super::NSCachedURLResponse;
use super::NSData;
use super::NSDateInterval;
use super::NSDictionary;
use super::NSError;
use super::NSHTTPCookie;
use super::NSHTTPURLResponse;
use super::NSInputStream;
use super::NSNetService;
use super::NSOperationQueue;
use super::NSOutputStream;
use super::NSString;
use super::NSURLAuthenticationChallenge;
use super::NSURLCache;
use super::NSURLCredential;
use super::NSURLCredentialStorage;
use super::NSURLProtectionSpace;
use super::NSURLResponse;
use super::NSURL;
use crate::Foundation::generated::NSHTTPCookieStorage::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSProgress::*;
use crate::Foundation::generated::NSURLRequest::*;
use crate::Security::generated::SecureTransport::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSURLSession;
    unsafe impl ClassType for NSURLSession {
        type Super = NSObject;
    }
);
impl NSURLSession {
    pub unsafe fn sessionWithConfiguration(
        configuration: &NSURLSessionConfiguration,
    ) -> Id<NSURLSession, Shared> {
        msg_send_id![Self::class(), sessionWithConfiguration: configuration]
    }
    pub unsafe fn sessionWithConfiguration_delegate_delegateQueue(
        configuration: &NSURLSessionConfiguration,
        delegate: TodoGenerics,
        queue: Option<&NSOperationQueue>,
    ) -> Id<NSURLSession, Shared> {
        msg_send_id![
            Self::class(),
            sessionWithConfiguration: configuration,
            delegate: delegate,
            delegateQueue: queue
        ]
    }
    pub unsafe fn finishTasksAndInvalidate(&self) {
        msg_send![self, finishTasksAndInvalidate]
    }
    pub unsafe fn invalidateAndCancel(&self) {
        msg_send![self, invalidateAndCancel]
    }
    pub unsafe fn resetWithCompletionHandler(&self, completionHandler: TodoBlock) {
        msg_send![self, resetWithCompletionHandler: completionHandler]
    }
    pub unsafe fn flushWithCompletionHandler(&self, completionHandler: TodoBlock) {
        msg_send![self, flushWithCompletionHandler: completionHandler]
    }
    pub unsafe fn getTasksWithCompletionHandler(&self, completionHandler: TodoBlock) {
        msg_send![self, getTasksWithCompletionHandler: completionHandler]
    }
    pub unsafe fn getAllTasksWithCompletionHandler(&self, completionHandler: TodoBlock) {
        msg_send![self, getAllTasksWithCompletionHandler: completionHandler]
    }
    pub unsafe fn dataTaskWithRequest(
        &self,
        request: &NSURLRequest,
    ) -> Id<NSURLSessionDataTask, Shared> {
        msg_send_id![self, dataTaskWithRequest: request]
    }
    pub unsafe fn dataTaskWithURL(&self, url: &NSURL) -> Id<NSURLSessionDataTask, Shared> {
        msg_send_id![self, dataTaskWithURL: url]
    }
    pub unsafe fn uploadTaskWithRequest_fromFile(
        &self,
        request: &NSURLRequest,
        fileURL: &NSURL,
    ) -> Id<NSURLSessionUploadTask, Shared> {
        msg_send_id![self, uploadTaskWithRequest: request, fromFile: fileURL]
    }
    pub unsafe fn uploadTaskWithRequest_fromData(
        &self,
        request: &NSURLRequest,
        bodyData: &NSData,
    ) -> Id<NSURLSessionUploadTask, Shared> {
        msg_send_id![self, uploadTaskWithRequest: request, fromData: bodyData]
    }
    pub unsafe fn uploadTaskWithStreamedRequest(
        &self,
        request: &NSURLRequest,
    ) -> Id<NSURLSessionUploadTask, Shared> {
        msg_send_id![self, uploadTaskWithStreamedRequest: request]
    }
    pub unsafe fn downloadTaskWithRequest(
        &self,
        request: &NSURLRequest,
    ) -> Id<NSURLSessionDownloadTask, Shared> {
        msg_send_id![self, downloadTaskWithRequest: request]
    }
    pub unsafe fn downloadTaskWithURL(&self, url: &NSURL) -> Id<NSURLSessionDownloadTask, Shared> {
        msg_send_id![self, downloadTaskWithURL: url]
    }
    pub unsafe fn downloadTaskWithResumeData(
        &self,
        resumeData: &NSData,
    ) -> Id<NSURLSessionDownloadTask, Shared> {
        msg_send_id![self, downloadTaskWithResumeData: resumeData]
    }
    pub unsafe fn streamTaskWithHostName_port(
        &self,
        hostname: &NSString,
        port: NSInteger,
    ) -> Id<NSURLSessionStreamTask, Shared> {
        msg_send_id![self, streamTaskWithHostName: hostname, port: port]
    }
    pub unsafe fn streamTaskWithNetService(
        &self,
        service: &NSNetService,
    ) -> Id<NSURLSessionStreamTask, Shared> {
        msg_send_id![self, streamTaskWithNetService: service]
    }
    pub unsafe fn webSocketTaskWithURL(
        &self,
        url: &NSURL,
    ) -> Id<NSURLSessionWebSocketTask, Shared> {
        msg_send_id![self, webSocketTaskWithURL: url]
    }
    pub unsafe fn webSocketTaskWithURL_protocols(
        &self,
        url: &NSURL,
        protocols: TodoGenerics,
    ) -> Id<NSURLSessionWebSocketTask, Shared> {
        msg_send_id![self, webSocketTaskWithURL: url, protocols: protocols]
    }
    pub unsafe fn webSocketTaskWithRequest(
        &self,
        request: &NSURLRequest,
    ) -> Id<NSURLSessionWebSocketTask, Shared> {
        msg_send_id![self, webSocketTaskWithRequest: request]
    }
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn new() -> Id<Self, Shared> {
        msg_send_id![Self::class(), new]
    }
    pub unsafe fn sharedSession() -> Id<NSURLSession, Shared> {
        msg_send_id![Self::class(), sharedSession]
    }
    pub unsafe fn delegateQueue(&self) -> Id<NSOperationQueue, Shared> {
        msg_send_id![self, delegateQueue]
    }
    pub unsafe fn delegate(&self) -> TodoGenerics {
        msg_send![self, delegate]
    }
    pub unsafe fn configuration(&self) -> Id<NSURLSessionConfiguration, Shared> {
        msg_send_id![self, configuration]
    }
    pub unsafe fn sessionDescription(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, sessionDescription]
    }
    pub unsafe fn setSessionDescription(&self, sessionDescription: Option<&NSString>) {
        msg_send![self, setSessionDescription: sessionDescription]
    }
}
#[doc = "NSURLSessionAsynchronousConvenience"]
impl NSURLSession {
    pub unsafe fn dataTaskWithRequest_completionHandler(
        &self,
        request: &NSURLRequest,
        completionHandler: TodoBlock,
    ) -> Id<NSURLSessionDataTask, Shared> {
        msg_send_id![
            self,
            dataTaskWithRequest: request,
            completionHandler: completionHandler
        ]
    }
    pub unsafe fn dataTaskWithURL_completionHandler(
        &self,
        url: &NSURL,
        completionHandler: TodoBlock,
    ) -> Id<NSURLSessionDataTask, Shared> {
        msg_send_id![
            self,
            dataTaskWithURL: url,
            completionHandler: completionHandler
        ]
    }
    pub unsafe fn uploadTaskWithRequest_fromFile_completionHandler(
        &self,
        request: &NSURLRequest,
        fileURL: &NSURL,
        completionHandler: TodoBlock,
    ) -> Id<NSURLSessionUploadTask, Shared> {
        msg_send_id![
            self,
            uploadTaskWithRequest: request,
            fromFile: fileURL,
            completionHandler: completionHandler
        ]
    }
    pub unsafe fn uploadTaskWithRequest_fromData_completionHandler(
        &self,
        request: &NSURLRequest,
        bodyData: Option<&NSData>,
        completionHandler: TodoBlock,
    ) -> Id<NSURLSessionUploadTask, Shared> {
        msg_send_id![
            self,
            uploadTaskWithRequest: request,
            fromData: bodyData,
            completionHandler: completionHandler
        ]
    }
    pub unsafe fn downloadTaskWithRequest_completionHandler(
        &self,
        request: &NSURLRequest,
        completionHandler: TodoBlock,
    ) -> Id<NSURLSessionDownloadTask, Shared> {
        msg_send_id![
            self,
            downloadTaskWithRequest: request,
            completionHandler: completionHandler
        ]
    }
    pub unsafe fn downloadTaskWithURL_completionHandler(
        &self,
        url: &NSURL,
        completionHandler: TodoBlock,
    ) -> Id<NSURLSessionDownloadTask, Shared> {
        msg_send_id![
            self,
            downloadTaskWithURL: url,
            completionHandler: completionHandler
        ]
    }
    pub unsafe fn downloadTaskWithResumeData_completionHandler(
        &self,
        resumeData: &NSData,
        completionHandler: TodoBlock,
    ) -> Id<NSURLSessionDownloadTask, Shared> {
        msg_send_id![
            self,
            downloadTaskWithResumeData: resumeData,
            completionHandler: completionHandler
        ]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSURLSessionTask;
    unsafe impl ClassType for NSURLSessionTask {
        type Super = NSObject;
    }
);
impl NSURLSessionTask {
    pub unsafe fn cancel(&self) {
        msg_send![self, cancel]
    }
    pub unsafe fn suspend(&self) {
        msg_send![self, suspend]
    }
    pub unsafe fn resume(&self) {
        msg_send![self, resume]
    }
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn new() -> Id<Self, Shared> {
        msg_send_id![Self::class(), new]
    }
    pub unsafe fn taskIdentifier(&self) -> NSUInteger {
        msg_send![self, taskIdentifier]
    }
    pub unsafe fn originalRequest(&self) -> Option<Id<NSURLRequest, Shared>> {
        msg_send_id![self, originalRequest]
    }
    pub unsafe fn currentRequest(&self) -> Option<Id<NSURLRequest, Shared>> {
        msg_send_id![self, currentRequest]
    }
    pub unsafe fn response(&self) -> Option<Id<NSURLResponse, Shared>> {
        msg_send_id![self, response]
    }
    pub unsafe fn delegate(&self) -> TodoGenerics {
        msg_send![self, delegate]
    }
    pub unsafe fn setDelegate(&self, delegate: TodoGenerics) {
        msg_send![self, setDelegate: delegate]
    }
    pub unsafe fn progress(&self) -> Id<NSProgress, Shared> {
        msg_send_id![self, progress]
    }
    pub unsafe fn earliestBeginDate(&self) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, earliestBeginDate]
    }
    pub unsafe fn setEarliestBeginDate(&self, earliestBeginDate: Option<&NSDate>) {
        msg_send![self, setEarliestBeginDate: earliestBeginDate]
    }
    pub unsafe fn countOfBytesClientExpectsToSend(&self) -> int64_t {
        msg_send![self, countOfBytesClientExpectsToSend]
    }
    pub unsafe fn setCountOfBytesClientExpectsToSend(
        &self,
        countOfBytesClientExpectsToSend: int64_t,
    ) {
        msg_send![
            self,
            setCountOfBytesClientExpectsToSend: countOfBytesClientExpectsToSend
        ]
    }
    pub unsafe fn countOfBytesClientExpectsToReceive(&self) -> int64_t {
        msg_send![self, countOfBytesClientExpectsToReceive]
    }
    pub unsafe fn setCountOfBytesClientExpectsToReceive(
        &self,
        countOfBytesClientExpectsToReceive: int64_t,
    ) {
        msg_send![
            self,
            setCountOfBytesClientExpectsToReceive: countOfBytesClientExpectsToReceive
        ]
    }
    pub unsafe fn countOfBytesSent(&self) -> int64_t {
        msg_send![self, countOfBytesSent]
    }
    pub unsafe fn countOfBytesReceived(&self) -> int64_t {
        msg_send![self, countOfBytesReceived]
    }
    pub unsafe fn countOfBytesExpectedToSend(&self) -> int64_t {
        msg_send![self, countOfBytesExpectedToSend]
    }
    pub unsafe fn countOfBytesExpectedToReceive(&self) -> int64_t {
        msg_send![self, countOfBytesExpectedToReceive]
    }
    pub unsafe fn taskDescription(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, taskDescription]
    }
    pub unsafe fn setTaskDescription(&self, taskDescription: Option<&NSString>) {
        msg_send![self, setTaskDescription: taskDescription]
    }
    pub unsafe fn state(&self) -> NSURLSessionTaskState {
        msg_send![self, state]
    }
    pub unsafe fn error(&self) -> Option<Id<NSError, Shared>> {
        msg_send_id![self, error]
    }
    pub unsafe fn priority(&self) -> c_float {
        msg_send![self, priority]
    }
    pub unsafe fn setPriority(&self, priority: c_float) {
        msg_send![self, setPriority: priority]
    }
    pub unsafe fn prefersIncrementalDelivery(&self) -> bool {
        msg_send![self, prefersIncrementalDelivery]
    }
    pub unsafe fn setPrefersIncrementalDelivery(&self, prefersIncrementalDelivery: bool) {
        msg_send![
            self,
            setPrefersIncrementalDelivery: prefersIncrementalDelivery
        ]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSURLSessionDataTask;
    unsafe impl ClassType for NSURLSessionDataTask {
        type Super = NSURLSessionTask;
    }
);
impl NSURLSessionDataTask {
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn new() -> Id<Self, Shared> {
        msg_send_id![Self::class(), new]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSURLSessionUploadTask;
    unsafe impl ClassType for NSURLSessionUploadTask {
        type Super = NSURLSessionDataTask;
    }
);
impl NSURLSessionUploadTask {
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn new() -> Id<Self, Shared> {
        msg_send_id![Self::class(), new]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSURLSessionDownloadTask;
    unsafe impl ClassType for NSURLSessionDownloadTask {
        type Super = NSURLSessionTask;
    }
);
impl NSURLSessionDownloadTask {
    pub unsafe fn cancelByProducingResumeData(&self, completionHandler: TodoBlock) {
        msg_send![self, cancelByProducingResumeData: completionHandler]
    }
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn new() -> Id<Self, Shared> {
        msg_send_id![Self::class(), new]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSURLSessionStreamTask;
    unsafe impl ClassType for NSURLSessionStreamTask {
        type Super = NSURLSessionTask;
    }
);
impl NSURLSessionStreamTask {
    pub unsafe fn readDataOfMinLength_maxLength_timeout_completionHandler(
        &self,
        minBytes: NSUInteger,
        maxBytes: NSUInteger,
        timeout: NSTimeInterval,
        completionHandler: TodoBlock,
    ) {
        msg_send![
            self,
            readDataOfMinLength: minBytes,
            maxLength: maxBytes,
            timeout: timeout,
            completionHandler: completionHandler
        ]
    }
    pub unsafe fn writeData_timeout_completionHandler(
        &self,
        data: &NSData,
        timeout: NSTimeInterval,
        completionHandler: TodoBlock,
    ) {
        msg_send![
            self,
            writeData: data,
            timeout: timeout,
            completionHandler: completionHandler
        ]
    }
    pub unsafe fn captureStreams(&self) {
        msg_send![self, captureStreams]
    }
    pub unsafe fn closeWrite(&self) {
        msg_send![self, closeWrite]
    }
    pub unsafe fn closeRead(&self) {
        msg_send![self, closeRead]
    }
    pub unsafe fn startSecureConnection(&self) {
        msg_send![self, startSecureConnection]
    }
    pub unsafe fn stopSecureConnection(&self) {
        msg_send![self, stopSecureConnection]
    }
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn new() -> Id<Self, Shared> {
        msg_send_id![Self::class(), new]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSURLSessionWebSocketMessage;
    unsafe impl ClassType for NSURLSessionWebSocketMessage {
        type Super = NSObject;
    }
);
impl NSURLSessionWebSocketMessage {
    pub unsafe fn initWithData(&self, data: &NSData) -> Id<Self, Shared> {
        msg_send_id![self, initWithData: data]
    }
    pub unsafe fn initWithString(&self, string: &NSString) -> Id<Self, Shared> {
        msg_send_id![self, initWithString: string]
    }
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn new() -> Id<Self, Shared> {
        msg_send_id![Self::class(), new]
    }
    pub unsafe fn type_(&self) -> NSURLSessionWebSocketMessageType {
        msg_send![self, type]
    }
    pub unsafe fn data(&self) -> Option<Id<NSData, Shared>> {
        msg_send_id![self, data]
    }
    pub unsafe fn string(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, string]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSURLSessionWebSocketTask;
    unsafe impl ClassType for NSURLSessionWebSocketTask {
        type Super = NSURLSessionTask;
    }
);
impl NSURLSessionWebSocketTask {
    pub unsafe fn sendMessage_completionHandler(
        &self,
        message: &NSURLSessionWebSocketMessage,
        completionHandler: TodoBlock,
    ) {
        msg_send![
            self,
            sendMessage: message,
            completionHandler: completionHandler
        ]
    }
    pub unsafe fn receiveMessageWithCompletionHandler(&self, completionHandler: TodoBlock) {
        msg_send![self, receiveMessageWithCompletionHandler: completionHandler]
    }
    pub unsafe fn sendPingWithPongReceiveHandler(&self, pongReceiveHandler: TodoBlock) {
        msg_send![self, sendPingWithPongReceiveHandler: pongReceiveHandler]
    }
    pub unsafe fn cancelWithCloseCode_reason(
        &self,
        closeCode: NSURLSessionWebSocketCloseCode,
        reason: Option<&NSData>,
    ) {
        msg_send![self, cancelWithCloseCode: closeCode, reason: reason]
    }
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn new() -> Id<Self, Shared> {
        msg_send_id![Self::class(), new]
    }
    pub unsafe fn maximumMessageSize(&self) -> NSInteger {
        msg_send![self, maximumMessageSize]
    }
    pub unsafe fn setMaximumMessageSize(&self, maximumMessageSize: NSInteger) {
        msg_send![self, setMaximumMessageSize: maximumMessageSize]
    }
    pub unsafe fn closeCode(&self) -> NSURLSessionWebSocketCloseCode {
        msg_send![self, closeCode]
    }
    pub unsafe fn closeReason(&self) -> Option<Id<NSData, Shared>> {
        msg_send_id![self, closeReason]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSURLSessionConfiguration;
    unsafe impl ClassType for NSURLSessionConfiguration {
        type Super = NSObject;
    }
);
impl NSURLSessionConfiguration {
    pub unsafe fn backgroundSessionConfigurationWithIdentifier(
        identifier: &NSString,
    ) -> Id<NSURLSessionConfiguration, Shared> {
        msg_send_id![
            Self::class(),
            backgroundSessionConfigurationWithIdentifier: identifier
        ]
    }
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn new() -> Id<Self, Shared> {
        msg_send_id![Self::class(), new]
    }
    pub unsafe fn defaultSessionConfiguration() -> Id<NSURLSessionConfiguration, Shared> {
        msg_send_id![Self::class(), defaultSessionConfiguration]
    }
    pub unsafe fn ephemeralSessionConfiguration() -> Id<NSURLSessionConfiguration, Shared> {
        msg_send_id![Self::class(), ephemeralSessionConfiguration]
    }
    pub unsafe fn identifier(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, identifier]
    }
    pub unsafe fn requestCachePolicy(&self) -> NSURLRequestCachePolicy {
        msg_send![self, requestCachePolicy]
    }
    pub unsafe fn setRequestCachePolicy(&self, requestCachePolicy: NSURLRequestCachePolicy) {
        msg_send![self, setRequestCachePolicy: requestCachePolicy]
    }
    pub unsafe fn timeoutIntervalForRequest(&self) -> NSTimeInterval {
        msg_send![self, timeoutIntervalForRequest]
    }
    pub unsafe fn setTimeoutIntervalForRequest(&self, timeoutIntervalForRequest: NSTimeInterval) {
        msg_send![
            self,
            setTimeoutIntervalForRequest: timeoutIntervalForRequest
        ]
    }
    pub unsafe fn timeoutIntervalForResource(&self) -> NSTimeInterval {
        msg_send![self, timeoutIntervalForResource]
    }
    pub unsafe fn setTimeoutIntervalForResource(&self, timeoutIntervalForResource: NSTimeInterval) {
        msg_send![
            self,
            setTimeoutIntervalForResource: timeoutIntervalForResource
        ]
    }
    pub unsafe fn networkServiceType(&self) -> NSURLRequestNetworkServiceType {
        msg_send![self, networkServiceType]
    }
    pub unsafe fn setNetworkServiceType(&self, networkServiceType: NSURLRequestNetworkServiceType) {
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
    pub unsafe fn setAllowsConstrainedNetworkAccess(&self, allowsConstrainedNetworkAccess: bool) {
        msg_send![
            self,
            setAllowsConstrainedNetworkAccess: allowsConstrainedNetworkAccess
        ]
    }
    pub unsafe fn waitsForConnectivity(&self) -> bool {
        msg_send![self, waitsForConnectivity]
    }
    pub unsafe fn setWaitsForConnectivity(&self, waitsForConnectivity: bool) {
        msg_send![self, setWaitsForConnectivity: waitsForConnectivity]
    }
    pub unsafe fn isDiscretionary(&self) -> bool {
        msg_send![self, isDiscretionary]
    }
    pub unsafe fn setDiscretionary(&self, discretionary: bool) {
        msg_send![self, setDiscretionary: discretionary]
    }
    pub unsafe fn sharedContainerIdentifier(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, sharedContainerIdentifier]
    }
    pub unsafe fn setSharedContainerIdentifier(
        &self,
        sharedContainerIdentifier: Option<&NSString>,
    ) {
        msg_send![
            self,
            setSharedContainerIdentifier: sharedContainerIdentifier
        ]
    }
    pub unsafe fn sessionSendsLaunchEvents(&self) -> bool {
        msg_send![self, sessionSendsLaunchEvents]
    }
    pub unsafe fn setSessionSendsLaunchEvents(&self, sessionSendsLaunchEvents: bool) {
        msg_send![self, setSessionSendsLaunchEvents: sessionSendsLaunchEvents]
    }
    pub unsafe fn connectionProxyDictionary(&self) -> Option<Id<NSDictionary, Shared>> {
        msg_send_id![self, connectionProxyDictionary]
    }
    pub unsafe fn setConnectionProxyDictionary(
        &self,
        connectionProxyDictionary: Option<&NSDictionary>,
    ) {
        msg_send![
            self,
            setConnectionProxyDictionary: connectionProxyDictionary
        ]
    }
    pub unsafe fn TLSMinimumSupportedProtocol(&self) -> SSLProtocol {
        msg_send![self, TLSMinimumSupportedProtocol]
    }
    pub unsafe fn setTLSMinimumSupportedProtocol(&self, TLSMinimumSupportedProtocol: SSLProtocol) {
        msg_send![
            self,
            setTLSMinimumSupportedProtocol: TLSMinimumSupportedProtocol
        ]
    }
    pub unsafe fn TLSMaximumSupportedProtocol(&self) -> SSLProtocol {
        msg_send![self, TLSMaximumSupportedProtocol]
    }
    pub unsafe fn setTLSMaximumSupportedProtocol(&self, TLSMaximumSupportedProtocol: SSLProtocol) {
        msg_send![
            self,
            setTLSMaximumSupportedProtocol: TLSMaximumSupportedProtocol
        ]
    }
    pub unsafe fn TLSMinimumSupportedProtocolVersion(&self) -> tls_protocol_version_t {
        msg_send![self, TLSMinimumSupportedProtocolVersion]
    }
    pub unsafe fn setTLSMinimumSupportedProtocolVersion(
        &self,
        TLSMinimumSupportedProtocolVersion: tls_protocol_version_t,
    ) {
        msg_send![
            self,
            setTLSMinimumSupportedProtocolVersion: TLSMinimumSupportedProtocolVersion
        ]
    }
    pub unsafe fn TLSMaximumSupportedProtocolVersion(&self) -> tls_protocol_version_t {
        msg_send![self, TLSMaximumSupportedProtocolVersion]
    }
    pub unsafe fn setTLSMaximumSupportedProtocolVersion(
        &self,
        TLSMaximumSupportedProtocolVersion: tls_protocol_version_t,
    ) {
        msg_send![
            self,
            setTLSMaximumSupportedProtocolVersion: TLSMaximumSupportedProtocolVersion
        ]
    }
    pub unsafe fn HTTPShouldUsePipelining(&self) -> bool {
        msg_send![self, HTTPShouldUsePipelining]
    }
    pub unsafe fn setHTTPShouldUsePipelining(&self, HTTPShouldUsePipelining: bool) {
        msg_send![self, setHTTPShouldUsePipelining: HTTPShouldUsePipelining]
    }
    pub unsafe fn HTTPShouldSetCookies(&self) -> bool {
        msg_send![self, HTTPShouldSetCookies]
    }
    pub unsafe fn setHTTPShouldSetCookies(&self, HTTPShouldSetCookies: bool) {
        msg_send![self, setHTTPShouldSetCookies: HTTPShouldSetCookies]
    }
    pub unsafe fn HTTPCookieAcceptPolicy(&self) -> NSHTTPCookieAcceptPolicy {
        msg_send![self, HTTPCookieAcceptPolicy]
    }
    pub unsafe fn setHTTPCookieAcceptPolicy(
        &self,
        HTTPCookieAcceptPolicy: NSHTTPCookieAcceptPolicy,
    ) {
        msg_send![self, setHTTPCookieAcceptPolicy: HTTPCookieAcceptPolicy]
    }
    pub unsafe fn HTTPAdditionalHeaders(&self) -> Option<Id<NSDictionary, Shared>> {
        msg_send_id![self, HTTPAdditionalHeaders]
    }
    pub unsafe fn setHTTPAdditionalHeaders(&self, HTTPAdditionalHeaders: Option<&NSDictionary>) {
        msg_send![self, setHTTPAdditionalHeaders: HTTPAdditionalHeaders]
    }
    pub unsafe fn HTTPMaximumConnectionsPerHost(&self) -> NSInteger {
        msg_send![self, HTTPMaximumConnectionsPerHost]
    }
    pub unsafe fn setHTTPMaximumConnectionsPerHost(
        &self,
        HTTPMaximumConnectionsPerHost: NSInteger,
    ) {
        msg_send![
            self,
            setHTTPMaximumConnectionsPerHost: HTTPMaximumConnectionsPerHost
        ]
    }
    pub unsafe fn HTTPCookieStorage(&self) -> Option<Id<NSHTTPCookieStorage, Shared>> {
        msg_send_id![self, HTTPCookieStorage]
    }
    pub unsafe fn setHTTPCookieStorage(&self, HTTPCookieStorage: Option<&NSHTTPCookieStorage>) {
        msg_send![self, setHTTPCookieStorage: HTTPCookieStorage]
    }
    pub unsafe fn URLCredentialStorage(&self) -> Option<Id<NSURLCredentialStorage, Shared>> {
        msg_send_id![self, URLCredentialStorage]
    }
    pub unsafe fn setURLCredentialStorage(
        &self,
        URLCredentialStorage: Option<&NSURLCredentialStorage>,
    ) {
        msg_send![self, setURLCredentialStorage: URLCredentialStorage]
    }
    pub unsafe fn URLCache(&self) -> Option<Id<NSURLCache, Shared>> {
        msg_send_id![self, URLCache]
    }
    pub unsafe fn setURLCache(&self, URLCache: Option<&NSURLCache>) {
        msg_send![self, setURLCache: URLCache]
    }
    pub unsafe fn shouldUseExtendedBackgroundIdleMode(&self) -> bool {
        msg_send![self, shouldUseExtendedBackgroundIdleMode]
    }
    pub unsafe fn setShouldUseExtendedBackgroundIdleMode(
        &self,
        shouldUseExtendedBackgroundIdleMode: bool,
    ) {
        msg_send![
            self,
            setShouldUseExtendedBackgroundIdleMode: shouldUseExtendedBackgroundIdleMode
        ]
    }
    pub unsafe fn protocolClasses(&self) -> TodoGenerics {
        msg_send![self, protocolClasses]
    }
    pub unsafe fn setProtocolClasses(&self, protocolClasses: TodoGenerics) {
        msg_send![self, setProtocolClasses: protocolClasses]
    }
    pub unsafe fn multipathServiceType(&self) -> NSURLSessionMultipathServiceType {
        msg_send![self, multipathServiceType]
    }
    pub unsafe fn setMultipathServiceType(
        &self,
        multipathServiceType: NSURLSessionMultipathServiceType,
    ) {
        msg_send![self, setMultipathServiceType: multipathServiceType]
    }
}
pub type NSURLSessionDelegate = NSObject;
pub type NSURLSessionTaskDelegate = NSObject;
pub type NSURLSessionDataDelegate = NSObject;
pub type NSURLSessionDownloadDelegate = NSObject;
pub type NSURLSessionStreamDelegate = NSObject;
pub type NSURLSessionWebSocketDelegate = NSObject;
#[doc = "NSURLSessionDeprecated"]
impl NSURLSessionConfiguration {
    pub unsafe fn backgroundSessionConfiguration(
        identifier: &NSString,
    ) -> Id<NSURLSessionConfiguration, Shared> {
        msg_send_id![Self::class(), backgroundSessionConfiguration: identifier]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSURLSessionTaskTransactionMetrics;
    unsafe impl ClassType for NSURLSessionTaskTransactionMetrics {
        type Super = NSObject;
    }
);
impl NSURLSessionTaskTransactionMetrics {
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn new() -> Id<Self, Shared> {
        msg_send_id![Self::class(), new]
    }
    pub unsafe fn request(&self) -> Id<NSURLRequest, Shared> {
        msg_send_id![self, request]
    }
    pub unsafe fn response(&self) -> Option<Id<NSURLResponse, Shared>> {
        msg_send_id![self, response]
    }
    pub unsafe fn fetchStartDate(&self) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, fetchStartDate]
    }
    pub unsafe fn domainLookupStartDate(&self) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, domainLookupStartDate]
    }
    pub unsafe fn domainLookupEndDate(&self) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, domainLookupEndDate]
    }
    pub unsafe fn connectStartDate(&self) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, connectStartDate]
    }
    pub unsafe fn secureConnectionStartDate(&self) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, secureConnectionStartDate]
    }
    pub unsafe fn secureConnectionEndDate(&self) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, secureConnectionEndDate]
    }
    pub unsafe fn connectEndDate(&self) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, connectEndDate]
    }
    pub unsafe fn requestStartDate(&self) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, requestStartDate]
    }
    pub unsafe fn requestEndDate(&self) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, requestEndDate]
    }
    pub unsafe fn responseStartDate(&self) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, responseStartDate]
    }
    pub unsafe fn responseEndDate(&self) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, responseEndDate]
    }
    pub unsafe fn networkProtocolName(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, networkProtocolName]
    }
    pub unsafe fn isProxyConnection(&self) -> bool {
        msg_send![self, isProxyConnection]
    }
    pub unsafe fn isReusedConnection(&self) -> bool {
        msg_send![self, isReusedConnection]
    }
    pub unsafe fn resourceFetchType(&self) -> NSURLSessionTaskMetricsResourceFetchType {
        msg_send![self, resourceFetchType]
    }
    pub unsafe fn countOfRequestHeaderBytesSent(&self) -> int64_t {
        msg_send![self, countOfRequestHeaderBytesSent]
    }
    pub unsafe fn countOfRequestBodyBytesSent(&self) -> int64_t {
        msg_send![self, countOfRequestBodyBytesSent]
    }
    pub unsafe fn countOfRequestBodyBytesBeforeEncoding(&self) -> int64_t {
        msg_send![self, countOfRequestBodyBytesBeforeEncoding]
    }
    pub unsafe fn countOfResponseHeaderBytesReceived(&self) -> int64_t {
        msg_send![self, countOfResponseHeaderBytesReceived]
    }
    pub unsafe fn countOfResponseBodyBytesReceived(&self) -> int64_t {
        msg_send![self, countOfResponseBodyBytesReceived]
    }
    pub unsafe fn countOfResponseBodyBytesAfterDecoding(&self) -> int64_t {
        msg_send![self, countOfResponseBodyBytesAfterDecoding]
    }
    pub unsafe fn localAddress(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, localAddress]
    }
    pub unsafe fn localPort(&self) -> Option<Id<NSNumber, Shared>> {
        msg_send_id![self, localPort]
    }
    pub unsafe fn remoteAddress(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, remoteAddress]
    }
    pub unsafe fn remotePort(&self) -> Option<Id<NSNumber, Shared>> {
        msg_send_id![self, remotePort]
    }
    pub unsafe fn negotiatedTLSProtocolVersion(&self) -> Option<Id<NSNumber, Shared>> {
        msg_send_id![self, negotiatedTLSProtocolVersion]
    }
    pub unsafe fn negotiatedTLSCipherSuite(&self) -> Option<Id<NSNumber, Shared>> {
        msg_send_id![self, negotiatedTLSCipherSuite]
    }
    pub unsafe fn isCellular(&self) -> bool {
        msg_send![self, isCellular]
    }
    pub unsafe fn isExpensive(&self) -> bool {
        msg_send![self, isExpensive]
    }
    pub unsafe fn isConstrained(&self) -> bool {
        msg_send![self, isConstrained]
    }
    pub unsafe fn isMultipath(&self) -> bool {
        msg_send![self, isMultipath]
    }
    pub unsafe fn domainResolutionProtocol(
        &self,
    ) -> NSURLSessionTaskMetricsDomainResolutionProtocol {
        msg_send![self, domainResolutionProtocol]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSURLSessionTaskMetrics;
    unsafe impl ClassType for NSURLSessionTaskMetrics {
        type Super = NSObject;
    }
);
impl NSURLSessionTaskMetrics {
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn new() -> Id<Self, Shared> {
        msg_send_id![Self::class(), new]
    }
    pub unsafe fn transactionMetrics(&self) -> TodoGenerics {
        msg_send![self, transactionMetrics]
    }
    pub unsafe fn taskInterval(&self) -> Id<NSDateInterval, Shared> {
        msg_send_id![self, taskInterval]
    }
    pub unsafe fn redirectCount(&self) -> NSUInteger {
        msg_send![self, redirectCount]
    }
}
