use super::__exported::NSIndexSet;
use super::__exported::NSScrubber;
use super::__exported::NSScrubberDelegate;
use crate::AppKit::generated::AppKitDefines::*;
use crate::Foundation::generated::NSGeometry::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSScrubberLayoutAttributes;
    unsafe impl ClassType for NSScrubberLayoutAttributes {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSScrubberLayoutAttributes {
        #[method(itemIndex)]
        pub unsafe fn itemIndex(&self) -> NSInteger;
        #[method(setItemIndex:)]
        pub unsafe fn setItemIndex(&self, itemIndex: NSInteger);
        #[method(frame)]
        pub unsafe fn frame(&self) -> NSRect;
        #[method(setFrame:)]
        pub unsafe fn setFrame(&self, frame: NSRect);
        #[method(alpha)]
        pub unsafe fn alpha(&self) -> CGFloat;
        #[method(setAlpha:)]
        pub unsafe fn setAlpha(&self, alpha: CGFloat);
        #[method_id(layoutAttributesForItemAtIndex:)]
        pub unsafe fn layoutAttributesForItemAtIndex(index: NSInteger) -> Id<Self, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSScrubberLayout;
    unsafe impl ClassType for NSScrubberLayout {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSScrubberLayout {
        #[method(layoutAttributesClass)]
        pub unsafe fn layoutAttributesClass() -> &Class;
        #[method_id(scrubber)]
        pub unsafe fn scrubber(&self) -> Option<Id<NSScrubber, Shared>>;
        #[method(visibleRect)]
        pub unsafe fn visibleRect(&self) -> NSRect;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Id<Self, Shared>;
        #[method(invalidateLayout)]
        pub unsafe fn invalidateLayout(&self);
        #[method(prepareLayout)]
        pub unsafe fn prepareLayout(&self);
        #[method(scrubberContentSize)]
        pub unsafe fn scrubberContentSize(&self) -> NSSize;
        #[method_id(layoutAttributesForItemAtIndex:)]
        pub unsafe fn layoutAttributesForItemAtIndex(
            &self,
            index: NSInteger,
        ) -> Option<Id<NSScrubberLayoutAttributes, Shared>>;
        #[method_id(layoutAttributesForItemsInRect:)]
        pub unsafe fn layoutAttributesForItemsInRect(
            &self,
            rect: NSRect,
        ) -> Id<NSSet<NSScrubberLayoutAttributes>, Shared>;
        #[method(shouldInvalidateLayoutForSelectionChange)]
        pub unsafe fn shouldInvalidateLayoutForSelectionChange(&self) -> bool;
        #[method(shouldInvalidateLayoutForHighlightChange)]
        pub unsafe fn shouldInvalidateLayoutForHighlightChange(&self) -> bool;
        #[method(shouldInvalidateLayoutForChangeFromVisibleRect:toVisibleRect:)]
        pub unsafe fn shouldInvalidateLayoutForChangeFromVisibleRect_toVisibleRect(
            &self,
            fromVisibleRect: NSRect,
            toVisibleRect: NSRect,
        ) -> bool;
        #[method(automaticallyMirrorsInRightToLeftLayout)]
        pub unsafe fn automaticallyMirrorsInRightToLeftLayout(&self) -> bool;
    }
);
pub type NSScrubberFlowLayoutDelegate = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSScrubberFlowLayout;
    unsafe impl ClassType for NSScrubberFlowLayout {
        type Super = NSScrubberLayout;
    }
);
extern_methods!(
    unsafe impl NSScrubberFlowLayout {
        #[method(itemSpacing)]
        pub unsafe fn itemSpacing(&self) -> CGFloat;
        #[method(setItemSpacing:)]
        pub unsafe fn setItemSpacing(&self, itemSpacing: CGFloat);
        #[method(itemSize)]
        pub unsafe fn itemSize(&self) -> NSSize;
        #[method(setItemSize:)]
        pub unsafe fn setItemSize(&self, itemSize: NSSize);
        #[method(invalidateLayoutForItemsAtIndexes:)]
        pub unsafe fn invalidateLayoutForItemsAtIndexes(&self, invalidItemIndexes: &NSIndexSet);
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSScrubberProportionalLayout;
    unsafe impl ClassType for NSScrubberProportionalLayout {
        type Super = NSScrubberLayout;
    }
);
extern_methods!(
    unsafe impl NSScrubberProportionalLayout {
        #[method(numberOfVisibleItems)]
        pub unsafe fn numberOfVisibleItems(&self) -> NSInteger;
        #[method(setNumberOfVisibleItems:)]
        pub unsafe fn setNumberOfVisibleItems(&self, numberOfVisibleItems: NSInteger);
        #[method_id(initWithNumberOfVisibleItems:)]
        pub unsafe fn initWithNumberOfVisibleItems(
            &self,
            numberOfVisibleItems: NSInteger,
        ) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Id<Self, Shared>;
    }
);
