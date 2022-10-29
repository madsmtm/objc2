#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSAnimation;
    unsafe impl ClassType for NSAnimation {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSAnimation {
        #[method_id(initWithDuration:animationCurve:)]
        pub unsafe fn initWithDuration_animationCurve(
            &self,
            duration: NSTimeInterval,
            animationCurve: NSAnimationCurve,
        ) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method(startAnimation)]
        pub unsafe fn startAnimation(&self);
        #[method(stopAnimation)]
        pub unsafe fn stopAnimation(&self);
        #[method(isAnimating)]
        pub unsafe fn isAnimating(&self) -> bool;
        #[method(currentProgress)]
        pub unsafe fn currentProgress(&self) -> NSAnimationProgress;
        #[method(setCurrentProgress:)]
        pub unsafe fn setCurrentProgress(&self, currentProgress: NSAnimationProgress);
        #[method(duration)]
        pub unsafe fn duration(&self) -> NSTimeInterval;
        #[method(setDuration:)]
        pub unsafe fn setDuration(&self, duration: NSTimeInterval);
        #[method(animationBlockingMode)]
        pub unsafe fn animationBlockingMode(&self) -> NSAnimationBlockingMode;
        #[method(setAnimationBlockingMode:)]
        pub unsafe fn setAnimationBlockingMode(
            &self,
            animationBlockingMode: NSAnimationBlockingMode,
        );
        #[method(frameRate)]
        pub unsafe fn frameRate(&self) -> c_float;
        #[method(setFrameRate:)]
        pub unsafe fn setFrameRate(&self, frameRate: c_float);
        #[method(animationCurve)]
        pub unsafe fn animationCurve(&self) -> NSAnimationCurve;
        #[method(setAnimationCurve:)]
        pub unsafe fn setAnimationCurve(&self, animationCurve: NSAnimationCurve);
        #[method(currentValue)]
        pub unsafe fn currentValue(&self) -> c_float;
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSAnimationDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSAnimationDelegate>);
        #[method_id(progressMarks)]
        pub unsafe fn progressMarks(&self) -> Id<NSArray<NSNumber>, Shared>;
        #[method(setProgressMarks:)]
        pub unsafe fn setProgressMarks(&self, progressMarks: &NSArray<NSNumber>);
        #[method(addProgressMark:)]
        pub unsafe fn addProgressMark(&self, progressMark: NSAnimationProgress);
        #[method(removeProgressMark:)]
        pub unsafe fn removeProgressMark(&self, progressMark: NSAnimationProgress);
        #[method(startWhenAnimation:reachesProgress:)]
        pub unsafe fn startWhenAnimation_reachesProgress(
            &self,
            animation: &NSAnimation,
            startProgress: NSAnimationProgress,
        );
        #[method(stopWhenAnimation:reachesProgress:)]
        pub unsafe fn stopWhenAnimation_reachesProgress(
            &self,
            animation: &NSAnimation,
            stopProgress: NSAnimationProgress,
        );
        #[method(clearStartAnimation)]
        pub unsafe fn clearStartAnimation(&self);
        #[method(clearStopAnimation)]
        pub unsafe fn clearStopAnimation(&self);
        #[method_id(runLoopModesForAnimating)]
        pub unsafe fn runLoopModesForAnimating(&self)
            -> Option<Id<NSArray<NSRunLoopMode>, Shared>>;
    }
);
pub type NSAnimationDelegate = NSObject;
pub type NSViewAnimationKey = NSString;
pub type NSViewAnimationEffectName = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSViewAnimation;
    unsafe impl ClassType for NSViewAnimation {
        type Super = NSAnimation;
    }
);
extern_methods!(
    unsafe impl NSViewAnimation {
        #[method_id(initWithViewAnimations:)]
        pub unsafe fn initWithViewAnimations(
            &self,
            viewAnimations: &NSArray<NSDictionary<NSViewAnimationKey, Object>>,
        ) -> Id<Self, Shared>;
        #[method_id(viewAnimations)]
        pub unsafe fn viewAnimations(
            &self,
        ) -> Id<NSArray<NSDictionary<NSViewAnimationKey, Object>>, Shared>;
        #[method(setViewAnimations:)]
        pub unsafe fn setViewAnimations(
            &self,
            viewAnimations: &NSArray<NSDictionary<NSViewAnimationKey, Object>>,
        );
    }
);
pub type NSAnimatablePropertyKey = NSString;
pub type NSAnimatablePropertyContainer = NSObject;
