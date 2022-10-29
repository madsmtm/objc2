#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSDataAssetName = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSDataAsset;
    unsafe impl ClassType for NSDataAsset {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSDataAsset {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(initWithName:)]
        pub unsafe fn initWithName(&self, name: &NSDataAssetName) -> Option<Id<Self, Shared>>;
        #[method_id(initWithName:bundle:)]
        pub unsafe fn initWithName_bundle(
            &self,
            name: &NSDataAssetName,
            bundle: &NSBundle,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(name)]
        pub unsafe fn name(&self) -> Id<NSDataAssetName, Shared>;
        #[method_id(data)]
        pub unsafe fn data(&self) -> Id<NSData, Shared>;
        #[method_id(typeIdentifier)]
        pub unsafe fn typeIdentifier(&self) -> Id<NSString, Shared>;
    }
);
