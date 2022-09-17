extern_class!(
    #[derive(Debug)]
    struct NSAutoreleasePool;
    unsafe impl ClassType for NSAutoreleasePool {
        type Super = NSObject;
    }
);
impl NSAutoreleasePool {
    pub unsafe fn addObject(anObject: &Object) {
        msg_send![Self::class(), addObject: anObject]
    }
    pub unsafe fn addObject(&self, anObject: &Object) {
        msg_send![self, addObject: anObject]
    }
    pub unsafe fn drain(&self) {
        msg_send![self, drain]
    }
}
