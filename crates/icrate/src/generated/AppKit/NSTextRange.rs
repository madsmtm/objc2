use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSTextLocation = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSTextRange;
    unsafe impl ClassType for NSTextRange {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSTextRange {
        #[method_id(initWithLocation:endLocation:)]
        pub unsafe fn initWithLocation_endLocation(
            &self,
            location: &NSTextLocation,
            endLocation: Option<&NSTextLocation>,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithLocation:)]
        pub unsafe fn initWithLocation(&self, location: &NSTextLocation) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
        #[method(isEmpty)]
        pub unsafe fn isEmpty(&self) -> bool;
        #[method_id(location)]
        pub unsafe fn location(&self) -> Id<NSTextLocation, Shared>;
        #[method_id(endLocation)]
        pub unsafe fn endLocation(&self) -> Id<NSTextLocation, Shared>;
        #[method(isEqualToTextRange:)]
        pub unsafe fn isEqualToTextRange(&self, textRange: &NSTextRange) -> bool;
        #[method(containsLocation:)]
        pub unsafe fn containsLocation(&self, location: &NSTextLocation) -> bool;
        #[method(containsRange:)]
        pub unsafe fn containsRange(&self, textRange: &NSTextRange) -> bool;
        #[method(intersectsWithTextRange:)]
        pub unsafe fn intersectsWithTextRange(&self, textRange: &NSTextRange) -> bool;
        #[method_id(textRangeByIntersectingWithTextRange:)]
        pub unsafe fn textRangeByIntersectingWithTextRange(
            &self,
            textRange: &NSTextRange,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(textRangeByFormingUnionWithTextRange:)]
        pub unsafe fn textRangeByFormingUnionWithTextRange(
            &self,
            textRange: &NSTextRange,
        ) -> Id<Self, Shared>;
    }
);
