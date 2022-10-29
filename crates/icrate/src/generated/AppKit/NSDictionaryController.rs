#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSDictionaryControllerKeyValuePair;
    unsafe impl ClassType for NSDictionaryControllerKeyValuePair {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSDictionaryControllerKeyValuePair {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(key)]
        pub unsafe fn key(&self) -> Option<Id<NSString, Shared>>;
        #[method(setKey:)]
        pub unsafe fn setKey(&self, key: Option<&NSString>);
        #[method_id(value)]
        pub unsafe fn value(&self) -> Option<Id<Object, Shared>>;
        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: Option<&Object>);
        #[method_id(localizedKey)]
        pub unsafe fn localizedKey(&self) -> Option<Id<NSString, Shared>>;
        #[method(setLocalizedKey:)]
        pub unsafe fn setLocalizedKey(&self, localizedKey: Option<&NSString>);
        #[method(isExplicitlyIncluded)]
        pub unsafe fn isExplicitlyIncluded(&self) -> bool;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSDictionaryController;
    unsafe impl ClassType for NSDictionaryController {
        type Super = NSArrayController;
    }
);
extern_methods!(
    unsafe impl NSDictionaryController {
        #[method_id(newObject)]
        pub unsafe fn newObject(&self) -> Id<NSDictionaryControllerKeyValuePair, Shared>;
        #[method_id(initialKey)]
        pub unsafe fn initialKey(&self) -> Id<NSString, Shared>;
        #[method(setInitialKey:)]
        pub unsafe fn setInitialKey(&self, initialKey: &NSString);
        #[method_id(initialValue)]
        pub unsafe fn initialValue(&self) -> Id<Object, Shared>;
        #[method(setInitialValue:)]
        pub unsafe fn setInitialValue(&self, initialValue: &Object);
        #[method_id(includedKeys)]
        pub unsafe fn includedKeys(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(setIncludedKeys:)]
        pub unsafe fn setIncludedKeys(&self, includedKeys: &NSArray<NSString>);
        #[method_id(excludedKeys)]
        pub unsafe fn excludedKeys(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(setExcludedKeys:)]
        pub unsafe fn setExcludedKeys(&self, excludedKeys: &NSArray<NSString>);
        #[method_id(localizedKeyDictionary)]
        pub unsafe fn localizedKeyDictionary(&self)
            -> Id<NSDictionary<NSString, NSString>, Shared>;
        #[method(setLocalizedKeyDictionary:)]
        pub unsafe fn setLocalizedKeyDictionary(
            &self,
            localizedKeyDictionary: &NSDictionary<NSString, NSString>,
        );
        #[method_id(localizedKeyTable)]
        pub unsafe fn localizedKeyTable(&self) -> Option<Id<NSString, Shared>>;
        #[method(setLocalizedKeyTable:)]
        pub unsafe fn setLocalizedKeyTable(&self, localizedKeyTable: Option<&NSString>);
    }
);