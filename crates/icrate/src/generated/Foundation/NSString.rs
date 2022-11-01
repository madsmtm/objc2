//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

pub type NSStringCompareOptions = NSUInteger;
pub const NSCaseInsensitiveSearch: NSStringCompareOptions = 1;
pub const NSLiteralSearch: NSStringCompareOptions = 2;
pub const NSBackwardsSearch: NSStringCompareOptions = 4;
pub const NSAnchoredSearch: NSStringCompareOptions = 8;
pub const NSNumericSearch: NSStringCompareOptions = 64;
pub const NSDiacriticInsensitiveSearch: NSStringCompareOptions = 128;
pub const NSWidthInsensitiveSearch: NSStringCompareOptions = 256;
pub const NSForcedOrderingSearch: NSStringCompareOptions = 512;
pub const NSRegularExpressionSearch: NSStringCompareOptions = 1024;

pub type NSStringEncoding = NSUInteger;

pub const NSASCIIStringEncoding: i32 = 1;
pub const NSNEXTSTEPStringEncoding: i32 = 2;
pub const NSJapaneseEUCStringEncoding: i32 = 3;
pub const NSUTF8StringEncoding: i32 = 4;
pub const NSISOLatin1StringEncoding: i32 = 5;
pub const NSSymbolStringEncoding: i32 = 6;
pub const NSNonLossyASCIIStringEncoding: i32 = 7;
pub const NSShiftJISStringEncoding: i32 = 8;
pub const NSISOLatin2StringEncoding: i32 = 9;
pub const NSUnicodeStringEncoding: i32 = 10;
pub const NSWindowsCP1251StringEncoding: i32 = 11;
pub const NSWindowsCP1252StringEncoding: i32 = 12;
pub const NSWindowsCP1253StringEncoding: i32 = 13;
pub const NSWindowsCP1254StringEncoding: i32 = 14;
pub const NSWindowsCP1250StringEncoding: i32 = 15;
pub const NSISO2022JPStringEncoding: i32 = 21;
pub const NSMacOSRomanStringEncoding: i32 = 30;
pub const NSUTF16StringEncoding: i32 = NSUnicodeStringEncoding;
pub const NSUTF16BigEndianStringEncoding: i32 = 0x90000100;
pub const NSUTF16LittleEndianStringEncoding: i32 = 0x94000100;
pub const NSUTF32StringEncoding: i32 = 0x8c000100;
pub const NSUTF32BigEndianStringEncoding: i32 = 0x98000100;
pub const NSUTF32LittleEndianStringEncoding: i32 = 0x9c000100;

pub type NSStringEncodingConversionOptions = NSUInteger;
pub const NSStringEncodingConversionAllowLossy: NSStringEncodingConversionOptions = 1;
pub const NSStringEncodingConversionExternalRepresentation: NSStringEncodingConversionOptions = 2;

