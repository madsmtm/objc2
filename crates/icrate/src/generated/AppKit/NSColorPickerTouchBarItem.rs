use super::__exported::NSColor;
use super::__exported::NSColorList;
use super::__exported::NSColorSpace;
use super::__exported::NSImage;
use super::__exported::NSString;
use super::__exported::NSViewController;
use crate::AppKit::generated::NSTouchBarItem::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSColorPickerTouchBarItem;
    unsafe impl ClassType for NSColorPickerTouchBarItem {
        type Super = NSTouchBarItem;
    }
);
extern_methods!(
    unsafe impl NSColorPickerTouchBarItem {
        #[method_id(colorPickerWithIdentifier:)]
        pub unsafe fn colorPickerWithIdentifier(
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self, Shared>;
        #[method_id(textColorPickerWithIdentifier:)]
        pub unsafe fn textColorPickerWithIdentifier(
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self, Shared>;
        #[method_id(strokeColorPickerWithIdentifier:)]
        pub unsafe fn strokeColorPickerWithIdentifier(
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self, Shared>;
        #[method_id(colorPickerWithIdentifier:buttonImage:)]
        pub unsafe fn colorPickerWithIdentifier_buttonImage(
            identifier: &NSTouchBarItemIdentifier,
            image: &NSImage,
        ) -> Id<Self, Shared>;
        #[method_id(color)]
        pub unsafe fn color(&self) -> Id<NSColor, Shared>;
        #[method(setColor:)]
        pub unsafe fn setColor(&self, color: &NSColor);
        #[method(showsAlpha)]
        pub unsafe fn showsAlpha(&self) -> bool;
        #[method(setShowsAlpha:)]
        pub unsafe fn setShowsAlpha(&self, showsAlpha: bool);
        #[method_id(allowedColorSpaces)]
        pub unsafe fn allowedColorSpaces(&self) -> Option<Id<NSArray<NSColorSpace>, Shared>>;
        #[method(setAllowedColorSpaces:)]
        pub unsafe fn setAllowedColorSpaces(
            &self,
            allowedColorSpaces: Option<&NSArray<NSColorSpace>>,
        );
        #[method_id(colorList)]
        pub unsafe fn colorList(&self) -> Option<Id<NSColorList, Shared>>;
        #[method(setColorList:)]
        pub unsafe fn setColorList(&self, colorList: Option<&NSColorList>);
        #[method_id(customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Id<NSString, Shared>;
        #[method(setCustomizationLabel:)]
        pub unsafe fn setCustomizationLabel(&self, customizationLabel: Option<&NSString>);
        #[method_id(target)]
        pub unsafe fn target(&self) -> Option<Id<Object, Shared>>;
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&Object>);
        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;
        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);
        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;
        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);
    }
);
