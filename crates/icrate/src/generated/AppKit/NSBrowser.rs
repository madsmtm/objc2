//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

pub static NSAppKitVersionNumberWithContinuousScrollingBrowser: NSAppKitVersion = 680.0;

pub static NSAppKitVersionNumberWithColumnResizingBrowser: NSAppKitVersion = 685.0;

pub type NSBrowserColumnsAutosaveName = NSString;

pub type NSBrowserColumnResizingType = NSUInteger;
pub const NSBrowserNoColumnResizing: NSBrowserColumnResizingType = 0;
pub const NSBrowserAutoColumnResizing: NSBrowserColumnResizingType = 1;
pub const NSBrowserUserColumnResizing: NSBrowserColumnResizingType = 2;

pub type NSBrowserDropOperation = NSUInteger;
pub const NSBrowserDropOn: NSBrowserDropOperation = 0;
pub const NSBrowserDropAbove: NSBrowserDropOperation = 1;

extern_class!(
    #[derive(Debug)]
    pub struct NSBrowser;

    unsafe impl ClassType for NSBrowser {
        type Super = NSControl;
    }
);

extern_methods!(
    unsafe impl NSBrowser {
        #[method(cellClass)]
        pub unsafe fn cellClass() -> &'static Class;

        #[method(loadColumnZero)]
        pub unsafe fn loadColumnZero(&self);

        #[method(isLoaded)]
        pub unsafe fn isLoaded(&self) -> bool;

        #[method(doubleAction)]
        pub unsafe fn doubleAction(&self) -> Option<Sel>;

        #[method(setDoubleAction:)]
        pub unsafe fn setDoubleAction(&self, doubleAction: Option<Sel>);

        #[method(setCellClass:)]
        pub unsafe fn setCellClass(&self, factoryId: &Class);

        #[method_id(@__retain_semantics Other cellPrototype)]
        pub unsafe fn cellPrototype(&self) -> Option<Id<Object, Shared>>;

        #[method(setCellPrototype:)]
        pub unsafe fn setCellPrototype(&self, cellPrototype: Option<&Object>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSBrowserDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSBrowserDelegate>);

        #[method(reusesColumns)]
        pub unsafe fn reusesColumns(&self) -> bool;

        #[method(setReusesColumns:)]
        pub unsafe fn setReusesColumns(&self, reusesColumns: bool);

        #[method(hasHorizontalScroller)]
        pub unsafe fn hasHorizontalScroller(&self) -> bool;

        #[method(setHasHorizontalScroller:)]
        pub unsafe fn setHasHorizontalScroller(&self, hasHorizontalScroller: bool);

        #[method(autohidesScroller)]
        pub unsafe fn autohidesScroller(&self) -> bool;

        #[method(setAutohidesScroller:)]
        pub unsafe fn setAutohidesScroller(&self, autohidesScroller: bool);

        #[method(separatesColumns)]
        pub unsafe fn separatesColumns(&self) -> bool;

        #[method(setSeparatesColumns:)]
        pub unsafe fn setSeparatesColumns(&self, separatesColumns: bool);

        #[method(isTitled)]
        pub unsafe fn isTitled(&self) -> bool;

        #[method(setTitled:)]
        pub unsafe fn setTitled(&self, titled: bool);

        #[method(minColumnWidth)]
        pub unsafe fn minColumnWidth(&self) -> CGFloat;

        #[method(setMinColumnWidth:)]
        pub unsafe fn setMinColumnWidth(&self, minColumnWidth: CGFloat);

        #[method(maxVisibleColumns)]
        pub unsafe fn maxVisibleColumns(&self) -> NSInteger;

        #[method(setMaxVisibleColumns:)]
        pub unsafe fn setMaxVisibleColumns(&self, maxVisibleColumns: NSInteger);

        #[method(allowsMultipleSelection)]
        pub unsafe fn allowsMultipleSelection(&self) -> bool;

        #[method(setAllowsMultipleSelection:)]
        pub unsafe fn setAllowsMultipleSelection(&self, allowsMultipleSelection: bool);

        #[method(allowsBranchSelection)]
        pub unsafe fn allowsBranchSelection(&self) -> bool;

        #[method(setAllowsBranchSelection:)]
        pub unsafe fn setAllowsBranchSelection(&self, allowsBranchSelection: bool);

        #[method(allowsEmptySelection)]
        pub unsafe fn allowsEmptySelection(&self) -> bool;

        #[method(setAllowsEmptySelection:)]
        pub unsafe fn setAllowsEmptySelection(&self, allowsEmptySelection: bool);

        #[method(takesTitleFromPreviousColumn)]
        pub unsafe fn takesTitleFromPreviousColumn(&self) -> bool;

        #[method(setTakesTitleFromPreviousColumn:)]
        pub unsafe fn setTakesTitleFromPreviousColumn(&self, takesTitleFromPreviousColumn: bool);

        #[method(sendsActionOnArrowKeys)]
        pub unsafe fn sendsActionOnArrowKeys(&self) -> bool;

        #[method(setSendsActionOnArrowKeys:)]
        pub unsafe fn setSendsActionOnArrowKeys(&self, sendsActionOnArrowKeys: bool);

        #[method_id(@__retain_semantics Other itemAtIndexPath:)]
        pub unsafe fn itemAtIndexPath(&self, indexPath: &NSIndexPath)
            -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other itemAtRow:inColumn:)]
        pub unsafe fn itemAtRow_inColumn(
            &self,
            row: NSInteger,
            column: NSInteger,
        ) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other indexPathForColumn:)]
        pub unsafe fn indexPathForColumn(&self, column: NSInteger) -> Id<NSIndexPath, Shared>;

        #[method(isLeafItem:)]
        pub unsafe fn isLeafItem(&self, item: Option<&Object>) -> bool;

        #[method(reloadDataForRowIndexes:inColumn:)]
        pub unsafe fn reloadDataForRowIndexes_inColumn(
            &self,
            rowIndexes: &NSIndexSet,
            column: NSInteger,
        );

        #[method_id(@__retain_semantics Other parentForItemsInColumn:)]
        pub unsafe fn parentForItemsInColumn(
            &self,
            column: NSInteger,
        ) -> Option<Id<Object, Shared>>;

        #[method(scrollRowToVisible:inColumn:)]
        pub unsafe fn scrollRowToVisible_inColumn(&self, row: NSInteger, column: NSInteger);

        #[method(setTitle:ofColumn:)]
        pub unsafe fn setTitle_ofColumn(&self, string: &NSString, column: NSInteger);

        #[method_id(@__retain_semantics Other titleOfColumn:)]
        pub unsafe fn titleOfColumn(&self, column: NSInteger) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other pathSeparator)]
        pub unsafe fn pathSeparator(&self) -> Id<NSString, Shared>;

        #[method(setPathSeparator:)]
        pub unsafe fn setPathSeparator(&self, pathSeparator: &NSString);

        #[method(setPath:)]
        pub unsafe fn setPath(&self, path: &NSString) -> bool;

        #[method_id(@__retain_semantics Other path)]
        pub unsafe fn path(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other pathToColumn:)]
        pub unsafe fn pathToColumn(&self, column: NSInteger) -> Id<NSString, Shared>;

        #[method(clickedColumn)]
        pub unsafe fn clickedColumn(&self) -> NSInteger;

        #[method(clickedRow)]
        pub unsafe fn clickedRow(&self) -> NSInteger;

        #[method(selectedColumn)]
        pub unsafe fn selectedColumn(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other selectedCell)]
        pub unsafe fn selectedCell(&self) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other selectedCellInColumn:)]
        pub unsafe fn selectedCellInColumn(&self, column: NSInteger) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other selectedCells)]
        pub unsafe fn selectedCells(&self) -> Option<Id<NSArray<NSCell>, Shared>>;

        #[method(selectRow:inColumn:)]
        pub unsafe fn selectRow_inColumn(&self, row: NSInteger, column: NSInteger);

        #[method(selectedRowInColumn:)]
        pub unsafe fn selectedRowInColumn(&self, column: NSInteger) -> NSInteger;

        #[method_id(@__retain_semantics Other selectionIndexPath)]
        pub unsafe fn selectionIndexPath(&self) -> Option<Id<NSIndexPath, Shared>>;

        #[method(setSelectionIndexPath:)]
        pub unsafe fn setSelectionIndexPath(&self, selectionIndexPath: Option<&NSIndexPath>);

        #[method_id(@__retain_semantics Other selectionIndexPaths)]
        pub unsafe fn selectionIndexPaths(&self) -> Id<NSArray<NSIndexPath>, Shared>;

        #[method(setSelectionIndexPaths:)]
        pub unsafe fn setSelectionIndexPaths(&self, selectionIndexPaths: &NSArray<NSIndexPath>);

        #[method(selectRowIndexes:inColumn:)]
        pub unsafe fn selectRowIndexes_inColumn(&self, indexes: &NSIndexSet, column: NSInteger);

        #[method_id(@__retain_semantics Other selectedRowIndexesInColumn:)]
        pub unsafe fn selectedRowIndexesInColumn(
            &self,
            column: NSInteger,
        ) -> Option<Id<NSIndexSet, Shared>>;

        #[method(reloadColumn:)]
        pub unsafe fn reloadColumn(&self, column: NSInteger);

        #[method(validateVisibleColumns)]
        pub unsafe fn validateVisibleColumns(&self);

        #[method(scrollColumnsRightBy:)]
        pub unsafe fn scrollColumnsRightBy(&self, shiftAmount: NSInteger);

        #[method(scrollColumnsLeftBy:)]
        pub unsafe fn scrollColumnsLeftBy(&self, shiftAmount: NSInteger);

        #[method(scrollColumnToVisible:)]
        pub unsafe fn scrollColumnToVisible(&self, column: NSInteger);

        #[method(lastColumn)]
        pub unsafe fn lastColumn(&self) -> NSInteger;

        #[method(setLastColumn:)]
        pub unsafe fn setLastColumn(&self, lastColumn: NSInteger);

        #[method(addColumn)]
        pub unsafe fn addColumn(&self);

        #[method(numberOfVisibleColumns)]
        pub unsafe fn numberOfVisibleColumns(&self) -> NSInteger;

        #[method(firstVisibleColumn)]
        pub unsafe fn firstVisibleColumn(&self) -> NSInteger;

        #[method(lastVisibleColumn)]
        pub unsafe fn lastVisibleColumn(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other loadedCellAtRow:column:)]
        pub unsafe fn loadedCellAtRow_column(
            &self,
            row: NSInteger,
            col: NSInteger,
        ) -> Option<Id<Object, Shared>>;

        #[method(selectAll:)]
        pub unsafe fn selectAll(&self, sender: Option<&Object>);

        #[method(tile)]
        pub unsafe fn tile(&self);

        #[method(doClick:)]
        pub unsafe fn doClick(&self, sender: Option<&Object>);

        #[method(doDoubleClick:)]
        pub unsafe fn doDoubleClick(&self, sender: Option<&Object>);

        #[method(sendAction)]
        pub unsafe fn sendAction(&self) -> bool;

        #[method(titleFrameOfColumn:)]
        pub unsafe fn titleFrameOfColumn(&self, column: NSInteger) -> NSRect;

        #[method(drawTitleOfColumn:inRect:)]
        pub unsafe fn drawTitleOfColumn_inRect(&self, column: NSInteger, rect: NSRect);

        #[method(titleHeight)]
        pub unsafe fn titleHeight(&self) -> CGFloat;

        #[method(frameOfColumn:)]
        pub unsafe fn frameOfColumn(&self, column: NSInteger) -> NSRect;

        #[method(frameOfInsideOfColumn:)]
        pub unsafe fn frameOfInsideOfColumn(&self, column: NSInteger) -> NSRect;

        #[method(frameOfRow:inColumn:)]
        pub unsafe fn frameOfRow_inColumn(&self, row: NSInteger, column: NSInteger) -> NSRect;

        #[method(getRow:column:forPoint:)]
        pub unsafe fn getRow_column_forPoint(
            &self,
            row: *mut NSInteger,
            column: *mut NSInteger,
            point: NSPoint,
        ) -> bool;

        #[method(columnWidthForColumnContentWidth:)]
        pub unsafe fn columnWidthForColumnContentWidth(
            &self,
            columnContentWidth: CGFloat,
        ) -> CGFloat;

        #[method(columnContentWidthForColumnWidth:)]
        pub unsafe fn columnContentWidthForColumnWidth(&self, columnWidth: CGFloat) -> CGFloat;

        #[method(columnResizingType)]
        pub unsafe fn columnResizingType(&self) -> NSBrowserColumnResizingType;

        #[method(setColumnResizingType:)]
        pub unsafe fn setColumnResizingType(&self, columnResizingType: NSBrowserColumnResizingType);

        #[method(prefersAllColumnUserResizing)]
        pub unsafe fn prefersAllColumnUserResizing(&self) -> bool;

        #[method(setPrefersAllColumnUserResizing:)]
        pub unsafe fn setPrefersAllColumnUserResizing(&self, prefersAllColumnUserResizing: bool);

        #[method(setWidth:ofColumn:)]
        pub unsafe fn setWidth_ofColumn(&self, columnWidth: CGFloat, columnIndex: NSInteger);

        #[method(widthOfColumn:)]
        pub unsafe fn widthOfColumn(&self, column: NSInteger) -> CGFloat;

        #[method(rowHeight)]
        pub unsafe fn rowHeight(&self) -> CGFloat;

        #[method(setRowHeight:)]
        pub unsafe fn setRowHeight(&self, rowHeight: CGFloat);

        #[method(noteHeightOfRowsWithIndexesChanged:inColumn:)]
        pub unsafe fn noteHeightOfRowsWithIndexesChanged_inColumn(
            &self,
            indexSet: &NSIndexSet,
            columnIndex: NSInteger,
        );

        #[method(setDefaultColumnWidth:)]
        pub unsafe fn setDefaultColumnWidth(&self, columnWidth: CGFloat);

        #[method(defaultColumnWidth)]
        pub unsafe fn defaultColumnWidth(&self) -> CGFloat;

        #[method_id(@__retain_semantics Other columnsAutosaveName)]
        pub unsafe fn columnsAutosaveName(&self) -> Id<NSBrowserColumnsAutosaveName, Shared>;

        #[method(setColumnsAutosaveName:)]
        pub unsafe fn setColumnsAutosaveName(
            &self,
            columnsAutosaveName: &NSBrowserColumnsAutosaveName,
        );

        #[method(removeSavedColumnsWithAutosaveName:)]
        pub unsafe fn removeSavedColumnsWithAutosaveName(name: &NSBrowserColumnsAutosaveName);

        #[method(canDragRowsWithIndexes:inColumn:withEvent:)]
        pub unsafe fn canDragRowsWithIndexes_inColumn_withEvent(
            &self,
            rowIndexes: &NSIndexSet,
            column: NSInteger,
            event: &NSEvent,
        ) -> bool;

        #[method_id(@__retain_semantics Other draggingImageForRowsWithIndexes:inColumn:withEvent:offset:)]
        pub unsafe fn draggingImageForRowsWithIndexes_inColumn_withEvent_offset(
            &self,
            rowIndexes: &NSIndexSet,
            column: NSInteger,
            event: &NSEvent,
            dragImageOffset: NSPointPointer,
        ) -> Option<Id<NSImage, Shared>>;

        #[method(setDraggingSourceOperationMask:forLocal:)]
        pub unsafe fn setDraggingSourceOperationMask_forLocal(
            &self,
            mask: NSDragOperation,
            isLocal: bool,
        );

        #[method(allowsTypeSelect)]
        pub unsafe fn allowsTypeSelect(&self) -> bool;

        #[method(setAllowsTypeSelect:)]
        pub unsafe fn setAllowsTypeSelect(&self, allowsTypeSelect: bool);

        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<NSColor, Shared>;

        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, backgroundColor: &NSColor);

        #[method(editItemAtIndexPath:withEvent:select:)]
        pub unsafe fn editItemAtIndexPath_withEvent_select(
            &self,
            indexPath: &NSIndexPath,
            event: Option<&NSEvent>,
            select: bool,
        );
    }
);

