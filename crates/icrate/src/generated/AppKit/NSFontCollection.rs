#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSFontCollectionMatchingOptionKey = NSString;
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
        #[method_id(fontCollectionWithDescriptors:)]
        pub unsafe fn fontCollectionWithDescriptors(
            queryDescriptors: &NSArray<NSFontDescriptor>,
        ) -> Id<NSFontCollection, Shared>;
        #[method_id(fontCollectionWithAllAvailableDescriptors)]
        pub unsafe fn fontCollectionWithAllAvailableDescriptors() -> Id<NSFontCollection, Shared>;
        #[method_id(fontCollectionWithLocale:)]
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
        #[method_id(allFontCollectionNames)]
        pub unsafe fn allFontCollectionNames() -> Id<NSArray<NSFontCollectionName>, Shared>;
        #[method_id(fontCollectionWithName:)]
        pub unsafe fn fontCollectionWithName(
            name: &NSFontCollectionName,
        ) -> Option<Id<NSFontCollection, Shared>>;
        #[method_id(fontCollectionWithName:visibility:)]
        pub unsafe fn fontCollectionWithName_visibility(
            name: &NSFontCollectionName,
            visibility: NSFontCollectionVisibility,
        ) -> Option<Id<NSFontCollection, Shared>>;
        #[method_id(queryDescriptors)]
        pub unsafe fn queryDescriptors(&self) -> Option<Id<NSArray<NSFontDescriptor>, Shared>>;
        #[method_id(exclusionDescriptors)]
        pub unsafe fn exclusionDescriptors(&self) -> Option<Id<NSArray<NSFontDescriptor>, Shared>>;
        #[method_id(matchingDescriptors)]
        pub unsafe fn matchingDescriptors(&self) -> Option<Id<NSArray<NSFontDescriptor>, Shared>>;
        #[method_id(matchingDescriptorsWithOptions:)]
        pub unsafe fn matchingDescriptorsWithOptions(
            &self,
            options: Option<&NSDictionary<NSFontCollectionMatchingOptionKey, NSNumber>>,
        ) -> Option<Id<NSArray<NSFontDescriptor>, Shared>>;
        #[method_id(matchingDescriptorsForFamily:)]
        pub unsafe fn matchingDescriptorsForFamily(
            &self,
            family: &NSString,
        ) -> Option<Id<NSArray<NSFontDescriptor>, Shared>>;
        #[method_id(matchingDescriptorsForFamily:options:)]
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
        #[method_id(fontCollectionWithDescriptors:)]
        pub unsafe fn fontCollectionWithDescriptors(
            queryDescriptors: &NSArray<NSFontDescriptor>,
        ) -> Id<NSMutableFontCollection, Shared>;
        #[method_id(fontCollectionWithAllAvailableDescriptors)]
        pub unsafe fn fontCollectionWithAllAvailableDescriptors(
        ) -> Id<NSMutableFontCollection, Shared>;
        #[method_id(fontCollectionWithLocale:)]
        pub unsafe fn fontCollectionWithLocale(
            locale: &NSLocale,
        ) -> Id<NSMutableFontCollection, Shared>;
        #[method_id(fontCollectionWithName:)]
        pub unsafe fn fontCollectionWithName(
            name: &NSFontCollectionName,
        ) -> Option<Id<NSMutableFontCollection, Shared>>;
        #[method_id(fontCollectionWithName:visibility:)]
        pub unsafe fn fontCollectionWithName_visibility(
            name: &NSFontCollectionName,
            visibility: NSFontCollectionVisibility,
        ) -> Option<Id<NSMutableFontCollection, Shared>>;
        #[method_id(queryDescriptors)]
        pub unsafe fn queryDescriptors(&self) -> Option<Id<NSArray<NSFontDescriptor>, Shared>>;
        #[method(setQueryDescriptors:)]
        pub unsafe fn setQueryDescriptors(
            &self,
            queryDescriptors: Option<&NSArray<NSFontDescriptor>>,
        );
        #[method_id(exclusionDescriptors)]
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
pub type NSFontCollectionUserInfoKey = NSString;
pub type NSFontCollectionActionTypeKey = NSString;