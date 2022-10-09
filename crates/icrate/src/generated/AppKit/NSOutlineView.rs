use super::__exported::NSButtonCell;
use super::__exported::NSNotification;
use super::__exported::NSString;
use super::__exported::NSTableColumn;
use super::__exported::NSTableHeaderView;
use super::__exported::NSTableView;
use super::__exported::NSTintConfiguration;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSTableView::*;
use crate::CoreFoundation::generated::CFDictionary::*;
use crate::Foundation::generated::NSArray::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSOutlineView;
    unsafe impl ClassType for NSOutlineView {
        type Super = NSTableView;
    }
);
extern_methods!(
    unsafe impl NSOutlineView {
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSOutlineViewDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSOutlineViewDelegate>);
        #[method_id(dataSource)]
        pub unsafe fn dataSource(&self) -> Option<Id<NSOutlineViewDataSource, Shared>>;
        #[method(setDataSource:)]
        pub unsafe fn setDataSource(&self, dataSource: Option<&NSOutlineViewDataSource>);
        #[method_id(outlineTableColumn)]
        pub unsafe fn outlineTableColumn(&self) -> Option<Id<NSTableColumn, Shared>>;
        #[method(setOutlineTableColumn:)]
        pub unsafe fn setOutlineTableColumn(&self, outlineTableColumn: Option<&NSTableColumn>);
        #[method(isExpandable:)]
        pub unsafe fn isExpandable(&self, item: Option<&Object>) -> bool;
        #[method(numberOfChildrenOfItem:)]
        pub unsafe fn numberOfChildrenOfItem(&self, item: Option<&Object>) -> NSInteger;
        #[method_id(child:ofItem:)]
        pub unsafe fn child_ofItem(
            &self,
            index: NSInteger,
            item: Option<&Object>,
        ) -> Option<Id<Object, Shared>>;
        #[method(expandItem:expandChildren:)]
        pub unsafe fn expandItem_expandChildren(&self, item: Option<&Object>, expandChildren: bool);
        #[method(expandItem:)]
        pub unsafe fn expandItem(&self, item: Option<&Object>);
        #[method(collapseItem:collapseChildren:)]
        pub unsafe fn collapseItem_collapseChildren(
            &self,
            item: Option<&Object>,
            collapseChildren: bool,
        );
        #[method(collapseItem:)]
        pub unsafe fn collapseItem(&self, item: Option<&Object>);
        #[method(reloadItem:reloadChildren:)]
        pub unsafe fn reloadItem_reloadChildren(&self, item: Option<&Object>, reloadChildren: bool);
        #[method(reloadItem:)]
        pub unsafe fn reloadItem(&self, item: Option<&Object>);
        #[method_id(parentForItem:)]
        pub unsafe fn parentForItem(&self, item: Option<&Object>) -> Option<Id<Object, Shared>>;
        #[method(childIndexForItem:)]
        pub unsafe fn childIndexForItem(&self, item: &Object) -> NSInteger;
        #[method_id(itemAtRow:)]
        pub unsafe fn itemAtRow(&self, row: NSInteger) -> Option<Id<Object, Shared>>;
        #[method(rowForItem:)]
        pub unsafe fn rowForItem(&self, item: Option<&Object>) -> NSInteger;
        #[method(levelForItem:)]
        pub unsafe fn levelForItem(&self, item: Option<&Object>) -> NSInteger;
        #[method(levelForRow:)]
        pub unsafe fn levelForRow(&self, row: NSInteger) -> NSInteger;
        #[method(isItemExpanded:)]
        pub unsafe fn isItemExpanded(&self, item: Option<&Object>) -> bool;
        #[method(indentationPerLevel)]
        pub unsafe fn indentationPerLevel(&self) -> CGFloat;
        #[method(setIndentationPerLevel:)]
        pub unsafe fn setIndentationPerLevel(&self, indentationPerLevel: CGFloat);
        #[method(indentationMarkerFollowsCell)]
        pub unsafe fn indentationMarkerFollowsCell(&self) -> bool;
        #[method(setIndentationMarkerFollowsCell:)]
        pub unsafe fn setIndentationMarkerFollowsCell(&self, indentationMarkerFollowsCell: bool);
        #[method(autoresizesOutlineColumn)]
        pub unsafe fn autoresizesOutlineColumn(&self) -> bool;
        #[method(setAutoresizesOutlineColumn:)]
        pub unsafe fn setAutoresizesOutlineColumn(&self, autoresizesOutlineColumn: bool);
        #[method(frameOfOutlineCellAtRow:)]
        pub unsafe fn frameOfOutlineCellAtRow(&self, row: NSInteger) -> NSRect;
        #[method(setDropItem:dropChildIndex:)]
        pub unsafe fn setDropItem_dropChildIndex(&self, item: Option<&Object>, index: NSInteger);
        #[method(shouldCollapseAutoExpandedItemsForDeposited:)]
        pub unsafe fn shouldCollapseAutoExpandedItemsForDeposited(&self, deposited: bool) -> bool;
        #[method(autosaveExpandedItems)]
        pub unsafe fn autosaveExpandedItems(&self) -> bool;
        #[method(setAutosaveExpandedItems:)]
        pub unsafe fn setAutosaveExpandedItems(&self, autosaveExpandedItems: bool);
        #[method(insertItemsAtIndexes:inParent:withAnimation:)]
        pub unsafe fn insertItemsAtIndexes_inParent_withAnimation(
            &self,
            indexes: &NSIndexSet,
            parent: Option<&Object>,
            animationOptions: NSTableViewAnimationOptions,
        );
        #[method(removeItemsAtIndexes:inParent:withAnimation:)]
        pub unsafe fn removeItemsAtIndexes_inParent_withAnimation(
            &self,
            indexes: &NSIndexSet,
            parent: Option<&Object>,
            animationOptions: NSTableViewAnimationOptions,
        );
        #[method(moveItemAtIndex:inParent:toIndex:inParent:)]
        pub unsafe fn moveItemAtIndex_inParent_toIndex_inParent(
            &self,
            fromIndex: NSInteger,
            oldParent: Option<&Object>,
            toIndex: NSInteger,
            newParent: Option<&Object>,
        );
        #[method(insertRowsAtIndexes:withAnimation:)]
        pub unsafe fn insertRowsAtIndexes_withAnimation(
            &self,
            indexes: &NSIndexSet,
            animationOptions: NSTableViewAnimationOptions,
        );
        #[method(removeRowsAtIndexes:withAnimation:)]
        pub unsafe fn removeRowsAtIndexes_withAnimation(
            &self,
            indexes: &NSIndexSet,
            animationOptions: NSTableViewAnimationOptions,
        );
        #[method(moveRowAtIndex:toIndex:)]
        pub unsafe fn moveRowAtIndex_toIndex(&self, oldIndex: NSInteger, newIndex: NSInteger);
        #[method(userInterfaceLayoutDirection)]
        pub unsafe fn userInterfaceLayoutDirection(&self) -> NSUserInterfaceLayoutDirection;
        #[method(setUserInterfaceLayoutDirection:)]
        pub unsafe fn setUserInterfaceLayoutDirection(
            &self,
            userInterfaceLayoutDirection: NSUserInterfaceLayoutDirection,
        );
        #[method(stronglyReferencesItems)]
        pub unsafe fn stronglyReferencesItems(&self) -> bool;
        #[method(setStronglyReferencesItems:)]
        pub unsafe fn setStronglyReferencesItems(&self, stronglyReferencesItems: bool);
    }
);
pub type NSOutlineViewDataSource = NSObject;
pub type NSOutlineViewDelegate = NSObject;
