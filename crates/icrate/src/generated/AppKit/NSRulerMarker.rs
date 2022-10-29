#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSRulerMarker;
    unsafe impl ClassType for NSRulerMarker {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSRulerMarker {
        #[method_id(initWithRulerView:markerLocation:image:imageOrigin:)]
        pub unsafe fn initWithRulerView_markerLocation_image_imageOrigin(
            &self,
            ruler: &NSRulerView,
            location: CGFloat,
            image: &NSImage,
            imageOrigin: NSPoint,
        ) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(ruler)]
        pub unsafe fn ruler(&self) -> Option<Id<NSRulerView, Shared>>;
        #[method(markerLocation)]
        pub unsafe fn markerLocation(&self) -> CGFloat;
        #[method(setMarkerLocation:)]
        pub unsafe fn setMarkerLocation(&self, markerLocation: CGFloat);
        #[method_id(image)]
        pub unsafe fn image(&self) -> Id<NSImage, Shared>;
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: &NSImage);
        #[method(imageOrigin)]
        pub unsafe fn imageOrigin(&self) -> NSPoint;
        #[method(setImageOrigin:)]
        pub unsafe fn setImageOrigin(&self, imageOrigin: NSPoint);
        #[method(isMovable)]
        pub unsafe fn isMovable(&self) -> bool;
        #[method(setMovable:)]
        pub unsafe fn setMovable(&self, movable: bool);
        #[method(isRemovable)]
        pub unsafe fn isRemovable(&self) -> bool;
        #[method(setRemovable:)]
        pub unsafe fn setRemovable(&self, removable: bool);
        #[method(isDragging)]
        pub unsafe fn isDragging(&self) -> bool;
        #[method_id(representedObject)]
        pub unsafe fn representedObject(&self) -> Option<Id<NSCopying, Shared>>;
        #[method(setRepresentedObject:)]
        pub unsafe fn setRepresentedObject(&self, representedObject: Option<&NSCopying>);
        #[method(imageRectInRuler)]
        pub unsafe fn imageRectInRuler(&self) -> NSRect;
        #[method(thicknessRequiredInRuler)]
        pub unsafe fn thicknessRequiredInRuler(&self) -> CGFloat;
        #[method(drawRect:)]
        pub unsafe fn drawRect(&self, rect: NSRect);
        #[method(trackMouse:adding:)]
        pub unsafe fn trackMouse_adding(&self, mouseDownEvent: &NSEvent, isAdding: bool) -> bool;
    }
);