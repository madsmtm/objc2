#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSSplitViewItem;
    unsafe impl ClassType for NSSplitViewItem {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSSplitViewItem {
        #[method_id(splitViewItemWithViewController:)]
        pub unsafe fn splitViewItemWithViewController(
            viewController: &NSViewController,
        ) -> Id<Self, Shared>;
        #[method_id(sidebarWithViewController:)]
        pub unsafe fn sidebarWithViewController(
            viewController: &NSViewController,
        ) -> Id<Self, Shared>;
        #[method_id(contentListWithViewController:)]
        pub unsafe fn contentListWithViewController(
            viewController: &NSViewController,
        ) -> Id<Self, Shared>;
        #[method(behavior)]
        pub unsafe fn behavior(&self) -> NSSplitViewItemBehavior;
        #[method_id(viewController)]
        pub unsafe fn viewController(&self) -> Id<NSViewController, Shared>;
        #[method(setViewController:)]
        pub unsafe fn setViewController(&self, viewController: &NSViewController);
        #[method(isCollapsed)]
        pub unsafe fn isCollapsed(&self) -> bool;
        #[method(setCollapsed:)]
        pub unsafe fn setCollapsed(&self, collapsed: bool);
        #[method(canCollapse)]
        pub unsafe fn canCollapse(&self) -> bool;
        #[method(setCanCollapse:)]
        pub unsafe fn setCanCollapse(&self, canCollapse: bool);
        #[method(collapseBehavior)]
        pub unsafe fn collapseBehavior(&self) -> NSSplitViewItemCollapseBehavior;
        #[method(setCollapseBehavior:)]
        pub unsafe fn setCollapseBehavior(&self, collapseBehavior: NSSplitViewItemCollapseBehavior);
        #[method(minimumThickness)]
        pub unsafe fn minimumThickness(&self) -> CGFloat;
        #[method(setMinimumThickness:)]
        pub unsafe fn setMinimumThickness(&self, minimumThickness: CGFloat);
        #[method(maximumThickness)]
        pub unsafe fn maximumThickness(&self) -> CGFloat;
        #[method(setMaximumThickness:)]
        pub unsafe fn setMaximumThickness(&self, maximumThickness: CGFloat);
        #[method(preferredThicknessFraction)]
        pub unsafe fn preferredThicknessFraction(&self) -> CGFloat;
        #[method(setPreferredThicknessFraction:)]
        pub unsafe fn setPreferredThicknessFraction(&self, preferredThicknessFraction: CGFloat);
        #[method(holdingPriority)]
        pub unsafe fn holdingPriority(&self) -> NSLayoutPriority;
        #[method(setHoldingPriority:)]
        pub unsafe fn setHoldingPriority(&self, holdingPriority: NSLayoutPriority);
        #[method(automaticMaximumThickness)]
        pub unsafe fn automaticMaximumThickness(&self) -> CGFloat;
        #[method(setAutomaticMaximumThickness:)]
        pub unsafe fn setAutomaticMaximumThickness(&self, automaticMaximumThickness: CGFloat);
        #[method(isSpringLoaded)]
        pub unsafe fn isSpringLoaded(&self) -> bool;
        #[method(setSpringLoaded:)]
        pub unsafe fn setSpringLoaded(&self, springLoaded: bool);
        #[method(allowsFullHeightLayout)]
        pub unsafe fn allowsFullHeightLayout(&self) -> bool;
        #[method(setAllowsFullHeightLayout:)]
        pub unsafe fn setAllowsFullHeightLayout(&self, allowsFullHeightLayout: bool);
        #[method(titlebarSeparatorStyle)]
        pub unsafe fn titlebarSeparatorStyle(&self) -> NSTitlebarSeparatorStyle;
        #[method(setTitlebarSeparatorStyle:)]
        pub unsafe fn setTitlebarSeparatorStyle(
            &self,
            titlebarSeparatorStyle: NSTitlebarSeparatorStyle,
        );
    }
);
