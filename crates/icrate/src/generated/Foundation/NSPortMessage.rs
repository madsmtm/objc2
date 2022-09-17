extern_class!(
    #[derive(Debug)]
    struct NSPortMessage;
    unsafe impl ClassType for NSPortMessage {
        type Super = NSObject;
    }
);
impl NSPortMessage {
    pub unsafe fn initWithSendPort_receivePort_components(
        &self,
        sendPort: Option<&NSPort>,
        replyPort: Option<&NSPort>,
        components: Option<&NSArray>,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithSendPort: sendPort,
            receivePort: replyPort,
            components: components
        ]
    }
    pub unsafe fn sendBeforeDate(&self, date: &NSDate) -> bool {
        msg_send![self, sendBeforeDate: date]
    }
    pub unsafe fn components(&self) -> Option<Id<NSArray, Shared>> {
        msg_send_id![self, components]
    }
    pub unsafe fn receivePort(&self) -> Option<Id<NSPort, Shared>> {
        msg_send_id![self, receivePort]
    }
    pub unsafe fn sendPort(&self) -> Option<Id<NSPort, Shared>> {
        msg_send_id![self, sendPort]
    }
    pub unsafe fn msgid(&self) -> uint32_t {
        msg_send![self, msgid]
    }
    pub unsafe fn setMsgid(&self, msgid: uint32_t) {
        msg_send![self, setMsgid: msgid]
    }
}
