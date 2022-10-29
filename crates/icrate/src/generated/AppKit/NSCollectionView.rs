#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSCollectionViewSupplementaryElementKind = NSString;
pub type NSCollectionViewElement = NSObject;
pub type NSCollectionViewSectionHeaderView = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSCollectionViewItem;
    unsafe impl ClassType for NSCollectionViewItem {
        type Super = NSViewController;
    }
);
extern_methods!(
    unsafe impl NSCollectionViewItem {
        #[method_id(collectionView)]
        pub unsafe fn collectionView(&self) -> Option<Id<NSCollectionView, Shared>>;
        #[method(isSelected)]
        pub unsafe fn isSelected(&self) -> bool;
        #[method(setSelected:)]
        pub unsafe fn setSelected(&self, selected: bool);
        #[method(highlightState)]
        pub unsafe fn highlightState(&self) -> NSCollectionViewItemHighlightState;
        #[method(setHighlightState:)]
        pub unsafe fn setHighlightState(&self, highlightState: NSCollectionViewItemHighlightState);
        #[method_id(imageView)]
        pub unsafe fn imageView(&self) -> Option<Id<NSImageView, Shared>>;
        #[method(setImageView:)]
        pub unsafe fn setImageView(&self, imageView: Option<&NSImageView>);
        #[method_id(textField)]
        pub unsafe fn textField(&self) -> Option<Id<NSTextField, Shared>>;
        #[method(setTextField:)]
        pub unsafe fn setTextField(&self, textField: Option<&NSTextField>);
        #[method_id(draggingImageComponents)]
        pub unsafe fn draggingImageComponents(
            &self,
        ) -> Id<NSArray<NSDraggingImageComponent>, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSCollectionView;
    unsafe impl ClassType for NSCollectionView {
        type Super = NSView;
    }
);
extern_methods!(
    unsafe impl NSCollectionView {
        #[method_id(dataSource)]
        pub unsafe fn dataSource(&self) -> Option<Id<NSCollectionViewDataSource, Shared>>;
        #[method(setDataSource:)]
        pub unsafe fn setDataSource(&self, dataSource: Option<&NSCollectionViewDataSource>);
        #[method_id(prefetchDataSource)]
        pub unsafe fn prefetchDataSource(&self) -> Option<Id<NSCollectionViewPrefetching, Shared>>;
        #[method(setPrefetchDataSource:)]
        pub unsafe fn setPrefetchDataSource(
            &self,
            prefetchDataSource: Option<&NSCollectionViewPrefetching>,
        );
        #[method_id(content)]
        pub unsafe fn content(&self) -> Id<NSArray<Object>, Shared>;
        #[method(setContent:)]
        pub unsafe fn setContent(&self, content: &NSArray<Object>);
        #[method(reloadData)]
        pub unsafe fn reloadData(&self);
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSCollectionViewDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSCollectionViewDelegate>);
        #[method_id(backgroundView)]
        pub unsafe fn backgroundView(&self) -> Option<Id<NSView, Shared>>;
        #[method(setBackgroundView:)]
        pub unsafe fn setBackgroundView(&self, backgroundView: Option<&NSView>);
        #[method(backgroundViewScrollsWithContent)]
        pub unsafe fn backgroundViewScrollsWithContent(&self) -> bool;
        #[method(setBackgroundViewScrollsWithContent:)]
        pub unsafe fn setBackgroundViewScrollsWithContent(
            &self,
            backgroundViewScrollsWithContent: bool,
        );
        #[method_id(collectionViewLayout)]
        pub unsafe fn collectionViewLayout(&self) -> Option<Id<NSCollectionViewLayout, Shared>>;
        #[method(setCollectionViewLayout:)]
        pub unsafe fn setCollectionViewLayout(
            &self,
            collectionViewLayout: Option<&NSCollectionViewLayout>,
        );
        #[method_id(layoutAttributesForItemAtIndexPath:)]
        pub unsafe fn layoutAttributesForItemAtIndexPath(
            &self,
            indexPath: &NSIndexPath,
        ) -> Option<Id<NSCollectionViewLayoutAttributes, Shared>>;
        #[method_id(layoutAttributesForSupplementaryElementOfKind:atIndexPath:)]
        pub unsafe fn layoutAttributesForSupplementaryElementOfKind_atIndexPath(
            &self,
            kind: &NSCollectionViewSupplementaryElementKind,
            indexPath: &NSIndexPath,
        ) -> Option<Id<NSCollectionViewLayoutAttributes, Shared>>;
        #[method(frameForItemAtIndex:)]
        pub unsafe fn frameForItemAtIndex(&self, index: NSUInteger) -> NSRect;
        #[method(frameForItemAtIndex:withNumberOfItems:)]
        pub unsafe fn frameForItemAtIndex_withNumberOfItems(
            &self,
            index: NSUInteger,
            numberOfItems: NSUInteger,
        ) -> NSRect;
        #[method_id(backgroundColors)]
        pub unsafe fn backgroundColors(&self) -> Id<NSArray<NSColor>, Shared>;
        #[method(setBackgroundColors:)]
        pub unsafe fn setBackgroundColors(&self, backgroundColors: Option<&NSArray<NSColor>>);
        #[method(numberOfSections)]
        pub unsafe fn numberOfSections(&self) -> NSInteger;
        #[method(numberOfItemsInSection:)]
        pub unsafe fn numberOfItemsInSection(&self, section: NSInteger) -> NSInteger;
        #[method(isFirstResponder)]
        pub unsafe fn isFirstResponder(&self) -> bool;
        #[method(isSelectable)]
        pub unsafe fn isSelectable(&self) -> bool;
        #[method(setSelectable:)]
        pub unsafe fn setSelectable(&self, selectable: bool);
        #[method(allowsEmptySelection)]
        pub unsafe fn allowsEmptySelection(&self) -> bool;
        #[method(setAllowsEmptySelection:)]
        pub unsafe fn setAllowsEmptySelection(&self, allowsEmptySelection: bool);
        #[method(allowsMultipleSelection)]
        pub unsafe fn allowsMultipleSelection(&self) -> bool;
        #[method(setAllowsMultipleSelection:)]
        pub unsafe fn setAllowsMultipleSelection(&self, allowsMultipleSelection: bool);
        #[method_id(selectionIndexes)]
        pub unsafe fn selectionIndexes(&self) -> Id<NSIndexSet, Shared>;
        #[method(setSelectionIndexes:)]
        pub unsafe fn setSelectionIndexes(&self, selectionIndexes: &NSIndexSet);
        #[method_id(selectionIndexPaths)]
        pub unsafe fn selectionIndexPaths(&self) -> Id<NSSet<NSIndexPath>, Shared>;
        #[method(setSelectionIndexPaths:)]
        pub unsafe fn setSelectionIndexPaths(&self, selectionIndexPaths: &NSSet<NSIndexPath>);
        #[method(selectItemsAtIndexPaths:scrollPosition:)]
        pub unsafe fn selectItemsAtIndexPaths_scrollPosition(
            &self,
            indexPaths: &NSSet<NSIndexPath>,
            scrollPosition: NSCollectionViewScrollPosition,
        );
        #[method(deselectItemsAtIndexPaths:)]
        pub unsafe fn deselectItemsAtIndexPaths(&self, indexPaths: &NSSet<NSIndexPath>);
        #[method(selectAll:)]
        pub unsafe fn selectAll(&self, sender: Option<&Object>);
        #[method(deselectAll:)]
        pub unsafe fn deselectAll(&self, sender: Option<&Object>);
        #[method(registerClass:forItemWithIdentifier:)]
        pub unsafe fn registerClass_forItemWithIdentifier(
            &self,
            itemClass: Option<&Class>,
            identifier: &NSUserInterfaceItemIdentifier,
        );
        #[method(registerNib:forItemWithIdentifier:)]
        pub unsafe fn registerNib_forItemWithIdentifier(
            &self,
            nib: Option<&NSNib>,
            identifier: &NSUserInterfaceItemIdentifier,
        );
        #[method(registerClass:forSupplementaryViewOfKind:withIdentifier:)]
        pub unsafe fn registerClass_forSupplementaryViewOfKind_withIdentifier(
            &self,
            viewClass: Option<&Class>,
            kind: &NSCollectionViewSupplementaryElementKind,
            identifier: &NSUserInterfaceItemIdentifier,
        );
        #[method(registerNib:forSupplementaryViewOfKind:withIdentifier:)]
        pub unsafe fn registerNib_forSupplementaryViewOfKind_withIdentifier(
            &self,
            nib: Option<&NSNib>,
            kind: &NSCollectionViewSupplementaryElementKind,
            identifier: &NSUserInterfaceItemIdentifier,
        );
        #[method_id(makeItemWithIdentifier:forIndexPath:)]
        pub unsafe fn makeItemWithIdentifier_forIndexPath(
            &self,
            identifier: &NSUserInterfaceItemIdentifier,
            indexPath: &NSIndexPath,
        ) -> Id<NSCollectionViewItem, Shared>;
        #[method_id(makeSupplementaryViewOfKind:withIdentifier:forIndexPath:)]
        pub unsafe fn makeSupplementaryViewOfKind_withIdentifier_forIndexPath(
            &self,
            elementKind: &NSCollectionViewSupplementaryElementKind,
            identifier: &NSUserInterfaceItemIdentifier,
            indexPath: &NSIndexPath,
        ) -> Id<NSView, Shared>;
        #[method_id(itemAtIndex:)]
        pub unsafe fn itemAtIndex(
            &self,
            index: NSUInteger,
        ) -> Option<Id<NSCollectionViewItem, Shared>>;
        #[method_id(itemAtIndexPath:)]
        pub unsafe fn itemAtIndexPath(
            &self,
            indexPath: &NSIndexPath,
        ) -> Option<Id<NSCollectionViewItem, Shared>>;
        #[method_id(visibleItems)]
        pub unsafe fn visibleItems(&self) -> Id<NSArray<NSCollectionViewItem>, Shared>;
        #[method_id(indexPathsForVisibleItems)]
        pub unsafe fn indexPathsForVisibleItems(&self) -> Id<NSSet<NSIndexPath>, Shared>;
        #[method_id(indexPathForItem:)]
        pub unsafe fn indexPathForItem(
            &self,
            item: &NSCollectionViewItem,
        ) -> Option<Id<NSIndexPath, Shared>>;
        #[method_id(indexPathForItemAtPoint:)]
        pub unsafe fn indexPathForItemAtPoint(
            &self,
            point: NSPoint,
        ) -> Option<Id<NSIndexPath, Shared>>;
        #[method_id(supplementaryViewForElementKind:atIndexPath:)]
        pub unsafe fn supplementaryViewForElementKind_atIndexPath(
            &self,
            elementKind: &NSCollectionViewSupplementaryElementKind,
            indexPath: &NSIndexPath,
        ) -> Option<Id<TodoProtocols, Shared>>;
        #[method_id(visibleSupplementaryViewsOfKind:)]
        pub unsafe fn visibleSupplementaryViewsOfKind(
            &self,
            elementKind: &NSCollectionViewSupplementaryElementKind,
        ) -> Id<NSArray<TodoProtocols>, Shared>;
        #[method_id(indexPathsForVisibleSupplementaryElementsOfKind:)]
        pub unsafe fn indexPathsForVisibleSupplementaryElementsOfKind(
            &self,
            elementKind: &NSCollectionViewSupplementaryElementKind,
        ) -> Id<NSSet<NSIndexPath>, Shared>;
        #[method(insertSections:)]
        pub unsafe fn insertSections(&self, sections: &NSIndexSet);
        #[method(deleteSections:)]
        pub unsafe fn deleteSections(&self, sections: &NSIndexSet);
        #[method(reloadSections:)]
        pub unsafe fn reloadSections(&self, sections: &NSIndexSet);
        #[method(moveSection:toSection:)]
        pub unsafe fn moveSection_toSection(&self, section: NSInteger, newSection: NSInteger);
        #[method(insertItemsAtIndexPaths:)]
        pub unsafe fn insertItemsAtIndexPaths(&self, indexPaths: &NSSet<NSIndexPath>);
        #[method(deleteItemsAtIndexPaths:)]
        pub unsafe fn deleteItemsAtIndexPaths(&self, indexPaths: &NSSet<NSIndexPath>);
        #[method(reloadItemsAtIndexPaths:)]
        pub unsafe fn reloadItemsAtIndexPaths(&self, indexPaths: &NSSet<NSIndexPath>);
        #[method(moveItemAtIndexPath:toIndexPath:)]
        pub unsafe fn moveItemAtIndexPath_toIndexPath(
            &self,
            indexPath: &NSIndexPath,
            newIndexPath: &NSIndexPath,
        );
        #[method(performBatchUpdates:completionHandler:)]
        pub unsafe fn performBatchUpdates_completionHandler(
            &self,
            updates: TodoBlock,
            completionHandler: TodoBlock,
        );
        #[method(toggleSectionCollapse:)]
        pub unsafe fn toggleSectionCollapse(&self, sender: &Object);
        #[method(scrollToItemsAtIndexPaths:scrollPosition:)]
        pub unsafe fn scrollToItemsAtIndexPaths_scrollPosition(
            &self,
            indexPaths: &NSSet<NSIndexPath>,
            scrollPosition: NSCollectionViewScrollPosition,
        );
        #[method(setDraggingSourceOperationMask:forLocal:)]
        pub unsafe fn setDraggingSourceOperationMask_forLocal(
            &self,
            dragOperationMask: NSDragOperation,
            localDestination: bool,
        );
        #[method_id(draggingImageForItemsAtIndexPaths:withEvent:offset:)]
        pub unsafe fn draggingImageForItemsAtIndexPaths_withEvent_offset(
            &self,
            indexPaths: &NSSet<NSIndexPath>,
            event: &NSEvent,
            dragImageOffset: NSPointPointer,
        ) -> Id<NSImage, Shared>;
        #[method_id(draggingImageForItemsAtIndexes:withEvent:offset:)]
        pub unsafe fn draggingImageForItemsAtIndexes_withEvent_offset(
            &self,
            indexes: &NSIndexSet,
            event: &NSEvent,
            dragImageOffset: NSPointPointer,
        ) -> Id<NSImage, Shared>;
    }
);
pub type NSCollectionViewDataSource = NSObject;
pub type NSCollectionViewPrefetching = NSObject;
pub type NSCollectionViewDelegate = NSObject;
extern_methods!(
    #[doc = "NSCollectionViewAdditions"]
    unsafe impl NSIndexPath {
        #[method_id(indexPathForItem:inSection:)]
        pub unsafe fn indexPathForItem_inSection(
            item: NSInteger,
            section: NSInteger,
        ) -> Id<NSIndexPath, Shared>;
        #[method(item)]
        pub unsafe fn item(&self) -> NSInteger;
        #[method(section)]
        pub unsafe fn section(&self) -> NSInteger;
    }
);
extern_methods!(
    #[doc = "NSCollectionViewAdditions"]
    unsafe impl NSSet {
        #[method_id(setWithCollectionViewIndexPath:)]
        pub unsafe fn setWithCollectionViewIndexPath(indexPath: &NSIndexPath) -> Id<Self, Shared>;
        #[method_id(setWithCollectionViewIndexPaths:)]
        pub unsafe fn setWithCollectionViewIndexPaths(
            indexPaths: &NSArray<NSIndexPath>,
        ) -> Id<Self, Shared>;
        #[method(enumerateIndexPathsWithOptions:usingBlock:)]
        pub unsafe fn enumerateIndexPathsWithOptions_usingBlock(
            &self,
            opts: NSEnumerationOptions,
            block: TodoBlock,
        );
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSCollectionView {
        #[method_id(newItemForRepresentedObject:)]
        pub unsafe fn newItemForRepresentedObject(
            &self,
            object: &Object,
        ) -> Id<NSCollectionViewItem, Shared>;
        #[method_id(itemPrototype)]
        pub unsafe fn itemPrototype(&self) -> Option<Id<NSCollectionViewItem, Shared>>;
        #[method(setItemPrototype:)]
        pub unsafe fn setItemPrototype(&self, itemPrototype: Option<&NSCollectionViewItem>);
        #[method(maxNumberOfRows)]
        pub unsafe fn maxNumberOfRows(&self) -> NSUInteger;
        #[method(setMaxNumberOfRows:)]
        pub unsafe fn setMaxNumberOfRows(&self, maxNumberOfRows: NSUInteger);
        #[method(maxNumberOfColumns)]
        pub unsafe fn maxNumberOfColumns(&self) -> NSUInteger;
        #[method(setMaxNumberOfColumns:)]
        pub unsafe fn setMaxNumberOfColumns(&self, maxNumberOfColumns: NSUInteger);
        #[method(minItemSize)]
        pub unsafe fn minItemSize(&self) -> NSSize;
        #[method(setMinItemSize:)]
        pub unsafe fn setMinItemSize(&self, minItemSize: NSSize);
        #[method(maxItemSize)]
        pub unsafe fn maxItemSize(&self) -> NSSize;
        #[method(setMaxItemSize:)]
        pub unsafe fn setMaxItemSize(&self, maxItemSize: NSSize);
    }
);