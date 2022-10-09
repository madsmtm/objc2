use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSTextFieldCell::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTableHeaderCell;
    unsafe impl ClassType for NSTableHeaderCell {
        type Super = NSTextFieldCell;
    }
);
extern_methods!(
    unsafe impl NSTableHeaderCell {
        #[method(drawSortIndicatorWithFrame:inView:ascending:priority:)]
        pub unsafe fn drawSortIndicatorWithFrame_inView_ascending_priority(
            &self,
            cellFrame: NSRect,
            controlView: &NSView,
            ascending: bool,
            priority: NSInteger,
        );
        #[method(sortIndicatorRectForBounds:)]
        pub unsafe fn sortIndicatorRectForBounds(&self, rect: NSRect) -> NSRect;
    }
);
