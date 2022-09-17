use super::NSArray;
use super::NSData;
use super::NSDictionary;
use super::NSInputStream;
use super::NSNumber;
use super::NSOutputStream;
use super::NSRunLoop;
use super::NSString;
use crate::Foundation::generated::NSDate::*;
use crate::Foundation::generated::NSError::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSRunLoop::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSNetService;
    unsafe impl ClassType for NSNetService {
        type Super = NSObject;
    }
);
impl NSNetService {
    pub unsafe fn initWithDomain_type_name_port(
        &self,
        domain: &NSString,
        type_: &NSString,
        name: &NSString,
        port: c_int,
    ) -> Id<Self, Shared> {
        msg_send_id ! [self , initWithDomain : domain , type : type_ , name : name , port : port]
    }
    pub unsafe fn initWithDomain_type_name(
        &self,
        domain: &NSString,
        type_: &NSString,
        name: &NSString,
    ) -> Id<Self, Shared> {
        msg_send_id ! [self , initWithDomain : domain , type : type_ , name : name]
    }
    pub unsafe fn scheduleInRunLoop_forMode(&self, aRunLoop: &NSRunLoop, mode: NSRunLoopMode) {
        msg_send![self, scheduleInRunLoop: aRunLoop, forMode: mode]
    }
    pub unsafe fn removeFromRunLoop_forMode(&self, aRunLoop: &NSRunLoop, mode: NSRunLoopMode) {
        msg_send![self, removeFromRunLoop: aRunLoop, forMode: mode]
    }
    pub unsafe fn publish(&self) {
        msg_send![self, publish]
    }
    pub unsafe fn publishWithOptions(&self, options: NSNetServiceOptions) {
        msg_send![self, publishWithOptions: options]
    }
    pub unsafe fn resolve(&self) {
        msg_send![self, resolve]
    }
    pub unsafe fn stop(&self) {
        msg_send![self, stop]
    }
    pub unsafe fn dictionaryFromTXTRecordData(txtData: &NSData) -> TodoGenerics {
        msg_send![Self::class(), dictionaryFromTXTRecordData: txtData]
    }
    pub unsafe fn dataFromTXTRecordDictionary(txtDictionary: TodoGenerics) -> Id<NSData, Shared> {
        msg_send_id![Self::class(), dataFromTXTRecordDictionary: txtDictionary]
    }
    pub unsafe fn resolveWithTimeout(&self, timeout: NSTimeInterval) {
        msg_send![self, resolveWithTimeout: timeout]
    }
    pub unsafe fn getInputStream_outputStream(
        &self,
        inputStream: *mut Option<&NSInputStream>,
        outputStream: *mut Option<&NSOutputStream>,
    ) -> bool {
        msg_send![
            self,
            getInputStream: inputStream,
            outputStream: outputStream
        ]
    }
    pub unsafe fn setTXTRecordData(&self, recordData: Option<&NSData>) -> bool {
        msg_send![self, setTXTRecordData: recordData]
    }
    pub unsafe fn TXTRecordData(&self) -> Option<Id<NSData, Shared>> {
        msg_send_id![self, TXTRecordData]
    }
    pub unsafe fn startMonitoring(&self) {
        msg_send![self, startMonitoring]
    }
    pub unsafe fn stopMonitoring(&self) {
        msg_send![self, stopMonitoring]
    }
    pub unsafe fn delegate(&self) -> TodoGenerics {
        msg_send![self, delegate]
    }
    pub unsafe fn setDelegate(&self, delegate: TodoGenerics) {
        msg_send![self, setDelegate: delegate]
    }
    pub unsafe fn includesPeerToPeer(&self) -> bool {
        msg_send![self, includesPeerToPeer]
    }
    pub unsafe fn setIncludesPeerToPeer(&self, includesPeerToPeer: bool) {
        msg_send![self, setIncludesPeerToPeer: includesPeerToPeer]
    }
    pub unsafe fn name(&self) -> Id<NSString, Shared> {
        msg_send_id![self, name]
    }
    pub unsafe fn type_(&self) -> Id<NSString, Shared> {
        msg_send_id![self, type]
    }
    pub unsafe fn domain(&self) -> Id<NSString, Shared> {
        msg_send_id![self, domain]
    }
    pub unsafe fn hostName(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, hostName]
    }
    pub unsafe fn addresses(&self) -> TodoGenerics {
        msg_send![self, addresses]
    }
    pub unsafe fn port(&self) -> NSInteger {
        msg_send![self, port]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSNetServiceBrowser;
    unsafe impl ClassType for NSNetServiceBrowser {
        type Super = NSObject;
    }
);
impl NSNetServiceBrowser {
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn scheduleInRunLoop_forMode(&self, aRunLoop: &NSRunLoop, mode: NSRunLoopMode) {
        msg_send![self, scheduleInRunLoop: aRunLoop, forMode: mode]
    }
    pub unsafe fn removeFromRunLoop_forMode(&self, aRunLoop: &NSRunLoop, mode: NSRunLoopMode) {
        msg_send![self, removeFromRunLoop: aRunLoop, forMode: mode]
    }
    pub unsafe fn searchForBrowsableDomains(&self) {
        msg_send![self, searchForBrowsableDomains]
    }
    pub unsafe fn searchForRegistrationDomains(&self) {
        msg_send![self, searchForRegistrationDomains]
    }
    pub unsafe fn searchForServicesOfType_inDomain(
        &self,
        type_: &NSString,
        domainString: &NSString,
    ) {
        msg_send![self, searchForServicesOfType: type_, inDomain: domainString]
    }
    pub unsafe fn stop(&self) {
        msg_send![self, stop]
    }
    pub unsafe fn delegate(&self) -> TodoGenerics {
        msg_send![self, delegate]
    }
    pub unsafe fn setDelegate(&self, delegate: TodoGenerics) {
        msg_send![self, setDelegate: delegate]
    }
    pub unsafe fn includesPeerToPeer(&self) -> bool {
        msg_send![self, includesPeerToPeer]
    }
    pub unsafe fn setIncludesPeerToPeer(&self, includesPeerToPeer: bool) {
        msg_send![self, setIncludesPeerToPeer: includesPeerToPeer]
    }
}
pub type NSNetServiceDelegate = NSObject;
pub type NSNetServiceBrowserDelegate = NSObject;
