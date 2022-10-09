use crate::AppKit::generated::AppKitDefines::*;
use crate::Foundation::generated::Foundation::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSStoryboardName = NSString;
pub type NSStoryboardSceneIdentifier = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSStoryboard;
    unsafe impl ClassType for NSStoryboard {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSStoryboard {
        #[method_id(mainStoryboard)]
        pub unsafe fn mainStoryboard() -> Option<Id<NSStoryboard, Shared>>;
        #[method_id(storyboardWithName:bundle:)]
        pub unsafe fn storyboardWithName_bundle(
            name: &NSStoryboardName,
            storyboardBundleOrNil: Option<&NSBundle>,
        ) -> Id<Self, Shared>;
        #[method_id(instantiateInitialController)]
        pub unsafe fn instantiateInitialController(&self) -> Option<Id<Object, Shared>>;
        #[method_id(instantiateInitialControllerWithCreator:)]
        pub unsafe fn instantiateInitialControllerWithCreator(
            &self,
            block: NSStoryboardControllerCreator,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(instantiateControllerWithIdentifier:)]
        pub unsafe fn instantiateControllerWithIdentifier(
            &self,
            identifier: &NSStoryboardSceneIdentifier,
        ) -> Id<Object, Shared>;
        #[method_id(instantiateControllerWithIdentifier:creator:)]
        pub unsafe fn instantiateControllerWithIdentifier_creator(
            &self,
            identifier: &NSStoryboardSceneIdentifier,
            block: NSStoryboardControllerCreator,
        ) -> Id<Object, Shared>;
    }
);
