use super::__exported::NSArray;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSString::*;
use crate::Foundation::generated::NSTextCheckingResult::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSRegularExpression;
    unsafe impl ClassType for NSRegularExpression {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSRegularExpression {
        #[method_id(regularExpressionWithPattern:options:error:)]
        pub unsafe fn regularExpressionWithPattern_options_error(
            pattern: &NSString,
            options: NSRegularExpressionOptions,
        ) -> Result<Id<NSRegularExpression, Shared>, Id<NSError, Shared>>;
        #[method_id(initWithPattern:options:error:)]
        pub unsafe fn initWithPattern_options_error(
            &self,
            pattern: &NSString,
            options: NSRegularExpressionOptions,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;
        #[method_id(pattern)]
        pub unsafe fn pattern(&self) -> Id<NSString, Shared>;
        #[method(options)]
        pub unsafe fn options(&self) -> NSRegularExpressionOptions;
        #[method(numberOfCaptureGroups)]
        pub unsafe fn numberOfCaptureGroups(&self) -> NSUInteger;
        #[method_id(escapedPatternForString:)]
        pub unsafe fn escapedPatternForString(string: &NSString) -> Id<NSString, Shared>;
    }
);
extern_methods!(
    #[doc = "NSMatching"]
    unsafe impl NSRegularExpression {
        #[method(enumerateMatchesInString:options:range:usingBlock:)]
        pub unsafe fn enumerateMatchesInString_options_range_usingBlock(
            &self,
            string: &NSString,
            options: NSMatchingOptions,
            range: NSRange,
            block: TodoBlock,
        );
        #[method_id(matchesInString:options:range:)]
        pub unsafe fn matchesInString_options_range(
            &self,
            string: &NSString,
            options: NSMatchingOptions,
            range: NSRange,
        ) -> Id<NSArray<NSTextCheckingResult>, Shared>;
        #[method(numberOfMatchesInString:options:range:)]
        pub unsafe fn numberOfMatchesInString_options_range(
            &self,
            string: &NSString,
            options: NSMatchingOptions,
            range: NSRange,
        ) -> NSUInteger;
        #[method_id(firstMatchInString:options:range:)]
        pub unsafe fn firstMatchInString_options_range(
            &self,
            string: &NSString,
            options: NSMatchingOptions,
            range: NSRange,
        ) -> Option<Id<NSTextCheckingResult, Shared>>;
        #[method(rangeOfFirstMatchInString:options:range:)]
        pub unsafe fn rangeOfFirstMatchInString_options_range(
            &self,
            string: &NSString,
            options: NSMatchingOptions,
            range: NSRange,
        ) -> NSRange;
    }
);
extern_methods!(
    #[doc = "NSReplacement"]
    unsafe impl NSRegularExpression {
        #[method_id(stringByReplacingMatchesInString:options:range:withTemplate:)]
        pub unsafe fn stringByReplacingMatchesInString_options_range_withTemplate(
            &self,
            string: &NSString,
            options: NSMatchingOptions,
            range: NSRange,
            templ: &NSString,
        ) -> Id<NSString, Shared>;
        #[method(replaceMatchesInString:options:range:withTemplate:)]
        pub unsafe fn replaceMatchesInString_options_range_withTemplate(
            &self,
            string: &NSMutableString,
            options: NSMatchingOptions,
            range: NSRange,
            templ: &NSString,
        ) -> NSUInteger;
        #[method_id(replacementStringForResult:inString:offset:template:)]
        pub unsafe fn replacementStringForResult_inString_offset_template(
            &self,
            result: &NSTextCheckingResult,
            string: &NSString,
            offset: NSInteger,
            templ: &NSString,
        ) -> Id<NSString, Shared>;
        #[method_id(escapedTemplateForString:)]
        pub unsafe fn escapedTemplateForString(string: &NSString) -> Id<NSString, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSDataDetector;
    unsafe impl ClassType for NSDataDetector {
        type Super = NSRegularExpression;
    }
);
extern_methods!(
    unsafe impl NSDataDetector {
        #[method_id(dataDetectorWithTypes:error:)]
        pub unsafe fn dataDetectorWithTypes_error(
            checkingTypes: NSTextCheckingTypes,
        ) -> Result<Id<NSDataDetector, Shared>, Id<NSError, Shared>>;
        #[method_id(initWithTypes:error:)]
        pub unsafe fn initWithTypes_error(
            &self,
            checkingTypes: NSTextCheckingTypes,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;
        #[method(checkingTypes)]
        pub unsafe fn checkingTypes(&self) -> NSTextCheckingTypes;
    }
);
