#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSUbiquitousKeyValueStore;
    unsafe impl ClassType for NSUbiquitousKeyValueStore {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSUbiquitousKeyValueStore {
        #[method_id(defaultStore)]
        pub unsafe fn defaultStore() -> Id<NSUbiquitousKeyValueStore, Shared>;
        #[method_id(objectForKey:)]
        pub unsafe fn objectForKey(&self, aKey: &NSString) -> Option<Id<Object, Shared>>;
        #[method(setObject:forKey:)]
        pub unsafe fn setObject_forKey(&self, anObject: Option<&Object>, aKey: &NSString);
        #[method(removeObjectForKey:)]
        pub unsafe fn removeObjectForKey(&self, aKey: &NSString);
        #[method_id(stringForKey:)]
        pub unsafe fn stringForKey(&self, aKey: &NSString) -> Option<Id<NSString, Shared>>;
        #[method_id(arrayForKey:)]
        pub unsafe fn arrayForKey(&self, aKey: &NSString) -> Option<Id<NSArray, Shared>>;
        #[method_id(dictionaryForKey:)]
        pub unsafe fn dictionaryForKey(
            &self,
            aKey: &NSString,
        ) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;
        #[method_id(dataForKey:)]
        pub unsafe fn dataForKey(&self, aKey: &NSString) -> Option<Id<NSData, Shared>>;
        #[method(longLongForKey:)]
        pub unsafe fn longLongForKey(&self, aKey: &NSString) -> c_longlong;
        #[method(doubleForKey:)]
        pub unsafe fn doubleForKey(&self, aKey: &NSString) -> c_double;
        #[method(boolForKey:)]
        pub unsafe fn boolForKey(&self, aKey: &NSString) -> bool;
        #[method(setString:forKey:)]
        pub unsafe fn setString_forKey(&self, aString: Option<&NSString>, aKey: &NSString);
        #[method(setData:forKey:)]
        pub unsafe fn setData_forKey(&self, aData: Option<&NSData>, aKey: &NSString);
        #[method(setArray:forKey:)]
        pub unsafe fn setArray_forKey(&self, anArray: Option<&NSArray>, aKey: &NSString);
        #[method(setDictionary:forKey:)]
        pub unsafe fn setDictionary_forKey(
            &self,
            aDictionary: Option<&NSDictionary<NSString, Object>>,
            aKey: &NSString,
        );
        #[method(setLongLong:forKey:)]
        pub unsafe fn setLongLong_forKey(&self, value: c_longlong, aKey: &NSString);
        #[method(setDouble:forKey:)]
        pub unsafe fn setDouble_forKey(&self, value: c_double, aKey: &NSString);
        #[method(setBool:forKey:)]
        pub unsafe fn setBool_forKey(&self, value: bool, aKey: &NSString);
        #[method_id(dictionaryRepresentation)]
        pub unsafe fn dictionaryRepresentation(&self)
            -> Id<NSDictionary<NSString, Object>, Shared>;
        #[method(synchronize)]
        pub unsafe fn synchronize(&self) -> bool;
    }
);
