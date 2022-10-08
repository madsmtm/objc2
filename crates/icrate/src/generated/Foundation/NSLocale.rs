use super::__exported::NSCalendar;
use crate::CoreFoundation::generated::CFLocale::*;
use crate::Foundation::generated::NSNotification::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSLocaleKey = NSString;
use super::__exported::NSArray;
use super::__exported::NSDictionary;
use super::__exported::NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSLocale;
    unsafe impl ClassType for NSLocale {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSLocale {
        # [method_id (objectForKey :)]
        pub unsafe fn objectForKey(&self, key: &NSLocaleKey) -> Option<Id<Object, Shared>>;
        # [method_id (displayNameForKey : value :)]
        pub unsafe fn displayNameForKey_value(
            &self,
            key: &NSLocaleKey,
            value: &Object,
        ) -> Option<Id<NSString, Shared>>;
        # [method_id (initWithLocaleIdentifier :)]
        pub unsafe fn initWithLocaleIdentifier(&self, string: &NSString) -> Id<Self, Shared>;
        # [method_id (initWithCoder :)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSExtendedLocale"]
    unsafe impl NSLocale {
        #[method_id(localeIdentifier)]
        pub unsafe fn localeIdentifier(&self) -> Id<NSString, Shared>;
        # [method_id (localizedStringForLocaleIdentifier :)]
        pub unsafe fn localizedStringForLocaleIdentifier(
            &self,
            localeIdentifier: &NSString,
        ) -> Id<NSString, Shared>;
        #[method_id(languageCode)]
        pub unsafe fn languageCode(&self) -> Id<NSString, Shared>;
        # [method_id (localizedStringForLanguageCode :)]
        pub unsafe fn localizedStringForLanguageCode(
            &self,
            languageCode: &NSString,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(countryCode)]
        pub unsafe fn countryCode(&self) -> Option<Id<NSString, Shared>>;
        # [method_id (localizedStringForCountryCode :)]
        pub unsafe fn localizedStringForCountryCode(
            &self,
            countryCode: &NSString,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(scriptCode)]
        pub unsafe fn scriptCode(&self) -> Option<Id<NSString, Shared>>;
        # [method_id (localizedStringForScriptCode :)]
        pub unsafe fn localizedStringForScriptCode(
            &self,
            scriptCode: &NSString,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(variantCode)]
        pub unsafe fn variantCode(&self) -> Option<Id<NSString, Shared>>;
        # [method_id (localizedStringForVariantCode :)]
        pub unsafe fn localizedStringForVariantCode(
            &self,
            variantCode: &NSString,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(exemplarCharacterSet)]
        pub unsafe fn exemplarCharacterSet(&self) -> Id<NSCharacterSet, Shared>;
        #[method_id(calendarIdentifier)]
        pub unsafe fn calendarIdentifier(&self) -> Id<NSString, Shared>;
        # [method_id (localizedStringForCalendarIdentifier :)]
        pub unsafe fn localizedStringForCalendarIdentifier(
            &self,
            calendarIdentifier: &NSString,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(collationIdentifier)]
        pub unsafe fn collationIdentifier(&self) -> Option<Id<NSString, Shared>>;
        # [method_id (localizedStringForCollationIdentifier :)]
        pub unsafe fn localizedStringForCollationIdentifier(
            &self,
            collationIdentifier: &NSString,
        ) -> Option<Id<NSString, Shared>>;
        #[method(usesMetricSystem)]
        pub unsafe fn usesMetricSystem(&self) -> bool;
        #[method_id(decimalSeparator)]
        pub unsafe fn decimalSeparator(&self) -> Id<NSString, Shared>;
        #[method_id(groupingSeparator)]
        pub unsafe fn groupingSeparator(&self) -> Id<NSString, Shared>;
        #[method_id(currencySymbol)]
        pub unsafe fn currencySymbol(&self) -> Id<NSString, Shared>;
        #[method_id(currencyCode)]
        pub unsafe fn currencyCode(&self) -> Option<Id<NSString, Shared>>;
        # [method_id (localizedStringForCurrencyCode :)]
        pub unsafe fn localizedStringForCurrencyCode(
            &self,
            currencyCode: &NSString,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(collatorIdentifier)]
        pub unsafe fn collatorIdentifier(&self) -> Id<NSString, Shared>;
        # [method_id (localizedStringForCollatorIdentifier :)]
        pub unsafe fn localizedStringForCollatorIdentifier(
            &self,
            collatorIdentifier: &NSString,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(quotationBeginDelimiter)]
        pub unsafe fn quotationBeginDelimiter(&self) -> Id<NSString, Shared>;
        #[method_id(quotationEndDelimiter)]
        pub unsafe fn quotationEndDelimiter(&self) -> Id<NSString, Shared>;
        #[method_id(alternateQuotationBeginDelimiter)]
        pub unsafe fn alternateQuotationBeginDelimiter(&self) -> Id<NSString, Shared>;
        #[method_id(alternateQuotationEndDelimiter)]
        pub unsafe fn alternateQuotationEndDelimiter(&self) -> Id<NSString, Shared>;
    }
);
extern_methods!(
    #[doc = "NSLocaleCreation"]
    unsafe impl NSLocale {
        #[method_id(autoupdatingCurrentLocale)]
        pub unsafe fn autoupdatingCurrentLocale() -> Id<NSLocale, Shared>;
        #[method_id(currentLocale)]
        pub unsafe fn currentLocale() -> Id<NSLocale, Shared>;
        #[method_id(systemLocale)]
        pub unsafe fn systemLocale() -> Id<NSLocale, Shared>;
        # [method_id (localeWithLocaleIdentifier :)]
        pub unsafe fn localeWithLocaleIdentifier(ident: &NSString) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
    }
);
extern_methods!(
    #[doc = "NSLocaleGeneralInfo"]
    unsafe impl NSLocale {
        #[method_id(availableLocaleIdentifiers)]
        pub unsafe fn availableLocaleIdentifiers() -> Id<NSArray<NSString>, Shared>;
        #[method_id(ISOLanguageCodes)]
        pub unsafe fn ISOLanguageCodes() -> Id<NSArray<NSString>, Shared>;
        #[method_id(ISOCountryCodes)]
        pub unsafe fn ISOCountryCodes() -> Id<NSArray<NSString>, Shared>;
        #[method_id(ISOCurrencyCodes)]
        pub unsafe fn ISOCurrencyCodes() -> Id<NSArray<NSString>, Shared>;
        #[method_id(commonISOCurrencyCodes)]
        pub unsafe fn commonISOCurrencyCodes() -> Id<NSArray<NSString>, Shared>;
        #[method_id(preferredLanguages)]
        pub unsafe fn preferredLanguages() -> Id<NSArray<NSString>, Shared>;
        # [method_id (componentsFromLocaleIdentifier :)]
        pub unsafe fn componentsFromLocaleIdentifier(
            string: &NSString,
        ) -> Id<NSDictionary<NSString, NSString>, Shared>;
        # [method_id (localeIdentifierFromComponents :)]
        pub unsafe fn localeIdentifierFromComponents(
            dict: &NSDictionary<NSString, NSString>,
        ) -> Id<NSString, Shared>;
        # [method_id (canonicalLocaleIdentifierFromString :)]
        pub unsafe fn canonicalLocaleIdentifierFromString(
            string: &NSString,
        ) -> Id<NSString, Shared>;
        # [method_id (canonicalLanguageIdentifierFromString :)]
        pub unsafe fn canonicalLanguageIdentifierFromString(
            string: &NSString,
        ) -> Id<NSString, Shared>;
        # [method_id (localeIdentifierFromWindowsLocaleCode :)]
        pub unsafe fn localeIdentifierFromWindowsLocaleCode(
            lcid: u32,
        ) -> Option<Id<NSString, Shared>>;
        # [method (windowsLocaleCodeFromLocaleIdentifier :)]
        pub unsafe fn windowsLocaleCodeFromLocaleIdentifier(localeIdentifier: &NSString) -> u32;
        # [method (characterDirectionForLanguage :)]
        pub unsafe fn characterDirectionForLanguage(
            isoLangCode: &NSString,
        ) -> NSLocaleLanguageDirection;
        # [method (lineDirectionForLanguage :)]
        pub unsafe fn lineDirectionForLanguage(isoLangCode: &NSString)
            -> NSLocaleLanguageDirection;
    }
);
