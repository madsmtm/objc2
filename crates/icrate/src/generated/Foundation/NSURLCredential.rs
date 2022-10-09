use super::__exported::NSArray;
use super::__exported::NSString;
use super::__exported::NSURLCredentialInternal;
use crate::Foundation::generated::NSObject::*;
use crate::Security::generated::Security::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSURLCredential;
    unsafe impl ClassType for NSURLCredential {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSURLCredential {
        #[method(persistence)]
        pub unsafe fn persistence(&self) -> NSURLCredentialPersistence;
    }
);
extern_methods!(
    #[doc = "NSInternetPassword"]
    unsafe impl NSURLCredential {
        #[method_id(initWithUser:password:persistence:)]
        pub unsafe fn initWithUser_password_persistence(
            &self,
            user: &NSString,
            password: &NSString,
            persistence: NSURLCredentialPersistence,
        ) -> Id<Self, Shared>;
        #[method_id(credentialWithUser:password:persistence:)]
        pub unsafe fn credentialWithUser_password_persistence(
            user: &NSString,
            password: &NSString,
            persistence: NSURLCredentialPersistence,
        ) -> Id<NSURLCredential, Shared>;
        #[method_id(user)]
        pub unsafe fn user(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(password)]
        pub unsafe fn password(&self) -> Option<Id<NSString, Shared>>;
        #[method(hasPassword)]
        pub unsafe fn hasPassword(&self) -> bool;
    }
);
extern_methods!(
    #[doc = "NSClientCertificate"]
    unsafe impl NSURLCredential {
        #[method_id(initWithIdentity:certificates:persistence:)]
        pub unsafe fn initWithIdentity_certificates_persistence(
            &self,
            identity: SecIdentityRef,
            certArray: Option<&NSArray>,
            persistence: NSURLCredentialPersistence,
        ) -> Id<Self, Shared>;
        #[method_id(credentialWithIdentity:certificates:persistence:)]
        pub unsafe fn credentialWithIdentity_certificates_persistence(
            identity: SecIdentityRef,
            certArray: Option<&NSArray>,
            persistence: NSURLCredentialPersistence,
        ) -> Id<NSURLCredential, Shared>;
        #[method(identity)]
        pub unsafe fn identity(&self) -> SecIdentityRef;
        #[method_id(certificates)]
        pub unsafe fn certificates(&self) -> Id<NSArray, Shared>;
    }
);
extern_methods!(
    #[doc = "NSServerTrust"]
    unsafe impl NSURLCredential {
        #[method_id(initWithTrust:)]
        pub unsafe fn initWithTrust(&self, trust: SecTrustRef) -> Id<Self, Shared>;
        #[method_id(credentialForTrust:)]
        pub unsafe fn credentialForTrust(trust: SecTrustRef) -> Id<NSURLCredential, Shared>;
    }
);
