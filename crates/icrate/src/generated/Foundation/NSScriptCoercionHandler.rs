#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    struct NSScriptCoercionHandler;
    unsafe impl ClassType for NSScriptCoercionHandler {
        type Super = NSObject;
    }
);
impl NSScriptCoercionHandler {
    pub unsafe fn sharedCoercionHandler() -> Id<NSScriptCoercionHandler, Shared> {
        msg_send_id![Self::class(), sharedCoercionHandler]
    }
    pub unsafe fn coerceValue_toClass(
        &self,
        value: &Object,
        toClass: &Class,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![self, coerceValue: value, toClass: toClass]
    }
    pub unsafe fn registerCoercer_selector_toConvertFromClass_toClass(
        &self,
        coercer: &Object,
        selector: Sel,
        fromClass: &Class,
        toClass: &Class,
    ) {
        msg_send![
            self,
            registerCoercer: coercer,
            selector: selector,
            toConvertFromClass: fromClass,
            toClass: toClass
        ]
    }
}
