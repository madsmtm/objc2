use super::__exported::NSCache;
use super::__exported::NSError;
use super::__exported::NSLocale;
use super::__exported::NSMutableDictionary;
use super::__exported::NSRecursiveLock;
use super::__exported::NSString;
use crate::CoreFoundation::generated::CFNumberFormatter::*;
use crate::Foundation::generated::NSFormatter::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSNumberFormatter;
    unsafe impl ClassType for NSNumberFormatter {
        type Super = NSFormatter;
    }
);
impl NSNumberFormatter {
    pub unsafe fn getObjectValue_forString_range_error(
        &self,
        obj: *mut *mut Object,
        string: &NSString,
        rangep: *mut NSRange,
        error: *mut *mut NSError,
    ) -> bool {
        msg_send![
            self,
            getObjectValue: obj,
            forString: string,
            range: rangep,
            error: error
        ]
    }
    pub unsafe fn stringFromNumber(&self, number: &NSNumber) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, stringFromNumber: number]
    }
    pub unsafe fn numberFromString(&self, string: &NSString) -> Option<Id<NSNumber, Shared>> {
        msg_send_id![self, numberFromString: string]
    }
    pub unsafe fn localizedStringFromNumber_numberStyle(
        num: &NSNumber,
        nstyle: NSNumberFormatterStyle,
    ) -> Id<NSString, Shared> {
        msg_send_id![
            Self::class(),
            localizedStringFromNumber: num,
            numberStyle: nstyle
        ]
    }
    pub unsafe fn defaultFormatterBehavior() -> NSNumberFormatterBehavior {
        msg_send![Self::class(), defaultFormatterBehavior]
    }
    pub unsafe fn setDefaultFormatterBehavior(behavior: NSNumberFormatterBehavior) {
        msg_send![Self::class(), setDefaultFormatterBehavior: behavior]
    }
    pub unsafe fn formattingContext(&self) -> NSFormattingContext {
        msg_send![self, formattingContext]
    }
    pub unsafe fn setFormattingContext(&self, formattingContext: NSFormattingContext) {
        msg_send![self, setFormattingContext: formattingContext]
    }
    pub unsafe fn numberStyle(&self) -> NSNumberFormatterStyle {
        msg_send![self, numberStyle]
    }
    pub unsafe fn setNumberStyle(&self, numberStyle: NSNumberFormatterStyle) {
        msg_send![self, setNumberStyle: numberStyle]
    }
    pub unsafe fn locale(&self) -> Id<NSLocale, Shared> {
        msg_send_id![self, locale]
    }
    pub unsafe fn setLocale(&self, locale: Option<&NSLocale>) {
        msg_send![self, setLocale: locale]
    }
    pub unsafe fn generatesDecimalNumbers(&self) -> bool {
        msg_send![self, generatesDecimalNumbers]
    }
    pub unsafe fn setGeneratesDecimalNumbers(&self, generatesDecimalNumbers: bool) {
        msg_send![self, setGeneratesDecimalNumbers: generatesDecimalNumbers]
    }
    pub unsafe fn formatterBehavior(&self) -> NSNumberFormatterBehavior {
        msg_send![self, formatterBehavior]
    }
    pub unsafe fn setFormatterBehavior(&self, formatterBehavior: NSNumberFormatterBehavior) {
        msg_send![self, setFormatterBehavior: formatterBehavior]
    }
    pub unsafe fn negativeFormat(&self) -> Id<NSString, Shared> {
        msg_send_id![self, negativeFormat]
    }
    pub unsafe fn setNegativeFormat(&self, negativeFormat: Option<&NSString>) {
        msg_send![self, setNegativeFormat: negativeFormat]
    }
    pub unsafe fn textAttributesForNegativeValues(
        &self,
    ) -> Option<Id<NSDictionary<NSString, Object>, Shared>> {
        msg_send_id![self, textAttributesForNegativeValues]
    }
    pub unsafe fn setTextAttributesForNegativeValues(
        &self,
        textAttributesForNegativeValues: Option<&NSDictionary<NSString, Object>>,
    ) {
        msg_send![
            self,
            setTextAttributesForNegativeValues: textAttributesForNegativeValues
        ]
    }
    pub unsafe fn positiveFormat(&self) -> Id<NSString, Shared> {
        msg_send_id![self, positiveFormat]
    }
    pub unsafe fn setPositiveFormat(&self, positiveFormat: Option<&NSString>) {
        msg_send![self, setPositiveFormat: positiveFormat]
    }
    pub unsafe fn textAttributesForPositiveValues(
        &self,
    ) -> Option<Id<NSDictionary<NSString, Object>, Shared>> {
        msg_send_id![self, textAttributesForPositiveValues]
    }
    pub unsafe fn setTextAttributesForPositiveValues(
        &self,
        textAttributesForPositiveValues: Option<&NSDictionary<NSString, Object>>,
    ) {
        msg_send![
            self,
            setTextAttributesForPositiveValues: textAttributesForPositiveValues
        ]
    }
    pub unsafe fn allowsFloats(&self) -> bool {
        msg_send![self, allowsFloats]
    }
    pub unsafe fn setAllowsFloats(&self, allowsFloats: bool) {
        msg_send![self, setAllowsFloats: allowsFloats]
    }
    pub unsafe fn decimalSeparator(&self) -> Id<NSString, Shared> {
        msg_send_id![self, decimalSeparator]
    }
    pub unsafe fn setDecimalSeparator(&self, decimalSeparator: Option<&NSString>) {
        msg_send![self, setDecimalSeparator: decimalSeparator]
    }
    pub unsafe fn alwaysShowsDecimalSeparator(&self) -> bool {
        msg_send![self, alwaysShowsDecimalSeparator]
    }
    pub unsafe fn setAlwaysShowsDecimalSeparator(&self, alwaysShowsDecimalSeparator: bool) {
        msg_send![
            self,
            setAlwaysShowsDecimalSeparator: alwaysShowsDecimalSeparator
        ]
    }
    pub unsafe fn currencyDecimalSeparator(&self) -> Id<NSString, Shared> {
        msg_send_id![self, currencyDecimalSeparator]
    }
    pub unsafe fn setCurrencyDecimalSeparator(&self, currencyDecimalSeparator: Option<&NSString>) {
        msg_send![self, setCurrencyDecimalSeparator: currencyDecimalSeparator]
    }
    pub unsafe fn usesGroupingSeparator(&self) -> bool {
        msg_send![self, usesGroupingSeparator]
    }
    pub unsafe fn setUsesGroupingSeparator(&self, usesGroupingSeparator: bool) {
        msg_send![self, setUsesGroupingSeparator: usesGroupingSeparator]
    }
    pub unsafe fn groupingSeparator(&self) -> Id<NSString, Shared> {
        msg_send_id![self, groupingSeparator]
    }
    pub unsafe fn setGroupingSeparator(&self, groupingSeparator: Option<&NSString>) {
        msg_send![self, setGroupingSeparator: groupingSeparator]
    }
    pub unsafe fn zeroSymbol(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, zeroSymbol]
    }
    pub unsafe fn setZeroSymbol(&self, zeroSymbol: Option<&NSString>) {
        msg_send![self, setZeroSymbol: zeroSymbol]
    }
    pub unsafe fn textAttributesForZero(
        &self,
    ) -> Option<Id<NSDictionary<NSString, Object>, Shared>> {
        msg_send_id![self, textAttributesForZero]
    }
    pub unsafe fn setTextAttributesForZero(
        &self,
        textAttributesForZero: Option<&NSDictionary<NSString, Object>>,
    ) {
        msg_send![self, setTextAttributesForZero: textAttributesForZero]
    }
    pub unsafe fn nilSymbol(&self) -> Id<NSString, Shared> {
        msg_send_id![self, nilSymbol]
    }
    pub unsafe fn setNilSymbol(&self, nilSymbol: &NSString) {
        msg_send![self, setNilSymbol: nilSymbol]
    }
    pub unsafe fn textAttributesForNil(
        &self,
    ) -> Option<Id<NSDictionary<NSString, Object>, Shared>> {
        msg_send_id![self, textAttributesForNil]
    }
    pub unsafe fn setTextAttributesForNil(
        &self,
        textAttributesForNil: Option<&NSDictionary<NSString, Object>>,
    ) {
        msg_send![self, setTextAttributesForNil: textAttributesForNil]
    }
    pub unsafe fn notANumberSymbol(&self) -> Id<NSString, Shared> {
        msg_send_id![self, notANumberSymbol]
    }
    pub unsafe fn setNotANumberSymbol(&self, notANumberSymbol: Option<&NSString>) {
        msg_send![self, setNotANumberSymbol: notANumberSymbol]
    }
    pub unsafe fn textAttributesForNotANumber(
        &self,
    ) -> Option<Id<NSDictionary<NSString, Object>, Shared>> {
        msg_send_id![self, textAttributesForNotANumber]
    }
    pub unsafe fn setTextAttributesForNotANumber(
        &self,
        textAttributesForNotANumber: Option<&NSDictionary<NSString, Object>>,
    ) {
        msg_send![
            self,
            setTextAttributesForNotANumber: textAttributesForNotANumber
        ]
    }
    pub unsafe fn positiveInfinitySymbol(&self) -> Id<NSString, Shared> {
        msg_send_id![self, positiveInfinitySymbol]
    }
    pub unsafe fn setPositiveInfinitySymbol(&self, positiveInfinitySymbol: &NSString) {
        msg_send![self, setPositiveInfinitySymbol: positiveInfinitySymbol]
    }
    pub unsafe fn textAttributesForPositiveInfinity(
        &self,
    ) -> Option<Id<NSDictionary<NSString, Object>, Shared>> {
        msg_send_id![self, textAttributesForPositiveInfinity]
    }
    pub unsafe fn setTextAttributesForPositiveInfinity(
        &self,
        textAttributesForPositiveInfinity: Option<&NSDictionary<NSString, Object>>,
    ) {
        msg_send![
            self,
            setTextAttributesForPositiveInfinity: textAttributesForPositiveInfinity
        ]
    }
    pub unsafe fn negativeInfinitySymbol(&self) -> Id<NSString, Shared> {
        msg_send_id![self, negativeInfinitySymbol]
    }
    pub unsafe fn setNegativeInfinitySymbol(&self, negativeInfinitySymbol: &NSString) {
        msg_send![self, setNegativeInfinitySymbol: negativeInfinitySymbol]
    }
    pub unsafe fn textAttributesForNegativeInfinity(
        &self,
    ) -> Option<Id<NSDictionary<NSString, Object>, Shared>> {
        msg_send_id![self, textAttributesForNegativeInfinity]
    }
    pub unsafe fn setTextAttributesForNegativeInfinity(
        &self,
        textAttributesForNegativeInfinity: Option<&NSDictionary<NSString, Object>>,
    ) {
        msg_send![
            self,
            setTextAttributesForNegativeInfinity: textAttributesForNegativeInfinity
        ]
    }
    pub unsafe fn positivePrefix(&self) -> Id<NSString, Shared> {
        msg_send_id![self, positivePrefix]
    }
    pub unsafe fn setPositivePrefix(&self, positivePrefix: Option<&NSString>) {
        msg_send![self, setPositivePrefix: positivePrefix]
    }
    pub unsafe fn positiveSuffix(&self) -> Id<NSString, Shared> {
        msg_send_id![self, positiveSuffix]
    }
    pub unsafe fn setPositiveSuffix(&self, positiveSuffix: Option<&NSString>) {
        msg_send![self, setPositiveSuffix: positiveSuffix]
    }
    pub unsafe fn negativePrefix(&self) -> Id<NSString, Shared> {
        msg_send_id![self, negativePrefix]
    }
    pub unsafe fn setNegativePrefix(&self, negativePrefix: Option<&NSString>) {
        msg_send![self, setNegativePrefix: negativePrefix]
    }
    pub unsafe fn negativeSuffix(&self) -> Id<NSString, Shared> {
        msg_send_id![self, negativeSuffix]
    }
    pub unsafe fn setNegativeSuffix(&self, negativeSuffix: Option<&NSString>) {
        msg_send![self, setNegativeSuffix: negativeSuffix]
    }
    pub unsafe fn currencyCode(&self) -> Id<NSString, Shared> {
        msg_send_id![self, currencyCode]
    }
    pub unsafe fn setCurrencyCode(&self, currencyCode: Option<&NSString>) {
        msg_send![self, setCurrencyCode: currencyCode]
    }
    pub unsafe fn currencySymbol(&self) -> Id<NSString, Shared> {
        msg_send_id![self, currencySymbol]
    }
    pub unsafe fn setCurrencySymbol(&self, currencySymbol: Option<&NSString>) {
        msg_send![self, setCurrencySymbol: currencySymbol]
    }
    pub unsafe fn internationalCurrencySymbol(&self) -> Id<NSString, Shared> {
        msg_send_id![self, internationalCurrencySymbol]
    }
    pub unsafe fn setInternationalCurrencySymbol(
        &self,
        internationalCurrencySymbol: Option<&NSString>,
    ) {
        msg_send![
            self,
            setInternationalCurrencySymbol: internationalCurrencySymbol
        ]
    }
    pub unsafe fn percentSymbol(&self) -> Id<NSString, Shared> {
        msg_send_id![self, percentSymbol]
    }
    pub unsafe fn setPercentSymbol(&self, percentSymbol: Option<&NSString>) {
        msg_send![self, setPercentSymbol: percentSymbol]
    }
    pub unsafe fn perMillSymbol(&self) -> Id<NSString, Shared> {
        msg_send_id![self, perMillSymbol]
    }
    pub unsafe fn setPerMillSymbol(&self, perMillSymbol: Option<&NSString>) {
        msg_send![self, setPerMillSymbol: perMillSymbol]
    }
    pub unsafe fn minusSign(&self) -> Id<NSString, Shared> {
        msg_send_id![self, minusSign]
    }
    pub unsafe fn setMinusSign(&self, minusSign: Option<&NSString>) {
        msg_send![self, setMinusSign: minusSign]
    }
    pub unsafe fn plusSign(&self) -> Id<NSString, Shared> {
        msg_send_id![self, plusSign]
    }
    pub unsafe fn setPlusSign(&self, plusSign: Option<&NSString>) {
        msg_send![self, setPlusSign: plusSign]
    }
    pub unsafe fn exponentSymbol(&self) -> Id<NSString, Shared> {
        msg_send_id![self, exponentSymbol]
    }
    pub unsafe fn setExponentSymbol(&self, exponentSymbol: Option<&NSString>) {
        msg_send![self, setExponentSymbol: exponentSymbol]
    }
    pub unsafe fn groupingSize(&self) -> NSUInteger {
        msg_send![self, groupingSize]
    }
    pub unsafe fn setGroupingSize(&self, groupingSize: NSUInteger) {
        msg_send![self, setGroupingSize: groupingSize]
    }
    pub unsafe fn secondaryGroupingSize(&self) -> NSUInteger {
        msg_send![self, secondaryGroupingSize]
    }
    pub unsafe fn setSecondaryGroupingSize(&self, secondaryGroupingSize: NSUInteger) {
        msg_send![self, setSecondaryGroupingSize: secondaryGroupingSize]
    }
    pub unsafe fn multiplier(&self) -> Option<Id<NSNumber, Shared>> {
        msg_send_id![self, multiplier]
    }
    pub unsafe fn setMultiplier(&self, multiplier: Option<&NSNumber>) {
        msg_send![self, setMultiplier: multiplier]
    }
    pub unsafe fn formatWidth(&self) -> NSUInteger {
        msg_send![self, formatWidth]
    }
    pub unsafe fn setFormatWidth(&self, formatWidth: NSUInteger) {
        msg_send![self, setFormatWidth: formatWidth]
    }
    pub unsafe fn paddingCharacter(&self) -> Id<NSString, Shared> {
        msg_send_id![self, paddingCharacter]
    }
    pub unsafe fn setPaddingCharacter(&self, paddingCharacter: Option<&NSString>) {
        msg_send![self, setPaddingCharacter: paddingCharacter]
    }
    pub unsafe fn paddingPosition(&self) -> NSNumberFormatterPadPosition {
        msg_send![self, paddingPosition]
    }
    pub unsafe fn setPaddingPosition(&self, paddingPosition: NSNumberFormatterPadPosition) {
        msg_send![self, setPaddingPosition: paddingPosition]
    }
    pub unsafe fn roundingMode(&self) -> NSNumberFormatterRoundingMode {
        msg_send![self, roundingMode]
    }
    pub unsafe fn setRoundingMode(&self, roundingMode: NSNumberFormatterRoundingMode) {
        msg_send![self, setRoundingMode: roundingMode]
    }
    pub unsafe fn roundingIncrement(&self) -> Id<NSNumber, Shared> {
        msg_send_id![self, roundingIncrement]
    }
    pub unsafe fn setRoundingIncrement(&self, roundingIncrement: Option<&NSNumber>) {
        msg_send![self, setRoundingIncrement: roundingIncrement]
    }
    pub unsafe fn minimumIntegerDigits(&self) -> NSUInteger {
        msg_send![self, minimumIntegerDigits]
    }
    pub unsafe fn setMinimumIntegerDigits(&self, minimumIntegerDigits: NSUInteger) {
        msg_send![self, setMinimumIntegerDigits: minimumIntegerDigits]
    }
    pub unsafe fn maximumIntegerDigits(&self) -> NSUInteger {
        msg_send![self, maximumIntegerDigits]
    }
    pub unsafe fn setMaximumIntegerDigits(&self, maximumIntegerDigits: NSUInteger) {
        msg_send![self, setMaximumIntegerDigits: maximumIntegerDigits]
    }
    pub unsafe fn minimumFractionDigits(&self) -> NSUInteger {
        msg_send![self, minimumFractionDigits]
    }
    pub unsafe fn setMinimumFractionDigits(&self, minimumFractionDigits: NSUInteger) {
        msg_send![self, setMinimumFractionDigits: minimumFractionDigits]
    }
    pub unsafe fn maximumFractionDigits(&self) -> NSUInteger {
        msg_send![self, maximumFractionDigits]
    }
    pub unsafe fn setMaximumFractionDigits(&self, maximumFractionDigits: NSUInteger) {
        msg_send![self, setMaximumFractionDigits: maximumFractionDigits]
    }
    pub unsafe fn minimum(&self) -> Option<Id<NSNumber, Shared>> {
        msg_send_id![self, minimum]
    }
    pub unsafe fn setMinimum(&self, minimum: Option<&NSNumber>) {
        msg_send![self, setMinimum: minimum]
    }
    pub unsafe fn maximum(&self) -> Option<Id<NSNumber, Shared>> {
        msg_send_id![self, maximum]
    }
    pub unsafe fn setMaximum(&self, maximum: Option<&NSNumber>) {
        msg_send![self, setMaximum: maximum]
    }
    pub unsafe fn currencyGroupingSeparator(&self) -> Id<NSString, Shared> {
        msg_send_id![self, currencyGroupingSeparator]
    }
    pub unsafe fn setCurrencyGroupingSeparator(
        &self,
        currencyGroupingSeparator: Option<&NSString>,
    ) {
        msg_send![
            self,
            setCurrencyGroupingSeparator: currencyGroupingSeparator
        ]
    }
    pub unsafe fn isLenient(&self) -> bool {
        msg_send![self, isLenient]
    }
    pub unsafe fn setLenient(&self, lenient: bool) {
        msg_send![self, setLenient: lenient]
    }
    pub unsafe fn usesSignificantDigits(&self) -> bool {
        msg_send![self, usesSignificantDigits]
    }
    pub unsafe fn setUsesSignificantDigits(&self, usesSignificantDigits: bool) {
        msg_send![self, setUsesSignificantDigits: usesSignificantDigits]
    }
    pub unsafe fn minimumSignificantDigits(&self) -> NSUInteger {
        msg_send![self, minimumSignificantDigits]
    }
    pub unsafe fn setMinimumSignificantDigits(&self, minimumSignificantDigits: NSUInteger) {
        msg_send![self, setMinimumSignificantDigits: minimumSignificantDigits]
    }
    pub unsafe fn maximumSignificantDigits(&self) -> NSUInteger {
        msg_send![self, maximumSignificantDigits]
    }
    pub unsafe fn setMaximumSignificantDigits(&self, maximumSignificantDigits: NSUInteger) {
        msg_send![self, setMaximumSignificantDigits: maximumSignificantDigits]
    }
    pub unsafe fn isPartialStringValidationEnabled(&self) -> bool {
        msg_send![self, isPartialStringValidationEnabled]
    }
    pub unsafe fn setPartialStringValidationEnabled(&self, partialStringValidationEnabled: bool) {
        msg_send![
            self,
            setPartialStringValidationEnabled: partialStringValidationEnabled
        ]
    }
}
use super::__exported::NSDecimalNumberHandler;
#[doc = "NSNumberFormatterCompatibility"]
impl NSNumberFormatter {
    pub unsafe fn hasThousandSeparators(&self) -> bool {
        msg_send![self, hasThousandSeparators]
    }
    pub unsafe fn setHasThousandSeparators(&self, hasThousandSeparators: bool) {
        msg_send![self, setHasThousandSeparators: hasThousandSeparators]
    }
    pub unsafe fn thousandSeparator(&self) -> Id<NSString, Shared> {
        msg_send_id![self, thousandSeparator]
    }
    pub unsafe fn setThousandSeparator(&self, thousandSeparator: Option<&NSString>) {
        msg_send![self, setThousandSeparator: thousandSeparator]
    }
    pub unsafe fn localizesFormat(&self) -> bool {
        msg_send![self, localizesFormat]
    }
    pub unsafe fn setLocalizesFormat(&self, localizesFormat: bool) {
        msg_send![self, setLocalizesFormat: localizesFormat]
    }
    pub unsafe fn format(&self) -> Id<NSString, Shared> {
        msg_send_id![self, format]
    }
    pub unsafe fn setFormat(&self, format: &NSString) {
        msg_send![self, setFormat: format]
    }
    pub unsafe fn attributedStringForZero(&self) -> Id<NSAttributedString, Shared> {
        msg_send_id![self, attributedStringForZero]
    }
    pub unsafe fn setAttributedStringForZero(&self, attributedStringForZero: &NSAttributedString) {
        msg_send![self, setAttributedStringForZero: attributedStringForZero]
    }
    pub unsafe fn attributedStringForNil(&self) -> Id<NSAttributedString, Shared> {
        msg_send_id![self, attributedStringForNil]
    }
    pub unsafe fn setAttributedStringForNil(&self, attributedStringForNil: &NSAttributedString) {
        msg_send![self, setAttributedStringForNil: attributedStringForNil]
    }
    pub unsafe fn attributedStringForNotANumber(&self) -> Id<NSAttributedString, Shared> {
        msg_send_id![self, attributedStringForNotANumber]
    }
    pub unsafe fn setAttributedStringForNotANumber(
        &self,
        attributedStringForNotANumber: &NSAttributedString,
    ) {
        msg_send![
            self,
            setAttributedStringForNotANumber: attributedStringForNotANumber
        ]
    }
    pub unsafe fn roundingBehavior(&self) -> Id<NSDecimalNumberHandler, Shared> {
        msg_send_id![self, roundingBehavior]
    }
    pub unsafe fn setRoundingBehavior(&self, roundingBehavior: &NSDecimalNumberHandler) {
        msg_send![self, setRoundingBehavior: roundingBehavior]
    }
}
