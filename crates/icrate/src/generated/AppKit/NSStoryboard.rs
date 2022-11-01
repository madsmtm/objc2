//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

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
        #[method_id(@__retain_semantics Other mainStoryboard)]
        pub unsafe fn mainStoryboard() -> Option<Id<NSStoryboard, Shared>>;

        #[method_id(@__retain_semantics Other storyboardWithName:bundle:)]
        pub unsafe fn storyboardWithName_bundle(
            name: &NSStoryboardName,
            storyboardBundleOrNil: Option<&NSBundle>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other instantiateInitialController)]
        pub unsafe fn instantiateInitialController(&self) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other instantiateInitialControllerWithCreator:)]
        pub unsafe fn instantiateInitialControllerWithCreator(
            &self,
            block: NSStoryboardControllerCreator,
        ) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other instantiateControllerWithIdentifier:)]
        pub unsafe fn instantiateControllerWithIdentifier(
            &self,
            identifier: &NSStoryboardSceneIdentifier,
        ) -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other instantiateControllerWithIdentifier:creator:)]
        pub unsafe fn instantiateControllerWithIdentifier_creator(
            &self,
            identifier: &NSStoryboardSceneIdentifier,
            block: NSStoryboardControllerCreator,
        ) -> Id<Object, Shared>;
    }
);
