//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub type NSTextFieldBezelStyle = NSUInteger;
pub const NSTextFieldSquareBezel: NSTextFieldBezelStyle = 0;
pub const NSTextFieldRoundedBezel: NSTextFieldBezelStyle = 1;

extern_class!(
    #[derive(Debug)]
    pub struct NSTextFieldCell;

    unsafe impl ClassType for NSTextFieldCell {
        type Super = NSActionCell;
    }
);

extern_methods!(
    unsafe impl NSTextFieldCell {
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(
            this: Option<Allocated<Self>>,
            string: &NSString,
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

        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Id<NSColor, Shared>>;

        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, backgroundColor: Option<&NSColor>);

        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;

        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, drawsBackground: bool);

        #[method_id(@__retain_semantics Other textColor)]
        pub unsafe fn textColor(&self) -> Option<Id<NSColor, Shared>>;

        #[method(setTextColor:)]
        pub unsafe fn setTextColor(&self, textColor: Option<&NSColor>);

        #[method_id(@__retain_semantics Other setUpFieldEditorAttributes:)]
        pub unsafe fn setUpFieldEditorAttributes(&self, textObj: &NSText) -> Id<NSText, Shared>;

        #[method(bezelStyle)]
        pub unsafe fn bezelStyle(&self) -> NSTextFieldBezelStyle;

        #[method(setBezelStyle:)]
        pub unsafe fn setBezelStyle(&self, bezelStyle: NSTextFieldBezelStyle);

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

        #[method(setWantsNotificationForMarkedText:)]
        pub unsafe fn setWantsNotificationForMarkedText(&self, flag: bool);

        #[method_id(@__retain_semantics Other allowedInputSourceLocales)]
        pub unsafe fn allowedInputSourceLocales(&self) -> Option<Id<NSArray<NSString>, Shared>>;

        #[method(setAllowedInputSourceLocales:)]
        pub unsafe fn setAllowedInputSourceLocales(
            &self,
            allowedInputSourceLocales: Option<&NSArray<NSString>>,
        );
    }
);
