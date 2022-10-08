use super::__exported::NSArray;
use super::__exported::NSDate;
use super::__exported::NSMutableDictionary;
use super::__exported::NSNumber;
use super::__exported::NSString;
use crate::Foundation::generated::NSDate::*;
use crate::Foundation::generated::NSNotification::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSThread;
    unsafe impl ClassType for NSThread {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSThread {
        #[method_id(currentThread)]
        pub unsafe fn currentThread() -> Id<NSThread, Shared>;
        # [method (detachNewThreadWithBlock :)]
        pub unsafe fn detachNewThreadWithBlock(block: TodoBlock);
        # [method (detachNewThreadSelector : toTarget : withObject :)]
        pub unsafe fn detachNewThreadSelector_toTarget_withObject(
            selector: Sel,
            target: &Object,
            argument: Option<&Object>,
        );
        #[method(isMultiThreaded)]
        pub unsafe fn isMultiThreaded() -> bool;
        #[method_id(threadDictionary)]
        pub unsafe fn threadDictionary(&self) -> Id<NSMutableDictionary, Shared>;
        # [method (sleepUntilDate :)]
        pub unsafe fn sleepUntilDate(date: &NSDate);
        # [method (sleepForTimeInterval :)]
        pub unsafe fn sleepForTimeInterval(ti: NSTimeInterval);
        #[method(exit)]
        pub unsafe fn exit();
        #[method(threadPriority)]
        pub unsafe fn threadPriority() -> c_double;
        # [method (setThreadPriority :)]
        pub unsafe fn setThreadPriority(p: c_double) -> bool;
        #[method(threadPriority)]
        pub unsafe fn threadPriority(&self) -> c_double;
        # [method (setThreadPriority :)]
        pub unsafe fn setThreadPriority(&self, threadPriority: c_double);
        #[method(qualityOfService)]
        pub unsafe fn qualityOfService(&self) -> NSQualityOfService;
        # [method (setQualityOfService :)]
        pub unsafe fn setQualityOfService(&self, qualityOfService: NSQualityOfService);
        #[method_id(callStackReturnAddresses)]
        pub unsafe fn callStackReturnAddresses() -> Id<NSArray<NSNumber>, Shared>;
        #[method_id(callStackSymbols)]
        pub unsafe fn callStackSymbols() -> Id<NSArray<NSString>, Shared>;
        #[method_id(name)]
        pub unsafe fn name(&self) -> Option<Id<NSString, Shared>>;
        # [method (setName :)]
        pub unsafe fn setName(&self, name: Option<&NSString>);
        #[method(stackSize)]
        pub unsafe fn stackSize(&self) -> NSUInteger;
        # [method (setStackSize :)]
        pub unsafe fn setStackSize(&self, stackSize: NSUInteger);
        #[method(isMainThread)]
        pub unsafe fn isMainThread(&self) -> bool;
        #[method(isMainThread)]
        pub unsafe fn isMainThread() -> bool;
        #[method_id(mainThread)]
        pub unsafe fn mainThread() -> Id<NSThread, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        # [method_id (initWithTarget : selector : object :)]
        pub unsafe fn initWithTarget_selector_object(
            &self,
            target: &Object,
            selector: Sel,
            argument: Option<&Object>,
        ) -> Id<Self, Shared>;
        # [method_id (initWithBlock :)]
        pub unsafe fn initWithBlock(&self, block: TodoBlock) -> Id<Self, Shared>;
        #[method(isExecuting)]
        pub unsafe fn isExecuting(&self) -> bool;
        #[method(isFinished)]
        pub unsafe fn isFinished(&self) -> bool;
        #[method(isCancelled)]
        pub unsafe fn isCancelled(&self) -> bool;
        #[method(cancel)]
        pub unsafe fn cancel(&self);
        #[method(start)]
        pub unsafe fn start(&self);
        #[method(main)]
        pub unsafe fn main(&self);
    }
);
extern_methods!(
    #[doc = "NSThreadPerformAdditions"]
    unsafe impl NSObject {
        # [method (performSelectorOnMainThread : withObject : waitUntilDone : modes :)]
        pub unsafe fn performSelectorOnMainThread_withObject_waitUntilDone_modes(
            &self,
            aSelector: Sel,
            arg: Option<&Object>,
            wait: bool,
            array: Option<&NSArray<NSString>>,
        );
        # [method (performSelectorOnMainThread : withObject : waitUntilDone :)]
        pub unsafe fn performSelectorOnMainThread_withObject_waitUntilDone(
            &self,
            aSelector: Sel,
            arg: Option<&Object>,
            wait: bool,
        );
        # [method (performSelector : onThread : withObject : waitUntilDone : modes :)]
        pub unsafe fn performSelector_onThread_withObject_waitUntilDone_modes(
            &self,
            aSelector: Sel,
            thr: &NSThread,
            arg: Option<&Object>,
            wait: bool,
            array: Option<&NSArray<NSString>>,
        );
        # [method (performSelector : onThread : withObject : waitUntilDone :)]
        pub unsafe fn performSelector_onThread_withObject_waitUntilDone(
            &self,
            aSelector: Sel,
            thr: &NSThread,
            arg: Option<&Object>,
            wait: bool,
        );
        # [method (performSelectorInBackground : withObject :)]
        pub unsafe fn performSelectorInBackground_withObject(
            &self,
            aSelector: Sel,
            arg: Option<&Object>,
        );
    }
);
