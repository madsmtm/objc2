use super::__exported::NSColorList;
use super::__exported::NSMutableArray;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSApplication::*;
use crate::AppKit::generated::NSPanel::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSColorPanel;
    unsafe impl ClassType for NSColorPanel {
        type Super = NSPanel;
    }
);
extern_methods!(
    unsafe impl NSColorPanel {
        #[method_id(sharedColorPanel)]
        pub unsafe fn sharedColorPanel() -> Id<NSColorPanel, Shared>;
        #[method(sharedColorPanelExists)]
        pub unsafe fn sharedColorPanelExists() -> bool;
        #[method(dragColor:withEvent:fromView:)]
        pub unsafe fn dragColor_withEvent_fromView(
            color: &NSColor,
            event: &NSEvent,
            sourceView: &NSView,
        ) -> bool;
        #[method(setPickerMask:)]
        pub unsafe fn setPickerMask(mask: NSColorPanelOptions);
        #[method(setPickerMode:)]
        pub unsafe fn setPickerMode(mode: NSColorPanelMode);
        #[method_id(accessoryView)]
        pub unsafe fn accessoryView(&self) -> Option<Id<NSView, Shared>>;
        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, accessoryView: Option<&NSView>);
        #[method(isContinuous)]
        pub unsafe fn isContinuous(&self) -> bool;
        #[method(setContinuous:)]
        pub unsafe fn setContinuous(&self, continuous: bool);
        #[method(showsAlpha)]
        pub unsafe fn showsAlpha(&self) -> bool;
        #[method(setShowsAlpha:)]
        pub unsafe fn setShowsAlpha(&self, showsAlpha: bool);
        #[method(mode)]
        pub unsafe fn mode(&self) -> NSColorPanelMode;
        #[method(setMode:)]
        pub unsafe fn setMode(&self, mode: NSColorPanelMode);
        #[method_id(color)]
        pub unsafe fn color(&self) -> Id<NSColor, Shared>;
        #[method(setColor:)]
        pub unsafe fn setColor(&self, color: &NSColor);
        #[method(alpha)]
        pub unsafe fn alpha(&self) -> CGFloat;
        #[method(setAction:)]
        pub unsafe fn setAction(&self, selector: Option<Sel>);
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&Object>);
        #[method(attachColorList:)]
        pub unsafe fn attachColorList(&self, colorList: &NSColorList);
        #[method(detachColorList:)]
        pub unsafe fn detachColorList(&self, colorList: &NSColorList);
    }
);
extern_methods!(
    #[doc = "NSColorPanel"]
    unsafe impl NSApplication {
        #[method(orderFrontColorPanel:)]
        pub unsafe fn orderFrontColorPanel(&self, sender: Option<&Object>);
    }
);
pub type NSColorChanging = NSObject;
extern_methods!(
    #[doc = "NSColorPanelResponderMethod"]
    unsafe impl NSObject {
        #[method(changeColor:)]
        pub unsafe fn changeColor(&self, sender: Option<&Object>);
    }
);
