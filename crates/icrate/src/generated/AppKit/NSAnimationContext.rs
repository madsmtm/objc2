#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSAnimationContext;
    unsafe impl ClassType for NSAnimationContext {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSAnimationContext {
        #[method(runAnimationGroup:completionHandler:)]
        pub unsafe fn runAnimationGroup_completionHandler(
            changes: TodoBlock,
            completionHandler: TodoBlock,
        );
        #[method(runAnimationGroup:)]
        pub unsafe fn runAnimationGroup(changes: TodoBlock);
        #[method(beginGrouping)]
        pub unsafe fn beginGrouping();
        #[method(endGrouping)]
        pub unsafe fn endGrouping();
        #[method_id(currentContext)]
        pub unsafe fn currentContext() -> Id<NSAnimationContext, Shared>;
        #[method(duration)]
        pub unsafe fn duration(&self) -> NSTimeInterval;
        #[method(setDuration:)]
        pub unsafe fn setDuration(&self, duration: NSTimeInterval);
        #[method_id(timingFunction)]
        pub unsafe fn timingFunction(&self) -> Option<Id<CAMediaTimingFunction, Shared>>;
        #[method(setTimingFunction:)]
        pub unsafe fn setTimingFunction(&self, timingFunction: Option<&CAMediaTimingFunction>);
        #[method(completionHandler)]
        pub unsafe fn completionHandler(&self) -> TodoBlock;
        #[method(setCompletionHandler:)]
        pub unsafe fn setCompletionHandler(&self, completionHandler: TodoBlock);
        #[method(allowsImplicitAnimation)]
        pub unsafe fn allowsImplicitAnimation(&self) -> bool;
        #[method(setAllowsImplicitAnimation:)]
        pub unsafe fn setAllowsImplicitAnimation(&self, allowsImplicitAnimation: bool);
    }
);