extern_class!(
    #[derive(Debug)]
    pub struct NSString;

    unsafe impl ClassType for NSString {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSString {
        #[method(length)]
        pub fn length(&self) -> NSUInteger;

        #[method(characterAtIndex:)]
        pub unsafe fn characterAtIndex(&self, index: NSUInteger) -> unichar;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;
    }
);

pub type NSStringEnumerationOptions = NSUInteger;
pub const NSStringEnumerationByLines: NSStringEnumerationOptions = 0;
pub const NSStringEnumerationByParagraphs: NSStringEnumerationOptions = 1;
pub const NSStringEnumerationByComposedCharacterSequences: NSStringEnumerationOptions = 2;
pub const NSStringEnumerationByWords: NSStringEnumerationOptions = 3;
pub const NSStringEnumerationBySentences: NSStringEnumerationOptions = 4;
pub const NSStringEnumerationByCaretPositions: NSStringEnumerationOptions = 5;
pub const NSStringEnumerationByDeletionClusters: NSStringEnumerationOptions = 6;
pub const NSStringEnumerationReverse: NSStringEnumerationOptions = 1 << 8;
pub const NSStringEnumerationSubstringNotRequired: NSStringEnumerationOptions = 1 << 9;
pub const NSStringEnumerationLocalized: NSStringEnumerationOptions = 1 << 10;

pub type NSStringTransform = NSString;

extern "C" {
    pub static NSStringTransformLatinToKatakana: &'static NSStringTransform;
}

extern "C" {
    pub static NSStringTransformLatinToHiragana: &'static NSStringTransform;
}

extern "C" {
    pub static NSStringTransformLatinToHangul: &'static NSStringTransform;
}

extern "C" {
    pub static NSStringTransformLatinToArabic: &'static NSStringTransform;
}

extern "C" {
    pub static NSStringTransformLatinToHebrew: &'static NSStringTransform;
}

extern "C" {
    pub static NSStringTransformLatinToThai: &'static NSStringTransform;
}

extern "C" {
    pub static NSStringTransformLatinToCyrillic: &'static NSStringTransform;
}

extern "C" {
    pub static NSStringTransformLatinToGreek: &'static NSStringTransform;
}

extern "C" {
    pub static NSStringTransformToLatin: &'static NSStringTransform;
}

extern "C" {
    pub static NSStringTransformMandarinToLatin: &'static NSStringTransform;
}

extern "C" {
    pub static NSStringTransformHiraganaToKatakana: &'static NSStringTransform;
}

extern "C" {
    pub static NSStringTransformFullwidthToHalfwidth: &'static NSStringTransform;
}

extern "C" {
    pub static NSStringTransformToXMLHex: &'static NSStringTransform;
}

extern "C" {
    pub static NSStringTransformToUnicodeName: &'static NSStringTransform;
}

extern "C" {
    pub static NSStringTransformStripCombiningMarks: &'static NSStringTransform;
}

extern "C" {
    pub static NSStringTransformStripDiacritics: &'static NSStringTransform;
}

extern_methods!(
    /// NSStringExtensionMethods
    unsafe impl NSString {
        #[method_id(@__retain_semantics Other substringFromIndex:)]
        pub unsafe fn substringFromIndex(&self, from: NSUInteger) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other substringToIndex:)]
        pub unsafe fn substringToIndex(&self, to: NSUInteger) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other substringWithRange:)]
        pub unsafe fn substringWithRange(&self, range: NSRange) -> Id<NSString, Shared>;

        #[method(getCharacters:range:)]
        pub unsafe fn getCharacters_range(&self, buffer: NonNull<unichar>, range: NSRange);

        #[method(compare:)]
        pub unsafe fn compare(&self, string: &NSString) -> NSComparisonResult;

        #[method(compare:options:)]
        pub unsafe fn compare_options(
            &self,
            string: &NSString,
            mask: NSStringCompareOptions,
        ) -> NSComparisonResult;

        #[method(compare:options:range:)]
        pub unsafe fn compare_options_range(
            &self,
            string: &NSString,
            mask: NSStringCompareOptions,
            rangeOfReceiverToCompare: NSRange,
        ) -> NSComparisonResult;

        #[method(compare:options:range:locale:)]
        pub unsafe fn compare_options_range_locale(
            &self,
            string: &NSString,
            mask: NSStringCompareOptions,
            rangeOfReceiverToCompare: NSRange,
            locale: Option<&Object>,
        ) -> NSComparisonResult;

        #[method(caseInsensitiveCompare:)]
        pub unsafe fn caseInsensitiveCompare(&self, string: &NSString) -> NSComparisonResult;

        #[method(localizedCompare:)]
        pub unsafe fn localizedCompare(&self, string: &NSString) -> NSComparisonResult;

        #[method(localizedCaseInsensitiveCompare:)]
        pub unsafe fn localizedCaseInsensitiveCompare(
            &self,
            string: &NSString,
        ) -> NSComparisonResult;

        #[method(localizedStandardCompare:)]
        pub unsafe fn localizedStandardCompare(&self, string: &NSString) -> NSComparisonResult;

        #[method(isEqualToString:)]
        pub unsafe fn isEqualToString(&self, aString: &NSString) -> bool;

        #[method(hasPrefix:)]
        pub unsafe fn hasPrefix(&self, str: &NSString) -> bool;

        #[method(hasSuffix:)]
        pub unsafe fn hasSuffix(&self, str: &NSString) -> bool;

        #[method_id(@__retain_semantics Other commonPrefixWithString:options:)]
        pub unsafe fn commonPrefixWithString_options(
            &self,
            str: &NSString,
            mask: NSStringCompareOptions,
        ) -> Id<NSString, Shared>;

        #[method(containsString:)]
        pub unsafe fn containsString(&self, str: &NSString) -> bool;

        #[method(localizedCaseInsensitiveContainsString:)]
        pub unsafe fn localizedCaseInsensitiveContainsString(&self, str: &NSString) -> bool;

        #[method(localizedStandardContainsString:)]
        pub unsafe fn localizedStandardContainsString(&self, str: &NSString) -> bool;

        #[method(localizedStandardRangeOfString:)]
        pub unsafe fn localizedStandardRangeOfString(&self, str: &NSString) -> NSRange;

        #[method(rangeOfString:)]
        pub unsafe fn rangeOfString(&self, searchString: &NSString) -> NSRange;

        #[method(rangeOfString:options:)]
        pub unsafe fn rangeOfString_options(
            &self,
            searchString: &NSString,
            mask: NSStringCompareOptions,
        ) -> NSRange;

        #[method(rangeOfString:options:range:)]
        pub unsafe fn rangeOfString_options_range(
            &self,
            searchString: &NSString,
            mask: NSStringCompareOptions,
            rangeOfReceiverToSearch: NSRange,
        ) -> NSRange;

        #[method(rangeOfString:options:range:locale:)]
        pub unsafe fn rangeOfString_options_range_locale(
            &self,
            searchString: &NSString,
            mask: NSStringCompareOptions,
            rangeOfReceiverToSearch: NSRange,
            locale: Option<&NSLocale>,
        ) -> NSRange;

        #[method(rangeOfCharacterFromSet:)]
        pub unsafe fn rangeOfCharacterFromSet(&self, searchSet: &NSCharacterSet) -> NSRange;

        #[method(rangeOfCharacterFromSet:options:)]
        pub unsafe fn rangeOfCharacterFromSet_options(
            &self,
            searchSet: &NSCharacterSet,
            mask: NSStringCompareOptions,
        ) -> NSRange;

        #[method(rangeOfCharacterFromSet:options:range:)]
        pub unsafe fn rangeOfCharacterFromSet_options_range(
            &self,
            searchSet: &NSCharacterSet,
            mask: NSStringCompareOptions,
            rangeOfReceiverToSearch: NSRange,
        ) -> NSRange;

        #[method(rangeOfComposedCharacterSequenceAtIndex:)]
        pub unsafe fn rangeOfComposedCharacterSequenceAtIndex(&self, index: NSUInteger) -> NSRange;

        #[method(rangeOfComposedCharacterSequencesForRange:)]
        pub unsafe fn rangeOfComposedCharacterSequencesForRange(&self, range: NSRange) -> NSRange;

        #[method_id(@__retain_semantics Other stringByAppendingString:)]
        pub fn stringByAppendingString(&self, aString: &NSString) -> Id<NSString, Shared>;

        #[method(doubleValue)]
        pub unsafe fn doubleValue(&self) -> c_double;

        #[method(floatValue)]
        pub unsafe fn floatValue(&self) -> c_float;

        #[method(intValue)]
        pub unsafe fn intValue(&self) -> c_int;

        #[method(integerValue)]
        pub unsafe fn integerValue(&self) -> NSInteger;

        #[method(longLongValue)]
        pub unsafe fn longLongValue(&self) -> c_longlong;

        #[method(boolValue)]
        pub unsafe fn boolValue(&self) -> bool;

        #[method_id(@__retain_semantics Other uppercaseString)]
        pub unsafe fn uppercaseString(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other lowercaseString)]
        pub unsafe fn lowercaseString(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other capitalizedString)]
        pub unsafe fn capitalizedString(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other localizedUppercaseString)]
        pub unsafe fn localizedUppercaseString(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other localizedLowercaseString)]
        pub unsafe fn localizedLowercaseString(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other localizedCapitalizedString)]
        pub unsafe fn localizedCapitalizedString(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other uppercaseStringWithLocale:)]
        pub unsafe fn uppercaseStringWithLocale(
            &self,
            locale: Option<&NSLocale>,
        ) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other lowercaseStringWithLocale:)]
        pub unsafe fn lowercaseStringWithLocale(
            &self,
            locale: Option<&NSLocale>,
        ) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other capitalizedStringWithLocale:)]
        pub unsafe fn capitalizedStringWithLocale(
            &self,
            locale: Option<&NSLocale>,
        ) -> Id<NSString, Shared>;

        #[method(getLineStart:end:contentsEnd:forRange:)]
        pub unsafe fn getLineStart_end_contentsEnd_forRange(
            &self,
            startPtr: *mut NSUInteger,
            lineEndPtr: *mut NSUInteger,
            contentsEndPtr: *mut NSUInteger,
            range: NSRange,
        );

        #[method(lineRangeForRange:)]
        pub unsafe fn lineRangeForRange(&self, range: NSRange) -> NSRange;

        #[method(getParagraphStart:end:contentsEnd:forRange:)]
        pub unsafe fn getParagraphStart_end_contentsEnd_forRange(
            &self,
            startPtr: *mut NSUInteger,
            parEndPtr: *mut NSUInteger,
            contentsEndPtr: *mut NSUInteger,
            range: NSRange,
        );

        #[method(paragraphRangeForRange:)]
        pub unsafe fn paragraphRangeForRange(&self, range: NSRange) -> NSRange;

        #[method(enumerateSubstringsInRange:options:usingBlock:)]
        pub unsafe fn enumerateSubstringsInRange_options_usingBlock(
            &self,
            range: NSRange,
            opts: NSStringEnumerationOptions,
            block: TodoBlock,
        );

        #[method(enumerateLinesUsingBlock:)]
        pub unsafe fn enumerateLinesUsingBlock(&self, block: TodoBlock);

        #[method(UTF8String)]
        pub fn UTF8String(&self) -> *mut c_char;

        #[method(fastestEncoding)]
        pub unsafe fn fastestEncoding(&self) -> NSStringEncoding;

        #[method(smallestEncoding)]
        pub unsafe fn smallestEncoding(&self) -> NSStringEncoding;

        #[method_id(@__retain_semantics Other dataUsingEncoding:allowLossyConversion:)]
        pub unsafe fn dataUsingEncoding_allowLossyConversion(
            &self,
            encoding: NSStringEncoding,
            lossy: bool,
        ) -> Option<Id<NSData, Shared>>;

        #[method_id(@__retain_semantics Other dataUsingEncoding:)]
        pub unsafe fn dataUsingEncoding(
            &self,
            encoding: NSStringEncoding,
        ) -> Option<Id<NSData, Shared>>;

        #[method(canBeConvertedToEncoding:)]
        pub unsafe fn canBeConvertedToEncoding(&self, encoding: NSStringEncoding) -> bool;

        #[method(cStringUsingEncoding:)]
        pub unsafe fn cStringUsingEncoding(&self, encoding: NSStringEncoding) -> *mut c_char;

        #[method(getCString:maxLength:encoding:)]
        pub unsafe fn getCString_maxLength_encoding(
            &self,
            buffer: NonNull<c_char>,
            maxBufferCount: NSUInteger,
            encoding: NSStringEncoding,
        ) -> bool;

        #[method(getBytes:maxLength:usedLength:encoding:options:range:remainingRange:)]
        pub unsafe fn getBytes_maxLength_usedLength_encoding_options_range_remainingRange(
            &self,
            buffer: *mut c_void,
            maxBufferCount: NSUInteger,
            usedBufferCount: *mut NSUInteger,
            encoding: NSStringEncoding,
            options: NSStringEncodingConversionOptions,
            range: NSRange,
            leftover: NSRangePointer,
        ) -> bool;

        #[method(maximumLengthOfBytesUsingEncoding:)]
        pub unsafe fn maximumLengthOfBytesUsingEncoding(&self, enc: NSStringEncoding)
            -> NSUInteger;

        #[method(lengthOfBytesUsingEncoding:)]
        pub fn lengthOfBytesUsingEncoding(&self, enc: NSStringEncoding) -> NSUInteger;

        #[method(availableStringEncodings)]
        pub unsafe fn availableStringEncodings() -> NonNull<NSStringEncoding>;

        #[method_id(@__retain_semantics Other localizedNameOfStringEncoding:)]
        pub unsafe fn localizedNameOfStringEncoding(
            encoding: NSStringEncoding,
        ) -> Id<NSString, Shared>;

        #[method(defaultCStringEncoding)]
        pub unsafe fn defaultCStringEncoding() -> NSStringEncoding;

        #[method_id(@__retain_semantics Other decomposedStringWithCanonicalMapping)]
        pub unsafe fn decomposedStringWithCanonicalMapping(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other precomposedStringWithCanonicalMapping)]
        pub unsafe fn precomposedStringWithCanonicalMapping(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other decomposedStringWithCompatibilityMapping)]
        pub unsafe fn decomposedStringWithCompatibilityMapping(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other precomposedStringWithCompatibilityMapping)]
        pub unsafe fn precomposedStringWithCompatibilityMapping(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other componentsSeparatedByString:)]
        pub unsafe fn componentsSeparatedByString(
            &self,
            separator: &NSString,
        ) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other componentsSeparatedByCharactersInSet:)]
        pub unsafe fn componentsSeparatedByCharactersInSet(
            &self,
            separator: &NSCharacterSet,
        ) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other stringByTrimmingCharactersInSet:)]
        pub unsafe fn stringByTrimmingCharactersInSet(
            &self,
            set: &NSCharacterSet,
        ) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other stringByPaddingToLength:withString:startingAtIndex:)]
        pub unsafe fn stringByPaddingToLength_withString_startingAtIndex(
            &self,
            newLength: NSUInteger,
            padString: &NSString,
            padIndex: NSUInteger,
        ) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other stringByFoldingWithOptions:locale:)]
        pub unsafe fn stringByFoldingWithOptions_locale(
            &self,
            options: NSStringCompareOptions,
            locale: Option<&NSLocale>,
        ) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other stringByReplacingOccurrencesOfString:withString:options:range:)]
        pub unsafe fn stringByReplacingOccurrencesOfString_withString_options_range(
            &self,
            target: &NSString,
            replacement: &NSString,
            options: NSStringCompareOptions,
            searchRange: NSRange,
        ) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other stringByReplacingOccurrencesOfString:withString:)]
        pub unsafe fn stringByReplacingOccurrencesOfString_withString(
            &self,
            target: &NSString,
            replacement: &NSString,
        ) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other stringByReplacingCharactersInRange:withString:)]
        pub unsafe fn stringByReplacingCharactersInRange_withString(
            &self,
            range: NSRange,
            replacement: &NSString,
        ) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other stringByApplyingTransform:reverse:)]
        pub unsafe fn stringByApplyingTransform_reverse(
            &self,
            transform: &NSStringTransform,
            reverse: bool,
        ) -> Option<Id<NSString, Shared>>;

        #[method(writeToURL:atomically:encoding:error:)]
        pub unsafe fn writeToURL_atomically_encoding_error(
            &self,
            url: &NSURL,
            useAuxiliaryFile: bool,
            enc: NSStringEncoding,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(writeToFile:atomically:encoding:error:)]
        pub unsafe fn writeToFile_atomically_encoding_error(
            &self,
            path: &NSString,
            useAuxiliaryFile: bool,
            enc: NSStringEncoding,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other description)]
        pub unsafe fn description(&self) -> Id<NSString, Shared>;

        #[method(hash)]
        pub unsafe fn hash(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Init initWithCharactersNoCopy:length:freeWhenDone:)]
        pub unsafe fn initWithCharactersNoCopy_length_freeWhenDone(
            this: Option<Allocated<Self>>,
            characters: NonNull<unichar>,
            length: NSUInteger,
            freeBuffer: bool,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCharactersNoCopy:length:deallocator:)]
        pub unsafe fn initWithCharactersNoCopy_length_deallocator(
            this: Option<Allocated<Self>>,
            chars: NonNull<unichar>,
            len: NSUInteger,
            deallocator: TodoBlock,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCharacters:length:)]
        pub unsafe fn initWithCharacters_length(
            this: Option<Allocated<Self>>,
            characters: NonNull<unichar>,
            length: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithUTF8String:)]
        pub unsafe fn initWithUTF8String(
            this: Option<Allocated<Self>>,
            nullTerminatedCString: NonNull<c_char>,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithString:)]
        pub unsafe fn initWithString(
            this: Option<Allocated<Self>>,
            aString: &NSString,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithFormat:arguments:)]
        pub unsafe fn initWithFormat_arguments(
            this: Option<Allocated<Self>>,
            format: &NSString,
            argList: va_list,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithFormat:locale:arguments:)]
        pub unsafe fn initWithFormat_locale_arguments(
            this: Option<Allocated<Self>>,
            format: &NSString,
            locale: Option<&Object>,
            argList: va_list,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithData:encoding:)]
        pub unsafe fn initWithData_encoding(
            this: Option<Allocated<Self>>,
            data: &NSData,
            encoding: NSStringEncoding,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithBytes:length:encoding:)]
        pub unsafe fn initWithBytes_length_encoding(
            this: Option<Allocated<Self>>,
            bytes: NonNull<c_void>,
            len: NSUInteger,
            encoding: NSStringEncoding,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithBytesNoCopy:length:encoding:freeWhenDone:)]
        pub unsafe fn initWithBytesNoCopy_length_encoding_freeWhenDone(
            this: Option<Allocated<Self>>,
            bytes: NonNull<c_void>,
            len: NSUInteger,
            encoding: NSStringEncoding,
            freeBuffer: bool,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithBytesNoCopy:length:encoding:deallocator:)]
        pub unsafe fn initWithBytesNoCopy_length_encoding_deallocator(
            this: Option<Allocated<Self>>,
            bytes: NonNull<c_void>,
            len: NSUInteger,
            encoding: NSStringEncoding,
            deallocator: TodoBlock,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other string)]
        pub unsafe fn string() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other stringWithString:)]
        pub unsafe fn stringWithString(string: &NSString) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other stringWithCharacters:length:)]
        pub unsafe fn stringWithCharacters_length(
            characters: NonNull<unichar>,
            length: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other stringWithUTF8String:)]
        pub unsafe fn stringWithUTF8String(
            nullTerminatedCString: NonNull<c_char>,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithCString:encoding:)]
        pub unsafe fn initWithCString_encoding(
            this: Option<Allocated<Self>>,
            nullTerminatedCString: NonNull<c_char>,
            encoding: NSStringEncoding,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other stringWithCString:encoding:)]
        pub unsafe fn stringWithCString_encoding(
            cString: NonNull<c_char>,
            enc: NSStringEncoding,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithContentsOfURL:encoding:error:)]
        pub unsafe fn initWithContentsOfURL_encoding_error(
            this: Option<Allocated<Self>>,
            url: &NSURL,
            enc: NSStringEncoding,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Init initWithContentsOfFile:encoding:error:)]
        pub unsafe fn initWithContentsOfFile_encoding_error(
            this: Option<Allocated<Self>>,
            path: &NSString,
            enc: NSStringEncoding,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other stringWithContentsOfURL:encoding:error:)]
        pub unsafe fn stringWithContentsOfURL_encoding_error(
            url: &NSURL,
            enc: NSStringEncoding,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other stringWithContentsOfFile:encoding:error:)]
        pub unsafe fn stringWithContentsOfFile_encoding_error(
            path: &NSString,
            enc: NSStringEncoding,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Init initWithContentsOfURL:usedEncoding:error:)]
        pub unsafe fn initWithContentsOfURL_usedEncoding_error(
            this: Option<Allocated<Self>>,
            url: &NSURL,
            enc: *mut NSStringEncoding,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Init initWithContentsOfFile:usedEncoding:error:)]
        pub unsafe fn initWithContentsOfFile_usedEncoding_error(
            this: Option<Allocated<Self>>,
            path: &NSString,
            enc: *mut NSStringEncoding,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other stringWithContentsOfURL:usedEncoding:error:)]
        pub unsafe fn stringWithContentsOfURL_usedEncoding_error(
            url: &NSURL,
            enc: *mut NSStringEncoding,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other stringWithContentsOfFile:usedEncoding:error:)]
        pub unsafe fn stringWithContentsOfFile_usedEncoding_error(
            path: &NSString,
            enc: *mut NSStringEncoding,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;
    }
);

