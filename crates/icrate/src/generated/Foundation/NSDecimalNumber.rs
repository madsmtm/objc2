#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSDecimalNumberBehaviors = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSDecimalNumber;
    unsafe impl ClassType for NSDecimalNumber {
        type Super = NSNumber;
    }
);
extern_methods!(
    unsafe impl NSDecimalNumber {
        #[method_id(initWithMantissa:exponent:isNegative:)]
        pub unsafe fn initWithMantissa_exponent_isNegative(
            &self,
            mantissa: c_ulonglong,
            exponent: c_short,
            flag: bool,
        ) -> Id<Self, Shared>;
        #[method_id(initWithDecimal:)]
        pub unsafe fn initWithDecimal(&self, dcm: NSDecimal) -> Id<Self, Shared>;
        #[method_id(initWithString:)]
        pub unsafe fn initWithString(&self, numberValue: Option<&NSString>) -> Id<Self, Shared>;
        #[method_id(initWithString:locale:)]
        pub unsafe fn initWithString_locale(
            &self,
            numberValue: Option<&NSString>,
            locale: Option<&Object>,
        ) -> Id<Self, Shared>;
        #[method_id(descriptionWithLocale:)]
        pub unsafe fn descriptionWithLocale(&self, locale: Option<&Object>)
            -> Id<NSString, Shared>;
        #[method(decimalValue)]
        pub unsafe fn decimalValue(&self) -> NSDecimal;
        #[method_id(decimalNumberWithMantissa:exponent:isNegative:)]
        pub unsafe fn decimalNumberWithMantissa_exponent_isNegative(
            mantissa: c_ulonglong,
            exponent: c_short,
            flag: bool,
        ) -> Id<NSDecimalNumber, Shared>;
        #[method_id(decimalNumberWithDecimal:)]
        pub unsafe fn decimalNumberWithDecimal(dcm: NSDecimal) -> Id<NSDecimalNumber, Shared>;
        #[method_id(decimalNumberWithString:)]
        pub unsafe fn decimalNumberWithString(
            numberValue: Option<&NSString>,
        ) -> Id<NSDecimalNumber, Shared>;
        #[method_id(decimalNumberWithString:locale:)]
        pub unsafe fn decimalNumberWithString_locale(
            numberValue: Option<&NSString>,
            locale: Option<&Object>,
        ) -> Id<NSDecimalNumber, Shared>;
        #[method_id(zero)]
        pub unsafe fn zero() -> Id<NSDecimalNumber, Shared>;
        #[method_id(one)]
        pub unsafe fn one() -> Id<NSDecimalNumber, Shared>;
        #[method_id(minimumDecimalNumber)]
        pub unsafe fn minimumDecimalNumber() -> Id<NSDecimalNumber, Shared>;
        #[method_id(maximumDecimalNumber)]
        pub unsafe fn maximumDecimalNumber() -> Id<NSDecimalNumber, Shared>;
        #[method_id(notANumber)]
        pub unsafe fn notANumber() -> Id<NSDecimalNumber, Shared>;
        #[method_id(decimalNumberByAdding:)]
        pub unsafe fn decimalNumberByAdding(
            &self,
            decimalNumber: &NSDecimalNumber,
        ) -> Id<NSDecimalNumber, Shared>;
        #[method_id(decimalNumberByAdding:withBehavior:)]
        pub unsafe fn decimalNumberByAdding_withBehavior(
            &self,
            decimalNumber: &NSDecimalNumber,
            behavior: Option<&NSDecimalNumberBehaviors>,
        ) -> Id<NSDecimalNumber, Shared>;
        #[method_id(decimalNumberBySubtracting:)]
        pub unsafe fn decimalNumberBySubtracting(
            &self,
            decimalNumber: &NSDecimalNumber,
        ) -> Id<NSDecimalNumber, Shared>;
        #[method_id(decimalNumberBySubtracting:withBehavior:)]
        pub unsafe fn decimalNumberBySubtracting_withBehavior(
            &self,
            decimalNumber: &NSDecimalNumber,
            behavior: Option<&NSDecimalNumberBehaviors>,
        ) -> Id<NSDecimalNumber, Shared>;
        #[method_id(decimalNumberByMultiplyingBy:)]
        pub unsafe fn decimalNumberByMultiplyingBy(
            &self,
            decimalNumber: &NSDecimalNumber,
        ) -> Id<NSDecimalNumber, Shared>;
        #[method_id(decimalNumberByMultiplyingBy:withBehavior:)]
        pub unsafe fn decimalNumberByMultiplyingBy_withBehavior(
            &self,
            decimalNumber: &NSDecimalNumber,
            behavior: Option<&NSDecimalNumberBehaviors>,
        ) -> Id<NSDecimalNumber, Shared>;
        #[method_id(decimalNumberByDividingBy:)]
        pub unsafe fn decimalNumberByDividingBy(
            &self,
            decimalNumber: &NSDecimalNumber,
        ) -> Id<NSDecimalNumber, Shared>;
        #[method_id(decimalNumberByDividingBy:withBehavior:)]
        pub unsafe fn decimalNumberByDividingBy_withBehavior(
            &self,
            decimalNumber: &NSDecimalNumber,
            behavior: Option<&NSDecimalNumberBehaviors>,
        ) -> Id<NSDecimalNumber, Shared>;
        #[method_id(decimalNumberByRaisingToPower:)]
        pub unsafe fn decimalNumberByRaisingToPower(
            &self,
            power: NSUInteger,
        ) -> Id<NSDecimalNumber, Shared>;
        #[method_id(decimalNumberByRaisingToPower:withBehavior:)]
        pub unsafe fn decimalNumberByRaisingToPower_withBehavior(
            &self,
            power: NSUInteger,
            behavior: Option<&NSDecimalNumberBehaviors>,
        ) -> Id<NSDecimalNumber, Shared>;
        #[method_id(decimalNumberByMultiplyingByPowerOf10:)]
        pub unsafe fn decimalNumberByMultiplyingByPowerOf10(
            &self,
            power: c_short,
        ) -> Id<NSDecimalNumber, Shared>;
        #[method_id(decimalNumberByMultiplyingByPowerOf10:withBehavior:)]
        pub unsafe fn decimalNumberByMultiplyingByPowerOf10_withBehavior(
            &self,
            power: c_short,
            behavior: Option<&NSDecimalNumberBehaviors>,
        ) -> Id<NSDecimalNumber, Shared>;
        #[method_id(decimalNumberByRoundingAccordingToBehavior:)]
        pub unsafe fn decimalNumberByRoundingAccordingToBehavior(
            &self,
            behavior: Option<&NSDecimalNumberBehaviors>,
        ) -> Id<NSDecimalNumber, Shared>;
        #[method(compare:)]
        pub unsafe fn compare(&self, decimalNumber: &NSNumber) -> NSComparisonResult;
        #[method_id(defaultBehavior)]
        pub unsafe fn defaultBehavior() -> Id<NSDecimalNumberBehaviors, Shared>;
        #[method(setDefaultBehavior:)]
        pub unsafe fn setDefaultBehavior(defaultBehavior: &NSDecimalNumberBehaviors);
        #[method(objCType)]
        pub unsafe fn objCType(&self) -> NonNull<c_char>;
        #[method(doubleValue)]
        pub unsafe fn doubleValue(&self) -> c_double;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSDecimalNumberHandler;
    unsafe impl ClassType for NSDecimalNumberHandler {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSDecimalNumberHandler {
        #[method_id(defaultDecimalNumberHandler)]
        pub unsafe fn defaultDecimalNumberHandler() -> Id<NSDecimalNumberHandler, Shared>;
        #[method_id(initWithRoundingMode:scale:raiseOnExactness:raiseOnOverflow:raiseOnUnderflow:raiseOnDivideByZero:)]
        pub unsafe fn initWithRoundingMode_scale_raiseOnExactness_raiseOnOverflow_raiseOnUnderflow_raiseOnDivideByZero(
            &self,
            roundingMode: NSRoundingMode,
            scale: c_short,
            exact: bool,
            overflow: bool,
            underflow: bool,
            divideByZero: bool,
        ) -> Id<Self, Shared>;
        #[method_id(decimalNumberHandlerWithRoundingMode:scale:raiseOnExactness:raiseOnOverflow:raiseOnUnderflow:raiseOnDivideByZero:)]
        pub unsafe fn decimalNumberHandlerWithRoundingMode_scale_raiseOnExactness_raiseOnOverflow_raiseOnUnderflow_raiseOnDivideByZero(
            roundingMode: NSRoundingMode,
            scale: c_short,
            exact: bool,
            overflow: bool,
            underflow: bool,
            divideByZero: bool,
        ) -> Id<Self, Shared>;
    }
);
extern_methods!(
    #[doc = "NSDecimalNumberExtensions"]
    unsafe impl NSNumber {
        #[method(decimalValue)]
        pub unsafe fn decimalValue(&self) -> NSDecimal;
    }
);
extern_methods!(
    #[doc = "NSDecimalNumberScanning"]
    unsafe impl NSScanner {
        #[method(scanDecimal:)]
        pub unsafe fn scanDecimal(&self, dcm: *mut NSDecimal) -> bool;
    }
);
