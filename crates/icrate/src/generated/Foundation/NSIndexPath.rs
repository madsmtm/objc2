use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSRange::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSIndexPath;
    unsafe impl ClassType for NSIndexPath {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSIndexPath {
        #[method_id(indexPathWithIndex:)]
        pub unsafe fn indexPathWithIndex(index: NSUInteger) -> Id<Self, Shared>;
        #[method_id(indexPathWithIndexes:length:)]
        pub unsafe fn indexPathWithIndexes_length(
            indexes: TodoArray,
            length: NSUInteger,
        ) -> Id<Self, Shared>;
        #[method_id(initWithIndexes:length:)]
        pub unsafe fn initWithIndexes_length(
            &self,
            indexes: TodoArray,
            length: NSUInteger,
        ) -> Id<Self, Shared>;
        #[method_id(initWithIndex:)]
        pub unsafe fn initWithIndex(&self, index: NSUInteger) -> Id<Self, Shared>;
        #[method_id(indexPathByAddingIndex:)]
        pub unsafe fn indexPathByAddingIndex(&self, index: NSUInteger) -> Id<NSIndexPath, Shared>;
        #[method_id(indexPathByRemovingLastIndex)]
        pub unsafe fn indexPathByRemovingLastIndex(&self) -> Id<NSIndexPath, Shared>;
        #[method(indexAtPosition:)]
        pub unsafe fn indexAtPosition(&self, position: NSUInteger) -> NSUInteger;
        #[method(length)]
        pub unsafe fn length(&self) -> NSUInteger;
        #[method(getIndexes:range:)]
        pub unsafe fn getIndexes_range(&self, indexes: NonNull<NSUInteger>, positionRange: NSRange);
        #[method(compare:)]
        pub unsafe fn compare(&self, otherObject: &NSIndexPath) -> NSComparisonResult;
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSIndexPath {
        #[method(getIndexes:)]
        pub unsafe fn getIndexes(&self, indexes: NonNull<NSUInteger>);
    }
);
