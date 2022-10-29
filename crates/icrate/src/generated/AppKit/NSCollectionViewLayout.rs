#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSCollectionViewDecorationElementKind = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSCollectionViewLayoutAttributes;
    unsafe impl ClassType for NSCollectionViewLayoutAttributes {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSCollectionViewLayoutAttributes {
        #[method(frame)]
        pub unsafe fn frame(&self) -> NSRect;
        #[method(setFrame:)]
        pub unsafe fn setFrame(&self, frame: NSRect);
        #[method(size)]
        pub unsafe fn size(&self) -> NSSize;
        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: NSSize);
        #[method(alpha)]
        pub unsafe fn alpha(&self) -> CGFloat;
        #[method(setAlpha:)]
        pub unsafe fn setAlpha(&self, alpha: CGFloat);
        #[method(zIndex)]
        pub unsafe fn zIndex(&self) -> NSInteger;
        #[method(setZIndex:)]
        pub unsafe fn setZIndex(&self, zIndex: NSInteger);
        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;
        #[method(setHidden:)]
        pub unsafe fn setHidden(&self, hidden: bool);
        #[method_id(indexPath)]
        pub unsafe fn indexPath(&self) -> Option<Id<NSIndexPath, Shared>>;
        #[method(setIndexPath:)]
        pub unsafe fn setIndexPath(&self, indexPath: Option<&NSIndexPath>);
        #[method(representedElementCategory)]
        pub unsafe fn representedElementCategory(&self) -> NSCollectionElementCategory;
        #[method_id(representedElementKind)]
        pub unsafe fn representedElementKind(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(layoutAttributesForItemWithIndexPath:)]
        pub unsafe fn layoutAttributesForItemWithIndexPath(
            indexPath: &NSIndexPath,
        ) -> Id<Self, Shared>;
        #[method_id(layoutAttributesForInterItemGapBeforeIndexPath:)]
        pub unsafe fn layoutAttributesForInterItemGapBeforeIndexPath(
            indexPath: &NSIndexPath,
        ) -> Id<Self, Shared>;
        #[method_id(layoutAttributesForSupplementaryViewOfKind:withIndexPath:)]
        pub unsafe fn layoutAttributesForSupplementaryViewOfKind_withIndexPath(
            elementKind: &NSCollectionViewSupplementaryElementKind,
            indexPath: &NSIndexPath,
        ) -> Id<Self, Shared>;
        #[method_id(layoutAttributesForDecorationViewOfKind:withIndexPath:)]
        pub unsafe fn layoutAttributesForDecorationViewOfKind_withIndexPath(
            decorationViewKind: &NSCollectionViewDecorationElementKind,
            indexPath: &NSIndexPath,
        ) -> Id<Self, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSCollectionViewUpdateItem;
    unsafe impl ClassType for NSCollectionViewUpdateItem {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSCollectionViewUpdateItem {
        #[method_id(indexPathBeforeUpdate)]
        pub unsafe fn indexPathBeforeUpdate(&self) -> Option<Id<NSIndexPath, Shared>>;
        #[method_id(indexPathAfterUpdate)]
        pub unsafe fn indexPathAfterUpdate(&self) -> Option<Id<NSIndexPath, Shared>>;
        #[method(updateAction)]
        pub unsafe fn updateAction(&self) -> NSCollectionUpdateAction;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSCollectionViewLayoutInvalidationContext;
    unsafe impl ClassType for NSCollectionViewLayoutInvalidationContext {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSCollectionViewLayoutInvalidationContext {
        #[method(invalidateEverything)]
        pub unsafe fn invalidateEverything(&self) -> bool;
        #[method(invalidateDataSourceCounts)]
        pub unsafe fn invalidateDataSourceCounts(&self) -> bool;
        #[method(invalidateItemsAtIndexPaths:)]
        pub unsafe fn invalidateItemsAtIndexPaths(&self, indexPaths: &NSSet<NSIndexPath>);
        #[method(invalidateSupplementaryElementsOfKind:atIndexPaths:)]
        pub unsafe fn invalidateSupplementaryElementsOfKind_atIndexPaths(
            &self,
            elementKind: &NSCollectionViewSupplementaryElementKind,
            indexPaths: &NSSet<NSIndexPath>,
        );
        #[method(invalidateDecorationElementsOfKind:atIndexPaths:)]
        pub unsafe fn invalidateDecorationElementsOfKind_atIndexPaths(
            &self,
            elementKind: &NSCollectionViewDecorationElementKind,
            indexPaths: &NSSet<NSIndexPath>,
        );
        #[method_id(invalidatedItemIndexPaths)]
        pub unsafe fn invalidatedItemIndexPaths(&self) -> Option<Id<NSSet<NSIndexPath>, Shared>>;
        #[method_id(invalidatedSupplementaryIndexPaths)]
        pub unsafe fn invalidatedSupplementaryIndexPaths(
            &self,
        ) -> Option<
            Id<NSDictionary<NSCollectionViewSupplementaryElementKind, NSSet<NSIndexPath>>, Shared>,
        >;
        #[method_id(invalidatedDecorationIndexPaths)]
        pub unsafe fn invalidatedDecorationIndexPaths(
            &self,
        ) -> Option<
            Id<NSDictionary<NSCollectionViewDecorationElementKind, NSSet<NSIndexPath>>, Shared>,
        >;
        #[method(contentOffsetAdjustment)]
        pub unsafe fn contentOffsetAdjustment(&self) -> NSPoint;
        #[method(setContentOffsetAdjustment:)]
        pub unsafe fn setContentOffsetAdjustment(&self, contentOffsetAdjustment: NSPoint);
        #[method(contentSizeAdjustment)]
        pub unsafe fn contentSizeAdjustment(&self) -> NSSize;
        #[method(setContentSizeAdjustment:)]
        pub unsafe fn setContentSizeAdjustment(&self, contentSizeAdjustment: NSSize);
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSCollectionViewLayout;
    unsafe impl ClassType for NSCollectionViewLayout {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSCollectionViewLayout {
        #[method_id(collectionView)]
        pub unsafe fn collectionView(&self) -> Option<Id<NSCollectionView, Shared>>;
        #[method(invalidateLayout)]
        pub unsafe fn invalidateLayout(&self);
        #[method(invalidateLayoutWithContext:)]
        pub unsafe fn invalidateLayoutWithContext(
            &self,
            context: &NSCollectionViewLayoutInvalidationContext,
        );
        #[method(registerClass:forDecorationViewOfKind:)]
        pub unsafe fn registerClass_forDecorationViewOfKind(
            &self,
            viewClass: Option<&Class>,
            elementKind: &NSCollectionViewDecorationElementKind,
        );
        #[method(registerNib:forDecorationViewOfKind:)]
        pub unsafe fn registerNib_forDecorationViewOfKind(
            &self,
            nib: Option<&NSNib>,
            elementKind: &NSCollectionViewDecorationElementKind,
        );
    }
);
extern_methods!(
    #[doc = "NSSubclassingHooks"]
    unsafe impl NSCollectionViewLayout {
        #[method(layoutAttributesClass)]
        pub unsafe fn layoutAttributesClass() -> &Class;
        #[method(invalidationContextClass)]
        pub unsafe fn invalidationContextClass() -> &Class;
        #[method(prepareLayout)]
        pub unsafe fn prepareLayout(&self);
        #[method_id(layoutAttributesForElementsInRect:)]
        pub unsafe fn layoutAttributesForElementsInRect(
            &self,
            rect: NSRect,
        ) -> Id<NSArray<NSCollectionViewLayoutAttributes>, Shared>;
        #[method_id(layoutAttributesForItemAtIndexPath:)]
        pub unsafe fn layoutAttributesForItemAtIndexPath(
            &self,
            indexPath: &NSIndexPath,
        ) -> Option<Id<NSCollectionViewLayoutAttributes, Shared>>;
        #[method_id(layoutAttributesForSupplementaryViewOfKind:atIndexPath:)]
        pub unsafe fn layoutAttributesForSupplementaryViewOfKind_atIndexPath(
            &self,
            elementKind: &NSCollectionViewSupplementaryElementKind,
            indexPath: &NSIndexPath,
        ) -> Option<Id<NSCollectionViewLayoutAttributes, Shared>>;
        #[method_id(layoutAttributesForDecorationViewOfKind:atIndexPath:)]
        pub unsafe fn layoutAttributesForDecorationViewOfKind_atIndexPath(
            &self,
            elementKind: &NSCollectionViewDecorationElementKind,
            indexPath: &NSIndexPath,
        ) -> Option<Id<NSCollectionViewLayoutAttributes, Shared>>;
        #[method_id(layoutAttributesForDropTargetAtPoint:)]
        pub unsafe fn layoutAttributesForDropTargetAtPoint(
            &self,
            pointInCollectionView: NSPoint,
        ) -> Option<Id<NSCollectionViewLayoutAttributes, Shared>>;
        #[method_id(layoutAttributesForInterItemGapBeforeIndexPath:)]
        pub unsafe fn layoutAttributesForInterItemGapBeforeIndexPath(
            &self,
            indexPath: &NSIndexPath,
        ) -> Option<Id<NSCollectionViewLayoutAttributes, Shared>>;
        #[method(shouldInvalidateLayoutForBoundsChange:)]
        pub unsafe fn shouldInvalidateLayoutForBoundsChange(&self, newBounds: NSRect) -> bool;
        #[method_id(invalidationContextForBoundsChange:)]
        pub unsafe fn invalidationContextForBoundsChange(
            &self,
            newBounds: NSRect,
        ) -> Id<NSCollectionViewLayoutInvalidationContext, Shared>;
        #[method(shouldInvalidateLayoutForPreferredLayoutAttributes:withOriginalAttributes:)]
        pub unsafe fn shouldInvalidateLayoutForPreferredLayoutAttributes_withOriginalAttributes(
            &self,
            preferredAttributes: &NSCollectionViewLayoutAttributes,
            originalAttributes: &NSCollectionViewLayoutAttributes,
        ) -> bool;
        #[method_id(invalidationContextForPreferredLayoutAttributes:withOriginalAttributes:)]
        pub unsafe fn invalidationContextForPreferredLayoutAttributes_withOriginalAttributes(
            &self,
            preferredAttributes: &NSCollectionViewLayoutAttributes,
            originalAttributes: &NSCollectionViewLayoutAttributes,
        ) -> Id<NSCollectionViewLayoutInvalidationContext, Shared>;
        #[method(targetContentOffsetForProposedContentOffset:withScrollingVelocity:)]
        pub unsafe fn targetContentOffsetForProposedContentOffset_withScrollingVelocity(
            &self,
            proposedContentOffset: NSPoint,
            velocity: NSPoint,
        ) -> NSPoint;
        #[method(targetContentOffsetForProposedContentOffset:)]
        pub unsafe fn targetContentOffsetForProposedContentOffset(
            &self,
            proposedContentOffset: NSPoint,
        ) -> NSPoint;
        #[method(collectionViewContentSize)]
        pub unsafe fn collectionViewContentSize(&self) -> NSSize;
    }
);
extern_methods!(
    #[doc = "NSUpdateSupportHooks"]
    unsafe impl NSCollectionViewLayout {
        #[method(prepareForCollectionViewUpdates:)]
        pub unsafe fn prepareForCollectionViewUpdates(
            &self,
            updateItems: &NSArray<NSCollectionViewUpdateItem>,
        );
        #[method(finalizeCollectionViewUpdates)]
        pub unsafe fn finalizeCollectionViewUpdates(&self);
        #[method(prepareForAnimatedBoundsChange:)]
        pub unsafe fn prepareForAnimatedBoundsChange(&self, oldBounds: NSRect);
        #[method(finalizeAnimatedBoundsChange)]
        pub unsafe fn finalizeAnimatedBoundsChange(&self);
        #[method(prepareForTransitionToLayout:)]
        pub unsafe fn prepareForTransitionToLayout(&self, newLayout: &NSCollectionViewLayout);
        #[method(prepareForTransitionFromLayout:)]
        pub unsafe fn prepareForTransitionFromLayout(&self, oldLayout: &NSCollectionViewLayout);
        #[method(finalizeLayoutTransition)]
        pub unsafe fn finalizeLayoutTransition(&self);
        #[method_id(initialLayoutAttributesForAppearingItemAtIndexPath:)]
        pub unsafe fn initialLayoutAttributesForAppearingItemAtIndexPath(
            &self,
            itemIndexPath: &NSIndexPath,
        ) -> Option<Id<NSCollectionViewLayoutAttributes, Shared>>;
        #[method_id(finalLayoutAttributesForDisappearingItemAtIndexPath:)]
        pub unsafe fn finalLayoutAttributesForDisappearingItemAtIndexPath(
            &self,
            itemIndexPath: &NSIndexPath,
        ) -> Option<Id<NSCollectionViewLayoutAttributes, Shared>>;
        #[method_id(initialLayoutAttributesForAppearingSupplementaryElementOfKind:atIndexPath:)]
        pub unsafe fn initialLayoutAttributesForAppearingSupplementaryElementOfKind_atIndexPath(
            &self,
            elementKind: &NSCollectionViewSupplementaryElementKind,
            elementIndexPath: &NSIndexPath,
        ) -> Option<Id<NSCollectionViewLayoutAttributes, Shared>>;
        #[method_id(finalLayoutAttributesForDisappearingSupplementaryElementOfKind:atIndexPath:)]
        pub unsafe fn finalLayoutAttributesForDisappearingSupplementaryElementOfKind_atIndexPath(
            &self,
            elementKind: &NSCollectionViewSupplementaryElementKind,
            elementIndexPath: &NSIndexPath,
        ) -> Option<Id<NSCollectionViewLayoutAttributes, Shared>>;
        #[method_id(initialLayoutAttributesForAppearingDecorationElementOfKind:atIndexPath:)]
        pub unsafe fn initialLayoutAttributesForAppearingDecorationElementOfKind_atIndexPath(
            &self,
            elementKind: &NSCollectionViewDecorationElementKind,
            decorationIndexPath: &NSIndexPath,
        ) -> Option<Id<NSCollectionViewLayoutAttributes, Shared>>;
        #[method_id(finalLayoutAttributesForDisappearingDecorationElementOfKind:atIndexPath:)]
        pub unsafe fn finalLayoutAttributesForDisappearingDecorationElementOfKind_atIndexPath(
            &self,
            elementKind: &NSCollectionViewDecorationElementKind,
            decorationIndexPath: &NSIndexPath,
        ) -> Option<Id<NSCollectionViewLayoutAttributes, Shared>>;
        #[method_id(indexPathsToDeleteForSupplementaryViewOfKind:)]
        pub unsafe fn indexPathsToDeleteForSupplementaryViewOfKind(
            &self,
            elementKind: &NSCollectionViewSupplementaryElementKind,
        ) -> Id<NSSet<NSIndexPath>, Shared>;
        #[method_id(indexPathsToDeleteForDecorationViewOfKind:)]
        pub unsafe fn indexPathsToDeleteForDecorationViewOfKind(
            &self,
            elementKind: &NSCollectionViewDecorationElementKind,
        ) -> Id<NSSet<NSIndexPath>, Shared>;
        #[method_id(indexPathsToInsertForSupplementaryViewOfKind:)]
        pub unsafe fn indexPathsToInsertForSupplementaryViewOfKind(
            &self,
            elementKind: &NSCollectionViewSupplementaryElementKind,
        ) -> Id<NSSet<NSIndexPath>, Shared>;
        #[method_id(indexPathsToInsertForDecorationViewOfKind:)]
        pub unsafe fn indexPathsToInsertForDecorationViewOfKind(
            &self,
            elementKind: &NSCollectionViewDecorationElementKind,
        ) -> Id<NSSet<NSIndexPath>, Shared>;
    }
);