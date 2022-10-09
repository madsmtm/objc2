use super::__exported::NSArray;
use super::__exported::NSOrthography;
use super::__exported::NSValue;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSString::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSLinguisticTagScheme = NSString;
pub type NSLinguisticTag = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSLinguisticTagger;
    unsafe impl ClassType for NSLinguisticTagger {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSLinguisticTagger {
        #[method_id(initWithTagSchemes:options:)]
        pub unsafe fn initWithTagSchemes_options(
            &self,
            tagSchemes: &NSArray<NSLinguisticTagScheme>,
            opts: NSUInteger,
        ) -> Id<Self, Shared>;
        #[method_id(tagSchemes)]
        pub unsafe fn tagSchemes(&self) -> Id<NSArray<NSLinguisticTagScheme>, Shared>;
        #[method_id(string)]
        pub unsafe fn string(&self) -> Option<Id<NSString, Shared>>;
        #[method(setString:)]
        pub unsafe fn setString(&self, string: Option<&NSString>);
        #[method_id(availableTagSchemesForUnit:language:)]
        pub unsafe fn availableTagSchemesForUnit_language(
            unit: NSLinguisticTaggerUnit,
            language: &NSString,
        ) -> Id<NSArray<NSLinguisticTagScheme>, Shared>;
        #[method_id(availableTagSchemesForLanguage:)]
        pub unsafe fn availableTagSchemesForLanguage(
            language: &NSString,
        ) -> Id<NSArray<NSLinguisticTagScheme>, Shared>;
        #[method(setOrthography:range:)]
        pub unsafe fn setOrthography_range(
            &self,
            orthography: Option<&NSOrthography>,
            range: NSRange,
        );
        #[method_id(orthographyAtIndex:effectiveRange:)]
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
        #[method_id(tagAtIndex:unit:scheme:tokenRange:)]
        pub unsafe fn tagAtIndex_unit_scheme_tokenRange(
            &self,
            charIndex: NSUInteger,
            unit: NSLinguisticTaggerUnit,
            scheme: &NSLinguisticTagScheme,
            tokenRange: NSRangePointer,
        ) -> Option<Id<NSLinguisticTag, Shared>>;
        #[method_id(tagsInRange:unit:scheme:options:tokenRanges:)]
        pub unsafe fn tagsInRange_unit_scheme_options_tokenRanges(
            &self,
            range: NSRange,
            unit: NSLinguisticTaggerUnit,
            scheme: &NSLinguisticTagScheme,
            options: NSLinguisticTaggerOptions,
            tokenRanges: Option<&mut Option<Id<NSArray<NSValue>, Shared>>>,
        ) -> Id<NSArray<NSLinguisticTag>, Shared>;
        #[method(enumerateTagsInRange:scheme:options:usingBlock:)]
        pub unsafe fn enumerateTagsInRange_scheme_options_usingBlock(
            &self,
            range: NSRange,
            tagScheme: &NSLinguisticTagScheme,
            opts: NSLinguisticTaggerOptions,
            block: TodoBlock,
        );
        #[method_id(tagAtIndex:scheme:tokenRange:sentenceRange:)]
        pub unsafe fn tagAtIndex_scheme_tokenRange_sentenceRange(
            &self,
            charIndex: NSUInteger,
            scheme: &NSLinguisticTagScheme,
            tokenRange: NSRangePointer,
            sentenceRange: NSRangePointer,
        ) -> Option<Id<NSLinguisticTag, Shared>>;
        #[method_id(tagsInRange:scheme:options:tokenRanges:)]
        pub unsafe fn tagsInRange_scheme_options_tokenRanges(
            &self,
            range: NSRange,
            tagScheme: &NSString,
            opts: NSLinguisticTaggerOptions,
            tokenRanges: Option<&mut Option<Id<NSArray<NSValue>, Shared>>>,
        ) -> Id<NSArray<NSString>, Shared>;
        #[method_id(dominantLanguage)]
        pub unsafe fn dominantLanguage(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(dominantLanguageForString:)]
        pub unsafe fn dominantLanguageForString(string: &NSString) -> Option<Id<NSString, Shared>>;
        #[method_id(tagForString:atIndex:unit:scheme:orthography:tokenRange:)]
        pub unsafe fn tagForString_atIndex_unit_scheme_orthography_tokenRange(
            string: &NSString,
            charIndex: NSUInteger,
            unit: NSLinguisticTaggerUnit,
            scheme: &NSLinguisticTagScheme,
            orthography: Option<&NSOrthography>,
            tokenRange: NSRangePointer,
        ) -> Option<Id<NSLinguisticTag, Shared>>;
        #[method_id(tagsForString:range:unit:scheme:options:orthography:tokenRanges:)]
        pub unsafe fn tagsForString_range_unit_scheme_options_orthography_tokenRanges(
            string: &NSString,
            range: NSRange,
            unit: NSLinguisticTaggerUnit,
            scheme: &NSLinguisticTagScheme,
            options: NSLinguisticTaggerOptions,
            orthography: Option<&NSOrthography>,
            tokenRanges: Option<&mut Option<Id<NSArray<NSValue>, Shared>>>,
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
        #[method_id(possibleTagsAtIndex:scheme:tokenRange:sentenceRange:scores:)]
        pub unsafe fn possibleTagsAtIndex_scheme_tokenRange_sentenceRange_scores(
            &self,
            charIndex: NSUInteger,
            tagScheme: &NSString,
            tokenRange: NSRangePointer,
            sentenceRange: NSRangePointer,
            scores: Option<&mut Option<Id<NSArray<NSValue>, Shared>>>,
        ) -> Option<Id<NSArray<NSString>, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSLinguisticAnalysis"]
    unsafe impl NSString {
        #[method_id(linguisticTagsInRange:scheme:options:orthography:tokenRanges:)]
        pub unsafe fn linguisticTagsInRange_scheme_options_orthography_tokenRanges(
            &self,
            range: NSRange,
            scheme: &NSLinguisticTagScheme,
            options: NSLinguisticTaggerOptions,
            orthography: Option<&NSOrthography>,
            tokenRanges: Option<&mut Option<Id<NSArray<NSValue>, Shared>>>,
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
