#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSCopying = NSObject;
pub type NSMutableCopying = NSObject;
pub type NSCoding = NSObject;
pub type NSSecureCoding = NSObject;
extern_methods!(
    #[doc = "NSCoderMethods"]
    unsafe impl NSObject {
        #[method(version)]
        pub unsafe fn version() -> NSInteger;
        #[method(setVersion:)]
        pub unsafe fn setVersion(aVersion: NSInteger);
        #[method(classForCoder)]
        pub unsafe fn classForCoder(&self) -> &Class;
        #[method_id(replacementObjectForCoder:)]
        pub unsafe fn replacementObjectForCoder(
            &self,
            coder: &NSCoder,
        ) -> Option<Id<Object, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSDeprecatedMethods"]
    unsafe impl NSObject {
        #[method(poseAsClass:)]
        pub unsafe fn poseAsClass(aClass: &Class);
    }
);
pub type NSDiscardableContent = NSObject;
extern_methods!(
    #[doc = "NSDiscardableContentProxy"]
    unsafe impl NSObject {
        #[method_id(autoContentAccessingProxy)]
        pub unsafe fn autoContentAccessingProxy(&self) -> Id<Object, Shared>;
    }
);
