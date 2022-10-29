#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPathCell;
    unsafe impl ClassType for NSPathCell {
        type Super = NSActionCell;
    }
);
extern_methods!(
    unsafe impl NSPathCell {
        #[method(pathStyle)]
        pub unsafe fn pathStyle(&self) -> NSPathStyle;
        #[method(setPathStyle:)]
        pub unsafe fn setPathStyle(&self, pathStyle: NSPathStyle);
        #[method_id(URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL, Shared>>;
        #[method(setURL:)]
        pub unsafe fn setURL(&self, URL: Option<&NSURL>);
        #[method(setObjectValue:)]
        pub unsafe fn setObjectValue(&self, obj: Option<&NSCopying>);
        #[method_id(allowedTypes)]
        pub unsafe fn allowedTypes(&self) -> Option<Id<NSArray<NSString>, Shared>>;
        #[method(setAllowedTypes:)]
        pub unsafe fn setAllowedTypes(&self, allowedTypes: Option<&NSArray<NSString>>);
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSPathCellDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSPathCellDelegate>);
        #[method(pathComponentCellClass)]
        pub unsafe fn pathComponentCellClass() -> &Class;
        #[method_id(pathComponentCells)]
        pub unsafe fn pathComponentCells(&self) -> Id<NSArray<NSPathComponentCell>, Shared>;
        #[method(setPathComponentCells:)]
        pub unsafe fn setPathComponentCells(
            &self,
            pathComponentCells: &NSArray<NSPathComponentCell>,
        );
        #[method(rectOfPathComponentCell:withFrame:inView:)]
        pub unsafe fn rectOfPathComponentCell_withFrame_inView(
            &self,
            cell: &NSPathComponentCell,
            frame: NSRect,
            view: &NSView,
        ) -> NSRect;
        #[method_id(pathComponentCellAtPoint:withFrame:inView:)]
        pub unsafe fn pathComponentCellAtPoint_withFrame_inView(
            &self,
            point: NSPoint,
            frame: NSRect,
            view: &NSView,
        ) -> Option<Id<NSPathComponentCell, Shared>>;
        #[method_id(clickedPathComponentCell)]
        pub unsafe fn clickedPathComponentCell(&self) -> Option<Id<NSPathComponentCell, Shared>>;
        #[method(mouseEntered:withFrame:inView:)]
        pub unsafe fn mouseEntered_withFrame_inView(
            &self,
            event: &NSEvent,
            frame: NSRect,
            view: &NSView,
        );
        #[method(mouseExited:withFrame:inView:)]
        pub unsafe fn mouseExited_withFrame_inView(
            &self,
            event: &NSEvent,
            frame: NSRect,
            view: &NSView,
        );
        #[method(doubleAction)]
        pub unsafe fn doubleAction(&self) -> Option<Sel>;
        #[method(setDoubleAction:)]
        pub unsafe fn setDoubleAction(&self, doubleAction: Option<Sel>);
        #[method_id(backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Id<NSColor, Shared>>;
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, backgroundColor: Option<&NSColor>);
        #[method_id(placeholderString)]
        pub unsafe fn placeholderString(&self) -> Option<Id<NSString, Shared>>;
        #[method(setPlaceholderString:)]
        pub unsafe fn setPlaceholderString(&self, placeholderString: Option<&NSString>);
        #[method_id(placeholderAttributedString)]
        pub unsafe fn placeholderAttributedString(&self) -> Option<Id<NSAttributedString, Shared>>;
        #[method(setPlaceholderAttributedString:)]
        pub unsafe fn setPlaceholderAttributedString(
            &self,
            placeholderAttributedString: Option<&NSAttributedString>,
        );
    }
);
pub type NSPathCellDelegate = NSObject;