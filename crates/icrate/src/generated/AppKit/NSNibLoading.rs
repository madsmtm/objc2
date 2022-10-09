use super::__exported::NSDictionary;
use super::__exported::NSString;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSNib::*;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSBundle::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_methods!(
    #[doc = "NSNibLoading"]
    unsafe impl NSBundle {
        #[method(loadNibNamed:owner:topLevelObjects:)]
        pub unsafe fn loadNibNamed_owner_topLevelObjects(
            &self,
            nibName: &NSNibName,
            owner: Option<&Object>,
            topLevelObjects: Option<&mut Option<Id<NSArray, Shared>>>,
        ) -> bool;
    }
);
extern_methods!(
    #[doc = "NSNibAwaking"]
    unsafe impl NSObject {
        #[method(awakeFromNib)]
        pub unsafe fn awakeFromNib(&self);
        #[method(prepareForInterfaceBuilder)]
        pub unsafe fn prepareForInterfaceBuilder(&self);
    }
);
extern_methods!(
    #[doc = "NSNibLoadingDeprecated"]
    unsafe impl NSBundle {
        #[method(loadNibFile:externalNameTable:withZone:)]
        pub unsafe fn loadNibFile_externalNameTable_withZone(
            fileName: Option<&NSString>,
            context: Option<&NSDictionary>,
            zone: *mut NSZone,
        ) -> bool;
        #[method(loadNibNamed:owner:)]
        pub unsafe fn loadNibNamed_owner(
            nibName: Option<&NSString>,
            owner: Option<&Object>,
        ) -> bool;
        #[method(loadNibFile:externalNameTable:withZone:)]
        pub unsafe fn loadNibFile_externalNameTable_withZone(
            &self,
            fileName: Option<&NSString>,
            context: Option<&NSDictionary>,
            zone: *mut NSZone,
        ) -> bool;
    }
);
