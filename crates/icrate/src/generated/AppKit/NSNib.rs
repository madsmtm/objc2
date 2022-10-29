#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSNibName = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSNib;
    unsafe impl ClassType for NSNib {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSNib {
        #[method_id(initWithNibNamed:bundle:)]
        pub unsafe fn initWithNibNamed_bundle(
            &self,
            nibName: &NSNibName,
            bundle: Option<&NSBundle>,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithNibData:bundle:)]
        pub unsafe fn initWithNibData_bundle(
            &self,
            nibData: &NSData,
            bundle: Option<&NSBundle>,
        ) -> Id<Self, Shared>;
        #[method(instantiateWithOwner:topLevelObjects:)]
        pub unsafe fn instantiateWithOwner_topLevelObjects(
            &self,
            owner: Option<&Object>,
            topLevelObjects: Option<&mut Option<Id<NSArray, Shared>>>,
        ) -> bool;
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSNib {
        #[method_id(initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            &self,
            nibFileURL: Option<&NSURL>,
        ) -> Option<Id<Object, Shared>>;
        #[method(instantiateNibWithExternalNameTable:)]
        pub unsafe fn instantiateNibWithExternalNameTable(
            &self,
            externalNameTable: Option<&NSDictionary>,
        ) -> bool;
        #[method(instantiateNibWithOwner:topLevelObjects:)]
        pub unsafe fn instantiateNibWithOwner_topLevelObjects(
            &self,
            owner: Option<&Object>,
            topLevelObjects: Option<&mut Option<Id<NSArray, Shared>>>,
        ) -> bool;
    }
);
