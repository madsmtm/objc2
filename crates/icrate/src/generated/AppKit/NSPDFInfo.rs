#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPDFInfo;
    unsafe impl ClassType for NSPDFInfo {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSPDFInfo {
        #[method_id(URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL, Shared>>;
        #[method(setURL:)]
        pub unsafe fn setURL(&self, URL: Option<&NSURL>);
        #[method(isFileExtensionHidden)]
        pub unsafe fn isFileExtensionHidden(&self) -> bool;
        #[method(setFileExtensionHidden:)]
        pub unsafe fn setFileExtensionHidden(&self, fileExtensionHidden: bool);
        #[method_id(tagNames)]
        pub unsafe fn tagNames(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(setTagNames:)]
        pub unsafe fn setTagNames(&self, tagNames: &NSArray<NSString>);
        #[method(orientation)]
        pub unsafe fn orientation(&self) -> NSPaperOrientation;
        #[method(setOrientation:)]
        pub unsafe fn setOrientation(&self, orientation: NSPaperOrientation);
        #[method(paperSize)]
        pub unsafe fn paperSize(&self) -> NSSize;
        #[method(setPaperSize:)]
        pub unsafe fn setPaperSize(&self, paperSize: NSSize);
        #[method_id(attributes)]
        pub unsafe fn attributes(
            &self,
        ) -> Id<NSMutableDictionary<NSPrintInfoAttributeKey, Object>, Shared>;
    }
);
