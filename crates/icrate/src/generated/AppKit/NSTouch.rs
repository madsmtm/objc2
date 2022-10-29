#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTouch;
    unsafe impl ClassType for NSTouch {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSTouch {
        #[method_id(identity)]
        pub unsafe fn identity(&self) -> Id<TodoProtocols, Shared>;
        #[method(phase)]
        pub unsafe fn phase(&self) -> NSTouchPhase;
        #[method(normalizedPosition)]
        pub unsafe fn normalizedPosition(&self) -> NSPoint;
        #[method(isResting)]
        pub unsafe fn isResting(&self) -> bool;
        #[method_id(device)]
        pub unsafe fn device(&self) -> Option<Id<Object, Shared>>;
        #[method(deviceSize)]
        pub unsafe fn deviceSize(&self) -> NSSize;
    }
);
extern_methods!(
    #[doc = "NSTouchBar"]
    unsafe impl NSTouch {
        #[method(type)]
        pub unsafe fn type_(&self) -> NSTouchType;
        #[method(locationInView:)]
        pub unsafe fn locationInView(&self, view: Option<&NSView>) -> NSPoint;
        #[method(previousLocationInView:)]
        pub unsafe fn previousLocationInView(&self, view: Option<&NSView>) -> NSPoint;
    }
);
