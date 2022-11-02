//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

pub type NSTextAlignment = NSInteger;
pub const NSTextAlignmentLeft: NSTextAlignment = 0;
pub const NSTextAlignmentRight: NSTextAlignment = 1;
pub const NSTextAlignmentCenter: NSTextAlignment = 2;
pub const NSTextAlignmentJustified: NSTextAlignment = 3;
pub const NSTextAlignmentNatural: NSTextAlignment = 4;

pub type NSWritingDirection = NSInteger;
pub const NSWritingDirectionNatural: NSWritingDirection = -1;
pub const NSWritingDirectionLeftToRight: NSWritingDirection = 0;
pub const NSWritingDirectionRightToLeft: NSWritingDirection = 1;

extern_class!(
    #[derive(Debug)]
    pub struct NSText;

    unsafe impl ClassType for NSText {
        type Super = NSView;
    }
);

extern_methods!(
    unsafe impl NSText {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frameRect: NSRect,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other string)]
        pub unsafe fn string(&self) -> Id<NSString, Shared>;

        #[method(setString:)]
        pub unsafe fn setString(&self, string: &NSString);

        #[method(replaceCharactersInRange:withString:)]
        pub unsafe fn replaceCharactersInRange_withString(&self, range: NSRange, string: &NSString);

        #[method(replaceCharactersInRange:withRTF:)]
        pub unsafe fn replaceCharactersInRange_withRTF(&self, range: NSRange, rtfData: &NSData);

        #[method(replaceCharactersInRange:withRTFD:)]
        pub unsafe fn replaceCharactersInRange_withRTFD(&self, range: NSRange, rtfdData: &NSData);

        #[method_id(@__retain_semantics Other RTFFromRange:)]
        pub unsafe fn RTFFromRange(&self, range: NSRange) -> Option<Id<NSData, Shared>>;

        #[method_id(@__retain_semantics Other RTFDFromRange:)]
        pub unsafe fn RTFDFromRange(&self, range: NSRange) -> Option<Id<NSData, Shared>>;

        #[method(writeRTFDToFile:atomically:)]
        pub unsafe fn writeRTFDToFile_atomically(&self, path: &NSString, flag: bool) -> bool;

        #[method(readRTFDFromFile:)]
        pub unsafe fn readRTFDFromFile(&self, path: &NSString) -> bool;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSTextDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSTextDelegate>);

        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);

        #[method(isSelectable)]
        pub unsafe fn isSelectable(&self) -> bool;

        #[method(setSelectable:)]
        pub unsafe fn setSelectable(&self, selectable: bool);

        #[method(isRichText)]
        pub unsafe fn isRichText(&self) -> bool;

        #[method(setRichText:)]
        pub unsafe fn setRichText(&self, richText: bool);

        #[method(importsGraphics)]
        pub unsafe fn importsGraphics(&self) -> bool;

        #[method(setImportsGraphics:)]
        pub unsafe fn setImportsGraphics(&self, importsGraphics: bool);

        #[method(isFieldEditor)]
        pub unsafe fn isFieldEditor(&self) -> bool;

        #[method(setFieldEditor:)]
        pub unsafe fn setFieldEditor(&self, fieldEditor: bool);

        #[method(usesFontPanel)]
        pub unsafe fn usesFontPanel(&self) -> bool;

        #[method(setUsesFontPanel:)]
        pub unsafe fn setUsesFontPanel(&self, usesFontPanel: bool);

        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;

        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, drawsBackground: bool);

        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Id<NSColor, Shared>>;

        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, backgroundColor: Option<&NSColor>);

        #[method(isRulerVisible)]
        pub unsafe fn isRulerVisible(&self) -> bool;

        #[method(selectedRange)]
        pub unsafe fn selectedRange(&self) -> NSRange;

        #[method(setSelectedRange:)]
        pub unsafe fn setSelectedRange(&self, selectedRange: NSRange);

        #[method(scrollRangeToVisible:)]
        pub unsafe fn scrollRangeToVisible(&self, range: NSRange);

        #[method_id(@__retain_semantics Other font)]
        pub unsafe fn font(&self) -> Option<Id<NSFont, Shared>>;

        #[method(setFont:)]
        pub unsafe fn setFont(&self, font: Option<&NSFont>);

        #[method_id(@__retain_semantics Other textColor)]
        pub unsafe fn textColor(&self) -> Option<Id<NSColor, Shared>>;

        #[method(setTextColor:)]
        pub unsafe fn setTextColor(&self, textColor: Option<&NSColor>);

        #[method(alignment)]
        pub unsafe fn alignment(&self) -> NSTextAlignment;

        #[method(setAlignment:)]
        pub unsafe fn setAlignment(&self, alignment: NSTextAlignment);

        #[method(baseWritingDirection)]
        pub unsafe fn baseWritingDirection(&self) -> NSWritingDirection;

        #[method(setBaseWritingDirection:)]
        pub unsafe fn setBaseWritingDirection(&self, baseWritingDirection: NSWritingDirection);

        #[method(setTextColor:range:)]
        pub unsafe fn setTextColor_range(&self, color: Option<&NSColor>, range: NSRange);

        #[method(setFont:range:)]
        pub unsafe fn setFont_range(&self, font: &NSFont, range: NSRange);

        #[method(maxSize)]
        pub unsafe fn maxSize(&self) -> NSSize;

        #[method(setMaxSize:)]
        pub unsafe fn setMaxSize(&self, maxSize: NSSize);

        #[method(minSize)]
        pub unsafe fn minSize(&self) -> NSSize;

        #[method(setMinSize:)]
        pub unsafe fn setMinSize(&self, minSize: NSSize);

        #[method(isHorizontallyResizable)]
        pub unsafe fn isHorizontallyResizable(&self) -> bool;

        #[method(setHorizontallyResizable:)]
        pub unsafe fn setHorizontallyResizable(&self, horizontallyResizable: bool);

        #[method(isVerticallyResizable)]
        pub unsafe fn isVerticallyResizable(&self) -> bool;

        #[method(setVerticallyResizable:)]
        pub unsafe fn setVerticallyResizable(&self, verticallyResizable: bool);

        #[method(sizeToFit)]
        pub unsafe fn sizeToFit(&self);

        #[method(copy:)]
        pub unsafe fn copy(&self, sender: Option<&Object>);

        #[method(copyFont:)]
        pub unsafe fn copyFont(&self, sender: Option<&Object>);

        #[method(copyRuler:)]
        pub unsafe fn copyRuler(&self, sender: Option<&Object>);

        #[method(cut:)]
        pub unsafe fn cut(&self, sender: Option<&Object>);

        #[method(delete:)]
        pub unsafe fn delete(&self, sender: Option<&Object>);

        #[method(paste:)]
        pub unsafe fn paste(&self, sender: Option<&Object>);

        #[method(pasteFont:)]
        pub unsafe fn pasteFont(&self, sender: Option<&Object>);

        #[method(pasteRuler:)]
        pub unsafe fn pasteRuler(&self, sender: Option<&Object>);

        #[method(selectAll:)]
        pub unsafe fn selectAll(&self, sender: Option<&Object>);

        #[method(changeFont:)]
        pub unsafe fn changeFont(&self, sender: Option<&Object>);

        #[method(alignLeft:)]
        pub unsafe fn alignLeft(&self, sender: Option<&Object>);

        #[method(alignRight:)]
        pub unsafe fn alignRight(&self, sender: Option<&Object>);

        #[method(alignCenter:)]
        pub unsafe fn alignCenter(&self, sender: Option<&Object>);

        #[method(subscript:)]
        pub unsafe fn subscript(&self, sender: Option<&Object>);

        #[method(superscript:)]
        pub unsafe fn superscript(&self, sender: Option<&Object>);

        #[method(underline:)]
        pub unsafe fn underline(&self, sender: Option<&Object>);

        #[method(unscript:)]
        pub unsafe fn unscript(&self, sender: Option<&Object>);

        #[method(showGuessPanel:)]
        pub unsafe fn showGuessPanel(&self, sender: Option<&Object>);

        #[method(checkSpelling:)]
        pub unsafe fn checkSpelling(&self, sender: Option<&Object>);

        #[method(toggleRuler:)]
        pub unsafe fn toggleRuler(&self, sender: Option<&Object>);
    }
);

