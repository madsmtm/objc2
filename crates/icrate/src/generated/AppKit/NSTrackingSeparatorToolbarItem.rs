#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTrackingSeparatorToolbarItem;
    unsafe impl ClassType for NSTrackingSeparatorToolbarItem {
        type Super = NSToolbarItem;
    }
);
extern_methods!(
    unsafe impl NSTrackingSeparatorToolbarItem {
        #[method_id(trackingSeparatorToolbarItemWithIdentifier:splitView:dividerIndex:)]
        pub unsafe fn trackingSeparatorToolbarItemWithIdentifier_splitView_dividerIndex(
            identifier: &NSToolbarItemIdentifier,
            splitView: &NSSplitView,
            dividerIndex: NSInteger,
        ) -> Id<Self, Shared>;
        #[method_id(splitView)]
        pub unsafe fn splitView(&self) -> Id<NSSplitView, Shared>;
        #[method(setSplitView:)]
        pub unsafe fn setSplitView(&self, splitView: &NSSplitView);
        #[method(dividerIndex)]
        pub unsafe fn dividerIndex(&self) -> NSInteger;
        #[method(setDividerIndex:)]
        pub unsafe fn setDividerIndex(&self, dividerIndex: NSInteger);
    }
);
