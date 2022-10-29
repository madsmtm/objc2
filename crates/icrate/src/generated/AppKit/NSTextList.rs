#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSTextListMarkerFormat = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSTextList;
    unsafe impl ClassType for NSTextList {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSTextList {
        #[method_id(initWithMarkerFormat:options:)]
        pub unsafe fn initWithMarkerFormat_options(
            &self,
            format: &NSTextListMarkerFormat,
            mask: NSUInteger,
        ) -> Id<Self, Shared>;
        #[method_id(markerFormat)]
        pub unsafe fn markerFormat(&self) -> Id<NSTextListMarkerFormat, Shared>;
        #[method(listOptions)]
        pub unsafe fn listOptions(&self) -> NSTextListOptions;
        #[method_id(markerForItemNumber:)]
        pub unsafe fn markerForItemNumber(&self, itemNum: NSInteger) -> Id<NSString, Shared>;
        #[method(startingItemNumber)]
        pub unsafe fn startingItemNumber(&self) -> NSInteger;
        #[method(setStartingItemNumber:)]
        pub unsafe fn setStartingItemNumber(&self, startingItemNumber: NSInteger);
    }
);