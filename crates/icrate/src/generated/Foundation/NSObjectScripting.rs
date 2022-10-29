#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_methods!(
    #[doc = "NSScripting"]
    unsafe impl NSObject {
        #[method_id(scriptingValueForSpecifier:)]
        pub unsafe fn scriptingValueForSpecifier(
            &self,
            objectSpecifier: &NSScriptObjectSpecifier,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(scriptingProperties)]
        pub unsafe fn scriptingProperties(
            &self,
        ) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;
        #[method(setScriptingProperties:)]
        pub unsafe fn setScriptingProperties(
            &self,
            scriptingProperties: Option<&NSDictionary<NSString, Object>>,
        );
        #[method_id(copyScriptingValue:forKey:withProperties:)]
        pub unsafe fn copyScriptingValue_forKey_withProperties(
            &self,
            value: &Object,
            key: &NSString,
            properties: &NSDictionary<NSString, Object>,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(newScriptingObjectOfClass:forValueForKey:withContentsValue:properties:)]
        pub unsafe fn newScriptingObjectOfClass_forValueForKey_withContentsValue_properties(
            &self,
            objectClass: &Class,
            key: &NSString,
            contentsValue: Option<&Object>,
            properties: &NSDictionary<NSString, Object>,
        ) -> Option<Id<Object, Shared>>;
    }
);
