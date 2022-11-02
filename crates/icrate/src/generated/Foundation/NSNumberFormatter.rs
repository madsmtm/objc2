//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

pub type NSNumberFormatterBehavior = NSUInteger;
pub const NSNumberFormatterBehaviorDefault: NSNumberFormatterBehavior = 0;
pub const NSNumberFormatterBehavior10_0: NSNumberFormatterBehavior = 1000;
pub const NSNumberFormatterBehavior10_4: NSNumberFormatterBehavior = 1040;

pub type NSNumberFormatterStyle = NSUInteger;
pub const NSNumberFormatterNoStyle: NSNumberFormatterStyle = 0;
pub const NSNumberFormatterDecimalStyle: NSNumberFormatterStyle = 1;
pub const NSNumberFormatterCurrencyStyle: NSNumberFormatterStyle = 2;
pub const NSNumberFormatterPercentStyle: NSNumberFormatterStyle = 3;
pub const NSNumberFormatterScientificStyle: NSNumberFormatterStyle = 4;
pub const NSNumberFormatterSpellOutStyle: NSNumberFormatterStyle = 5;
pub const NSNumberFormatterOrdinalStyle: NSNumberFormatterStyle = 6;
pub const NSNumberFormatterCurrencyISOCodeStyle: NSNumberFormatterStyle = 8;
pub const NSNumberFormatterCurrencyPluralStyle: NSNumberFormatterStyle = 9;
pub const NSNumberFormatterCurrencyAccountingStyle: NSNumberFormatterStyle = 10;

pub type NSNumberFormatterPadPosition = NSUInteger;
pub const NSNumberFormatterPadBeforePrefix: NSNumberFormatterPadPosition = 0;
pub const NSNumberFormatterPadAfterPrefix: NSNumberFormatterPadPosition = 1;
pub const NSNumberFormatterPadBeforeSuffix: NSNumberFormatterPadPosition = 2;
pub const NSNumberFormatterPadAfterSuffix: NSNumberFormatterPadPosition = 3;

pub type NSNumberFormatterRoundingMode = NSUInteger;
pub const NSNumberFormatterRoundCeiling: NSNumberFormatterRoundingMode = 0;
pub const NSNumberFormatterRoundFloor: NSNumberFormatterRoundingMode = 1;
pub const NSNumberFormatterRoundDown: NSNumberFormatterRoundingMode = 2;
pub const NSNumberFormatterRoundUp: NSNumberFormatterRoundingMode = 3;
pub const NSNumberFormatterRoundHalfEven: NSNumberFormatterRoundingMode = 4;
pub const NSNumberFormatterRoundHalfDown: NSNumberFormatterRoundingMode = 5;
pub const NSNumberFormatterRoundHalfUp: NSNumberFormatterRoundingMode = 6;

extern_class!(
    #[derive(Debug)]
    pub struct NSNumberFormatter;

    unsafe impl ClassType for NSNumberFormatter {
        type Super = NSFormatter;
    }
);

