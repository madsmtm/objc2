#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSProtocolChecker;
    unsafe impl ClassType for NSProtocolChecker {
        type Super = NSProxy;
    }
);
extern_methods!(
    unsafe impl NSProtocolChecker {
        #[method_id(protocol)]
        pub unsafe fn protocol(&self) -> Id<Protocol, Shared>;
        #[method_id(target)]
        pub unsafe fn target(&self) -> Option<Id<NSObject, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSProtocolCheckerCreation"]
    unsafe impl NSProtocolChecker {
        #[method_id(protocolCheckerWithTarget:protocol:)]
        pub unsafe fn protocolCheckerWithTarget_protocol(
            anObject: &NSObject,
            aProtocol: &Protocol,
        ) -> Id<Self, Shared>;
        #[method_id(initWithTarget:protocol:)]
        pub unsafe fn initWithTarget_protocol(
            &self,
            anObject: &NSObject,
            aProtocol: &Protocol,
        ) -> Id<Self, Shared>;
    }
);
