extern_class!(
    #[derive(Debug)]
    struct NSEnumerator;
    unsafe impl ClassType for NSEnumerator {
        type Super = NSObject;
    }
);
impl NSEnumerator {
    pub unsafe fn nextObject(&self) -> ObjectType {
        msg_send![self, nextObject]
    }
}
#[doc = "NSExtendedEnumerator"]
impl NSEnumerator {
    pub unsafe fn allObjects(&self) -> TodoGenerics {
        msg_send![self, allObjects]
    }
}
