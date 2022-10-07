use super::__exported::NSArray;
use super::__exported::NSData;
use super::__exported::NSDate;
use super::__exported::NSDictionary;
use super::__exported::NSLocale;
use super::__exported::NSString;
use crate::Foundation::generated::NSDate::*;
use crate::Foundation::generated::NSNotification::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTimeZone;
    unsafe impl ClassType for NSTimeZone {
        type Super = NSObject;
    }
);
impl NSTimeZone {
    pub unsafe fn name(&self) -> Id<NSString, Shared> {
        msg_send_id![self, name]
    }
    pub unsafe fn data(&self) -> Id<NSData, Shared> {
        msg_send_id![self, data]
    }
    pub unsafe fn secondsFromGMTForDate(&self, aDate: &NSDate) -> NSInteger {
        msg_send![self, secondsFromGMTForDate: aDate]
    }
    pub unsafe fn abbreviationForDate(&self, aDate: &NSDate) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, abbreviationForDate: aDate]
    }
    pub unsafe fn isDaylightSavingTimeForDate(&self, aDate: &NSDate) -> bool {
        msg_send![self, isDaylightSavingTimeForDate: aDate]
    }
    pub unsafe fn daylightSavingTimeOffsetForDate(&self, aDate: &NSDate) -> NSTimeInterval {
        msg_send![self, daylightSavingTimeOffsetForDate: aDate]
    }
    pub unsafe fn nextDaylightSavingTimeTransitionAfterDate(
        &self,
        aDate: &NSDate,
    ) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, nextDaylightSavingTimeTransitionAfterDate: aDate]
    }
}
#[doc = "NSExtendedTimeZone"]
impl NSTimeZone {
    pub unsafe fn systemTimeZone() -> Id<NSTimeZone, Shared> {
        msg_send_id![Self::class(), systemTimeZone]
    }
    pub unsafe fn resetSystemTimeZone() {
        msg_send![Self::class(), resetSystemTimeZone]
    }
    pub unsafe fn defaultTimeZone() -> Id<NSTimeZone, Shared> {
        msg_send_id![Self::class(), defaultTimeZone]
    }
    pub unsafe fn setDefaultTimeZone(defaultTimeZone: &NSTimeZone) {
        msg_send![Self::class(), setDefaultTimeZone: defaultTimeZone]
    }
    pub unsafe fn localTimeZone() -> Id<NSTimeZone, Shared> {
        msg_send_id![Self::class(), localTimeZone]
    }
    pub unsafe fn knownTimeZoneNames() -> Id<NSArray<NSString>, Shared> {
        msg_send_id![Self::class(), knownTimeZoneNames]
    }
    pub unsafe fn abbreviationDictionary() -> Id<NSDictionary<NSString, NSString>, Shared> {
        msg_send_id![Self::class(), abbreviationDictionary]
    }
    pub unsafe fn setAbbreviationDictionary(
        abbreviationDictionary: &NSDictionary<NSString, NSString>,
    ) {
        msg_send![
            Self::class(),
            setAbbreviationDictionary: abbreviationDictionary
        ]
    }
    pub unsafe fn timeZoneDataVersion() -> Id<NSString, Shared> {
        msg_send_id![Self::class(), timeZoneDataVersion]
    }
    pub unsafe fn secondsFromGMT(&self) -> NSInteger {
        msg_send![self, secondsFromGMT]
    }
    pub unsafe fn abbreviation(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, abbreviation]
    }
    pub unsafe fn isDaylightSavingTime(&self) -> bool {
        msg_send![self, isDaylightSavingTime]
    }
    pub unsafe fn daylightSavingTimeOffset(&self) -> NSTimeInterval {
        msg_send![self, daylightSavingTimeOffset]
    }
    pub unsafe fn nextDaylightSavingTimeTransition(&self) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, nextDaylightSavingTimeTransition]
    }
    pub unsafe fn description(&self) -> Id<NSString, Shared> {
        msg_send_id![self, description]
    }
    pub unsafe fn isEqualToTimeZone(&self, aTimeZone: &NSTimeZone) -> bool {
        msg_send![self, isEqualToTimeZone: aTimeZone]
    }
    pub unsafe fn localizedName_locale(
        &self,
        style: NSTimeZoneNameStyle,
        locale: Option<&NSLocale>,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, localizedName: style, locale: locale]
    }
}
#[doc = "NSTimeZoneCreation"]
impl NSTimeZone {
    pub unsafe fn timeZoneWithName(tzName: &NSString) -> Option<Id<Self, Shared>> {
        msg_send_id![Self::class(), timeZoneWithName: tzName]
    }
    pub unsafe fn timeZoneWithName_data(
        tzName: &NSString,
        aData: Option<&NSData>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![Self::class(), timeZoneWithName: tzName, data: aData]
    }
    pub unsafe fn initWithName(&self, tzName: &NSString) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithName: tzName]
    }
    pub unsafe fn initWithName_data(
        &self,
        tzName: &NSString,
        aData: Option<&NSData>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithName: tzName, data: aData]
    }
    pub unsafe fn timeZoneForSecondsFromGMT(seconds: NSInteger) -> Id<Self, Shared> {
        msg_send_id![Self::class(), timeZoneForSecondsFromGMT: seconds]
    }
    pub unsafe fn timeZoneWithAbbreviation(abbreviation: &NSString) -> Option<Id<Self, Shared>> {
        msg_send_id![Self::class(), timeZoneWithAbbreviation: abbreviation]
    }
}
