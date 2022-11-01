//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

pub type NSTextBlockValueType = NSUInteger;
pub const NSTextBlockAbsoluteValueType: NSTextBlockValueType = 0;
pub const NSTextBlockPercentageValueType: NSTextBlockValueType = 1;

pub type NSTextBlockDimension = NSUInteger;
pub const NSTextBlockWidth: NSTextBlockDimension = 0;
pub const NSTextBlockMinimumWidth: NSTextBlockDimension = 1;
pub const NSTextBlockMaximumWidth: NSTextBlockDimension = 2;
pub const NSTextBlockHeight: NSTextBlockDimension = 4;
pub const NSTextBlockMinimumHeight: NSTextBlockDimension = 5;
pub const NSTextBlockMaximumHeight: NSTextBlockDimension = 6;

pub type NSTextBlockLayer = NSInteger;
pub const NSTextBlockPadding: NSTextBlockLayer = -1;
pub const NSTextBlockBorder: NSTextBlockLayer = 0;
pub const NSTextBlockMargin: NSTextBlockLayer = 1;

pub type NSTextBlockVerticalAlignment = NSUInteger;
pub const NSTextBlockTopAlignment: NSTextBlockVerticalAlignment = 0;
pub const NSTextBlockMiddleAlignment: NSTextBlockVerticalAlignment = 1;
pub const NSTextBlockBottomAlignment: NSTextBlockVerticalAlignment = 2;
pub const NSTextBlockBaselineAlignment: NSTextBlockVerticalAlignment = 3;

pub type NSTextTableLayoutAlgorithm = NSUInteger;
pub const NSTextTableAutomaticLayoutAlgorithm: NSTextTableLayoutAlgorithm = 0;
pub const NSTextTableFixedLayoutAlgorithm: NSTextTableLayoutAlgorithm = 1;

