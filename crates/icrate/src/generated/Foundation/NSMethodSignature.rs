extern_class!(
    #[derive(Debug)]
    struct NSMethodSignature;
    unsafe impl ClassType for NSMethodSignature {
        type Super = NSObject;
    }
);
impl NSMethodSignature {
    pub unsafe fn signatureWithObjCTypes(
        types: NonNull<c_char>,
    ) -> Option<Id<NSMethodSignature, Shared>> {
        msg_send_id![Self::class(), signatureWithObjCTypes: types]
    }
    pub unsafe fn getArgumentTypeAtIndex(&self, idx: NSUInteger) -> NonNull<c_char> {
        msg_send![self, getArgumentTypeAtIndex: idx]
    }
    pub unsafe fn isOneway(&self) -> bool {
        msg_send![self, isOneway]
    }
    pub unsafe fn numberOfArguments(&self) -> NSUInteger {
        msg_send![self, numberOfArguments]
    }
    pub unsafe fn frameLength(&self) -> NSUInteger {
        msg_send![self, frameLength]
    }
    pub unsafe fn methodReturnType(&self) -> NonNull<c_char> {
        msg_send![self, methodReturnType]
    }
    pub unsafe fn methodReturnLength(&self) -> NSUInteger {
        msg_send![self, methodReturnLength]
    }
}