extern_methods!(
    unsafe impl NSNumberFormatter {
        #[method(formattingContext)]
        pub unsafe fn formattingContext(&self) -> NSFormattingContext;

        #[method(setFormattingContext:)]
        pub unsafe fn setFormattingContext(&self, formattingContext: NSFormattingContext);

        #[method(getObjectValue:forString:range:error:)]
        pub unsafe fn getObjectValue_forString_range_error(
            &self,
            obj: *mut *mut Object,
            string: &NSString,
            rangep: *mut NSRange,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other stringFromNumber:)]
        pub unsafe fn stringFromNumber(&self, number: &NSNumber) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other numberFromString:)]
        pub unsafe fn numberFromString(&self, string: &NSString) -> Option<Id<NSNumber, Shared>>;

        #[method_id(@__retain_semantics Other localizedStringFromNumber:numberStyle:)]
        pub unsafe fn localizedStringFromNumber_numberStyle(
            num: &NSNumber,
            nstyle: NSNumberFormatterStyle,
        ) -> Id<NSString, Shared>;

        #[method(defaultFormatterBehavior)]
        pub unsafe fn defaultFormatterBehavior() -> NSNumberFormatterBehavior;

        #[method(setDefaultFormatterBehavior:)]
        pub unsafe fn setDefaultFormatterBehavior(behavior: NSNumberFormatterBehavior);

        #[method(numberStyle)]
        pub unsafe fn numberStyle(&self) -> NSNumberFormatterStyle;

        #[method(setNumberStyle:)]
        pub unsafe fn setNumberStyle(&self, numberStyle: NSNumberFormatterStyle);

        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Id<NSLocale, Shared>;

        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[method(generatesDecimalNumbers)]
        pub unsafe fn generatesDecimalNumbers(&self) -> bool;

        #[method(setGeneratesDecimalNumbers:)]
        pub unsafe fn setGeneratesDecimalNumbers(&self, generatesDecimalNumbers: bool);

        #[method(formatterBehavior)]
        pub unsafe fn formatterBehavior(&self) -> NSNumberFormatterBehavior;

        #[method(setFormatterBehavior:)]
        pub unsafe fn setFormatterBehavior(&self, formatterBehavior: NSNumberFormatterBehavior);

        #[method_id(@__retain_semantics Other negativeFormat)]
        pub unsafe fn negativeFormat(&self) -> Id<NSString, Shared>;

        #[method(setNegativeFormat:)]
        pub unsafe fn setNegativeFormat(&self, negativeFormat: Option<&NSString>);

        #[method_id(@__retain_semantics Other textAttributesForNegativeValues)]
        pub unsafe fn textAttributesForNegativeValues(
            &self,
        ) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;

        #[method(setTextAttributesForNegativeValues:)]
        pub unsafe fn setTextAttributesForNegativeValues(
            &self,
            textAttributesForNegativeValues: Option<&NSDictionary<NSString, Object>>,
        );

        #[method_id(@__retain_semantics Other positiveFormat)]
        pub unsafe fn positiveFormat(&self) -> Id<NSString, Shared>;

        #[method(setPositiveFormat:)]
        pub unsafe fn setPositiveFormat(&self, positiveFormat: Option<&NSString>);

        #[method_id(@__retain_semantics Other textAttributesForPositiveValues)]
        pub unsafe fn textAttributesForPositiveValues(
            &self,
        ) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;

        #[method(setTextAttributesForPositiveValues:)]
        pub unsafe fn setTextAttributesForPositiveValues(
            &self,
            textAttributesForPositiveValues: Option<&NSDictionary<NSString, Object>>,
        );

        #[method(allowsFloats)]
        pub unsafe fn allowsFloats(&self) -> bool;

        #[method(setAllowsFloats:)]
        pub unsafe fn setAllowsFloats(&self, allowsFloats: bool);

        #[method_id(@__retain_semantics Other decimalSeparator)]
        pub unsafe fn decimalSeparator(&self) -> Id<NSString, Shared>;

        #[method(setDecimalSeparator:)]
        pub unsafe fn setDecimalSeparator(&self, decimalSeparator: Option<&NSString>);

        #[method(alwaysShowsDecimalSeparator)]
        pub unsafe fn alwaysShowsDecimalSeparator(&self) -> bool;

        #[method(setAlwaysShowsDecimalSeparator:)]
        pub unsafe fn setAlwaysShowsDecimalSeparator(&self, alwaysShowsDecimalSeparator: bool);

        #[method_id(@__retain_semantics Other currencyDecimalSeparator)]
        pub unsafe fn currencyDecimalSeparator(&self) -> Id<NSString, Shared>;

        #[method(setCurrencyDecimalSeparator:)]
        pub unsafe fn setCurrencyDecimalSeparator(
            &self,
            currencyDecimalSeparator: Option<&NSString>,
        );

        #[method(usesGroupingSeparator)]
        pub unsafe fn usesGroupingSeparator(&self) -> bool;

        #[method(setUsesGroupingSeparator:)]
        pub unsafe fn setUsesGroupingSeparator(&self, usesGroupingSeparator: bool);

        #[method_id(@__retain_semantics Other groupingSeparator)]
        pub unsafe fn groupingSeparator(&self) -> Id<NSString, Shared>;

        #[method(setGroupingSeparator:)]
        pub unsafe fn setGroupingSeparator(&self, groupingSeparator: Option<&NSString>);

        #[method_id(@__retain_semantics Other zeroSymbol)]
        pub unsafe fn zeroSymbol(&self) -> Option<Id<NSString, Shared>>;

        #[method(setZeroSymbol:)]
        pub unsafe fn setZeroSymbol(&self, zeroSymbol: Option<&NSString>);

        #[method_id(@__retain_semantics Other textAttributesForZero)]
        pub unsafe fn textAttributesForZero(
            &self,
        ) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;

        #[method(setTextAttributesForZero:)]
        pub unsafe fn setTextAttributesForZero(
            &self,
            textAttributesForZero: Option<&NSDictionary<NSString, Object>>,
        );

        #[method_id(@__retain_semantics Other nilSymbol)]
        pub unsafe fn nilSymbol(&self) -> Id<NSString, Shared>;

        #[method(setNilSymbol:)]
        pub unsafe fn setNilSymbol(&self, nilSymbol: &NSString);

        #[method_id(@__retain_semantics Other textAttributesForNil)]
        pub unsafe fn textAttributesForNil(
            &self,
        ) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;

        #[method(setTextAttributesForNil:)]
        pub unsafe fn setTextAttributesForNil(
            &self,
            textAttributesForNil: Option<&NSDictionary<NSString, Object>>,
        );

        #[method_id(@__retain_semantics Other notANumberSymbol)]
        pub unsafe fn notANumberSymbol(&self) -> Id<NSString, Shared>;

        #[method(setNotANumberSymbol:)]
        pub unsafe fn setNotANumberSymbol(&self, notANumberSymbol: Option<&NSString>);

        #[method_id(@__retain_semantics Other textAttributesForNotANumber)]
        pub unsafe fn textAttributesForNotANumber(
            &self,
        ) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;

        #[method(setTextAttributesForNotANumber:)]
        pub unsafe fn setTextAttributesForNotANumber(
            &self,
            textAttributesForNotANumber: Option<&NSDictionary<NSString, Object>>,
        );

        #[method_id(@__retain_semantics Other positiveInfinitySymbol)]
        pub unsafe fn positiveInfinitySymbol(&self) -> Id<NSString, Shared>;

        #[method(setPositiveInfinitySymbol:)]
        pub unsafe fn setPositiveInfinitySymbol(&self, positiveInfinitySymbol: &NSString);

        #[method_id(@__retain_semantics Other textAttributesForPositiveInfinity)]
        pub unsafe fn textAttributesForPositiveInfinity(
            &self,
        ) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;

        #[method(setTextAttributesForPositiveInfinity:)]
        pub unsafe fn setTextAttributesForPositiveInfinity(
            &self,
            textAttributesForPositiveInfinity: Option<&NSDictionary<NSString, Object>>,
        );

        #[method_id(@__retain_semantics Other negativeInfinitySymbol)]
        pub unsafe fn negativeInfinitySymbol(&self) -> Id<NSString, Shared>;

        #[method(setNegativeInfinitySymbol:)]
        pub unsafe fn setNegativeInfinitySymbol(&self, negativeInfinitySymbol: &NSString);

        #[method_id(@__retain_semantics Other textAttributesForNegativeInfinity)]
        pub unsafe fn textAttributesForNegativeInfinity(
            &self,
        ) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;

        #[method(setTextAttributesForNegativeInfinity:)]
        pub unsafe fn setTextAttributesForNegativeInfinity(
            &self,
            textAttributesForNegativeInfinity: Option<&NSDictionary<NSString, Object>>,
        );

        #[method_id(@__retain_semantics Other positivePrefix)]
        pub unsafe fn positivePrefix(&self) -> Id<NSString, Shared>;

        #[method(setPositivePrefix:)]
        pub unsafe fn setPositivePrefix(&self, positivePrefix: Option<&NSString>);

        #[method_id(@__retain_semantics Other positiveSuffix)]
        pub unsafe fn positiveSuffix(&self) -> Id<NSString, Shared>;

        #[method(setPositiveSuffix:)]
        pub unsafe fn setPositiveSuffix(&self, positiveSuffix: Option<&NSString>);

        #[method_id(@__retain_semantics Other negativePrefix)]
        pub unsafe fn negativePrefix(&self) -> Id<NSString, Shared>;

        #[method(setNegativePrefix:)]
        pub unsafe fn setNegativePrefix(&self, negativePrefix: Option<&NSString>);

        #[method_id(@__retain_semantics Other negativeSuffix)]
        pub unsafe fn negativeSuffix(&self) -> Id<NSString, Shared>;

        #[method(setNegativeSuffix:)]
        pub unsafe fn setNegativeSuffix(&self, negativeSuffix: Option<&NSString>);

        #[method_id(@__retain_semantics Other currencyCode)]
        pub unsafe fn currencyCode(&self) -> Id<NSString, Shared>;

        #[method(setCurrencyCode:)]
        pub unsafe fn setCurrencyCode(&self, currencyCode: Option<&NSString>);

        #[method_id(@__retain_semantics Other currencySymbol)]
        pub unsafe fn currencySymbol(&self) -> Id<NSString, Shared>;

        #[method(setCurrencySymbol:)]
        pub unsafe fn setCurrencySymbol(&self, currencySymbol: Option<&NSString>);

        #[method_id(@__retain_semantics Other internationalCurrencySymbol)]
        pub unsafe fn internationalCurrencySymbol(&self) -> Id<NSString, Shared>;

        #[method(setInternationalCurrencySymbol:)]
        pub unsafe fn setInternationalCurrencySymbol(
            &self,
            internationalCurrencySymbol: Option<&NSString>,
        );

        #[method_id(@__retain_semantics Other percentSymbol)]
        pub unsafe fn percentSymbol(&self) -> Id<NSString, Shared>;

        #[method(setPercentSymbol:)]
        pub unsafe fn setPercentSymbol(&self, percentSymbol: Option<&NSString>);

        #[method_id(@__retain_semantics Other perMillSymbol)]
        pub unsafe fn perMillSymbol(&self) -> Id<NSString, Shared>;

        #[method(setPerMillSymbol:)]
        pub unsafe fn setPerMillSymbol(&self, perMillSymbol: Option<&NSString>);

        #[method_id(@__retain_semantics Other minusSign)]
        pub unsafe fn minusSign(&self) -> Id<NSString, Shared>;

        #[method(setMinusSign:)]
        pub unsafe fn setMinusSign(&self, minusSign: Option<&NSString>);

        #[method_id(@__retain_semantics Other plusSign)]
        pub unsafe fn plusSign(&self) -> Id<NSString, Shared>;

        #[method(setPlusSign:)]
        pub unsafe fn setPlusSign(&self, plusSign: Option<&NSString>);

        #[method_id(@__retain_semantics Other exponentSymbol)]
        pub unsafe fn exponentSymbol(&self) -> Id<NSString, Shared>;

        #[method(setExponentSymbol:)]
        pub unsafe fn setExponentSymbol(&self, exponentSymbol: Option<&NSString>);

        #[method(groupingSize)]
        pub unsafe fn groupingSize(&self) -> NSUInteger;

        #[method(setGroupingSize:)]
        pub unsafe fn setGroupingSize(&self, groupingSize: NSUInteger);

        #[method(secondaryGroupingSize)]
        pub unsafe fn secondaryGroupingSize(&self) -> NSUInteger;

        #[method(setSecondaryGroupingSize:)]
        pub unsafe fn setSecondaryGroupingSize(&self, secondaryGroupingSize: NSUInteger);

        #[method_id(@__retain_semantics Other multiplier)]
        pub unsafe fn multiplier(&self) -> Option<Id<NSNumber, Shared>>;

        #[method(setMultiplier:)]
        pub unsafe fn setMultiplier(&self, multiplier: Option<&NSNumber>);

        #[method(formatWidth)]
        pub unsafe fn formatWidth(&self) -> NSUInteger;

        #[method(setFormatWidth:)]
        pub unsafe fn setFormatWidth(&self, formatWidth: NSUInteger);

        #[method_id(@__retain_semantics Other paddingCharacter)]
        pub unsafe fn paddingCharacter(&self) -> Id<NSString, Shared>;

        #[method(setPaddingCharacter:)]
        pub unsafe fn setPaddingCharacter(&self, paddingCharacter: Option<&NSString>);

        #[method(paddingPosition)]
        pub unsafe fn paddingPosition(&self) -> NSNumberFormatterPadPosition;

        #[method(setPaddingPosition:)]
        pub unsafe fn setPaddingPosition(&self, paddingPosition: NSNumberFormatterPadPosition);

        #[method(roundingMode)]
        pub unsafe fn roundingMode(&self) -> NSNumberFormatterRoundingMode;

        #[method(setRoundingMode:)]
        pub unsafe fn setRoundingMode(&self, roundingMode: NSNumberFormatterRoundingMode);

        #[method_id(@__retain_semantics Other roundingIncrement)]
        pub unsafe fn roundingIncrement(&self) -> Id<NSNumber, Shared>;

        #[method(setRoundingIncrement:)]
        pub unsafe fn setRoundingIncrement(&self, roundingIncrement: Option<&NSNumber>);

        #[method(minimumIntegerDigits)]
        pub unsafe fn minimumIntegerDigits(&self) -> NSUInteger;

        #[method(setMinimumIntegerDigits:)]
        pub unsafe fn setMinimumIntegerDigits(&self, minimumIntegerDigits: NSUInteger);

        #[method(maximumIntegerDigits)]
        pub unsafe fn maximumIntegerDigits(&self) -> NSUInteger;

        #[method(setMaximumIntegerDigits:)]
        pub unsafe fn setMaximumIntegerDigits(&self, maximumIntegerDigits: NSUInteger);

        #[method(minimumFractionDigits)]
        pub unsafe fn minimumFractionDigits(&self) -> NSUInteger;

        #[method(setMinimumFractionDigits:)]
        pub unsafe fn setMinimumFractionDigits(&self, minimumFractionDigits: NSUInteger);

        #[method(maximumFractionDigits)]
        pub unsafe fn maximumFractionDigits(&self) -> NSUInteger;

        #[method(setMaximumFractionDigits:)]
        pub unsafe fn setMaximumFractionDigits(&self, maximumFractionDigits: NSUInteger);

        #[method_id(@__retain_semantics Other minimum)]
        pub unsafe fn minimum(&self) -> Option<Id<NSNumber, Shared>>;

        #[method(setMinimum:)]
        pub unsafe fn setMinimum(&self, minimum: Option<&NSNumber>);

        #[method_id(@__retain_semantics Other maximum)]
        pub unsafe fn maximum(&self) -> Option<Id<NSNumber, Shared>>;

        #[method(setMaximum:)]
        pub unsafe fn setMaximum(&self, maximum: Option<&NSNumber>);

        #[method_id(@__retain_semantics Other currencyGroupingSeparator)]
        pub unsafe fn currencyGroupingSeparator(&self) -> Id<NSString, Shared>;

        #[method(setCurrencyGroupingSeparator:)]
        pub unsafe fn setCurrencyGroupingSeparator(
            &self,
            currencyGroupingSeparator: Option<&NSString>,
        );

        #[method(isLenient)]
        pub unsafe fn isLenient(&self) -> bool;

        #[method(setLenient:)]
        pub unsafe fn setLenient(&self, lenient: bool);

        #[method(usesSignificantDigits)]
        pub unsafe fn usesSignificantDigits(&self) -> bool;

        #[method(setUsesSignificantDigits:)]
        pub unsafe fn setUsesSignificantDigits(&self, usesSignificantDigits: bool);

        #[method(minimumSignificantDigits)]
        pub unsafe fn minimumSignificantDigits(&self) -> NSUInteger;

        #[method(setMinimumSignificantDigits:)]
        pub unsafe fn setMinimumSignificantDigits(&self, minimumSignificantDigits: NSUInteger);

        #[method(maximumSignificantDigits)]
        pub unsafe fn maximumSignificantDigits(&self) -> NSUInteger;

        #[method(setMaximumSignificantDigits:)]
        pub unsafe fn setMaximumSignificantDigits(&self, maximumSignificantDigits: NSUInteger);

        #[method(isPartialStringValidationEnabled)]
        pub unsafe fn isPartialStringValidationEnabled(&self) -> bool;

        #[method(setPartialStringValidationEnabled:)]
        pub unsafe fn setPartialStringValidationEnabled(
            &self,
            partialStringValidationEnabled: bool,
        );
    }
);

