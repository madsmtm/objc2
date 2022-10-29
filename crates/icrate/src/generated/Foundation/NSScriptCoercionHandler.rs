#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSScriptCoercionHandler;
    unsafe impl ClassType for NSScriptCoercionHandler {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSScriptCoercionHandler {
        #[method_id(sharedCoercionHandler)]
        pub unsafe fn sharedCoercionHandler() -> Id<NSScriptCoercionHandler, Shared>;
        #[method_id(coerceValue:toClass:)]
        pub unsafe fn coerceValue_toClass(
            &self,
            value: &Object,
            toClass: &Class,
        ) -> Option<Id<Object, Shared>>;
        #[method(registerCoercer:selector:toConvertFromClass:toClass:)]
        pub unsafe fn registerCoercer_selector_toConvertFromClass_toClass(
            &self,
            coercer: &Object,
            selector: Sel,
            fromClass: &Class,
            toClass: &Class,
        );
    }
);
