use super::__exported::NSPasteboardWriting;
use super::__exported::NSString;
use crate::AppKit::generated::AppKitDefines::*;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSGeometry::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSDraggingImageComponentKey = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSDraggingImageComponent;
    unsafe impl ClassType for NSDraggingImageComponent {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSDraggingImageComponent {
        #[method_id(draggingImageComponentWithKey:)]
        pub unsafe fn draggingImageComponentWithKey(
            key: &NSDraggingImageComponentKey,
        ) -> Id<NSDraggingImageComponent, Shared>;
        #[method_id(initWithKey:)]
        pub unsafe fn initWithKey(&self, key: &NSDraggingImageComponentKey) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(key)]
        pub unsafe fn key(&self) -> Id<NSDraggingImageComponentKey, Shared>;
        #[method(setKey:)]
        pub unsafe fn setKey(&self, key: &NSDraggingImageComponentKey);
        #[method_id(contents)]
        pub unsafe fn contents(&self) -> Option<Id<Object, Shared>>;
        #[method(setContents:)]
        pub unsafe fn setContents(&self, contents: Option<&Object>);
        #[method(frame)]
        pub unsafe fn frame(&self) -> NSRect;
        #[method(setFrame:)]
        pub unsafe fn setFrame(&self, frame: NSRect);
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSDraggingItem;
    unsafe impl ClassType for NSDraggingItem {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSDraggingItem {
        #[method_id(initWithPasteboardWriter:)]
        pub unsafe fn initWithPasteboardWriter(
            &self,
            pasteboardWriter: &NSPasteboardWriting,
        ) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(item)]
        pub unsafe fn item(&self) -> Id<Object, Shared>;
        #[method(draggingFrame)]
        pub unsafe fn draggingFrame(&self) -> NSRect;
        #[method(setDraggingFrame:)]
        pub unsafe fn setDraggingFrame(&self, draggingFrame: NSRect);
        #[method(imageComponentsProvider)]
        pub unsafe fn imageComponentsProvider(&self) -> TodoBlock;
        #[method(setImageComponentsProvider:)]
        pub unsafe fn setImageComponentsProvider(&self, imageComponentsProvider: TodoBlock);
        #[method(setDraggingFrame:contents:)]
        pub unsafe fn setDraggingFrame_contents(&self, frame: NSRect, contents: Option<&Object>);
        #[method_id(imageComponents)]
        pub unsafe fn imageComponents(
            &self,
        ) -> Option<Id<NSArray<NSDraggingImageComponent>, Shared>>;
    }
);
