use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSCollectionView::*;
use crate::AppKit::generated::NSCollectionViewLayout::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSCollectionViewFlowLayoutInvalidationContext;
    unsafe impl ClassType for NSCollectionViewFlowLayoutInvalidationContext {
        type Super = NSCollectionViewLayoutInvalidationContext;
    }
);
extern_methods!(
    unsafe impl NSCollectionViewFlowLayoutInvalidationContext {
        #[method(invalidateFlowLayoutDelegateMetrics)]
        pub unsafe fn invalidateFlowLayoutDelegateMetrics(&self) -> bool;
        #[method(setInvalidateFlowLayoutDelegateMetrics:)]
        pub unsafe fn setInvalidateFlowLayoutDelegateMetrics(
            &self,
            invalidateFlowLayoutDelegateMetrics: bool,
        );
        #[method(invalidateFlowLayoutAttributes)]
        pub unsafe fn invalidateFlowLayoutAttributes(&self) -> bool;
        #[method(setInvalidateFlowLayoutAttributes:)]
        pub unsafe fn setInvalidateFlowLayoutAttributes(
            &self,
            invalidateFlowLayoutAttributes: bool,
        );
    }
);
pub type NSCollectionViewDelegateFlowLayout = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSCollectionViewFlowLayout;
    unsafe impl ClassType for NSCollectionViewFlowLayout {
        type Super = NSCollectionViewLayout;
    }
);
extern_methods!(
    unsafe impl NSCollectionViewFlowLayout {
        #[method(minimumLineSpacing)]
        pub unsafe fn minimumLineSpacing(&self) -> CGFloat;
        #[method(setMinimumLineSpacing:)]
        pub unsafe fn setMinimumLineSpacing(&self, minimumLineSpacing: CGFloat);
        #[method(minimumInteritemSpacing)]
        pub unsafe fn minimumInteritemSpacing(&self) -> CGFloat;
        #[method(setMinimumInteritemSpacing:)]
        pub unsafe fn setMinimumInteritemSpacing(&self, minimumInteritemSpacing: CGFloat);
        #[method(itemSize)]
        pub unsafe fn itemSize(&self) -> NSSize;
        #[method(setItemSize:)]
        pub unsafe fn setItemSize(&self, itemSize: NSSize);
        #[method(estimatedItemSize)]
        pub unsafe fn estimatedItemSize(&self) -> NSSize;
        #[method(setEstimatedItemSize:)]
        pub unsafe fn setEstimatedItemSize(&self, estimatedItemSize: NSSize);
        #[method(scrollDirection)]
        pub unsafe fn scrollDirection(&self) -> NSCollectionViewScrollDirection;
        #[method(setScrollDirection:)]
        pub unsafe fn setScrollDirection(&self, scrollDirection: NSCollectionViewScrollDirection);
        #[method(headerReferenceSize)]
        pub unsafe fn headerReferenceSize(&self) -> NSSize;
        #[method(setHeaderReferenceSize:)]
        pub unsafe fn setHeaderReferenceSize(&self, headerReferenceSize: NSSize);
        #[method(footerReferenceSize)]
        pub unsafe fn footerReferenceSize(&self) -> NSSize;
        #[method(setFooterReferenceSize:)]
        pub unsafe fn setFooterReferenceSize(&self, footerReferenceSize: NSSize);
        #[method(sectionInset)]
        pub unsafe fn sectionInset(&self) -> NSEdgeInsets;
        #[method(setSectionInset:)]
        pub unsafe fn setSectionInset(&self, sectionInset: NSEdgeInsets);
        #[method(sectionHeadersPinToVisibleBounds)]
        pub unsafe fn sectionHeadersPinToVisibleBounds(&self) -> bool;
        #[method(setSectionHeadersPinToVisibleBounds:)]
        pub unsafe fn setSectionHeadersPinToVisibleBounds(
            &self,
            sectionHeadersPinToVisibleBounds: bool,
        );
        #[method(sectionFootersPinToVisibleBounds)]
        pub unsafe fn sectionFootersPinToVisibleBounds(&self) -> bool;
        #[method(setSectionFootersPinToVisibleBounds:)]
        pub unsafe fn setSectionFootersPinToVisibleBounds(
            &self,
            sectionFootersPinToVisibleBounds: bool,
        );
        #[method(sectionAtIndexIsCollapsed:)]
        pub unsafe fn sectionAtIndexIsCollapsed(&self, sectionIndex: NSUInteger) -> bool;
        #[method(collapseSectionAtIndex:)]
        pub unsafe fn collapseSectionAtIndex(&self, sectionIndex: NSUInteger);
        #[method(expandSectionAtIndex:)]
        pub unsafe fn expandSectionAtIndex(&self, sectionIndex: NSUInteger);
    }
);
