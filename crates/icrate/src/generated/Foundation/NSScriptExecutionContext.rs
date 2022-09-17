extern_class!(
    #[derive(Debug)]
    struct NSScriptExecutionContext;
    unsafe impl ClassType for NSScriptExecutionContext {
        type Super = NSObject;
    }
);
impl NSScriptExecutionContext {
    pub unsafe fn sharedScriptExecutionContext() -> Id<NSScriptExecutionContext, Shared> {
        msg_send_id![Self::class(), sharedScriptExecutionContext]
    }
    pub unsafe fn topLevelObject(&self) -> Option<Id<Object, Shared>> {
        msg_send_id![self, topLevelObject]
    }
    pub unsafe fn setTopLevelObject(&self, topLevelObject: Option<&Object>) {
        msg_send![self, setTopLevelObject: topLevelObject]
    }
    pub unsafe fn objectBeingTested(&self) -> Option<Id<Object, Shared>> {
        msg_send_id![self, objectBeingTested]
    }
    pub unsafe fn setObjectBeingTested(&self, objectBeingTested: Option<&Object>) {
        msg_send![self, setObjectBeingTested: objectBeingTested]
    }
    pub unsafe fn rangeContainerObject(&self) -> Option<Id<Object, Shared>> {
        msg_send_id![self, rangeContainerObject]
    }
    pub unsafe fn setRangeContainerObject(&self, rangeContainerObject: Option<&Object>) {
        msg_send![self, setRangeContainerObject: rangeContainerObject]
    }
}
