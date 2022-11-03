//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSBackgroundActivityResult {
        NSBackgroundActivityResultFinished = 1,
        NSBackgroundActivityResultDeferred = 2,
    }
);

pub type NSBackgroundActivityCompletionHandler = *mut Block<(NSBackgroundActivityResult,), ()>;

extern_class!(
    #[derive(Debug)]
    pub struct NSBackgroundActivityScheduler;

    unsafe impl ClassType for NSBackgroundActivityScheduler {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSBackgroundActivityScheduler {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Option<Allocated<Self>>,
            identifier: &NSString,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString, Shared>;

        #[method(qualityOfService)]
        pub unsafe fn qualityOfService(&self) -> NSQualityOfService;

        #[method(setQualityOfService:)]
        pub unsafe fn setQualityOfService(&self, qualityOfService: NSQualityOfService);

        #[method(repeats)]
        pub unsafe fn repeats(&self) -> bool;

        #[method(setRepeats:)]
        pub unsafe fn setRepeats(&self, repeats: bool);

        #[method(interval)]
        pub unsafe fn interval(&self) -> NSTimeInterval;

        #[method(setInterval:)]
        pub unsafe fn setInterval(&self, interval: NSTimeInterval);

        #[method(tolerance)]
        pub unsafe fn tolerance(&self) -> NSTimeInterval;

        #[method(setTolerance:)]
        pub unsafe fn setTolerance(&self, tolerance: NSTimeInterval);

        #[method(scheduleWithBlock:)]
        pub unsafe fn scheduleWithBlock(
            &self,
            block: &Block<(NSBackgroundActivityCompletionHandler,), ()>,
        );

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);

        #[method(shouldDefer)]
        pub unsafe fn shouldDefer(&self) -> bool;
    }
);
