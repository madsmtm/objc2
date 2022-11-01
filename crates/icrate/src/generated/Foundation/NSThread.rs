//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

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

        #[method(detachNewThreadWithBlock:)]
        pub unsafe fn detachNewThreadWithBlock(block: TodoBlock);

        #[method(detachNewThreadSelector:toTarget:withObject:)]
        pub unsafe fn detachNewThreadSelector_toTarget_withObject(
            selector: Sel,
            target: &Object,
            argument: Option<&Object>,
        );

        #[method(isMultiThreaded)]
        pub unsafe fn isMultiThreaded() -> bool;

        #[method_id(threadDictionary)]
        pub unsafe fn threadDictionary(&self) -> Id<NSMutableDictionary, Shared>;

        #[method(sleepUntilDate:)]
        pub unsafe fn sleepUntilDate(date: &NSDate);

        #[method(sleepForTimeInterval:)]
        pub unsafe fn sleepForTimeInterval(ti: NSTimeInterval);

        #[method(exit)]
        pub unsafe fn exit();

        #[method(threadPriority)]
        pub unsafe fn threadPriority() -> c_double;

        #[method(setThreadPriority:)]
        pub unsafe fn setThreadPriority(p: c_double) -> bool;

        #[method(qualityOfService)]
        pub unsafe fn qualityOfService(&self) -> NSQualityOfService;

        #[method(setQualityOfService:)]
        pub unsafe fn setQualityOfService(&self, qualityOfService: NSQualityOfService);

        #[method_id(callStackReturnAddresses)]
        pub unsafe fn callStackReturnAddresses() -> Id<NSArray<NSNumber>, Shared>;

        #[method_id(callStackSymbols)]
        pub unsafe fn callStackSymbols() -> Id<NSArray<NSString>, Shared>;

        #[method_id(name)]
        pub unsafe fn name(&self) -> Option<Id<NSString, Shared>>;

        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[method(stackSize)]
        pub unsafe fn stackSize(&self) -> NSUInteger;

        #[method(setStackSize:)]
        pub unsafe fn setStackSize(&self, stackSize: NSUInteger);

        #[method_id(mainThread)]
        pub unsafe fn mainThread() -> Id<NSThread, Shared>;

        #[method_id(init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(initWithTarget:selector:object:)]
        pub unsafe fn initWithTarget_selector_object(
            this: Option<Allocated<Self>>,
            target: &Object,
            selector: Sel,
            argument: Option<&Object>,
        ) -> Id<Self, Shared>;

        #[method_id(initWithBlock:)]
        pub unsafe fn initWithBlock(
            this: Option<Allocated<Self>>,
            block: TodoBlock,
        ) -> Id<Self, Shared>;

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

extern "C" {
    pub static NSWillBecomeMultiThreadedNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSDidBecomeSingleThreadedNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSThreadWillExitNotification: &'static NSNotificationName;
}

extern_methods!(
    /// NSThreadPerformAdditions
    unsafe impl NSObject {
        #[method(performSelectorOnMainThread:withObject:waitUntilDone:modes:)]
        pub unsafe fn performSelectorOnMainThread_withObject_waitUntilDone_modes(
            &self,
            aSelector: Sel,
            arg: Option<&Object>,
            wait: bool,
            array: Option<&NSArray<NSString>>,
        );

        #[method(performSelectorOnMainThread:withObject:waitUntilDone:)]
        pub unsafe fn performSelectorOnMainThread_withObject_waitUntilDone(
            &self,
            aSelector: Sel,
            arg: Option<&Object>,
            wait: bool,
        );

        #[method(performSelector:onThread:withObject:waitUntilDone:modes:)]
        pub unsafe fn performSelector_onThread_withObject_waitUntilDone_modes(
            &self,
            aSelector: Sel,
            thr: &NSThread,
            arg: Option<&Object>,
            wait: bool,
            array: Option<&NSArray<NSString>>,
        );

        #[method(performSelector:onThread:withObject:waitUntilDone:)]
        pub unsafe fn performSelector_onThread_withObject_waitUntilDone(
            &self,
            aSelector: Sel,
            thr: &NSThread,
            arg: Option<&Object>,
            wait: bool,
        );

        #[method(performSelectorInBackground:withObject:)]
        pub unsafe fn performSelectorInBackground_withObject(
            &self,
            aSelector: Sel,
            arg: Option<&Object>,
        );
    }
);
