#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSBackgroundActivityScheduler;
    unsafe impl ClassType for NSBackgroundActivityScheduler {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSBackgroundActivityScheduler {
        #[method_id(initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(&self, identifier: &NSString) -> Id<Self, Shared>;
        #[method_id(identifier)]
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
        pub unsafe fn scheduleWithBlock(&self, block: TodoBlock);
        #[method(invalidate)]
        pub unsafe fn invalidate(&self);
        #[method(shouldDefer)]
        pub unsafe fn shouldDefer(&self) -> bool;
    }
);