extern "C" {
    pub static NSBrowserColumnConfigurationDidChangeNotification: &'static NSNotificationName;
}

pub type NSBrowserDelegate = NSObject;

extern_methods!(
    /// NSDeprecated
    unsafe impl NSBrowser {
        #[method(setAcceptsArrowKeys:)]
        pub unsafe fn setAcceptsArrowKeys(&self, flag: bool);

        #[method(acceptsArrowKeys)]
        pub unsafe fn acceptsArrowKeys(&self) -> bool;

        #[method(displayColumn:)]
        pub unsafe fn displayColumn(&self, column: NSInteger);

        #[method(displayAllColumns)]
        pub unsafe fn displayAllColumns(&self);

        #[method(scrollViaScroller:)]
        pub unsafe fn scrollViaScroller(&self, sender: Option<&NSScroller>);

        #[method(updateScroller)]
        pub unsafe fn updateScroller(&self);

        #[method(setMatrixClass:)]
        pub unsafe fn setMatrixClass(&self, factoryId: &Class);

        #[method(matrixClass)]
        pub unsafe fn matrixClass(&self) -> &'static Class;

        #[method(columnOfMatrix:)]
        pub unsafe fn columnOfMatrix(&self, matrix: &NSMatrix) -> NSInteger;

        #[method_id(@__retain_semantics Other matrixInColumn:)]
        pub unsafe fn matrixInColumn(&self, column: NSInteger) -> Option<Id<NSMatrix, Shared>>;
    }
);
