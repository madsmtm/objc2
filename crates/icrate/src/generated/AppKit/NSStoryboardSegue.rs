use crate::AppKit::generated::AppKitDefines::*;
use crate::Foundation::generated::Foundation::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSStoryboardSegueIdentifier = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSStoryboardSegue;
    unsafe impl ClassType for NSStoryboardSegue {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSStoryboardSegue {
        #[method_id(segueWithIdentifier:source:destination:performHandler:)]
        pub unsafe fn segueWithIdentifier_source_destination_performHandler(
            identifier: &NSStoryboardSegueIdentifier,
            sourceController: &Object,
            destinationController: &Object,
            performHandler: TodoBlock,
        ) -> Id<Self, Shared>;
        #[method_id(initWithIdentifier:source:destination:)]
        pub unsafe fn initWithIdentifier_source_destination(
            &self,
            identifier: &NSStoryboardSegueIdentifier,
            sourceController: &Object,
            destinationController: &Object,
        ) -> Id<Self, Shared>;
        #[method_id(identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSStoryboardSegueIdentifier, Shared>>;
        #[method_id(sourceController)]
        pub unsafe fn sourceController(&self) -> Id<Object, Shared>;
        #[method_id(destinationController)]
        pub unsafe fn destinationController(&self) -> Id<Object, Shared>;
        #[method(perform)]
        pub unsafe fn perform(&self);
    }
);
pub type NSSeguePerforming = NSObject;
