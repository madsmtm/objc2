#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSTextEffectStyle = NSString;
extern_methods!(
    #[doc = "NSAttributedStringAttributeFixing"]
    unsafe impl NSMutableAttributedString {
        #[method(fixAttributesInRange:)]
        pub unsafe fn fixAttributesInRange(&self, range: NSRange);
        #[method(fixFontAttributeInRange:)]
        pub unsafe fn fixFontAttributeInRange(&self, range: NSRange);
        #[method(fixParagraphStyleAttributeInRange:)]
        pub unsafe fn fixParagraphStyleAttributeInRange(&self, range: NSRange);
        #[method(fixAttachmentAttributeInRange:)]
        pub unsafe fn fixAttachmentAttributeInRange(&self, range: NSRange);
    }
);
pub type NSAttributedStringDocumentType = NSString;
pub type NSTextLayoutSectionKey = NSString;
pub type NSAttributedStringDocumentAttributeKey = NSString;
pub type NSAttributedStringDocumentReadingOptionKey = NSString;
extern_methods!(
    #[doc = "NSAttributedStringDocumentFormats"]
    unsafe impl NSAttributedString {
        #[method_id(initWithURL:options:documentAttributes:error:)]
        pub unsafe fn initWithURL_options_documentAttributes_error(
            &self,
            url: &NSURL,
            options: &NSDictionary<NSAttributedStringDocumentReadingOptionKey, Object>,
            dict: Option<
                &mut Option<
                    Id<NSDictionary<NSAttributedStringDocumentAttributeKey, Object>, Shared>,
                >,
            >,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;
        #[method_id(initWithData:options:documentAttributes:error:)]
        pub unsafe fn initWithData_options_documentAttributes_error(
            &self,
            data: &NSData,
            options: &NSDictionary<NSAttributedStringDocumentReadingOptionKey, Object>,
            dict: Option<
                &mut Option<
                    Id<NSDictionary<NSAttributedStringDocumentAttributeKey, Object>, Shared>,
                >,
            >,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;
        #[method_id(dataFromRange:documentAttributes:error:)]
        pub unsafe fn dataFromRange_documentAttributes_error(
            &self,
            range: NSRange,
            dict: &NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Result<Id<NSData, Shared>, Id<NSError, Shared>>;
        #[method_id(fileWrapperFromRange:documentAttributes:error:)]
        pub unsafe fn fileWrapperFromRange_documentAttributes_error(
            &self,
            range: NSRange,
            dict: &NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Result<Id<NSFileWrapper, Shared>, Id<NSError, Shared>>;
        #[method_id(initWithRTF:documentAttributes:)]
        pub unsafe fn initWithRTF_documentAttributes(
            &self,
            data: &NSData,
            dict: Option<
                &mut Option<
                    Id<NSDictionary<NSAttributedStringDocumentAttributeKey, Object>, Shared>,
                >,
            >,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithRTFD:documentAttributes:)]
        pub unsafe fn initWithRTFD_documentAttributes(
            &self,
            data: &NSData,
            dict: Option<
                &mut Option<
                    Id<NSDictionary<NSAttributedStringDocumentAttributeKey, Object>, Shared>,
                >,
            >,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithHTML:documentAttributes:)]
        pub unsafe fn initWithHTML_documentAttributes(
            &self,
            data: &NSData,
            dict: Option<
                &mut Option<
                    Id<NSDictionary<NSAttributedStringDocumentAttributeKey, Object>, Shared>,
                >,
            >,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithHTML:baseURL:documentAttributes:)]
        pub unsafe fn initWithHTML_baseURL_documentAttributes(
            &self,
            data: &NSData,
            base: &NSURL,
            dict: Option<
                &mut Option<
                    Id<NSDictionary<NSAttributedStringDocumentAttributeKey, Object>, Shared>,
                >,
            >,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithDocFormat:documentAttributes:)]
        pub unsafe fn initWithDocFormat_documentAttributes(
            &self,
            data: &NSData,
            dict: Option<
                &mut Option<
                    Id<NSDictionary<NSAttributedStringDocumentAttributeKey, Object>, Shared>,
                >,
            >,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithHTML:options:documentAttributes:)]
        pub unsafe fn initWithHTML_options_documentAttributes(
            &self,
            data: &NSData,
            options: &NSDictionary<NSAttributedStringDocumentReadingOptionKey, Object>,
            dict: Option<
                &mut Option<
                    Id<NSDictionary<NSAttributedStringDocumentAttributeKey, Object>, Shared>,
                >,
            >,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithRTFDFileWrapper:documentAttributes:)]
        pub unsafe fn initWithRTFDFileWrapper_documentAttributes(
            &self,
            wrapper: &NSFileWrapper,
            dict: Option<
                &mut Option<
                    Id<NSDictionary<NSAttributedStringDocumentAttributeKey, Object>, Shared>,
                >,
            >,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(RTFFromRange:documentAttributes:)]
        pub unsafe fn RTFFromRange_documentAttributes(
            &self,
            range: NSRange,
            dict: &NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Option<Id<NSData, Shared>>;
        #[method_id(RTFDFromRange:documentAttributes:)]
        pub unsafe fn RTFDFromRange_documentAttributes(
            &self,
            range: NSRange,
            dict: &NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Option<Id<NSData, Shared>>;
        #[method_id(RTFDFileWrapperFromRange:documentAttributes:)]
        pub unsafe fn RTFDFileWrapperFromRange_documentAttributes(
            &self,
            range: NSRange,
            dict: &NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Option<Id<NSFileWrapper, Shared>>;
        #[method_id(docFormatFromRange:documentAttributes:)]
        pub unsafe fn docFormatFromRange_documentAttributes(
            &self,
            range: NSRange,
            dict: &NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Option<Id<NSData, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSMutableAttributedStringDocumentFormats"]
    unsafe impl NSMutableAttributedString {
        #[method(readFromURL:options:documentAttributes:error:)]
        pub unsafe fn readFromURL_options_documentAttributes_error(
            &self,
            url: &NSURL,
            opts: &NSDictionary<NSAttributedStringDocumentReadingOptionKey, Object>,
            dict: Option<
                &mut Option<
                    Id<NSDictionary<NSAttributedStringDocumentAttributeKey, Object>, Shared>,
                >,
            >,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method(readFromData:options:documentAttributes:error:)]
        pub unsafe fn readFromData_options_documentAttributes_error(
            &self,
            data: &NSData,
            opts: &NSDictionary<NSAttributedStringDocumentReadingOptionKey, Object>,
            dict: Option<
                &mut Option<
                    Id<NSDictionary<NSAttributedStringDocumentAttributeKey, Object>, Shared>,
                >,
            >,
        ) -> Result<(), Id<NSError, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSAttributedStringKitAdditions"]
    unsafe impl NSAttributedString {
        #[method_id(fontAttributesInRange:)]
        pub unsafe fn fontAttributesInRange(
            &self,
            range: NSRange,
        ) -> Id<NSDictionary<NSAttributedStringKey, Object>, Shared>;
        #[method_id(rulerAttributesInRange:)]
        pub unsafe fn rulerAttributesInRange(
            &self,
            range: NSRange,
        ) -> Id<NSDictionary<NSAttributedStringKey, Object>, Shared>;
        #[method(containsAttachmentsInRange:)]
        pub unsafe fn containsAttachmentsInRange(&self, range: NSRange) -> bool;
        #[method(lineBreakBeforeIndex:withinRange:)]
        pub unsafe fn lineBreakBeforeIndex_withinRange(
            &self,
            location: NSUInteger,
            aRange: NSRange,
        ) -> NSUInteger;
        #[method(lineBreakByHyphenatingBeforeIndex:withinRange:)]
        pub unsafe fn lineBreakByHyphenatingBeforeIndex_withinRange(
            &self,
            location: NSUInteger,
            aRange: NSRange,
        ) -> NSUInteger;
        #[method(doubleClickAtIndex:)]
        pub unsafe fn doubleClickAtIndex(&self, location: NSUInteger) -> NSRange;
        #[method(nextWordFromIndex:forward:)]
        pub unsafe fn nextWordFromIndex_forward(
            &self,
            location: NSUInteger,
            isForward: bool,
        ) -> NSUInteger;
        #[method(rangeOfTextBlock:atIndex:)]
        pub unsafe fn rangeOfTextBlock_atIndex(
            &self,
            block: &NSTextBlock,
            location: NSUInteger,
        ) -> NSRange;
        #[method(rangeOfTextTable:atIndex:)]
        pub unsafe fn rangeOfTextTable_atIndex(
            &self,
            table: &NSTextTable,
            location: NSUInteger,
        ) -> NSRange;
        #[method(rangeOfTextList:atIndex:)]
        pub unsafe fn rangeOfTextList_atIndex(
            &self,
            list: &NSTextList,
            location: NSUInteger,
        ) -> NSRange;
        #[method(itemNumberInTextList:atIndex:)]
        pub unsafe fn itemNumberInTextList_atIndex(
            &self,
            list: &NSTextList,
            location: NSUInteger,
        ) -> NSInteger;
    }
);
extern_methods!(
    #[doc = "NSAttributedStringPasteboardAdditions"]
    unsafe impl NSAttributedString {
        #[method_id(textTypes)]
        pub unsafe fn textTypes() -> Id<NSArray<NSString>, Shared>;
        #[method_id(textUnfilteredTypes)]
        pub unsafe fn textUnfilteredTypes() -> Id<NSArray<NSString>, Shared>;
    }
);
extern_methods!(
    #[doc = "NSMutableAttributedStringKitAdditions"]
    unsafe impl NSMutableAttributedString {
        #[method(superscriptRange:)]
        pub unsafe fn superscriptRange(&self, range: NSRange);
        #[method(subscriptRange:)]
        pub unsafe fn subscriptRange(&self, range: NSRange);
        #[method(unscriptRange:)]
        pub unsafe fn unscriptRange(&self, range: NSRange);
        #[method(applyFontTraits:range:)]
        pub unsafe fn applyFontTraits_range(&self, traitMask: NSFontTraitMask, range: NSRange);
        #[method(setAlignment:range:)]
        pub unsafe fn setAlignment_range(&self, alignment: NSTextAlignment, range: NSRange);
        #[method(setBaseWritingDirection:range:)]
        pub unsafe fn setBaseWritingDirection_range(
            &self,
            writingDirection: NSWritingDirection,
            range: NSRange,
        );
    }
);
extern_methods!(
    #[doc = "NSDeprecatedKitAdditions"]
    unsafe impl NSAttributedString {
        #[method(containsAttachments)]
        pub unsafe fn containsAttachments(&self) -> bool;
        #[method_id(textFileTypes)]
        pub unsafe fn textFileTypes() -> Id<NSArray, Shared>;
        #[method_id(textPasteboardTypes)]
        pub unsafe fn textPasteboardTypes() -> Id<NSArray, Shared>;
        #[method_id(textUnfilteredFileTypes)]
        pub unsafe fn textUnfilteredFileTypes() -> Id<NSArray, Shared>;
        #[method_id(textUnfilteredPasteboardTypes)]
        pub unsafe fn textUnfilteredPasteboardTypes() -> Id<NSArray, Shared>;
        #[method_id(initWithURL:documentAttributes:)]
        pub unsafe fn initWithURL_documentAttributes(
            &self,
            url: &NSURL,
            dict: Option<&mut Option<Id<NSDictionary, Shared>>>,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithPath:documentAttributes:)]
        pub unsafe fn initWithPath_documentAttributes(
            &self,
            path: &NSString,
            dict: Option<&mut Option<Id<NSDictionary, Shared>>>,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(URLAtIndex:effectiveRange:)]
        pub unsafe fn URLAtIndex_effectiveRange(
            &self,
            location: NSUInteger,
            effectiveRange: NSRangePointer,
        ) -> Option<Id<NSURL, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSDeprecatedKitAdditions"]
    unsafe impl NSMutableAttributedString {
        #[method(readFromURL:options:documentAttributes:)]
        pub unsafe fn readFromURL_options_documentAttributes(
            &self,
            url: &NSURL,
            options: &NSDictionary,
            dict: Option<&mut Option<Id<NSDictionary, Shared>>>,
        ) -> bool;
        #[method(readFromData:options:documentAttributes:)]
        pub unsafe fn readFromData_options_documentAttributes(
            &self,
            data: &NSData,
            options: &NSDictionary,
            dict: Option<&mut Option<Id<NSDictionary, Shared>>>,
        ) -> bool;
    }
);
