use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSCell::*;
use crate::AppKit::generated::NSView::*;
use crate::Foundation::generated::Foundation::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSProgressIndicator;
    unsafe impl ClassType for NSProgressIndicator {
        type Super = NSView;
    }
);
extern_methods!(
    unsafe impl NSProgressIndicator {
        #[method(isIndeterminate)]
        pub unsafe fn isIndeterminate(&self) -> bool;
        #[method(setIndeterminate:)]
        pub unsafe fn setIndeterminate(&self, indeterminate: bool);
        #[method(isBezeled)]
        pub unsafe fn isBezeled(&self) -> bool;
        #[method(setBezeled:)]
        pub unsafe fn setBezeled(&self, bezeled: bool);
        #[method(controlTint)]
        pub unsafe fn controlTint(&self) -> NSControlTint;
        #[method(setControlTint:)]
        pub unsafe fn setControlTint(&self, controlTint: NSControlTint);
        #[method(controlSize)]
        pub unsafe fn controlSize(&self) -> NSControlSize;
        #[method(setControlSize:)]
        pub unsafe fn setControlSize(&self, controlSize: NSControlSize);
        #[method(doubleValue)]
        pub unsafe fn doubleValue(&self) -> c_double;
        #[method(setDoubleValue:)]
        pub unsafe fn setDoubleValue(&self, doubleValue: c_double);
        #[method(incrementBy:)]
        pub unsafe fn incrementBy(&self, delta: c_double);
        #[method(minValue)]
        pub unsafe fn minValue(&self) -> c_double;
        #[method(setMinValue:)]
        pub unsafe fn setMinValue(&self, minValue: c_double);
        #[method(maxValue)]
        pub unsafe fn maxValue(&self) -> c_double;
        #[method(setMaxValue:)]
        pub unsafe fn setMaxValue(&self, maxValue: c_double);
        #[method(usesThreadedAnimation)]
        pub unsafe fn usesThreadedAnimation(&self) -> bool;
        #[method(setUsesThreadedAnimation:)]
        pub unsafe fn setUsesThreadedAnimation(&self, usesThreadedAnimation: bool);
        #[method(startAnimation:)]
        pub unsafe fn startAnimation(&self, sender: Option<&Object>);
        #[method(stopAnimation:)]
        pub unsafe fn stopAnimation(&self, sender: Option<&Object>);
        #[method(style)]
        pub unsafe fn style(&self) -> NSProgressIndicatorStyle;
        #[method(setStyle:)]
        pub unsafe fn setStyle(&self, style: NSProgressIndicatorStyle);
        #[method(sizeToFit)]
        pub unsafe fn sizeToFit(&self);
        #[method(isDisplayedWhenStopped)]
        pub unsafe fn isDisplayedWhenStopped(&self) -> bool;
        #[method(setDisplayedWhenStopped:)]
        pub unsafe fn setDisplayedWhenStopped(&self, displayedWhenStopped: bool);
    }
);
extern_methods!(
    #[doc = "NSProgressIndicatorDeprecated"]
    unsafe impl NSProgressIndicator {
        #[method(animationDelay)]
        pub unsafe fn animationDelay(&self) -> NSTimeInterval;
        #[method(setAnimationDelay:)]
        pub unsafe fn setAnimationDelay(&self, delay: NSTimeInterval);
        #[method(animate:)]
        pub unsafe fn animate(&self, sender: Option<&Object>);
    }
);
