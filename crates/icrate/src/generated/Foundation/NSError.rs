use super::__exported::NSArray;
use super::__exported::NSDictionary;
use super::__exported::NSString;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
pub type NSErrorDomain = NSString;
pub type NSErrorUserInfoKey = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSError;
    unsafe impl ClassType for NSError {
        type Super = NSObject;
    }
);
impl NSError {
    pub unsafe fn initWithDomain_code_userInfo(
        &self,
        domain: &NSErrorDomain,
        code: NSInteger,
        dict: TodoGenerics,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithDomain: domain, code: code, userInfo: dict]
    }
    pub unsafe fn errorWithDomain_code_userInfo(
        domain: &NSErrorDomain,
        code: NSInteger,
        dict: TodoGenerics,
    ) -> Id<Self, Shared> {
        msg_send_id![
            Self::class(),
            errorWithDomain: domain,
            code: code,
            userInfo: dict
        ]
    }
    pub unsafe fn setUserInfoValueProviderForDomain_provider(
        errorDomain: &NSErrorDomain,
        provider: TodoBlock,
    ) {
        msg_send![
            Self::class(),
            setUserInfoValueProviderForDomain: errorDomain,
            provider: provider
        ]
    }
    pub unsafe fn userInfoValueProviderForDomain(errorDomain: &NSErrorDomain) -> TodoBlock {
        msg_send![Self::class(), userInfoValueProviderForDomain: errorDomain]
    }
    pub unsafe fn domain(&self) -> Id<NSErrorDomain, Shared> {
        msg_send_id![self, domain]
    }
    pub unsafe fn code(&self) -> NSInteger {
        msg_send![self, code]
    }
    pub unsafe fn userInfo(&self) -> TodoGenerics {
        msg_send![self, userInfo]
    }
    pub unsafe fn localizedDescription(&self) -> Id<NSString, Shared> {
        msg_send_id![self, localizedDescription]
    }
    pub unsafe fn localizedFailureReason(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, localizedFailureReason]
    }
    pub unsafe fn localizedRecoverySuggestion(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, localizedRecoverySuggestion]
    }
    pub unsafe fn localizedRecoveryOptions(&self) -> TodoGenerics {
        msg_send![self, localizedRecoveryOptions]
    }
    pub unsafe fn recoveryAttempter(&self) -> Option<Id<Object, Shared>> {
        msg_send_id![self, recoveryAttempter]
    }
    pub unsafe fn helpAnchor(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, helpAnchor]
    }
    pub unsafe fn underlyingErrors(&self) -> TodoGenerics {
        msg_send![self, underlyingErrors]
    }
}
#[doc = "NSErrorRecoveryAttempting"]
impl NSObject {
    pub unsafe fn attemptRecoveryFromError_optionIndex_delegate_didRecoverSelector_contextInfo(
        &self,
        error: &NSError,
        recoveryOptionIndex: NSUInteger,
        delegate: Option<&Object>,
        didRecoverSelector: Option<Sel>,
        contextInfo: *mut c_void,
    ) {
        msg_send![
            self,
            attemptRecoveryFromError: error,
            optionIndex: recoveryOptionIndex,
            delegate: delegate,
            didRecoverSelector: didRecoverSelector,
            contextInfo: contextInfo
        ]
    }
    pub unsafe fn attemptRecoveryFromError_optionIndex(
        &self,
        error: &NSError,
        recoveryOptionIndex: NSUInteger,
    ) -> bool {
        msg_send![
            self,
            attemptRecoveryFromError: error,
            optionIndex: recoveryOptionIndex
        ]
    }
}
