#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSPrinterTypeName = NSString;
pub type NSPrinterPaperName = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSPrinter;
    unsafe impl ClassType for NSPrinter {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSPrinter {
        #[method_id(printerNames)]
        pub unsafe fn printerNames() -> Id<NSArray<NSString>, Shared>;
        #[method_id(printerTypes)]
        pub unsafe fn printerTypes() -> Id<NSArray<NSPrinterTypeName>, Shared>;
        #[method_id(printerWithName:)]
        pub unsafe fn printerWithName(name: &NSString) -> Option<Id<NSPrinter, Shared>>;
        #[method_id(printerWithType:)]
        pub unsafe fn printerWithType(type_: &NSPrinterTypeName) -> Option<Id<NSPrinter, Shared>>;
        #[method_id(name)]
        pub unsafe fn name(&self) -> Id<NSString, Shared>;
        #[method_id(type)]
        pub unsafe fn type_(&self) -> Id<NSPrinterTypeName, Shared>;
        #[method(languageLevel)]
        pub unsafe fn languageLevel(&self) -> NSInteger;
        #[method(pageSizeForPaper:)]
        pub unsafe fn pageSizeForPaper(&self, paperName: &NSPrinterPaperName) -> NSSize;
        #[method_id(deviceDescription)]
        pub unsafe fn deviceDescription(
            &self,
        ) -> Id<NSDictionary<NSDeviceDescriptionKey, Object>, Shared>;
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSPrinter {
        #[method(statusForTable:)]
        pub unsafe fn statusForTable(&self, tableName: &NSString) -> NSPrinterTableStatus;
        #[method(isKey:inTable:)]
        pub unsafe fn isKey_inTable(&self, key: Option<&NSString>, table: &NSString) -> bool;
        #[method(booleanForKey:inTable:)]
        pub unsafe fn booleanForKey_inTable(
            &self,
            key: Option<&NSString>,
            table: &NSString,
        ) -> bool;
        #[method(floatForKey:inTable:)]
        pub unsafe fn floatForKey_inTable(
            &self,
            key: Option<&NSString>,
            table: &NSString,
        ) -> c_float;
        #[method(intForKey:inTable:)]
        pub unsafe fn intForKey_inTable(&self, key: Option<&NSString>, table: &NSString) -> c_int;
        #[method(rectForKey:inTable:)]
        pub unsafe fn rectForKey_inTable(&self, key: Option<&NSString>, table: &NSString)
            -> NSRect;
        #[method(sizeForKey:inTable:)]
        pub unsafe fn sizeForKey_inTable(&self, key: Option<&NSString>, table: &NSString)
            -> NSSize;
        #[method_id(stringForKey:inTable:)]
        pub unsafe fn stringForKey_inTable(
            &self,
            key: Option<&NSString>,
            table: &NSString,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(stringListForKey:inTable:)]
        pub unsafe fn stringListForKey_inTable(
            &self,
            key: Option<&NSString>,
            table: &NSString,
        ) -> Option<Id<NSArray, Shared>>;
        #[method(imageRectForPaper:)]
        pub unsafe fn imageRectForPaper(&self, paperName: Option<&NSString>) -> NSRect;
        #[method(acceptsBinary)]
        pub unsafe fn acceptsBinary(&self) -> bool;
        #[method(isColor)]
        pub unsafe fn isColor(&self) -> bool;
        #[method(isFontAvailable:)]
        pub unsafe fn isFontAvailable(&self, faceName: Option<&NSString>) -> bool;
        #[method(isOutputStackInReverseOrder)]
        pub unsafe fn isOutputStackInReverseOrder(&self) -> bool;
        #[method_id(printerWithName:domain:includeUnavailable:)]
        pub unsafe fn printerWithName_domain_includeUnavailable(
            name: &NSString,
            domain: Option<&NSString>,
            flag: bool,
        ) -> Option<Id<NSPrinter, Shared>>;
        #[method_id(domain)]
        pub unsafe fn domain(&self) -> Id<NSString, Shared>;
        #[method_id(host)]
        pub unsafe fn host(&self) -> Id<NSString, Shared>;
        #[method_id(note)]
        pub unsafe fn note(&self) -> Id<NSString, Shared>;
    }
);