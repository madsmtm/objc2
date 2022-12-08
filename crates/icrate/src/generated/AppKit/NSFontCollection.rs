//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub type NSFontCollectionVisibility = NSUInteger;
pub const NSFontCollectionVisibilityProcess: NSFontCollectionVisibility = 1 << 0;
pub const NSFontCollectionVisibilityUser: NSFontCollectionVisibility = 1 << 1;
pub const NSFontCollectionVisibilityComputer: NSFontCollectionVisibility = 1 << 2;

pub type NSFontCollectionMatchingOptionKey = NSString;

extern "C" {
    pub static NSFontCollectionIncludeDisabledFontsOption:
        &'static NSFontCollectionMatchingOptionKey;
}

extern "C" {
    pub static NSFontCollectionRemoveDuplicatesOption: &'static NSFontCollectionMatchingOptionKey;
}

extern "C" {
    pub static NSFontCollectionDisallowAutoActivationOption:
        &'static NSFontCollectionMatchingOptionKey;
}

pub type NSFontCollectionName = NSString;

extern_class!(
    #[derive(Debug)]
    pub struct NSFontCollection;

    unsafe impl ClassType for NSFontCollection {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSFontCollection {
        #[method_id(@__retain_semantics Other fontCollectionWithDescriptors:)]
        pub unsafe fn fontCollectionWithDescriptors(
            queryDescriptors: &NSArray<NSFontDescriptor>,
        ) -> Id<NSFontCollection, Shared>;

        #[method_id(@__retain_semantics Other fontCollectionWithAllAvailableDescriptors)]
        pub unsafe fn fontCollectionWithAllAvailableDescriptors() -> Id<NSFontCollection, Shared>;

        #[method_id(@__retain_semantics Other fontCollectionWithLocale:)]
        pub unsafe fn fontCollectionWithLocale(
            locale: &NSLocale,
        ) -> Option<Id<NSFontCollection, Shared>>;

        #[method(showFontCollection:withName:visibility:error:)]
        pub unsafe fn showFontCollection_withName_visibility_error(
            collection: &NSFontCollection,
            name: &NSFontCollectionName,
            visibility: NSFontCollectionVisibility,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(hideFontCollectionWithName:visibility:error:)]
        pub unsafe fn hideFontCollectionWithName_visibility_error(
            name: &NSFontCollectionName,
            visibility: NSFontCollectionVisibility,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(renameFontCollectionWithName:visibility:toName:error:)]
        pub unsafe fn renameFontCollectionWithName_visibility_toName_error(
            oldName: &NSFontCollectionName,
            visibility: NSFontCollectionVisibility,
            newName: &NSFontCollectionName,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other allFontCollectionNames)]
        pub unsafe fn allFontCollectionNames() -> Id<NSArray<NSFontCollectionName>, Shared>;

        #[method_id(@__retain_semantics Other fontCollectionWithName:)]
        pub unsafe fn fontCollectionWithName(
            name: &NSFontCollectionName,
        ) -> Option<Id<NSFontCollection, Shared>>;

        #[method_id(@__retain_semantics Other fontCollectionWithName:visibility:)]
        pub unsafe fn fontCollectionWithName_visibility(
            name: &NSFontCollectionName,
            visibility: NSFontCollectionVisibility,
        ) -> Option<Id<NSFontCollection, Shared>>;

        #[method_id(@__retain_semantics Other queryDescriptors)]
        pub unsafe fn queryDescriptors(&self) -> Option<Id<NSArray<NSFontDescriptor>, Shared>>;

        #[method_id(@__retain_semantics Other exclusionDescriptors)]
        pub unsafe fn exclusionDescriptors(&self) -> Option<Id<NSArray<NSFontDescriptor>, Shared>>;

        #[method_id(@__retain_semantics Other matchingDescriptors)]
        pub unsafe fn matchingDescriptors(&self) -> Option<Id<NSArray<NSFontDescriptor>, Shared>>;

        #[method_id(@__retain_semantics Other matchingDescriptorsWithOptions:)]
        pub unsafe fn matchingDescriptorsWithOptions(
            &self,
            options: Option<&NSDictionary<NSFontCollectionMatchingOptionKey, NSNumber>>,
        ) -> Option<Id<NSArray<NSFontDescriptor>, Shared>>;

        #[method_id(@__retain_semantics Other matchingDescriptorsForFamily:)]
        pub unsafe fn matchingDescriptorsForFamily(
            &self,
            family: &NSString,
        ) -> Option<Id<NSArray<NSFontDescriptor>, Shared>>;

        #[method_id(@__retain_semantics Other matchingDescriptorsForFamily:options:)]
        pub unsafe fn matchingDescriptorsForFamily_options(
            &self,
            family: &NSString,
            options: Option<&NSDictionary<NSFontCollectionMatchingOptionKey, NSNumber>>,
        ) -> Option<Id<NSArray<NSFontDescriptor>, Shared>>;
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSMutableFontCollection;

    unsafe impl ClassType for NSMutableFontCollection {
        type Super = NSFontCollection;
    }
);

extern_methods!(
    unsafe impl NSMutableFontCollection {
        #[method_id(@__retain_semantics Other fontCollectionWithDescriptors:)]
        pub unsafe fn fontCollectionWithDescriptors(
            queryDescriptors: &NSArray<NSFontDescriptor>,
        ) -> Id<NSMutableFontCollection, Shared>;

        #[method_id(@__retain_semantics Other fontCollectionWithAllAvailableDescriptors)]
        pub unsafe fn fontCollectionWithAllAvailableDescriptors(
        ) -> Id<NSMutableFontCollection, Shared>;

        #[method_id(@__retain_semantics Other fontCollectionWithLocale:)]
        pub unsafe fn fontCollectionWithLocale(
            locale: &NSLocale,
        ) -> Id<NSMutableFontCollection, Shared>;

        #[method_id(@__retain_semantics Other fontCollectionWithName:)]
        pub unsafe fn fontCollectionWithName(
            name: &NSFontCollectionName,
        ) -> Option<Id<NSMutableFontCollection, Shared>>;

        #[method_id(@__retain_semantics Other fontCollectionWithName:visibility:)]
        pub unsafe fn fontCollectionWithName_visibility(
            name: &NSFontCollectionName,
            visibility: NSFontCollectionVisibility,
        ) -> Option<Id<NSMutableFontCollection, Shared>>;

        #[method_id(@__retain_semantics Other queryDescriptors)]
        pub unsafe fn queryDescriptors(&self) -> Option<Id<NSArray<NSFontDescriptor>, Shared>>;

        #[method(setQueryDescriptors:)]
        pub unsafe fn setQueryDescriptors(
            &self,
            queryDescriptors: Option<&NSArray<NSFontDescriptor>>,
        );

        #[method_id(@__retain_semantics Other exclusionDescriptors)]
        pub unsafe fn exclusionDescriptors(&self) -> Option<Id<NSArray<NSFontDescriptor>, Shared>>;

        #[method(setExclusionDescriptors:)]
        pub unsafe fn setExclusionDescriptors(
            &self,
            exclusionDescriptors: Option<&NSArray<NSFontDescriptor>>,
        );

        #[method(addQueryForDescriptors:)]
        pub unsafe fn addQueryForDescriptors(&self, descriptors: &NSArray<NSFontDescriptor>);

        #[method(removeQueryForDescriptors:)]
        pub unsafe fn removeQueryForDescriptors(&self, descriptors: &NSArray<NSFontDescriptor>);
    }
);

extern "C" {
    pub static NSFontCollectionDidChangeNotification: &'static NSNotificationName;
}

pub type NSFontCollectionUserInfoKey = NSString;

extern "C" {
    pub static NSFontCollectionActionKey: &'static NSFontCollectionUserInfoKey;
}

extern "C" {
    pub static NSFontCollectionNameKey: &'static NSFontCollectionUserInfoKey;
}

extern "C" {
    pub static NSFontCollectionOldNameKey: &'static NSFontCollectionUserInfoKey;
}

extern "C" {
    pub static NSFontCollectionVisibilityKey: &'static NSFontCollectionUserInfoKey;
}

pub type NSFontCollectionActionTypeKey = NSString;

extern "C" {
    pub static NSFontCollectionWasShown: &'static NSFontCollectionActionTypeKey;
}

extern "C" {
    pub static NSFontCollectionWasHidden: &'static NSFontCollectionActionTypeKey;
}

extern "C" {
    pub static NSFontCollectionWasRenamed: &'static NSFontCollectionActionTypeKey;
}

extern "C" {
    pub static NSFontCollectionAllFonts: &'static NSFontCollectionName;
}

extern "C" {
    pub static NSFontCollectionUser: &'static NSFontCollectionName;
}

extern "C" {
    pub static NSFontCollectionFavorites: &'static NSFontCollectionName;
}

extern "C" {
    pub static NSFontCollectionRecentlyUsed: &'static NSFontCollectionName;
}
