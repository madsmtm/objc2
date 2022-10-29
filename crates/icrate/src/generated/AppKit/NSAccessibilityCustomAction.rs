#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSAccessibilityCustomAction;
    unsafe impl ClassType for NSAccessibilityCustomAction {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSAccessibilityCustomAction {
        #[method_id(initWithName:handler:)]
        pub unsafe fn initWithName_handler(
            &self,
            name: &NSString,
            handler: TodoBlock,
        ) -> Id<Self, Shared>;
        #[method_id(initWithName:target:selector:)]
        pub unsafe fn initWithName_target_selector(
            &self,
            name: &NSString,
            target: &NSObject,
            selector: Sel,
        ) -> Id<Self, Shared>;
        #[method_id(name)]
        pub unsafe fn name(&self) -> Id<NSString, Shared>;
        #[method(setName:)]
        pub unsafe fn setName(&self, name: &NSString);
        #[method(handler)]
        pub unsafe fn handler(&self) -> TodoBlock;
        #[method(setHandler:)]
        pub unsafe fn setHandler(&self, handler: TodoBlock);
        #[method_id(target)]
        pub unsafe fn target(&self) -> Option<Id<NSObject, Shared>>;
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&NSObject>);
        #[method(selector)]
        pub unsafe fn selector(&self) -> Option<Sel>;
        #[method(setSelector:)]
        pub unsafe fn setSelector(&self, selector: Option<Sel>);
    }
);
