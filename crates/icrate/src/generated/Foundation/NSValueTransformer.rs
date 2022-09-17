#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSValueTransformer;
    unsafe impl ClassType for NSValueTransformer {
        type Super = NSObject;
    }
);
impl NSValueTransformer {
    pub unsafe fn setValueTransformer_forName(
        transformer: Option<&NSValueTransformer>,
        name: NSValueTransformerName,
    ) {
        msg_send![
            Self::class(),
            setValueTransformer: transformer,
            forName: name
        ]
    }
    pub unsafe fn valueTransformerForName(
        name: NSValueTransformerName,
    ) -> Option<Id<NSValueTransformer, Shared>> {
        msg_send_id![Self::class(), valueTransformerForName: name]
    }
    pub unsafe fn valueTransformerNames() -> TodoGenerics {
        msg_send![Self::class(), valueTransformerNames]
    }
    pub unsafe fn transformedValueClass() -> &Class {
        msg_send![Self::class(), transformedValueClass]
    }
    pub unsafe fn allowsReverseTransformation() -> bool {
        msg_send![Self::class(), allowsReverseTransformation]
    }
    pub unsafe fn transformedValue(&self, value: Option<&Object>) -> Option<Id<Object, Shared>> {
        msg_send_id![self, transformedValue: value]
    }
    pub unsafe fn reverseTransformedValue(
        &self,
        value: Option<&Object>,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![self, reverseTransformedValue: value]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSSecureUnarchiveFromDataTransformer;
    unsafe impl ClassType for NSSecureUnarchiveFromDataTransformer {
        type Super = NSValueTransformer;
    }
);
impl NSSecureUnarchiveFromDataTransformer {
    pub unsafe fn allowedTopLevelClasses() -> TodoGenerics {
        msg_send![Self::class(), allowedTopLevelClasses]
    }
}
