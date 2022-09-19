use super::__exported::NSString;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
#[doc = "NSScriptKeyValueCoding"]
impl NSObject {
    pub unsafe fn valueAtIndex_inPropertyWithKey(
        &self,
        index: NSUInteger,
        key: &NSString,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![self, valueAtIndex: index, inPropertyWithKey: key]
    }
    pub unsafe fn valueWithName_inPropertyWithKey(
        &self,
        name: &NSString,
        key: &NSString,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![self, valueWithName: name, inPropertyWithKey: key]
    }
    pub unsafe fn valueWithUniqueID_inPropertyWithKey(
        &self,
        uniqueID: &Object,
        key: &NSString,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![self, valueWithUniqueID: uniqueID, inPropertyWithKey: key]
    }
    pub unsafe fn insertValue_atIndex_inPropertyWithKey(
        &self,
        value: &Object,
        index: NSUInteger,
        key: &NSString,
    ) {
        msg_send![
            self,
            insertValue: value,
            atIndex: index,
            inPropertyWithKey: key
        ]
    }
    pub unsafe fn removeValueAtIndex_fromPropertyWithKey(&self, index: NSUInteger, key: &NSString) {
        msg_send![self, removeValueAtIndex: index, fromPropertyWithKey: key]
    }
    pub unsafe fn replaceValueAtIndex_inPropertyWithKey_withValue(
        &self,
        index: NSUInteger,
        key: &NSString,
        value: &Object,
    ) {
        msg_send![
            self,
            replaceValueAtIndex: index,
            inPropertyWithKey: key,
            withValue: value
        ]
    }
    pub unsafe fn insertValue_inPropertyWithKey(&self, value: &Object, key: &NSString) {
        msg_send![self, insertValue: value, inPropertyWithKey: key]
    }
    pub unsafe fn coerceValue_forKey(
        &self,
        value: Option<&Object>,
        key: &NSString,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![self, coerceValue: value, forKey: key]
    }
}
