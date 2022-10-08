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
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTimeZone;
    unsafe impl ClassType for NSTimeZone {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSTimeZone {
        #[method_id(name)]
        pub unsafe fn name(&self) -> Id<NSString, Shared>;
        #[method_id(data)]
        pub unsafe fn data(&self) -> Id<NSData, Shared>;
        # [method (secondsFromGMTForDate :)]
        pub unsafe fn secondsFromGMTForDate(&self, aDate: &NSDate) -> NSInteger;
        # [method_id (abbreviationForDate :)]
        pub unsafe fn abbreviationForDate(&self, aDate: &NSDate) -> Option<Id<NSString, Shared>>;
        # [method (isDaylightSavingTimeForDate :)]
        pub unsafe fn isDaylightSavingTimeForDate(&self, aDate: &NSDate) -> bool;
        # [method (daylightSavingTimeOffsetForDate :)]
        pub unsafe fn daylightSavingTimeOffsetForDate(&self, aDate: &NSDate) -> NSTimeInterval;
        # [method_id (nextDaylightSavingTimeTransitionAfterDate :)]
        pub unsafe fn nextDaylightSavingTimeTransitionAfterDate(
            &self,
            aDate: &NSDate,
        ) -> Option<Id<NSDate, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSExtendedTimeZone"]
    unsafe impl NSTimeZone {
        #[method_id(systemTimeZone)]
        pub unsafe fn systemTimeZone() -> Id<NSTimeZone, Shared>;
        #[method(resetSystemTimeZone)]
        pub unsafe fn resetSystemTimeZone();
        #[method_id(defaultTimeZone)]
        pub unsafe fn defaultTimeZone() -> Id<NSTimeZone, Shared>;
        # [method (setDefaultTimeZone :)]
        pub unsafe fn setDefaultTimeZone(defaultTimeZone: &NSTimeZone);
        #[method_id(localTimeZone)]
        pub unsafe fn localTimeZone() -> Id<NSTimeZone, Shared>;
        #[method_id(knownTimeZoneNames)]
        pub unsafe fn knownTimeZoneNames() -> Id<NSArray<NSString>, Shared>;
        #[method_id(abbreviationDictionary)]
        pub unsafe fn abbreviationDictionary() -> Id<NSDictionary<NSString, NSString>, Shared>;
        # [method (setAbbreviationDictionary :)]
        pub unsafe fn setAbbreviationDictionary(
            abbreviationDictionary: &NSDictionary<NSString, NSString>,
        );
        #[method_id(timeZoneDataVersion)]
        pub unsafe fn timeZoneDataVersion() -> Id<NSString, Shared>;
        #[method(secondsFromGMT)]
        pub unsafe fn secondsFromGMT(&self) -> NSInteger;
        #[method_id(abbreviation)]
        pub unsafe fn abbreviation(&self) -> Option<Id<NSString, Shared>>;
        #[method(isDaylightSavingTime)]
        pub unsafe fn isDaylightSavingTime(&self) -> bool;
        #[method(daylightSavingTimeOffset)]
        pub unsafe fn daylightSavingTimeOffset(&self) -> NSTimeInterval;
        #[method_id(nextDaylightSavingTimeTransition)]
        pub unsafe fn nextDaylightSavingTimeTransition(&self) -> Option<Id<NSDate, Shared>>;
        #[method_id(description)]
        pub unsafe fn description(&self) -> Id<NSString, Shared>;
        # [method (isEqualToTimeZone :)]
        pub unsafe fn isEqualToTimeZone(&self, aTimeZone: &NSTimeZone) -> bool;
        # [method_id (localizedName : locale :)]
        pub unsafe fn localizedName_locale(
            &self,
            style: NSTimeZoneNameStyle,
            locale: Option<&NSLocale>,
        ) -> Option<Id<NSString, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSTimeZoneCreation"]
    unsafe impl NSTimeZone {
        # [method_id (timeZoneWithName :)]
        pub unsafe fn timeZoneWithName(tzName: &NSString) -> Option<Id<Self, Shared>>;
        # [method_id (timeZoneWithName : data :)]
        pub unsafe fn timeZoneWithName_data(
            tzName: &NSString,
            aData: Option<&NSData>,
        ) -> Option<Id<Self, Shared>>;
        # [method_id (initWithName :)]
        pub unsafe fn initWithName(&self, tzName: &NSString) -> Option<Id<Self, Shared>>;
        # [method_id (initWithName : data :)]
        pub unsafe fn initWithName_data(
            &self,
            tzName: &NSString,
            aData: Option<&NSData>,
        ) -> Option<Id<Self, Shared>>;
        # [method_id (timeZoneForSecondsFromGMT :)]
        pub unsafe fn timeZoneForSecondsFromGMT(seconds: NSInteger) -> Id<Self, Shared>;
        # [method_id (timeZoneWithAbbreviation :)]
        pub unsafe fn timeZoneWithAbbreviation(abbreviation: &NSString)
            -> Option<Id<Self, Shared>>;
    }
);
