use super::__exported::NSImage;
use super::__exported::NSSlider;
use crate::AppKit::generated::NSAccessibility::*;
use crate::Foundation::generated::Foundation::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSSliderAccessory;
    unsafe impl ClassType for NSSliderAccessory {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSSliderAccessory {
        #[method_id(accessoryWithImage:)]
        pub unsafe fn accessoryWithImage(image: &NSImage) -> Id<NSSliderAccessory, Shared>;
        #[method_id(behavior)]
        pub unsafe fn behavior(&self) -> Id<NSSliderAccessoryBehavior, Shared>;
        #[method(setBehavior:)]
        pub unsafe fn setBehavior(&self, behavior: &NSSliderAccessoryBehavior);
        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;
        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);
    }
);
extern_methods!(
    unsafe impl NSSliderAccessory {}
);
extern_class!(
    #[derive(Debug)]
    pub struct NSSliderAccessoryBehavior;
    unsafe impl ClassType for NSSliderAccessoryBehavior {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSSliderAccessoryBehavior {
        #[method_id(automaticBehavior)]
        pub unsafe fn automaticBehavior() -> Id<NSSliderAccessoryBehavior, Shared>;
        #[method_id(valueStepBehavior)]
        pub unsafe fn valueStepBehavior() -> Id<NSSliderAccessoryBehavior, Shared>;
        #[method_id(valueResetBehavior)]
        pub unsafe fn valueResetBehavior() -> Id<NSSliderAccessoryBehavior, Shared>;
        #[method_id(behaviorWithTarget:action:)]
        pub unsafe fn behaviorWithTarget_action(
            target: Option<&Object>,
            action: Sel,
        ) -> Id<NSSliderAccessoryBehavior, Shared>;
        #[method_id(behaviorWithHandler:)]
        pub unsafe fn behaviorWithHandler(
            handler: TodoBlock,
        ) -> Id<NSSliderAccessoryBehavior, Shared>;
        #[method(handleAction:)]
        pub unsafe fn handleAction(&self, sender: &NSSliderAccessory);
    }
);
