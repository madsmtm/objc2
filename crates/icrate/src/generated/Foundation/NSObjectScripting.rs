#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
#[doc = "NSScripting"]
impl NSObject {
    pub unsafe fn scriptingValueForSpecifier(
        &self,
        objectSpecifier: &NSScriptObjectSpecifier,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![self, scriptingValueForSpecifier: objectSpecifier]
    }
    pub unsafe fn copyScriptingValue_forKey_withProperties(
        &self,
        value: &Object,
        key: &NSString,
        properties: TodoGenerics,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![
            self,
            copyScriptingValue: value,
            forKey: key,
            withProperties: properties
        ]
    }
    pub unsafe fn newScriptingObjectOfClass_forValueForKey_withContentsValue_properties(
        &self,
        objectClass: &Class,
        key: &NSString,
        contentsValue: Option<&Object>,
        properties: TodoGenerics,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![
            self,
            newScriptingObjectOfClass: objectClass,
            forValueForKey: key,
            withContentsValue: contentsValue,
            properties: properties
        ]
    }
    pub unsafe fn scriptingProperties(&self) -> TodoGenerics {
        msg_send![self, scriptingProperties]
    }
    pub unsafe fn setScriptingProperties(&self, scriptingProperties: TodoGenerics) {
        msg_send![self, setScriptingProperties: scriptingProperties]
    }
}