extern_methods!(
    /// NSNumberFormatterCompatibility
    unsafe impl NSNumberFormatter {
        #[method(hasThousandSeparators)]
        pub unsafe fn hasThousandSeparators(&self) -> bool;

        #[method(setHasThousandSeparators:)]
        pub unsafe fn setHasThousandSeparators(&self, hasThousandSeparators: bool);

        #[method_id(@__retain_semantics Other thousandSeparator)]
        pub unsafe fn thousandSeparator(&self) -> Id<NSString, Shared>;

        #[method(setThousandSeparator:)]
        pub unsafe fn setThousandSeparator(&self, thousandSeparator: Option<&NSString>);

        #[method(localizesFormat)]
        pub unsafe fn localizesFormat(&self) -> bool;

        #[method(setLocalizesFormat:)]
        pub unsafe fn setLocalizesFormat(&self, localizesFormat: bool);

        #[method_id(@__retain_semantics Other format)]
        pub unsafe fn format(&self) -> Id<NSString, Shared>;

        #[method(setFormat:)]
        pub unsafe fn setFormat(&self, format: &NSString);

        #[method_id(@__retain_semantics Other attributedStringForZero)]
        pub unsafe fn attributedStringForZero(&self) -> Id<NSAttributedString, Shared>;

        #[method(setAttributedStringForZero:)]
        pub unsafe fn setAttributedStringForZero(
            &self,
            attributedStringForZero: &NSAttributedString,
        );

        #[method_id(@__retain_semantics Other attributedStringForNil)]
        pub unsafe fn attributedStringForNil(&self) -> Id<NSAttributedString, Shared>;

        #[method(setAttributedStringForNil:)]
        pub unsafe fn setAttributedStringForNil(&self, attributedStringForNil: &NSAttributedString);

        #[method_id(@__retain_semantics Other attributedStringForNotANumber)]
        pub unsafe fn attributedStringForNotANumber(&self) -> Id<NSAttributedString, Shared>;

        #[method(setAttributedStringForNotANumber:)]
        pub unsafe fn setAttributedStringForNotANumber(
            &self,
            attributedStringForNotANumber: &NSAttributedString,
        );

        #[method_id(@__retain_semantics Other roundingBehavior)]
        pub unsafe fn roundingBehavior(&self) -> Id<NSDecimalNumberHandler, Shared>;

        #[method(setRoundingBehavior:)]
        pub unsafe fn setRoundingBehavior(&self, roundingBehavior: &NSDecimalNumberHandler);
    }
);
