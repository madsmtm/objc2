use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSSplitView::*;
use crate::AppKit::generated::NSSplitViewItem::*;
use crate::AppKit::generated::NSViewController::*;
use crate::Foundation::generated::NSArray::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSSplitViewController;
    unsafe impl ClassType for NSSplitViewController {
        type Super = NSViewController;
    }
);
extern_methods!(
    unsafe impl NSSplitViewController {
        #[method_id(splitView)]
        pub unsafe fn splitView(&self) -> Id<NSSplitView, Shared>;
        #[method(setSplitView:)]
        pub unsafe fn setSplitView(&self, splitView: &NSSplitView);
        #[method_id(splitViewItems)]
        pub unsafe fn splitViewItems(&self) -> Id<NSArray<NSSplitViewItem>, Shared>;
        #[method(setSplitViewItems:)]
        pub unsafe fn setSplitViewItems(&self, splitViewItems: &NSArray<NSSplitViewItem>);
        #[method(addSplitViewItem:)]
        pub unsafe fn addSplitViewItem(&self, splitViewItem: &NSSplitViewItem);
        #[method(insertSplitViewItem:atIndex:)]
        pub unsafe fn insertSplitViewItem_atIndex(
            &self,
            splitViewItem: &NSSplitViewItem,
            index: NSInteger,
        );
        #[method(removeSplitViewItem:)]
        pub unsafe fn removeSplitViewItem(&self, splitViewItem: &NSSplitViewItem);
        #[method_id(splitViewItemForViewController:)]
        pub unsafe fn splitViewItemForViewController(
            &self,
            viewController: &NSViewController,
        ) -> Option<Id<NSSplitViewItem, Shared>>;
        #[method(minimumThicknessForInlineSidebars)]
        pub unsafe fn minimumThicknessForInlineSidebars(&self) -> CGFloat;
        #[method(setMinimumThicknessForInlineSidebars:)]
        pub unsafe fn setMinimumThicknessForInlineSidebars(
            &self,
            minimumThicknessForInlineSidebars: CGFloat,
        );
        #[method(validateUserInterfaceItem:)]
        pub unsafe fn validateUserInterfaceItem(&self, item: &NSValidatedUserInterfaceItem)
            -> bool;
        #[method(viewDidLoad)]
        pub unsafe fn viewDidLoad(&self);
        #[method(splitView:canCollapseSubview:)]
        pub unsafe fn splitView_canCollapseSubview(
            &self,
            splitView: &NSSplitView,
            subview: &NSView,
        ) -> bool;
        #[method(splitView:shouldCollapseSubview:forDoubleClickOnDividerAtIndex:)]
        pub unsafe fn splitView_shouldCollapseSubview_forDoubleClickOnDividerAtIndex(
            &self,
            splitView: &NSSplitView,
            subview: &NSView,
            dividerIndex: NSInteger,
        ) -> bool;
        #[method(splitView:shouldHideDividerAtIndex:)]
        pub unsafe fn splitView_shouldHideDividerAtIndex(
            &self,
            splitView: &NSSplitView,
            dividerIndex: NSInteger,
        ) -> bool;
        #[method(splitView:effectiveRect:forDrawnRect:ofDividerAtIndex:)]
        pub unsafe fn splitView_effectiveRect_forDrawnRect_ofDividerAtIndex(
            &self,
            splitView: &NSSplitView,
            proposedEffectiveRect: NSRect,
            drawnRect: NSRect,
            dividerIndex: NSInteger,
        ) -> NSRect;
        #[method(splitView:additionalEffectiveRectOfDividerAtIndex:)]
        pub unsafe fn splitView_additionalEffectiveRectOfDividerAtIndex(
            &self,
            splitView: &NSSplitView,
            dividerIndex: NSInteger,
        ) -> NSRect;
    }
);
extern_methods!(
    #[doc = "NSSplitViewControllerToggleSidebarAction"]
    unsafe impl NSSplitViewController {
        #[method(toggleSidebar:)]
        pub unsafe fn toggleSidebar(&self, sender: Option<&Object>);
    }
);
