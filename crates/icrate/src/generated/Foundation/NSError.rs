use super::__exported::NSArray;
use super::__exported::NSDictionary;
use super::__exported::NSString;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSErrorDomain = NSString;
pub type NSErrorUserInfoKey = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSError;
    unsafe impl ClassType for NSError {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSError {
        # [method_id (initWithDomain : code : userInfo :)]
        pub unsafe fn initWithDomain_code_userInfo(
            &self,
            domain: &NSErrorDomain,
            code: NSInteger,
            dict: Option<&NSDictionary<NSErrorUserInfoKey, Object>>,
        ) -> Id<Self, Shared>;
        # [method_id (errorWithDomain : code : userInfo :)]
        pub unsafe fn errorWithDomain_code_userInfo(
            domain: &NSErrorDomain,
            code: NSInteger,
            dict: Option<&NSDictionary<NSErrorUserInfoKey, Object>>,
        ) -> Id<Self, Shared>;
        #[method_id(domain)]
        pub unsafe fn domain(&self) -> Id<NSErrorDomain, Shared>;
        #[method(code)]
        pub unsafe fn code(&self) -> NSInteger;
        #[method_id(userInfo)]
        pub unsafe fn userInfo(&self) -> Id<NSDictionary<NSErrorUserInfoKey, Object>, Shared>;
        #[method_id(localizedDescription)]
        pub unsafe fn localizedDescription(&self) -> Id<NSString, Shared>;
        #[method_id(localizedFailureReason)]
        pub unsafe fn localizedFailureReason(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(localizedRecoverySuggestion)]
        pub unsafe fn localizedRecoverySuggestion(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(localizedRecoveryOptions)]
        pub unsafe fn localizedRecoveryOptions(&self) -> Option<Id<NSArray<NSString>, Shared>>;
        #[method_id(recoveryAttempter)]
        pub unsafe fn recoveryAttempter(&self) -> Option<Id<Object, Shared>>;
        #[method_id(helpAnchor)]
        pub unsafe fn helpAnchor(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(underlyingErrors)]
        pub unsafe fn underlyingErrors(&self) -> Id<NSArray<NSError>, Shared>;
        # [method (setUserInfoValueProviderForDomain : provider :)]
        pub unsafe fn setUserInfoValueProviderForDomain_provider(
            errorDomain: &NSErrorDomain,
            provider: TodoBlock,
        );
        # [method (userInfoValueProviderForDomain :)]
        pub unsafe fn userInfoValueProviderForDomain(errorDomain: &NSErrorDomain) -> TodoBlock;
    }
);
extern_methods!(
    #[doc = "NSErrorRecoveryAttempting"]
    unsafe impl NSObject {
        # [method (attemptRecoveryFromError : optionIndex : delegate : didRecoverSelector : contextInfo :)]
        pub unsafe fn attemptRecoveryFromError_optionIndex_delegate_didRecoverSelector_contextInfo(
            &self,
            error: &NSError,
            recoveryOptionIndex: NSUInteger,
            delegate: Option<&Object>,
            didRecoverSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );
        # [method (attemptRecoveryFromError : optionIndex :)]
        pub unsafe fn attemptRecoveryFromError_optionIndex(
            &self,
            error: &NSError,
            recoveryOptionIndex: NSUInteger,
        ) -> bool;
    }
);
