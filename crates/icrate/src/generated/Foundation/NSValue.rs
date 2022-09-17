extern_class!(
    #[derive(Debug)]
    struct NSValue;
    unsafe impl ClassType for NSValue {
        type Super = NSObject;
    }
);
impl NSValue {
    pub unsafe fn getValue_size(&self, value: NonNull<c_void>, size: NSUInteger) {
        msg_send![self, getValue: value, size: size]
    }
    pub unsafe fn initWithBytes_objCType(
        &self,
        value: NonNull<c_void>,
        type_: NonNull<c_char>,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithBytes: value, objCType: type_]
    }
    pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCoder: coder]
    }
    pub unsafe fn objCType(&self) -> NonNull<c_char> {
        msg_send![self, objCType]
    }
}
#[doc = "NSValueCreation"]
impl NSValue {
    pub unsafe fn valueWithBytes_objCType(
        value: NonNull<c_void>,
        type_: NonNull<c_char>,
    ) -> Id<NSValue, Shared> {
        msg_send_id![Self::class(), valueWithBytes: value, objCType: type_]
    }
    pub unsafe fn value_withObjCType(
        value: NonNull<c_void>,
        type_: NonNull<c_char>,
    ) -> Id<NSValue, Shared> {
        msg_send_id![Self::class(), value: value, withObjCType: type_]
    }
}
#[doc = "NSValueExtensionMethods"]
impl NSValue {
    pub unsafe fn valueWithNonretainedObject(anObject: Option<&Object>) -> Id<NSValue, Shared> {
        msg_send_id![Self::class(), valueWithNonretainedObject: anObject]
    }
    pub unsafe fn valueWithPointer(pointer: *mut c_void) -> Id<NSValue, Shared> {
        msg_send_id![Self::class(), valueWithPointer: pointer]
    }
    pub unsafe fn isEqualToValue(&self, value: &NSValue) -> bool {
        msg_send![self, isEqualToValue: value]
    }
    pub unsafe fn nonretainedObjectValue(&self) -> Option<Id<Object, Shared>> {
        msg_send_id![self, nonretainedObjectValue]
    }
    pub unsafe fn pointerValue(&self) -> *mut c_void {
        msg_send![self, pointerValue]
    }
}
extern_class!(
    #[derive(Debug)]
    struct NSNumber;
    unsafe impl ClassType for NSNumber {
        type Super = NSValue;
    }
);
impl NSNumber {
    pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCoder: coder]
    }
    pub unsafe fn initWithChar(&self, value: c_char) -> Id<NSNumber, Shared> {
        msg_send_id![self, initWithChar: value]
    }
    pub unsafe fn initWithUnsignedChar(&self, value: c_uchar) -> Id<NSNumber, Shared> {
        msg_send_id![self, initWithUnsignedChar: value]
    }
    pub unsafe fn initWithShort(&self, value: c_short) -> Id<NSNumber, Shared> {
        msg_send_id![self, initWithShort: value]
    }
    pub unsafe fn initWithUnsignedShort(&self, value: c_ushort) -> Id<NSNumber, Shared> {
        msg_send_id![self, initWithUnsignedShort: value]
    }
    pub unsafe fn initWithInt(&self, value: c_int) -> Id<NSNumber, Shared> {
        msg_send_id![self, initWithInt: value]
    }
    pub unsafe fn initWithUnsignedInt(&self, value: c_uint) -> Id<NSNumber, Shared> {
        msg_send_id![self, initWithUnsignedInt: value]
    }
    pub unsafe fn initWithLong(&self, value: c_long) -> Id<NSNumber, Shared> {
        msg_send_id![self, initWithLong: value]
    }
    pub unsafe fn initWithUnsignedLong(&self, value: c_ulong) -> Id<NSNumber, Shared> {
        msg_send_id![self, initWithUnsignedLong: value]
    }
    pub unsafe fn initWithLongLong(&self, value: c_longlong) -> Id<NSNumber, Shared> {
        msg_send_id![self, initWithLongLong: value]
    }
    pub unsafe fn initWithUnsignedLongLong(&self, value: c_ulonglong) -> Id<NSNumber, Shared> {
        msg_send_id![self, initWithUnsignedLongLong: value]
    }
    pub unsafe fn initWithFloat(&self, value: c_float) -> Id<NSNumber, Shared> {
        msg_send_id![self, initWithFloat: value]
    }
    pub unsafe fn initWithDouble(&self, value: c_double) -> Id<NSNumber, Shared> {
        msg_send_id![self, initWithDouble: value]
    }
    pub unsafe fn initWithBool(&self, value: bool) -> Id<NSNumber, Shared> {
        msg_send_id![self, initWithBool: value]
    }
    pub unsafe fn initWithInteger(&self, value: NSInteger) -> Id<NSNumber, Shared> {
        msg_send_id![self, initWithInteger: value]
    }
    pub unsafe fn initWithUnsignedInteger(&self, value: NSUInteger) -> Id<NSNumber, Shared> {
        msg_send_id![self, initWithUnsignedInteger: value]
    }
    pub unsafe fn compare(&self, otherNumber: &NSNumber) -> NSComparisonResult {
        msg_send![self, compare: otherNumber]
    }
    pub unsafe fn isEqualToNumber(&self, number: &NSNumber) -> bool {
        msg_send![self, isEqualToNumber: number]
    }
    pub unsafe fn descriptionWithLocale(&self, locale: Option<&Object>) -> Id<NSString, Shared> {
        msg_send_id![self, descriptionWithLocale: locale]
    }
    pub unsafe fn charValue(&self) -> c_char {
        msg_send![self, charValue]
    }
    pub unsafe fn unsignedCharValue(&self) -> c_uchar {
        msg_send![self, unsignedCharValue]
    }
    pub unsafe fn shortValue(&self) -> c_short {
        msg_send![self, shortValue]
    }
    pub unsafe fn unsignedShortValue(&self) -> c_ushort {
        msg_send![self, unsignedShortValue]
    }
    pub unsafe fn intValue(&self) -> c_int {
        msg_send![self, intValue]
    }
    pub unsafe fn unsignedIntValue(&self) -> c_uint {
        msg_send![self, unsignedIntValue]
    }
    pub unsafe fn longValue(&self) -> c_long {
        msg_send![self, longValue]
    }
    pub unsafe fn unsignedLongValue(&self) -> c_ulong {
        msg_send![self, unsignedLongValue]
    }
    pub unsafe fn longLongValue(&self) -> c_longlong {
        msg_send![self, longLongValue]
    }
    pub unsafe fn unsignedLongLongValue(&self) -> c_ulonglong {
        msg_send![self, unsignedLongLongValue]
    }
    pub unsafe fn floatValue(&self) -> c_float {
        msg_send![self, floatValue]
    }
    pub unsafe fn doubleValue(&self) -> c_double {
        msg_send![self, doubleValue]
    }
    pub unsafe fn boolValue(&self) -> bool {
        msg_send![self, boolValue]
    }
    pub unsafe fn integerValue(&self) -> NSInteger {
        msg_send![self, integerValue]
    }
    pub unsafe fn unsignedIntegerValue(&self) -> NSUInteger {
        msg_send![self, unsignedIntegerValue]
    }
    pub unsafe fn stringValue(&self) -> Id<NSString, Shared> {
        msg_send_id![self, stringValue]
    }
}
#[doc = "NSNumberCreation"]
impl NSNumber {
    pub unsafe fn numberWithChar(value: c_char) -> Id<NSNumber, Shared> {
        msg_send_id![Self::class(), numberWithChar: value]
    }
    pub unsafe fn numberWithUnsignedChar(value: c_uchar) -> Id<NSNumber, Shared> {
        msg_send_id![Self::class(), numberWithUnsignedChar: value]
    }
    pub unsafe fn numberWithShort(value: c_short) -> Id<NSNumber, Shared> {
        msg_send_id![Self::class(), numberWithShort: value]
    }
    pub unsafe fn numberWithUnsignedShort(value: c_ushort) -> Id<NSNumber, Shared> {
        msg_send_id![Self::class(), numberWithUnsignedShort: value]
    }
    pub unsafe fn numberWithInt(value: c_int) -> Id<NSNumber, Shared> {
        msg_send_id![Self::class(), numberWithInt: value]
    }
    pub unsafe fn numberWithUnsignedInt(value: c_uint) -> Id<NSNumber, Shared> {
        msg_send_id![Self::class(), numberWithUnsignedInt: value]
    }
    pub unsafe fn numberWithLong(value: c_long) -> Id<NSNumber, Shared> {
        msg_send_id![Self::class(), numberWithLong: value]
    }
    pub unsafe fn numberWithUnsignedLong(value: c_ulong) -> Id<NSNumber, Shared> {
        msg_send_id![Self::class(), numberWithUnsignedLong: value]
    }
    pub unsafe fn numberWithLongLong(value: c_longlong) -> Id<NSNumber, Shared> {
        msg_send_id![Self::class(), numberWithLongLong: value]
    }
    pub unsafe fn numberWithUnsignedLongLong(value: c_ulonglong) -> Id<NSNumber, Shared> {
        msg_send_id![Self::class(), numberWithUnsignedLongLong: value]
    }
    pub unsafe fn numberWithFloat(value: c_float) -> Id<NSNumber, Shared> {
        msg_send_id![Self::class(), numberWithFloat: value]
    }
    pub unsafe fn numberWithDouble(value: c_double) -> Id<NSNumber, Shared> {
        msg_send_id![Self::class(), numberWithDouble: value]
    }
    pub unsafe fn numberWithBool(value: bool) -> Id<NSNumber, Shared> {
        msg_send_id![Self::class(), numberWithBool: value]
    }
    pub unsafe fn numberWithInteger(value: NSInteger) -> Id<NSNumber, Shared> {
        msg_send_id![Self::class(), numberWithInteger: value]
    }
    pub unsafe fn numberWithUnsignedInteger(value: NSUInteger) -> Id<NSNumber, Shared> {
        msg_send_id![Self::class(), numberWithUnsignedInteger: value]
    }
}
#[doc = "NSDeprecated"]
impl NSValue {
    pub unsafe fn getValue(&self, value: NonNull<c_void>) {
        msg_send![self, getValue: value]
    }
}
