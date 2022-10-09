use crate::Foundation::generated::NSDictionary::*;
use crate::Foundation::generated::NSString::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSAttributedStringKey = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSAttributedString;
    unsafe impl ClassType for NSAttributedString {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSAttributedString {
        #[method_id(string)]
        pub unsafe fn string(&self) -> Id<NSString, Shared>;
        #[method_id(attributesAtIndex:effectiveRange:)]
        pub unsafe fn attributesAtIndex_effectiveRange(
            &self,
            location: NSUInteger,
            range: NSRangePointer,
        ) -> Id<NSDictionary<NSAttributedStringKey, Object>, Shared>;
    }
);
extern_methods!(
    #[doc = "NSExtendedAttributedString"]
    unsafe impl NSAttributedString {
        #[method(length)]
        pub unsafe fn length(&self) -> NSUInteger;
        #[method_id(attribute:atIndex:effectiveRange:)]
        pub unsafe fn attribute_atIndex_effectiveRange(
            &self,
            attrName: &NSAttributedStringKey,
            location: NSUInteger,
            range: NSRangePointer,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(attributedSubstringFromRange:)]
        pub unsafe fn attributedSubstringFromRange(
            &self,
            range: NSRange,
        ) -> Id<NSAttributedString, Shared>;
        #[method_id(attributesAtIndex:longestEffectiveRange:inRange:)]
        pub unsafe fn attributesAtIndex_longestEffectiveRange_inRange(
            &self,
            location: NSUInteger,
            range: NSRangePointer,
            rangeLimit: NSRange,
        ) -> Id<NSDictionary<NSAttributedStringKey, Object>, Shared>;
        #[method_id(attribute:atIndex:longestEffectiveRange:inRange:)]
        pub unsafe fn attribute_atIndex_longestEffectiveRange_inRange(
            &self,
            attrName: &NSAttributedStringKey,
            location: NSUInteger,
            range: NSRangePointer,
            rangeLimit: NSRange,
        ) -> Option<Id<Object, Shared>>;
        #[method(isEqualToAttributedString:)]
        pub unsafe fn isEqualToAttributedString(&self, other: &NSAttributedString) -> bool;
        #[method_id(initWithString:)]
        pub unsafe fn initWithString(&self, str: &NSString) -> Id<Self, Shared>;
        #[method_id(initWithString:attributes:)]
        pub unsafe fn initWithString_attributes(
            &self,
            str: &NSString,
            attrs: Option<&NSDictionary<NSAttributedStringKey, Object>>,
        ) -> Id<Self, Shared>;
        #[method_id(initWithAttributedString:)]
        pub unsafe fn initWithAttributedString(
            &self,
            attrStr: &NSAttributedString,
        ) -> Id<Self, Shared>;
        #[method(enumerateAttributesInRange:options:usingBlock:)]
        pub unsafe fn enumerateAttributesInRange_options_usingBlock(
            &self,
            enumerationRange: NSRange,
            opts: NSAttributedStringEnumerationOptions,
            block: TodoBlock,
        );
        #[method(enumerateAttribute:inRange:options:usingBlock:)]
        pub unsafe fn enumerateAttribute_inRange_options_usingBlock(
            &self,
            attrName: &NSAttributedStringKey,
            enumerationRange: NSRange,
            opts: NSAttributedStringEnumerationOptions,
            block: TodoBlock,
        );
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSMutableAttributedString;
    unsafe impl ClassType for NSMutableAttributedString {
        type Super = NSAttributedString;
    }
);
extern_methods!(
    unsafe impl NSMutableAttributedString {
        #[method(replaceCharactersInRange:withString:)]
        pub unsafe fn replaceCharactersInRange_withString(&self, range: NSRange, str: &NSString);
        #[method(setAttributes:range:)]
        pub unsafe fn setAttributes_range(
            &self,
            attrs: Option<&NSDictionary<NSAttributedStringKey, Object>>,
            range: NSRange,
        );
    }
);
extern_methods!(
    #[doc = "NSExtendedMutableAttributedString"]
    unsafe impl NSMutableAttributedString {
        #[method_id(mutableString)]
        pub unsafe fn mutableString(&self) -> Id<NSMutableString, Shared>;
        #[method(addAttribute:value:range:)]
        pub unsafe fn addAttribute_value_range(
            &self,
            name: &NSAttributedStringKey,
            value: &Object,
            range: NSRange,
        );
        #[method(addAttributes:range:)]
        pub unsafe fn addAttributes_range(
            &self,
            attrs: &NSDictionary<NSAttributedStringKey, Object>,
            range: NSRange,
        );
        #[method(removeAttribute:range:)]
        pub unsafe fn removeAttribute_range(&self, name: &NSAttributedStringKey, range: NSRange);
        #[method(replaceCharactersInRange:withAttributedString:)]
        pub unsafe fn replaceCharactersInRange_withAttributedString(
            &self,
            range: NSRange,
            attrString: &NSAttributedString,
        );
        #[method(insertAttributedString:atIndex:)]
        pub unsafe fn insertAttributedString_atIndex(
            &self,
            attrString: &NSAttributedString,
            loc: NSUInteger,
        );
        #[method(appendAttributedString:)]
        pub unsafe fn appendAttributedString(&self, attrString: &NSAttributedString);
        #[method(deleteCharactersInRange:)]
        pub unsafe fn deleteCharactersInRange(&self, range: NSRange);
        #[method(setAttributedString:)]
        pub unsafe fn setAttributedString(&self, attrString: &NSAttributedString);
        #[method(beginEditing)]
        pub unsafe fn beginEditing(&self);
        #[method(endEditing)]
        pub unsafe fn endEditing(&self);
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSAttributedStringMarkdownParsingOptions;
    unsafe impl ClassType for NSAttributedStringMarkdownParsingOptions {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSAttributedStringMarkdownParsingOptions {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method(allowsExtendedAttributes)]
        pub unsafe fn allowsExtendedAttributes(&self) -> bool;
        #[method(setAllowsExtendedAttributes:)]
        pub unsafe fn setAllowsExtendedAttributes(&self, allowsExtendedAttributes: bool);
        #[method(interpretedSyntax)]
        pub unsafe fn interpretedSyntax(&self) -> NSAttributedStringMarkdownInterpretedSyntax;
        #[method(setInterpretedSyntax:)]
        pub unsafe fn setInterpretedSyntax(
            &self,
            interpretedSyntax: NSAttributedStringMarkdownInterpretedSyntax,
        );
        #[method(failurePolicy)]
        pub unsafe fn failurePolicy(&self) -> NSAttributedStringMarkdownParsingFailurePolicy;
        #[method(setFailurePolicy:)]
        pub unsafe fn setFailurePolicy(
            &self,
            failurePolicy: NSAttributedStringMarkdownParsingFailurePolicy,
        );
        #[method_id(languageCode)]
        pub unsafe fn languageCode(&self) -> Option<Id<NSString, Shared>>;
        #[method(setLanguageCode:)]
        pub unsafe fn setLanguageCode(&self, languageCode: Option<&NSString>);
    }
);
extern_methods!(
    #[doc = "NSAttributedStringCreateFromMarkdown"]
    unsafe impl NSAttributedString {
        #[method_id(initWithContentsOfMarkdownFileAtURL:options:baseURL:error:)]
        pub unsafe fn initWithContentsOfMarkdownFileAtURL_options_baseURL_error(
            &self,
            markdownFile: &NSURL,
            options: Option<&NSAttributedStringMarkdownParsingOptions>,
            baseURL: Option<&NSURL>,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;
        #[method_id(initWithMarkdown:options:baseURL:error:)]
        pub unsafe fn initWithMarkdown_options_baseURL_error(
            &self,
            markdown: &NSData,
            options: Option<&NSAttributedStringMarkdownParsingOptions>,
            baseURL: Option<&NSURL>,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;
        #[method_id(initWithMarkdownString:options:baseURL:error:)]
        pub unsafe fn initWithMarkdownString_options_baseURL_error(
            &self,
            markdownString: &NSString,
            options: Option<&NSAttributedStringMarkdownParsingOptions>,
            baseURL: Option<&NSURL>,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSAttributedStringFormatting"]
    unsafe impl NSAttributedString {
        #[method_id(initWithFormat:options:locale:arguments:)]
        pub unsafe fn initWithFormat_options_locale_arguments(
            &self,
            format: &NSAttributedString,
            options: NSAttributedStringFormattingOptions,
            locale: Option<&NSLocale>,
            arguments: va_list,
        ) -> Id<Self, Shared>;
    }
);
extern_methods!(
    #[doc = "NSMutableAttributedStringFormatting"]
    unsafe impl NSMutableAttributedString {}
);
extern_methods!(
    #[doc = "NSMorphology"]
    unsafe impl NSAttributedString {
        #[method_id(attributedStringByInflectingString)]
        pub unsafe fn attributedStringByInflectingString(&self) -> Id<NSAttributedString, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSPresentationIntent;
    unsafe impl ClassType for NSPresentationIntent {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSPresentationIntent {
        #[method(intentKind)]
        pub unsafe fn intentKind(&self) -> NSPresentationIntentKind;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(parentIntent)]
        pub unsafe fn parentIntent(&self) -> Option<Id<NSPresentationIntent, Shared>>;
        #[method_id(paragraphIntentWithIdentity:nestedInsideIntent:)]
        pub unsafe fn paragraphIntentWithIdentity_nestedInsideIntent(
            identity: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent, Shared>;
        #[method_id(headerIntentWithIdentity:level:nestedInsideIntent:)]
        pub unsafe fn headerIntentWithIdentity_level_nestedInsideIntent(
            identity: NSInteger,
            level: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent, Shared>;
        #[method_id(codeBlockIntentWithIdentity:languageHint:nestedInsideIntent:)]
        pub unsafe fn codeBlockIntentWithIdentity_languageHint_nestedInsideIntent(
            identity: NSInteger,
            languageHint: Option<&NSString>,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent, Shared>;
        #[method_id(thematicBreakIntentWithIdentity:nestedInsideIntent:)]
        pub unsafe fn thematicBreakIntentWithIdentity_nestedInsideIntent(
            identity: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent, Shared>;
        #[method_id(orderedListIntentWithIdentity:nestedInsideIntent:)]
        pub unsafe fn orderedListIntentWithIdentity_nestedInsideIntent(
            identity: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent, Shared>;
        #[method_id(unorderedListIntentWithIdentity:nestedInsideIntent:)]
        pub unsafe fn unorderedListIntentWithIdentity_nestedInsideIntent(
            identity: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent, Shared>;
        #[method_id(listItemIntentWithIdentity:ordinal:nestedInsideIntent:)]
        pub unsafe fn listItemIntentWithIdentity_ordinal_nestedInsideIntent(
            identity: NSInteger,
            ordinal: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent, Shared>;
        #[method_id(blockQuoteIntentWithIdentity:nestedInsideIntent:)]
        pub unsafe fn blockQuoteIntentWithIdentity_nestedInsideIntent(
            identity: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent, Shared>;
        #[method_id(tableIntentWithIdentity:columnCount:alignments:nestedInsideIntent:)]
        pub unsafe fn tableIntentWithIdentity_columnCount_alignments_nestedInsideIntent(
            identity: NSInteger,
            columnCount: NSInteger,
            alignments: &NSArray<NSNumber>,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent, Shared>;
        #[method_id(tableHeaderRowIntentWithIdentity:nestedInsideIntent:)]
        pub unsafe fn tableHeaderRowIntentWithIdentity_nestedInsideIntent(
            identity: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent, Shared>;
        #[method_id(tableRowIntentWithIdentity:row:nestedInsideIntent:)]
        pub unsafe fn tableRowIntentWithIdentity_row_nestedInsideIntent(
            identity: NSInteger,
            row: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent, Shared>;
        #[method_id(tableCellIntentWithIdentity:column:nestedInsideIntent:)]
        pub unsafe fn tableCellIntentWithIdentity_column_nestedInsideIntent(
            identity: NSInteger,
            column: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent, Shared>;
        #[method(identity)]
        pub unsafe fn identity(&self) -> NSInteger;
        #[method(ordinal)]
        pub unsafe fn ordinal(&self) -> NSInteger;
        #[method_id(columnAlignments)]
        pub unsafe fn columnAlignments(&self) -> Option<Id<NSArray<NSNumber>, Shared>>;
        #[method(columnCount)]
        pub unsafe fn columnCount(&self) -> NSInteger;
        #[method(headerLevel)]
        pub unsafe fn headerLevel(&self) -> NSInteger;
        #[method_id(languageHint)]
        pub unsafe fn languageHint(&self) -> Option<Id<NSString, Shared>>;
        #[method(column)]
        pub unsafe fn column(&self) -> NSInteger;
        #[method(row)]
        pub unsafe fn row(&self) -> NSInteger;
        #[method(indentationLevel)]
        pub unsafe fn indentationLevel(&self) -> NSInteger;
        #[method(isEquivalentToPresentationIntent:)]
        pub unsafe fn isEquivalentToPresentationIntent(&self, other: &NSPresentationIntent)
            -> bool;
    }
);
