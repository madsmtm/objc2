use super::__exported::NSError;
use super::__exported::NSString;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSDictionary::*;
use crate::Foundation::generated::NSException::*;
use crate::Foundation::generated::NSOrderedSet::*;
use crate::Foundation::generated::NSSet::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
pub type NSKeyValueOperator = NSString;
#[doc = "NSKeyValueCoding"]
impl NSObject {
    pub unsafe fn valueForKey(&self, key: &NSString) -> Option<Id<Object, Shared>> {
        msg_send_id![self, valueForKey: key]
    }
    pub unsafe fn setValue_forKey(&self, value: Option<&Object>, key: &NSString) {
        msg_send![self, setValue: value, forKey: key]
    }
    pub unsafe fn validateValue_forKey_error(
        &self,
        ioValue: NonNull<*mut Object>,
        inKey: &NSString,
        outError: *mut *mut NSError,
    ) -> bool {
        msg_send![self, validateValue: ioValue, forKey: inKey, error: outError]
    }
    pub unsafe fn mutableArrayValueForKey(&self, key: &NSString) -> Id<NSMutableArray, Shared> {
        msg_send_id![self, mutableArrayValueForKey: key]
    }
    pub unsafe fn mutableOrderedSetValueForKey(
        &self,
        key: &NSString,
    ) -> Id<NSMutableOrderedSet, Shared> {
        msg_send_id![self, mutableOrderedSetValueForKey: key]
    }
    pub unsafe fn mutableSetValueForKey(&self, key: &NSString) -> Id<NSMutableSet, Shared> {
        msg_send_id![self, mutableSetValueForKey: key]
    }
    pub unsafe fn valueForKeyPath(&self, keyPath: &NSString) -> Option<Id<Object, Shared>> {
        msg_send_id![self, valueForKeyPath: keyPath]
    }
    pub unsafe fn setValue_forKeyPath(&self, value: Option<&Object>, keyPath: &NSString) {
        msg_send![self, setValue: value, forKeyPath: keyPath]
    }
    pub unsafe fn validateValue_forKeyPath_error(
        &self,
        ioValue: NonNull<*mut Object>,
        inKeyPath: &NSString,
        outError: *mut *mut NSError,
    ) -> bool {
        msg_send![
            self,
            validateValue: ioValue,
            forKeyPath: inKeyPath,
            error: outError
        ]
    }
    pub unsafe fn mutableArrayValueForKeyPath(
        &self,
        keyPath: &NSString,
    ) -> Id<NSMutableArray, Shared> {
        msg_send_id![self, mutableArrayValueForKeyPath: keyPath]
    }
    pub unsafe fn mutableOrderedSetValueForKeyPath(
        &self,
        keyPath: &NSString,
    ) -> Id<NSMutableOrderedSet, Shared> {
        msg_send_id![self, mutableOrderedSetValueForKeyPath: keyPath]
    }
    pub unsafe fn mutableSetValueForKeyPath(&self, keyPath: &NSString) -> Id<NSMutableSet, Shared> {
        msg_send_id![self, mutableSetValueForKeyPath: keyPath]
    }
    pub unsafe fn valueForUndefinedKey(&self, key: &NSString) -> Option<Id<Object, Shared>> {
        msg_send_id![self, valueForUndefinedKey: key]
    }
    pub unsafe fn setValue_forUndefinedKey(&self, value: Option<&Object>, key: &NSString) {
        msg_send![self, setValue: value, forUndefinedKey: key]
    }
    pub unsafe fn setNilValueForKey(&self, key: &NSString) {
        msg_send![self, setNilValueForKey: key]
    }
    pub unsafe fn dictionaryWithValuesForKeys(
        &self,
        keys: &NSArray<NSString>,
    ) -> Id<NSDictionary<NSString, Object>, Shared> {
        msg_send_id![self, dictionaryWithValuesForKeys: keys]
    }
    pub unsafe fn setValuesForKeysWithDictionary(
        &self,
        keyedValues: &NSDictionary<NSString, Object>,
    ) {
        msg_send![self, setValuesForKeysWithDictionary: keyedValues]
    }
    pub unsafe fn accessInstanceVariablesDirectly() -> bool {
        msg_send![Self::class(), accessInstanceVariablesDirectly]
    }
}
#[doc = "NSKeyValueCoding"]
impl<ObjectType: Message> NSArray<ObjectType> {
    pub unsafe fn valueForKey(&self, key: &NSString) -> Id<Object, Shared> {
        msg_send_id![self, valueForKey: key]
    }
    pub unsafe fn setValue_forKey(&self, value: Option<&Object>, key: &NSString) {
        msg_send![self, setValue: value, forKey: key]
    }
}
#[doc = "NSKeyValueCoding"]
impl<KeyType: Message, ObjectType: Message> NSDictionary<KeyType, ObjectType> {
    pub unsafe fn valueForKey(&self, key: &NSString) -> Option<Id<ObjectType, Shared>> {
        msg_send_id![self, valueForKey: key]
    }
}
#[doc = "NSKeyValueCoding"]
impl<KeyType: Message, ObjectType: Message> NSMutableDictionary<KeyType, ObjectType> {
    pub unsafe fn setValue_forKey(&self, value: Option<&ObjectType>, key: &NSString) {
        msg_send![self, setValue: value, forKey: key]
    }
}
#[doc = "NSKeyValueCoding"]
impl<ObjectType: Message> NSOrderedSet<ObjectType> {
    pub unsafe fn valueForKey(&self, key: &NSString) -> Id<Object, Shared> {
        msg_send_id![self, valueForKey: key]
    }
    pub unsafe fn setValue_forKey(&self, value: Option<&Object>, key: &NSString) {
        msg_send![self, setValue: value, forKey: key]
    }
}
#[doc = "NSKeyValueCoding"]
impl<ObjectType: Message> NSSet<ObjectType> {
    pub unsafe fn valueForKey(&self, key: &NSString) -> Id<Object, Shared> {
        msg_send_id![self, valueForKey: key]
    }
    pub unsafe fn setValue_forKey(&self, value: Option<&Object>, key: &NSString) {
        msg_send![self, setValue: value, forKey: key]
    }
}
#[doc = "NSDeprecatedKeyValueCoding"]
impl NSObject {
    pub unsafe fn useStoredAccessor() -> bool {
        msg_send![Self::class(), useStoredAccessor]
    }
    pub unsafe fn storedValueForKey(&self, key: &NSString) -> Option<Id<Object, Shared>> {
        msg_send_id![self, storedValueForKey: key]
    }
    pub unsafe fn takeStoredValue_forKey(&self, value: Option<&Object>, key: &NSString) {
        msg_send![self, takeStoredValue: value, forKey: key]
    }
    pub unsafe fn takeValue_forKey(&self, value: Option<&Object>, key: &NSString) {
        msg_send![self, takeValue: value, forKey: key]
    }
    pub unsafe fn takeValue_forKeyPath(&self, value: Option<&Object>, keyPath: &NSString) {
        msg_send![self, takeValue: value, forKeyPath: keyPath]
    }
    pub unsafe fn handleQueryWithUnboundKey(&self, key: &NSString) -> Option<Id<Object, Shared>> {
        msg_send_id![self, handleQueryWithUnboundKey: key]
    }
    pub unsafe fn handleTakeValue_forUnboundKey(&self, value: Option<&Object>, key: &NSString) {
        msg_send![self, handleTakeValue: value, forUnboundKey: key]
    }
    pub unsafe fn unableToSetNilForKey(&self, key: &NSString) {
        msg_send![self, unableToSetNilForKey: key]
    }
    pub unsafe fn valuesForKeys(&self, keys: &NSArray) -> Id<NSDictionary, Shared> {
        msg_send_id![self, valuesForKeys: keys]
    }
    pub unsafe fn takeValuesFromDictionary(&self, properties: &NSDictionary) {
        msg_send![self, takeValuesFromDictionary: properties]
    }
}
