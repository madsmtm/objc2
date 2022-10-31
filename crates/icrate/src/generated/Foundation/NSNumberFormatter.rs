//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};

pub type NSNumberFormatterBehavior = NSUInteger;
pub const NSNumberFormatterBehaviorDefault: NSNumberFormatterBehavior = 0;
pub const NSNumberFormatterBehavior10_0: NSNumberFormatterBehavior = 1000;
pub const NSNumberFormatterBehavior10_4: NSNumberFormatterBehavior = 1040;

pub type NSNumberFormatterStyle = NSUInteger;
pub const NSNumberFormatterNoStyle: NSNumberFormatterStyle = kCFNumberFormatterNoStyle;
pub const NSNumberFormatterDecimalStyle: NSNumberFormatterStyle = kCFNumberFormatterDecimalStyle;
pub const NSNumberFormatterCurrencyStyle: NSNumberFormatterStyle = kCFNumberFormatterCurrencyStyle;
pub const NSNumberFormatterPercentStyle: NSNumberFormatterStyle = kCFNumberFormatterPercentStyle;
pub const NSNumberFormatterScientificStyle: NSNumberFormatterStyle =
    kCFNumberFormatterScientificStyle;
pub const NSNumberFormatterSpellOutStyle: NSNumberFormatterStyle = kCFNumberFormatterSpellOutStyle;
pub const NSNumberFormatterOrdinalStyle: NSNumberFormatterStyle = kCFNumberFormatterOrdinalStyle;
pub const NSNumberFormatterCurrencyISOCodeStyle: NSNumberFormatterStyle =
    kCFNumberFormatterCurrencyISOCodeStyle;
pub const NSNumberFormatterCurrencyPluralStyle: NSNumberFormatterStyle =
    kCFNumberFormatterCurrencyPluralStyle;
pub const NSNumberFormatterCurrencyAccountingStyle: NSNumberFormatterStyle =
    kCFNumberFormatterCurrencyAccountingStyle;

pub type NSNumberFormatterPadPosition = NSUInteger;
pub const NSNumberFormatterPadBeforePrefix: NSNumberFormatterPadPosition =
    kCFNumberFormatterPadBeforePrefix;
pub const NSNumberFormatterPadAfterPrefix: NSNumberFormatterPadPosition =
    kCFNumberFormatterPadAfterPrefix;
pub const NSNumberFormatterPadBeforeSuffix: NSNumberFormatterPadPosition =
    kCFNumberFormatterPadBeforeSuffix;
pub const NSNumberFormatterPadAfterSuffix: NSNumberFormatterPadPosition =
    kCFNumberFormatterPadAfterSuffix;

pub type NSNumberFormatterRoundingMode = NSUInteger;
pub const NSNumberFormatterRoundCeiling: NSNumberFormatterRoundingMode =
    kCFNumberFormatterRoundCeiling;
pub const NSNumberFormatterRoundFloor: NSNumberFormatterRoundingMode = kCFNumberFormatterRoundFloor;
pub const NSNumberFormatterRoundDown: NSNumberFormatterRoundingMode = kCFNumberFormatterRoundDown;
pub const NSNumberFormatterRoundUp: NSNumberFormatterRoundingMode = kCFNumberFormatterRoundUp;
pub const NSNumberFormatterRoundHalfEven: NSNumberFormatterRoundingMode =
    kCFNumberFormatterRoundHalfEven;
pub const NSNumberFormatterRoundHalfDown: NSNumberFormatterRoundingMode =
    kCFNumberFormatterRoundHalfDown;
