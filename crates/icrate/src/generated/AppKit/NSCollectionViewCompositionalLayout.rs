#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSCollectionViewCompositionalLayoutConfiguration;
    unsafe impl ClassType for NSCollectionViewCompositionalLayoutConfiguration {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSCollectionViewCompositionalLayoutConfiguration {
        #[method(scrollDirection)]
        pub unsafe fn scrollDirection(&self) -> NSCollectionViewScrollDirection;
        #[method(setScrollDirection:)]
        pub unsafe fn setScrollDirection(&self, scrollDirection: NSCollectionViewScrollDirection);
        #[method(interSectionSpacing)]
        pub unsafe fn interSectionSpacing(&self) -> CGFloat;
        #[method(setInterSectionSpacing:)]
        pub unsafe fn setInterSectionSpacing(&self, interSectionSpacing: CGFloat);
        #[method_id(boundarySupplementaryItems)]
        pub unsafe fn boundarySupplementaryItems(
            &self,
        ) -> Id<NSArray<NSCollectionLayoutBoundarySupplementaryItem>, Shared>;
        #[method(setBoundarySupplementaryItems:)]
        pub unsafe fn setBoundarySupplementaryItems(
            &self,
            boundarySupplementaryItems: &NSArray<NSCollectionLayoutBoundarySupplementaryItem>,
        );
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSCollectionViewCompositionalLayout;
    unsafe impl ClassType for NSCollectionViewCompositionalLayout {
        type Super = NSCollectionViewLayout;
    }
);
extern_methods!(
    unsafe impl NSCollectionViewCompositionalLayout {
        #[method_id(initWithSection:)]
        pub unsafe fn initWithSection(
            &self,
            section: &NSCollectionLayoutSection,
        ) -> Id<Self, Shared>;
        #[method_id(initWithSection:configuration:)]
        pub unsafe fn initWithSection_configuration(
            &self,
            section: &NSCollectionLayoutSection,
            configuration: &NSCollectionViewCompositionalLayoutConfiguration,
        ) -> Id<Self, Shared>;
        #[method_id(initWithSectionProvider:)]
        pub unsafe fn initWithSectionProvider(
            &self,
            sectionProvider: NSCollectionViewCompositionalLayoutSectionProvider,
        ) -> Id<Self, Shared>;
        #[method_id(initWithSectionProvider:configuration:)]
        pub unsafe fn initWithSectionProvider_configuration(
            &self,
            sectionProvider: NSCollectionViewCompositionalLayoutSectionProvider,
            configuration: &NSCollectionViewCompositionalLayoutConfiguration,
        ) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
        #[method_id(configuration)]
        pub unsafe fn configuration(
            &self,
        ) -> Id<NSCollectionViewCompositionalLayoutConfiguration, Shared>;
        #[method(setConfiguration:)]
        pub unsafe fn setConfiguration(
            &self,
            configuration: &NSCollectionViewCompositionalLayoutConfiguration,
        );
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSCollectionLayoutSection;
    unsafe impl ClassType for NSCollectionLayoutSection {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSCollectionLayoutSection {
        #[method_id(sectionWithGroup:)]
        pub unsafe fn sectionWithGroup(group: &NSCollectionLayoutGroup) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
        #[method(contentInsets)]
        pub unsafe fn contentInsets(&self) -> NSDirectionalEdgeInsets;
        #[method(setContentInsets:)]
        pub unsafe fn setContentInsets(&self, contentInsets: NSDirectionalEdgeInsets);
        #[method(interGroupSpacing)]
        pub unsafe fn interGroupSpacing(&self) -> CGFloat;
        #[method(setInterGroupSpacing:)]
        pub unsafe fn setInterGroupSpacing(&self, interGroupSpacing: CGFloat);
        #[method(orthogonalScrollingBehavior)]
        pub unsafe fn orthogonalScrollingBehavior(
            &self,
        ) -> NSCollectionLayoutSectionOrthogonalScrollingBehavior;
        #[method(setOrthogonalScrollingBehavior:)]
        pub unsafe fn setOrthogonalScrollingBehavior(
            &self,
            orthogonalScrollingBehavior: NSCollectionLayoutSectionOrthogonalScrollingBehavior,
        );
        #[method_id(boundarySupplementaryItems)]
        pub unsafe fn boundarySupplementaryItems(
            &self,
        ) -> Id<NSArray<NSCollectionLayoutBoundarySupplementaryItem>, Shared>;
        #[method(setBoundarySupplementaryItems:)]
        pub unsafe fn setBoundarySupplementaryItems(
            &self,
            boundarySupplementaryItems: &NSArray<NSCollectionLayoutBoundarySupplementaryItem>,
        );
        #[method(supplementariesFollowContentInsets)]
        pub unsafe fn supplementariesFollowContentInsets(&self) -> bool;
        #[method(setSupplementariesFollowContentInsets:)]
        pub unsafe fn setSupplementariesFollowContentInsets(
            &self,
            supplementariesFollowContentInsets: bool,
        );
        #[method(visibleItemsInvalidationHandler)]
        pub unsafe fn visibleItemsInvalidationHandler(
            &self,
        ) -> NSCollectionLayoutSectionVisibleItemsInvalidationHandler;
        #[method(setVisibleItemsInvalidationHandler:)]
        pub unsafe fn setVisibleItemsInvalidationHandler(
            &self,
            visibleItemsInvalidationHandler : NSCollectionLayoutSectionVisibleItemsInvalidationHandler,
        );
        #[method_id(decorationItems)]
        pub unsafe fn decorationItems(
            &self,
        ) -> Id<NSArray<NSCollectionLayoutDecorationItem>, Shared>;
        #[method(setDecorationItems:)]
        pub unsafe fn setDecorationItems(
            &self,
            decorationItems: &NSArray<NSCollectionLayoutDecorationItem>,
        );
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSCollectionLayoutItem;
    unsafe impl ClassType for NSCollectionLayoutItem {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSCollectionLayoutItem {
        #[method_id(itemWithLayoutSize:)]
        pub unsafe fn itemWithLayoutSize(layoutSize: &NSCollectionLayoutSize) -> Id<Self, Shared>;
        #[method_id(itemWithLayoutSize:supplementaryItems:)]
        pub unsafe fn itemWithLayoutSize_supplementaryItems(
            layoutSize: &NSCollectionLayoutSize,
            supplementaryItems: &NSArray<NSCollectionLayoutSupplementaryItem>,
        ) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
        #[method(contentInsets)]
        pub unsafe fn contentInsets(&self) -> NSDirectionalEdgeInsets;
        #[method(setContentInsets:)]
        pub unsafe fn setContentInsets(&self, contentInsets: NSDirectionalEdgeInsets);
        #[method_id(edgeSpacing)]
        pub unsafe fn edgeSpacing(&self) -> Option<Id<NSCollectionLayoutEdgeSpacing, Shared>>;
        #[method(setEdgeSpacing:)]
        pub unsafe fn setEdgeSpacing(&self, edgeSpacing: Option<&NSCollectionLayoutEdgeSpacing>);
        #[method_id(layoutSize)]
        pub unsafe fn layoutSize(&self) -> Id<NSCollectionLayoutSize, Shared>;
        #[method_id(supplementaryItems)]
        pub unsafe fn supplementaryItems(
            &self,
        ) -> Id<NSArray<NSCollectionLayoutSupplementaryItem>, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSCollectionLayoutGroupCustomItem;
    unsafe impl ClassType for NSCollectionLayoutGroupCustomItem {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSCollectionLayoutGroupCustomItem {
        #[method_id(customItemWithFrame:)]
        pub unsafe fn customItemWithFrame(frame: NSRect) -> Id<Self, Shared>;
        #[method_id(customItemWithFrame:zIndex:)]
        pub unsafe fn customItemWithFrame_zIndex(
            frame: NSRect,
            zIndex: NSInteger,
        ) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
        #[method(frame)]
        pub unsafe fn frame(&self) -> NSRect;
        #[method(zIndex)]
        pub unsafe fn zIndex(&self) -> NSInteger;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSCollectionLayoutGroup;
    unsafe impl ClassType for NSCollectionLayoutGroup {
        type Super = NSCollectionLayoutItem;
    }
);
extern_methods!(
    unsafe impl NSCollectionLayoutGroup {
        #[method_id(horizontalGroupWithLayoutSize:subitem:count:)]
        pub unsafe fn horizontalGroupWithLayoutSize_subitem_count(
            layoutSize: &NSCollectionLayoutSize,
            subitem: &NSCollectionLayoutItem,
            count: NSInteger,
        ) -> Id<Self, Shared>;
        #[method_id(horizontalGroupWithLayoutSize:subitems:)]
        pub unsafe fn horizontalGroupWithLayoutSize_subitems(
            layoutSize: &NSCollectionLayoutSize,
            subitems: &NSArray<NSCollectionLayoutItem>,
        ) -> Id<Self, Shared>;
        #[method_id(verticalGroupWithLayoutSize:subitem:count:)]
        pub unsafe fn verticalGroupWithLayoutSize_subitem_count(
            layoutSize: &NSCollectionLayoutSize,
            subitem: &NSCollectionLayoutItem,
            count: NSInteger,
        ) -> Id<Self, Shared>;
        #[method_id(verticalGroupWithLayoutSize:subitems:)]
        pub unsafe fn verticalGroupWithLayoutSize_subitems(
            layoutSize: &NSCollectionLayoutSize,
            subitems: &NSArray<NSCollectionLayoutItem>,
        ) -> Id<Self, Shared>;
        #[method_id(customGroupWithLayoutSize:itemProvider:)]
        pub unsafe fn customGroupWithLayoutSize_itemProvider(
            layoutSize: &NSCollectionLayoutSize,
            itemProvider: NSCollectionLayoutGroupCustomItemProvider,
        ) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
        #[method_id(supplementaryItems)]
        pub unsafe fn supplementaryItems(
            &self,
        ) -> Id<NSArray<NSCollectionLayoutSupplementaryItem>, Shared>;
        #[method(setSupplementaryItems:)]
        pub unsafe fn setSupplementaryItems(
            &self,
            supplementaryItems: &NSArray<NSCollectionLayoutSupplementaryItem>,
        );
        #[method_id(interItemSpacing)]
        pub unsafe fn interItemSpacing(&self) -> Option<Id<NSCollectionLayoutSpacing, Shared>>;
        #[method(setInterItemSpacing:)]
        pub unsafe fn setInterItemSpacing(
            &self,
            interItemSpacing: Option<&NSCollectionLayoutSpacing>,
        );
        #[method_id(subitems)]
        pub unsafe fn subitems(&self) -> Id<NSArray<NSCollectionLayoutItem>, Shared>;
        #[method_id(visualDescription)]
        pub unsafe fn visualDescription(&self) -> Id<NSString, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSCollectionLayoutDimension;
    unsafe impl ClassType for NSCollectionLayoutDimension {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSCollectionLayoutDimension {
        #[method_id(fractionalWidthDimension:)]
        pub unsafe fn fractionalWidthDimension(fractionalWidth: CGFloat) -> Id<Self, Shared>;
        #[method_id(fractionalHeightDimension:)]
        pub unsafe fn fractionalHeightDimension(fractionalHeight: CGFloat) -> Id<Self, Shared>;
        #[method_id(absoluteDimension:)]
        pub unsafe fn absoluteDimension(absoluteDimension: CGFloat) -> Id<Self, Shared>;
        #[method_id(estimatedDimension:)]
        pub unsafe fn estimatedDimension(estimatedDimension: CGFloat) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
        #[method(isFractionalWidth)]
        pub unsafe fn isFractionalWidth(&self) -> bool;
        #[method(isFractionalHeight)]
        pub unsafe fn isFractionalHeight(&self) -> bool;
        #[method(isAbsolute)]
        pub unsafe fn isAbsolute(&self) -> bool;
        #[method(isEstimated)]
        pub unsafe fn isEstimated(&self) -> bool;
        #[method(dimension)]
        pub unsafe fn dimension(&self) -> CGFloat;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSCollectionLayoutSize;
    unsafe impl ClassType for NSCollectionLayoutSize {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSCollectionLayoutSize {
        #[method_id(sizeWithWidthDimension:heightDimension:)]
        pub unsafe fn sizeWithWidthDimension_heightDimension(
            width: &NSCollectionLayoutDimension,
            height: &NSCollectionLayoutDimension,
        ) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
        #[method_id(widthDimension)]
        pub unsafe fn widthDimension(&self) -> Id<NSCollectionLayoutDimension, Shared>;
        #[method_id(heightDimension)]
        pub unsafe fn heightDimension(&self) -> Id<NSCollectionLayoutDimension, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSCollectionLayoutSpacing;
    unsafe impl ClassType for NSCollectionLayoutSpacing {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSCollectionLayoutSpacing {
        #[method_id(flexibleSpacing:)]
        pub unsafe fn flexibleSpacing(flexibleSpacing: CGFloat) -> Id<Self, Shared>;
        #[method_id(fixedSpacing:)]
        pub unsafe fn fixedSpacing(fixedSpacing: CGFloat) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
        #[method(spacing)]
        pub unsafe fn spacing(&self) -> CGFloat;
        #[method(isFlexibleSpacing)]
        pub unsafe fn isFlexibleSpacing(&self) -> bool;
        #[method(isFixedSpacing)]
        pub unsafe fn isFixedSpacing(&self) -> bool;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSCollectionLayoutEdgeSpacing;
    unsafe impl ClassType for NSCollectionLayoutEdgeSpacing {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSCollectionLayoutEdgeSpacing {
        #[method_id(spacingForLeading:top:trailing:bottom:)]
        pub unsafe fn spacingForLeading_top_trailing_bottom(
            leading: Option<&NSCollectionLayoutSpacing>,
            top: Option<&NSCollectionLayoutSpacing>,
            trailing: Option<&NSCollectionLayoutSpacing>,
            bottom: Option<&NSCollectionLayoutSpacing>,
        ) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
        #[method_id(leading)]
        pub unsafe fn leading(&self) -> Option<Id<NSCollectionLayoutSpacing, Shared>>;
        #[method_id(top)]
        pub unsafe fn top(&self) -> Option<Id<NSCollectionLayoutSpacing, Shared>>;
        #[method_id(trailing)]
        pub unsafe fn trailing(&self) -> Option<Id<NSCollectionLayoutSpacing, Shared>>;
        #[method_id(bottom)]
        pub unsafe fn bottom(&self) -> Option<Id<NSCollectionLayoutSpacing, Shared>>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSCollectionLayoutSupplementaryItem;
    unsafe impl ClassType for NSCollectionLayoutSupplementaryItem {
        type Super = NSCollectionLayoutItem;
    }
);
extern_methods!(
    unsafe impl NSCollectionLayoutSupplementaryItem {
        #[method_id(supplementaryItemWithLayoutSize:elementKind:containerAnchor:)]
        pub unsafe fn supplementaryItemWithLayoutSize_elementKind_containerAnchor(
            layoutSize: &NSCollectionLayoutSize,
            elementKind: &NSString,
            containerAnchor: &NSCollectionLayoutAnchor,
        ) -> Id<Self, Shared>;
        #[method_id(supplementaryItemWithLayoutSize:elementKind:containerAnchor:itemAnchor:)]
        pub unsafe fn supplementaryItemWithLayoutSize_elementKind_containerAnchor_itemAnchor(
            layoutSize: &NSCollectionLayoutSize,
            elementKind: &NSString,
            containerAnchor: &NSCollectionLayoutAnchor,
            itemAnchor: &NSCollectionLayoutAnchor,
        ) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
        #[method(zIndex)]
        pub unsafe fn zIndex(&self) -> NSInteger;
        #[method(setZIndex:)]
        pub unsafe fn setZIndex(&self, zIndex: NSInteger);
        #[method_id(elementKind)]
        pub unsafe fn elementKind(&self) -> Id<NSString, Shared>;
        #[method_id(containerAnchor)]
        pub unsafe fn containerAnchor(&self) -> Id<NSCollectionLayoutAnchor, Shared>;
        #[method_id(itemAnchor)]
        pub unsafe fn itemAnchor(&self) -> Option<Id<NSCollectionLayoutAnchor, Shared>>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSCollectionLayoutBoundarySupplementaryItem;
    unsafe impl ClassType for NSCollectionLayoutBoundarySupplementaryItem {
        type Super = NSCollectionLayoutSupplementaryItem;
    }
);
extern_methods!(
    unsafe impl NSCollectionLayoutBoundarySupplementaryItem {
        #[method_id(boundarySupplementaryItemWithLayoutSize:elementKind:alignment:)]
        pub unsafe fn boundarySupplementaryItemWithLayoutSize_elementKind_alignment(
            layoutSize: &NSCollectionLayoutSize,
            elementKind: &NSString,
            alignment: NSRectAlignment,
        ) -> Id<Self, Shared>;
        #[method_id(boundarySupplementaryItemWithLayoutSize:elementKind:alignment:absoluteOffset:)]
        pub unsafe fn boundarySupplementaryItemWithLayoutSize_elementKind_alignment_absoluteOffset(
            layoutSize: &NSCollectionLayoutSize,
            elementKind: &NSString,
            alignment: NSRectAlignment,
            absoluteOffset: NSPoint,
        ) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
        #[method(extendsBoundary)]
        pub unsafe fn extendsBoundary(&self) -> bool;
        #[method(setExtendsBoundary:)]
        pub unsafe fn setExtendsBoundary(&self, extendsBoundary: bool);
        #[method(pinToVisibleBounds)]
        pub unsafe fn pinToVisibleBounds(&self) -> bool;
        #[method(setPinToVisibleBounds:)]
        pub unsafe fn setPinToVisibleBounds(&self, pinToVisibleBounds: bool);
        #[method(alignment)]
        pub unsafe fn alignment(&self) -> NSRectAlignment;
        #[method(offset)]
        pub unsafe fn offset(&self) -> NSPoint;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSCollectionLayoutDecorationItem;
    unsafe impl ClassType for NSCollectionLayoutDecorationItem {
        type Super = NSCollectionLayoutItem;
    }
);
extern_methods!(
    unsafe impl NSCollectionLayoutDecorationItem {
        #[method_id(backgroundDecorationItemWithElementKind:)]
        pub unsafe fn backgroundDecorationItemWithElementKind(
            elementKind: &NSString,
        ) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
        #[method(zIndex)]
        pub unsafe fn zIndex(&self) -> NSInteger;
        #[method(setZIndex:)]
        pub unsafe fn setZIndex(&self, zIndex: NSInteger);
        #[method_id(elementKind)]
        pub unsafe fn elementKind(&self) -> Id<NSString, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSCollectionLayoutAnchor;
    unsafe impl ClassType for NSCollectionLayoutAnchor {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSCollectionLayoutAnchor {
        #[method_id(layoutAnchorWithEdges:)]
        pub unsafe fn layoutAnchorWithEdges(edges: NSDirectionalRectEdge) -> Id<Self, Shared>;
        #[method_id(layoutAnchorWithEdges:absoluteOffset:)]
        pub unsafe fn layoutAnchorWithEdges_absoluteOffset(
            edges: NSDirectionalRectEdge,
            absoluteOffset: NSPoint,
        ) -> Id<Self, Shared>;
        #[method_id(layoutAnchorWithEdges:fractionalOffset:)]
        pub unsafe fn layoutAnchorWithEdges_fractionalOffset(
            edges: NSDirectionalRectEdge,
            fractionalOffset: NSPoint,
        ) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
        #[method(edges)]
        pub unsafe fn edges(&self) -> NSDirectionalRectEdge;
        #[method(offset)]
        pub unsafe fn offset(&self) -> NSPoint;
        #[method(isAbsoluteOffset)]
        pub unsafe fn isAbsoluteOffset(&self) -> bool;
        #[method(isFractionalOffset)]
        pub unsafe fn isFractionalOffset(&self) -> bool;
    }
);
pub type NSCollectionLayoutContainer = NSObject;
pub type NSCollectionLayoutEnvironment = NSObject;
pub type NSCollectionLayoutVisibleItem = NSObject;