pub type NSStringEncodingDetectionOptionsKey = NSString;

extern "C" {
    pub static NSStringEncodingDetectionSuggestedEncodingsKey:
        &'static NSStringEncodingDetectionOptionsKey;
}

extern "C" {
    pub static NSStringEncodingDetectionDisallowedEncodingsKey:
        &'static NSStringEncodingDetectionOptionsKey;
}

extern "C" {
    pub static NSStringEncodingDetectionUseOnlySuggestedEncodingsKey:
        &'static NSStringEncodingDetectionOptionsKey;
}

extern "C" {
    pub static NSStringEncodingDetectionAllowLossyKey: &'static NSStringEncodingDetectionOptionsKey;
}

extern "C" {
    pub static NSStringEncodingDetectionFromWindowsKey:
        &'static NSStringEncodingDetectionOptionsKey;
}

extern "C" {
    pub static NSStringEncodingDetectionLossySubstitutionKey:
        &'static NSStringEncodingDetectionOptionsKey;
}

extern "C" {
    pub static NSStringEncodingDetectionLikelyLanguageKey:
        &'static NSStringEncodingDetectionOptionsKey;
}

extern_methods!(
    /// NSStringEncodingDetection
    unsafe impl NSString {
        #[method(stringEncodingForData:encodingOptions:convertedString:usedLossyConversion:)]
        pub unsafe fn stringEncodingForData_encodingOptions_convertedString_usedLossyConversion(
            data: &NSData,
            opts: Option<&NSDictionary<NSStringEncodingDetectionOptionsKey, Object>>,
            string: Option<&mut Option<Id<NSString, Shared>>>,
            usedLossyConversion: *mut bool,
        ) -> NSStringEncoding;
    }
);