pub const NSEnterCharacter: c_uint = 0x0003;
pub const NSBackspaceCharacter: c_uint = 0x0008;
pub const NSTabCharacter: c_uint = 0x0009;
pub const NSNewlineCharacter: c_uint = 0x000a;
pub const NSFormFeedCharacter: c_uint = 0x000c;
pub const NSCarriageReturnCharacter: c_uint = 0x000d;
pub const NSBackTabCharacter: c_uint = 0x0019;
pub const NSDeleteCharacter: c_uint = 0x007f;
pub const NSLineSeparatorCharacter: c_uint = 0x2028;
pub const NSParagraphSeparatorCharacter: c_uint = 0x2029;

pub type NSTextMovement = NSInteger;
pub const NSTextMovementReturn: NSTextMovement = 0x10;
pub const NSTextMovementTab: NSTextMovement = 0x11;
pub const NSTextMovementBacktab: NSTextMovement = 0x12;
pub const NSTextMovementLeft: NSTextMovement = 0x13;
pub const NSTextMovementRight: NSTextMovement = 0x14;
pub const NSTextMovementUp: NSTextMovement = 0x15;
pub const NSTextMovementDown: NSTextMovement = 0x16;
pub const NSTextMovementCancel: NSTextMovement = 0x17;
pub const NSTextMovementOther: NSTextMovement = 0;

extern "C" {
    pub static NSTextDidBeginEditingNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSTextDidEndEditingNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSTextDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSTextMovementUserInfoKey: &'static NSString;
}

pub const NSIllegalTextMovement: c_uint = 0;
pub const NSReturnTextMovement: c_uint = 0x10;
pub const NSTabTextMovement: c_uint = 0x11;
pub const NSBacktabTextMovement: c_uint = 0x12;
pub const NSLeftTextMovement: c_uint = 0x13;
pub const NSRightTextMovement: c_uint = 0x14;
pub const NSUpTextMovement: c_uint = 0x15;
pub const NSDownTextMovement: c_uint = 0x16;
pub const NSCancelTextMovement: c_uint = 0x17;
pub const NSOtherTextMovement: c_uint = 0;

pub type NSTextDelegate = NSObject;

pub const NSTextWritingDirectionEmbedding: c_uint = 0 << 1;
pub const NSTextWritingDirectionOverride: c_uint = 1 << 1;

pub static NSLeftTextAlignment: NSTextAlignment = NSTextAlignmentLeft;

pub static NSRightTextAlignment: NSTextAlignment = NSTextAlignmentRight;

pub static NSCenterTextAlignment: NSTextAlignment = NSTextAlignmentCenter;

pub static NSJustifiedTextAlignment: NSTextAlignment = NSTextAlignmentJustified;

pub static NSNaturalTextAlignment: NSTextAlignment = NSTextAlignmentNatural;
