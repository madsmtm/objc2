#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTextSelectionNavigation;
    unsafe impl ClassType for NSTextSelectionNavigation {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSTextSelectionNavigation {
        #[method_id(initWithDataSource:)]
        pub unsafe fn initWithDataSource(
            &self,
            dataSource: &NSTextSelectionDataSource,
        ) -> Id<Self, Shared>;
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(textSelectionDataSource)]
        pub unsafe fn textSelectionDataSource(
            &self,
        ) -> Option<Id<NSTextSelectionDataSource, Shared>>;
        #[method(allowsNonContiguousRanges)]
        pub unsafe fn allowsNonContiguousRanges(&self) -> bool;
        #[method(setAllowsNonContiguousRanges:)]
        pub unsafe fn setAllowsNonContiguousRanges(&self, allowsNonContiguousRanges: bool);
        #[method(rotatesCoordinateSystemForLayoutOrientation)]
        pub unsafe fn rotatesCoordinateSystemForLayoutOrientation(&self) -> bool;
        #[method(setRotatesCoordinateSystemForLayoutOrientation:)]
        pub unsafe fn setRotatesCoordinateSystemForLayoutOrientation(
            &self,
            rotatesCoordinateSystemForLayoutOrientation: bool,
        );
        #[method(flushLayoutCache)]
        pub unsafe fn flushLayoutCache(&self);
        #[method_id(destinationSelectionForTextSelection:direction:destination:extending:confined:)]
        pub unsafe fn destinationSelectionForTextSelection_direction_destination_extending_confined(
            &self,
            textSelection: &NSTextSelection,
            direction: NSTextSelectionNavigationDirection,
            destination: NSTextSelectionNavigationDestination,
            extending: bool,
            confined: bool,
        ) -> Option<Id<NSTextSelection, Shared>>;
        #[method_id(textSelectionsInteractingAtPoint:inContainerAtLocation:anchors:modifiers:selecting:bounds:)]
        pub unsafe fn textSelectionsInteractingAtPoint_inContainerAtLocation_anchors_modifiers_selecting_bounds(
            &self,
            point: CGPoint,
            containerLocation: &NSTextLocation,
            anchors: &NSArray<NSTextSelection>,
            modifiers: NSTextSelectionNavigationModifier,
            selecting: bool,
            bounds: CGRect,
        ) -> Id<NSArray<NSTextSelection>, Shared>;
        #[method_id(textSelectionForSelectionGranularity:enclosingTextSelection:)]
        pub unsafe fn textSelectionForSelectionGranularity_enclosingTextSelection(
            &self,
            selectionGranularity: NSTextSelectionGranularity,
            textSelection: &NSTextSelection,
        ) -> Id<NSTextSelection, Shared>;
        #[method_id(textSelectionForSelectionGranularity:enclosingPoint:inContainerAtLocation:)]
        pub unsafe fn textSelectionForSelectionGranularity_enclosingPoint_inContainerAtLocation(
            &self,
            selectionGranularity: NSTextSelectionGranularity,
            point: CGPoint,
            location: &NSTextLocation,
        ) -> Option<Id<NSTextSelection, Shared>>;
        #[method_id(resolvedInsertionLocationForTextSelection:writingDirection:)]
        pub unsafe fn resolvedInsertionLocationForTextSelection_writingDirection(
            &self,
            textSelection: &NSTextSelection,
            writingDirection: NSTextSelectionNavigationWritingDirection,
        ) -> Option<Id<NSTextLocation, Shared>>;
        #[method_id(deletionRangesForTextSelection:direction:destination:allowsDecomposition:)]
        pub unsafe fn deletionRangesForTextSelection_direction_destination_allowsDecomposition(
            &self,
            textSelection: &NSTextSelection,
            direction: NSTextSelectionNavigationDirection,
            destination: NSTextSelectionNavigationDestination,
            allowsDecomposition: bool,
        ) -> Id<NSArray<NSTextRange>, Shared>;
    }
);
pub type NSTextSelectionDataSource = NSObject;
