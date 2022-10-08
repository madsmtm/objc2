use super::__exported::NSArray;
use super::__exported::NSString;
use super::__exported::NSURLCredentialInternal;
use crate::Foundation::generated::NSObject::*;
use crate::Security::generated::Security::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSURLCredential;
    unsafe impl ClassType for NSURLCredential {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSURLCredential {
        pub unsafe fn persistence(&self) -> NSURLCredentialPersistence {
            msg_send![self, persistence]
        }
    }
);
extern_methods!(
    #[doc = "NSInternetPassword"]
    unsafe impl NSURLCredential {
        pub unsafe fn initWithUser_password_persistence(
            &self,
            user: &NSString,
            password: &NSString,
            persistence: NSURLCredentialPersistence,
        ) -> Id<Self, Shared> {
            msg_send_id![
                self,
                initWithUser: user,
                password: password,
                persistence: persistence
            ]
        }
        pub unsafe fn credentialWithUser_password_persistence(
            user: &NSString,
            password: &NSString,
            persistence: NSURLCredentialPersistence,
        ) -> Id<NSURLCredential, Shared> {
            msg_send_id![
                Self::class(),
                credentialWithUser: user,
                password: password,
                persistence: persistence
            ]
        }
        pub unsafe fn user(&self) -> Option<Id<NSString, Shared>> {
            msg_send_id![self, user]
        }
        pub unsafe fn password(&self) -> Option<Id<NSString, Shared>> {
            msg_send_id![self, password]
        }
        pub unsafe fn hasPassword(&self) -> bool {
            msg_send![self, hasPassword]
        }
    }
);
extern_methods!(
    #[doc = "NSClientCertificate"]
    unsafe impl NSURLCredential {
        pub unsafe fn initWithIdentity_certificates_persistence(
            &self,
            identity: SecIdentityRef,
            certArray: Option<&NSArray>,
            persistence: NSURLCredentialPersistence,
        ) -> Id<Self, Shared> {
            msg_send_id![
                self,
                initWithIdentity: identity,
                certificates: certArray,
                persistence: persistence
            ]
        }
        pub unsafe fn credentialWithIdentity_certificates_persistence(
            identity: SecIdentityRef,
            certArray: Option<&NSArray>,
            persistence: NSURLCredentialPersistence,
        ) -> Id<NSURLCredential, Shared> {
            msg_send_id![
                Self::class(),
                credentialWithIdentity: identity,
                certificates: certArray,
                persistence: persistence
            ]
        }
        pub unsafe fn identity(&self) -> SecIdentityRef {
            msg_send![self, identity]
        }
        pub unsafe fn certificates(&self) -> Id<NSArray, Shared> {
            msg_send_id![self, certificates]
        }
    }
);
extern_methods!(
    #[doc = "NSServerTrust"]
    unsafe impl NSURLCredential {
        pub unsafe fn initWithTrust(&self, trust: SecTrustRef) -> Id<Self, Shared> {
            msg_send_id![self, initWithTrust: trust]
        }
        pub unsafe fn credentialForTrust(trust: SecTrustRef) -> Id<NSURLCredential, Shared> {
            msg_send_id![Self::class(), credentialForTrust: trust]
        }
    }
);
