#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_methods!(
    #[doc = "NSScriptKeyValueCoding"]
    unsafe impl NSObject {
        #[method_id(valueAtIndex:inPropertyWithKey:)]
        pub unsafe fn valueAtIndex_inPropertyWithKey(
            &self,
            index: NSUInteger,
            key: &NSString,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(valueWithName:inPropertyWithKey:)]
        pub unsafe fn valueWithName_inPropertyWithKey(
            &self,
            name: &NSString,
            key: &NSString,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(valueWithUniqueID:inPropertyWithKey:)]
        pub unsafe fn valueWithUniqueID_inPropertyWithKey(
            &self,
            uniqueID: &Object,
            key: &NSString,
        ) -> Option<Id<Object, Shared>>;
        #[method(insertValue:atIndex:inPropertyWithKey:)]
        pub unsafe fn insertValue_atIndex_inPropertyWithKey(
            &self,
            value: &Object,
            index: NSUInteger,
            key: &NSString,
        );
        #[method(removeValueAtIndex:fromPropertyWithKey:)]
        pub unsafe fn removeValueAtIndex_fromPropertyWithKey(
            &self,
            index: NSUInteger,
            key: &NSString,
        );
        #[method(replaceValueAtIndex:inPropertyWithKey:withValue:)]
        pub unsafe fn replaceValueAtIndex_inPropertyWithKey_withValue(
            &self,
            index: NSUInteger,
            key: &NSString,
            value: &Object,
        );
        #[method(insertValue:inPropertyWithKey:)]
        pub unsafe fn insertValue_inPropertyWithKey(&self, value: &Object, key: &NSString);
        #[method_id(coerceValue:forKey:)]
        pub unsafe fn coerceValue_forKey(
            &self,
            value: Option<&Object>,
            key: &NSString,
        ) -> Option<Id<Object, Shared>>;
    }
);
