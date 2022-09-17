#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    struct NSLinguisticTagger;
    unsafe impl ClassType for NSLinguisticTagger {
        type Super = NSObject;
    }
);
impl NSLinguisticTagger {
    pub unsafe fn initWithTagSchemes_options(
        &self,
        tagSchemes: TodoGenerics,
        opts: NSUInteger,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithTagSchemes: tagSchemes, options: opts]
    }
    pub unsafe fn availableTagSchemesForUnit_language(
        unit: NSLinguisticTaggerUnit,
        language: &NSString,
    ) -> TodoGenerics {
        msg_send![
            Self::class(),
            availableTagSchemesForUnit: unit,
            language: language
        ]
    }
    pub unsafe fn availableTagSchemesForLanguage(language: &NSString) -> TodoGenerics {
        msg_send![Self::class(), availableTagSchemesForLanguage: language]
    }
    pub unsafe fn setOrthography_range(&self, orthography: Option<&NSOrthography>, range: NSRange) {
        msg_send![self, setOrthography: orthography, range: range]
    }
    pub unsafe fn orthographyAtIndex_effectiveRange(
        &self,
        charIndex: NSUInteger,
        effectiveRange: NSRangePointer,
    ) -> Option<Id<NSOrthography, Shared>> {
        msg_send_id![
            self,
            orthographyAtIndex: charIndex,
            effectiveRange: effectiveRange
        ]
    }
    pub unsafe fn stringEditedInRange_changeInLength(&self, newRange: NSRange, delta: NSInteger) {
        msg_send![self, stringEditedInRange: newRange, changeInLength: delta]
    }
    pub unsafe fn tokenRangeAtIndex_unit(
        &self,
        charIndex: NSUInteger,
        unit: NSLinguisticTaggerUnit,
    ) -> NSRange {
        msg_send![self, tokenRangeAtIndex: charIndex, unit: unit]
    }
    pub unsafe fn sentenceRangeForRange(&self, range: NSRange) -> NSRange {
        msg_send![self, sentenceRangeForRange: range]
    }
    pub unsafe fn enumerateTagsInRange_unit_scheme_options_usingBlock(
        &self,
        range: NSRange,
        unit: NSLinguisticTaggerUnit,
        scheme: NSLinguisticTagScheme,
        options: NSLinguisticTaggerOptions,
        block: TodoBlock,
    ) {
        msg_send![
            self,
            enumerateTagsInRange: range,
            unit: unit,
            scheme: scheme,
            options: options,
            usingBlock: block
        ]
    }
    pub unsafe fn tagAtIndex_unit_scheme_tokenRange(
        &self,
        charIndex: NSUInteger,
        unit: NSLinguisticTaggerUnit,
        scheme: NSLinguisticTagScheme,
        tokenRange: NSRangePointer,
    ) -> NSLinguisticTag {
        msg_send![
            self,
            tagAtIndex: charIndex,
            unit: unit,
            scheme: scheme,
            tokenRange: tokenRange
        ]
    }
    pub unsafe fn tagsInRange_unit_scheme_options_tokenRanges(
        &self,
        range: NSRange,
        unit: NSLinguisticTaggerUnit,
        scheme: NSLinguisticTagScheme,
        options: NSLinguisticTaggerOptions,
        tokenRanges: *mut TodoGenerics,
    ) -> TodoGenerics {
        msg_send![
            self,
            tagsInRange: range,
            unit: unit,
            scheme: scheme,
            options: options,
            tokenRanges: tokenRanges
        ]
    }
    pub unsafe fn enumerateTagsInRange_scheme_options_usingBlock(
        &self,
        range: NSRange,
        tagScheme: NSLinguisticTagScheme,
        opts: NSLinguisticTaggerOptions,
        block: TodoBlock,
    ) {
        msg_send![
            self,
            enumerateTagsInRange: range,
            scheme: tagScheme,
            options: opts,
            usingBlock: block
        ]
    }
    pub unsafe fn tagAtIndex_scheme_tokenRange_sentenceRange(
        &self,
        charIndex: NSUInteger,
        scheme: NSLinguisticTagScheme,
        tokenRange: NSRangePointer,
        sentenceRange: NSRangePointer,
    ) -> NSLinguisticTag {
        msg_send![
            self,
            tagAtIndex: charIndex,
            scheme: scheme,
            tokenRange: tokenRange,
            sentenceRange: sentenceRange
        ]
    }
    pub unsafe fn tagsInRange_scheme_options_tokenRanges(
        &self,
        range: NSRange,
        tagScheme: &NSString,
        opts: NSLinguisticTaggerOptions,
        tokenRanges: *mut TodoGenerics,
    ) -> TodoGenerics {
        msg_send![
            self,
            tagsInRange: range,
            scheme: tagScheme,
            options: opts,
            tokenRanges: tokenRanges
        ]
    }
    pub unsafe fn dominantLanguageForString(string: &NSString) -> Option<Id<NSString, Shared>> {
        msg_send_id![Self::class(), dominantLanguageForString: string]
    }
    pub unsafe fn tagForString_atIndex_unit_scheme_orthography_tokenRange(
        string: &NSString,
        charIndex: NSUInteger,
        unit: NSLinguisticTaggerUnit,
        scheme: NSLinguisticTagScheme,
        orthography: Option<&NSOrthography>,
        tokenRange: NSRangePointer,
    ) -> NSLinguisticTag {
        msg_send![
            Self::class(),
            tagForString: string,
            atIndex: charIndex,
            unit: unit,
            scheme: scheme,
            orthography: orthography,
            tokenRange: tokenRange
        ]
    }
    pub unsafe fn tagsForString_range_unit_scheme_options_orthography_tokenRanges(
        string: &NSString,
        range: NSRange,
        unit: NSLinguisticTaggerUnit,
        scheme: NSLinguisticTagScheme,
        options: NSLinguisticTaggerOptions,
        orthography: Option<&NSOrthography>,
        tokenRanges: *mut TodoGenerics,
    ) -> TodoGenerics {
        msg_send![
            Self::class(),
            tagsForString: string,
            range: range,
            unit: unit,
            scheme: scheme,
            options: options,
            orthography: orthography,
            tokenRanges: tokenRanges
        ]
    }
    pub unsafe fn enumerateTagsForString_range_unit_scheme_options_orthography_usingBlock(
        string: &NSString,
        range: NSRange,
        unit: NSLinguisticTaggerUnit,
        scheme: NSLinguisticTagScheme,
        options: NSLinguisticTaggerOptions,
        orthography: Option<&NSOrthography>,
        block: TodoBlock,
    ) {
        msg_send![
            Self::class(),
            enumerateTagsForString: string,
            range: range,
            unit: unit,
            scheme: scheme,
            options: options,
            orthography: orthography,
            usingBlock: block
        ]
    }
    pub unsafe fn possibleTagsAtIndex_scheme_tokenRange_sentenceRange_scores(
        &self,
        charIndex: NSUInteger,
        tagScheme: &NSString,
        tokenRange: NSRangePointer,
        sentenceRange: NSRangePointer,
        scores: *mut TodoGenerics,
    ) -> TodoGenerics {
        msg_send![
            self,
            possibleTagsAtIndex: charIndex,
            scheme: tagScheme,
            tokenRange: tokenRange,
            sentenceRange: sentenceRange,
            scores: scores
        ]
    }
    pub unsafe fn tagSchemes(&self) -> TodoGenerics {
        msg_send![self, tagSchemes]
    }
    pub unsafe fn string(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, string]
    }
    pub unsafe fn setString(&self, string: Option<&NSString>) {
        msg_send![self, setString: string]
    }
    pub unsafe fn dominantLanguage(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, dominantLanguage]
    }
}
#[doc = "NSLinguisticAnalysis"]
impl NSString {
    pub unsafe fn linguisticTagsInRange_scheme_options_orthography_tokenRanges(
        &self,
        range: NSRange,
        scheme: NSLinguisticTagScheme,
        options: NSLinguisticTaggerOptions,
        orthography: Option<&NSOrthography>,
        tokenRanges: *mut TodoGenerics,
    ) -> TodoGenerics {
        msg_send![
            self,
            linguisticTagsInRange: range,
            scheme: scheme,
            options: options,
            orthography: orthography,
            tokenRanges: tokenRanges
        ]
    }
    pub unsafe fn enumerateLinguisticTagsInRange_scheme_options_orthography_usingBlock(
        &self,
        range: NSRange,
        scheme: NSLinguisticTagScheme,
        options: NSLinguisticTaggerOptions,
        orthography: Option<&NSOrthography>,
        block: TodoBlock,
    ) {
        msg_send![
            self,
            enumerateLinguisticTagsInRange: range,
            scheme: scheme,
            options: options,
            orthography: orthography,
            usingBlock: block
        ]
    }
}
