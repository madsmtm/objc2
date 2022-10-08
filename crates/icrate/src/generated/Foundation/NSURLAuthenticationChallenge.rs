use super::__exported::NSError;
use super::__exported::NSURLCredential;
use super::__exported::NSURLProtectionSpace;
use super::__exported::NSURLResponse;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, msg_send, msg_send_id, ClassType};
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
        pub unsafe fn initWithProtectionSpace_proposedCredential_previousFailureCount_failureResponse_error_sender(
            &self,
            space: &NSURLProtectionSpace,
            credential: Option<&NSURLCredential>,
            previousFailureCount: NSInteger,
            response: Option<&NSURLResponse>,
            error: Option<&NSError>,
            sender: &NSURLAuthenticationChallengeSender,
        ) -> Id<Self, Shared> {
            msg_send_id![
                self,
                initWithProtectionSpace: space,
                proposedCredential: credential,
                previousFailureCount: previousFailureCount,
                failureResponse: response,
                error: error,
                sender: sender
            ]
        }
        pub unsafe fn initWithAuthenticationChallenge_sender(
            &self,
            challenge: &NSURLAuthenticationChallenge,
            sender: &NSURLAuthenticationChallengeSender,
        ) -> Id<Self, Shared> {
            msg_send_id![
                self,
                initWithAuthenticationChallenge: challenge,
                sender: sender
            ]
        }
        pub unsafe fn protectionSpace(&self) -> Id<NSURLProtectionSpace, Shared> {
            msg_send_id![self, protectionSpace]
        }
        pub unsafe fn proposedCredential(&self) -> Option<Id<NSURLCredential, Shared>> {
            msg_send_id![self, proposedCredential]
        }
        pub unsafe fn previousFailureCount(&self) -> NSInteger {
            msg_send![self, previousFailureCount]
        }
        pub unsafe fn failureResponse(&self) -> Option<Id<NSURLResponse, Shared>> {
            msg_send_id![self, failureResponse]
        }
        pub unsafe fn error(&self) -> Option<Id<NSError, Shared>> {
            msg_send_id![self, error]
        }
        pub unsafe fn sender(&self) -> Option<Id<NSURLAuthenticationChallengeSender, Shared>> {
            msg_send_id![self, sender]
        }
    }
);
