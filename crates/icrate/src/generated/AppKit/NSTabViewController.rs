#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTabViewController;
    unsafe impl ClassType for NSTabViewController {
        type Super = NSViewController;
    }
);
extern_methods!(
    unsafe impl NSTabViewController {
        #[method(tabStyle)]
        pub unsafe fn tabStyle(&self) -> NSTabViewControllerTabStyle;
        #[method(setTabStyle:)]
        pub unsafe fn setTabStyle(&self, tabStyle: NSTabViewControllerTabStyle);
        #[method_id(tabView)]
        pub unsafe fn tabView(&self) -> Id<NSTabView, Shared>;
        #[method(setTabView:)]
        pub unsafe fn setTabView(&self, tabView: &NSTabView);
        #[method(transitionOptions)]
        pub unsafe fn transitionOptions(&self) -> NSViewControllerTransitionOptions;
        #[method(setTransitionOptions:)]
        pub unsafe fn setTransitionOptions(
            &self,
            transitionOptions: NSViewControllerTransitionOptions,
        );
        #[method(canPropagateSelectedChildViewControllerTitle)]
        pub unsafe fn canPropagateSelectedChildViewControllerTitle(&self) -> bool;
        #[method(setCanPropagateSelectedChildViewControllerTitle:)]
        pub unsafe fn setCanPropagateSelectedChildViewControllerTitle(
            &self,
            canPropagateSelectedChildViewControllerTitle: bool,
        );
        #[method_id(tabViewItems)]
        pub unsafe fn tabViewItems(&self) -> Id<NSArray<NSTabViewItem>, Shared>;
        #[method(setTabViewItems:)]
        pub unsafe fn setTabViewItems(&self, tabViewItems: &NSArray<NSTabViewItem>);
        #[method(selectedTabViewItemIndex)]
        pub unsafe fn selectedTabViewItemIndex(&self) -> NSInteger;
        #[method(setSelectedTabViewItemIndex:)]
        pub unsafe fn setSelectedTabViewItemIndex(&self, selectedTabViewItemIndex: NSInteger);
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
        #[method_id(tabViewItemForViewController:)]
        pub unsafe fn tabViewItemForViewController(
            &self,
            viewController: &NSViewController,
        ) -> Option<Id<NSTabViewItem, Shared>>;
        #[method(viewDidLoad)]
        pub unsafe fn viewDidLoad(&self);
        #[method(tabView:willSelectTabViewItem:)]
        pub unsafe fn tabView_willSelectTabViewItem(
            &self,
            tabView: &NSTabView,
            tabViewItem: Option<&NSTabViewItem>,
        );
        #[method(tabView:didSelectTabViewItem:)]
        pub unsafe fn tabView_didSelectTabViewItem(
            &self,
            tabView: &NSTabView,
            tabViewItem: Option<&NSTabViewItem>,
        );
        #[method(tabView:shouldSelectTabViewItem:)]
        pub unsafe fn tabView_shouldSelectTabViewItem(
            &self,
            tabView: &NSTabView,
            tabViewItem: Option<&NSTabViewItem>,
        ) -> bool;
        #[method_id(toolbar:itemForItemIdentifier:willBeInsertedIntoToolbar:)]
        pub unsafe fn toolbar_itemForItemIdentifier_willBeInsertedIntoToolbar(
            &self,
            toolbar: &NSToolbar,
            itemIdentifier: &NSToolbarItemIdentifier,
            flag: bool,
        ) -> Option<Id<NSToolbarItem, Shared>>;
        #[method_id(toolbarDefaultItemIdentifiers:)]
        pub unsafe fn toolbarDefaultItemIdentifiers(
            &self,
            toolbar: &NSToolbar,
        ) -> Id<NSArray<NSToolbarItemIdentifier>, Shared>;
        #[method_id(toolbarAllowedItemIdentifiers:)]
        pub unsafe fn toolbarAllowedItemIdentifiers(
            &self,
            toolbar: &NSToolbar,
        ) -> Id<NSArray<NSToolbarItemIdentifier>, Shared>;
        #[method_id(toolbarSelectableItemIdentifiers:)]
        pub unsafe fn toolbarSelectableItemIdentifiers(
            &self,
            toolbar: &NSToolbar,
        ) -> Id<NSArray<NSToolbarItemIdentifier>, Shared>;
    }
);