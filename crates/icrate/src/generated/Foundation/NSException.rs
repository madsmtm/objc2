use super::__exported::NSArray;
use super::__exported::NSDictionary;
use super::__exported::NSNumber;
use super::__exported::NSString;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSString::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSException;
    unsafe impl ClassType for NSException {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSException {
        pub unsafe fn exceptionWithName_reason_userInfo(
            name: &NSExceptionName,
            reason: Option<&NSString>,
            userInfo: Option<&NSDictionary>,
        ) -> Id<NSException, Shared> {
            msg_send_id![
                Self::class(),
                exceptionWithName: name,
                reason: reason,
                userInfo: userInfo
            ]
        }
        pub unsafe fn initWithName_reason_userInfo(
            &self,
            aName: &NSExceptionName,
            aReason: Option<&NSString>,
            aUserInfo: Option<&NSDictionary>,
        ) -> Id<Self, Shared> {
            msg_send_id![
                self,
                initWithName: aName,
                reason: aReason,
                userInfo: aUserInfo
            ]
        }
        pub unsafe fn name(&self) -> Id<NSExceptionName, Shared> {
            msg_send_id![self, name]
        }
        pub unsafe fn reason(&self) -> Option<Id<NSString, Shared>> {
            msg_send_id![self, reason]
        }
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary, Shared>> {
            msg_send_id![self, userInfo]
        }
        pub unsafe fn callStackReturnAddresses(&self) -> Id<NSArray<NSNumber>, Shared> {
            msg_send_id![self, callStackReturnAddresses]
        }
        pub unsafe fn callStackSymbols(&self) -> Id<NSArray<NSString>, Shared> {
            msg_send_id![self, callStackSymbols]
        }
        pub unsafe fn raise(&self) {
            msg_send![self, raise]
        }
    }
);
extern_methods!(
    #[doc = "NSExceptionRaisingConveniences"]
    unsafe impl NSException {
        pub unsafe fn raise_format_arguments(
            name: &NSExceptionName,
            format: &NSString,
            argList: va_list,
        ) {
            msg_send![
                Self::class(),
                raise: name,
                format: format,
                arguments: argList
            ]
        }
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSAssertionHandler;
    unsafe impl ClassType for NSAssertionHandler {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSAssertionHandler {
        pub unsafe fn currentHandler() -> Id<NSAssertionHandler, Shared> {
            msg_send_id![Self::class(), currentHandler]
        }
    }
);