extern_methods!(
    /// NSItemProvider
    unsafe impl NSString {}
);

extern_class!(
    #[derive(Debug)]
    pub struct NSMutableString;

    unsafe impl ClassType for NSMutableString {
        type Super = NSString;
    }
);

extern_methods!(
    unsafe impl NSMutableString {
        #[method(replaceCharactersInRange:withString:)]
        pub unsafe fn replaceCharactersInRange_withString(
            &self,
            range: NSRange,
            aString: &NSString,
        );
    }
);

extern_methods!(
    /// NSMutableStringExtensionMethods
    unsafe impl NSMutableString {
        #[method(insertString:atIndex:)]
        pub unsafe fn insertString_atIndex(&self, aString: &NSString, loc: NSUInteger);

        #[method(deleteCharactersInRange:)]
        pub unsafe fn deleteCharactersInRange(&self, range: NSRange);

        #[method(appendString:)]
        pub unsafe fn appendString(&self, aString: &NSString);

        #[method(setString:)]
        pub unsafe fn setString(&self, aString: &NSString);

        #[method(replaceOccurrencesOfString:withString:options:range:)]
        pub unsafe fn replaceOccurrencesOfString_withString_options_range(
            &self,
            target: &NSString,
            replacement: &NSString,
            options: NSStringCompareOptions,
            searchRange: NSRange,
        ) -> NSUInteger;

        #[method(applyTransform:reverse:range:updatedRange:)]
        pub unsafe fn applyTransform_reverse_range_updatedRange(
            &self,
            transform: &NSStringTransform,
            reverse: bool,
            range: NSRange,
            resultingRange: NSRangePointer,
        ) -> bool;

        #[method_id(@__retain_semantics Init initWithCapacity:)]
        pub unsafe fn initWithCapacity(
            this: Option<Allocated<Self>>,
            capacity: NSUInteger,
        ) -> Id<NSMutableString, Shared>;

        #[method_id(@__retain_semantics Other stringWithCapacity:)]
        pub unsafe fn stringWithCapacity(capacity: NSUInteger) -> Id<NSMutableString, Shared>;
    }
);

