use super::__exported::NSArray;
use super::__exported::NSData;
use super::__exported::NSDictionary;
use super::__exported::NSString;
use crate::Foundation::generated::NSNotification::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSUbiquitousKeyValueStore;
    unsafe impl ClassType for NSUbiquitousKeyValueStore {
        type Super = NSObject;
    }
);
impl NSUbiquitousKeyValueStore {
    pub unsafe fn defaultStore() -> Id<NSUbiquitousKeyValueStore, Shared> {
        msg_send_id![Self::class(), defaultStore]
    }
    pub unsafe fn objectForKey(&self, aKey: &NSString) -> Option<Id<Object, Shared>> {
        msg_send_id![self, objectForKey: aKey]
    }
    pub unsafe fn setObject_forKey(&self, anObject: Option<&Object>, aKey: &NSString) {
        msg_send![self, setObject: anObject, forKey: aKey]
    }
    pub unsafe fn removeObjectForKey(&self, aKey: &NSString) {
        msg_send![self, removeObjectForKey: aKey]
    }
    pub unsafe fn stringForKey(&self, aKey: &NSString) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, stringForKey: aKey]
    }
    pub unsafe fn arrayForKey(&self, aKey: &NSString) -> Option<Id<NSArray, Shared>> {
        msg_send_id![self, arrayForKey: aKey]
    }
    pub unsafe fn dictionaryForKey(
        &self,
        aKey: &NSString,
    ) -> Option<Id<NSDictionary<NSString, Object>, Shared>> {
        msg_send_id![self, dictionaryForKey: aKey]
    }
    pub unsafe fn dataForKey(&self, aKey: &NSString) -> Option<Id<NSData, Shared>> {
        msg_send_id![self, dataForKey: aKey]
    }
    pub unsafe fn longLongForKey(&self, aKey: &NSString) -> c_longlong {
        msg_send![self, longLongForKey: aKey]
    }
    pub unsafe fn doubleForKey(&self, aKey: &NSString) -> c_double {
        msg_send![self, doubleForKey: aKey]
    }
    pub unsafe fn boolForKey(&self, aKey: &NSString) -> bool {
        msg_send![self, boolForKey: aKey]
    }
    pub unsafe fn setString_forKey(&self, aString: Option<&NSString>, aKey: &NSString) {
        msg_send![self, setString: aString, forKey: aKey]
    }
    pub unsafe fn setData_forKey(&self, aData: Option<&NSData>, aKey: &NSString) {
        msg_send![self, setData: aData, forKey: aKey]
    }
    pub unsafe fn setArray_forKey(&self, anArray: Option<&NSArray>, aKey: &NSString) {
        msg_send![self, setArray: anArray, forKey: aKey]
    }
    pub unsafe fn setDictionary_forKey(
        &self,
        aDictionary: Option<&NSDictionary<NSString, Object>>,
        aKey: &NSString,
    ) {
        msg_send![self, setDictionary: aDictionary, forKey: aKey]
    }
    pub unsafe fn setLongLong_forKey(&self, value: c_longlong, aKey: &NSString) {
        msg_send![self, setLongLong: value, forKey: aKey]
    }
    pub unsafe fn setDouble_forKey(&self, value: c_double, aKey: &NSString) {
        msg_send![self, setDouble: value, forKey: aKey]
    }
    pub unsafe fn setBool_forKey(&self, value: bool, aKey: &NSString) {
        msg_send![self, setBool: value, forKey: aKey]
    }
    pub unsafe fn dictionaryRepresentation(&self) -> Id<NSDictionary<NSString, Object>, Shared> {
        msg_send_id![self, dictionaryRepresentation]
    }
    pub unsafe fn synchronize(&self) -> bool {
        msg_send![self, synchronize]
    }
}
