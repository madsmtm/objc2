extern_class!(
    #[derive(Debug)]
    struct NSProtocolChecker;
    unsafe impl ClassType for NSProtocolChecker {
        type Super = NSProxy;
    }
);
impl NSProtocolChecker {
    pub unsafe fn protocol(&self) -> Id<Protocol, Shared> {
        msg_send_id![self, protocol]
    }
    pub unsafe fn target(&self) -> Option<Id<NSObject, Shared>> {
        msg_send_id![self, target]
    }
}
#[doc = "NSProtocolCheckerCreation"]
impl NSProtocolChecker {
    pub unsafe fn protocolCheckerWithTarget_protocol(
        anObject: &NSObject,
        aProtocol: &Protocol,
    ) -> Id<Self, Shared> {
        msg_send_id![
            Self::class(),
            protocolCheckerWithTarget: anObject,
            protocol: aProtocol
        ]
    }
    pub unsafe fn initWithTarget_protocol(
        &self,
        anObject: &NSObject,
        aProtocol: &Protocol,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithTarget: anObject, protocol: aProtocol]
    }
}