extern "C" {
    pub static NSCharacterConversionException: &'static NSExceptionName;
}

extern "C" {
    pub static NSParseErrorException: &'static NSExceptionName;
}

extern_methods!(
    /// NSExtendedStringPropertyListParsing
    unsafe impl NSString {
        #[method_id(@__retain_semantics Other propertyList)]
        pub unsafe fn propertyList(&self) -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other propertyListFromStringsFileFormat)]
        pub unsafe fn propertyListFromStringsFileFormat(&self) -> Option<Id<NSDictionary, Shared>>;
    }
);

extern_methods!(
    /// NSStringDeprecated
    unsafe impl NSString {
        #[method(cString)]
        pub unsafe fn cString(&self) -> *mut c_char;

        #[method(lossyCString)]
        pub unsafe fn lossyCString(&self) -> *mut c_char;

        #[method(cStringLength)]
        pub unsafe fn cStringLength(&self) -> NSUInteger;

        #[method(getCString:)]
        pub unsafe fn getCString(&self, bytes: NonNull<c_char>);

        #[method(getCString:maxLength:)]
        pub unsafe fn getCString_maxLength(&self, bytes: NonNull<c_char>, maxLength: NSUInteger);

        #[method(getCString:maxLength:range:remainingRange:)]
        pub unsafe fn getCString_maxLength_range_remainingRange(
            &self,
            bytes: NonNull<c_char>,
            maxLength: NSUInteger,
            aRange: NSRange,
            leftoverRange: NSRangePointer,
        );

        #[method(writeToFile:atomically:)]
        pub unsafe fn writeToFile_atomically(
            &self,
            path: &NSString,
            useAuxiliaryFile: bool,
        ) -> bool;

        #[method(writeToURL:atomically:)]
        pub unsafe fn writeToURL_atomically(&self, url: &NSURL, atomically: bool) -> bool;

        #[method_id(@__retain_semantics Init initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            this: Option<Allocated<Self>>,
            path: &NSString,
        ) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Option<Allocated<Self>>,
            url: &NSURL,
        ) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other stringWithContentsOfFile:)]
        pub unsafe fn stringWithContentsOfFile(path: &NSString) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other stringWithContentsOfURL:)]
        pub unsafe fn stringWithContentsOfURL(url: &NSURL) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Init initWithCStringNoCopy:length:freeWhenDone:)]
        pub unsafe fn initWithCStringNoCopy_length_freeWhenDone(
            this: Option<Allocated<Self>>,
            bytes: NonNull<c_char>,
            length: NSUInteger,
            freeBuffer: bool,
        ) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Init initWithCString:length:)]
        pub unsafe fn initWithCString_length(
            this: Option<Allocated<Self>>,
            bytes: NonNull<c_char>,
            length: NSUInteger,
        ) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Init initWithCString:)]
        pub unsafe fn initWithCString(
            this: Option<Allocated<Self>>,
            bytes: NonNull<c_char>,
        ) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other stringWithCString:length:)]
        pub unsafe fn stringWithCString_length(
            bytes: NonNull<c_char>,
            length: NSUInteger,
        ) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other stringWithCString:)]
        pub unsafe fn stringWithCString(bytes: NonNull<c_char>) -> Option<Id<Object, Shared>>;

        #[method(getCharacters:)]
        pub unsafe fn getCharacters(&self, buffer: NonNull<unichar>);
    }
);

pub const NSProprietaryStringEncoding: i32 = 65536;

extern_class!(
    #[derive(Debug)]
    pub struct NSSimpleCString;

    unsafe impl ClassType for NSSimpleCString {
        type Super = NSString;
    }
);

extern_methods!(
    unsafe impl NSSimpleCString {}
);

extern_class!(
    #[derive(Debug)]
    pub struct NSConstantString;

    unsafe impl ClassType for NSConstantString {
        type Super = NSSimpleCString;
    }
);

extern_methods!(
    unsafe impl NSConstantString {}
);
