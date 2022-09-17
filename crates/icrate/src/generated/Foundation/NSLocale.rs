extern_class!(
    #[derive(Debug)]
    struct NSLocale;
    unsafe impl ClassType for NSLocale {
        type Super = NSObject;
    }
);
impl NSLocale {
    pub unsafe fn objectForKey(&self, key: NSLocaleKey) -> Option<Id<Object, Shared>> {
        msg_send_id![self, objectForKey: key]
    }
    pub unsafe fn displayNameForKey_value(
        &self,
        key: NSLocaleKey,
        value: &Object,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, displayNameForKey: key, value: value]
    }
    pub unsafe fn initWithLocaleIdentifier(&self, string: &NSString) -> Id<Self, Shared> {
        msg_send_id![self, initWithLocaleIdentifier: string]
    }
    pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCoder: coder]
    }
}
#[doc = "NSExtendedLocale"]
impl NSLocale {
    pub unsafe fn localizedStringForLocaleIdentifier(
        &self,
        localeIdentifier: &NSString,
    ) -> Id<NSString, Shared> {
        msg_send_id![self, localizedStringForLocaleIdentifier: localeIdentifier]
    }
    pub unsafe fn localizedStringForLanguageCode(
        &self,
        languageCode: &NSString,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, localizedStringForLanguageCode: languageCode]
    }
    pub unsafe fn localizedStringForCountryCode(
        &self,
        countryCode: &NSString,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, localizedStringForCountryCode: countryCode]
    }
    pub unsafe fn localizedStringForScriptCode(
        &self,
        scriptCode: &NSString,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, localizedStringForScriptCode: scriptCode]
    }
    pub unsafe fn localizedStringForVariantCode(
        &self,
        variantCode: &NSString,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, localizedStringForVariantCode: variantCode]
    }
    pub unsafe fn localizedStringForCalendarIdentifier(
        &self,
        calendarIdentifier: &NSString,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![
            self,
            localizedStringForCalendarIdentifier: calendarIdentifier
        ]
    }
    pub unsafe fn localizedStringForCollationIdentifier(
        &self,
        collationIdentifier: &NSString,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![
            self,
            localizedStringForCollationIdentifier: collationIdentifier
        ]
    }
    pub unsafe fn localizedStringForCurrencyCode(
        &self,
        currencyCode: &NSString,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, localizedStringForCurrencyCode: currencyCode]
    }
    pub unsafe fn localizedStringForCollatorIdentifier(
        &self,
        collatorIdentifier: &NSString,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![
            self,
            localizedStringForCollatorIdentifier: collatorIdentifier
        ]
    }
    pub unsafe fn localeIdentifier(&self) -> Id<NSString, Shared> {
        msg_send_id![self, localeIdentifier]
    }
    pub unsafe fn languageCode(&self) -> Id<NSString, Shared> {
        msg_send_id![self, languageCode]
    }
    pub unsafe fn countryCode(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, countryCode]
    }
    pub unsafe fn scriptCode(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, scriptCode]
    }
    pub unsafe fn variantCode(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, variantCode]
    }
    pub unsafe fn exemplarCharacterSet(&self) -> Id<NSCharacterSet, Shared> {
        msg_send_id![self, exemplarCharacterSet]
    }
    pub unsafe fn calendarIdentifier(&self) -> Id<NSString, Shared> {
        msg_send_id![self, calendarIdentifier]
    }
    pub unsafe fn collationIdentifier(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, collationIdentifier]
    }
    pub unsafe fn usesMetricSystem(&self) -> bool {
        msg_send![self, usesMetricSystem]
    }
    pub unsafe fn decimalSeparator(&self) -> Id<NSString, Shared> {
        msg_send_id![self, decimalSeparator]
    }
    pub unsafe fn groupingSeparator(&self) -> Id<NSString, Shared> {
        msg_send_id![self, groupingSeparator]
    }
    pub unsafe fn currencySymbol(&self) -> Id<NSString, Shared> {
        msg_send_id![self, currencySymbol]
    }
    pub unsafe fn currencyCode(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, currencyCode]
    }
    pub unsafe fn collatorIdentifier(&self) -> Id<NSString, Shared> {
        msg_send_id![self, collatorIdentifier]
    }
    pub unsafe fn quotationBeginDelimiter(&self) -> Id<NSString, Shared> {
        msg_send_id![self, quotationBeginDelimiter]
    }
    pub unsafe fn quotationEndDelimiter(&self) -> Id<NSString, Shared> {
        msg_send_id![self, quotationEndDelimiter]
    }
    pub unsafe fn alternateQuotationBeginDelimiter(&self) -> Id<NSString, Shared> {
        msg_send_id![self, alternateQuotationBeginDelimiter]
    }
    pub unsafe fn alternateQuotationEndDelimiter(&self) -> Id<NSString, Shared> {
        msg_send_id![self, alternateQuotationEndDelimiter]
    }
}
#[doc = "NSLocaleCreation"]
impl NSLocale {
    pub unsafe fn localeWithLocaleIdentifier(ident: &NSString) -> Id<Self, Shared> {
        msg_send_id![Self::class(), localeWithLocaleIdentifier: ident]
    }
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn autoupdatingCurrentLocale() -> Id<NSLocale, Shared> {
        msg_send_id![Self::class(), autoupdatingCurrentLocale]
    }
    pub unsafe fn currentLocale() -> Id<NSLocale, Shared> {
        msg_send_id![Self::class(), currentLocale]
    }
    pub unsafe fn systemLocale() -> Id<NSLocale, Shared> {
        msg_send_id![Self::class(), systemLocale]
    }
}
#[doc = "NSLocaleGeneralInfo"]
impl NSLocale {
    pub unsafe fn componentsFromLocaleIdentifier(string: &NSString) -> TodoGenerics {
        msg_send![Self::class(), componentsFromLocaleIdentifier: string]
    }
    pub unsafe fn localeIdentifierFromComponents(dict: TodoGenerics) -> Id<NSString, Shared> {
        msg_send_id![Self::class(), localeIdentifierFromComponents: dict]
    }
    pub unsafe fn canonicalLocaleIdentifierFromString(string: &NSString) -> Id<NSString, Shared> {
        msg_send_id![Self::class(), canonicalLocaleIdentifierFromString: string]
    }
    pub unsafe fn canonicalLanguageIdentifierFromString(string: &NSString) -> Id<NSString, Shared> {
        msg_send_id![Self::class(), canonicalLanguageIdentifierFromString: string]
    }
    pub unsafe fn localeIdentifierFromWindowsLocaleCode(
        lcid: uint32_t,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![Self::class(), localeIdentifierFromWindowsLocaleCode: lcid]
    }
    pub unsafe fn windowsLocaleCodeFromLocaleIdentifier(localeIdentifier: &NSString) -> uint32_t {
        msg_send![
            Self::class(),
            windowsLocaleCodeFromLocaleIdentifier: localeIdentifier
        ]
    }
    pub unsafe fn characterDirectionForLanguage(
        isoLangCode: &NSString,
    ) -> NSLocaleLanguageDirection {
        msg_send![Self::class(), characterDirectionForLanguage: isoLangCode]
    }
    pub unsafe fn lineDirectionForLanguage(isoLangCode: &NSString) -> NSLocaleLanguageDirection {
        msg_send![Self::class(), lineDirectionForLanguage: isoLangCode]
    }
    pub unsafe fn availableLocaleIdentifiers() -> TodoGenerics {
        msg_send![Self::class(), availableLocaleIdentifiers]
    }
    pub unsafe fn ISOLanguageCodes() -> TodoGenerics {
        msg_send![Self::class(), ISOLanguageCodes]
    }
    pub unsafe fn ISOCountryCodes() -> TodoGenerics {
        msg_send![Self::class(), ISOCountryCodes]
    }
    pub unsafe fn ISOCurrencyCodes() -> TodoGenerics {
        msg_send![Self::class(), ISOCurrencyCodes]
    }
    pub unsafe fn commonISOCurrencyCodes() -> TodoGenerics {
        msg_send![Self::class(), commonISOCurrencyCodes]
    }
    pub unsafe fn preferredLanguages() -> TodoGenerics {
        msg_send![Self::class(), preferredLanguages]
    }
}
