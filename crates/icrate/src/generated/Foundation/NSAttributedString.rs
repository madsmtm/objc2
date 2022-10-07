use crate::Foundation::generated::NSDictionary::*;
use crate::Foundation::generated::NSString::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
pub type NSAttributedStringKey = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSAttributedString;
    unsafe impl ClassType for NSAttributedString {
        type Super = NSObject;
    }
);
impl NSAttributedString {
    pub unsafe fn string(&self) -> Id<NSString, Shared> {
        msg_send_id![self, string]
    }
    pub unsafe fn attributesAtIndex_effectiveRange(
        &self,
        location: NSUInteger,
        range: NSRangePointer,
    ) -> Id<NSDictionary<NSAttributedStringKey, Object>, Shared> {
        msg_send_id![self, attributesAtIndex: location, effectiveRange: range]
    }
}
#[doc = "NSExtendedAttributedString"]
impl NSAttributedString {
    pub unsafe fn length(&self) -> NSUInteger {
        msg_send![self, length]
    }
    pub unsafe fn attribute_atIndex_effectiveRange(
        &self,
        attrName: &NSAttributedStringKey,
        location: NSUInteger,
        range: NSRangePointer,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![
            self,
            attribute: attrName,
            atIndex: location,
            effectiveRange: range
        ]
    }
    pub unsafe fn attributedSubstringFromRange(
        &self,
        range: NSRange,
    ) -> Id<NSAttributedString, Shared> {
        msg_send_id![self, attributedSubstringFromRange: range]
    }
    pub unsafe fn attributesAtIndex_longestEffectiveRange_inRange(
        &self,
        location: NSUInteger,
        range: NSRangePointer,
        rangeLimit: NSRange,
    ) -> Id<NSDictionary<NSAttributedStringKey, Object>, Shared> {
        msg_send_id![
            self,
            attributesAtIndex: location,
            longestEffectiveRange: range,
            inRange: rangeLimit
        ]
    }
    pub unsafe fn attribute_atIndex_longestEffectiveRange_inRange(
        &self,
        attrName: &NSAttributedStringKey,
        location: NSUInteger,
        range: NSRangePointer,
        rangeLimit: NSRange,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![
            self,
            attribute: attrName,
            atIndex: location,
            longestEffectiveRange: range,
            inRange: rangeLimit
        ]
    }
    pub unsafe fn isEqualToAttributedString(&self, other: &NSAttributedString) -> bool {
        msg_send![self, isEqualToAttributedString: other]
    }
    pub unsafe fn initWithString(&self, str: &NSString) -> Id<Self, Shared> {
        msg_send_id![self, initWithString: str]
    }
    pub unsafe fn initWithString_attributes(
        &self,
        str: &NSString,
        attrs: Option<&NSDictionary<NSAttributedStringKey, Object>>,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithString: str, attributes: attrs]
    }
    pub unsafe fn initWithAttributedString(
        &self,
        attrStr: &NSAttributedString,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithAttributedString: attrStr]
    }
    pub unsafe fn enumerateAttributesInRange_options_usingBlock(
        &self,
        enumerationRange: NSRange,
        opts: NSAttributedStringEnumerationOptions,
        block: TodoBlock,
    ) {
        msg_send![
            self,
            enumerateAttributesInRange: enumerationRange,
            options: opts,
            usingBlock: block
        ]
    }
    pub unsafe fn enumerateAttribute_inRange_options_usingBlock(
        &self,
        attrName: &NSAttributedStringKey,
        enumerationRange: NSRange,
        opts: NSAttributedStringEnumerationOptions,
        block: TodoBlock,
    ) {
        msg_send![
            self,
            enumerateAttribute: attrName,
            inRange: enumerationRange,
            options: opts,
            usingBlock: block
        ]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSMutableAttributedString;
    unsafe impl ClassType for NSMutableAttributedString {
        type Super = NSAttributedString;
    }
);
impl NSMutableAttributedString {
    pub unsafe fn replaceCharactersInRange_withString(&self, range: NSRange, str: &NSString) {
        msg_send![self, replaceCharactersInRange: range, withString: str]
    }
    pub unsafe fn setAttributes_range(
        &self,
        attrs: Option<&NSDictionary<NSAttributedStringKey, Object>>,
        range: NSRange,
    ) {
        msg_send![self, setAttributes: attrs, range: range]
    }
}
#[doc = "NSExtendedMutableAttributedString"]
impl NSMutableAttributedString {
    pub unsafe fn mutableString(&self) -> Id<NSMutableString, Shared> {
        msg_send_id![self, mutableString]
    }
    pub unsafe fn addAttribute_value_range(
        &self,
        name: &NSAttributedStringKey,
        value: &Object,
        range: NSRange,
    ) {
        msg_send![self, addAttribute: name, value: value, range: range]
    }
    pub unsafe fn addAttributes_range(
        &self,
        attrs: &NSDictionary<NSAttributedStringKey, Object>,
        range: NSRange,
    ) {
        msg_send![self, addAttributes: attrs, range: range]
    }
    pub unsafe fn removeAttribute_range(&self, name: &NSAttributedStringKey, range: NSRange) {
        msg_send![self, removeAttribute: name, range: range]
    }
    pub unsafe fn replaceCharactersInRange_withAttributedString(
        &self,
        range: NSRange,
        attrString: &NSAttributedString,
    ) {
        msg_send![
            self,
            replaceCharactersInRange: range,
            withAttributedString: attrString
        ]
    }
    pub unsafe fn insertAttributedString_atIndex(
        &self,
        attrString: &NSAttributedString,
        loc: NSUInteger,
    ) {
        msg_send![self, insertAttributedString: attrString, atIndex: loc]
    }
    pub unsafe fn appendAttributedString(&self, attrString: &NSAttributedString) {
        msg_send![self, appendAttributedString: attrString]
    }
    pub unsafe fn deleteCharactersInRange(&self, range: NSRange) {
        msg_send![self, deleteCharactersInRange: range]
    }
    pub unsafe fn setAttributedString(&self, attrString: &NSAttributedString) {
        msg_send![self, setAttributedString: attrString]
    }
    pub unsafe fn beginEditing(&self) {
        msg_send![self, beginEditing]
    }
    pub unsafe fn endEditing(&self) {
        msg_send![self, endEditing]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSAttributedStringMarkdownParsingOptions;
    unsafe impl ClassType for NSAttributedStringMarkdownParsingOptions {
        type Super = NSObject;
    }
);
impl NSAttributedStringMarkdownParsingOptions {
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn allowsExtendedAttributes(&self) -> bool {
        msg_send![self, allowsExtendedAttributes]
    }
    pub unsafe fn setAllowsExtendedAttributes(&self, allowsExtendedAttributes: bool) {
        msg_send![self, setAllowsExtendedAttributes: allowsExtendedAttributes]
    }
    pub unsafe fn interpretedSyntax(&self) -> NSAttributedStringMarkdownInterpretedSyntax {
        msg_send![self, interpretedSyntax]
    }
    pub unsafe fn setInterpretedSyntax(
        &self,
        interpretedSyntax: NSAttributedStringMarkdownInterpretedSyntax,
    ) {
        msg_send![self, setInterpretedSyntax: interpretedSyntax]
    }
    pub unsafe fn failurePolicy(&self) -> NSAttributedStringMarkdownParsingFailurePolicy {
        msg_send![self, failurePolicy]
    }
    pub unsafe fn setFailurePolicy(
        &self,
        failurePolicy: NSAttributedStringMarkdownParsingFailurePolicy,
    ) {
        msg_send![self, setFailurePolicy: failurePolicy]
    }
    pub unsafe fn languageCode(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, languageCode]
    }
    pub unsafe fn setLanguageCode(&self, languageCode: Option<&NSString>) {
        msg_send![self, setLanguageCode: languageCode]
    }
}
#[doc = "NSAttributedStringCreateFromMarkdown"]
impl NSAttributedString {
    pub unsafe fn initWithContentsOfMarkdownFileAtURL_options_baseURL_error(
        &self,
        markdownFile: &NSURL,
        options: Option<&NSAttributedStringMarkdownParsingOptions>,
        baseURL: Option<&NSURL>,
        error: *mut *mut NSError,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            self,
            initWithContentsOfMarkdownFileAtURL: markdownFile,
            options: options,
            baseURL: baseURL,
            error: error
        ]
    }
    pub unsafe fn initWithMarkdown_options_baseURL_error(
        &self,
        markdown: &NSData,
        options: Option<&NSAttributedStringMarkdownParsingOptions>,
        baseURL: Option<&NSURL>,
        error: *mut *mut NSError,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            self,
            initWithMarkdown: markdown,
            options: options,
            baseURL: baseURL,
            error: error
        ]
    }
    pub unsafe fn initWithMarkdownString_options_baseURL_error(
        &self,
        markdownString: &NSString,
        options: Option<&NSAttributedStringMarkdownParsingOptions>,
        baseURL: Option<&NSURL>,
        error: *mut *mut NSError,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            self,
            initWithMarkdownString: markdownString,
            options: options,
            baseURL: baseURL,
            error: error
        ]
    }
}
#[doc = "NSAttributedStringFormatting"]
impl NSAttributedString {
    pub unsafe fn initWithFormat_options_locale_arguments(
        &self,
        format: &NSAttributedString,
        options: NSAttributedStringFormattingOptions,
        locale: Option<&NSLocale>,
        arguments: va_list,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithFormat: format,
            options: options,
            locale: locale,
            arguments: arguments
        ]
    }
}
#[doc = "NSMutableAttributedStringFormatting"]
impl NSMutableAttributedString {}
#[doc = "NSMorphology"]
impl NSAttributedString {
    pub unsafe fn attributedStringByInflectingString(&self) -> Id<NSAttributedString, Shared> {
        msg_send_id![self, attributedStringByInflectingString]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSPresentationIntent;
    unsafe impl ClassType for NSPresentationIntent {
        type Super = NSObject;
    }
);
impl NSPresentationIntent {
    pub unsafe fn intentKind(&self) -> NSPresentationIntentKind {
        msg_send![self, intentKind]
    }
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn parentIntent(&self) -> Option<Id<NSPresentationIntent, Shared>> {
        msg_send_id![self, parentIntent]
    }
    pub unsafe fn paragraphIntentWithIdentity_nestedInsideIntent(
        identity: NSInteger,
        parent: Option<&NSPresentationIntent>,
    ) -> Id<NSPresentationIntent, Shared> {
        msg_send_id![
            Self::class(),
            paragraphIntentWithIdentity: identity,
            nestedInsideIntent: parent
        ]
    }
    pub unsafe fn headerIntentWithIdentity_level_nestedInsideIntent(
        identity: NSInteger,
        level: NSInteger,
        parent: Option<&NSPresentationIntent>,
    ) -> Id<NSPresentationIntent, Shared> {
        msg_send_id![
            Self::class(),
            headerIntentWithIdentity: identity,
            level: level,
            nestedInsideIntent: parent
        ]
    }
    pub unsafe fn codeBlockIntentWithIdentity_languageHint_nestedInsideIntent(
        identity: NSInteger,
        languageHint: Option<&NSString>,
        parent: Option<&NSPresentationIntent>,
    ) -> Id<NSPresentationIntent, Shared> {
        msg_send_id![
            Self::class(),
            codeBlockIntentWithIdentity: identity,
            languageHint: languageHint,
            nestedInsideIntent: parent
        ]
    }
    pub unsafe fn thematicBreakIntentWithIdentity_nestedInsideIntent(
        identity: NSInteger,
        parent: Option<&NSPresentationIntent>,
    ) -> Id<NSPresentationIntent, Shared> {
        msg_send_id![
            Self::class(),
            thematicBreakIntentWithIdentity: identity,
            nestedInsideIntent: parent
        ]
    }
    pub unsafe fn orderedListIntentWithIdentity_nestedInsideIntent(
        identity: NSInteger,
        parent: Option<&NSPresentationIntent>,
    ) -> Id<NSPresentationIntent, Shared> {
        msg_send_id![
            Self::class(),
            orderedListIntentWithIdentity: identity,
            nestedInsideIntent: parent
        ]
    }
    pub unsafe fn unorderedListIntentWithIdentity_nestedInsideIntent(
        identity: NSInteger,
        parent: Option<&NSPresentationIntent>,
    ) -> Id<NSPresentationIntent, Shared> {
        msg_send_id![
            Self::class(),
            unorderedListIntentWithIdentity: identity,
            nestedInsideIntent: parent
        ]
    }
    pub unsafe fn listItemIntentWithIdentity_ordinal_nestedInsideIntent(
        identity: NSInteger,
        ordinal: NSInteger,
        parent: Option<&NSPresentationIntent>,
    ) -> Id<NSPresentationIntent, Shared> {
        msg_send_id![
            Self::class(),
            listItemIntentWithIdentity: identity,
            ordinal: ordinal,
            nestedInsideIntent: parent
        ]
    }
    pub unsafe fn blockQuoteIntentWithIdentity_nestedInsideIntent(
        identity: NSInteger,
        parent: Option<&NSPresentationIntent>,
    ) -> Id<NSPresentationIntent, Shared> {
        msg_send_id![
            Self::class(),
            blockQuoteIntentWithIdentity: identity,
            nestedInsideIntent: parent
        ]
    }
    pub unsafe fn tableIntentWithIdentity_columnCount_alignments_nestedInsideIntent(
        identity: NSInteger,
        columnCount: NSInteger,
        alignments: &NSArray<NSNumber>,
        parent: Option<&NSPresentationIntent>,
    ) -> Id<NSPresentationIntent, Shared> {
        msg_send_id![
            Self::class(),
            tableIntentWithIdentity: identity,
            columnCount: columnCount,
            alignments: alignments,
            nestedInsideIntent: parent
        ]
    }
    pub unsafe fn tableHeaderRowIntentWithIdentity_nestedInsideIntent(
        identity: NSInteger,
        parent: Option<&NSPresentationIntent>,
    ) -> Id<NSPresentationIntent, Shared> {
        msg_send_id![
            Self::class(),
            tableHeaderRowIntentWithIdentity: identity,
            nestedInsideIntent: parent
        ]
    }
    pub unsafe fn tableRowIntentWithIdentity_row_nestedInsideIntent(
        identity: NSInteger,
        row: NSInteger,
        parent: Option<&NSPresentationIntent>,
    ) -> Id<NSPresentationIntent, Shared> {
        msg_send_id![
            Self::class(),
            tableRowIntentWithIdentity: identity,
            row: row,
            nestedInsideIntent: parent
        ]
    }
    pub unsafe fn tableCellIntentWithIdentity_column_nestedInsideIntent(
        identity: NSInteger,
        column: NSInteger,
        parent: Option<&NSPresentationIntent>,
    ) -> Id<NSPresentationIntent, Shared> {
        msg_send_id![
            Self::class(),
            tableCellIntentWithIdentity: identity,
            column: column,
            nestedInsideIntent: parent
        ]
    }
    pub unsafe fn identity(&self) -> NSInteger {
        msg_send![self, identity]
    }
    pub unsafe fn ordinal(&self) -> NSInteger {
        msg_send![self, ordinal]
    }
    pub unsafe fn columnAlignments(&self) -> Option<Id<NSArray<NSNumber>, Shared>> {
        msg_send_id![self, columnAlignments]
    }
    pub unsafe fn columnCount(&self) -> NSInteger {
        msg_send![self, columnCount]
    }
    pub unsafe fn headerLevel(&self) -> NSInteger {
        msg_send![self, headerLevel]
    }
    pub unsafe fn languageHint(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, languageHint]
    }
    pub unsafe fn column(&self) -> NSInteger {
        msg_send![self, column]
    }
    pub unsafe fn row(&self) -> NSInteger {
        msg_send![self, row]
    }
    pub unsafe fn indentationLevel(&self) -> NSInteger {
        msg_send![self, indentationLevel]
    }
    pub unsafe fn isEquivalentToPresentationIntent(&self, other: &NSPresentationIntent) -> bool {
        msg_send![self, isEquivalentToPresentationIntent: other]
    }
}
