use super::__exported::NSData;
use super::__exported::NSFileWrapper;
use super::__exported::NSMutableDictionary;
use crate::AppKit::generated::AppKitDefines::*;
use crate::CoreFoundation::generated::CFBase::*;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSDictionary::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSString::*;
use crate::Foundation::generated::NSURL::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSPasteboardType = NSString;
pub type NSPasteboardName = NSString;
pub type NSPasteboardReadingOptionKey = NSString;
use super::__exported::NSPasteboardItem;
extern_class!(
    #[derive(Debug)]
    pub struct NSPasteboard;
    unsafe impl ClassType for NSPasteboard {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSPasteboard {
        #[method_id(generalPasteboard)]
        pub unsafe fn generalPasteboard() -> Id<NSPasteboard, Shared>;
        #[method_id(pasteboardWithName:)]
        pub unsafe fn pasteboardWithName(name: &NSPasteboardName) -> Id<NSPasteboard, Shared>;
        #[method_id(pasteboardWithUniqueName)]
        pub unsafe fn pasteboardWithUniqueName() -> Id<NSPasteboard, Shared>;
        #[method_id(name)]
        pub unsafe fn name(&self) -> Id<NSPasteboardName, Shared>;
        #[method(changeCount)]
        pub unsafe fn changeCount(&self) -> NSInteger;
        #[method(prepareForNewContentsWithOptions:)]
        pub unsafe fn prepareForNewContentsWithOptions(
            &self,
            options: NSPasteboardContentsOptions,
        ) -> NSInteger;
        #[method(clearContents)]
        pub unsafe fn clearContents(&self) -> NSInteger;
        #[method(writeObjects:)]
        pub unsafe fn writeObjects(&self, objects: &NSArray<NSPasteboardWriting>) -> bool;
        #[method_id(readObjectsForClasses:options:)]
        pub unsafe fn readObjectsForClasses_options(
            &self,
            classArray: &NSArray<TodoClass>,
            options: Option<&NSDictionary<NSPasteboardReadingOptionKey, Object>>,
        ) -> Option<Id<NSArray, Shared>>;
        #[method_id(pasteboardItems)]
        pub unsafe fn pasteboardItems(&self) -> Option<Id<NSArray<NSPasteboardItem>, Shared>>;
        #[method(indexOfPasteboardItem:)]
        pub unsafe fn indexOfPasteboardItem(&self, pasteboardItem: &NSPasteboardItem)
            -> NSUInteger;
        #[method(canReadItemWithDataConformingToTypes:)]
        pub unsafe fn canReadItemWithDataConformingToTypes(
            &self,
            types: &NSArray<NSString>,
        ) -> bool;
        #[method(canReadObjectForClasses:options:)]
        pub unsafe fn canReadObjectForClasses_options(
            &self,
            classArray: &NSArray<TodoClass>,
            options: Option<&NSDictionary<NSPasteboardReadingOptionKey, Object>>,
        ) -> bool;
        #[method(declareTypes:owner:)]
        pub unsafe fn declareTypes_owner(
            &self,
            newTypes: &NSArray<NSPasteboardType>,
            newOwner: Option<&Object>,
        ) -> NSInteger;
        #[method(addTypes:owner:)]
        pub unsafe fn addTypes_owner(
            &self,
            newTypes: &NSArray<NSPasteboardType>,
            newOwner: Option<&Object>,
        ) -> NSInteger;
        #[method_id(types)]
        pub unsafe fn types(&self) -> Option<Id<NSArray<NSPasteboardType>, Shared>>;
        #[method_id(availableTypeFromArray:)]
        pub unsafe fn availableTypeFromArray(
            &self,
            types: &NSArray<NSPasteboardType>,
        ) -> Option<Id<NSPasteboardType, Shared>>;
        #[method(setData:forType:)]
        pub unsafe fn setData_forType(
            &self,
            data: Option<&NSData>,
            dataType: &NSPasteboardType,
        ) -> bool;
        #[method(setPropertyList:forType:)]
        pub unsafe fn setPropertyList_forType(
            &self,
            plist: &Object,
            dataType: &NSPasteboardType,
        ) -> bool;
        #[method(setString:forType:)]
        pub unsafe fn setString_forType(
            &self,
            string: &NSString,
            dataType: &NSPasteboardType,
        ) -> bool;
        #[method_id(dataForType:)]
        pub unsafe fn dataForType(&self, dataType: &NSPasteboardType)
            -> Option<Id<NSData, Shared>>;
        #[method_id(propertyListForType:)]
        pub unsafe fn propertyListForType(
            &self,
            dataType: &NSPasteboardType,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(stringForType:)]
        pub unsafe fn stringForType(
            &self,
            dataType: &NSPasteboardType,
        ) -> Option<Id<NSString, Shared>>;
    }
);
extern_methods!(
    #[doc = "FilterServices"]
    unsafe impl NSPasteboard {
        #[method_id(typesFilterableTo:)]
        pub unsafe fn typesFilterableTo(
            type_: &NSPasteboardType,
        ) -> Id<NSArray<NSPasteboardType>, Shared>;
        #[method_id(pasteboardByFilteringFile:)]
        pub unsafe fn pasteboardByFilteringFile(filename: &NSString) -> Id<NSPasteboard, Shared>;
        #[method_id(pasteboardByFilteringData:ofType:)]
        pub unsafe fn pasteboardByFilteringData_ofType(
            data: &NSData,
            type_: &NSPasteboardType,
        ) -> Id<NSPasteboard, Shared>;
        #[method_id(pasteboardByFilteringTypesInPasteboard:)]
        pub unsafe fn pasteboardByFilteringTypesInPasteboard(
            pboard: &NSPasteboard,
        ) -> Id<NSPasteboard, Shared>;
    }
);
pub type NSPasteboardTypeOwner = NSObject;
extern_methods!(
    #[doc = "NSPasteboardOwner"]
    unsafe impl NSObject {
        #[method(pasteboard:provideDataForType:)]
        pub unsafe fn pasteboard_provideDataForType(
            &self,
            sender: &NSPasteboard,
            type_: &NSPasteboardType,
        );
        #[method(pasteboardChangedOwner:)]
        pub unsafe fn pasteboardChangedOwner(&self, sender: &NSPasteboard);
    }
);
pub type NSPasteboardWriting = NSObject;
pub type NSPasteboardReading = NSObject;
extern_methods!(
    #[doc = "NSPasteboardSupport"]
    unsafe impl NSURL {
        #[method_id(URLFromPasteboard:)]
        pub unsafe fn URLFromPasteboard(pasteBoard: &NSPasteboard) -> Option<Id<NSURL, Shared>>;
        #[method(writeToPasteboard:)]
        pub unsafe fn writeToPasteboard(&self, pasteBoard: &NSPasteboard);
    }
);
extern_methods!(
    #[doc = "NSPasteboardSupport"]
    unsafe impl NSString {}
);
extern_methods!(
    #[doc = "NSFileContents"]
    unsafe impl NSPasteboard {
        #[method(writeFileContents:)]
        pub unsafe fn writeFileContents(&self, filename: &NSString) -> bool;
        #[method_id(readFileContentsType:toFile:)]
        pub unsafe fn readFileContentsType_toFile(
            &self,
            type_: Option<&NSPasteboardType>,
            filename: &NSString,
        ) -> Option<Id<NSString, Shared>>;
        #[method(writeFileWrapper:)]
        pub unsafe fn writeFileWrapper(&self, wrapper: &NSFileWrapper) -> bool;
        #[method_id(readFileWrapper)]
        pub unsafe fn readFileWrapper(&self) -> Option<Id<NSFileWrapper, Shared>>;
    }
);
