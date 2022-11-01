//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

pub type NSColorListName = NSString;

pub type NSColorName = NSString;

extern_class!(
    #[derive(Debug)]
    pub struct NSColorList;

    unsafe impl ClassType for NSColorList {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSColorList {
        #[method_id(availableColorLists)]
        pub unsafe fn availableColorLists() -> Id<NSArray<NSColorList>, Shared>;

        #[method_id(colorListNamed:)]
        pub unsafe fn colorListNamed(name: &NSColorListName) -> Option<Id<NSColorList, Shared>>;

        #[method_id(initWithName:)]
        pub unsafe fn initWithName(&self, name: &NSColorListName) -> Id<Self, Shared>;

        #[method_id(initWithName:fromFile:)]
        pub unsafe fn initWithName_fromFile(
            &self,
            name: &NSColorListName,
            path: Option<&NSString>,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(name)]
        pub unsafe fn name(&self) -> Option<Id<NSColorListName, Shared>>;

        #[method(setColor:forKey:)]
        pub unsafe fn setColor_forKey(&self, color: &NSColor, key: &NSColorName);

        #[method(insertColor:key:atIndex:)]
        pub unsafe fn insertColor_key_atIndex(
            &self,
            color: &NSColor,
            key: &NSColorName,
            loc: NSUInteger,
        );

        #[method(removeColorWithKey:)]
        pub unsafe fn removeColorWithKey(&self, key: &NSColorName);

        #[method_id(colorWithKey:)]
        pub unsafe fn colorWithKey(&self, key: &NSColorName) -> Option<Id<NSColor, Shared>>;

        #[method_id(allKeys)]
        pub unsafe fn allKeys(&self) -> Id<NSArray<NSColorName>, Shared>;

        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[method(writeToURL:error:)]
        pub unsafe fn writeToURL_error(
            &self,
            url: Option<&NSURL>,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(writeToFile:)]
        pub unsafe fn writeToFile(&self, path: Option<&NSString>) -> bool;

        #[method(removeFile)]
        pub unsafe fn removeFile(&self);
    }
);

extern "C" {
    static NSColorListDidChangeNotification: &'static NSNotificationName;
}
