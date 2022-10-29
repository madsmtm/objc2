#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSTableViewAutosaveName = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSTableView;
    unsafe impl ClassType for NSTableView {
        type Super = NSControl;
    }
);
extern_methods!(
    unsafe impl NSTableView {
        #[method_id(initWithFrame:)]
        pub unsafe fn initWithFrame(&self, frameRect: NSRect) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(dataSource)]
        pub unsafe fn dataSource(&self) -> Option<Id<NSTableViewDataSource, Shared>>;
        #[method(setDataSource:)]
        pub unsafe fn setDataSource(&self, dataSource: Option<&NSTableViewDataSource>);
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSTableViewDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSTableViewDelegate>);
        #[method_id(headerView)]
        pub unsafe fn headerView(&self) -> Option<Id<NSTableHeaderView, Shared>>;
        #[method(setHeaderView:)]
        pub unsafe fn setHeaderView(&self, headerView: Option<&NSTableHeaderView>);
        #[method_id(cornerView)]
        pub unsafe fn cornerView(&self) -> Option<Id<NSView, Shared>>;
        #[method(setCornerView:)]
        pub unsafe fn setCornerView(&self, cornerView: Option<&NSView>);
        #[method(allowsColumnReordering)]
        pub unsafe fn allowsColumnReordering(&self) -> bool;
        #[method(setAllowsColumnReordering:)]
        pub unsafe fn setAllowsColumnReordering(&self, allowsColumnReordering: bool);
        #[method(allowsColumnResizing)]
        pub unsafe fn allowsColumnResizing(&self) -> bool;
        #[method(setAllowsColumnResizing:)]
        pub unsafe fn setAllowsColumnResizing(&self, allowsColumnResizing: bool);
        #[method(columnAutoresizingStyle)]
        pub unsafe fn columnAutoresizingStyle(&self) -> NSTableViewColumnAutoresizingStyle;
        #[method(setColumnAutoresizingStyle:)]
        pub unsafe fn setColumnAutoresizingStyle(
            &self,
            columnAutoresizingStyle: NSTableViewColumnAutoresizingStyle,
        );
        #[method(gridStyleMask)]
        pub unsafe fn gridStyleMask(&self) -> NSTableViewGridLineStyle;
        #[method(setGridStyleMask:)]
        pub unsafe fn setGridStyleMask(&self, gridStyleMask: NSTableViewGridLineStyle);
        #[method(intercellSpacing)]
        pub unsafe fn intercellSpacing(&self) -> NSSize;
        #[method(setIntercellSpacing:)]
        pub unsafe fn setIntercellSpacing(&self, intercellSpacing: NSSize);
        #[method(usesAlternatingRowBackgroundColors)]
        pub unsafe fn usesAlternatingRowBackgroundColors(&self) -> bool;
        #[method(setUsesAlternatingRowBackgroundColors:)]
        pub unsafe fn setUsesAlternatingRowBackgroundColors(
            &self,
            usesAlternatingRowBackgroundColors: bool,
        );
        #[method_id(backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<NSColor, Shared>;
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, backgroundColor: &NSColor);
        #[method_id(gridColor)]
        pub unsafe fn gridColor(&self) -> Id<NSColor, Shared>;
        #[method(setGridColor:)]
        pub unsafe fn setGridColor(&self, gridColor: &NSColor);
        #[method(rowSizeStyle)]
        pub unsafe fn rowSizeStyle(&self) -> NSTableViewRowSizeStyle;
        #[method(setRowSizeStyle:)]
        pub unsafe fn setRowSizeStyle(&self, rowSizeStyle: NSTableViewRowSizeStyle);
        #[method(effectiveRowSizeStyle)]
        pub unsafe fn effectiveRowSizeStyle(&self) -> NSTableViewRowSizeStyle;
        #[method(rowHeight)]
        pub unsafe fn rowHeight(&self) -> CGFloat;
        #[method(setRowHeight:)]
        pub unsafe fn setRowHeight(&self, rowHeight: CGFloat);
        #[method(noteHeightOfRowsWithIndexesChanged:)]
        pub unsafe fn noteHeightOfRowsWithIndexesChanged(&self, indexSet: &NSIndexSet);
        #[method_id(tableColumns)]
        pub unsafe fn tableColumns(&self) -> Id<NSArray<NSTableColumn>, Shared>;
        #[method(numberOfColumns)]
        pub unsafe fn numberOfColumns(&self) -> NSInteger;
        #[method(numberOfRows)]
        pub unsafe fn numberOfRows(&self) -> NSInteger;
        #[method(addTableColumn:)]
        pub unsafe fn addTableColumn(&self, tableColumn: &NSTableColumn);
        #[method(removeTableColumn:)]
        pub unsafe fn removeTableColumn(&self, tableColumn: &NSTableColumn);
        #[method(moveColumn:toColumn:)]
        pub unsafe fn moveColumn_toColumn(&self, oldIndex: NSInteger, newIndex: NSInteger);
        #[method(columnWithIdentifier:)]
        pub unsafe fn columnWithIdentifier(
            &self,
            identifier: &NSUserInterfaceItemIdentifier,
        ) -> NSInteger;
        #[method_id(tableColumnWithIdentifier:)]
        pub unsafe fn tableColumnWithIdentifier(
            &self,
            identifier: &NSUserInterfaceItemIdentifier,
        ) -> Option<Id<NSTableColumn, Shared>>;
        #[method(tile)]
        pub unsafe fn tile(&self);
        #[method(sizeToFit)]
        pub unsafe fn sizeToFit(&self);
        #[method(sizeLastColumnToFit)]
        pub unsafe fn sizeLastColumnToFit(&self);
        #[method(scrollRowToVisible:)]
        pub unsafe fn scrollRowToVisible(&self, row: NSInteger);
        #[method(scrollColumnToVisible:)]
        pub unsafe fn scrollColumnToVisible(&self, column: NSInteger);
        #[method(reloadData)]
        pub unsafe fn reloadData(&self);
        #[method(noteNumberOfRowsChanged)]
        pub unsafe fn noteNumberOfRowsChanged(&self);
        #[method(reloadDataForRowIndexes:columnIndexes:)]
        pub unsafe fn reloadDataForRowIndexes_columnIndexes(
            &self,
            rowIndexes: &NSIndexSet,
            columnIndexes: &NSIndexSet,
        );
        #[method(editedColumn)]
        pub unsafe fn editedColumn(&self) -> NSInteger;
        #[method(editedRow)]
        pub unsafe fn editedRow(&self) -> NSInteger;
        #[method(clickedColumn)]
        pub unsafe fn clickedColumn(&self) -> NSInteger;
        #[method(clickedRow)]
        pub unsafe fn clickedRow(&self) -> NSInteger;
        #[method(doubleAction)]
        pub unsafe fn doubleAction(&self) -> Option<Sel>;
        #[method(setDoubleAction:)]
        pub unsafe fn setDoubleAction(&self, doubleAction: Option<Sel>);
        #[method_id(sortDescriptors)]
        pub unsafe fn sortDescriptors(&self) -> Id<NSArray<NSSortDescriptor>, Shared>;
        #[method(setSortDescriptors:)]
        pub unsafe fn setSortDescriptors(&self, sortDescriptors: &NSArray<NSSortDescriptor>);
        #[method(setIndicatorImage:inTableColumn:)]
        pub unsafe fn setIndicatorImage_inTableColumn(
            &self,
            image: Option<&NSImage>,
            tableColumn: &NSTableColumn,
        );
        #[method_id(indicatorImageInTableColumn:)]
        pub unsafe fn indicatorImageInTableColumn(
            &self,
            tableColumn: &NSTableColumn,
        ) -> Option<Id<NSImage, Shared>>;
        #[method_id(highlightedTableColumn)]
        pub unsafe fn highlightedTableColumn(&self) -> Option<Id<NSTableColumn, Shared>>;
        #[method(setHighlightedTableColumn:)]
        pub unsafe fn setHighlightedTableColumn(
            &self,
            highlightedTableColumn: Option<&NSTableColumn>,
        );
        #[method(verticalMotionCanBeginDrag)]
        pub unsafe fn verticalMotionCanBeginDrag(&self) -> bool;
        #[method(setVerticalMotionCanBeginDrag:)]
        pub unsafe fn setVerticalMotionCanBeginDrag(&self, verticalMotionCanBeginDrag: bool);
        #[method(canDragRowsWithIndexes:atPoint:)]
        pub unsafe fn canDragRowsWithIndexes_atPoint(
            &self,
            rowIndexes: &NSIndexSet,
            mouseDownPoint: NSPoint,
        ) -> bool;
        #[method_id(dragImageForRowsWithIndexes:tableColumns:event:offset:)]
        pub unsafe fn dragImageForRowsWithIndexes_tableColumns_event_offset(
            &self,
            dragRows: &NSIndexSet,
            tableColumns: &NSArray<NSTableColumn>,
            dragEvent: &NSEvent,
            dragImageOffset: NSPointPointer,
        ) -> Id<NSImage, Shared>;
        #[method(setDraggingSourceOperationMask:forLocal:)]
        pub unsafe fn setDraggingSourceOperationMask_forLocal(
            &self,
            mask: NSDragOperation,
            isLocal: bool,
        );
        #[method(setDropRow:dropOperation:)]
        pub unsafe fn setDropRow_dropOperation(
            &self,
            row: NSInteger,
            dropOperation: NSTableViewDropOperation,
        );
        #[method(allowsMultipleSelection)]
        pub unsafe fn allowsMultipleSelection(&self) -> bool;
        #[method(setAllowsMultipleSelection:)]
        pub unsafe fn setAllowsMultipleSelection(&self, allowsMultipleSelection: bool);
        #[method(allowsEmptySelection)]
        pub unsafe fn allowsEmptySelection(&self) -> bool;
        #[method(setAllowsEmptySelection:)]
        pub unsafe fn setAllowsEmptySelection(&self, allowsEmptySelection: bool);
        #[method(allowsColumnSelection)]
        pub unsafe fn allowsColumnSelection(&self) -> bool;
        #[method(setAllowsColumnSelection:)]
        pub unsafe fn setAllowsColumnSelection(&self, allowsColumnSelection: bool);
        #[method(selectAll:)]
        pub unsafe fn selectAll(&self, sender: Option<&Object>);
        #[method(deselectAll:)]
        pub unsafe fn deselectAll(&self, sender: Option<&Object>);
        #[method(selectColumnIndexes:byExtendingSelection:)]
        pub unsafe fn selectColumnIndexes_byExtendingSelection(
            &self,
            indexes: &NSIndexSet,
            extend: bool,
        );
        #[method(selectRowIndexes:byExtendingSelection:)]
        pub unsafe fn selectRowIndexes_byExtendingSelection(
            &self,
            indexes: &NSIndexSet,
            extend: bool,
        );
        #[method_id(selectedColumnIndexes)]
        pub unsafe fn selectedColumnIndexes(&self) -> Id<NSIndexSet, Shared>;
        #[method_id(selectedRowIndexes)]
        pub unsafe fn selectedRowIndexes(&self) -> Id<NSIndexSet, Shared>;
        #[method(deselectColumn:)]
        pub unsafe fn deselectColumn(&self, column: NSInteger);
        #[method(deselectRow:)]
        pub unsafe fn deselectRow(&self, row: NSInteger);
        #[method(selectedColumn)]
        pub unsafe fn selectedColumn(&self) -> NSInteger;
        #[method(selectedRow)]
        pub unsafe fn selectedRow(&self) -> NSInteger;
        #[method(isColumnSelected:)]
        pub unsafe fn isColumnSelected(&self, column: NSInteger) -> bool;
        #[method(isRowSelected:)]
        pub unsafe fn isRowSelected(&self, row: NSInteger) -> bool;
        #[method(numberOfSelectedColumns)]
        pub unsafe fn numberOfSelectedColumns(&self) -> NSInteger;
        #[method(numberOfSelectedRows)]
        pub unsafe fn numberOfSelectedRows(&self) -> NSInteger;
        #[method(allowsTypeSelect)]
        pub unsafe fn allowsTypeSelect(&self) -> bool;
        #[method(setAllowsTypeSelect:)]
        pub unsafe fn setAllowsTypeSelect(&self, allowsTypeSelect: bool);
        #[method(style)]
        pub unsafe fn style(&self) -> NSTableViewStyle;
        #[method(setStyle:)]
        pub unsafe fn setStyle(&self, style: NSTableViewStyle);
        #[method(effectiveStyle)]
        pub unsafe fn effectiveStyle(&self) -> NSTableViewStyle;
        #[method(selectionHighlightStyle)]
        pub unsafe fn selectionHighlightStyle(&self) -> NSTableViewSelectionHighlightStyle;
        #[method(setSelectionHighlightStyle:)]
        pub unsafe fn setSelectionHighlightStyle(
            &self,
            selectionHighlightStyle: NSTableViewSelectionHighlightStyle,
        );
        #[method(draggingDestinationFeedbackStyle)]
        pub unsafe fn draggingDestinationFeedbackStyle(
            &self,
        ) -> NSTableViewDraggingDestinationFeedbackStyle;
        #[method(setDraggingDestinationFeedbackStyle:)]
        pub unsafe fn setDraggingDestinationFeedbackStyle(
            &self,
            draggingDestinationFeedbackStyle: NSTableViewDraggingDestinationFeedbackStyle,
        );
        #[method(rectOfColumn:)]
        pub unsafe fn rectOfColumn(&self, column: NSInteger) -> NSRect;
        #[method(rectOfRow:)]
        pub unsafe fn rectOfRow(&self, row: NSInteger) -> NSRect;
        #[method_id(columnIndexesInRect:)]
        pub unsafe fn columnIndexesInRect(&self, rect: NSRect) -> Id<NSIndexSet, Shared>;
        #[method(rowsInRect:)]
        pub unsafe fn rowsInRect(&self, rect: NSRect) -> NSRange;
        #[method(columnAtPoint:)]
        pub unsafe fn columnAtPoint(&self, point: NSPoint) -> NSInteger;
        #[method(rowAtPoint:)]
        pub unsafe fn rowAtPoint(&self, point: NSPoint) -> NSInteger;
        #[method(frameOfCellAtColumn:row:)]
        pub unsafe fn frameOfCellAtColumn_row(&self, column: NSInteger, row: NSInteger) -> NSRect;
        #[method_id(autosaveName)]
        pub unsafe fn autosaveName(&self) -> Option<Id<NSTableViewAutosaveName, Shared>>;
        #[method(setAutosaveName:)]
        pub unsafe fn setAutosaveName(&self, autosaveName: Option<&NSTableViewAutosaveName>);
        #[method(autosaveTableColumns)]
        pub unsafe fn autosaveTableColumns(&self) -> bool;
        #[method(setAutosaveTableColumns:)]
        pub unsafe fn setAutosaveTableColumns(&self, autosaveTableColumns: bool);
        #[method(editColumn:row:withEvent:select:)]
        pub unsafe fn editColumn_row_withEvent_select(
            &self,
            column: NSInteger,
            row: NSInteger,
            event: Option<&NSEvent>,
            select: bool,
        );
        #[method(drawRow:clipRect:)]
        pub unsafe fn drawRow_clipRect(&self, row: NSInteger, clipRect: NSRect);
        #[method(highlightSelectionInClipRect:)]
        pub unsafe fn highlightSelectionInClipRect(&self, clipRect: NSRect);
        #[method(drawGridInClipRect:)]
        pub unsafe fn drawGridInClipRect(&self, clipRect: NSRect);
        #[method(drawBackgroundInClipRect:)]
        pub unsafe fn drawBackgroundInClipRect(&self, clipRect: NSRect);
        #[method_id(viewAtColumn:row:makeIfNecessary:)]
        pub unsafe fn viewAtColumn_row_makeIfNecessary(
            &self,
            column: NSInteger,
            row: NSInteger,
            makeIfNecessary: bool,
        ) -> Option<Id<NSView, Shared>>;
        #[method_id(rowViewAtRow:makeIfNecessary:)]
        pub unsafe fn rowViewAtRow_makeIfNecessary(
            &self,
            row: NSInteger,
            makeIfNecessary: bool,
        ) -> Option<Id<NSTableRowView, Shared>>;
        #[method(rowForView:)]
        pub unsafe fn rowForView(&self, view: &NSView) -> NSInteger;
        #[method(columnForView:)]
        pub unsafe fn columnForView(&self, view: &NSView) -> NSInteger;
        #[method_id(makeViewWithIdentifier:owner:)]
        pub unsafe fn makeViewWithIdentifier_owner(
            &self,
            identifier: &NSUserInterfaceItemIdentifier,
            owner: Option<&Object>,
        ) -> Option<Id<NSView, Shared>>;
        #[method(enumerateAvailableRowViewsUsingBlock:)]
        pub unsafe fn enumerateAvailableRowViewsUsingBlock(&self, handler: TodoBlock);
        #[method(floatsGroupRows)]
        pub unsafe fn floatsGroupRows(&self) -> bool;
        #[method(setFloatsGroupRows:)]
        pub unsafe fn setFloatsGroupRows(&self, floatsGroupRows: bool);
        #[method(rowActionsVisible)]
        pub unsafe fn rowActionsVisible(&self) -> bool;
        #[method(setRowActionsVisible:)]
        pub unsafe fn setRowActionsVisible(&self, rowActionsVisible: bool);
        #[method(beginUpdates)]
        pub unsafe fn beginUpdates(&self);
        #[method(endUpdates)]
        pub unsafe fn endUpdates(&self);
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
        #[method(hideRowsAtIndexes:withAnimation:)]
        pub unsafe fn hideRowsAtIndexes_withAnimation(
            &self,
            indexes: &NSIndexSet,
            rowAnimation: NSTableViewAnimationOptions,
        );
        #[method(unhideRowsAtIndexes:withAnimation:)]
        pub unsafe fn unhideRowsAtIndexes_withAnimation(
            &self,
            indexes: &NSIndexSet,
            rowAnimation: NSTableViewAnimationOptions,
        );
        #[method_id(hiddenRowIndexes)]
        pub unsafe fn hiddenRowIndexes(&self) -> Id<NSIndexSet, Shared>;
        #[method(registerNib:forIdentifier:)]
        pub unsafe fn registerNib_forIdentifier(
            &self,
            nib: Option<&NSNib>,
            identifier: &NSUserInterfaceItemIdentifier,
        );
        #[method_id(registeredNibsByIdentifier)]
        pub unsafe fn registeredNibsByIdentifier(
            &self,
        ) -> Option<Id<NSDictionary<NSUserInterfaceItemIdentifier, NSNib>, Shared>>;
        #[method(didAddRowView:forRow:)]
        pub unsafe fn didAddRowView_forRow(&self, rowView: &NSTableRowView, row: NSInteger);
        #[method(didRemoveRowView:forRow:)]
        pub unsafe fn didRemoveRowView_forRow(&self, rowView: &NSTableRowView, row: NSInteger);
        #[method(usesStaticContents)]
        pub unsafe fn usesStaticContents(&self) -> bool;
        #[method(setUsesStaticContents:)]
        pub unsafe fn setUsesStaticContents(&self, usesStaticContents: bool);
        #[method(userInterfaceLayoutDirection)]
        pub unsafe fn userInterfaceLayoutDirection(&self) -> NSUserInterfaceLayoutDirection;
        #[method(setUserInterfaceLayoutDirection:)]
        pub unsafe fn setUserInterfaceLayoutDirection(
            &self,
            userInterfaceLayoutDirection: NSUserInterfaceLayoutDirection,
        );
        #[method(usesAutomaticRowHeights)]
        pub unsafe fn usesAutomaticRowHeights(&self) -> bool;
        #[method(setUsesAutomaticRowHeights:)]
        pub unsafe fn setUsesAutomaticRowHeights(&self, usesAutomaticRowHeights: bool);
    }
);
pub type NSTableViewDelegate = NSObject;
pub type NSTableViewDataSource = NSObject;
extern_methods!(
    #[doc = "NSTableViewDataSourceDeprecated"]
    unsafe impl NSObject {
        #[method(tableView:writeRows:toPasteboard:)]
        pub unsafe fn tableView_writeRows_toPasteboard(
            &self,
            tableView: &NSTableView,
            rows: &NSArray,
            pboard: &NSPasteboard,
        ) -> bool;
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSTableView {
        #[method(setDrawsGrid:)]
        pub unsafe fn setDrawsGrid(&self, flag: bool);
        #[method(drawsGrid)]
        pub unsafe fn drawsGrid(&self) -> bool;
        #[method(selectColumn:byExtendingSelection:)]
        pub unsafe fn selectColumn_byExtendingSelection(&self, column: NSInteger, extend: bool);
        #[method(selectRow:byExtendingSelection:)]
        pub unsafe fn selectRow_byExtendingSelection(&self, row: NSInteger, extend: bool);
        #[method_id(selectedColumnEnumerator)]
        pub unsafe fn selectedColumnEnumerator(&self) -> Id<NSEnumerator, Shared>;
        #[method_id(selectedRowEnumerator)]
        pub unsafe fn selectedRowEnumerator(&self) -> Id<NSEnumerator, Shared>;
        #[method_id(dragImageForRows:event:dragImageOffset:)]
        pub unsafe fn dragImageForRows_event_dragImageOffset(
            &self,
            dragRows: &NSArray,
            dragEvent: &NSEvent,
            dragImageOffset: NSPointPointer,
        ) -> Option<Id<NSImage, Shared>>;
        #[method(setAutoresizesAllColumnsToFit:)]
        pub unsafe fn setAutoresizesAllColumnsToFit(&self, flag: bool);
        #[method(autoresizesAllColumnsToFit)]
        pub unsafe fn autoresizesAllColumnsToFit(&self) -> bool;
        #[method(columnsInRect:)]
        pub unsafe fn columnsInRect(&self, rect: NSRect) -> NSRange;
        #[method_id(preparedCellAtColumn:row:)]
        pub unsafe fn preparedCellAtColumn_row(
            &self,
            column: NSInteger,
            row: NSInteger,
        ) -> Option<Id<NSCell, Shared>>;
        #[method(textShouldBeginEditing:)]
        pub unsafe fn textShouldBeginEditing(&self, textObject: &NSText) -> bool;
        #[method(textShouldEndEditing:)]
        pub unsafe fn textShouldEndEditing(&self, textObject: &NSText) -> bool;
        #[method(textDidBeginEditing:)]
        pub unsafe fn textDidBeginEditing(&self, notification: &NSNotification);
        #[method(textDidEndEditing:)]
        pub unsafe fn textDidEndEditing(&self, notification: &NSNotification);
        #[method(textDidChange:)]
        pub unsafe fn textDidChange(&self, notification: &NSNotification);
        #[method(shouldFocusCell:atColumn:row:)]
        pub unsafe fn shouldFocusCell_atColumn_row(
            &self,
            cell: &NSCell,
            column: NSInteger,
            row: NSInteger,
        ) -> bool;
        #[method(focusedColumn)]
        pub unsafe fn focusedColumn(&self) -> NSInteger;
        #[method(setFocusedColumn:)]
        pub unsafe fn setFocusedColumn(&self, focusedColumn: NSInteger);
        #[method(performClickOnCellAtColumn:row:)]
        pub unsafe fn performClickOnCellAtColumn_row(&self, column: NSInteger, row: NSInteger);
    }
);