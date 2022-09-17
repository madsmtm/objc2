use super::NSArray;
use super::NSData;
use super::NSDate;
use super::NSError;
use super::NSLock;
use super::NSNumber;
use super::NSXPCConnection;
use crate::dispatch::generated::dispatch::*;
use crate::CoreFoundation::generated::CFBase::*;
use crate::Foundation::generated::NSDictionary::*;
use crate::Foundation::generated::NSEnumerator::*;
use crate::Foundation::generated::NSError::*;
use crate::Foundation::generated::NSNotification::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSPathUtilities::*;
use crate::Foundation::generated::NSURL::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSFileManager;
    unsafe impl ClassType for NSFileManager {
        type Super = NSObject;
    }
);
impl NSFileManager {
    pub unsafe fn mountedVolumeURLsIncludingResourceValuesForKeys_options(
        &self,
        propertyKeys: TodoGenerics,
        options: NSVolumeEnumerationOptions,
    ) -> TodoGenerics {
        msg_send![
            self,
            mountedVolumeURLsIncludingResourceValuesForKeys: propertyKeys,
            options: options
        ]
    }
    pub unsafe fn unmountVolumeAtURL_options_completionHandler(
        &self,
        url: &NSURL,
        mask: NSFileManagerUnmountOptions,
        completionHandler: TodoBlock,
    ) {
        msg_send![
            self,
            unmountVolumeAtURL: url,
            options: mask,
            completionHandler: completionHandler
        ]
    }
    pub unsafe fn contentsOfDirectoryAtURL_includingPropertiesForKeys_options_error(
        &self,
        url: &NSURL,
        keys: TodoGenerics,
        mask: NSDirectoryEnumerationOptions,
        error: *mut Option<&NSError>,
    ) -> TodoGenerics {
        msg_send![
            self,
            contentsOfDirectoryAtURL: url,
            includingPropertiesForKeys: keys,
            options: mask,
            error: error
        ]
    }
    pub unsafe fn URLsForDirectory_inDomains(
        &self,
        directory: NSSearchPathDirectory,
        domainMask: NSSearchPathDomainMask,
    ) -> TodoGenerics {
        msg_send![self, URLsForDirectory: directory, inDomains: domainMask]
    }
    pub unsafe fn URLForDirectory_inDomain_appropriateForURL_create_error(
        &self,
        directory: NSSearchPathDirectory,
        domain: NSSearchPathDomainMask,
        url: Option<&NSURL>,
        shouldCreate: bool,
        error: *mut Option<&NSError>,
    ) -> Option<Id<NSURL, Shared>> {
        msg_send_id![
            self,
            URLForDirectory: directory,
            inDomain: domain,
            appropriateForURL: url,
            create: shouldCreate,
            error: error
        ]
    }
    pub unsafe fn getRelationship_ofDirectoryAtURL_toItemAtURL_error(
        &self,
        outRelationship: NonNull<NSURLRelationship>,
        directoryURL: &NSURL,
        otherURL: &NSURL,
        error: *mut Option<&NSError>,
    ) -> bool {
        msg_send![
            self,
            getRelationship: outRelationship,
            ofDirectoryAtURL: directoryURL,
            toItemAtURL: otherURL,
            error: error
        ]
    }
    pub unsafe fn getRelationship_ofDirectory_inDomain_toItemAtURL_error(
        &self,
        outRelationship: NonNull<NSURLRelationship>,
        directory: NSSearchPathDirectory,
        domainMask: NSSearchPathDomainMask,
        url: &NSURL,
        error: *mut Option<&NSError>,
    ) -> bool {
        msg_send![
            self,
            getRelationship: outRelationship,
            ofDirectory: directory,
            inDomain: domainMask,
            toItemAtURL: url,
            error: error
        ]
    }
    pub unsafe fn createDirectoryAtURL_withIntermediateDirectories_attributes_error(
        &self,
        url: &NSURL,
        createIntermediates: bool,
        attributes: TodoGenerics,
        error: *mut Option<&NSError>,
    ) -> bool {
        msg_send![
            self,
            createDirectoryAtURL: url,
            withIntermediateDirectories: createIntermediates,
            attributes: attributes,
            error: error
        ]
    }
    pub unsafe fn createSymbolicLinkAtURL_withDestinationURL_error(
        &self,
        url: &NSURL,
        destURL: &NSURL,
        error: *mut Option<&NSError>,
    ) -> bool {
        msg_send![
            self,
            createSymbolicLinkAtURL: url,
            withDestinationURL: destURL,
            error: error
        ]
    }
    pub unsafe fn setAttributes_ofItemAtPath_error(
        &self,
        attributes: TodoGenerics,
        path: &NSString,
        error: *mut Option<&NSError>,
    ) -> bool {
        msg_send![
            self,
            setAttributes: attributes,
            ofItemAtPath: path,
            error: error
        ]
    }
    pub unsafe fn createDirectoryAtPath_withIntermediateDirectories_attributes_error(
        &self,
        path: &NSString,
        createIntermediates: bool,
        attributes: TodoGenerics,
        error: *mut Option<&NSError>,
    ) -> bool {
        msg_send![
            self,
            createDirectoryAtPath: path,
            withIntermediateDirectories: createIntermediates,
            attributes: attributes,
            error: error
        ]
    }
    pub unsafe fn contentsOfDirectoryAtPath_error(
        &self,
        path: &NSString,
        error: *mut Option<&NSError>,
    ) -> TodoGenerics {
        msg_send![self, contentsOfDirectoryAtPath: path, error: error]
    }
    pub unsafe fn subpathsOfDirectoryAtPath_error(
        &self,
        path: &NSString,
        error: *mut Option<&NSError>,
    ) -> TodoGenerics {
        msg_send![self, subpathsOfDirectoryAtPath: path, error: error]
    }
    pub unsafe fn attributesOfItemAtPath_error(
        &self,
        path: &NSString,
        error: *mut Option<&NSError>,
    ) -> TodoGenerics {
        msg_send![self, attributesOfItemAtPath: path, error: error]
    }
    pub unsafe fn attributesOfFileSystemForPath_error(
        &self,
        path: &NSString,
        error: *mut Option<&NSError>,
    ) -> TodoGenerics {
        msg_send![self, attributesOfFileSystemForPath: path, error: error]
    }
    pub unsafe fn createSymbolicLinkAtPath_withDestinationPath_error(
        &self,
        path: &NSString,
        destPath: &NSString,
        error: *mut Option<&NSError>,
    ) -> bool {
        msg_send![
            self,
            createSymbolicLinkAtPath: path,
            withDestinationPath: destPath,
            error: error
        ]
    }
    pub unsafe fn destinationOfSymbolicLinkAtPath_error(
        &self,
        path: &NSString,
        error: *mut Option<&NSError>,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, destinationOfSymbolicLinkAtPath: path, error: error]
    }
    pub unsafe fn copyItemAtPath_toPath_error(
        &self,
        srcPath: &NSString,
        dstPath: &NSString,
        error: *mut Option<&NSError>,
    ) -> bool {
        msg_send![self, copyItemAtPath: srcPath, toPath: dstPath, error: error]
    }
    pub unsafe fn moveItemAtPath_toPath_error(
        &self,
        srcPath: &NSString,
        dstPath: &NSString,
        error: *mut Option<&NSError>,
    ) -> bool {
        msg_send![self, moveItemAtPath: srcPath, toPath: dstPath, error: error]
    }
    pub unsafe fn linkItemAtPath_toPath_error(
        &self,
        srcPath: &NSString,
        dstPath: &NSString,
        error: *mut Option<&NSError>,
    ) -> bool {
        msg_send![self, linkItemAtPath: srcPath, toPath: dstPath, error: error]
    }
    pub unsafe fn removeItemAtPath_error(
        &self,
        path: &NSString,
        error: *mut Option<&NSError>,
    ) -> bool {
        msg_send![self, removeItemAtPath: path, error: error]
    }
    pub unsafe fn copyItemAtURL_toURL_error(
        &self,
        srcURL: &NSURL,
        dstURL: &NSURL,
        error: *mut Option<&NSError>,
    ) -> bool {
        msg_send![self, copyItemAtURL: srcURL, toURL: dstURL, error: error]
    }
    pub unsafe fn moveItemAtURL_toURL_error(
        &self,
        srcURL: &NSURL,
        dstURL: &NSURL,
        error: *mut Option<&NSError>,
    ) -> bool {
        msg_send![self, moveItemAtURL: srcURL, toURL: dstURL, error: error]
    }
    pub unsafe fn linkItemAtURL_toURL_error(
        &self,
        srcURL: &NSURL,
        dstURL: &NSURL,
        error: *mut Option<&NSError>,
    ) -> bool {
        msg_send![self, linkItemAtURL: srcURL, toURL: dstURL, error: error]
    }
    pub unsafe fn removeItemAtURL_error(&self, URL: &NSURL, error: *mut Option<&NSError>) -> bool {
        msg_send![self, removeItemAtURL: URL, error: error]
    }
    pub unsafe fn trashItemAtURL_resultingItemURL_error(
        &self,
        url: &NSURL,
        outResultingURL: *mut Option<&NSURL>,
        error: *mut Option<&NSError>,
    ) -> bool {
        msg_send![
            self,
            trashItemAtURL: url,
            resultingItemURL: outResultingURL,
            error: error
        ]
    }
    pub unsafe fn fileAttributesAtPath_traverseLink(
        &self,
        path: &NSString,
        yorn: bool,
    ) -> Option<Id<NSDictionary, Shared>> {
        msg_send_id![self, fileAttributesAtPath: path, traverseLink: yorn]
    }
    pub unsafe fn changeFileAttributes_atPath(
        &self,
        attributes: &NSDictionary,
        path: &NSString,
    ) -> bool {
        msg_send![self, changeFileAttributes: attributes, atPath: path]
    }
    pub unsafe fn directoryContentsAtPath(&self, path: &NSString) -> Option<Id<NSArray, Shared>> {
        msg_send_id![self, directoryContentsAtPath: path]
    }
    pub unsafe fn fileSystemAttributesAtPath(
        &self,
        path: &NSString,
    ) -> Option<Id<NSDictionary, Shared>> {
        msg_send_id![self, fileSystemAttributesAtPath: path]
    }
    pub unsafe fn pathContentOfSymbolicLinkAtPath(
        &self,
        path: &NSString,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, pathContentOfSymbolicLinkAtPath: path]
    }
    pub unsafe fn createSymbolicLinkAtPath_pathContent(
        &self,
        path: &NSString,
        otherpath: &NSString,
    ) -> bool {
        msg_send![self, createSymbolicLinkAtPath: path, pathContent: otherpath]
    }
    pub unsafe fn createDirectoryAtPath_attributes(
        &self,
        path: &NSString,
        attributes: &NSDictionary,
    ) -> bool {
        msg_send![self, createDirectoryAtPath: path, attributes: attributes]
    }
    pub unsafe fn linkPath_toPath_handler(
        &self,
        src: &NSString,
        dest: &NSString,
        handler: Option<&Object>,
    ) -> bool {
        msg_send![self, linkPath: src, toPath: dest, handler: handler]
    }
    pub unsafe fn copyPath_toPath_handler(
        &self,
        src: &NSString,
        dest: &NSString,
        handler: Option<&Object>,
    ) -> bool {
        msg_send![self, copyPath: src, toPath: dest, handler: handler]
    }
    pub unsafe fn movePath_toPath_handler(
        &self,
        src: &NSString,
        dest: &NSString,
        handler: Option<&Object>,
    ) -> bool {
        msg_send![self, movePath: src, toPath: dest, handler: handler]
    }
    pub unsafe fn removeFileAtPath_handler(
        &self,
        path: &NSString,
        handler: Option<&Object>,
    ) -> bool {
        msg_send![self, removeFileAtPath: path, handler: handler]
    }
    pub unsafe fn changeCurrentDirectoryPath(&self, path: &NSString) -> bool {
        msg_send![self, changeCurrentDirectoryPath: path]
    }
    pub unsafe fn fileExistsAtPath(&self, path: &NSString) -> bool {
        msg_send![self, fileExistsAtPath: path]
    }
    pub unsafe fn fileExistsAtPath_isDirectory(
        &self,
        path: &NSString,
        isDirectory: *mut bool,
    ) -> bool {
        msg_send![self, fileExistsAtPath: path, isDirectory: isDirectory]
    }
    pub unsafe fn isReadableFileAtPath(&self, path: &NSString) -> bool {
        msg_send![self, isReadableFileAtPath: path]
    }
    pub unsafe fn isWritableFileAtPath(&self, path: &NSString) -> bool {
        msg_send![self, isWritableFileAtPath: path]
    }
    pub unsafe fn isExecutableFileAtPath(&self, path: &NSString) -> bool {
        msg_send![self, isExecutableFileAtPath: path]
    }
    pub unsafe fn isDeletableFileAtPath(&self, path: &NSString) -> bool {
        msg_send![self, isDeletableFileAtPath: path]
    }
    pub unsafe fn contentsEqualAtPath_andPath(&self, path1: &NSString, path2: &NSString) -> bool {
        msg_send![self, contentsEqualAtPath: path1, andPath: path2]
    }
    pub unsafe fn displayNameAtPath(&self, path: &NSString) -> Id<NSString, Shared> {
        msg_send_id![self, displayNameAtPath: path]
    }
    pub unsafe fn componentsToDisplayForPath(&self, path: &NSString) -> TodoGenerics {
        msg_send![self, componentsToDisplayForPath: path]
    }
    pub unsafe fn enumeratorAtPath(&self, path: &NSString) -> TodoGenerics {
        msg_send![self, enumeratorAtPath: path]
    }
    pub unsafe fn enumeratorAtURL_includingPropertiesForKeys_options_errorHandler(
        &self,
        url: &NSURL,
        keys: TodoGenerics,
        mask: NSDirectoryEnumerationOptions,
        handler: TodoBlock,
    ) -> TodoGenerics {
        msg_send![
            self,
            enumeratorAtURL: url,
            includingPropertiesForKeys: keys,
            options: mask,
            errorHandler: handler
        ]
    }
    pub unsafe fn subpathsAtPath(&self, path: &NSString) -> TodoGenerics {
        msg_send![self, subpathsAtPath: path]
    }
    pub unsafe fn contentsAtPath(&self, path: &NSString) -> Option<Id<NSData, Shared>> {
        msg_send_id![self, contentsAtPath: path]
    }
    pub unsafe fn createFileAtPath_contents_attributes(
        &self,
        path: &NSString,
        data: Option<&NSData>,
        attr: TodoGenerics,
    ) -> bool {
        msg_send![
            self,
            createFileAtPath: path,
            contents: data,
            attributes: attr
        ]
    }
    pub unsafe fn fileSystemRepresentationWithPath(&self, path: &NSString) -> NonNull<c_char> {
        msg_send![self, fileSystemRepresentationWithPath: path]
    }
    pub unsafe fn stringWithFileSystemRepresentation_length(
        &self,
        str: NonNull<c_char>,
        len: NSUInteger,
    ) -> Id<NSString, Shared> {
        msg_send_id![self, stringWithFileSystemRepresentation: str, length: len]
    }
    pub unsafe fn replaceItemAtURL_withItemAtURL_backupItemName_options_resultingItemURL_error(
        &self,
        originalItemURL: &NSURL,
        newItemURL: &NSURL,
        backupItemName: Option<&NSString>,
        options: NSFileManagerItemReplacementOptions,
        resultingURL: *mut Option<&NSURL>,
        error: *mut Option<&NSError>,
    ) -> bool {
        msg_send![
            self,
            replaceItemAtURL: originalItemURL,
            withItemAtURL: newItemURL,
            backupItemName: backupItemName,
            options: options,
            resultingItemURL: resultingURL,
            error: error
        ]
    }
    pub unsafe fn setUbiquitous_itemAtURL_destinationURL_error(
        &self,
        flag: bool,
        url: &NSURL,
        destinationURL: &NSURL,
        error: *mut Option<&NSError>,
    ) -> bool {
        msg_send![
            self,
            setUbiquitous: flag,
            itemAtURL: url,
            destinationURL: destinationURL,
            error: error
        ]
    }
    pub unsafe fn isUbiquitousItemAtURL(&self, url: &NSURL) -> bool {
        msg_send![self, isUbiquitousItemAtURL: url]
    }
    pub unsafe fn startDownloadingUbiquitousItemAtURL_error(
        &self,
        url: &NSURL,
        error: *mut Option<&NSError>,
    ) -> bool {
        msg_send![self, startDownloadingUbiquitousItemAtURL: url, error: error]
    }
    pub unsafe fn evictUbiquitousItemAtURL_error(
        &self,
        url: &NSURL,
        error: *mut Option<&NSError>,
    ) -> bool {
        msg_send![self, evictUbiquitousItemAtURL: url, error: error]
    }
    pub unsafe fn URLForUbiquityContainerIdentifier(
        &self,
        containerIdentifier: Option<&NSString>,
    ) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, URLForUbiquityContainerIdentifier: containerIdentifier]
    }
    pub unsafe fn URLForPublishingUbiquitousItemAtURL_expirationDate_error(
        &self,
        url: &NSURL,
        outDate: *mut Option<&NSDate>,
        error: *mut Option<&NSError>,
    ) -> Option<Id<NSURL, Shared>> {
        msg_send_id![
            self,
            URLForPublishingUbiquitousItemAtURL: url,
            expirationDate: outDate,
            error: error
        ]
    }
    pub unsafe fn getFileProviderServicesForItemAtURL_completionHandler(
        &self,
        url: &NSURL,
        completionHandler: TodoBlock,
    ) {
        msg_send![
            self,
            getFileProviderServicesForItemAtURL: url,
            completionHandler: completionHandler
        ]
    }
    pub unsafe fn containerURLForSecurityApplicationGroupIdentifier(
        &self,
        groupIdentifier: &NSString,
    ) -> Option<Id<NSURL, Shared>> {
        msg_send_id![
            self,
            containerURLForSecurityApplicationGroupIdentifier: groupIdentifier
        ]
    }
    pub unsafe fn defaultManager() -> Id<NSFileManager, Shared> {
        msg_send_id![Self::class(), defaultManager]
    }
    pub unsafe fn delegate(&self) -> TodoGenerics {
        msg_send![self, delegate]
    }
    pub unsafe fn setDelegate(&self, delegate: TodoGenerics) {
        msg_send![self, setDelegate: delegate]
    }
    pub unsafe fn currentDirectoryPath(&self) -> Id<NSString, Shared> {
        msg_send_id![self, currentDirectoryPath]
    }
    pub unsafe fn ubiquityIdentityToken(&self) -> TodoGenerics {
        msg_send![self, ubiquityIdentityToken]
    }
}
#[doc = "NSUserInformation"]
impl NSFileManager {
    pub unsafe fn homeDirectoryForUser(&self, userName: &NSString) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, homeDirectoryForUser: userName]
    }
    pub unsafe fn homeDirectoryForCurrentUser(&self) -> Id<NSURL, Shared> {
        msg_send_id![self, homeDirectoryForCurrentUser]
    }
    pub unsafe fn temporaryDirectory(&self) -> Id<NSURL, Shared> {
        msg_send_id![self, temporaryDirectory]
    }
}
#[doc = "NSCopyLinkMoveHandler"]
impl NSObject {
    pub unsafe fn fileManager_shouldProceedAfterError(
        &self,
        fm: &NSFileManager,
        errorInfo: &NSDictionary,
    ) -> bool {
        msg_send![self, fileManager: fm, shouldProceedAfterError: errorInfo]
    }
    pub unsafe fn fileManager_willProcessPath(&self, fm: &NSFileManager, path: &NSString) {
        msg_send![self, fileManager: fm, willProcessPath: path]
    }
}
pub type NSFileManagerDelegate = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSDirectoryEnumerator;
    unsafe impl ClassType for NSDirectoryEnumerator {
        type Super = NSEnumerator;
    }
);
impl NSDirectoryEnumerator {
    pub unsafe fn skipDescendents(&self) {
        msg_send![self, skipDescendents]
    }
    pub unsafe fn skipDescendants(&self) {
        msg_send![self, skipDescendants]
    }
    pub unsafe fn fileAttributes(&self) -> TodoGenerics {
        msg_send![self, fileAttributes]
    }
    pub unsafe fn directoryAttributes(&self) -> TodoGenerics {
        msg_send![self, directoryAttributes]
    }
    pub unsafe fn isEnumeratingDirectoryPostOrder(&self) -> bool {
        msg_send![self, isEnumeratingDirectoryPostOrder]
    }
    pub unsafe fn level(&self) -> NSUInteger {
        msg_send![self, level]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSFileProviderService;
    unsafe impl ClassType for NSFileProviderService {
        type Super = NSObject;
    }
);
impl NSFileProviderService {
    pub unsafe fn getFileProviderConnectionWithCompletionHandler(
        &self,
        completionHandler: TodoBlock,
    ) {
        msg_send![
            self,
            getFileProviderConnectionWithCompletionHandler: completionHandler
        ]
    }
    pub unsafe fn name(&self) -> NSFileProviderServiceName {
        msg_send![self, name]
    }
}
#[doc = "NSFileAttributes"]
impl NSDictionary {
    pub unsafe fn fileSize(&self) -> c_ulonglong {
        msg_send![self, fileSize]
    }
    pub unsafe fn fileModificationDate(&self) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, fileModificationDate]
    }
    pub unsafe fn fileType(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, fileType]
    }
    pub unsafe fn filePosixPermissions(&self) -> NSUInteger {
        msg_send![self, filePosixPermissions]
    }
    pub unsafe fn fileOwnerAccountName(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, fileOwnerAccountName]
    }
    pub unsafe fn fileGroupOwnerAccountName(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, fileGroupOwnerAccountName]
    }
    pub unsafe fn fileSystemNumber(&self) -> NSInteger {
        msg_send![self, fileSystemNumber]
    }
    pub unsafe fn fileSystemFileNumber(&self) -> NSUInteger {
        msg_send![self, fileSystemFileNumber]
    }
    pub unsafe fn fileExtensionHidden(&self) -> bool {
        msg_send![self, fileExtensionHidden]
    }
    pub unsafe fn fileHFSCreatorCode(&self) -> OSType {
        msg_send![self, fileHFSCreatorCode]
    }
    pub unsafe fn fileHFSTypeCode(&self) -> OSType {
        msg_send![self, fileHFSTypeCode]
    }
    pub unsafe fn fileIsImmutable(&self) -> bool {
        msg_send![self, fileIsImmutable]
    }
    pub unsafe fn fileIsAppendOnly(&self) -> bool {
        msg_send![self, fileIsAppendOnly]
    }
    pub unsafe fn fileCreationDate(&self) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, fileCreationDate]
    }
    pub unsafe fn fileOwnerAccountID(&self) -> Option<Id<NSNumber, Shared>> {
        msg_send_id![self, fileOwnerAccountID]
    }
    pub unsafe fn fileGroupOwnerAccountID(&self) -> Option<Id<NSNumber, Shared>> {
        msg_send_id![self, fileGroupOwnerAccountID]
    }
}
