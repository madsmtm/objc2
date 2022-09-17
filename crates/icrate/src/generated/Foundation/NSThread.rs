use super::NSArray;
use super::NSDate;
use super::NSMutableDictionary;
use super::NSNumber;
use super::NSString;
use crate::Foundation::generated::NSDate::*;
use crate::Foundation::generated::NSNotification::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSThread;
    unsafe impl ClassType for NSThread {
        type Super = NSObject;
    }
);
impl NSThread {
    pub unsafe fn detachNewThreadWithBlock(block: TodoBlock) {
        msg_send![Self::class(), detachNewThreadWithBlock: block]
    }
    pub unsafe fn detachNewThreadSelector_toTarget_withObject(
        selector: Sel,
        target: &Object,
        argument: Option<&Object>,
    ) {
        msg_send![
            Self::class(),
            detachNewThreadSelector: selector,
            toTarget: target,
            withObject: argument
        ]
    }
    pub unsafe fn isMultiThreaded() -> bool {
        msg_send![Self::class(), isMultiThreaded]
    }
    pub unsafe fn sleepUntilDate(date: &NSDate) {
        msg_send![Self::class(), sleepUntilDate: date]
    }
    pub unsafe fn sleepForTimeInterval(ti: NSTimeInterval) {
        msg_send![Self::class(), sleepForTimeInterval: ti]
    }
    pub unsafe fn exit() {
        msg_send![Self::class(), exit]
    }
    pub unsafe fn threadPriority() -> c_double {
        msg_send![Self::class(), threadPriority]
    }
    pub unsafe fn setThreadPriority(p: c_double) -> bool {
        msg_send![Self::class(), setThreadPriority: p]
    }
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn initWithTarget_selector_object(
        &self,
        target: &Object,
        selector: Sel,
        argument: Option<&Object>,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithTarget: target,
            selector: selector,
            object: argument
        ]
    }
    pub unsafe fn initWithBlock(&self, block: TodoBlock) -> Id<Self, Shared> {
        msg_send_id![self, initWithBlock: block]
    }
    pub unsafe fn cancel(&self) {
        msg_send![self, cancel]
    }
    pub unsafe fn start(&self) {
        msg_send![self, start]
    }
    pub unsafe fn main(&self) {
        msg_send![self, main]
    }
    pub unsafe fn currentThread() -> Id<NSThread, Shared> {
        msg_send_id![Self::class(), currentThread]
    }
    pub unsafe fn threadDictionary(&self) -> Id<NSMutableDictionary, Shared> {
        msg_send_id![self, threadDictionary]
    }
    pub unsafe fn threadPriority(&self) -> c_double {
        msg_send![self, threadPriority]
    }
    pub unsafe fn setThreadPriority(&self, threadPriority: c_double) {
        msg_send![self, setThreadPriority: threadPriority]
    }
    pub unsafe fn qualityOfService(&self) -> NSQualityOfService {
        msg_send![self, qualityOfService]
    }
    pub unsafe fn setQualityOfService(&self, qualityOfService: NSQualityOfService) {
        msg_send![self, setQualityOfService: qualityOfService]
    }
    pub unsafe fn callStackReturnAddresses() -> TodoGenerics {
        msg_send![Self::class(), callStackReturnAddresses]
    }
    pub unsafe fn callStackSymbols() -> TodoGenerics {
        msg_send![Self::class(), callStackSymbols]
    }
    pub unsafe fn name(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, name]
    }
    pub unsafe fn setName(&self, name: Option<&NSString>) {
        msg_send![self, setName: name]
    }
    pub unsafe fn stackSize(&self) -> NSUInteger {
        msg_send![self, stackSize]
    }
    pub unsafe fn setStackSize(&self, stackSize: NSUInteger) {
        msg_send![self, setStackSize: stackSize]
    }
    pub unsafe fn isMainThread(&self) -> bool {
        msg_send![self, isMainThread]
    }
    pub unsafe fn isMainThread() -> bool {
        msg_send![Self::class(), isMainThread]
    }
    pub unsafe fn mainThread() -> Id<NSThread, Shared> {
        msg_send_id![Self::class(), mainThread]
    }
    pub unsafe fn isExecuting(&self) -> bool {
        msg_send![self, isExecuting]
    }
    pub unsafe fn isFinished(&self) -> bool {
        msg_send![self, isFinished]
    }
    pub unsafe fn isCancelled(&self) -> bool {
        msg_send![self, isCancelled]
    }
}
#[doc = "NSThreadPerformAdditions"]
impl NSObject {
    pub unsafe fn performSelectorOnMainThread_withObject_waitUntilDone_modes(
        &self,
        aSelector: Sel,
        arg: Option<&Object>,
        wait: bool,
        array: TodoGenerics,
    ) {
        msg_send![
            self,
            performSelectorOnMainThread: aSelector,
            withObject: arg,
            waitUntilDone: wait,
            modes: array
        ]
    }
    pub unsafe fn performSelectorOnMainThread_withObject_waitUntilDone(
        &self,
        aSelector: Sel,
        arg: Option<&Object>,
        wait: bool,
    ) {
        msg_send![
            self,
            performSelectorOnMainThread: aSelector,
            withObject: arg,
            waitUntilDone: wait
        ]
    }
    pub unsafe fn performSelector_onThread_withObject_waitUntilDone_modes(
        &self,
        aSelector: Sel,
        thr: &NSThread,
        arg: Option<&Object>,
        wait: bool,
        array: TodoGenerics,
    ) {
        msg_send![
            self,
            performSelector: aSelector,
            onThread: thr,
            withObject: arg,
            waitUntilDone: wait,
            modes: array
        ]
    }
    pub unsafe fn performSelector_onThread_withObject_waitUntilDone(
        &self,
        aSelector: Sel,
        thr: &NSThread,
        arg: Option<&Object>,
        wait: bool,
    ) {
        msg_send![
            self,
            performSelector: aSelector,
            onThread: thr,
            withObject: arg,
            waitUntilDone: wait
        ]
    }
    pub unsafe fn performSelectorInBackground_withObject(
        &self,
        aSelector: Sel,
        arg: Option<&Object>,
    ) {
        msg_send![
            self,
            performSelectorInBackground: aSelector,
            withObject: arg
        ]
    }
}
