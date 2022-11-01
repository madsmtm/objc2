//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

pub type NSFormattingContext = NSInteger;
pub const NSFormattingContextUnknown: NSFormattingContext = 0;
pub const NSFormattingContextDynamic: NSFormattingContext = 1;
pub const NSFormattingContextStandalone: NSFormattingContext = 2;
pub const NSFormattingContextListItem: NSFormattingContext = 3;
pub const NSFormattingContextBeginningOfSentence: NSFormattingContext = 4;
pub const NSFormattingContextMiddleOfSentence: NSFormattingContext = 5;

pub type NSFormattingUnitStyle = NSInteger;
pub const NSFormattingUnitStyleShort: NSFormattingUnitStyle = 1;
pub const NSFormattingUnitStyleMedium: NSFormattingUnitStyle = 2;
pub const NSFormattingUnitStyleLong: NSFormattingUnitStyle = 3;

extern_class!(
    #[derive(Debug)]
    pub struct NSFormatter;

    unsafe impl ClassType for NSFormatter {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSFormatter {
        #[method_id(@__retain_semantics Other stringForObjectValue:)]
        pub unsafe fn stringForObjectValue(
            &self,
            obj: Option<&Object>,
        ) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other attributedStringForObjectValue:withDefaultAttributes:)]
        pub unsafe fn attributedStringForObjectValue_withDefaultAttributes(
            &self,
            obj: &Object,
            attrs: Option<&NSDictionary<NSAttributedStringKey, Object>>,
        ) -> Option<Id<NSAttributedString, Shared>>;

        #[method_id(@__retain_semantics Other editingStringForObjectValue:)]
        pub unsafe fn editingStringForObjectValue(
            &self,
            obj: &Object,
        ) -> Option<Id<NSString, Shared>>;

        #[method(getObjectValue:forString:errorDescription:)]
        pub unsafe fn getObjectValue_forString_errorDescription(
            &self,
            obj: *mut *mut Object,
            string: &NSString,
            error: *mut *mut NSString,
        ) -> bool;

        #[method(isPartialStringValid:newEditingString:errorDescription:)]
        pub unsafe fn isPartialStringValid_newEditingString_errorDescription(
            &self,
            partialString: &NSString,
            newString: *mut *mut NSString,
            error: *mut *mut NSString,
        ) -> bool;

        #[method(isPartialStringValid:proposedSelectedRange:originalString:originalSelectedRange:errorDescription:)]
        pub unsafe fn isPartialStringValid_proposedSelectedRange_originalString_originalSelectedRange_errorDescription(
            &self,
            partialStringPtr: NonNull<NonNull<NSString>>,
            proposedSelRangePtr: NSRangePointer,
            origString: &NSString,
            origSelRange: NSRange,
            error: *mut *mut NSString,
        ) -> bool;
    }
);
