#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPathControl;
    unsafe impl ClassType for NSPathControl {
        type Super = NSControl;
    }
);
extern_methods!(
    unsafe impl NSPathControl {
        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;
        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);
        #[method_id(allowedTypes)]
        pub unsafe fn allowedTypes(&self) -> Option<Id<NSArray<NSString>, Shared>>;
        #[method(setAllowedTypes:)]
        pub unsafe fn setAllowedTypes(&self, allowedTypes: Option<&NSArray<NSString>>);
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
        #[method_id(URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL, Shared>>;
        #[method(setURL:)]
        pub unsafe fn setURL(&self, URL: Option<&NSURL>);
        #[method(doubleAction)]
        pub unsafe fn doubleAction(&self) -> Option<Sel>;
        #[method(setDoubleAction:)]
        pub unsafe fn setDoubleAction(&self, doubleAction: Option<Sel>);
        #[method(pathStyle)]
        pub unsafe fn pathStyle(&self) -> NSPathStyle;
        #[method(setPathStyle:)]
        pub unsafe fn setPathStyle(&self, pathStyle: NSPathStyle);
        #[method_id(clickedPathItem)]
        pub unsafe fn clickedPathItem(&self) -> Option<Id<NSPathControlItem, Shared>>;
        #[method_id(pathItems)]
        pub unsafe fn pathItems(&self) -> Id<NSArray<NSPathControlItem>, Shared>;
        #[method(setPathItems:)]
        pub unsafe fn setPathItems(&self, pathItems: &NSArray<NSPathControlItem>);
        #[method_id(backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Id<NSColor, Shared>>;
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, backgroundColor: Option<&NSColor>);
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSPathControlDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSPathControlDelegate>);
        #[method(setDraggingSourceOperationMask:forLocal:)]
        pub unsafe fn setDraggingSourceOperationMask_forLocal(
            &self,
            mask: NSDragOperation,
            isLocal: bool,
        );
        #[method_id(menu)]
        pub unsafe fn menu(&self) -> Option<Id<NSMenu, Shared>>;
        #[method(setMenu:)]
        pub unsafe fn setMenu(&self, menu: Option<&NSMenu>);
    }
);
pub type NSPathControlDelegate = NSObject;
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSPathControl {
        #[method_id(clickedPathComponentCell)]
        pub unsafe fn clickedPathComponentCell(&self) -> Option<Id<NSPathComponentCell, Shared>>;
        #[method_id(pathComponentCells)]
        pub unsafe fn pathComponentCells(&self) -> Id<NSArray<NSPathComponentCell>, Shared>;
        #[method(setPathComponentCells:)]
        pub unsafe fn setPathComponentCells(&self, cells: &NSArray<NSPathComponentCell>);
    }
);