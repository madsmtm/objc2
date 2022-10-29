#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTableHeaderView;
    unsafe impl ClassType for NSTableHeaderView {
        type Super = NSView;
    }
);
extern_methods!(
    unsafe impl NSTableHeaderView {
        #[method_id(tableView)]
        pub unsafe fn tableView(&self) -> Option<Id<NSTableView, Shared>>;
        #[method(setTableView:)]
        pub unsafe fn setTableView(&self, tableView: Option<&NSTableView>);
        #[method(draggedColumn)]
        pub unsafe fn draggedColumn(&self) -> NSInteger;
        #[method(draggedDistance)]
        pub unsafe fn draggedDistance(&self) -> CGFloat;
        #[method(resizedColumn)]
        pub unsafe fn resizedColumn(&self) -> NSInteger;
        #[method(headerRectOfColumn:)]
        pub unsafe fn headerRectOfColumn(&self, column: NSInteger) -> NSRect;
        #[method(columnAtPoint:)]
        pub unsafe fn columnAtPoint(&self, point: NSPoint) -> NSInteger;
    }
);
