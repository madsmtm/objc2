extern_class!(
    #[derive(Debug)]
    struct NSPointerFunctions;
    unsafe impl ClassType for NSPointerFunctions {
        type Super = NSObject;
    }
);
impl NSPointerFunctions {
    pub unsafe fn initWithOptions(&self, options: NSPointerFunctionsOptions) -> Id<Self, Shared> {
        msg_send_id![self, initWithOptions: options]
    }
    pub unsafe fn pointerFunctionsWithOptions(
        options: NSPointerFunctionsOptions,
    ) -> Id<NSPointerFunctions, Shared> {
        msg_send_id![Self::class(), pointerFunctionsWithOptions: options]
    }
    pub unsafe fn hashFunction(&self) -> *mut TodoFunction {
        msg_send![self, hashFunction]
    }
    pub unsafe fn setHashFunction(&self, hashFunction: *mut TodoFunction) {
        msg_send![self, setHashFunction: hashFunction]
    }
    pub unsafe fn isEqualFunction(&self) -> *mut TodoFunction {
        msg_send![self, isEqualFunction]
    }
    pub unsafe fn setIsEqualFunction(&self, isEqualFunction: *mut TodoFunction) {
        msg_send![self, setIsEqualFunction: isEqualFunction]
    }
    pub unsafe fn sizeFunction(&self) -> *mut TodoFunction {
        msg_send![self, sizeFunction]
    }
    pub unsafe fn setSizeFunction(&self, sizeFunction: *mut TodoFunction) {
        msg_send![self, setSizeFunction: sizeFunction]
    }
    pub unsafe fn descriptionFunction(&self) -> *mut TodoFunction {
        msg_send![self, descriptionFunction]
    }
    pub unsafe fn setDescriptionFunction(&self, descriptionFunction: *mut TodoFunction) {
        msg_send![self, setDescriptionFunction: descriptionFunction]
    }
    pub unsafe fn relinquishFunction(&self) -> *mut TodoFunction {
        msg_send![self, relinquishFunction]
    }
    pub unsafe fn setRelinquishFunction(&self, relinquishFunction: *mut TodoFunction) {
        msg_send![self, setRelinquishFunction: relinquishFunction]
    }
    pub unsafe fn acquireFunction(&self) -> *mut TodoFunction {
        msg_send![self, acquireFunction]
    }
    pub unsafe fn setAcquireFunction(&self, acquireFunction: *mut TodoFunction) {
        msg_send![self, setAcquireFunction: acquireFunction]
    }
    pub unsafe fn usesStrongWriteBarrier(&self) -> bool {
        msg_send![self, usesStrongWriteBarrier]
    }
    pub unsafe fn setUsesStrongWriteBarrier(&self, usesStrongWriteBarrier: bool) {
        msg_send![self, setUsesStrongWriteBarrier: usesStrongWriteBarrier]
    }
    pub unsafe fn usesWeakReadAndWriteBarriers(&self) -> bool {
        msg_send![self, usesWeakReadAndWriteBarriers]
    }
    pub unsafe fn setUsesWeakReadAndWriteBarriers(&self, usesWeakReadAndWriteBarriers: bool) {
        msg_send![
            self,
            setUsesWeakReadAndWriteBarriers: usesWeakReadAndWriteBarriers
        ]
    }
}
