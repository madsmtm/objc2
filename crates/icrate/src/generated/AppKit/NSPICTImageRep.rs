#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPICTImageRep;
    unsafe impl ClassType for NSPICTImageRep {
        type Super = NSImageRep;
    }
);
extern_methods!(
    unsafe impl NSPICTImageRep {
        #[method_id(imageRepWithData:)]
        pub unsafe fn imageRepWithData(pictData: &NSData) -> Option<Id<Self, Shared>>;
        #[method_id(initWithData:)]
        pub unsafe fn initWithData(&self, pictData: &NSData) -> Option<Id<Self, Shared>>;
        #[method_id(PICTRepresentation)]
        pub unsafe fn PICTRepresentation(&self) -> Id<NSData, Shared>;
        #[method(boundingBox)]
        pub unsafe fn boundingBox(&self) -> NSRect;
    }
);