pub const NSNumberFormatterRoundHalfUp: NSNumberFormatterRoundingMode =
    kCFNumberFormatterRoundHalfUp;

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
            obj: Option<&mut Option<Id<Object, Shared>>>,
            string: &NSString,
            rangep: *mut NSRange,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method_id(stringFromNumber:)]
        pub unsafe fn stringFromNumber(&self, number: &NSNumber) -> Option<Id<NSString, Shared>>;

        #[method_id(numberFromString:)]
        pub unsafe fn numberFromString(&self, string: &NSString) -> Option<Id<NSNumber, Shared>>;

        #[method_id(localizedStringFromNumber:numberStyle:)]
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

        #[method_id(locale)]
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

        #[method_id(negativeFormat)]
        pub unsafe fn negativeFormat(&self) -> Id<NSString, Shared>;

        #[method(setNegativeFormat:)]
        pub unsafe fn setNegativeFormat(&self, negativeFormat: Option<&NSString>);

        #[method_id(textAttributesForNegativeValues)]
        pub unsafe fn textAttributesForNegativeValues(
            &self,
        ) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;

        #[method(setTextAttributesForNegativeValues:)]
        pub unsafe fn setTextAttributesForNegativeValues(
            &self,
            textAttributesForNegativeValues: Option<&NSDictionary<NSString, Object>>,
        );

        #[method_id(positiveFormat)]
        pub unsafe fn positiveFormat(&self) -> Id<NSString, Shared>;

        #[method(setPositiveFormat:)]
        pub unsafe fn setPositiveFormat(&self, positiveFormat: Option<&NSString>);

        #[method_id(textAttributesForPositiveValues)]
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

        #[method_id(decimalSeparator)]
        pub unsafe fn decimalSeparator(&self) -> Id<NSString, Shared>;

        #[method(setDecimalSeparator:)]
        pub unsafe fn setDecimalSeparator(&self, decimalSeparator: Option<&NSString>);

        #[method(alwaysShowsDecimalSeparator)]
        pub unsafe fn alwaysShowsDecimalSeparator(&self) -> bool;

        #[method(setAlwaysShowsDecimalSeparator:)]
        pub unsafe fn setAlwaysShowsDecimalSeparator(&self, alwaysShowsDecimalSeparator: bool);

        #[method_id(currencyDecimalSeparator)]
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

        #[method_id(groupingSeparator)]
        pub unsafe fn groupingSeparator(&self) -> Id<NSString, Shared>;

        #[method(setGroupingSeparator:)]
        pub unsafe fn setGroupingSeparator(&self, groupingSeparator: Option<&NSString>);

        #[method_id(zeroSymbol)]
        pub unsafe fn zeroSymbol(&self) -> Option<Id<NSString, Shared>>;

        #[method(setZeroSymbol:)]
        pub unsafe fn setZeroSymbol(&self, zeroSymbol: Option<&NSString>);

        #[method_id(textAttributesForZero)]
        pub unsafe fn textAttributesForZero(
            &self,
        ) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;

        #[method(setTextAttributesForZero:)]
        pub unsafe fn setTextAttributesForZero(
            &self,
            textAttributesForZero: Option<&NSDictionary<NSString, Object>>,
        );

        #[method_id(nilSymbol)]
        pub unsafe fn nilSymbol(&self) -> Id<NSString, Shared>;

        #[method(setNilSymbol:)]
        pub unsafe fn setNilSymbol(&self, nilSymbol: &NSString);

        #[method_id(textAttributesForNil)]
        pub unsafe fn textAttributesForNil(
            &self,
        ) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;

        #[method(setTextAttributesForNil:)]
        pub unsafe fn setTextAttributesForNil(
            &self,
            textAttributesForNil: Option<&NSDictionary<NSString, Object>>,
        );

        #[method_id(notANumberSymbol)]
        pub unsafe fn notANumberSymbol(&self) -> Id<NSString, Shared>;

        #[method(setNotANumberSymbol:)]
        pub unsafe fn setNotANumberSymbol(&self, notANumberSymbol: Option<&NSString>);

        #[method_id(textAttributesForNotANumber)]
        pub unsafe fn textAttributesForNotANumber(
            &self,
        ) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;

        #[method(setTextAttributesForNotANumber:)]
        pub unsafe fn setTextAttributesForNotANumber(
            &self,
            textAttributesForNotANumber: Option<&NSDictionary<NSString, Object>>,
        );

        #[method_id(positiveInfinitySymbol)]
        pub unsafe fn positiveInfinitySymbol(&self) -> Id<NSString, Shared>;

        #[method(setPositiveInfinitySymbol:)]
        pub unsafe fn setPositiveInfinitySymbol(&self, positiveInfinitySymbol: &NSString);

        #[method_id(textAttributesForPositiveInfinity)]
        pub unsafe fn textAttributesForPositiveInfinity(
            &self,
        ) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;

        #[method(setTextAttributesForPositiveInfinity:)]
        pub unsafe fn setTextAttributesForPositiveInfinity(
            &self,
            textAttributesForPositiveInfinity: Option<&NSDictionary<NSString, Object>>,
        );

        #[method_id(negativeInfinitySymbol)]
        pub unsafe fn negativeInfinitySymbol(&self) -> Id<NSString, Shared>;

        #[method(setNegativeInfinitySymbol:)]
        pub unsafe fn setNegativeInfinitySymbol(&self, negativeInfinitySymbol: &NSString);

        #[method_id(textAttributesForNegativeInfinity)]
        pub unsafe fn textAttributesForNegativeInfinity(
            &self,
        ) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;

        #[method(setTextAttributesForNegativeInfinity:)]
        pub unsafe fn setTextAttributesForNegativeInfinity(
            &self,
            textAttributesForNegativeInfinity: Option<&NSDictionary<NSString, Object>>,
        );

        #[method_id(positivePrefix)]
        pub unsafe fn positivePrefix(&self) -> Id<NSString, Shared>;

        #[method(setPositivePrefix:)]
        pub unsafe fn setPositivePrefix(&self, positivePrefix: Option<&NSString>);

        #[method_id(positiveSuffix)]
        pub unsafe fn positiveSuffix(&self) -> Id<NSString, Shared>;

        #[method(setPositiveSuffix:)]
        pub unsafe fn setPositiveSuffix(&self, positiveSuffix: Option<&NSString>);

        #[method_id(negativePrefix)]
        pub unsafe fn negativePrefix(&self) -> Id<NSString, Shared>;

        #[method(setNegativePrefix:)]
        pub unsafe fn setNegativePrefix(&self, negativePrefix: Option<&NSString>);

        #[method_id(negativeSuffix)]
        pub unsafe fn negativeSuffix(&self) -> Id<NSString, Shared>;

        #[method(setNegativeSuffix:)]
        pub unsafe fn setNegativeSuffix(&self, negativeSuffix: Option<&NSString>);

        #[method_id(currencyCode)]
        pub unsafe fn currencyCode(&self) -> Id<NSString, Shared>;

        #[method(setCurrencyCode:)]
        pub unsafe fn setCurrencyCode(&self, currencyCode: Option<&NSString>);

        #[method_id(currencySymbol)]
        pub unsafe fn currencySymbol(&self) -> Id<NSString, Shared>;

        #[method(setCurrencySymbol:)]
        pub unsafe fn setCurrencySymbol(&self, currencySymbol: Option<&NSString>);

        #[method_id(internationalCurrencySymbol)]
        pub unsafe fn internationalCurrencySymbol(&self) -> Id<NSString, Shared>;

        #[method(setInternationalCurrencySymbol:)]
        pub unsafe fn setInternationalCurrencySymbol(
            &self,
            internationalCurrencySymbol: Option<&NSString>,
        );

        #[method_id(percentSymbol)]
        pub unsafe fn percentSymbol(&self) -> Id<NSString, Shared>;

        #[method(setPercentSymbol:)]
        pub unsafe fn setPercentSymbol(&self, percentSymbol: Option<&NSString>);

        #[method_id(perMillSymbol)]
        pub unsafe fn perMillSymbol(&self) -> Id<NSString, Shared>;

        #[method(setPerMillSymbol:)]
        pub unsafe fn setPerMillSymbol(&self, perMillSymbol: Option<&NSString>);

        #[method_id(minusSign)]
        pub unsafe fn minusSign(&self) -> Id<NSString, Shared>;

        #[method(setMinusSign:)]
        pub unsafe fn setMinusSign(&self, minusSign: Option<&NSString>);

        #[method_id(plusSign)]
        pub unsafe fn plusSign(&self) -> Id<NSString, Shared>;

        #[method(setPlusSign:)]
        pub unsafe fn setPlusSign(&self, plusSign: Option<&NSString>);

        #[method_id(exponentSymbol)]
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

        #[method_id(multiplier)]
        pub unsafe fn multiplier(&self) -> Option<Id<NSNumber, Shared>>;

        #[method(setMultiplier:)]
        pub unsafe fn setMultiplier(&self, multiplier: Option<&NSNumber>);

        #[method(formatWidth)]
        pub unsafe fn formatWidth(&self) -> NSUInteger;

        #[method(setFormatWidth:)]
        pub unsafe fn setFormatWidth(&self, formatWidth: NSUInteger);

        #[method_id(paddingCharacter)]
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

        #[method_id(roundingIncrement)]
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

        #[method_id(minimum)]
        pub unsafe fn minimum(&self) -> Option<Id<NSNumber, Shared>>;

        #[method(setMinimum:)]
        pub unsafe fn setMinimum(&self, minimum: Option<&NSNumber>);

        #[method_id(maximum)]
        pub unsafe fn maximum(&self) -> Option<Id<NSNumber, Shared>>;

        #[method(setMaximum:)]
        pub unsafe fn setMaximum(&self, maximum: Option<&NSNumber>);

        #[method_id(currencyGroupingSeparator)]
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

        #[method_id(thousandSeparator)]
        pub unsafe fn thousandSeparator(&self) -> Id<NSString, Shared>;

        #[method(setThousandSeparator:)]
        pub unsafe fn setThousandSeparator(&self, thousandSeparator: Option<&NSString>);

        #[method(localizesFormat)]
        pub unsafe fn localizesFormat(&self) -> bool;

        #[method(setLocalizesFormat:)]
        pub unsafe fn setLocalizesFormat(&self, localizesFormat: bool);

        #[method_id(format)]
        pub unsafe fn format(&self) -> Id<NSString, Shared>;

        #[method(setFormat:)]
        pub unsafe fn setFormat(&self, format: &NSString);

        #[method_id(attributedStringForZero)]
        pub unsafe fn attributedStringForZero(&self) -> Id<NSAttributedString, Shared>;

        #[method(setAttributedStringForZero:)]
        pub unsafe fn setAttributedStringForZero(
            &self,
            attributedStringForZero: &NSAttributedString,
        );

        #[method_id(attributedStringForNil)]
        pub unsafe fn attributedStringForNil(&self) -> Id<NSAttributedString, Shared>;

        #[method(setAttributedStringForNil:)]
        pub unsafe fn setAttributedStringForNil(&self, attributedStringForNil: &NSAttributedString);

        #[method_id(attributedStringForNotANumber)]
        pub unsafe fn attributedStringForNotANumber(&self) -> Id<NSAttributedString, Shared>;

        #[method(setAttributedStringForNotANumber:)]
        pub unsafe fn setAttributedStringForNotANumber(
            &self,
            attributedStringForNotANumber: &NSAttributedString,
        );

        #[method_id(roundingBehavior)]
        pub unsafe fn roundingBehavior(&self) -> Id<NSDecimalNumberHandler, Shared>;

        #[method(setRoundingBehavior:)]
        pub unsafe fn setRoundingBehavior(&self, roundingBehavior: &NSDecimalNumberHandler);
    }
);
