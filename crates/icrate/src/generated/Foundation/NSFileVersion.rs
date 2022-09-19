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
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSFileVersion;
    unsafe impl ClassType for NSFileVersion {
        type Super = NSObject;
    }
);
impl NSFileVersion {
    pub unsafe fn currentVersionOfItemAtURL(url: &NSURL) -> Option<Id<NSFileVersion, Shared>> {
        msg_send_id![Self::class(), currentVersionOfItemAtURL: url]
    }
    pub unsafe fn otherVersionsOfItemAtURL(
        url: &NSURL,
    ) -> Option<Id<NSArray<NSFileVersion>, Shared>> {
        msg_send_id![Self::class(), otherVersionsOfItemAtURL: url]
    }
    pub unsafe fn unresolvedConflictVersionsOfItemAtURL(
        url: &NSURL,
    ) -> Option<Id<NSArray<NSFileVersion>, Shared>> {
        msg_send_id![Self::class(), unresolvedConflictVersionsOfItemAtURL: url]
    }
    pub unsafe fn getNonlocalVersionsOfItemAtURL_completionHandler(
        url: &NSURL,
        completionHandler: TodoBlock,
    ) {
        msg_send![
            Self::class(),
            getNonlocalVersionsOfItemAtURL: url,
            completionHandler: completionHandler
        ]
    }
    pub unsafe fn versionOfItemAtURL_forPersistentIdentifier(
        url: &NSURL,
        persistentIdentifier: &Object,
    ) -> Option<Id<NSFileVersion, Shared>> {
        msg_send_id![
            Self::class(),
            versionOfItemAtURL: url,
            forPersistentIdentifier: persistentIdentifier
        ]
    }
    pub unsafe fn addVersionOfItemAtURL_withContentsOfURL_options_error(
        url: &NSURL,
        contentsURL: &NSURL,
        options: NSFileVersionAddingOptions,
        outError: *mut *mut NSError,
    ) -> Option<Id<NSFileVersion, Shared>> {
        msg_send_id![
            Self::class(),
            addVersionOfItemAtURL: url,
            withContentsOfURL: contentsURL,
            options: options,
            error: outError
        ]
    }
    pub unsafe fn temporaryDirectoryURLForNewVersionOfItemAtURL(url: &NSURL) -> Id<NSURL, Shared> {
        msg_send_id![
            Self::class(),
            temporaryDirectoryURLForNewVersionOfItemAtURL: url
        ]
    }
    pub unsafe fn replaceItemAtURL_options_error(
        &self,
        url: &NSURL,
        options: NSFileVersionReplacingOptions,
        error: *mut *mut NSError,
    ) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, replaceItemAtURL: url, options: options, error: error]
    }
    pub unsafe fn removeAndReturnError(&self, outError: *mut *mut NSError) -> bool {
        msg_send![self, removeAndReturnError: outError]
    }
    pub unsafe fn removeOtherVersionsOfItemAtURL_error(
        url: &NSURL,
        outError: *mut *mut NSError,
    ) -> bool {
        msg_send![
            Self::class(),
            removeOtherVersionsOfItemAtURL: url,
            error: outError
        ]
    }
    pub unsafe fn URL(&self) -> Id<NSURL, Shared> {
        msg_send_id![self, URL]
    }
    pub unsafe fn localizedName(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, localizedName]
    }
    pub unsafe fn localizedNameOfSavingComputer(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, localizedNameOfSavingComputer]
    }
    pub unsafe fn originatorNameComponents(&self) -> Option<Id<NSPersonNameComponents, Shared>> {
        msg_send_id![self, originatorNameComponents]
    }
    pub unsafe fn modificationDate(&self) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, modificationDate]
    }
    pub unsafe fn persistentIdentifier(&self) -> Id<id, Shared> {
        msg_send_id![self, persistentIdentifier]
    }
    pub unsafe fn isConflict(&self) -> bool {
        msg_send![self, isConflict]
    }
    pub unsafe fn isResolved(&self) -> bool {
        msg_send![self, isResolved]
    }
    pub unsafe fn setResolved(&self, resolved: bool) {
        msg_send![self, setResolved: resolved]
    }
    pub unsafe fn isDiscardable(&self) -> bool {
        msg_send![self, isDiscardable]
    }
    pub unsafe fn setDiscardable(&self, discardable: bool) {
        msg_send![self, setDiscardable: discardable]
    }
    pub unsafe fn hasLocalContents(&self) -> bool {
        msg_send![self, hasLocalContents]
    }
    pub unsafe fn hasThumbnail(&self) -> bool {
        msg_send![self, hasThumbnail]
    }
}
