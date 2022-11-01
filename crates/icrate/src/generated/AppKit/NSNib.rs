//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

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
        #[method_id(@__retain_semantics Init initWithNibNamed:bundle:)]
        pub unsafe fn initWithNibNamed_bundle(
            this: Option<Allocated<Self>>,
            nibName: &NSNibName,
            bundle: Option<&NSBundle>,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithNibData:bundle:)]
        pub unsafe fn initWithNibData_bundle(
            this: Option<Allocated<Self>>,
            nibData: &NSData,
            bundle: Option<&NSBundle>,
        ) -> Id<Self, Shared>;

        #[method(instantiateWithOwner:topLevelObjects:)]
        pub unsafe fn instantiateWithOwner_topLevelObjects(
            &self,
            owner: Option<&Object>,
            topLevelObjects: *mut *mut NSArray,
        ) -> bool;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSNib {
        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Option<Allocated<Self>>,
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
            topLevelObjects: *mut *mut NSArray,
        ) -> bool;
    }
);

extern "C" {
    pub static NSNibOwner: &'static NSString;
}

extern "C" {
    pub static NSNibTopLevelObjects: &'static NSString;
}
