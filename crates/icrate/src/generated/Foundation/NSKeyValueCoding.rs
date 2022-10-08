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
use objc2::{extern_class, extern_methods, ClassType};
pub type NSKeyValueOperator = NSString;
extern_methods!(
    #[doc = "NSKeyValueCoding"]
    unsafe impl NSObject {
        #[method(accessInstanceVariablesDirectly)]
        pub unsafe fn accessInstanceVariablesDirectly() -> bool;
        # [method_id (valueForKey :)]
        pub unsafe fn valueForKey(&self, key: &NSString) -> Option<Id<Object, Shared>>;
        # [method (setValue : forKey :)]
        pub unsafe fn setValue_forKey(&self, value: Option<&Object>, key: &NSString);
        # [method (validateValue : forKey : error :)]
        pub unsafe fn validateValue_forKey_error(
            &self,
            ioValue: &mut Option<Id<Object, Shared>>,
            inKey: &NSString,
        ) -> Result<(), Id<NSError, Shared>>;
        # [method_id (mutableArrayValueForKey :)]
        pub unsafe fn mutableArrayValueForKey(&self, key: &NSString) -> Id<NSMutableArray, Shared>;
        # [method_id (mutableOrderedSetValueForKey :)]
        pub unsafe fn mutableOrderedSetValueForKey(
            &self,
            key: &NSString,
        ) -> Id<NSMutableOrderedSet, Shared>;
        # [method_id (mutableSetValueForKey :)]
        pub unsafe fn mutableSetValueForKey(&self, key: &NSString) -> Id<NSMutableSet, Shared>;
        # [method_id (valueForKeyPath :)]
        pub unsafe fn valueForKeyPath(&self, keyPath: &NSString) -> Option<Id<Object, Shared>>;
        # [method (setValue : forKeyPath :)]
        pub unsafe fn setValue_forKeyPath(&self, value: Option<&Object>, keyPath: &NSString);
        # [method (validateValue : forKeyPath : error :)]
        pub unsafe fn validateValue_forKeyPath_error(
            &self,
            ioValue: &mut Option<Id<Object, Shared>>,
            inKeyPath: &NSString,
        ) -> Result<(), Id<NSError, Shared>>;
        # [method_id (mutableArrayValueForKeyPath :)]
        pub unsafe fn mutableArrayValueForKeyPath(
            &self,
            keyPath: &NSString,
        ) -> Id<NSMutableArray, Shared>;
        # [method_id (mutableOrderedSetValueForKeyPath :)]
        pub unsafe fn mutableOrderedSetValueForKeyPath(
            &self,
            keyPath: &NSString,
        ) -> Id<NSMutableOrderedSet, Shared>;
        # [method_id (mutableSetValueForKeyPath :)]
        pub unsafe fn mutableSetValueForKeyPath(
            &self,
            keyPath: &NSString,
        ) -> Id<NSMutableSet, Shared>;
        # [method_id (valueForUndefinedKey :)]
        pub unsafe fn valueForUndefinedKey(&self, key: &NSString) -> Option<Id<Object, Shared>>;
        # [method (setValue : forUndefinedKey :)]
        pub unsafe fn setValue_forUndefinedKey(&self, value: Option<&Object>, key: &NSString);
        # [method (setNilValueForKey :)]
        pub unsafe fn setNilValueForKey(&self, key: &NSString);
        # [method_id (dictionaryWithValuesForKeys :)]
        pub unsafe fn dictionaryWithValuesForKeys(
            &self,
            keys: &NSArray<NSString>,
        ) -> Id<NSDictionary<NSString, Object>, Shared>;
        # [method (setValuesForKeysWithDictionary :)]
        pub unsafe fn setValuesForKeysWithDictionary(
            &self,
            keyedValues: &NSDictionary<NSString, Object>,
        );
    }
);
extern_methods!(
    #[doc = "NSKeyValueCoding"]
    unsafe impl<ObjectType: Message> NSArray<ObjectType> {
        # [method_id (valueForKey :)]
        pub unsafe fn valueForKey(&self, key: &NSString) -> Id<Object, Shared>;
        # [method (setValue : forKey :)]
        pub unsafe fn setValue_forKey(&self, value: Option<&Object>, key: &NSString);
    }
);
extern_methods!(
    #[doc = "NSKeyValueCoding"]
    unsafe impl<KeyType: Message, ObjectType: Message> NSDictionary<KeyType, ObjectType> {
        # [method_id (valueForKey :)]
        pub unsafe fn valueForKey(&self, key: &NSString) -> Option<Id<ObjectType, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSKeyValueCoding"]
    unsafe impl<KeyType: Message, ObjectType: Message> NSMutableDictionary<KeyType, ObjectType> {
        # [method (setValue : forKey :)]
        pub unsafe fn setValue_forKey(&self, value: Option<&ObjectType>, key: &NSString);
    }
);
extern_methods!(
    #[doc = "NSKeyValueCoding"]
    unsafe impl<ObjectType: Message> NSOrderedSet<ObjectType> {
        # [method_id (valueForKey :)]
        pub unsafe fn valueForKey(&self, key: &NSString) -> Id<Object, Shared>;
        # [method (setValue : forKey :)]
        pub unsafe fn setValue_forKey(&self, value: Option<&Object>, key: &NSString);
    }
);
extern_methods!(
    #[doc = "NSKeyValueCoding"]
    unsafe impl<ObjectType: Message> NSSet<ObjectType> {
        # [method_id (valueForKey :)]
        pub unsafe fn valueForKey(&self, key: &NSString) -> Id<Object, Shared>;
        # [method (setValue : forKey :)]
        pub unsafe fn setValue_forKey(&self, value: Option<&Object>, key: &NSString);
    }
);
extern_methods!(
    #[doc = "NSDeprecatedKeyValueCoding"]
    unsafe impl NSObject {
        #[method(useStoredAccessor)]
        pub unsafe fn useStoredAccessor() -> bool;
        # [method_id (storedValueForKey :)]
        pub unsafe fn storedValueForKey(&self, key: &NSString) -> Option<Id<Object, Shared>>;
        # [method (takeStoredValue : forKey :)]
        pub unsafe fn takeStoredValue_forKey(&self, value: Option<&Object>, key: &NSString);
        # [method (takeValue : forKey :)]
        pub unsafe fn takeValue_forKey(&self, value: Option<&Object>, key: &NSString);
        # [method (takeValue : forKeyPath :)]
        pub unsafe fn takeValue_forKeyPath(&self, value: Option<&Object>, keyPath: &NSString);
        # [method_id (handleQueryWithUnboundKey :)]
        pub unsafe fn handleQueryWithUnboundKey(
            &self,
            key: &NSString,
        ) -> Option<Id<Object, Shared>>;
        # [method (handleTakeValue : forUnboundKey :)]
        pub unsafe fn handleTakeValue_forUnboundKey(&self, value: Option<&Object>, key: &NSString);
        # [method (unableToSetNilForKey :)]
        pub unsafe fn unableToSetNilForKey(&self, key: &NSString);
        # [method_id (valuesForKeys :)]
        pub unsafe fn valuesForKeys(&self, keys: &NSArray) -> Id<NSDictionary, Shared>;
        # [method (takeValuesFromDictionary :)]
        pub unsafe fn takeValuesFromDictionary(&self, properties: &NSDictionary);
    }
);