extern_class!(
    #[derive(Debug)]
    pub struct NSTextBlock;

    unsafe impl ClassType for NSTextBlock {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSTextBlock {
        #[method_id(init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method(setValue:type:forDimension:)]
        pub unsafe fn setValue_type_forDimension(
            &self,
            val: CGFloat,
            type_: NSTextBlockValueType,
            dimension: NSTextBlockDimension,
        );

        #[method(valueForDimension:)]
        pub unsafe fn valueForDimension(&self, dimension: NSTextBlockDimension) -> CGFloat;

        #[method(valueTypeForDimension:)]
        pub unsafe fn valueTypeForDimension(
            &self,
            dimension: NSTextBlockDimension,
        ) -> NSTextBlockValueType;

        #[method(setContentWidth:type:)]
        pub unsafe fn setContentWidth_type(&self, val: CGFloat, type_: NSTextBlockValueType);

        #[method(contentWidth)]
        pub unsafe fn contentWidth(&self) -> CGFloat;

        #[method(contentWidthValueType)]
        pub unsafe fn contentWidthValueType(&self) -> NSTextBlockValueType;

        #[method(setWidth:type:forLayer:edge:)]
        pub unsafe fn setWidth_type_forLayer_edge(
            &self,
            val: CGFloat,
            type_: NSTextBlockValueType,
            layer: NSTextBlockLayer,
            edge: NSRectEdge,
        );

        #[method(setWidth:type:forLayer:)]
        pub unsafe fn setWidth_type_forLayer(
            &self,
            val: CGFloat,
            type_: NSTextBlockValueType,
            layer: NSTextBlockLayer,
        );

        #[method(widthForLayer:edge:)]
        pub unsafe fn widthForLayer_edge(
            &self,
            layer: NSTextBlockLayer,
            edge: NSRectEdge,
        ) -> CGFloat;

        #[method(widthValueTypeForLayer:edge:)]
        pub unsafe fn widthValueTypeForLayer_edge(
            &self,
            layer: NSTextBlockLayer,
            edge: NSRectEdge,
        ) -> NSTextBlockValueType;

        #[method(verticalAlignment)]
        pub unsafe fn verticalAlignment(&self) -> NSTextBlockVerticalAlignment;

        #[method(setVerticalAlignment:)]
        pub unsafe fn setVerticalAlignment(&self, verticalAlignment: NSTextBlockVerticalAlignment);

        #[method_id(backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Id<NSColor, Shared>>;

        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, backgroundColor: Option<&NSColor>);

        #[method(setBorderColor:forEdge:)]
        pub unsafe fn setBorderColor_forEdge(&self, color: Option<&NSColor>, edge: NSRectEdge);

        #[method(setBorderColor:)]
        pub unsafe fn setBorderColor(&self, color: Option<&NSColor>);

        #[method_id(borderColorForEdge:)]
        pub unsafe fn borderColorForEdge(&self, edge: NSRectEdge) -> Option<Id<NSColor, Shared>>;

        #[method(rectForLayoutAtPoint:inRect:textContainer:characterRange:)]
        pub unsafe fn rectForLayoutAtPoint_inRect_textContainer_characterRange(
            &self,
            startingPoint: NSPoint,
            rect: NSRect,
            textContainer: &NSTextContainer,
            charRange: NSRange,
        ) -> NSRect;

        #[method(boundsRectForContentRect:inRect:textContainer:characterRange:)]
        pub unsafe fn boundsRectForContentRect_inRect_textContainer_characterRange(
            &self,
            contentRect: NSRect,
            rect: NSRect,
            textContainer: &NSTextContainer,
            charRange: NSRange,
        ) -> NSRect;

        #[method(drawBackgroundWithFrame:inView:characterRange:layoutManager:)]
        pub unsafe fn drawBackgroundWithFrame_inView_characterRange_layoutManager(
            &self,
            frameRect: NSRect,
            controlView: &NSView,
            charRange: NSRange,
            layoutManager: &NSLayoutManager,
        );
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSTextTableBlock;

    unsafe impl ClassType for NSTextTableBlock {
        type Super = NSTextBlock;
    }
);

extern_methods!(
    unsafe impl NSTextTableBlock {
        #[method_id(initWithTable:startingRow:rowSpan:startingColumn:columnSpan:)]
        pub unsafe fn initWithTable_startingRow_rowSpan_startingColumn_columnSpan(
            this: Option<Allocated<Self>>,
            table: &NSTextTable,
            row: NSInteger,
            rowSpan: NSInteger,
            col: NSInteger,
            colSpan: NSInteger,
        ) -> Id<Self, Shared>;

        #[method_id(table)]
        pub unsafe fn table(&self) -> Id<NSTextTable, Shared>;

        #[method(startingRow)]
        pub unsafe fn startingRow(&self) -> NSInteger;

        #[method(rowSpan)]
        pub unsafe fn rowSpan(&self) -> NSInteger;

        #[method(startingColumn)]
        pub unsafe fn startingColumn(&self) -> NSInteger;

        #[method(columnSpan)]
        pub unsafe fn columnSpan(&self) -> NSInteger;
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSTextTable;

    unsafe impl ClassType for NSTextTable {
        type Super = NSTextBlock;
    }
);

extern_methods!(
    unsafe impl NSTextTable {
        #[method(numberOfColumns)]
        pub unsafe fn numberOfColumns(&self) -> NSUInteger;

        #[method(setNumberOfColumns:)]
        pub unsafe fn setNumberOfColumns(&self, numberOfColumns: NSUInteger);

        #[method(layoutAlgorithm)]
        pub unsafe fn layoutAlgorithm(&self) -> NSTextTableLayoutAlgorithm;

        #[method(setLayoutAlgorithm:)]
        pub unsafe fn setLayoutAlgorithm(&self, layoutAlgorithm: NSTextTableLayoutAlgorithm);

        #[method(collapsesBorders)]
        pub unsafe fn collapsesBorders(&self) -> bool;

        #[method(setCollapsesBorders:)]
        pub unsafe fn setCollapsesBorders(&self, collapsesBorders: bool);

        #[method(hidesEmptyCells)]
        pub unsafe fn hidesEmptyCells(&self) -> bool;

        #[method(setHidesEmptyCells:)]
        pub unsafe fn setHidesEmptyCells(&self, hidesEmptyCells: bool);

        #[method(rectForBlock:layoutAtPoint:inRect:textContainer:characterRange:)]
        pub unsafe fn rectForBlock_layoutAtPoint_inRect_textContainer_characterRange(
            &self,
            block: &NSTextTableBlock,
            startingPoint: NSPoint,
            rect: NSRect,
            textContainer: &NSTextContainer,
            charRange: NSRange,
        ) -> NSRect;

        #[method(boundsRectForBlock:contentRect:inRect:textContainer:characterRange:)]
        pub unsafe fn boundsRectForBlock_contentRect_inRect_textContainer_characterRange(
            &self,
            block: &NSTextTableBlock,
            contentRect: NSRect,
            rect: NSRect,
            textContainer: &NSTextContainer,
            charRange: NSRange,
        ) -> NSRect;

        #[method(drawBackgroundForBlock:withFrame:inView:characterRange:layoutManager:)]
        pub unsafe fn drawBackgroundForBlock_withFrame_inView_characterRange_layoutManager(
            &self,
            block: &NSTextTableBlock,
            frameRect: NSRect,
            controlView: &NSView,
            charRange: NSRange,
            layoutManager: &NSLayoutManager,
        );
    }
);
