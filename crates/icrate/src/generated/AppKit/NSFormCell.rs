#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSFormCell;
    unsafe impl ClassType for NSFormCell {
        type Super = NSActionCell;
    }
);
extern_methods!(
    unsafe impl NSFormCell {
        #[method_id(initTextCell:)]
        pub unsafe fn initTextCell(&self, string: Option<&NSString>) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Id<Self, Shared>;
        #[method_id(initImageCell:)]
        pub unsafe fn initImageCell(&self, image: Option<&NSImage>) -> Id<Self, Shared>;
        #[method(titleWidth:)]
        pub unsafe fn titleWidth(&self, size: NSSize) -> CGFloat;
        #[method(titleWidth)]
        pub unsafe fn titleWidth(&self) -> CGFloat;
        #[method(setTitleWidth:)]
        pub unsafe fn setTitleWidth(&self, titleWidth: CGFloat);
        #[method_id(title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);
        #[method_id(titleFont)]
        pub unsafe fn titleFont(&self) -> Id<NSFont, Shared>;
        #[method(setTitleFont:)]
        pub unsafe fn setTitleFont(&self, titleFont: &NSFont);
        #[method(isOpaque)]
        pub unsafe fn isOpaque(&self) -> bool;
        #[method_id(placeholderString)]
        pub unsafe fn placeholderString(&self) -> Option<Id<NSString, Shared>>;
        #[method(setPlaceholderString:)]
        pub unsafe fn setPlaceholderString(&self, placeholderString: Option<&NSString>);
        #[method_id(placeholderAttributedString)]
        pub unsafe fn placeholderAttributedString(&self) -> Option<Id<NSAttributedString, Shared>>;
        #[method(setPlaceholderAttributedString:)]
        pub unsafe fn setPlaceholderAttributedString(
            &self,
            placeholderAttributedString: Option<&NSAttributedString>,
        );
        #[method(titleAlignment)]
        pub unsafe fn titleAlignment(&self) -> NSTextAlignment;
        #[method(setTitleAlignment:)]
        pub unsafe fn setTitleAlignment(&self, titleAlignment: NSTextAlignment);
        #[method(titleBaseWritingDirection)]
        pub unsafe fn titleBaseWritingDirection(&self) -> NSWritingDirection;
        #[method(setTitleBaseWritingDirection:)]
        pub unsafe fn setTitleBaseWritingDirection(
            &self,
            titleBaseWritingDirection: NSWritingDirection,
        );
        #[method(preferredTextFieldWidth)]
        pub unsafe fn preferredTextFieldWidth(&self) -> CGFloat;
        #[method(setPreferredTextFieldWidth:)]
        pub unsafe fn setPreferredTextFieldWidth(&self, preferredTextFieldWidth: CGFloat);
    }
);
extern_methods!(
    #[doc = "NSKeyboardUI"]
    unsafe impl NSFormCell {
        #[method(setTitleWithMnemonic:)]
        pub unsafe fn setTitleWithMnemonic(&self, stringWithAmpersand: Option<&NSString>);
    }
);
extern_methods!(
    #[doc = "NSFormCellAttributedStringMethods"]
    unsafe impl NSFormCell {
        #[method_id(attributedTitle)]
        pub unsafe fn attributedTitle(&self) -> Id<NSAttributedString, Shared>;
        #[method(setAttributedTitle:)]
        pub unsafe fn setAttributedTitle(&self, attributedTitle: &NSAttributedString);
    }
);
