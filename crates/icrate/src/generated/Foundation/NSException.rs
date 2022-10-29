#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSException;
    unsafe impl ClassType for NSException {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSException {
        #[method_id(exceptionWithName:reason:userInfo:)]
        pub unsafe fn exceptionWithName_reason_userInfo(
            name: &NSExceptionName,
            reason: Option<&NSString>,
            userInfo: Option<&NSDictionary>,
        ) -> Id<NSException, Shared>;
        #[method_id(initWithName:reason:userInfo:)]
        pub unsafe fn initWithName_reason_userInfo(
            &self,
            aName: &NSExceptionName,
            aReason: Option<&NSString>,
            aUserInfo: Option<&NSDictionary>,
        ) -> Id<Self, Shared>;
        #[method_id(name)]
        pub unsafe fn name(&self) -> Id<NSExceptionName, Shared>;
        #[method_id(reason)]
        pub unsafe fn reason(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary, Shared>>;
        #[method_id(callStackReturnAddresses)]
        pub unsafe fn callStackReturnAddresses(&self) -> Id<NSArray<NSNumber>, Shared>;
        #[method_id(callStackSymbols)]
        pub unsafe fn callStackSymbols(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(raise)]
        pub unsafe fn raise(&self);
    }
);
extern_methods!(
    #[doc = "NSExceptionRaisingConveniences"]
    unsafe impl NSException {
        #[method(raise:format:arguments:)]
        pub unsafe fn raise_format_arguments(
            name: &NSExceptionName,
            format: &NSString,
            argList: va_list,
        );
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
        #[method_id(currentHandler)]
        pub unsafe fn currentHandler() -> Id<NSAssertionHandler, Shared>;
    }
);
