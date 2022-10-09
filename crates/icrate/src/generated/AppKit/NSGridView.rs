use super::__exported::NSLayoutConstraint;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSLayoutAnchor::*;
use crate::AppKit::generated::NSView::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSGridView;
    unsafe impl ClassType for NSGridView {
        type Super = NSView;
    }
);
extern_methods!(
    unsafe impl NSGridView {
        #[method_id(initWithFrame:)]
        pub unsafe fn initWithFrame(&self, frameRect: NSRect) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(gridViewWithNumberOfColumns:rows:)]
        pub unsafe fn gridViewWithNumberOfColumns_rows(
            columnCount: NSInteger,
            rowCount: NSInteger,
        ) -> Id<Self, Shared>;
        #[method_id(gridViewWithViews:)]
        pub unsafe fn gridViewWithViews(rows: &NSArray<NSArray<NSView>>) -> Id<Self, Shared>;
        #[method(numberOfRows)]
        pub unsafe fn numberOfRows(&self) -> NSInteger;
        #[method(numberOfColumns)]
        pub unsafe fn numberOfColumns(&self) -> NSInteger;
        #[method_id(rowAtIndex:)]
        pub unsafe fn rowAtIndex(&self, index: NSInteger) -> Id<NSGridRow, Shared>;
        #[method(indexOfRow:)]
        pub unsafe fn indexOfRow(&self, row: &NSGridRow) -> NSInteger;
        #[method_id(columnAtIndex:)]
        pub unsafe fn columnAtIndex(&self, index: NSInteger) -> Id<NSGridColumn, Shared>;
        #[method(indexOfColumn:)]
        pub unsafe fn indexOfColumn(&self, column: &NSGridColumn) -> NSInteger;
        #[method_id(cellAtColumnIndex:rowIndex:)]
        pub unsafe fn cellAtColumnIndex_rowIndex(
            &self,
            columnIndex: NSInteger,
            rowIndex: NSInteger,
        ) -> Id<NSGridCell, Shared>;
        #[method_id(cellForView:)]
        pub unsafe fn cellForView(&self, view: &NSView) -> Option<Id<NSGridCell, Shared>>;
        #[method_id(addRowWithViews:)]
        pub unsafe fn addRowWithViews(&self, views: &NSArray<NSView>) -> Id<NSGridRow, Shared>;
        #[method_id(insertRowAtIndex:withViews:)]
        pub unsafe fn insertRowAtIndex_withViews(
            &self,
            index: NSInteger,
            views: &NSArray<NSView>,
        ) -> Id<NSGridRow, Shared>;
        #[method(moveRowAtIndex:toIndex:)]
        pub unsafe fn moveRowAtIndex_toIndex(&self, fromIndex: NSInteger, toIndex: NSInteger);
        #[method(removeRowAtIndex:)]
        pub unsafe fn removeRowAtIndex(&self, index: NSInteger);
        #[method_id(addColumnWithViews:)]
        pub unsafe fn addColumnWithViews(
            &self,
            views: &NSArray<NSView>,
        ) -> Id<NSGridColumn, Shared>;
        #[method_id(insertColumnAtIndex:withViews:)]
        pub unsafe fn insertColumnAtIndex_withViews(
            &self,
            index: NSInteger,
            views: &NSArray<NSView>,
        ) -> Id<NSGridColumn, Shared>;
        #[method(moveColumnAtIndex:toIndex:)]
        pub unsafe fn moveColumnAtIndex_toIndex(&self, fromIndex: NSInteger, toIndex: NSInteger);
        #[method(removeColumnAtIndex:)]
        pub unsafe fn removeColumnAtIndex(&self, index: NSInteger);
        #[method(xPlacement)]
        pub unsafe fn xPlacement(&self) -> NSGridCellPlacement;
        #[method(setXPlacement:)]
        pub unsafe fn setXPlacement(&self, xPlacement: NSGridCellPlacement);
        #[method(yPlacement)]
        pub unsafe fn yPlacement(&self) -> NSGridCellPlacement;
        #[method(setYPlacement:)]
        pub unsafe fn setYPlacement(&self, yPlacement: NSGridCellPlacement);
        #[method(rowAlignment)]
        pub unsafe fn rowAlignment(&self) -> NSGridRowAlignment;
        #[method(setRowAlignment:)]
        pub unsafe fn setRowAlignment(&self, rowAlignment: NSGridRowAlignment);
        #[method(rowSpacing)]
        pub unsafe fn rowSpacing(&self) -> CGFloat;
        #[method(setRowSpacing:)]
        pub unsafe fn setRowSpacing(&self, rowSpacing: CGFloat);
        #[method(columnSpacing)]
        pub unsafe fn columnSpacing(&self) -> CGFloat;
        #[method(setColumnSpacing:)]
        pub unsafe fn setColumnSpacing(&self, columnSpacing: CGFloat);
        #[method(mergeCellsInHorizontalRange:verticalRange:)]
        pub unsafe fn mergeCellsInHorizontalRange_verticalRange(
            &self,
            hRange: NSRange,
            vRange: NSRange,
        );
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSGridRow;
    unsafe impl ClassType for NSGridRow {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSGridRow {
        #[method_id(gridView)]
        pub unsafe fn gridView(&self) -> Option<Id<NSGridView, Shared>>;
        #[method(numberOfCells)]
        pub unsafe fn numberOfCells(&self) -> NSInteger;
        #[method_id(cellAtIndex:)]
        pub unsafe fn cellAtIndex(&self, index: NSInteger) -> Id<NSGridCell, Shared>;
        #[method(yPlacement)]
        pub unsafe fn yPlacement(&self) -> NSGridCellPlacement;
        #[method(setYPlacement:)]
        pub unsafe fn setYPlacement(&self, yPlacement: NSGridCellPlacement);
        #[method(rowAlignment)]
        pub unsafe fn rowAlignment(&self) -> NSGridRowAlignment;
        #[method(setRowAlignment:)]
        pub unsafe fn setRowAlignment(&self, rowAlignment: NSGridRowAlignment);
        #[method(height)]
        pub unsafe fn height(&self) -> CGFloat;
        #[method(setHeight:)]
        pub unsafe fn setHeight(&self, height: CGFloat);
        #[method(topPadding)]
        pub unsafe fn topPadding(&self) -> CGFloat;
        #[method(setTopPadding:)]
        pub unsafe fn setTopPadding(&self, topPadding: CGFloat);
        #[method(bottomPadding)]
        pub unsafe fn bottomPadding(&self) -> CGFloat;
        #[method(setBottomPadding:)]
        pub unsafe fn setBottomPadding(&self, bottomPadding: CGFloat);
        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;
        #[method(setHidden:)]
        pub unsafe fn setHidden(&self, hidden: bool);
        #[method(mergeCellsInRange:)]
        pub unsafe fn mergeCellsInRange(&self, range: NSRange);
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSGridColumn;
    unsafe impl ClassType for NSGridColumn {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSGridColumn {
        #[method_id(gridView)]
        pub unsafe fn gridView(&self) -> Option<Id<NSGridView, Shared>>;
        #[method(numberOfCells)]
        pub unsafe fn numberOfCells(&self) -> NSInteger;
        #[method_id(cellAtIndex:)]
        pub unsafe fn cellAtIndex(&self, index: NSInteger) -> Id<NSGridCell, Shared>;
        #[method(xPlacement)]
        pub unsafe fn xPlacement(&self) -> NSGridCellPlacement;
        #[method(setXPlacement:)]
        pub unsafe fn setXPlacement(&self, xPlacement: NSGridCellPlacement);
        #[method(width)]
        pub unsafe fn width(&self) -> CGFloat;
        #[method(setWidth:)]
        pub unsafe fn setWidth(&self, width: CGFloat);
        #[method(leadingPadding)]
        pub unsafe fn leadingPadding(&self) -> CGFloat;
        #[method(setLeadingPadding:)]
        pub unsafe fn setLeadingPadding(&self, leadingPadding: CGFloat);
        #[method(trailingPadding)]
        pub unsafe fn trailingPadding(&self) -> CGFloat;
        #[method(setTrailingPadding:)]
        pub unsafe fn setTrailingPadding(&self, trailingPadding: CGFloat);
        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;
        #[method(setHidden:)]
        pub unsafe fn setHidden(&self, hidden: bool);
        #[method(mergeCellsInRange:)]
        pub unsafe fn mergeCellsInRange(&self, range: NSRange);
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSGridCell;
    unsafe impl ClassType for NSGridCell {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSGridCell {
        #[method_id(contentView)]
        pub unsafe fn contentView(&self) -> Option<Id<NSView, Shared>>;
        #[method(setContentView:)]
        pub unsafe fn setContentView(&self, contentView: Option<&NSView>);
        #[method_id(emptyContentView)]
        pub unsafe fn emptyContentView() -> Id<NSView, Shared>;
        #[method_id(row)]
        pub unsafe fn row(&self) -> Option<Id<NSGridRow, Shared>>;
        #[method_id(column)]
        pub unsafe fn column(&self) -> Option<Id<NSGridColumn, Shared>>;
        #[method(xPlacement)]
        pub unsafe fn xPlacement(&self) -> NSGridCellPlacement;
        #[method(setXPlacement:)]
        pub unsafe fn setXPlacement(&self, xPlacement: NSGridCellPlacement);
        #[method(yPlacement)]
        pub unsafe fn yPlacement(&self) -> NSGridCellPlacement;
        #[method(setYPlacement:)]
        pub unsafe fn setYPlacement(&self, yPlacement: NSGridCellPlacement);
        #[method(rowAlignment)]
        pub unsafe fn rowAlignment(&self) -> NSGridRowAlignment;
        #[method(setRowAlignment:)]
        pub unsafe fn setRowAlignment(&self, rowAlignment: NSGridRowAlignment);
        #[method_id(customPlacementConstraints)]
        pub unsafe fn customPlacementConstraints(&self) -> Id<NSArray<NSLayoutConstraint>, Shared>;
        #[method(setCustomPlacementConstraints:)]
        pub unsafe fn setCustomPlacementConstraints(
            &self,
            customPlacementConstraints: &NSArray<NSLayoutConstraint>,
        );
    }
);
