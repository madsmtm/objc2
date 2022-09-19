use super::__exported::NSArray;
use super::__exported::NSSet;
use crate::dispatch::generated::dispatch::*;
use crate::sys::generated::qos::*;
use crate::Foundation::generated::NSException::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSProgress::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSOperation;
    unsafe impl ClassType for NSOperation {
        type Super = NSObject;
    }
);
impl NSOperation {
    pub unsafe fn start(&self) {
        msg_send![self, start]
    }
    pub unsafe fn main(&self) {
        msg_send![self, main]
    }
    pub unsafe fn cancel(&self) {
        msg_send![self, cancel]
    }
    pub unsafe fn addDependency(&self, op: &NSOperation) {
        msg_send![self, addDependency: op]
    }
    pub unsafe fn removeDependency(&self, op: &NSOperation) {
        msg_send![self, removeDependency: op]
    }
    pub unsafe fn waitUntilFinished(&self) {
        msg_send![self, waitUntilFinished]
    }
    pub unsafe fn isCancelled(&self) -> bool {
        msg_send![self, isCancelled]
    }
    pub unsafe fn isExecuting(&self) -> bool {
        msg_send![self, isExecuting]
    }
    pub unsafe fn isFinished(&self) -> bool {
        msg_send![self, isFinished]
    }
    pub unsafe fn isConcurrent(&self) -> bool {
        msg_send![self, isConcurrent]
    }
    pub unsafe fn isAsynchronous(&self) -> bool {
        msg_send![self, isAsynchronous]
    }
    pub unsafe fn isReady(&self) -> bool {
        msg_send![self, isReady]
    }
    pub unsafe fn dependencies(&self) -> Id<NSArray<NSOperation>, Shared> {
        msg_send_id![self, dependencies]
    }
    pub unsafe fn queuePriority(&self) -> NSOperationQueuePriority {
        msg_send![self, queuePriority]
    }
    pub unsafe fn setQueuePriority(&self, queuePriority: NSOperationQueuePriority) {
        msg_send![self, setQueuePriority: queuePriority]
    }
    pub unsafe fn completionBlock(&self) -> TodoBlock {
        msg_send![self, completionBlock]
    }
    pub unsafe fn setCompletionBlock(&self, completionBlock: TodoBlock) {
        msg_send![self, setCompletionBlock: completionBlock]
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
    pub unsafe fn name(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, name]
    }
    pub unsafe fn setName(&self, name: Option<&NSString>) {
        msg_send![self, setName: name]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSBlockOperation;
    unsafe impl ClassType for NSBlockOperation {
        type Super = NSOperation;
    }
);
impl NSBlockOperation {
    pub unsafe fn blockOperationWithBlock(block: TodoBlock) -> Id<Self, Shared> {
        msg_send_id![Self::class(), blockOperationWithBlock: block]
    }
    pub unsafe fn addExecutionBlock(&self, block: TodoBlock) {
        msg_send![self, addExecutionBlock: block]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSInvocationOperation;
    unsafe impl ClassType for NSInvocationOperation {
        type Super = NSOperation;
    }
);
impl NSInvocationOperation {
    pub unsafe fn initWithTarget_selector_object(
        &self,
        target: &Object,
        sel: Sel,
        arg: Option<&Object>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithTarget: target, selector: sel, object: arg]
    }
    pub unsafe fn initWithInvocation(&self, inv: &NSInvocation) -> Id<Self, Shared> {
        msg_send_id![self, initWithInvocation: inv]
    }
    pub unsafe fn invocation(&self) -> Id<NSInvocation, Shared> {
        msg_send_id![self, invocation]
    }
    pub unsafe fn result(&self) -> Option<Id<Object, Shared>> {
        msg_send_id![self, result]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSOperationQueue;
    unsafe impl ClassType for NSOperationQueue {
        type Super = NSObject;
    }
);
impl NSOperationQueue {
    pub unsafe fn addOperation(&self, op: &NSOperation) {
        msg_send![self, addOperation: op]
    }
    pub unsafe fn addOperations_waitUntilFinished(&self, ops: &NSArray<NSOperation>, wait: bool) {
        msg_send![self, addOperations: ops, waitUntilFinished: wait]
    }
    pub unsafe fn addOperationWithBlock(&self, block: TodoBlock) {
        msg_send![self, addOperationWithBlock: block]
    }
    pub unsafe fn addBarrierBlock(&self, barrier: TodoBlock) {
        msg_send![self, addBarrierBlock: barrier]
    }
    pub unsafe fn cancelAllOperations(&self) {
        msg_send![self, cancelAllOperations]
    }
    pub unsafe fn waitUntilAllOperationsAreFinished(&self) {
        msg_send![self, waitUntilAllOperationsAreFinished]
    }
    pub unsafe fn progress(&self) -> Id<NSProgress, Shared> {
        msg_send_id![self, progress]
    }
    pub unsafe fn maxConcurrentOperationCount(&self) -> NSInteger {
        msg_send![self, maxConcurrentOperationCount]
    }
    pub unsafe fn setMaxConcurrentOperationCount(&self, maxConcurrentOperationCount: NSInteger) {
        msg_send![
            self,
            setMaxConcurrentOperationCount: maxConcurrentOperationCount
        ]
    }
    pub unsafe fn isSuspended(&self) -> bool {
        msg_send![self, isSuspended]
    }
    pub unsafe fn setSuspended(&self, suspended: bool) {
        msg_send![self, setSuspended: suspended]
    }
    pub unsafe fn name(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, name]
    }
    pub unsafe fn setName(&self, name: Option<&NSString>) {
        msg_send![self, setName: name]
    }
    pub unsafe fn qualityOfService(&self) -> NSQualityOfService {
        msg_send![self, qualityOfService]
    }
    pub unsafe fn setQualityOfService(&self, qualityOfService: NSQualityOfService) {
        msg_send![self, setQualityOfService: qualityOfService]
    }
    pub unsafe fn underlyingQueue(&self) -> Option<Id<dispatch_queue_t, Shared>> {
        msg_send_id![self, underlyingQueue]
    }
    pub unsafe fn setUnderlyingQueue(&self, underlyingQueue: Option<&dispatch_queue_t>) {
        msg_send![self, setUnderlyingQueue: underlyingQueue]
    }
    pub unsafe fn currentQueue() -> Option<Id<NSOperationQueue, Shared>> {
        msg_send_id![Self::class(), currentQueue]
    }
    pub unsafe fn mainQueue() -> Id<NSOperationQueue, Shared> {
        msg_send_id![Self::class(), mainQueue]
    }
}
#[doc = "NSDeprecated"]
impl NSOperationQueue {
    pub unsafe fn operations(&self) -> Id<NSArray<NSOperation>, Shared> {
        msg_send_id![self, operations]
    }
    pub unsafe fn operationCount(&self) -> NSUInteger {
        msg_send![self, operationCount]
    }
}
