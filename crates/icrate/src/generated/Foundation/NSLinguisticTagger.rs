//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

pub type NSLinguisticTagScheme = NSString;

extern_static!(NSLinguisticTagSchemeTokenType: &'static NSLinguisticTagScheme);

extern_static!(NSLinguisticTagSchemeLexicalClass: &'static NSLinguisticTagScheme);

extern_static!(NSLinguisticTagSchemeNameType: &'static NSLinguisticTagScheme);

extern_static!(NSLinguisticTagSchemeNameTypeOrLexicalClass: &'static NSLinguisticTagScheme);

extern_static!(NSLinguisticTagSchemeLemma: &'static NSLinguisticTagScheme);

extern_static!(NSLinguisticTagSchemeLanguage: &'static NSLinguisticTagScheme);

extern_static!(NSLinguisticTagSchemeScript: &'static NSLinguisticTagScheme);

pub type NSLinguisticTag = NSString;

extern_static!(NSLinguisticTagWord: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagPunctuation: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagWhitespace: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagOther: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagNoun: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagVerb: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagAdjective: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagAdverb: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagPronoun: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagDeterminer: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagParticle: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagPreposition: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagNumber: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagConjunction: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagInterjection: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagClassifier: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagIdiom: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagOtherWord: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagSentenceTerminator: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagOpenQuote: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagCloseQuote: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagOpenParenthesis: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagCloseParenthesis: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagWordJoiner: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagDash: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagOtherPunctuation: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagParagraphBreak: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagOtherWhitespace: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagPersonalName: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagPlaceName: &'static NSLinguisticTag);

extern_static!(NSLinguisticTagOrganizationName: &'static NSLinguisticTag);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSLinguisticTaggerUnit {
        NSLinguisticTaggerUnitWord = 0,
        NSLinguisticTaggerUnitSentence = 1,
        NSLinguisticTaggerUnitParagraph = 2,
        NSLinguisticTaggerUnitDocument = 3,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSLinguisticTaggerOptions {
        NSLinguisticTaggerOmitWords = 1 << 0,
        NSLinguisticTaggerOmitPunctuation = 1 << 1,
        NSLinguisticTaggerOmitWhitespace = 1 << 2,
        NSLinguisticTaggerOmitOther = 1 << 3,
        NSLinguisticTaggerJoinNames = 1 << 4,
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSLinguisticTagger;

    unsafe impl ClassType for NSLinguisticTagger {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSLinguisticTagger {
        #[method_id(@__retain_semantics Init initWithTagSchemes:options:)]
        pub unsafe fn initWithTagSchemes_options(
            this: Option<Allocated<Self>>,
            tagSchemes: &NSArray<NSLinguisticTagScheme>,
            opts: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other tagSchemes)]
        pub unsafe fn tagSchemes(&self) -> Id<NSArray<NSLinguisticTagScheme>, Shared>;

        #[method_id(@__retain_semantics Other string)]
        pub unsafe fn string(&self) -> Option<Id<NSString, Shared>>;

        #[method(setString:)]
        pub unsafe fn setString(&self, string: Option<&NSString>);

        #[method_id(@__retain_semantics Other availableTagSchemesForUnit:language:)]
        pub unsafe fn availableTagSchemesForUnit_language(
            unit: NSLinguisticTaggerUnit,
            language: &NSString,
        ) -> Id<NSArray<NSLinguisticTagScheme>, Shared>;

        #[method_id(@__retain_semantics Other availableTagSchemesForLanguage:)]
        pub unsafe fn availableTagSchemesForLanguage(
            language: &NSString,
        ) -> Id<NSArray<NSLinguisticTagScheme>, Shared>;

        #[method(setOrthography:range:)]
        pub unsafe fn setOrthography_range(
            &self,
            orthography: Option<&NSOrthography>,
            range: NSRange,
        );

        #[method_id(@__retain_semantics Other orthographyAtIndex:effectiveRange:)]
        pub unsafe fn orthographyAtIndex_effectiveRange(
            &self,
            charIndex: NSUInteger,
            effectiveRange: NSRangePointer,
        ) -> Option<Id<NSOrthography, Shared>>;

        #[method(stringEditedInRange:changeInLength:)]
        pub unsafe fn stringEditedInRange_changeInLength(
            &self,
            newRange: NSRange,
            delta: NSInteger,
        );

        #[method(tokenRangeAtIndex:unit:)]
        pub unsafe fn tokenRangeAtIndex_unit(
            &self,
            charIndex: NSUInteger,
            unit: NSLinguisticTaggerUnit,
        ) -> NSRange;

        #[method(sentenceRangeForRange:)]
        pub unsafe fn sentenceRangeForRange(&self, range: NSRange) -> NSRange;

        #[method(enumerateTagsInRange:unit:scheme:options:usingBlock:)]
        pub unsafe fn enumerateTagsInRange_unit_scheme_options_usingBlock(
            &self,
            range: NSRange,
            unit: NSLinguisticTaggerUnit,
            scheme: &NSLinguisticTagScheme,
            options: NSLinguisticTaggerOptions,
            block: TodoBlock,
        );

        #[method_id(@__retain_semantics Other tagAtIndex:unit:scheme:tokenRange:)]
        pub unsafe fn tagAtIndex_unit_scheme_tokenRange(
            &self,
            charIndex: NSUInteger,
            unit: NSLinguisticTaggerUnit,
            scheme: &NSLinguisticTagScheme,
            tokenRange: NSRangePointer,
        ) -> Option<Id<NSLinguisticTag, Shared>>;

        #[method_id(@__retain_semantics Other tagsInRange:unit:scheme:options:tokenRanges:)]
        pub unsafe fn tagsInRange_unit_scheme_options_tokenRanges(
            &self,
            range: NSRange,
            unit: NSLinguisticTaggerUnit,
            scheme: &NSLinguisticTagScheme,
            options: NSLinguisticTaggerOptions,
            tokenRanges: *mut *mut NSArray<NSValue>,
        ) -> Id<NSArray<NSLinguisticTag>, Shared>;

        #[method(enumerateTagsInRange:scheme:options:usingBlock:)]
        pub unsafe fn enumerateTagsInRange_scheme_options_usingBlock(
            &self,
            range: NSRange,
            tagScheme: &NSLinguisticTagScheme,
            opts: NSLinguisticTaggerOptions,
            block: TodoBlock,
        );

        #[method_id(@__retain_semantics Other tagAtIndex:scheme:tokenRange:sentenceRange:)]
        pub unsafe fn tagAtIndex_scheme_tokenRange_sentenceRange(
            &self,
            charIndex: NSUInteger,
            scheme: &NSLinguisticTagScheme,
            tokenRange: NSRangePointer,
            sentenceRange: NSRangePointer,
        ) -> Option<Id<NSLinguisticTag, Shared>>;

        #[method_id(@__retain_semantics Other tagsInRange:scheme:options:tokenRanges:)]
        pub unsafe fn tagsInRange_scheme_options_tokenRanges(
            &self,
            range: NSRange,
            tagScheme: &NSString,
            opts: NSLinguisticTaggerOptions,
            tokenRanges: *mut *mut NSArray<NSValue>,
        ) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other dominantLanguage)]
        pub unsafe fn dominantLanguage(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other dominantLanguageForString:)]
        pub unsafe fn dominantLanguageForString(string: &NSString) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other tagForString:atIndex:unit:scheme:orthography:tokenRange:)]
        pub unsafe fn tagForString_atIndex_unit_scheme_orthography_tokenRange(
            string: &NSString,
            charIndex: NSUInteger,
            unit: NSLinguisticTaggerUnit,
            scheme: &NSLinguisticTagScheme,
            orthography: Option<&NSOrthography>,
            tokenRange: NSRangePointer,
        ) -> Option<Id<NSLinguisticTag, Shared>>;

        #[method_id(@__retain_semantics Other tagsForString:range:unit:scheme:options:orthography:tokenRanges:)]
        pub unsafe fn tagsForString_range_unit_scheme_options_orthography_tokenRanges(
            string: &NSString,
            range: NSRange,
            unit: NSLinguisticTaggerUnit,
            scheme: &NSLinguisticTagScheme,
            options: NSLinguisticTaggerOptions,
            orthography: Option<&NSOrthography>,
            tokenRanges: *mut *mut NSArray<NSValue>,
        ) -> Id<NSArray<NSLinguisticTag>, Shared>;

        #[method(enumerateTagsForString:range:unit:scheme:options:orthography:usingBlock:)]
        pub unsafe fn enumerateTagsForString_range_unit_scheme_options_orthography_usingBlock(
            string: &NSString,
            range: NSRange,
            unit: NSLinguisticTaggerUnit,
            scheme: &NSLinguisticTagScheme,
            options: NSLinguisticTaggerOptions,
            orthography: Option<&NSOrthography>,
            block: TodoBlock,
        );

        #[method_id(@__retain_semantics Other possibleTagsAtIndex:scheme:tokenRange:sentenceRange:scores:)]
        pub unsafe fn possibleTagsAtIndex_scheme_tokenRange_sentenceRange_scores(
            &self,
            charIndex: NSUInteger,
            tagScheme: &NSString,
            tokenRange: NSRangePointer,
            sentenceRange: NSRangePointer,
            scores: *mut *mut NSArray<NSValue>,
        ) -> Option<Id<NSArray<NSString>, Shared>>;
    }
);

extern_methods!(
    /// NSLinguisticAnalysis
    unsafe impl NSString {
        #[method_id(@__retain_semantics Other linguisticTagsInRange:scheme:options:orthography:tokenRanges:)]
        pub unsafe fn linguisticTagsInRange_scheme_options_orthography_tokenRanges(
            &self,
            range: NSRange,
            scheme: &NSLinguisticTagScheme,
            options: NSLinguisticTaggerOptions,
            orthography: Option<&NSOrthography>,
            tokenRanges: *mut *mut NSArray<NSValue>,
        ) -> Id<NSArray<NSLinguisticTag>, Shared>;

        #[method(enumerateLinguisticTagsInRange:scheme:options:orthography:usingBlock:)]
        pub unsafe fn enumerateLinguisticTagsInRange_scheme_options_orthography_usingBlock(
            &self,
            range: NSRange,
            scheme: &NSLinguisticTagScheme,
            options: NSLinguisticTaggerOptions,
            orthography: Option<&NSOrthography>,
            block: TodoBlock,
        );
    }
);
