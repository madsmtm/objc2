#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSCloneCommand;
    unsafe impl ClassType for NSCloneCommand {
        type Super = NSScriptCommand;
    }
);
extern_methods!(
    unsafe impl NSCloneCommand {
        #[method(setReceiversSpecifier:)]
        pub unsafe fn setReceiversSpecifier(&self, receiversRef: Option<&NSScriptObjectSpecifier>);
        #[method_id(keySpecifier)]
        pub unsafe fn keySpecifier(&self) -> Id<NSScriptObjectSpecifier, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSCloseCommand;
    unsafe impl ClassType for NSCloseCommand {
        type Super = NSScriptCommand;
    }
);
extern_methods!(
    unsafe impl NSCloseCommand {
        #[method(saveOptions)]
        pub unsafe fn saveOptions(&self) -> NSSaveOptions;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSCountCommand;
    unsafe impl ClassType for NSCountCommand {
        type Super = NSScriptCommand;
    }
);
extern_methods!(
    unsafe impl NSCountCommand {}
);
extern_class!(
    #[derive(Debug)]
    pub struct NSCreateCommand;
    unsafe impl ClassType for NSCreateCommand {
        type Super = NSScriptCommand;
    }
);
extern_methods!(
    unsafe impl NSCreateCommand {
        #[method_id(createClassDescription)]
        pub unsafe fn createClassDescription(&self) -> Id<NSScriptClassDescription, Shared>;
        #[method_id(resolvedKeyDictionary)]
        pub unsafe fn resolvedKeyDictionary(&self) -> Id<NSDictionary<NSString, Object>, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSDeleteCommand;
    unsafe impl ClassType for NSDeleteCommand {
        type Super = NSScriptCommand;
    }
);
extern_methods!(
    unsafe impl NSDeleteCommand {
        #[method(setReceiversSpecifier:)]
        pub unsafe fn setReceiversSpecifier(&self, receiversRef: Option<&NSScriptObjectSpecifier>);
        #[method_id(keySpecifier)]
        pub unsafe fn keySpecifier(&self) -> Id<NSScriptObjectSpecifier, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSExistsCommand;
    unsafe impl ClassType for NSExistsCommand {
        type Super = NSScriptCommand;
    }
);
extern_methods!(
    unsafe impl NSExistsCommand {}
);
extern_class!(
    #[derive(Debug)]
    pub struct NSGetCommand;
    unsafe impl ClassType for NSGetCommand {
        type Super = NSScriptCommand;
    }
);
extern_methods!(
    unsafe impl NSGetCommand {}
);
extern_class!(
    #[derive(Debug)]
    pub struct NSMoveCommand;
    unsafe impl ClassType for NSMoveCommand {
        type Super = NSScriptCommand;
    }
);
extern_methods!(
    unsafe impl NSMoveCommand {
        #[method(setReceiversSpecifier:)]
        pub unsafe fn setReceiversSpecifier(&self, receiversRef: Option<&NSScriptObjectSpecifier>);
        #[method_id(keySpecifier)]
        pub unsafe fn keySpecifier(&self) -> Id<NSScriptObjectSpecifier, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSQuitCommand;
    unsafe impl ClassType for NSQuitCommand {
        type Super = NSScriptCommand;
    }
);
extern_methods!(
    unsafe impl NSQuitCommand {
        #[method(saveOptions)]
        pub unsafe fn saveOptions(&self) -> NSSaveOptions;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSSetCommand;
    unsafe impl ClassType for NSSetCommand {
        type Super = NSScriptCommand;
    }
);
extern_methods!(
    unsafe impl NSSetCommand {
        #[method(setReceiversSpecifier:)]
        pub unsafe fn setReceiversSpecifier(&self, receiversRef: Option<&NSScriptObjectSpecifier>);
        #[method_id(keySpecifier)]
        pub unsafe fn keySpecifier(&self) -> Id<NSScriptObjectSpecifier, Shared>;
    }
);
