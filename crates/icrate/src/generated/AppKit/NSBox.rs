#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSBox;
    unsafe impl ClassType for NSBox {
        type Super = NSView;
    }
);
extern_methods!(
    unsafe impl NSBox {
        #[method(boxType)]
        pub unsafe fn boxType(&self) -> NSBoxType;
        #[method(setBoxType:)]
        pub unsafe fn setBoxType(&self, boxType: NSBoxType);
        #[method(titlePosition)]
        pub unsafe fn titlePosition(&self) -> NSTitlePosition;
        #[method(setTitlePosition:)]
        pub unsafe fn setTitlePosition(&self, titlePosition: NSTitlePosition);
        #[method_id(title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);
        #[method_id(titleFont)]
        pub unsafe fn titleFont(&self) -> Id<NSFont, Shared>;
        #[method(setTitleFont:)]
        pub unsafe fn setTitleFont(&self, titleFont: &NSFont);
        #[method(borderRect)]
        pub unsafe fn borderRect(&self) -> NSRect;
        #[method(titleRect)]
        pub unsafe fn titleRect(&self) -> NSRect;
        #[method_id(titleCell)]
        pub unsafe fn titleCell(&self) -> Id<Object, Shared>;
        #[method(contentViewMargins)]
        pub unsafe fn contentViewMargins(&self) -> NSSize;
        #[method(setContentViewMargins:)]
        pub unsafe fn setContentViewMargins(&self, contentViewMargins: NSSize);
        #[method(sizeToFit)]
        pub unsafe fn sizeToFit(&self);
        #[method(setFrameFromContentFrame:)]
        pub unsafe fn setFrameFromContentFrame(&self, contentFrame: NSRect);
        #[method_id(contentView)]
        pub unsafe fn contentView(&self) -> Option<Id<NSView, Shared>>;
        #[method(setContentView:)]
        pub unsafe fn setContentView(&self, contentView: Option<&NSView>);
        #[method(isTransparent)]
        pub unsafe fn isTransparent(&self) -> bool;
        #[method(setTransparent:)]
        pub unsafe fn setTransparent(&self, transparent: bool);
        #[method(borderWidth)]
        pub unsafe fn borderWidth(&self) -> CGFloat;
        #[method(setBorderWidth:)]
        pub unsafe fn setBorderWidth(&self, borderWidth: CGFloat);
        #[method(cornerRadius)]
        pub unsafe fn cornerRadius(&self) -> CGFloat;
        #[method(setCornerRadius:)]
        pub unsafe fn setCornerRadius(&self, cornerRadius: CGFloat);
        #[method_id(borderColor)]
        pub unsafe fn borderColor(&self) -> Id<NSColor, Shared>;
        #[method(setBorderColor:)]
        pub unsafe fn setBorderColor(&self, borderColor: &NSColor);
        #[method_id(fillColor)]
        pub unsafe fn fillColor(&self) -> Id<NSColor, Shared>;
        #[method(setFillColor:)]
        pub unsafe fn setFillColor(&self, fillColor: &NSColor);
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSBox {
        #[method(borderType)]
        pub unsafe fn borderType(&self) -> NSBorderType;
        #[method(setBorderType:)]
        pub unsafe fn setBorderType(&self, borderType: NSBorderType);
        #[method(setTitleWithMnemonic:)]
        pub unsafe fn setTitleWithMnemonic(&self, stringWithAmpersand: Option<&NSString>);
    }
);