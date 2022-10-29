#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTabView;
    unsafe impl ClassType for NSTabView {
        type Super = NSView;
    }
);
extern_methods!(
    unsafe impl NSTabView {
        #[method(selectTabViewItem:)]
        pub unsafe fn selectTabViewItem(&self, tabViewItem: Option<&NSTabViewItem>);
        #[method(selectTabViewItemAtIndex:)]
        pub unsafe fn selectTabViewItemAtIndex(&self, index: NSInteger);
        #[method(selectTabViewItemWithIdentifier:)]
        pub unsafe fn selectTabViewItemWithIdentifier(&self, identifier: &Object);
        #[method(takeSelectedTabViewItemFromSender:)]
        pub unsafe fn takeSelectedTabViewItemFromSender(&self, sender: Option<&Object>);
        #[method(selectFirstTabViewItem:)]
        pub unsafe fn selectFirstTabViewItem(&self, sender: Option<&Object>);
        #[method(selectLastTabViewItem:)]
        pub unsafe fn selectLastTabViewItem(&self, sender: Option<&Object>);
        #[method(selectNextTabViewItem:)]
        pub unsafe fn selectNextTabViewItem(&self, sender: Option<&Object>);
        #[method(selectPreviousTabViewItem:)]
        pub unsafe fn selectPreviousTabViewItem(&self, sender: Option<&Object>);
        #[method_id(selectedTabViewItem)]
        pub unsafe fn selectedTabViewItem(&self) -> Option<Id<NSTabViewItem, Shared>>;
        #[method_id(font)]
        pub unsafe fn font(&self) -> Id<NSFont, Shared>;
        #[method(setFont:)]
        pub unsafe fn setFont(&self, font: &NSFont);
        #[method(tabViewType)]
        pub unsafe fn tabViewType(&self) -> NSTabViewType;
        #[method(setTabViewType:)]
        pub unsafe fn setTabViewType(&self, tabViewType: NSTabViewType);
        #[method(tabPosition)]
        pub unsafe fn tabPosition(&self) -> NSTabPosition;
        #[method(setTabPosition:)]
        pub unsafe fn setTabPosition(&self, tabPosition: NSTabPosition);
        #[method(tabViewBorderType)]
        pub unsafe fn tabViewBorderType(&self) -> NSTabViewBorderType;
        #[method(setTabViewBorderType:)]
        pub unsafe fn setTabViewBorderType(&self, tabViewBorderType: NSTabViewBorderType);
        #[method_id(tabViewItems)]
        pub unsafe fn tabViewItems(&self) -> Id<NSArray<NSTabViewItem>, Shared>;
        #[method(setTabViewItems:)]
        pub unsafe fn setTabViewItems(&self, tabViewItems: &NSArray<NSTabViewItem>);
        #[method(allowsTruncatedLabels)]
        pub unsafe fn allowsTruncatedLabels(&self) -> bool;
        #[method(setAllowsTruncatedLabels:)]
        pub unsafe fn setAllowsTruncatedLabels(&self, allowsTruncatedLabels: bool);
        #[method(minimumSize)]
        pub unsafe fn minimumSize(&self) -> NSSize;
        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;
        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, drawsBackground: bool);
        #[method(controlSize)]
        pub unsafe fn controlSize(&self) -> NSControlSize;
        #[method(setControlSize:)]
        pub unsafe fn setControlSize(&self, controlSize: NSControlSize);
        #[method(addTabViewItem:)]
        pub unsafe fn addTabViewItem(&self, tabViewItem: &NSTabViewItem);
        #[method(insertTabViewItem:atIndex:)]
        pub unsafe fn insertTabViewItem_atIndex(
            &self,
            tabViewItem: &NSTabViewItem,
            index: NSInteger,
        );
        #[method(removeTabViewItem:)]
        pub unsafe fn removeTabViewItem(&self, tabViewItem: &NSTabViewItem);
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSTabViewDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSTabViewDelegate>);
        #[method_id(tabViewItemAtPoint:)]
        pub unsafe fn tabViewItemAtPoint(
            &self,
            point: NSPoint,
        ) -> Option<Id<NSTabViewItem, Shared>>;
        #[method(contentRect)]
        pub unsafe fn contentRect(&self) -> NSRect;
        #[method(numberOfTabViewItems)]
        pub unsafe fn numberOfTabViewItems(&self) -> NSInteger;
        #[method(indexOfTabViewItem:)]
        pub unsafe fn indexOfTabViewItem(&self, tabViewItem: &NSTabViewItem) -> NSInteger;
        #[method_id(tabViewItemAtIndex:)]
        pub unsafe fn tabViewItemAtIndex(&self, index: NSInteger) -> Id<NSTabViewItem, Shared>;
        #[method(indexOfTabViewItemWithIdentifier:)]
        pub unsafe fn indexOfTabViewItemWithIdentifier(&self, identifier: &Object) -> NSInteger;
        #[method(controlTint)]
        pub unsafe fn controlTint(&self) -> NSControlTint;
        #[method(setControlTint:)]
        pub unsafe fn setControlTint(&self, controlTint: NSControlTint);
    }
);
pub type NSTabViewDelegate = NSObject;
