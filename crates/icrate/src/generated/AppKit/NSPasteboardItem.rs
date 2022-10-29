#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPasteboardItem;
    unsafe impl ClassType for NSPasteboardItem {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSPasteboardItem {
        #[method_id(types)]
        pub unsafe fn types(&self) -> Id<NSArray<NSPasteboardType>, Shared>;
        #[method_id(availableTypeFromArray:)]
        pub unsafe fn availableTypeFromArray(
            &self,
            types: &NSArray<NSPasteboardType>,
        ) -> Option<Id<NSPasteboardType, Shared>>;
        #[method(setDataProvider:forTypes:)]
        pub unsafe fn setDataProvider_forTypes(
            &self,
            dataProvider: &NSPasteboardItemDataProvider,
            types: &NSArray<NSPasteboardType>,
        ) -> bool;
        #[method(setData:forType:)]
        pub unsafe fn setData_forType(&self, data: &NSData, type_: &NSPasteboardType) -> bool;
        #[method(setString:forType:)]
        pub unsafe fn setString_forType(&self, string: &NSString, type_: &NSPasteboardType)
            -> bool;
        #[method(setPropertyList:forType:)]
        pub unsafe fn setPropertyList_forType(
            &self,
            propertyList: &Object,
            type_: &NSPasteboardType,
        ) -> bool;
        #[method_id(dataForType:)]
        pub unsafe fn dataForType(&self, type_: &NSPasteboardType) -> Option<Id<NSData, Shared>>;
        #[method_id(stringForType:)]
        pub unsafe fn stringForType(
            &self,
            type_: &NSPasteboardType,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(propertyListForType:)]
        pub unsafe fn propertyListForType(
            &self,
            type_: &NSPasteboardType,
        ) -> Option<Id<Object, Shared>>;
    }
);
pub type NSPasteboardItemDataProvider = NSObject;