use super::NSData;
use super::NSSet;
use super::NSString;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSCoder;
    unsafe impl ClassType for NSCoder {
        type Super = NSObject;
    }
);
impl NSCoder {
    pub unsafe fn encodeValueOfObjCType_at(&self, type_: NonNull<c_char>, addr: NonNull<c_void>) {
        msg_send![self, encodeValueOfObjCType: type_, at: addr]
    }
    pub unsafe fn encodeDataObject(&self, data: &NSData) {
        msg_send![self, encodeDataObject: data]
    }
    pub unsafe fn decodeDataObject(&self) -> Option<Id<NSData, Shared>> {
        msg_send_id![self, decodeDataObject]
    }
    pub unsafe fn decodeValueOfObjCType_at_size(
        &self,
        type_: NonNull<c_char>,
        data: NonNull<c_void>,
        size: NSUInteger,
    ) {
        msg_send![self, decodeValueOfObjCType: type_, at: data, size: size]
    }
    pub unsafe fn versionForClassName(&self, className: &NSString) -> NSInteger {
        msg_send![self, versionForClassName: className]
    }
}
#[doc = "NSExtendedCoder"]
impl NSCoder {
    pub unsafe fn encodeObject(&self, object: Option<&Object>) {
        msg_send![self, encodeObject: object]
    }
    pub unsafe fn encodeRootObject(&self, rootObject: &Object) {
        msg_send![self, encodeRootObject: rootObject]
    }
    pub unsafe fn encodeBycopyObject(&self, anObject: Option<&Object>) {
        msg_send![self, encodeBycopyObject: anObject]
    }
    pub unsafe fn encodeByrefObject(&self, anObject: Option<&Object>) {
        msg_send![self, encodeByrefObject: anObject]
    }
    pub unsafe fn encodeConditionalObject(&self, object: Option<&Object>) {
        msg_send![self, encodeConditionalObject: object]
    }
    pub unsafe fn encodeArrayOfObjCType_count_at(
        &self,
        type_: NonNull<c_char>,
        count: NSUInteger,
        array: NonNull<c_void>,
    ) {
        msg_send![self, encodeArrayOfObjCType: type_, count: count, at: array]
    }
    pub unsafe fn encodeBytes_length(&self, byteaddr: *mut c_void, length: NSUInteger) {
        msg_send![self, encodeBytes: byteaddr, length: length]
    }
    pub unsafe fn decodeObject(&self) -> Option<Id<Object, Shared>> {
        msg_send_id![self, decodeObject]
    }
    pub unsafe fn decodeTopLevelObjectAndReturnError(
        &self,
        error: *mut Option<&NSError>,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![self, decodeTopLevelObjectAndReturnError: error]
    }
    pub unsafe fn decodeArrayOfObjCType_count_at(
        &self,
        itemType: NonNull<c_char>,
        count: NSUInteger,
        array: NonNull<c_void>,
    ) {
        msg_send![
            self,
            decodeArrayOfObjCType: itemType,
            count: count,
            at: array
        ]
    }
    pub unsafe fn decodeBytesWithReturnedLength(
        &self,
        lengthp: NonNull<NSUInteger>,
    ) -> *mut c_void {
        msg_send![self, decodeBytesWithReturnedLength: lengthp]
    }
    pub unsafe fn encodePropertyList(&self, aPropertyList: &Object) {
        msg_send![self, encodePropertyList: aPropertyList]
    }
    pub unsafe fn decodePropertyList(&self) -> Option<Id<Object, Shared>> {
        msg_send_id![self, decodePropertyList]
    }
    pub unsafe fn setObjectZone(&self, zone: *mut NSZone) {
        msg_send![self, setObjectZone: zone]
    }
    pub unsafe fn objectZone(&self) -> *mut NSZone {
        msg_send![self, objectZone]
    }
    pub unsafe fn encodeObject_forKey(&self, object: Option<&Object>, key: &NSString) {
        msg_send![self, encodeObject: object, forKey: key]
    }
    pub unsafe fn encodeConditionalObject_forKey(&self, object: Option<&Object>, key: &NSString) {
        msg_send![self, encodeConditionalObject: object, forKey: key]
    }
    pub unsafe fn encodeBool_forKey(&self, value: bool, key: &NSString) {
        msg_send![self, encodeBool: value, forKey: key]
    }
    pub unsafe fn encodeInt_forKey(&self, value: c_int, key: &NSString) {
        msg_send![self, encodeInt: value, forKey: key]
    }
    pub unsafe fn encodeInt32_forKey(&self, value: int32_t, key: &NSString) {
        msg_send![self, encodeInt32: value, forKey: key]
    }
    pub unsafe fn encodeInt64_forKey(&self, value: int64_t, key: &NSString) {
        msg_send![self, encodeInt64: value, forKey: key]
    }
    pub unsafe fn encodeFloat_forKey(&self, value: c_float, key: &NSString) {
        msg_send![self, encodeFloat: value, forKey: key]
    }
    pub unsafe fn encodeDouble_forKey(&self, value: c_double, key: &NSString) {
        msg_send![self, encodeDouble: value, forKey: key]
    }
    pub unsafe fn encodeBytes_length_forKey(
        &self,
        bytes: *mut uint8_t,
        length: NSUInteger,
        key: &NSString,
    ) {
        msg_send![self, encodeBytes: bytes, length: length, forKey: key]
    }
    pub unsafe fn containsValueForKey(&self, key: &NSString) -> bool {
        msg_send![self, containsValueForKey: key]
    }
    pub unsafe fn decodeObjectForKey(&self, key: &NSString) -> Option<Id<Object, Shared>> {
        msg_send_id![self, decodeObjectForKey: key]
    }
    pub unsafe fn decodeTopLevelObjectForKey_error(
        &self,
        key: &NSString,
        error: *mut Option<&NSError>,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![self, decodeTopLevelObjectForKey: key, error: error]
    }
    pub unsafe fn decodeBoolForKey(&self, key: &NSString) -> bool {
        msg_send![self, decodeBoolForKey: key]
    }
    pub unsafe fn decodeIntForKey(&self, key: &NSString) -> c_int {
        msg_send![self, decodeIntForKey: key]
    }
    pub unsafe fn decodeInt32ForKey(&self, key: &NSString) -> int32_t {
        msg_send![self, decodeInt32ForKey: key]
    }
    pub unsafe fn decodeInt64ForKey(&self, key: &NSString) -> int64_t {
        msg_send![self, decodeInt64ForKey: key]
    }
    pub unsafe fn decodeFloatForKey(&self, key: &NSString) -> c_float {
        msg_send![self, decodeFloatForKey: key]
    }
    pub unsafe fn decodeDoubleForKey(&self, key: &NSString) -> c_double {
        msg_send![self, decodeDoubleForKey: key]
    }
    pub unsafe fn decodeBytesForKey_returnedLength(
        &self,
        key: &NSString,
        lengthp: *mut NSUInteger,
    ) -> *mut uint8_t {
        msg_send![self, decodeBytesForKey: key, returnedLength: lengthp]
    }
    pub unsafe fn encodeInteger_forKey(&self, value: NSInteger, key: &NSString) {
        msg_send![self, encodeInteger: value, forKey: key]
    }
    pub unsafe fn decodeIntegerForKey(&self, key: &NSString) -> NSInteger {
        msg_send![self, decodeIntegerForKey: key]
    }
    pub unsafe fn decodeObjectOfClass_forKey(
        &self,
        aClass: &Class,
        key: &NSString,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![self, decodeObjectOfClass: aClass, forKey: key]
    }
    pub unsafe fn decodeTopLevelObjectOfClass_forKey_error(
        &self,
        aClass: &Class,
        key: &NSString,
        error: *mut Option<&NSError>,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![
            self,
            decodeTopLevelObjectOfClass: aClass,
            forKey: key,
            error: error
        ]
    }
    pub unsafe fn decodeArrayOfObjectsOfClass_forKey(
        &self,
        cls: &Class,
        key: &NSString,
    ) -> Option<Id<NSArray, Shared>> {
        msg_send_id![self, decodeArrayOfObjectsOfClass: cls, forKey: key]
    }
    pub unsafe fn decodeDictionaryWithKeysOfClass_objectsOfClass_forKey(
        &self,
        keyCls: &Class,
        objectCls: &Class,
        key: &NSString,
    ) -> Option<Id<NSDictionary, Shared>> {
        msg_send_id![
            self,
            decodeDictionaryWithKeysOfClass: keyCls,
            objectsOfClass: objectCls,
            forKey: key
        ]
    }
    pub unsafe fn decodeObjectOfClasses_forKey(
        &self,
        classes: TodoGenerics,
        key: &NSString,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![self, decodeObjectOfClasses: classes, forKey: key]
    }
    pub unsafe fn decodeTopLevelObjectOfClasses_forKey_error(
        &self,
        classes: TodoGenerics,
        key: &NSString,
        error: *mut Option<&NSError>,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![
            self,
            decodeTopLevelObjectOfClasses: classes,
            forKey: key,
            error: error
        ]
    }
    pub unsafe fn decodeArrayOfObjectsOfClasses_forKey(
        &self,
        classes: TodoGenerics,
        key: &NSString,
    ) -> Option<Id<NSArray, Shared>> {
        msg_send_id![self, decodeArrayOfObjectsOfClasses: classes, forKey: key]
    }
    pub unsafe fn decodeDictionaryWithKeysOfClasses_objectsOfClasses_forKey(
        &self,
        keyClasses: TodoGenerics,
        objectClasses: TodoGenerics,
        key: &NSString,
    ) -> Option<Id<NSDictionary, Shared>> {
        msg_send_id![
            self,
            decodeDictionaryWithKeysOfClasses: keyClasses,
            objectsOfClasses: objectClasses,
            forKey: key
        ]
    }
    pub unsafe fn decodePropertyListForKey(&self, key: &NSString) -> Option<Id<Object, Shared>> {
        msg_send_id![self, decodePropertyListForKey: key]
    }
    pub unsafe fn failWithError(&self, error: &NSError) {
        msg_send![self, failWithError: error]
    }
    pub unsafe fn systemVersion(&self) -> c_uint {
        msg_send![self, systemVersion]
    }
    pub unsafe fn allowsKeyedCoding(&self) -> bool {
        msg_send![self, allowsKeyedCoding]
    }
    pub unsafe fn requiresSecureCoding(&self) -> bool {
        msg_send![self, requiresSecureCoding]
    }
    pub unsafe fn allowedClasses(&self) -> TodoGenerics {
        msg_send![self, allowedClasses]
    }
    pub unsafe fn decodingFailurePolicy(&self) -> NSDecodingFailurePolicy {
        msg_send![self, decodingFailurePolicy]
    }
    pub unsafe fn error(&self) -> Option<Id<NSError, Shared>> {
        msg_send_id![self, error]
    }
}
#[doc = "NSTypedstreamCompatibility"]
impl NSCoder {
    pub unsafe fn encodeNXObject(&self, object: &Object) {
        msg_send![self, encodeNXObject: object]
    }
    pub unsafe fn decodeNXObject(&self) -> Option<Id<Object, Shared>> {
        msg_send_id![self, decodeNXObject]
    }
}
#[doc = "NSDeprecated"]
impl NSCoder {
    pub unsafe fn decodeValueOfObjCType_at(&self, type_: NonNull<c_char>, data: NonNull<c_void>) {
        msg_send![self, decodeValueOfObjCType: type_, at: data]
    }
}
