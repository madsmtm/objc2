use crate::Foundation::generated::NSDictionary::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSString::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPersonNameComponents;
    unsafe impl ClassType for NSPersonNameComponents {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSPersonNameComponents {
        #[method_id(namePrefix)]
        pub unsafe fn namePrefix(&self) -> Option<Id<NSString, Shared>>;
        #[method(setNamePrefix:)]
        pub unsafe fn setNamePrefix(&self, namePrefix: Option<&NSString>);
        #[method_id(givenName)]
        pub unsafe fn givenName(&self) -> Option<Id<NSString, Shared>>;
        #[method(setGivenName:)]
        pub unsafe fn setGivenName(&self, givenName: Option<&NSString>);
        #[method_id(middleName)]
        pub unsafe fn middleName(&self) -> Option<Id<NSString, Shared>>;
        #[method(setMiddleName:)]
        pub unsafe fn setMiddleName(&self, middleName: Option<&NSString>);
        #[method_id(familyName)]
        pub unsafe fn familyName(&self) -> Option<Id<NSString, Shared>>;
        #[method(setFamilyName:)]
        pub unsafe fn setFamilyName(&self, familyName: Option<&NSString>);
        #[method_id(nameSuffix)]
        pub unsafe fn nameSuffix(&self) -> Option<Id<NSString, Shared>>;
        #[method(setNameSuffix:)]
        pub unsafe fn setNameSuffix(&self, nameSuffix: Option<&NSString>);
        #[method_id(nickname)]
        pub unsafe fn nickname(&self) -> Option<Id<NSString, Shared>>;
        #[method(setNickname:)]
        pub unsafe fn setNickname(&self, nickname: Option<&NSString>);
        #[method_id(phoneticRepresentation)]
        pub unsafe fn phoneticRepresentation(&self) -> Option<Id<NSPersonNameComponents, Shared>>;
        #[method(setPhoneticRepresentation:)]
        pub unsafe fn setPhoneticRepresentation(
            &self,
            phoneticRepresentation: Option<&NSPersonNameComponents>,
        );
    }
);
