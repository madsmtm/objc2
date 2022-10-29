#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSColorPicker;
    unsafe impl ClassType for NSColorPicker {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSColorPicker {
        #[method_id(initWithPickerMask:colorPanel:)]
        pub unsafe fn initWithPickerMask_colorPanel(
            &self,
            mask: NSUInteger,
            owningColorPanel: &NSColorPanel,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(colorPanel)]
        pub unsafe fn colorPanel(&self) -> Id<NSColorPanel, Shared>;
        #[method_id(provideNewButtonImage)]
        pub unsafe fn provideNewButtonImage(&self) -> Id<NSImage, Shared>;
        #[method(insertNewButtonImage:in:)]
        pub unsafe fn insertNewButtonImage_in(
            &self,
            newButtonImage: &NSImage,
            buttonCell: &NSButtonCell,
        );
        #[method(viewSizeChanged:)]
        pub unsafe fn viewSizeChanged(&self, sender: Option<&Object>);
        #[method(attachColorList:)]
        pub unsafe fn attachColorList(&self, colorList: &NSColorList);
        #[method(detachColorList:)]
        pub unsafe fn detachColorList(&self, colorList: &NSColorList);
        #[method(setMode:)]
        pub unsafe fn setMode(&self, mode: NSColorPanelMode);
        #[method_id(buttonToolTip)]
        pub unsafe fn buttonToolTip(&self) -> Id<NSString, Shared>;
        #[method(minContentSize)]
        pub unsafe fn minContentSize(&self) -> NSSize;
    }
);
