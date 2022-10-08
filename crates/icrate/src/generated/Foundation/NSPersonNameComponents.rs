use crate::Foundation::generated::NSDictionary::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSString::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPersonNameComponents;
    unsafe impl ClassType for NSPersonNameComponents {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSPersonNameComponents {
        pub unsafe fn namePrefix(&self) -> Option<Id<NSString, Shared>> {
            msg_send_id![self, namePrefix]
        }
        pub unsafe fn setNamePrefix(&self, namePrefix: Option<&NSString>) {
            msg_send![self, setNamePrefix: namePrefix]
        }
        pub unsafe fn givenName(&self) -> Option<Id<NSString, Shared>> {
            msg_send_id![self, givenName]
        }
        pub unsafe fn setGivenName(&self, givenName: Option<&NSString>) {
            msg_send![self, setGivenName: givenName]
        }
        pub unsafe fn middleName(&self) -> Option<Id<NSString, Shared>> {
            msg_send_id![self, middleName]
        }
        pub unsafe fn setMiddleName(&self, middleName: Option<&NSString>) {
            msg_send![self, setMiddleName: middleName]
        }
        pub unsafe fn familyName(&self) -> Option<Id<NSString, Shared>> {
            msg_send_id![self, familyName]
        }
        pub unsafe fn setFamilyName(&self, familyName: Option<&NSString>) {
            msg_send![self, setFamilyName: familyName]
        }
        pub unsafe fn nameSuffix(&self) -> Option<Id<NSString, Shared>> {
            msg_send_id![self, nameSuffix]
        }
        pub unsafe fn setNameSuffix(&self, nameSuffix: Option<&NSString>) {
            msg_send![self, setNameSuffix: nameSuffix]
        }
        pub unsafe fn nickname(&self) -> Option<Id<NSString, Shared>> {
            msg_send_id![self, nickname]
        }
        pub unsafe fn setNickname(&self, nickname: Option<&NSString>) {
            msg_send![self, setNickname: nickname]
        }
        pub unsafe fn phoneticRepresentation(&self) -> Option<Id<NSPersonNameComponents, Shared>> {
            msg_send_id![self, phoneticRepresentation]
        }
        pub unsafe fn setPhoneticRepresentation(
            &self,
            phoneticRepresentation: Option<&NSPersonNameComponents>,
        ) {
            msg_send![self, setPhoneticRepresentation: phoneticRepresentation]
        }
    }
);
