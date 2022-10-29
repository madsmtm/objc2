#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSValue;
    unsafe impl ClassType for NSValue {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSValue {
        #[method(getValue:size:)]
        pub unsafe fn getValue_size(&self, value: NonNull<c_void>, size: NSUInteger);
        #[method(objCType)]
        pub unsafe fn objCType(&self) -> NonNull<c_char>;
        #[method_id(initWithBytes:objCType:)]
        pub unsafe fn initWithBytes_objCType(
            &self,
            value: NonNull<c_void>,
            type_: NonNull<c_char>,
        ) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSValueCreation"]
    unsafe impl NSValue {
        #[method_id(valueWithBytes:objCType:)]
        pub unsafe fn valueWithBytes_objCType(
            value: NonNull<c_void>,
            type_: NonNull<c_char>,
        ) -> Id<NSValue, Shared>;
        #[method_id(value:withObjCType:)]
        pub unsafe fn value_withObjCType(
            value: NonNull<c_void>,
            type_: NonNull<c_char>,
        ) -> Id<NSValue, Shared>;
    }
);
extern_methods!(
    #[doc = "NSValueExtensionMethods"]
    unsafe impl NSValue {
        #[method_id(valueWithNonretainedObject:)]
        pub unsafe fn valueWithNonretainedObject(anObject: Option<&Object>) -> Id<NSValue, Shared>;
        #[method_id(nonretainedObjectValue)]
        pub unsafe fn nonretainedObjectValue(&self) -> Option<Id<Object, Shared>>;
        #[method_id(valueWithPointer:)]
        pub unsafe fn valueWithPointer(pointer: *mut c_void) -> Id<NSValue, Shared>;
        #[method(pointerValue)]
        pub unsafe fn pointerValue(&self) -> *mut c_void;
        #[method(isEqualToValue:)]
        pub unsafe fn isEqualToValue(&self, value: &NSValue) -> bool;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSNumber;
    unsafe impl ClassType for NSNumber {
        type Super = NSValue;
    }
);
extern_methods!(
    unsafe impl NSNumber {
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(initWithChar:)]
        pub unsafe fn initWithChar(&self, value: c_char) -> Id<NSNumber, Shared>;
        #[method_id(initWithUnsignedChar:)]
        pub unsafe fn initWithUnsignedChar(&self, value: c_uchar) -> Id<NSNumber, Shared>;
        #[method_id(initWithShort:)]
        pub unsafe fn initWithShort(&self, value: c_short) -> Id<NSNumber, Shared>;
        #[method_id(initWithUnsignedShort:)]
        pub unsafe fn initWithUnsignedShort(&self, value: c_ushort) -> Id<NSNumber, Shared>;
        #[method_id(initWithInt:)]
        pub unsafe fn initWithInt(&self, value: c_int) -> Id<NSNumber, Shared>;
        #[method_id(initWithUnsignedInt:)]
        pub unsafe fn initWithUnsignedInt(&self, value: c_uint) -> Id<NSNumber, Shared>;
        #[method_id(initWithLong:)]
        pub unsafe fn initWithLong(&self, value: c_long) -> Id<NSNumber, Shared>;
        #[method_id(initWithUnsignedLong:)]
        pub unsafe fn initWithUnsignedLong(&self, value: c_ulong) -> Id<NSNumber, Shared>;
        #[method_id(initWithLongLong:)]
        pub unsafe fn initWithLongLong(&self, value: c_longlong) -> Id<NSNumber, Shared>;
        #[method_id(initWithUnsignedLongLong:)]
        pub unsafe fn initWithUnsignedLongLong(&self, value: c_ulonglong) -> Id<NSNumber, Shared>;
        #[method_id(initWithFloat:)]
        pub unsafe fn initWithFloat(&self, value: c_float) -> Id<NSNumber, Shared>;
        #[method_id(initWithDouble:)]
        pub unsafe fn initWithDouble(&self, value: c_double) -> Id<NSNumber, Shared>;
        #[method_id(initWithBool:)]
        pub unsafe fn initWithBool(&self, value: bool) -> Id<NSNumber, Shared>;
        #[method_id(initWithInteger:)]
        pub unsafe fn initWithInteger(&self, value: NSInteger) -> Id<NSNumber, Shared>;
        #[method_id(initWithUnsignedInteger:)]
        pub unsafe fn initWithUnsignedInteger(&self, value: NSUInteger) -> Id<NSNumber, Shared>;
        #[method(charValue)]
        pub unsafe fn charValue(&self) -> c_char;
        #[method(unsignedCharValue)]
        pub unsafe fn unsignedCharValue(&self) -> c_uchar;
        #[method(shortValue)]
        pub unsafe fn shortValue(&self) -> c_short;
        #[method(unsignedShortValue)]
        pub unsafe fn unsignedShortValue(&self) -> c_ushort;
        #[method(intValue)]
        pub unsafe fn intValue(&self) -> c_int;
        #[method(unsignedIntValue)]
        pub unsafe fn unsignedIntValue(&self) -> c_uint;
        #[method(longValue)]
        pub unsafe fn longValue(&self) -> c_long;
        #[method(unsignedLongValue)]
        pub unsafe fn unsignedLongValue(&self) -> c_ulong;
        #[method(longLongValue)]
        pub unsafe fn longLongValue(&self) -> c_longlong;
        #[method(unsignedLongLongValue)]
        pub unsafe fn unsignedLongLongValue(&self) -> c_ulonglong;
        #[method(floatValue)]
        pub unsafe fn floatValue(&self) -> c_float;
        #[method(doubleValue)]
        pub unsafe fn doubleValue(&self) -> c_double;
        #[method(boolValue)]
        pub unsafe fn boolValue(&self) -> bool;
        #[method(integerValue)]
        pub unsafe fn integerValue(&self) -> NSInteger;
        #[method(unsignedIntegerValue)]
        pub unsafe fn unsignedIntegerValue(&self) -> NSUInteger;
        #[method_id(stringValue)]
        pub unsafe fn stringValue(&self) -> Id<NSString, Shared>;
        #[method(compare:)]
        pub unsafe fn compare(&self, otherNumber: &NSNumber) -> NSComparisonResult;
        #[method(isEqualToNumber:)]
        pub unsafe fn isEqualToNumber(&self, number: &NSNumber) -> bool;
        #[method_id(descriptionWithLocale:)]
        pub unsafe fn descriptionWithLocale(&self, locale: Option<&Object>)
            -> Id<NSString, Shared>;
    }
);
extern_methods!(
    #[doc = "NSNumberCreation"]
    unsafe impl NSNumber {
        #[method_id(numberWithChar:)]
        pub unsafe fn numberWithChar(value: c_char) -> Id<NSNumber, Shared>;
        #[method_id(numberWithUnsignedChar:)]
        pub unsafe fn numberWithUnsignedChar(value: c_uchar) -> Id<NSNumber, Shared>;
        #[method_id(numberWithShort:)]
        pub unsafe fn numberWithShort(value: c_short) -> Id<NSNumber, Shared>;
        #[method_id(numberWithUnsignedShort:)]
        pub unsafe fn numberWithUnsignedShort(value: c_ushort) -> Id<NSNumber, Shared>;
        #[method_id(numberWithInt:)]
        pub unsafe fn numberWithInt(value: c_int) -> Id<NSNumber, Shared>;
        #[method_id(numberWithUnsignedInt:)]
        pub unsafe fn numberWithUnsignedInt(value: c_uint) -> Id<NSNumber, Shared>;
        #[method_id(numberWithLong:)]
        pub unsafe fn numberWithLong(value: c_long) -> Id<NSNumber, Shared>;
        #[method_id(numberWithUnsignedLong:)]
        pub unsafe fn numberWithUnsignedLong(value: c_ulong) -> Id<NSNumber, Shared>;
        #[method_id(numberWithLongLong:)]
        pub unsafe fn numberWithLongLong(value: c_longlong) -> Id<NSNumber, Shared>;
        #[method_id(numberWithUnsignedLongLong:)]
        pub unsafe fn numberWithUnsignedLongLong(value: c_ulonglong) -> Id<NSNumber, Shared>;
        #[method_id(numberWithFloat:)]
        pub unsafe fn numberWithFloat(value: c_float) -> Id<NSNumber, Shared>;
        #[method_id(numberWithDouble:)]
        pub unsafe fn numberWithDouble(value: c_double) -> Id<NSNumber, Shared>;
        #[method_id(numberWithBool:)]
        pub unsafe fn numberWithBool(value: bool) -> Id<NSNumber, Shared>;
        #[method_id(numberWithInteger:)]
        pub unsafe fn numberWithInteger(value: NSInteger) -> Id<NSNumber, Shared>;
        #[method_id(numberWithUnsignedInteger:)]
        pub unsafe fn numberWithUnsignedInteger(value: NSUInteger) -> Id<NSNumber, Shared>;
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSValue {
        #[method(getValue:)]
        pub unsafe fn getValue(&self, value: NonNull<c_void>);
    }
);
