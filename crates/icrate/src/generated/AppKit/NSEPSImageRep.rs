#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSEPSImageRep;
    unsafe impl ClassType for NSEPSImageRep {
        type Super = NSImageRep;
    }
);
extern_methods!(
    unsafe impl NSEPSImageRep {
        #[method_id(imageRepWithData:)]
        pub unsafe fn imageRepWithData(epsData: &NSData) -> Option<Id<Self, Shared>>;
        #[method_id(initWithData:)]
        pub unsafe fn initWithData(&self, epsData: &NSData) -> Option<Id<Self, Shared>>;
        #[method(prepareGState)]
        pub unsafe fn prepareGState(&self);
        #[method_id(EPSRepresentation)]
        pub unsafe fn EPSRepresentation(&self) -> Id<NSData, Shared>;
        #[method(boundingBox)]
        pub unsafe fn boundingBox(&self) -> NSRect;
    }
);
