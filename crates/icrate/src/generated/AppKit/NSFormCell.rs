//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug)]
    pub struct NSFormCell;

    unsafe impl ClassType for NSFormCell {
        type Super = NSActionCell;
    }
);

extern_methods!(
    unsafe impl NSFormCell {
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(
            this: Option<Allocated<Self>>,
            string: Option<&NSString>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(
            this: Option<Allocated<Self>>,
            image: Option<&NSImage>,
        ) -> Id<Self, Shared>;

        #[method(titleWidth)]
        pub unsafe fn titleWidth(&self) -> CGFloat;

        #[method(setTitleWidth:)]
        pub unsafe fn setTitleWidth(&self, titleWidth: CGFloat);

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[method_id(@__retain_semantics Other titleFont)]
        pub unsafe fn titleFont(&self) -> Id<NSFont, Shared>;

        #[method(setTitleFont:)]
        pub unsafe fn setTitleFont(&self, titleFont: &NSFont);

        #[method(isOpaque)]
        pub unsafe fn isOpaque(&self) -> bool;

        #[method_id(@__retain_semantics Other placeholderString)]
        pub unsafe fn placeholderString(&self) -> Option<Id<NSString, Shared>>;

        #[method(setPlaceholderString:)]
        pub unsafe fn setPlaceholderString(&self, placeholderString: Option<&NSString>);

        #[method_id(@__retain_semantics Other placeholderAttributedString)]
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
    /// NSKeyboardUI
    unsafe impl NSFormCell {
        #[method(setTitleWithMnemonic:)]
        pub unsafe fn setTitleWithMnemonic(&self, stringWithAmpersand: Option<&NSString>);
    }
);

extern_methods!(
    /// NSFormCellAttributedStringMethods
    unsafe impl NSFormCell {
        #[method_id(@__retain_semantics Other attributedTitle)]
        pub unsafe fn attributedTitle(&self) -> Id<NSAttributedString, Shared>;

        #[method(setAttributedTitle:)]
        pub unsafe fn setAttributedTitle(&self, attributedTitle: &NSAttributedString);
    }
);
