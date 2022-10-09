use super::__exported::NSLayoutConstraint;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSCell::*;
use crate::AppKit::generated::NSTableView::*;
use crate::AppKit::generated::NSView::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTableRowView;
    unsafe impl ClassType for NSTableRowView {
        type Super = NSView;
    }
);
extern_methods!(
    unsafe impl NSTableRowView {
        #[method(selectionHighlightStyle)]
        pub unsafe fn selectionHighlightStyle(&self) -> NSTableViewSelectionHighlightStyle;
        #[method(setSelectionHighlightStyle:)]
        pub unsafe fn setSelectionHighlightStyle(
            &self,
            selectionHighlightStyle: NSTableViewSelectionHighlightStyle,
        );
        #[method(isEmphasized)]
        pub unsafe fn isEmphasized(&self) -> bool;
        #[method(setEmphasized:)]
        pub unsafe fn setEmphasized(&self, emphasized: bool);
        #[method(isGroupRowStyle)]
        pub unsafe fn isGroupRowStyle(&self) -> bool;
        #[method(setGroupRowStyle:)]
        pub unsafe fn setGroupRowStyle(&self, groupRowStyle: bool);
        #[method(isSelected)]
        pub unsafe fn isSelected(&self) -> bool;
        #[method(setSelected:)]
        pub unsafe fn setSelected(&self, selected: bool);
        #[method(isPreviousRowSelected)]
        pub unsafe fn isPreviousRowSelected(&self) -> bool;
        #[method(setPreviousRowSelected:)]
        pub unsafe fn setPreviousRowSelected(&self, previousRowSelected: bool);
        #[method(isNextRowSelected)]
        pub unsafe fn isNextRowSelected(&self) -> bool;
        #[method(setNextRowSelected:)]
        pub unsafe fn setNextRowSelected(&self, nextRowSelected: bool);
        #[method(isFloating)]
        pub unsafe fn isFloating(&self) -> bool;
        #[method(setFloating:)]
        pub unsafe fn setFloating(&self, floating: bool);
        #[method(isTargetForDropOperation)]
        pub unsafe fn isTargetForDropOperation(&self) -> bool;
        #[method(setTargetForDropOperation:)]
        pub unsafe fn setTargetForDropOperation(&self, targetForDropOperation: bool);
        #[method(draggingDestinationFeedbackStyle)]
        pub unsafe fn draggingDestinationFeedbackStyle(
            &self,
        ) -> NSTableViewDraggingDestinationFeedbackStyle;
        #[method(setDraggingDestinationFeedbackStyle:)]
        pub unsafe fn setDraggingDestinationFeedbackStyle(
            &self,
            draggingDestinationFeedbackStyle: NSTableViewDraggingDestinationFeedbackStyle,
        );
        #[method(indentationForDropOperation)]
        pub unsafe fn indentationForDropOperation(&self) -> CGFloat;
        #[method(setIndentationForDropOperation:)]
        pub unsafe fn setIndentationForDropOperation(&self, indentationForDropOperation: CGFloat);
        #[method(interiorBackgroundStyle)]
        pub unsafe fn interiorBackgroundStyle(&self) -> NSBackgroundStyle;
        #[method_id(backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<NSColor, Shared>;
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, backgroundColor: &NSColor);
        #[method(drawBackgroundInRect:)]
        pub unsafe fn drawBackgroundInRect(&self, dirtyRect: NSRect);
        #[method(drawSelectionInRect:)]
        pub unsafe fn drawSelectionInRect(&self, dirtyRect: NSRect);
        #[method(drawSeparatorInRect:)]
        pub unsafe fn drawSeparatorInRect(&self, dirtyRect: NSRect);
        #[method(drawDraggingDestinationFeedbackInRect:)]
        pub unsafe fn drawDraggingDestinationFeedbackInRect(&self, dirtyRect: NSRect);
        #[method_id(viewAtColumn:)]
        pub unsafe fn viewAtColumn(&self, column: NSInteger) -> Option<Id<Object, Shared>>;
        #[method(numberOfColumns)]
        pub unsafe fn numberOfColumns(&self) -> NSInteger;
    }
);
