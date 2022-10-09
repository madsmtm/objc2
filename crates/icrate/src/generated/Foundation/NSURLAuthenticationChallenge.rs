use super::__exported::NSError;
use super::__exported::NSURLCredential;
use super::__exported::NSURLProtectionSpace;
use super::__exported::NSURLResponse;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSURLAuthenticationChallengeSender = NSObject;
use super::__exported::NSURLAuthenticationChallengeInternal;
extern_class!(
    #[derive(Debug)]
    pub struct NSURLAuthenticationChallenge;
    unsafe impl ClassType for NSURLAuthenticationChallenge {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSURLAuthenticationChallenge {
        #[method_id(initWithProtectionSpace:proposedCredential:previousFailureCount:failureResponse:error:sender:)]
        pub unsafe fn initWithProtectionSpace_proposedCredential_previousFailureCount_failureResponse_error_sender(
            &self,
            space: &NSURLProtectionSpace,
            credential: Option<&NSURLCredential>,
            previousFailureCount: NSInteger,
            response: Option<&NSURLResponse>,
            error: Option<&NSError>,
            sender: &NSURLAuthenticationChallengeSender,
        ) -> Id<Self, Shared>;
        #[method_id(initWithAuthenticationChallenge:sender:)]
        pub unsafe fn initWithAuthenticationChallenge_sender(
            &self,
            challenge: &NSURLAuthenticationChallenge,
            sender: &NSURLAuthenticationChallengeSender,
        ) -> Id<Self, Shared>;
        #[method_id(protectionSpace)]
        pub unsafe fn protectionSpace(&self) -> Id<NSURLProtectionSpace, Shared>;
        #[method_id(proposedCredential)]
        pub unsafe fn proposedCredential(&self) -> Option<Id<NSURLCredential, Shared>>;
        #[method(previousFailureCount)]
        pub unsafe fn previousFailureCount(&self) -> NSInteger;
        #[method_id(failureResponse)]
        pub unsafe fn failureResponse(&self) -> Option<Id<NSURLResponse, Shared>>;
        #[method_id(error)]
        pub unsafe fn error(&self) -> Option<Id<NSError, Shared>>;
        #[method_id(sender)]
        pub unsafe fn sender(&self) -> Option<Id<NSURLAuthenticationChallengeSender, Shared>>;
    }
);
