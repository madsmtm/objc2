use super::__exported::NSArray;
use super::__exported::NSDate;
use super::__exported::NSDictionary;
use super::__exported::NSOrthography;
use super::__exported::NSRegularExpression;
use super::__exported::NSString;
use super::__exported::NSTimeZone;
use super::__exported::NSURL;
use crate::Foundation::generated::NSDate::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSRange::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSTextCheckingTypes = u64;
pub type NSTextCheckingKey = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSTextCheckingResult;
    unsafe impl ClassType for NSTextCheckingResult {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSTextCheckingResult {
        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSTextCheckingType;
        #[method(range)]
        pub unsafe fn range(&self) -> NSRange;
    }
);
extern_methods!(
    #[doc = "NSTextCheckingResultOptional"]
    unsafe impl NSTextCheckingResult {
        #[method_id(orthography)]
        pub unsafe fn orthography(&self) -> Option<Id<NSOrthography, Shared>>;
        #[method_id(grammarDetails)]
        pub unsafe fn grammarDetails(
            &self,
        ) -> Option<Id<NSArray<NSDictionary<NSString, Object>>, Shared>>;
        #[method_id(date)]
        pub unsafe fn date(&self) -> Option<Id<NSDate, Shared>>;
        #[method_id(timeZone)]
        pub unsafe fn timeZone(&self) -> Option<Id<NSTimeZone, Shared>>;
        #[method(duration)]
        pub unsafe fn duration(&self) -> NSTimeInterval;
        #[method_id(components)]
        pub unsafe fn components(
            &self,
        ) -> Option<Id<NSDictionary<NSTextCheckingKey, NSString>, Shared>>;
        #[method_id(URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL, Shared>>;
        #[method_id(replacementString)]
        pub unsafe fn replacementString(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(alternativeStrings)]
        pub unsafe fn alternativeStrings(&self) -> Option<Id<NSArray<NSString>, Shared>>;
        #[method_id(regularExpression)]
        pub unsafe fn regularExpression(&self) -> Option<Id<NSRegularExpression, Shared>>;
        #[method_id(phoneNumber)]
        pub unsafe fn phoneNumber(&self) -> Option<Id<NSString, Shared>>;
        #[method(numberOfRanges)]
        pub unsafe fn numberOfRanges(&self) -> NSUInteger;
        # [method (rangeAtIndex :)]
        pub unsafe fn rangeAtIndex(&self, idx: NSUInteger) -> NSRange;
        # [method (rangeWithName :)]
        pub unsafe fn rangeWithName(&self, name: &NSString) -> NSRange;
        # [method_id (resultByAdjustingRangesWithOffset :)]
        pub unsafe fn resultByAdjustingRangesWithOffset(
            &self,
            offset: NSInteger,
        ) -> Id<NSTextCheckingResult, Shared>;
        #[method_id(addressComponents)]
        pub unsafe fn addressComponents(
            &self,
        ) -> Option<Id<NSDictionary<NSTextCheckingKey, NSString>, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSTextCheckingResultCreation"]
    unsafe impl NSTextCheckingResult {
        # [method_id (orthographyCheckingResultWithRange : orthography :)]
        pub unsafe fn orthographyCheckingResultWithRange_orthography(
            range: NSRange,
            orthography: &NSOrthography,
        ) -> Id<NSTextCheckingResult, Shared>;
        # [method_id (spellCheckingResultWithRange :)]
        pub unsafe fn spellCheckingResultWithRange(
            range: NSRange,
        ) -> Id<NSTextCheckingResult, Shared>;
        # [method_id (grammarCheckingResultWithRange : details :)]
        pub unsafe fn grammarCheckingResultWithRange_details(
            range: NSRange,
            details: &NSArray<NSDictionary<NSString, Object>>,
        ) -> Id<NSTextCheckingResult, Shared>;
        # [method_id (dateCheckingResultWithRange : date :)]
        pub unsafe fn dateCheckingResultWithRange_date(
            range: NSRange,
            date: &NSDate,
        ) -> Id<NSTextCheckingResult, Shared>;
        # [method_id (dateCheckingResultWithRange : date : timeZone : duration :)]
        pub unsafe fn dateCheckingResultWithRange_date_timeZone_duration(
            range: NSRange,
            date: &NSDate,
            timeZone: &NSTimeZone,
            duration: NSTimeInterval,
        ) -> Id<NSTextCheckingResult, Shared>;
        # [method_id (addressCheckingResultWithRange : components :)]
        pub unsafe fn addressCheckingResultWithRange_components(
            range: NSRange,
            components: &NSDictionary<NSTextCheckingKey, NSString>,
        ) -> Id<NSTextCheckingResult, Shared>;
        # [method_id (linkCheckingResultWithRange : URL :)]
        pub unsafe fn linkCheckingResultWithRange_URL(
            range: NSRange,
            url: &NSURL,
        ) -> Id<NSTextCheckingResult, Shared>;
        # [method_id (quoteCheckingResultWithRange : replacementString :)]
        pub unsafe fn quoteCheckingResultWithRange_replacementString(
            range: NSRange,
            replacementString: &NSString,
        ) -> Id<NSTextCheckingResult, Shared>;
        # [method_id (dashCheckingResultWithRange : replacementString :)]
        pub unsafe fn dashCheckingResultWithRange_replacementString(
            range: NSRange,
            replacementString: &NSString,
        ) -> Id<NSTextCheckingResult, Shared>;
        # [method_id (replacementCheckingResultWithRange : replacementString :)]
        pub unsafe fn replacementCheckingResultWithRange_replacementString(
            range: NSRange,
            replacementString: &NSString,
        ) -> Id<NSTextCheckingResult, Shared>;
        # [method_id (correctionCheckingResultWithRange : replacementString :)]
        pub unsafe fn correctionCheckingResultWithRange_replacementString(
            range: NSRange,
            replacementString: &NSString,
        ) -> Id<NSTextCheckingResult, Shared>;
        # [method_id (correctionCheckingResultWithRange : replacementString : alternativeStrings :)]
        pub unsafe fn correctionCheckingResultWithRange_replacementString_alternativeStrings(
            range: NSRange,
            replacementString: &NSString,
            alternativeStrings: &NSArray<NSString>,
        ) -> Id<NSTextCheckingResult, Shared>;
        # [method_id (regularExpressionCheckingResultWithRanges : count : regularExpression :)]
        pub unsafe fn regularExpressionCheckingResultWithRanges_count_regularExpression(
            ranges: NSRangePointer,
            count: NSUInteger,
            regularExpression: &NSRegularExpression,
        ) -> Id<NSTextCheckingResult, Shared>;
        # [method_id (phoneNumberCheckingResultWithRange : phoneNumber :)]
        pub unsafe fn phoneNumberCheckingResultWithRange_phoneNumber(
            range: NSRange,
            phoneNumber: &NSString,
        ) -> Id<NSTextCheckingResult, Shared>;
        # [method_id (transitInformationCheckingResultWithRange : components :)]
        pub unsafe fn transitInformationCheckingResultWithRange_components(
            range: NSRange,
            components: &NSDictionary<NSTextCheckingKey, NSString>,
        ) -> Id<NSTextCheckingResult, Shared>;
    }
);
