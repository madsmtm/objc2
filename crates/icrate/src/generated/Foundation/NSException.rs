#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSException;
    unsafe impl ClassType for NSException {
        type Super = NSObject;
    }
);
impl NSException {
    pub unsafe fn exceptionWithName_reason_userInfo(
        name: NSExceptionName,
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
        aName: NSExceptionName,
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
    pub unsafe fn raise(&self) {
        msg_send![self, raise]
    }
    pub unsafe fn name(&self) -> NSExceptionName {
        msg_send![self, name]
    }
    pub unsafe fn reason(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, reason]
    }
    pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary, Shared>> {
        msg_send_id![self, userInfo]
    }
    pub unsafe fn callStackReturnAddresses(&self) -> TodoGenerics {
        msg_send![self, callStackReturnAddresses]
    }
    pub unsafe fn callStackSymbols(&self) -> TodoGenerics {
        msg_send![self, callStackSymbols]
    }
}
#[doc = "NSExceptionRaisingConveniences"]
impl NSException {
    pub unsafe fn raise_format_arguments(
        name: NSExceptionName,
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
extern_class!(
    #[derive(Debug)]
    pub struct NSAssertionHandler;
    unsafe impl ClassType for NSAssertionHandler {
        type Super = NSObject;
    }
);
impl NSAssertionHandler {
    pub unsafe fn currentHandler() -> Id<NSAssertionHandler, Shared> {
        msg_send_id![Self::class(), currentHandler]
    }
}
