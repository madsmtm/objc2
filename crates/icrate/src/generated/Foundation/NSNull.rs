extern_class!(
    #[derive(Debug)]
    struct NSNull;
    unsafe impl ClassType for NSNull {
        type Super = NSObject;
    }
);
impl NSNull {
    pub unsafe fn null() -> Id<NSNull, Shared> {
        msg_send_id![Self::class(), null]
    }
}
