use super::__exported::NSArray;
use super::__exported::NSDate;
use super::__exported::NSDictionary;
use super::__exported::NSError;
use super::__exported::NSPersonNameComponents;
use super::__exported::NSString;
use super::__exported::NSURL;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSFileVersion;
    unsafe impl ClassType for NSFileVersion {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSFileVersion {
        #[method_id(currentVersionOfItemAtURL:)]
        pub unsafe fn currentVersionOfItemAtURL(url: &NSURL) -> Option<Id<NSFileVersion, Shared>>;
        #[method_id(otherVersionsOfItemAtURL:)]
        pub unsafe fn otherVersionsOfItemAtURL(
            url: &NSURL,
        ) -> Option<Id<NSArray<NSFileVersion>, Shared>>;
        #[method_id(unresolvedConflictVersionsOfItemAtURL:)]
        pub unsafe fn unresolvedConflictVersionsOfItemAtURL(
            url: &NSURL,
        ) -> Option<Id<NSArray<NSFileVersion>, Shared>>;
        #[method(getNonlocalVersionsOfItemAtURL:completionHandler:)]
        pub unsafe fn getNonlocalVersionsOfItemAtURL_completionHandler(
            url: &NSURL,
            completionHandler: TodoBlock,
        );
        #[method_id(versionOfItemAtURL:forPersistentIdentifier:)]
        pub unsafe fn versionOfItemAtURL_forPersistentIdentifier(
            url: &NSURL,
            persistentIdentifier: &Object,
        ) -> Option<Id<NSFileVersion, Shared>>;
        #[method_id(addVersionOfItemAtURL:withContentsOfURL:options:error:)]
        pub unsafe fn addVersionOfItemAtURL_withContentsOfURL_options_error(
            url: &NSURL,
            contentsURL: &NSURL,
            options: NSFileVersionAddingOptions,
        ) -> Result<Id<NSFileVersion, Shared>, Id<NSError, Shared>>;
        #[method_id(temporaryDirectoryURLForNewVersionOfItemAtURL:)]
        pub unsafe fn temporaryDirectoryURLForNewVersionOfItemAtURL(
            url: &NSURL,
        ) -> Id<NSURL, Shared>;
        #[method_id(URL)]
        pub unsafe fn URL(&self) -> Id<NSURL, Shared>;
        #[method_id(localizedName)]
        pub unsafe fn localizedName(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(localizedNameOfSavingComputer)]
        pub unsafe fn localizedNameOfSavingComputer(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(originatorNameComponents)]
        pub unsafe fn originatorNameComponents(&self)
            -> Option<Id<NSPersonNameComponents, Shared>>;
        #[method_id(modificationDate)]
        pub unsafe fn modificationDate(&self) -> Option<Id<NSDate, Shared>>;
        #[method_id(persistentIdentifier)]
        pub unsafe fn persistentIdentifier(&self) -> Id<NSCoding, Shared>;
        #[method(isConflict)]
        pub unsafe fn isConflict(&self) -> bool;
        #[method(isResolved)]
        pub unsafe fn isResolved(&self) -> bool;
        #[method(setResolved:)]
        pub unsafe fn setResolved(&self, resolved: bool);
        #[method(isDiscardable)]
        pub unsafe fn isDiscardable(&self) -> bool;
        #[method(setDiscardable:)]
        pub unsafe fn setDiscardable(&self, discardable: bool);
        #[method(hasLocalContents)]
        pub unsafe fn hasLocalContents(&self) -> bool;
        #[method(hasThumbnail)]
        pub unsafe fn hasThumbnail(&self) -> bool;
        #[method_id(replaceItemAtURL:options:error:)]
        pub unsafe fn replaceItemAtURL_options_error(
            &self,
            url: &NSURL,
            options: NSFileVersionReplacingOptions,
        ) -> Result<Id<NSURL, Shared>, Id<NSError, Shared>>;
        #[method(removeAndReturnError:)]
        pub unsafe fn removeAndReturnError(&self) -> Result<(), Id<NSError, Shared>>;
        #[method(removeOtherVersionsOfItemAtURL:error:)]
        pub unsafe fn removeOtherVersionsOfItemAtURL_error(
            url: &NSURL,
        ) -> Result<(), Id<NSError, Shared>>;
    }
);
