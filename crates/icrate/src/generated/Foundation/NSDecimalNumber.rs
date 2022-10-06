use crate::Foundation::generated::NSDecimal::*;
use crate::Foundation::generated::NSDictionary::*;
use crate::Foundation::generated::NSException::*;
use crate::Foundation::generated::NSScanner::*;
use crate::Foundation::generated::NSValue::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
pub type NSDecimalNumberBehaviors = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSDecimalNumber;
    unsafe impl ClassType for NSDecimalNumber {
        type Super = NSNumber;
    }
);
impl NSDecimalNumber {
    pub unsafe fn initWithMantissa_exponent_isNegative(
        &self,
        mantissa: c_ulonglong,
        exponent: c_short,
        flag: bool,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithMantissa: mantissa,
            exponent: exponent,
            isNegative: flag
        ]
    }
    pub unsafe fn initWithDecimal(&self, dcm: NSDecimal) -> Id<Self, Shared> {
        msg_send_id![self, initWithDecimal: dcm]
    }
    pub unsafe fn initWithString(&self, numberValue: Option<&NSString>) -> Id<Self, Shared> {
        msg_send_id![self, initWithString: numberValue]
    }
    pub unsafe fn initWithString_locale(
        &self,
        numberValue: Option<&NSString>,
        locale: Option<&Object>,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithString: numberValue, locale: locale]
    }
    pub unsafe fn descriptionWithLocale(&self, locale: Option<&Object>) -> Id<NSString, Shared> {
        msg_send_id![self, descriptionWithLocale: locale]
    }
    pub unsafe fn decimalNumberWithMantissa_exponent_isNegative(
        mantissa: c_ulonglong,
        exponent: c_short,
        flag: bool,
    ) -> Id<NSDecimalNumber, Shared> {
        msg_send_id![
            Self::class(),
            decimalNumberWithMantissa: mantissa,
            exponent: exponent,
            isNegative: flag
        ]
    }
    pub unsafe fn decimalNumberWithDecimal(dcm: NSDecimal) -> Id<NSDecimalNumber, Shared> {
        msg_send_id![Self::class(), decimalNumberWithDecimal: dcm]
    }
    pub unsafe fn decimalNumberWithString(
        numberValue: Option<&NSString>,
    ) -> Id<NSDecimalNumber, Shared> {
        msg_send_id![Self::class(), decimalNumberWithString: numberValue]
    }
    pub unsafe fn decimalNumberWithString_locale(
        numberValue: Option<&NSString>,
        locale: Option<&Object>,
    ) -> Id<NSDecimalNumber, Shared> {
        msg_send_id![
            Self::class(),
            decimalNumberWithString: numberValue,
            locale: locale
        ]
    }
    pub unsafe fn decimalNumberByAdding(
        &self,
        decimalNumber: &NSDecimalNumber,
    ) -> Id<NSDecimalNumber, Shared> {
        msg_send_id![self, decimalNumberByAdding: decimalNumber]
    }
    pub unsafe fn decimalNumberByAdding_withBehavior(
        &self,
        decimalNumber: &NSDecimalNumber,
        behavior: Option<&NSDecimalNumberBehaviors>,
    ) -> Id<NSDecimalNumber, Shared> {
        msg_send_id![
            self,
            decimalNumberByAdding: decimalNumber,
            withBehavior: behavior
        ]
    }
    pub unsafe fn decimalNumberBySubtracting(
        &self,
        decimalNumber: &NSDecimalNumber,
    ) -> Id<NSDecimalNumber, Shared> {
        msg_send_id![self, decimalNumberBySubtracting: decimalNumber]
    }
    pub unsafe fn decimalNumberBySubtracting_withBehavior(
        &self,
        decimalNumber: &NSDecimalNumber,
        behavior: Option<&NSDecimalNumberBehaviors>,
    ) -> Id<NSDecimalNumber, Shared> {
        msg_send_id![
            self,
            decimalNumberBySubtracting: decimalNumber,
            withBehavior: behavior
        ]
    }
    pub unsafe fn decimalNumberByMultiplyingBy(
        &self,
        decimalNumber: &NSDecimalNumber,
    ) -> Id<NSDecimalNumber, Shared> {
        msg_send_id![self, decimalNumberByMultiplyingBy: decimalNumber]
    }
    pub unsafe fn decimalNumberByMultiplyingBy_withBehavior(
        &self,
        decimalNumber: &NSDecimalNumber,
        behavior: Option<&NSDecimalNumberBehaviors>,
    ) -> Id<NSDecimalNumber, Shared> {
        msg_send_id![
            self,
            decimalNumberByMultiplyingBy: decimalNumber,
            withBehavior: behavior
        ]
    }
    pub unsafe fn decimalNumberByDividingBy(
        &self,
        decimalNumber: &NSDecimalNumber,
    ) -> Id<NSDecimalNumber, Shared> {
        msg_send_id![self, decimalNumberByDividingBy: decimalNumber]
    }
    pub unsafe fn decimalNumberByDividingBy_withBehavior(
        &self,
        decimalNumber: &NSDecimalNumber,
        behavior: Option<&NSDecimalNumberBehaviors>,
    ) -> Id<NSDecimalNumber, Shared> {
        msg_send_id![
            self,
            decimalNumberByDividingBy: decimalNumber,
            withBehavior: behavior
        ]
    }
    pub unsafe fn decimalNumberByRaisingToPower(
        &self,
        power: NSUInteger,
    ) -> Id<NSDecimalNumber, Shared> {
        msg_send_id![self, decimalNumberByRaisingToPower: power]
    }
    pub unsafe fn decimalNumberByRaisingToPower_withBehavior(
        &self,
        power: NSUInteger,
        behavior: Option<&NSDecimalNumberBehaviors>,
    ) -> Id<NSDecimalNumber, Shared> {
        msg_send_id![
            self,
            decimalNumberByRaisingToPower: power,
            withBehavior: behavior
        ]
    }
    pub unsafe fn decimalNumberByMultiplyingByPowerOf10(
        &self,
        power: c_short,
    ) -> Id<NSDecimalNumber, Shared> {
        msg_send_id![self, decimalNumberByMultiplyingByPowerOf10: power]
    }
    pub unsafe fn decimalNumberByMultiplyingByPowerOf10_withBehavior(
        &self,
        power: c_short,
        behavior: Option<&NSDecimalNumberBehaviors>,
    ) -> Id<NSDecimalNumber, Shared> {
        msg_send_id![
            self,
            decimalNumberByMultiplyingByPowerOf10: power,
            withBehavior: behavior
        ]
    }
    pub unsafe fn decimalNumberByRoundingAccordingToBehavior(
        &self,
        behavior: Option<&NSDecimalNumberBehaviors>,
    ) -> Id<NSDecimalNumber, Shared> {
        msg_send_id![self, decimalNumberByRoundingAccordingToBehavior: behavior]
    }
    pub unsafe fn compare(&self, decimalNumber: &NSNumber) -> NSComparisonResult {
        msg_send![self, compare: decimalNumber]
    }
    pub unsafe fn decimalValue(&self) -> NSDecimal {
        msg_send![self, decimalValue]
    }
    pub unsafe fn zero() -> Id<NSDecimalNumber, Shared> {
        msg_send_id![Self::class(), zero]
    }
    pub unsafe fn one() -> Id<NSDecimalNumber, Shared> {
        msg_send_id![Self::class(), one]
    }
    pub unsafe fn minimumDecimalNumber() -> Id<NSDecimalNumber, Shared> {
        msg_send_id![Self::class(), minimumDecimalNumber]
    }
    pub unsafe fn maximumDecimalNumber() -> Id<NSDecimalNumber, Shared> {
        msg_send_id![Self::class(), maximumDecimalNumber]
    }
    pub unsafe fn notANumber() -> Id<NSDecimalNumber, Shared> {
        msg_send_id![Self::class(), notANumber]
    }
    pub unsafe fn defaultBehavior() -> Id<NSDecimalNumberBehaviors, Shared> {
        msg_send_id![Self::class(), defaultBehavior]
    }
    pub unsafe fn setDefaultBehavior(defaultBehavior: &NSDecimalNumberBehaviors) {
        msg_send![Self::class(), setDefaultBehavior: defaultBehavior]
    }
    pub unsafe fn objCType(&self) -> NonNull<c_char> {
        msg_send![self, objCType]
    }
    pub unsafe fn doubleValue(&self) -> c_double {
        msg_send![self, doubleValue]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSDecimalNumberHandler;
    unsafe impl ClassType for NSDecimalNumberHandler {
        type Super = NSObject;
    }
);
impl NSDecimalNumberHandler {
    pub unsafe fn initWithRoundingMode_scale_raiseOnExactness_raiseOnOverflow_raiseOnUnderflow_raiseOnDivideByZero(
        &self,
        roundingMode: NSRoundingMode,
        scale: c_short,
        exact: bool,
        overflow: bool,
        underflow: bool,
        divideByZero: bool,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithRoundingMode: roundingMode,
            scale: scale,
            raiseOnExactness: exact,
            raiseOnOverflow: overflow,
            raiseOnUnderflow: underflow,
            raiseOnDivideByZero: divideByZero
        ]
    }
    pub unsafe fn decimalNumberHandlerWithRoundingMode_scale_raiseOnExactness_raiseOnOverflow_raiseOnUnderflow_raiseOnDivideByZero(
        roundingMode: NSRoundingMode,
        scale: c_short,
        exact: bool,
        overflow: bool,
        underflow: bool,
        divideByZero: bool,
    ) -> Id<Self, Shared> {
        msg_send_id![
            Self::class(),
            decimalNumberHandlerWithRoundingMode: roundingMode,
            scale: scale,
            raiseOnExactness: exact,
            raiseOnOverflow: overflow,
            raiseOnUnderflow: underflow,
            raiseOnDivideByZero: divideByZero
        ]
    }
    pub unsafe fn defaultDecimalNumberHandler() -> Id<NSDecimalNumberHandler, Shared> {
        msg_send_id![Self::class(), defaultDecimalNumberHandler]
    }
}
#[doc = "NSDecimalNumberExtensions"]
impl NSNumber {
    pub unsafe fn decimalValue(&self) -> NSDecimal {
        msg_send![self, decimalValue]
    }
}
#[doc = "NSDecimalNumberScanning"]
impl NSScanner {
    pub unsafe fn scanDecimal(&self, dcm: *mut NSDecimal) -> bool {
        msg_send![self, scanDecimal: dcm]
    }
}
