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
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
pub type NSTextCheckingTypes = u64;
pub type NSTextCheckingKey = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSTextCheckingResult;
    unsafe impl ClassType for NSTextCheckingResult {
        type Super = NSObject;
    }
);
impl NSTextCheckingResult {
    pub unsafe fn resultType(&self) -> NSTextCheckingType {
        msg_send![self, resultType]
    }
    pub unsafe fn range(&self) -> NSRange {
        msg_send![self, range]
    }
}
#[doc = "NSTextCheckingResultOptional"]
impl NSTextCheckingResult {
    pub unsafe fn orthography(&self) -> Option<Id<NSOrthography, Shared>> {
        msg_send_id![self, orthography]
    }
    pub unsafe fn grammarDetails(
        &self,
    ) -> Option<Id<NSArray<NSDictionary<NSString, Object>>, Shared>> {
        msg_send_id![self, grammarDetails]
    }
    pub unsafe fn date(&self) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, date]
    }
    pub unsafe fn timeZone(&self) -> Option<Id<NSTimeZone, Shared>> {
        msg_send_id![self, timeZone]
    }
    pub unsafe fn duration(&self) -> NSTimeInterval {
        msg_send![self, duration]
    }
    pub unsafe fn components(
        &self,
    ) -> Option<Id<NSDictionary<NSTextCheckingKey, NSString>, Shared>> {
        msg_send_id![self, components]
    }
    pub unsafe fn URL(&self) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, URL]
    }
    pub unsafe fn replacementString(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, replacementString]
    }
    pub unsafe fn alternativeStrings(&self) -> Option<Id<NSArray<NSString>, Shared>> {
        msg_send_id![self, alternativeStrings]
    }
    pub unsafe fn regularExpression(&self) -> Option<Id<NSRegularExpression, Shared>> {
        msg_send_id![self, regularExpression]
    }
    pub unsafe fn phoneNumber(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, phoneNumber]
    }
    pub unsafe fn numberOfRanges(&self) -> NSUInteger {
        msg_send![self, numberOfRanges]
    }
    pub unsafe fn rangeAtIndex(&self, idx: NSUInteger) -> NSRange {
        msg_send![self, rangeAtIndex: idx]
    }
    pub unsafe fn rangeWithName(&self, name: &NSString) -> NSRange {
        msg_send![self, rangeWithName: name]
    }
    pub unsafe fn resultByAdjustingRangesWithOffset(
        &self,
        offset: NSInteger,
    ) -> Id<NSTextCheckingResult, Shared> {
        msg_send_id![self, resultByAdjustingRangesWithOffset: offset]
    }
    pub unsafe fn addressComponents(
        &self,
    ) -> Option<Id<NSDictionary<NSTextCheckingKey, NSString>, Shared>> {
        msg_send_id![self, addressComponents]
    }
}
#[doc = "NSTextCheckingResultCreation"]
impl NSTextCheckingResult {
    pub unsafe fn orthographyCheckingResultWithRange_orthography(
        range: NSRange,
        orthography: &NSOrthography,
    ) -> Id<NSTextCheckingResult, Shared> {
        msg_send_id![
            Self::class(),
            orthographyCheckingResultWithRange: range,
            orthography: orthography
        ]
    }
    pub unsafe fn spellCheckingResultWithRange(range: NSRange) -> Id<NSTextCheckingResult, Shared> {
        msg_send_id![Self::class(), spellCheckingResultWithRange: range]
    }
    pub unsafe fn grammarCheckingResultWithRange_details(
        range: NSRange,
        details: &NSArray<NSDictionary<NSString, Object>>,
    ) -> Id<NSTextCheckingResult, Shared> {
        msg_send_id![
            Self::class(),
            grammarCheckingResultWithRange: range,
            details: details
        ]
    }
    pub unsafe fn dateCheckingResultWithRange_date(
        range: NSRange,
        date: &NSDate,
    ) -> Id<NSTextCheckingResult, Shared> {
        msg_send_id![
            Self::class(),
            dateCheckingResultWithRange: range,
            date: date
        ]
    }
    pub unsafe fn dateCheckingResultWithRange_date_timeZone_duration(
        range: NSRange,
        date: &NSDate,
        timeZone: &NSTimeZone,
        duration: NSTimeInterval,
    ) -> Id<NSTextCheckingResult, Shared> {
        msg_send_id![
            Self::class(),
            dateCheckingResultWithRange: range,
            date: date,
            timeZone: timeZone,
            duration: duration
        ]
    }
    pub unsafe fn addressCheckingResultWithRange_components(
        range: NSRange,
        components: &NSDictionary<NSTextCheckingKey, NSString>,
    ) -> Id<NSTextCheckingResult, Shared> {
        msg_send_id![
            Self::class(),
            addressCheckingResultWithRange: range,
            components: components
        ]
    }
    pub unsafe fn linkCheckingResultWithRange_URL(
        range: NSRange,
        url: &NSURL,
    ) -> Id<NSTextCheckingResult, Shared> {
        msg_send_id![Self::class(), linkCheckingResultWithRange: range, URL: url]
    }
    pub unsafe fn quoteCheckingResultWithRange_replacementString(
        range: NSRange,
        replacementString: &NSString,
    ) -> Id<NSTextCheckingResult, Shared> {
        msg_send_id![
            Self::class(),
            quoteCheckingResultWithRange: range,
            replacementString: replacementString
        ]
    }
    pub unsafe fn dashCheckingResultWithRange_replacementString(
        range: NSRange,
        replacementString: &NSString,
    ) -> Id<NSTextCheckingResult, Shared> {
        msg_send_id![
            Self::class(),
            dashCheckingResultWithRange: range,
            replacementString: replacementString
        ]
    }
    pub unsafe fn replacementCheckingResultWithRange_replacementString(
        range: NSRange,
        replacementString: &NSString,
    ) -> Id<NSTextCheckingResult, Shared> {
        msg_send_id![
            Self::class(),
            replacementCheckingResultWithRange: range,
            replacementString: replacementString
        ]
    }
    pub unsafe fn correctionCheckingResultWithRange_replacementString(
        range: NSRange,
        replacementString: &NSString,
    ) -> Id<NSTextCheckingResult, Shared> {
        msg_send_id![
            Self::class(),
            correctionCheckingResultWithRange: range,
            replacementString: replacementString
        ]
    }
    pub unsafe fn correctionCheckingResultWithRange_replacementString_alternativeStrings(
        range: NSRange,
        replacementString: &NSString,
        alternativeStrings: &NSArray<NSString>,
    ) -> Id<NSTextCheckingResult, Shared> {
        msg_send_id![
            Self::class(),
            correctionCheckingResultWithRange: range,
            replacementString: replacementString,
            alternativeStrings: alternativeStrings
        ]
    }
    pub unsafe fn regularExpressionCheckingResultWithRanges_count_regularExpression(
        ranges: NSRangePointer,
        count: NSUInteger,
        regularExpression: &NSRegularExpression,
    ) -> Id<NSTextCheckingResult, Shared> {
        msg_send_id![
            Self::class(),
            regularExpressionCheckingResultWithRanges: ranges,
            count: count,
            regularExpression: regularExpression
        ]
    }
    pub unsafe fn phoneNumberCheckingResultWithRange_phoneNumber(
        range: NSRange,
        phoneNumber: &NSString,
    ) -> Id<NSTextCheckingResult, Shared> {
        msg_send_id![
            Self::class(),
            phoneNumberCheckingResultWithRange: range,
            phoneNumber: phoneNumber
        ]
    }
    pub unsafe fn transitInformationCheckingResultWithRange_components(
        range: NSRange,
        components: &NSDictionary<NSTextCheckingKey, NSString>,
    ) -> Id<NSTextCheckingResult, Shared> {
        msg_send_id![
            Self::class(),
            transitInformationCheckingResultWithRange: range,
            components: components
        ]
    }
}
