use super::__exported::NSArray;
use super::__exported::NSCharacterSet;
use super::__exported::NSData;
use super::__exported::NSDictionary;
use super::__exported::NSError;
use super::__exported::NSLocale;
use super::__exported::NSURL;
use crate::Foundation::generated::NSItemProvider::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSRange::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
pub type NSStringEncoding = NSUInteger;
extern_class!(
    #[derive(Debug)]
    pub struct NSString;
    unsafe impl ClassType for NSString {
        type Super = NSObject;
    }
);
impl NSString {
    pub fn length(&self) -> NSUInteger {
        msg_send![self, length]
    }
    pub unsafe fn characterAtIndex(&self, index: NSUInteger) -> unichar {
        msg_send![self, characterAtIndex: index]
    }
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCoder: coder]
    }
}
pub type NSStringTransform = NSString;
#[doc = "NSStringExtensionMethods"]
impl NSString {
    pub unsafe fn substringFromIndex(&self, from: NSUInteger) -> Id<NSString, Shared> {
        msg_send_id![self, substringFromIndex: from]
    }
    pub unsafe fn substringToIndex(&self, to: NSUInteger) -> Id<NSString, Shared> {
        msg_send_id![self, substringToIndex: to]
    }
    pub unsafe fn substringWithRange(&self, range: NSRange) -> Id<NSString, Shared> {
        msg_send_id![self, substringWithRange: range]
    }
    pub unsafe fn getCharacters_range(&self, buffer: NonNull<unichar>, range: NSRange) {
        msg_send![self, getCharacters: buffer, range: range]
    }
    pub unsafe fn compare(&self, string: &NSString) -> NSComparisonResult {
        msg_send![self, compare: string]
    }
    pub unsafe fn compare_options(
        &self,
        string: &NSString,
        mask: NSStringCompareOptions,
    ) -> NSComparisonResult {
        msg_send![self, compare: string, options: mask]
    }
    pub unsafe fn compare_options_range(
        &self,
        string: &NSString,
        mask: NSStringCompareOptions,
        rangeOfReceiverToCompare: NSRange,
    ) -> NSComparisonResult {
        msg_send![
            self,
            compare: string,
            options: mask,
            range: rangeOfReceiverToCompare
        ]
    }
    pub unsafe fn compare_options_range_locale(
        &self,
        string: &NSString,
        mask: NSStringCompareOptions,
        rangeOfReceiverToCompare: NSRange,
        locale: Option<&Object>,
    ) -> NSComparisonResult {
        msg_send![
            self,
            compare: string,
            options: mask,
            range: rangeOfReceiverToCompare,
            locale: locale
        ]
    }
    pub unsafe fn caseInsensitiveCompare(&self, string: &NSString) -> NSComparisonResult {
        msg_send![self, caseInsensitiveCompare: string]
    }
    pub unsafe fn localizedCompare(&self, string: &NSString) -> NSComparisonResult {
        msg_send![self, localizedCompare: string]
    }
    pub unsafe fn localizedCaseInsensitiveCompare(&self, string: &NSString) -> NSComparisonResult {
        msg_send![self, localizedCaseInsensitiveCompare: string]
    }
    pub unsafe fn localizedStandardCompare(&self, string: &NSString) -> NSComparisonResult {
        msg_send![self, localizedStandardCompare: string]
    }
    pub unsafe fn isEqualToString(&self, aString: &NSString) -> bool {
        msg_send![self, isEqualToString: aString]
    }
    pub unsafe fn hasPrefix(&self, str: &NSString) -> bool {
        msg_send![self, hasPrefix: str]
    }
    pub unsafe fn hasSuffix(&self, str: &NSString) -> bool {
        msg_send![self, hasSuffix: str]
    }
    pub unsafe fn commonPrefixWithString_options(
        &self,
        str: &NSString,
        mask: NSStringCompareOptions,
    ) -> Id<NSString, Shared> {
        msg_send_id![self, commonPrefixWithString: str, options: mask]
    }
    pub unsafe fn containsString(&self, str: &NSString) -> bool {
        msg_send![self, containsString: str]
    }
    pub unsafe fn localizedCaseInsensitiveContainsString(&self, str: &NSString) -> bool {
        msg_send![self, localizedCaseInsensitiveContainsString: str]
    }
    pub unsafe fn localizedStandardContainsString(&self, str: &NSString) -> bool {
        msg_send![self, localizedStandardContainsString: str]
    }
    pub unsafe fn localizedStandardRangeOfString(&self, str: &NSString) -> NSRange {
        msg_send![self, localizedStandardRangeOfString: str]
    }
    pub unsafe fn rangeOfString(&self, searchString: &NSString) -> NSRange {
        msg_send![self, rangeOfString: searchString]
    }
    pub unsafe fn rangeOfString_options(
        &self,
        searchString: &NSString,
        mask: NSStringCompareOptions,
    ) -> NSRange {
        msg_send![self, rangeOfString: searchString, options: mask]
    }
    pub unsafe fn rangeOfString_options_range(
        &self,
        searchString: &NSString,
        mask: NSStringCompareOptions,
        rangeOfReceiverToSearch: NSRange,
    ) -> NSRange {
        msg_send![
            self,
            rangeOfString: searchString,
            options: mask,
            range: rangeOfReceiverToSearch
        ]
    }
    pub unsafe fn rangeOfString_options_range_locale(
        &self,
        searchString: &NSString,
        mask: NSStringCompareOptions,
        rangeOfReceiverToSearch: NSRange,
        locale: Option<&NSLocale>,
    ) -> NSRange {
        msg_send![
            self,
            rangeOfString: searchString,
            options: mask,
            range: rangeOfReceiverToSearch,
            locale: locale
        ]
    }
    pub unsafe fn rangeOfCharacterFromSet(&self, searchSet: &NSCharacterSet) -> NSRange {
        msg_send![self, rangeOfCharacterFromSet: searchSet]
    }
    pub unsafe fn rangeOfCharacterFromSet_options(
        &self,
        searchSet: &NSCharacterSet,
        mask: NSStringCompareOptions,
    ) -> NSRange {
        msg_send![self, rangeOfCharacterFromSet: searchSet, options: mask]
    }
    pub unsafe fn rangeOfCharacterFromSet_options_range(
        &self,
        searchSet: &NSCharacterSet,
        mask: NSStringCompareOptions,
        rangeOfReceiverToSearch: NSRange,
    ) -> NSRange {
        msg_send![
            self,
            rangeOfCharacterFromSet: searchSet,
            options: mask,
            range: rangeOfReceiverToSearch
        ]
    }
    pub unsafe fn rangeOfComposedCharacterSequenceAtIndex(&self, index: NSUInteger) -> NSRange {
        msg_send![self, rangeOfComposedCharacterSequenceAtIndex: index]
    }
    pub unsafe fn rangeOfComposedCharacterSequencesForRange(&self, range: NSRange) -> NSRange {
        msg_send![self, rangeOfComposedCharacterSequencesForRange: range]
    }
    pub fn stringByAppendingString(&self, aString: &NSString) -> Id<NSString, Shared> {
        msg_send_id![self, stringByAppendingString: aString]
    }
    pub unsafe fn doubleValue(&self) -> c_double {
        msg_send![self, doubleValue]
    }
    pub unsafe fn floatValue(&self) -> c_float {
        msg_send![self, floatValue]
    }
    pub unsafe fn intValue(&self) -> c_int {
        msg_send![self, intValue]
    }
    pub unsafe fn integerValue(&self) -> NSInteger {
        msg_send![self, integerValue]
    }
    pub unsafe fn longLongValue(&self) -> c_longlong {
        msg_send![self, longLongValue]
    }
    pub unsafe fn boolValue(&self) -> bool {
        msg_send![self, boolValue]
    }
    pub unsafe fn uppercaseString(&self) -> Id<NSString, Shared> {
        msg_send_id![self, uppercaseString]
    }
    pub unsafe fn lowercaseString(&self) -> Id<NSString, Shared> {
        msg_send_id![self, lowercaseString]
    }
    pub unsafe fn capitalizedString(&self) -> Id<NSString, Shared> {
        msg_send_id![self, capitalizedString]
    }
    pub unsafe fn localizedUppercaseString(&self) -> Id<NSString, Shared> {
        msg_send_id![self, localizedUppercaseString]
    }
    pub unsafe fn localizedLowercaseString(&self) -> Id<NSString, Shared> {
        msg_send_id![self, localizedLowercaseString]
    }
    pub unsafe fn localizedCapitalizedString(&self) -> Id<NSString, Shared> {
        msg_send_id![self, localizedCapitalizedString]
    }
    pub unsafe fn uppercaseStringWithLocale(
        &self,
        locale: Option<&NSLocale>,
    ) -> Id<NSString, Shared> {
        msg_send_id![self, uppercaseStringWithLocale: locale]
    }
    pub unsafe fn lowercaseStringWithLocale(
        &self,
        locale: Option<&NSLocale>,
    ) -> Id<NSString, Shared> {
        msg_send_id![self, lowercaseStringWithLocale: locale]
    }
    pub unsafe fn capitalizedStringWithLocale(
        &self,
        locale: Option<&NSLocale>,
    ) -> Id<NSString, Shared> {
        msg_send_id![self, capitalizedStringWithLocale: locale]
    }
    pub unsafe fn getLineStart_end_contentsEnd_forRange(
        &self,
        startPtr: *mut NSUInteger,
        lineEndPtr: *mut NSUInteger,
        contentsEndPtr: *mut NSUInteger,
        range: NSRange,
    ) {
        msg_send![
            self,
            getLineStart: startPtr,
            end: lineEndPtr,
            contentsEnd: contentsEndPtr,
            forRange: range
        ]
    }
    pub unsafe fn lineRangeForRange(&self, range: NSRange) -> NSRange {
        msg_send![self, lineRangeForRange: range]
    }
    pub unsafe fn getParagraphStart_end_contentsEnd_forRange(
        &self,
        startPtr: *mut NSUInteger,
        parEndPtr: *mut NSUInteger,
        contentsEndPtr: *mut NSUInteger,
        range: NSRange,
    ) {
        msg_send![
            self,
            getParagraphStart: startPtr,
            end: parEndPtr,
            contentsEnd: contentsEndPtr,
            forRange: range
        ]
    }
    pub unsafe fn paragraphRangeForRange(&self, range: NSRange) -> NSRange {
        msg_send![self, paragraphRangeForRange: range]
    }
    pub unsafe fn enumerateSubstringsInRange_options_usingBlock(
        &self,
        range: NSRange,
        opts: NSStringEnumerationOptions,
        block: TodoBlock,
    ) {
        msg_send![
            self,
            enumerateSubstringsInRange: range,
            options: opts,
            usingBlock: block
        ]
    }
    pub unsafe fn enumerateLinesUsingBlock(&self, block: TodoBlock) {
        msg_send![self, enumerateLinesUsingBlock: block]
    }
    pub fn UTF8String(&self) -> *mut c_char {
        msg_send![self, UTF8String]
    }
    pub unsafe fn fastestEncoding(&self) -> NSStringEncoding {
        msg_send![self, fastestEncoding]
    }
    pub unsafe fn smallestEncoding(&self) -> NSStringEncoding {
        msg_send![self, smallestEncoding]
    }
    pub unsafe fn dataUsingEncoding_allowLossyConversion(
        &self,
        encoding: NSStringEncoding,
        lossy: bool,
    ) -> Option<Id<NSData, Shared>> {
        msg_send_id![
            self,
            dataUsingEncoding: encoding,
            allowLossyConversion: lossy
        ]
    }
    pub unsafe fn dataUsingEncoding(
        &self,
        encoding: NSStringEncoding,
    ) -> Option<Id<NSData, Shared>> {
        msg_send_id![self, dataUsingEncoding: encoding]
    }
    pub unsafe fn canBeConvertedToEncoding(&self, encoding: NSStringEncoding) -> bool {
        msg_send![self, canBeConvertedToEncoding: encoding]
    }
    pub unsafe fn cStringUsingEncoding(&self, encoding: NSStringEncoding) -> *mut c_char {
        msg_send![self, cStringUsingEncoding: encoding]
    }
    pub unsafe fn getCString_maxLength_encoding(
        &self,
        buffer: NonNull<c_char>,
        maxBufferCount: NSUInteger,
        encoding: NSStringEncoding,
    ) -> bool {
        msg_send![
            self,
            getCString: buffer,
            maxLength: maxBufferCount,
            encoding: encoding
        ]
    }
    pub unsafe fn getBytes_maxLength_usedLength_encoding_options_range_remainingRange(
        &self,
        buffer: *mut c_void,
        maxBufferCount: NSUInteger,
        usedBufferCount: *mut NSUInteger,
        encoding: NSStringEncoding,
        options: NSStringEncodingConversionOptions,
        range: NSRange,
        leftover: NSRangePointer,
    ) -> bool {
        msg_send![
            self,
            getBytes: buffer,
            maxLength: maxBufferCount,
            usedLength: usedBufferCount,
            encoding: encoding,
            options: options,
            range: range,
            remainingRange: leftover
        ]
    }
    pub unsafe fn maximumLengthOfBytesUsingEncoding(&self, enc: NSStringEncoding) -> NSUInteger {
        msg_send![self, maximumLengthOfBytesUsingEncoding: enc]
    }
    pub fn lengthOfBytesUsingEncoding(&self, enc: NSStringEncoding) -> NSUInteger {
        msg_send![self, lengthOfBytesUsingEncoding: enc]
    }
    pub unsafe fn availableStringEncodings() -> NonNull<NSStringEncoding> {
        msg_send![Self::class(), availableStringEncodings]
    }
    pub unsafe fn localizedNameOfStringEncoding(
        encoding: NSStringEncoding,
    ) -> Id<NSString, Shared> {
        msg_send_id![Self::class(), localizedNameOfStringEncoding: encoding]
    }
    pub unsafe fn defaultCStringEncoding() -> NSStringEncoding {
        msg_send![Self::class(), defaultCStringEncoding]
    }
    pub unsafe fn decomposedStringWithCanonicalMapping(&self) -> Id<NSString, Shared> {
        msg_send_id![self, decomposedStringWithCanonicalMapping]
    }
    pub unsafe fn precomposedStringWithCanonicalMapping(&self) -> Id<NSString, Shared> {
        msg_send_id![self, precomposedStringWithCanonicalMapping]
    }
    pub unsafe fn decomposedStringWithCompatibilityMapping(&self) -> Id<NSString, Shared> {
        msg_send_id![self, decomposedStringWithCompatibilityMapping]
    }
    pub unsafe fn precomposedStringWithCompatibilityMapping(&self) -> Id<NSString, Shared> {
        msg_send_id![self, precomposedStringWithCompatibilityMapping]
    }
    pub unsafe fn componentsSeparatedByString(
        &self,
        separator: &NSString,
    ) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, componentsSeparatedByString: separator]
    }
    pub unsafe fn componentsSeparatedByCharactersInSet(
        &self,
        separator: &NSCharacterSet,
    ) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, componentsSeparatedByCharactersInSet: separator]
    }
    pub unsafe fn stringByTrimmingCharactersInSet(
        &self,
        set: &NSCharacterSet,
    ) -> Id<NSString, Shared> {
        msg_send_id![self, stringByTrimmingCharactersInSet: set]
    }
    pub unsafe fn stringByPaddingToLength_withString_startingAtIndex(
        &self,
        newLength: NSUInteger,
        padString: &NSString,
        padIndex: NSUInteger,
    ) -> Id<NSString, Shared> {
        msg_send_id![
            self,
            stringByPaddingToLength: newLength,
            withString: padString,
            startingAtIndex: padIndex
        ]
    }
    pub unsafe fn stringByFoldingWithOptions_locale(
        &self,
        options: NSStringCompareOptions,
        locale: Option<&NSLocale>,
    ) -> Id<NSString, Shared> {
        msg_send_id![self, stringByFoldingWithOptions: options, locale: locale]
    }
    pub unsafe fn stringByReplacingOccurrencesOfString_withString_options_range(
        &self,
        target: &NSString,
        replacement: &NSString,
        options: NSStringCompareOptions,
        searchRange: NSRange,
    ) -> Id<NSString, Shared> {
        msg_send_id![
            self,
            stringByReplacingOccurrencesOfString: target,
            withString: replacement,
            options: options,
            range: searchRange
        ]
    }
    pub unsafe fn stringByReplacingOccurrencesOfString_withString(
        &self,
        target: &NSString,
        replacement: &NSString,
    ) -> Id<NSString, Shared> {
        msg_send_id![
            self,
            stringByReplacingOccurrencesOfString: target,
            withString: replacement
        ]
    }
    pub unsafe fn stringByReplacingCharactersInRange_withString(
        &self,
        range: NSRange,
        replacement: &NSString,
    ) -> Id<NSString, Shared> {
        msg_send_id![
            self,
            stringByReplacingCharactersInRange: range,
            withString: replacement
        ]
    }
    pub unsafe fn stringByApplyingTransform_reverse(
        &self,
        transform: &NSStringTransform,
        reverse: bool,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, stringByApplyingTransform: transform, reverse: reverse]
    }
    pub unsafe fn writeToURL_atomically_encoding_error(
        &self,
        url: &NSURL,
        useAuxiliaryFile: bool,
        enc: NSStringEncoding,
        error: *mut *mut NSError,
    ) -> bool {
        msg_send![
            self,
            writeToURL: url,
            atomically: useAuxiliaryFile,
            encoding: enc,
            error: error
        ]
    }
    pub unsafe fn writeToFile_atomically_encoding_error(
        &self,
        path: &NSString,
        useAuxiliaryFile: bool,
        enc: NSStringEncoding,
        error: *mut *mut NSError,
    ) -> bool {
        msg_send![
            self,
            writeToFile: path,
            atomically: useAuxiliaryFile,
            encoding: enc,
            error: error
        ]
    }
    pub unsafe fn description(&self) -> Id<NSString, Shared> {
        msg_send_id![self, description]
    }
    pub unsafe fn hash(&self) -> NSUInteger {
        msg_send![self, hash]
    }
    pub unsafe fn initWithCharactersNoCopy_length_freeWhenDone(
        &self,
        characters: NonNull<unichar>,
        length: NSUInteger,
        freeBuffer: bool,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithCharactersNoCopy: characters,
            length: length,
            freeWhenDone: freeBuffer
        ]
    }
    pub unsafe fn initWithCharactersNoCopy_length_deallocator(
        &self,
        chars: NonNull<unichar>,
        len: NSUInteger,
        deallocator: TodoBlock,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithCharactersNoCopy: chars,
            length: len,
            deallocator: deallocator
        ]
    }
    pub unsafe fn initWithCharacters_length(
        &self,
        characters: NonNull<unichar>,
        length: NSUInteger,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithCharacters: characters, length: length]
    }
    pub unsafe fn initWithUTF8String(
        &self,
        nullTerminatedCString: NonNull<c_char>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithUTF8String: nullTerminatedCString]
    }
    pub unsafe fn initWithString(&self, aString: &NSString) -> Id<Self, Shared> {
        msg_send_id![self, initWithString: aString]
    }
    pub unsafe fn initWithFormat_arguments(
        &self,
        format: &NSString,
        argList: va_list,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithFormat: format, arguments: argList]
    }
    pub unsafe fn initWithFormat_locale_arguments(
        &self,
        format: &NSString,
        locale: Option<&Object>,
        argList: va_list,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithFormat: format,
            locale: locale,
            arguments: argList
        ]
    }
    pub unsafe fn initWithData_encoding(
        &self,
        data: &NSData,
        encoding: NSStringEncoding,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithData: data, encoding: encoding]
    }
    pub unsafe fn initWithBytes_length_encoding(
        &self,
        bytes: NonNull<c_void>,
        len: NSUInteger,
        encoding: NSStringEncoding,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithBytes: bytes, length: len, encoding: encoding]
    }
    pub unsafe fn initWithBytesNoCopy_length_encoding_freeWhenDone(
        &self,
        bytes: NonNull<c_void>,
        len: NSUInteger,
        encoding: NSStringEncoding,
        freeBuffer: bool,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            self,
            initWithBytesNoCopy: bytes,
            length: len,
            encoding: encoding,
            freeWhenDone: freeBuffer
        ]
    }
    pub unsafe fn initWithBytesNoCopy_length_encoding_deallocator(
        &self,
        bytes: NonNull<c_void>,
        len: NSUInteger,
        encoding: NSStringEncoding,
        deallocator: TodoBlock,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            self,
            initWithBytesNoCopy: bytes,
            length: len,
            encoding: encoding,
            deallocator: deallocator
        ]
    }
    pub unsafe fn string() -> Id<Self, Shared> {
        msg_send_id![Self::class(), string]
    }
    pub unsafe fn stringWithString(string: &NSString) -> Id<Self, Shared> {
        msg_send_id![Self::class(), stringWithString: string]
    }
    pub unsafe fn stringWithCharacters_length(
        characters: NonNull<unichar>,
        length: NSUInteger,
    ) -> Id<Self, Shared> {
        msg_send_id![
            Self::class(),
            stringWithCharacters: characters,
            length: length
        ]
    }
    pub unsafe fn stringWithUTF8String(
        nullTerminatedCString: NonNull<c_char>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![Self::class(), stringWithUTF8String: nullTerminatedCString]
    }
    pub unsafe fn initWithCString_encoding(
        &self,
        nullTerminatedCString: NonNull<c_char>,
        encoding: NSStringEncoding,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            self,
            initWithCString: nullTerminatedCString,
            encoding: encoding
        ]
    }
    pub unsafe fn stringWithCString_encoding(
        cString: NonNull<c_char>,
        enc: NSStringEncoding,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![Self::class(), stringWithCString: cString, encoding: enc]
    }
    pub unsafe fn initWithContentsOfURL_encoding_error(
        &self,
        url: &NSURL,
        enc: NSStringEncoding,
        error: *mut *mut NSError,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            self,
            initWithContentsOfURL: url,
            encoding: enc,
            error: error
        ]
    }
    pub unsafe fn initWithContentsOfFile_encoding_error(
        &self,
        path: &NSString,
        enc: NSStringEncoding,
        error: *mut *mut NSError,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            self,
            initWithContentsOfFile: path,
            encoding: enc,
            error: error
        ]
    }
    pub unsafe fn stringWithContentsOfURL_encoding_error(
        url: &NSURL,
        enc: NSStringEncoding,
        error: *mut *mut NSError,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            Self::class(),
            stringWithContentsOfURL: url,
            encoding: enc,
            error: error
        ]
    }
    pub unsafe fn stringWithContentsOfFile_encoding_error(
        path: &NSString,
        enc: NSStringEncoding,
        error: *mut *mut NSError,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            Self::class(),
            stringWithContentsOfFile: path,
            encoding: enc,
            error: error
        ]
    }
    pub unsafe fn initWithContentsOfURL_usedEncoding_error(
        &self,
        url: &NSURL,
        enc: *mut NSStringEncoding,
        error: *mut *mut NSError,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            self,
            initWithContentsOfURL: url,
            usedEncoding: enc,
            error: error
        ]
    }
    pub unsafe fn initWithContentsOfFile_usedEncoding_error(
        &self,
        path: &NSString,
        enc: *mut NSStringEncoding,
        error: *mut *mut NSError,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            self,
            initWithContentsOfFile: path,
            usedEncoding: enc,
            error: error
        ]
    }
    pub unsafe fn stringWithContentsOfURL_usedEncoding_error(
        url: &NSURL,
        enc: *mut NSStringEncoding,
        error: *mut *mut NSError,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            Self::class(),
            stringWithContentsOfURL: url,
            usedEncoding: enc,
            error: error
        ]
    }
    pub unsafe fn stringWithContentsOfFile_usedEncoding_error(
        path: &NSString,
        enc: *mut NSStringEncoding,
        error: *mut *mut NSError,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            Self::class(),
            stringWithContentsOfFile: path,
            usedEncoding: enc,
            error: error
        ]
    }
}
pub type NSStringEncodingDetectionOptionsKey = NSString;
#[doc = "NSStringEncodingDetection"]
impl NSString {
    pub unsafe fn stringEncodingForData_encodingOptions_convertedString_usedLossyConversion(
        data: &NSData,
        opts: Option<&NSDictionary<NSStringEncodingDetectionOptionsKey, Object>>,
        string: Option<&mut Option<Id<NSString, Shared>>>,
        usedLossyConversion: *mut bool,
    ) -> NSStringEncoding {
        msg_send![
            Self::class(),
            stringEncodingForData: data,
            encodingOptions: opts,
            convertedString: string,
            usedLossyConversion: usedLossyConversion
        ]
    }
}
#[doc = "NSItemProvider"]
impl NSString {}
extern_class!(
    #[derive(Debug)]
    pub struct NSMutableString;
    unsafe impl ClassType for NSMutableString {
        type Super = NSString;
    }
);
impl NSMutableString {
    pub unsafe fn replaceCharactersInRange_withString(&self, range: NSRange, aString: &NSString) {
        msg_send![self, replaceCharactersInRange: range, withString: aString]
    }
}
#[doc = "NSMutableStringExtensionMethods"]
impl NSMutableString {
    pub unsafe fn insertString_atIndex(&self, aString: &NSString, loc: NSUInteger) {
        msg_send![self, insertString: aString, atIndex: loc]
    }
    pub unsafe fn deleteCharactersInRange(&self, range: NSRange) {
        msg_send![self, deleteCharactersInRange: range]
    }
    pub unsafe fn appendString(&self, aString: &NSString) {
        msg_send![self, appendString: aString]
    }
    pub unsafe fn setString(&self, aString: &NSString) {
        msg_send![self, setString: aString]
    }
    pub unsafe fn replaceOccurrencesOfString_withString_options_range(
        &self,
        target: &NSString,
        replacement: &NSString,
        options: NSStringCompareOptions,
        searchRange: NSRange,
    ) -> NSUInteger {
        msg_send![
            self,
            replaceOccurrencesOfString: target,
            withString: replacement,
            options: options,
            range: searchRange
        ]
    }
    pub unsafe fn applyTransform_reverse_range_updatedRange(
        &self,
        transform: &NSStringTransform,
        reverse: bool,
        range: NSRange,
        resultingRange: NSRangePointer,
    ) -> bool {
        msg_send![
            self,
            applyTransform: transform,
            reverse: reverse,
            range: range,
            updatedRange: resultingRange
        ]
    }
    pub unsafe fn initWithCapacity(&self, capacity: NSUInteger) -> Id<NSMutableString, Shared> {
        msg_send_id![self, initWithCapacity: capacity]
    }
    pub unsafe fn stringWithCapacity(capacity: NSUInteger) -> Id<NSMutableString, Shared> {
        msg_send_id![Self::class(), stringWithCapacity: capacity]
    }
}
#[doc = "NSExtendedStringPropertyListParsing"]
impl NSString {
    pub unsafe fn propertyList(&self) -> Id<Object, Shared> {
        msg_send_id![self, propertyList]
    }
    pub unsafe fn propertyListFromStringsFileFormat(&self) -> Option<Id<NSDictionary, Shared>> {
        msg_send_id![self, propertyListFromStringsFileFormat]
    }
}
#[doc = "NSStringDeprecated"]
impl NSString {
    pub unsafe fn cString(&self) -> *mut c_char {
        msg_send![self, cString]
    }
    pub unsafe fn lossyCString(&self) -> *mut c_char {
        msg_send![self, lossyCString]
    }
    pub unsafe fn cStringLength(&self) -> NSUInteger {
        msg_send![self, cStringLength]
    }
    pub unsafe fn getCString(&self, bytes: NonNull<c_char>) {
        msg_send![self, getCString: bytes]
    }
    pub unsafe fn getCString_maxLength(&self, bytes: NonNull<c_char>, maxLength: NSUInteger) {
        msg_send![self, getCString: bytes, maxLength: maxLength]
    }
    pub unsafe fn getCString_maxLength_range_remainingRange(
        &self,
        bytes: NonNull<c_char>,
        maxLength: NSUInteger,
        aRange: NSRange,
        leftoverRange: NSRangePointer,
    ) {
        msg_send![
            self,
            getCString: bytes,
            maxLength: maxLength,
            range: aRange,
            remainingRange: leftoverRange
        ]
    }
    pub unsafe fn writeToFile_atomically(&self, path: &NSString, useAuxiliaryFile: bool) -> bool {
        msg_send![self, writeToFile: path, atomically: useAuxiliaryFile]
    }
    pub unsafe fn writeToURL_atomically(&self, url: &NSURL, atomically: bool) -> bool {
        msg_send![self, writeToURL: url, atomically: atomically]
    }
    pub unsafe fn initWithContentsOfFile(&self, path: &NSString) -> Option<Id<Object, Shared>> {
        msg_send_id![self, initWithContentsOfFile: path]
    }
    pub unsafe fn initWithContentsOfURL(&self, url: &NSURL) -> Option<Id<Object, Shared>> {
        msg_send_id![self, initWithContentsOfURL: url]
    }
    pub unsafe fn stringWithContentsOfFile(path: &NSString) -> Option<Id<Object, Shared>> {
        msg_send_id![Self::class(), stringWithContentsOfFile: path]
    }
    pub unsafe fn stringWithContentsOfURL(url: &NSURL) -> Option<Id<Object, Shared>> {
        msg_send_id![Self::class(), stringWithContentsOfURL: url]
    }
    pub unsafe fn initWithCStringNoCopy_length_freeWhenDone(
        &self,
        bytes: NonNull<c_char>,
        length: NSUInteger,
        freeBuffer: bool,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![
            self,
            initWithCStringNoCopy: bytes,
            length: length,
            freeWhenDone: freeBuffer
        ]
    }
    pub unsafe fn initWithCString_length(
        &self,
        bytes: NonNull<c_char>,
        length: NSUInteger,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![self, initWithCString: bytes, length: length]
    }
    pub unsafe fn initWithCString(&self, bytes: NonNull<c_char>) -> Option<Id<Object, Shared>> {
        msg_send_id![self, initWithCString: bytes]
    }
    pub unsafe fn stringWithCString_length(
        bytes: NonNull<c_char>,
        length: NSUInteger,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![Self::class(), stringWithCString: bytes, length: length]
    }
    pub unsafe fn stringWithCString(bytes: NonNull<c_char>) -> Option<Id<Object, Shared>> {
        msg_send_id![Self::class(), stringWithCString: bytes]
    }
    pub unsafe fn getCharacters(&self, buffer: NonNull<unichar>) {
        msg_send![self, getCharacters: buffer]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSSimpleCString;
    unsafe impl ClassType for NSSimpleCString {
        type Super = NSString;
    }
);
impl NSSimpleCString {}
extern_class!(
    #[derive(Debug)]
    pub struct NSConstantString;
    unsafe impl ClassType for NSConstantString {
        type Super = NSSimpleCString;
    }
);
impl NSConstantString {}
