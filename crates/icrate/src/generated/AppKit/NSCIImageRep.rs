#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSCIImageRep;
    unsafe impl ClassType for NSCIImageRep {
        type Super = NSImageRep;
    }
);
extern_methods!(
    unsafe impl NSCIImageRep {
        #[method_id(imageRepWithCIImage:)]
        pub unsafe fn imageRepWithCIImage(image: &CIImage) -> Id<Self, Shared>;
        #[method_id(initWithCIImage:)]
        pub unsafe fn initWithCIImage(&self, image: &CIImage) -> Id<Self, Shared>;
        #[method_id(CIImage)]
        pub unsafe fn CIImage(&self) -> Id<CIImage, Shared>;
    }
);
extern_methods!(
    #[doc = "NSAppKitAdditions"]
    unsafe impl CIImage {
        #[method_id(initWithBitmapImageRep:)]
        pub unsafe fn initWithBitmapImageRep(
            &self,
            bitmapImageRep: &NSBitmapImageRep,
        ) -> Option<Id<Self, Shared>>;
        #[method(drawInRect:fromRect:operation:fraction:)]
        pub unsafe fn drawInRect_fromRect_operation_fraction(
            &self,
            rect: NSRect,
            fromRect: NSRect,
            op: NSCompositingOperation,
            delta: CGFloat,
        );
        #[method(drawAtPoint:fromRect:operation:fraction:)]
        pub unsafe fn drawAtPoint_fromRect_operation_fraction(
            &self,
            point: NSPoint,
            fromRect: NSRect,
            op: NSCompositingOperation,
            delta: CGFloat,
        );
    }
);
