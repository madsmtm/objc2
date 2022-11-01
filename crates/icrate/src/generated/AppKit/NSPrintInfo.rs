//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

pub type NSPaperOrientation = NSInteger;
pub const NSPaperOrientationPortrait: NSPaperOrientation = 0;
pub const NSPaperOrientationLandscape: NSPaperOrientation = 1;

pub type NSPrintingPaginationMode = NSUInteger;
pub const NSPrintingPaginationModeAutomatic: NSPrintingPaginationMode = 0;
pub const NSPrintingPaginationModeFit: NSPrintingPaginationMode = 1;
pub const NSPrintingPaginationModeClip: NSPrintingPaginationMode = 2;

pub type NSPrintInfoAttributeKey = NSString;

extern "C" {
    pub static NSPrintPaperName: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintPaperSize: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintOrientation: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintScalingFactor: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintLeftMargin: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintRightMargin: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintTopMargin: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintBottomMargin: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintHorizontallyCentered: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintVerticallyCentered: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintHorizontalPagination: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintVerticalPagination: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintPrinter: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintCopies: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintAllPages: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintFirstPage: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintLastPage: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintMustCollate: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintReversePageOrder: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintJobDisposition: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintPagesAcross: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintPagesDown: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintTime: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintDetailedErrorReporting: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintFaxNumber: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintPrinterName: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintSelectionOnly: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintJobSavingURL: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintJobSavingFileNameExtensionHidden: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintHeaderAndFooter: &'static NSPrintInfoAttributeKey;
}

pub type NSPrintJobDispositionValue = NSString;

extern "C" {
    pub static NSPrintSpoolJob: &'static NSPrintJobDispositionValue;
}

extern "C" {
    pub static NSPrintPreviewJob: &'static NSPrintJobDispositionValue;
}

extern "C" {
    pub static NSPrintSaveJob: &'static NSPrintJobDispositionValue;
}

extern "C" {
    pub static NSPrintCancelJob: &'static NSPrintJobDispositionValue;
}

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
            this: Option<Allocated<Self>>,
            attributes: &NSDictionary<NSPrintInfoAttributeKey, Object>,
        ) -> Id<Self, Shared>;

        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Id<Self, Shared>;

        #[method_id(init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

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
    /// NSDeprecated
    unsafe impl NSPrintInfo {
        #[method(setDefaultPrinter:)]
        pub unsafe fn setDefaultPrinter(printer: Option<&NSPrinter>);

        #[method(sizeForPaperName:)]
        pub unsafe fn sizeForPaperName(name: Option<&NSPrinterPaperName>) -> NSSize;
    }
);

extern "C" {
    pub static NSPrintFormName: &'static NSString;
}

extern "C" {
    pub static NSPrintJobFeatures: &'static NSString;
}

extern "C" {
    pub static NSPrintManualFeed: &'static NSString;
}

extern "C" {
    pub static NSPrintPagesPerSheet: &'static NSString;
}

extern "C" {
    pub static NSPrintPaperFeed: &'static NSString;
}

extern "C" {
    pub static NSPrintSavePath: &'static NSString;
}

pub type NSPrintingOrientation = NSUInteger;
pub const NSPortraitOrientation: NSPrintingOrientation = 0;
pub const NSLandscapeOrientation: NSPrintingOrientation = 1;

pub static NSAutoPagination: NSPrintingPaginationMode = NSPrintingPaginationModeAutomatic;

pub static NSFitPagination: NSPrintingPaginationMode = NSPrintingPaginationModeFit;

pub static NSClipPagination: NSPrintingPaginationMode = NSPrintingPaginationModeClip;
