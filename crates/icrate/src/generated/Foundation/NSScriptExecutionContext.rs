#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSScriptExecutionContext;
    unsafe impl ClassType for NSScriptExecutionContext {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSScriptExecutionContext {
        #[method_id(sharedScriptExecutionContext)]
        pub unsafe fn sharedScriptExecutionContext() -> Id<NSScriptExecutionContext, Shared>;
        #[method_id(topLevelObject)]
        pub unsafe fn topLevelObject(&self) -> Option<Id<Object, Shared>>;
        #[method(setTopLevelObject:)]
        pub unsafe fn setTopLevelObject(&self, topLevelObject: Option<&Object>);
        #[method_id(objectBeingTested)]
        pub unsafe fn objectBeingTested(&self) -> Option<Id<Object, Shared>>;
        #[method(setObjectBeingTested:)]
        pub unsafe fn setObjectBeingTested(&self, objectBeingTested: Option<&Object>);
        #[method_id(rangeContainerObject)]
        pub unsafe fn rangeContainerObject(&self) -> Option<Id<Object, Shared>>;
        #[method(setRangeContainerObject:)]
        pub unsafe fn setRangeContainerObject(&self, rangeContainerObject: Option<&Object>);
    }
);
