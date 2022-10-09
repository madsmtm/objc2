use super::__exported::NSCell;
use super::__exported::NSImage;
use super::__exported::NSSortDescriptor;
use super::__exported::NSTableHeaderCell;
use super::__exported::NSTableView;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSUserInterfaceItemIdentification::*;
use crate::Foundation::generated::NSGeometry::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTableColumn;
    unsafe impl ClassType for NSTableColumn {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSTableColumn {
        #[method_id(initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            &self,
            identifier: &NSUserInterfaceItemIdentifier,
        ) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Id<Self, Shared>;
        #[method_id(identifier)]
        pub unsafe fn identifier(&self) -> Id<NSUserInterfaceItemIdentifier, Shared>;
        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: &NSUserInterfaceItemIdentifier);
        #[method_id(tableView)]
        pub unsafe fn tableView(&self) -> Option<Id<NSTableView, Shared>>;
        #[method(setTableView:)]
        pub unsafe fn setTableView(&self, tableView: Option<&NSTableView>);
        #[method(width)]
        pub unsafe fn width(&self) -> CGFloat;
        #[method(setWidth:)]
        pub unsafe fn setWidth(&self, width: CGFloat);
        #[method(minWidth)]
        pub unsafe fn minWidth(&self) -> CGFloat;
        #[method(setMinWidth:)]
        pub unsafe fn setMinWidth(&self, minWidth: CGFloat);
        #[method(maxWidth)]
        pub unsafe fn maxWidth(&self) -> CGFloat;
        #[method(setMaxWidth:)]
        pub unsafe fn setMaxWidth(&self, maxWidth: CGFloat);
        #[method_id(title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);
        #[method_id(headerCell)]
        pub unsafe fn headerCell(&self) -> Id<NSTableHeaderCell, Shared>;
        #[method(setHeaderCell:)]
        pub unsafe fn setHeaderCell(&self, headerCell: &NSTableHeaderCell);
        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;
        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);
        #[method(sizeToFit)]
        pub unsafe fn sizeToFit(&self);
        #[method_id(sortDescriptorPrototype)]
        pub unsafe fn sortDescriptorPrototype(&self) -> Option<Id<NSSortDescriptor, Shared>>;
        #[method(setSortDescriptorPrototype:)]
        pub unsafe fn setSortDescriptorPrototype(
            &self,
            sortDescriptorPrototype: Option<&NSSortDescriptor>,
        );
        #[method(resizingMask)]
        pub unsafe fn resizingMask(&self) -> NSTableColumnResizingOptions;
        #[method(setResizingMask:)]
        pub unsafe fn setResizingMask(&self, resizingMask: NSTableColumnResizingOptions);
        #[method_id(headerToolTip)]
        pub unsafe fn headerToolTip(&self) -> Option<Id<NSString, Shared>>;
        #[method(setHeaderToolTip:)]
        pub unsafe fn setHeaderToolTip(&self, headerToolTip: Option<&NSString>);
        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;
        #[method(setHidden:)]
        pub unsafe fn setHidden(&self, hidden: bool);
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSTableColumn {
        #[method(setResizable:)]
        pub unsafe fn setResizable(&self, flag: bool);
        #[method(isResizable)]
        pub unsafe fn isResizable(&self) -> bool;
        #[method_id(dataCell)]
        pub unsafe fn dataCell(&self) -> Id<Object, Shared>;
        #[method(setDataCell:)]
        pub unsafe fn setDataCell(&self, dataCell: &Object);
        #[method_id(dataCellForRow:)]
        pub unsafe fn dataCellForRow(&self, row: NSInteger) -> Id<Object, Shared>;
    }
);
