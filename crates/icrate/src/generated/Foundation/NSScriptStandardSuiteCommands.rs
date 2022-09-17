extern_class!(
    #[derive(Debug)]
    struct NSCloneCommand;
    unsafe impl ClassType for NSCloneCommand {
        type Super = NSScriptCommand;
    }
);
impl NSCloneCommand {
    pub unsafe fn setReceiversSpecifier(&self, receiversRef: Option<&NSScriptObjectSpecifier>) {
        msg_send![self, setReceiversSpecifier: receiversRef]
    }
    pub unsafe fn keySpecifier(&self) -> Id<NSScriptObjectSpecifier, Shared> {
        msg_send_id![self, keySpecifier]
    }
}
extern_class!(
    #[derive(Debug)]
    struct NSCloseCommand;
    unsafe impl ClassType for NSCloseCommand {
        type Super = NSScriptCommand;
    }
);
impl NSCloseCommand {
    pub unsafe fn saveOptions(&self) -> NSSaveOptions {
        msg_send![self, saveOptions]
    }
}
extern_class!(
    #[derive(Debug)]
    struct NSCountCommand;
    unsafe impl ClassType for NSCountCommand {
        type Super = NSScriptCommand;
    }
);
impl NSCountCommand {}
extern_class!(
    #[derive(Debug)]
    struct NSCreateCommand;
    unsafe impl ClassType for NSCreateCommand {
        type Super = NSScriptCommand;
    }
);
impl NSCreateCommand {
    pub unsafe fn createClassDescription(&self) -> Id<NSScriptClassDescription, Shared> {
        msg_send_id![self, createClassDescription]
    }
    pub unsafe fn resolvedKeyDictionary(&self) -> TodoGenerics {
        msg_send![self, resolvedKeyDictionary]
    }
}
extern_class!(
    #[derive(Debug)]
    struct NSDeleteCommand;
    unsafe impl ClassType for NSDeleteCommand {
        type Super = NSScriptCommand;
    }
);
impl NSDeleteCommand {
    pub unsafe fn setReceiversSpecifier(&self, receiversRef: Option<&NSScriptObjectSpecifier>) {
        msg_send![self, setReceiversSpecifier: receiversRef]
    }
    pub unsafe fn keySpecifier(&self) -> Id<NSScriptObjectSpecifier, Shared> {
        msg_send_id![self, keySpecifier]
    }
}
extern_class!(
    #[derive(Debug)]
    struct NSExistsCommand;
    unsafe impl ClassType for NSExistsCommand {
        type Super = NSScriptCommand;
    }
);
impl NSExistsCommand {}
extern_class!(
    #[derive(Debug)]
    struct NSGetCommand;
    unsafe impl ClassType for NSGetCommand {
        type Super = NSScriptCommand;
    }
);
impl NSGetCommand {}
extern_class!(
    #[derive(Debug)]
    struct NSMoveCommand;
    unsafe impl ClassType for NSMoveCommand {
        type Super = NSScriptCommand;
    }
);
impl NSMoveCommand {
    pub unsafe fn setReceiversSpecifier(&self, receiversRef: Option<&NSScriptObjectSpecifier>) {
        msg_send![self, setReceiversSpecifier: receiversRef]
    }
    pub unsafe fn keySpecifier(&self) -> Id<NSScriptObjectSpecifier, Shared> {
        msg_send_id![self, keySpecifier]
    }
}
extern_class!(
    #[derive(Debug)]
    struct NSQuitCommand;
    unsafe impl ClassType for NSQuitCommand {
        type Super = NSScriptCommand;
    }
);
impl NSQuitCommand {
    pub unsafe fn saveOptions(&self) -> NSSaveOptions {
        msg_send![self, saveOptions]
    }
}
extern_class!(
    #[derive(Debug)]
    struct NSSetCommand;
    unsafe impl ClassType for NSSetCommand {
        type Super = NSScriptCommand;
    }
);
impl NSSetCommand {
    pub unsafe fn setReceiversSpecifier(&self, receiversRef: Option<&NSScriptObjectSpecifier>) {
        msg_send![self, setReceiversSpecifier: receiversRef]
    }
    pub unsafe fn keySpecifier(&self) -> Id<NSScriptObjectSpecifier, Shared> {
        msg_send_id![self, keySpecifier]
    }
}
