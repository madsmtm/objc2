use super::__exported::NSArray;
use super::__exported::NSString;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSValueTransformerName = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSValueTransformer;
    unsafe impl ClassType for NSValueTransformer {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSValueTransformer {
        #[method(setValueTransformer:forName:)]
        pub unsafe fn setValueTransformer_forName(
            transformer: Option<&NSValueTransformer>,
            name: &NSValueTransformerName,
        );
        #[method_id(valueTransformerForName:)]
        pub unsafe fn valueTransformerForName(
            name: &NSValueTransformerName,
        ) -> Option<Id<NSValueTransformer, Shared>>;
        #[method_id(valueTransformerNames)]
        pub unsafe fn valueTransformerNames() -> Id<NSArray<NSValueTransformerName>, Shared>;
        #[method(transformedValueClass)]
        pub unsafe fn transformedValueClass() -> &Class;
        #[method(allowsReverseTransformation)]
        pub unsafe fn allowsReverseTransformation() -> bool;
        #[method_id(transformedValue:)]
        pub unsafe fn transformedValue(&self, value: Option<&Object>)
            -> Option<Id<Object, Shared>>;
        #[method_id(reverseTransformedValue:)]
        pub unsafe fn reverseTransformedValue(
            &self,
            value: Option<&Object>,
        ) -> Option<Id<Object, Shared>>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSSecureUnarchiveFromDataTransformer;
    unsafe impl ClassType for NSSecureUnarchiveFromDataTransformer {
        type Super = NSValueTransformer;
    }
);
extern_methods!(
    unsafe impl NSSecureUnarchiveFromDataTransformer {
        #[method_id(allowedTopLevelClasses)]
        pub unsafe fn allowedTopLevelClasses() -> Id<NSArray<TodoClass>, Shared>;
    }
);
