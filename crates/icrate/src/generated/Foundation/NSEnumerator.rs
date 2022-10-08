use super::__exported::NSArray;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSFastEnumeration = NSObject;
__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSEnumerator<ObjectType: Message>;
    unsafe impl<ObjectType: Message> ClassType for NSEnumerator<ObjectType> {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl<ObjectType: Message> NSEnumerator<ObjectType> {
        #[method_id(nextObject)]
        pub unsafe fn nextObject(&self) -> Option<Id<ObjectType, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSExtendedEnumerator"]
    unsafe impl<ObjectType: Message> NSEnumerator<ObjectType> {
        #[method_id(allObjects)]
        pub unsafe fn allObjects(&self) -> Id<NSArray<ObjectType>, Shared>;
    }
);
