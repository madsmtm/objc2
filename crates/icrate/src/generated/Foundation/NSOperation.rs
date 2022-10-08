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
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSOperation;
    unsafe impl ClassType for NSOperation {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSOperation {
        #[method(start)]
        pub unsafe fn start(&self);
        #[method(main)]
        pub unsafe fn main(&self);
        #[method(isCancelled)]
        pub unsafe fn isCancelled(&self) -> bool;
        #[method(cancel)]
        pub unsafe fn cancel(&self);
        #[method(isExecuting)]
        pub unsafe fn isExecuting(&self) -> bool;
        #[method(isFinished)]
        pub unsafe fn isFinished(&self) -> bool;
        #[method(isConcurrent)]
        pub unsafe fn isConcurrent(&self) -> bool;
        #[method(isAsynchronous)]
        pub unsafe fn isAsynchronous(&self) -> bool;
        #[method(isReady)]
        pub unsafe fn isReady(&self) -> bool;
        # [method (addDependency :)]
        pub unsafe fn addDependency(&self, op: &NSOperation);
        # [method (removeDependency :)]
        pub unsafe fn removeDependency(&self, op: &NSOperation);
        #[method_id(dependencies)]
        pub unsafe fn dependencies(&self) -> Id<NSArray<NSOperation>, Shared>;
        #[method(queuePriority)]
        pub unsafe fn queuePriority(&self) -> NSOperationQueuePriority;
        # [method (setQueuePriority :)]
        pub unsafe fn setQueuePriority(&self, queuePriority: NSOperationQueuePriority);
        #[method(completionBlock)]
        pub unsafe fn completionBlock(&self) -> TodoBlock;
        # [method (setCompletionBlock :)]
        pub unsafe fn setCompletionBlock(&self, completionBlock: TodoBlock);
        #[method(waitUntilFinished)]
        pub unsafe fn waitUntilFinished(&self);
        #[method(threadPriority)]
        pub unsafe fn threadPriority(&self) -> c_double;
        # [method (setThreadPriority :)]
        pub unsafe fn setThreadPriority(&self, threadPriority: c_double);
        #[method(qualityOfService)]
        pub unsafe fn qualityOfService(&self) -> NSQualityOfService;
        # [method (setQualityOfService :)]
        pub unsafe fn setQualityOfService(&self, qualityOfService: NSQualityOfService);
        #[method_id(name)]
        pub unsafe fn name(&self) -> Option<Id<NSString, Shared>>;
        # [method (setName :)]
        pub unsafe fn setName(&self, name: Option<&NSString>);
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSBlockOperation;
    unsafe impl ClassType for NSBlockOperation {
        type Super = NSOperation;
    }
);
extern_methods!(
    unsafe impl NSBlockOperation {
        # [method_id (blockOperationWithBlock :)]
        pub unsafe fn blockOperationWithBlock(block: TodoBlock) -> Id<Self, Shared>;
        # [method (addExecutionBlock :)]
        pub unsafe fn addExecutionBlock(&self, block: TodoBlock);
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSInvocationOperation;
    unsafe impl ClassType for NSInvocationOperation {
        type Super = NSOperation;
    }
);
extern_methods!(
    unsafe impl NSInvocationOperation {
        # [method_id (initWithTarget : selector : object :)]
        pub unsafe fn initWithTarget_selector_object(
            &self,
            target: &Object,
            sel: Sel,
            arg: Option<&Object>,
        ) -> Option<Id<Self, Shared>>;
        # [method_id (initWithInvocation :)]
        pub unsafe fn initWithInvocation(&self, inv: &NSInvocation) -> Id<Self, Shared>;
        #[method_id(invocation)]
        pub unsafe fn invocation(&self) -> Id<NSInvocation, Shared>;
        #[method_id(result)]
        pub unsafe fn result(&self) -> Option<Id<Object, Shared>>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSOperationQueue;
    unsafe impl ClassType for NSOperationQueue {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSOperationQueue {
        #[method_id(progress)]
        pub unsafe fn progress(&self) -> Id<NSProgress, Shared>;
        # [method (addOperation :)]
        pub unsafe fn addOperation(&self, op: &NSOperation);
        # [method (addOperations : waitUntilFinished :)]
        pub unsafe fn addOperations_waitUntilFinished(
            &self,
            ops: &NSArray<NSOperation>,
            wait: bool,
        );
        # [method (addOperationWithBlock :)]
        pub unsafe fn addOperationWithBlock(&self, block: TodoBlock);
        # [method (addBarrierBlock :)]
        pub unsafe fn addBarrierBlock(&self, barrier: TodoBlock);
        #[method(maxConcurrentOperationCount)]
        pub unsafe fn maxConcurrentOperationCount(&self) -> NSInteger;
        # [method (setMaxConcurrentOperationCount :)]
        pub unsafe fn setMaxConcurrentOperationCount(&self, maxConcurrentOperationCount: NSInteger);
        #[method(isSuspended)]
        pub unsafe fn isSuspended(&self) -> bool;
        # [method (setSuspended :)]
        pub unsafe fn setSuspended(&self, suspended: bool);
        #[method_id(name)]
        pub unsafe fn name(&self) -> Option<Id<NSString, Shared>>;
        # [method (setName :)]
        pub unsafe fn setName(&self, name: Option<&NSString>);
        #[method(qualityOfService)]
        pub unsafe fn qualityOfService(&self) -> NSQualityOfService;
        # [method (setQualityOfService :)]
        pub unsafe fn setQualityOfService(&self, qualityOfService: NSQualityOfService);
        #[method_id(underlyingQueue)]
        pub unsafe fn underlyingQueue(&self) -> Option<Id<dispatch_queue_t, Shared>>;
        # [method (setUnderlyingQueue :)]
        pub unsafe fn setUnderlyingQueue(&self, underlyingQueue: Option<&dispatch_queue_t>);
        #[method(cancelAllOperations)]
        pub unsafe fn cancelAllOperations(&self);
        #[method(waitUntilAllOperationsAreFinished)]
        pub unsafe fn waitUntilAllOperationsAreFinished(&self);
        #[method_id(currentQueue)]
        pub unsafe fn currentQueue() -> Option<Id<NSOperationQueue, Shared>>;
        #[method_id(mainQueue)]
        pub unsafe fn mainQueue() -> Id<NSOperationQueue, Shared>;
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSOperationQueue {
        #[method_id(operations)]
        pub unsafe fn operations(&self) -> Id<NSArray<NSOperation>, Shared>;
        #[method(operationCount)]
        pub unsafe fn operationCount(&self) -> NSUInteger;
    }
);
