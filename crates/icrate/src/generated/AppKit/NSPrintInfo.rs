use super::__exported::NSPDFInfo;
use super::__exported::NSPrinter;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSPrinter::*;
use crate::Foundation::generated::NSDictionary::*;
use crate::Foundation::generated::NSGeometry::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSPrintInfoAttributeKey = NSString;
pub type NSPrintJobDispositionValue = NSString;
pub type NSPrintInfoSettingKey = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSPrintInfo;
    unsafe impl ClassType for NSPrintInfo {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSPrintInfo {
        #[method_id(sharedPrintInfo)]
        pub unsafe fn sharedPrintInfo() -> Id<NSPrintInfo, Shared>;
        #[method(setSharedPrintInfo:)]
        pub unsafe fn setSharedPrintInfo(sharedPrintInfo: &NSPrintInfo);
        #[method_id(initWithDictionary:)]
        pub unsafe fn initWithDictionary(
            &self,
            attributes: &NSDictionary<NSPrintInfoAttributeKey, Object>,
        ) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(dictionary)]
        pub unsafe fn dictionary(
            &self,
        ) -> Id<NSMutableDictionary<NSPrintInfoAttributeKey, Object>, Shared>;
        #[method_id(paperName)]
        pub unsafe fn paperName(&self) -> Option<Id<NSPrinterPaperName, Shared>>;
        #[method(setPaperName:)]
        pub unsafe fn setPaperName(&self, paperName: Option<&NSPrinterPaperName>);
        #[method(paperSize)]
        pub unsafe fn paperSize(&self) -> NSSize;
        #[method(setPaperSize:)]
        pub unsafe fn setPaperSize(&self, paperSize: NSSize);
        #[method(orientation)]
        pub unsafe fn orientation(&self) -> NSPaperOrientation;
        #[method(setOrientation:)]
        pub unsafe fn setOrientation(&self, orientation: NSPaperOrientation);
        #[method(scalingFactor)]
        pub unsafe fn scalingFactor(&self) -> CGFloat;
        #[method(setScalingFactor:)]
        pub unsafe fn setScalingFactor(&self, scalingFactor: CGFloat);
        #[method(leftMargin)]
        pub unsafe fn leftMargin(&self) -> CGFloat;
        #[method(setLeftMargin:)]
        pub unsafe fn setLeftMargin(&self, leftMargin: CGFloat);
        #[method(rightMargin)]
        pub unsafe fn rightMargin(&self) -> CGFloat;
        #[method(setRightMargin:)]
        pub unsafe fn setRightMargin(&self, rightMargin: CGFloat);
        #[method(topMargin)]
        pub unsafe fn topMargin(&self) -> CGFloat;
        #[method(setTopMargin:)]
        pub unsafe fn setTopMargin(&self, topMargin: CGFloat);
        #[method(bottomMargin)]
        pub unsafe fn bottomMargin(&self) -> CGFloat;
        #[method(setBottomMargin:)]
        pub unsafe fn setBottomMargin(&self, bottomMargin: CGFloat);
        #[method(isHorizontallyCentered)]
        pub unsafe fn isHorizontallyCentered(&self) -> bool;
        #[method(setHorizontallyCentered:)]
        pub unsafe fn setHorizontallyCentered(&self, horizontallyCentered: bool);
        #[method(isVerticallyCentered)]
        pub unsafe fn isVerticallyCentered(&self) -> bool;
        #[method(setVerticallyCentered:)]
        pub unsafe fn setVerticallyCentered(&self, verticallyCentered: bool);
        #[method(horizontalPagination)]
        pub unsafe fn horizontalPagination(&self) -> NSPrintingPaginationMode;
        #[method(setHorizontalPagination:)]
        pub unsafe fn setHorizontalPagination(
            &self,
            horizontalPagination: NSPrintingPaginationMode,
        );
        #[method(verticalPagination)]
        pub unsafe fn verticalPagination(&self) -> NSPrintingPaginationMode;
        #[method(setVerticalPagination:)]
        pub unsafe fn setVerticalPagination(&self, verticalPagination: NSPrintingPaginationMode);
        #[method_id(jobDisposition)]
        pub unsafe fn jobDisposition(&self) -> Id<NSPrintJobDispositionValue, Shared>;
        #[method(setJobDisposition:)]
        pub unsafe fn setJobDisposition(&self, jobDisposition: &NSPrintJobDispositionValue);
        #[method_id(printer)]
        pub unsafe fn printer(&self) -> Id<NSPrinter, Shared>;
        #[method(setPrinter:)]
        pub unsafe fn setPrinter(&self, printer: &NSPrinter);
        #[method(setUpPrintOperationDefaultValues)]
        pub unsafe fn setUpPrintOperationDefaultValues(&self);
        #[method(imageablePageBounds)]
        pub unsafe fn imageablePageBounds(&self) -> NSRect;
        #[method_id(localizedPaperName)]
        pub unsafe fn localizedPaperName(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(defaultPrinter)]
        pub unsafe fn defaultPrinter() -> Option<Id<NSPrinter, Shared>>;
        #[method_id(printSettings)]
        pub unsafe fn printSettings(
            &self,
        ) -> Id<NSMutableDictionary<NSPrintInfoSettingKey, Object>, Shared>;
        #[method(PMPrintSession)]
        pub unsafe fn PMPrintSession(&self) -> NonNull<c_void>;
        #[method(PMPageFormat)]
        pub unsafe fn PMPageFormat(&self) -> NonNull<c_void>;
        #[method(PMPrintSettings)]
        pub unsafe fn PMPrintSettings(&self) -> NonNull<c_void>;
        #[method(updateFromPMPageFormat)]
        pub unsafe fn updateFromPMPageFormat(&self);
        #[method(updateFromPMPrintSettings)]
        pub unsafe fn updateFromPMPrintSettings(&self);
        #[method(isSelectionOnly)]
        pub unsafe fn isSelectionOnly(&self) -> bool;
        #[method(setSelectionOnly:)]
        pub unsafe fn setSelectionOnly(&self, selectionOnly: bool);
        #[method(takeSettingsFromPDFInfo:)]
        pub unsafe fn takeSettingsFromPDFInfo(&self, inPDFInfo: &NSPDFInfo);
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSPrintInfo {
        #[method(setDefaultPrinter:)]
        pub unsafe fn setDefaultPrinter(printer: Option<&NSPrinter>);
        #[method(sizeForPaperName:)]
        pub unsafe fn sizeForPaperName(name: Option<&NSPrinterPaperName>) -> NSSize;
    }
);
