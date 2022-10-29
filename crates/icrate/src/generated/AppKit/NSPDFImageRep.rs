#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPDFImageRep;
    unsafe impl ClassType for NSPDFImageRep {
        type Super = NSImageRep;
    }
);
extern_methods!(
    unsafe impl NSPDFImageRep {
        #[method_id(imageRepWithData:)]
        pub unsafe fn imageRepWithData(pdfData: &NSData) -> Option<Id<Self, Shared>>;
        #[method_id(initWithData:)]
        pub unsafe fn initWithData(&self, pdfData: &NSData) -> Option<Id<Self, Shared>>;
        #[method_id(PDFRepresentation)]
        pub unsafe fn PDFRepresentation(&self) -> Id<NSData, Shared>;
        #[method(bounds)]
        pub unsafe fn bounds(&self) -> NSRect;
        #[method(currentPage)]
        pub unsafe fn currentPage(&self) -> NSInteger;
        #[method(setCurrentPage:)]
        pub unsafe fn setCurrentPage(&self, currentPage: NSInteger);
        #[method(pageCount)]
        pub unsafe fn pageCount(&self) -> NSInteger;
    }
);