#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSAppleScript;
    unsafe impl ClassType for NSAppleScript {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSAppleScript {
        #[method_id(initWithContentsOfURL:error:)]
        pub unsafe fn initWithContentsOfURL_error(
            &self,
            url: &NSURL,
            errorInfo: Option<&mut Option<Id<NSDictionary<NSString, Object>, Shared>>>,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithSource:)]
        pub unsafe fn initWithSource(&self, source: &NSString) -> Option<Id<Self, Shared>>;
        #[method_id(source)]
        pub unsafe fn source(&self) -> Option<Id<NSString, Shared>>;
        #[method(isCompiled)]
        pub unsafe fn isCompiled(&self) -> bool;
        #[method(compileAndReturnError:)]
        pub unsafe fn compileAndReturnError(
            &self,
            errorInfo: Option<&mut Option<Id<NSDictionary<NSString, Object>, Shared>>>,
        ) -> bool;
        #[method_id(executeAndReturnError:)]
        pub unsafe fn executeAndReturnError(
            &self,
            errorInfo: Option<&mut Option<Id<NSDictionary<NSString, Object>, Shared>>>,
        ) -> Id<NSAppleEventDescriptor, Shared>;
        #[method_id(executeAppleEvent:error:)]
        pub unsafe fn executeAppleEvent_error(
            &self,
            event: &NSAppleEventDescriptor,
            errorInfo: Option<&mut Option<Id<NSDictionary<NSString, Object>, Shared>>>,
        ) -> Id<NSAppleEventDescriptor, Shared>;
    }
);
