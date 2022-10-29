#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPopUpButtonCell;
    unsafe impl ClassType for NSPopUpButtonCell {
        type Super = NSMenuItemCell;
    }
);
extern_methods!(
    unsafe impl NSPopUpButtonCell {
        #[method_id(initTextCell:pullsDown:)]
        pub unsafe fn initTextCell_pullsDown(
            &self,
            stringValue: &NSString,
            pullDown: bool,
        ) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Id<Self, Shared>;
        #[method_id(menu)]
        pub unsafe fn menu(&self) -> Option<Id<NSMenu, Shared>>;
        #[method(setMenu:)]
        pub unsafe fn setMenu(&self, menu: Option<&NSMenu>);
        #[method(pullsDown)]
        pub unsafe fn pullsDown(&self) -> bool;
        #[method(setPullsDown:)]
        pub unsafe fn setPullsDown(&self, pullsDown: bool);
        #[method(autoenablesItems)]
        pub unsafe fn autoenablesItems(&self) -> bool;
        #[method(setAutoenablesItems:)]
        pub unsafe fn setAutoenablesItems(&self, autoenablesItems: bool);
        #[method(preferredEdge)]
        pub unsafe fn preferredEdge(&self) -> NSRectEdge;
        #[method(setPreferredEdge:)]
        pub unsafe fn setPreferredEdge(&self, preferredEdge: NSRectEdge);
        #[method(usesItemFromMenu)]
        pub unsafe fn usesItemFromMenu(&self) -> bool;
        #[method(setUsesItemFromMenu:)]
        pub unsafe fn setUsesItemFromMenu(&self, usesItemFromMenu: bool);
        #[method(altersStateOfSelectedItem)]
        pub unsafe fn altersStateOfSelectedItem(&self) -> bool;
        #[method(setAltersStateOfSelectedItem:)]
        pub unsafe fn setAltersStateOfSelectedItem(&self, altersStateOfSelectedItem: bool);
        #[method(addItemWithTitle:)]
        pub unsafe fn addItemWithTitle(&self, title: &NSString);
        #[method(addItemsWithTitles:)]
        pub unsafe fn addItemsWithTitles(&self, itemTitles: &NSArray<NSString>);
        #[method(insertItemWithTitle:atIndex:)]
        pub unsafe fn insertItemWithTitle_atIndex(&self, title: &NSString, index: NSInteger);
        #[method(removeItemWithTitle:)]
        pub unsafe fn removeItemWithTitle(&self, title: &NSString);
        #[method(removeItemAtIndex:)]
        pub unsafe fn removeItemAtIndex(&self, index: NSInteger);
        #[method(removeAllItems)]
        pub unsafe fn removeAllItems(&self);
        #[method_id(itemArray)]
        pub unsafe fn itemArray(&self) -> Id<NSArray<NSMenuItem>, Shared>;
        #[method(numberOfItems)]
        pub unsafe fn numberOfItems(&self) -> NSInteger;
        #[method(indexOfItem:)]
        pub unsafe fn indexOfItem(&self, item: &NSMenuItem) -> NSInteger;
        #[method(indexOfItemWithTitle:)]
        pub unsafe fn indexOfItemWithTitle(&self, title: &NSString) -> NSInteger;
        #[method(indexOfItemWithTag:)]
        pub unsafe fn indexOfItemWithTag(&self, tag: NSInteger) -> NSInteger;
        #[method(indexOfItemWithRepresentedObject:)]
        pub unsafe fn indexOfItemWithRepresentedObject(&self, obj: Option<&Object>) -> NSInteger;
        #[method(indexOfItemWithTarget:andAction:)]
        pub unsafe fn indexOfItemWithTarget_andAction(
            &self,
            target: Option<&Object>,
            actionSelector: Option<Sel>,
        ) -> NSInteger;
        #[method_id(itemAtIndex:)]
        pub unsafe fn itemAtIndex(&self, index: NSInteger) -> Option<Id<NSMenuItem, Shared>>;
        #[method_id(itemWithTitle:)]
        pub unsafe fn itemWithTitle(&self, title: &NSString) -> Option<Id<NSMenuItem, Shared>>;
        #[method_id(lastItem)]
        pub unsafe fn lastItem(&self) -> Option<Id<NSMenuItem, Shared>>;
        #[method(selectItem:)]
        pub unsafe fn selectItem(&self, item: Option<&NSMenuItem>);
        #[method(selectItemAtIndex:)]
        pub unsafe fn selectItemAtIndex(&self, index: NSInteger);
        #[method(selectItemWithTitle:)]
        pub unsafe fn selectItemWithTitle(&self, title: &NSString);
        #[method(selectItemWithTag:)]
        pub unsafe fn selectItemWithTag(&self, tag: NSInteger) -> bool;
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, string: Option<&NSString>);
        #[method_id(selectedItem)]
        pub unsafe fn selectedItem(&self) -> Option<Id<NSMenuItem, Shared>>;
        #[method(indexOfSelectedItem)]
        pub unsafe fn indexOfSelectedItem(&self) -> NSInteger;
        #[method(synchronizeTitleAndSelectedItem)]
        pub unsafe fn synchronizeTitleAndSelectedItem(&self);
        #[method_id(itemTitleAtIndex:)]
        pub unsafe fn itemTitleAtIndex(&self, index: NSInteger) -> Id<NSString, Shared>;
        #[method_id(itemTitles)]
        pub unsafe fn itemTitles(&self) -> Id<NSArray<NSString>, Shared>;
        #[method_id(titleOfSelectedItem)]
        pub unsafe fn titleOfSelectedItem(&self) -> Option<Id<NSString, Shared>>;
        #[method(attachPopUpWithFrame:inView:)]
        pub unsafe fn attachPopUpWithFrame_inView(&self, cellFrame: NSRect, controlView: &NSView);
        #[method(dismissPopUp)]
        pub unsafe fn dismissPopUp(&self);
        #[method(performClickWithFrame:inView:)]
        pub unsafe fn performClickWithFrame_inView(&self, frame: NSRect, controlView: &NSView);
        #[method(arrowPosition)]
        pub unsafe fn arrowPosition(&self) -> NSPopUpArrowPosition;
        #[method(setArrowPosition:)]
        pub unsafe fn setArrowPosition(&self, arrowPosition: NSPopUpArrowPosition);
    }
);