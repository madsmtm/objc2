use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSControl::*;
use crate::AppKit::generated::NSParagraphStyle::*;
use crate::AppKit::generated::NSTextContent::*;
use crate::AppKit::generated::NSTextFieldCell::*;
use crate::AppKit::generated::NSUserInterfaceValidation::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTextField;
    unsafe impl ClassType for NSTextField {
        type Super = NSControl;
    }
);
extern_methods!(
    unsafe impl NSTextField {
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
        #[method_id(backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Id<NSColor, Shared>>;
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, backgroundColor: Option<&NSColor>);
        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;
        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, drawsBackground: bool);
        #[method_id(textColor)]
        pub unsafe fn textColor(&self) -> Option<Id<NSColor, Shared>>;
        #[method(setTextColor:)]
        pub unsafe fn setTextColor(&self, textColor: Option<&NSColor>);
        #[method(isBordered)]
        pub unsafe fn isBordered(&self) -> bool;
        #[method(setBordered:)]
        pub unsafe fn setBordered(&self, bordered: bool);
        #[method(isBezeled)]
        pub unsafe fn isBezeled(&self) -> bool;
        #[method(setBezeled:)]
        pub unsafe fn setBezeled(&self, bezeled: bool);
        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;
        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);
        #[method(isSelectable)]
        pub unsafe fn isSelectable(&self) -> bool;
        #[method(setSelectable:)]
        pub unsafe fn setSelectable(&self, selectable: bool);
        #[method(selectText:)]
        pub unsafe fn selectText(&self, sender: Option<&Object>);
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSTextFieldDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSTextFieldDelegate>);
        #[method(textShouldBeginEditing:)]
        pub unsafe fn textShouldBeginEditing(&self, textObject: &NSText) -> bool;
        #[method(textShouldEndEditing:)]
        pub unsafe fn textShouldEndEditing(&self, textObject: &NSText) -> bool;
        #[method(textDidBeginEditing:)]
        pub unsafe fn textDidBeginEditing(&self, notification: &NSNotification);
        #[method(textDidEndEditing:)]
        pub unsafe fn textDidEndEditing(&self, notification: &NSNotification);
        #[method(textDidChange:)]
        pub unsafe fn textDidChange(&self, notification: &NSNotification);
        #[method(acceptsFirstResponder)]
        pub unsafe fn acceptsFirstResponder(&self) -> bool;
        #[method(bezelStyle)]
        pub unsafe fn bezelStyle(&self) -> NSTextFieldBezelStyle;
        #[method(setBezelStyle:)]
        pub unsafe fn setBezelStyle(&self, bezelStyle: NSTextFieldBezelStyle);
        #[method(preferredMaxLayoutWidth)]
        pub unsafe fn preferredMaxLayoutWidth(&self) -> CGFloat;
        #[method(setPreferredMaxLayoutWidth:)]
        pub unsafe fn setPreferredMaxLayoutWidth(&self, preferredMaxLayoutWidth: CGFloat);
        #[method(maximumNumberOfLines)]
        pub unsafe fn maximumNumberOfLines(&self) -> NSInteger;
        #[method(setMaximumNumberOfLines:)]
        pub unsafe fn setMaximumNumberOfLines(&self, maximumNumberOfLines: NSInteger);
        #[method(allowsDefaultTighteningForTruncation)]
        pub unsafe fn allowsDefaultTighteningForTruncation(&self) -> bool;
        #[method(setAllowsDefaultTighteningForTruncation:)]
        pub unsafe fn setAllowsDefaultTighteningForTruncation(
            &self,
            allowsDefaultTighteningForTruncation: bool,
        );
        #[method(lineBreakStrategy)]
        pub unsafe fn lineBreakStrategy(&self) -> NSLineBreakStrategy;
        #[method(setLineBreakStrategy:)]
        pub unsafe fn setLineBreakStrategy(&self, lineBreakStrategy: NSLineBreakStrategy);
    }
);
extern_methods!(
    #[doc = "NSTouchBar"]
    unsafe impl NSTextField {
        #[method(isAutomaticTextCompletionEnabled)]
        pub unsafe fn isAutomaticTextCompletionEnabled(&self) -> bool;
        #[method(setAutomaticTextCompletionEnabled:)]
        pub unsafe fn setAutomaticTextCompletionEnabled(
            &self,
            automaticTextCompletionEnabled: bool,
        );
        #[method(allowsCharacterPickerTouchBarItem)]
        pub unsafe fn allowsCharacterPickerTouchBarItem(&self) -> bool;
        #[method(setAllowsCharacterPickerTouchBarItem:)]
        pub unsafe fn setAllowsCharacterPickerTouchBarItem(
            &self,
            allowsCharacterPickerTouchBarItem: bool,
        );
    }
);
extern_methods!(
    #[doc = "NSTextFieldConvenience"]
    unsafe impl NSTextField {
        #[method_id(labelWithString:)]
        pub unsafe fn labelWithString(stringValue: &NSString) -> Id<Self, Shared>;
        #[method_id(wrappingLabelWithString:)]
        pub unsafe fn wrappingLabelWithString(stringValue: &NSString) -> Id<Self, Shared>;
        #[method_id(labelWithAttributedString:)]
        pub unsafe fn labelWithAttributedString(
            attributedStringValue: &NSAttributedString,
        ) -> Id<Self, Shared>;
        #[method_id(textFieldWithString:)]
        pub unsafe fn textFieldWithString(stringValue: &NSString) -> Id<Self, Shared>;
    }
);
extern_methods!(
    #[doc = "NSTextFieldAttributedStringMethods"]
    unsafe impl NSTextField {
        #[method(allowsEditingTextAttributes)]
        pub unsafe fn allowsEditingTextAttributes(&self) -> bool;
        #[method(setAllowsEditingTextAttributes:)]
        pub unsafe fn setAllowsEditingTextAttributes(&self, allowsEditingTextAttributes: bool);
        #[method(importsGraphics)]
        pub unsafe fn importsGraphics(&self) -> bool;
        #[method(setImportsGraphics:)]
        pub unsafe fn setImportsGraphics(&self, importsGraphics: bool);
    }
);
pub type NSTextFieldDelegate = NSObject;
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSTextField {
        #[method(setTitleWithMnemonic:)]
        pub unsafe fn setTitleWithMnemonic(&self, stringWithAmpersand: Option<&NSString>);
    }
);
