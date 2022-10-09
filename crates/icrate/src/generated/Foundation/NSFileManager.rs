use super::__exported::NSArray;
use super::__exported::NSData;
use super::__exported::NSDate;
use super::__exported::NSError;
use super::__exported::NSLock;
use super::__exported::NSNumber;
use super::__exported::NSXPCConnection;
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
use objc2::{extern_class, extern_methods, ClassType};
pub type NSFileAttributeKey = NSString;
pub type NSFileAttributeType = NSString;
pub type NSFileProtectionType = NSString;
pub type NSFileProviderServiceName = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSFileManager;
    unsafe impl ClassType for NSFileManager {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSFileManager {
        #[method_id(defaultManager)]
        pub unsafe fn defaultManager() -> Id<NSFileManager, Shared>;
        #[method_id(mountedVolumeURLsIncludingResourceValuesForKeys:options:)]
        pub unsafe fn mountedVolumeURLsIncludingResourceValuesForKeys_options(
            &self,
            propertyKeys: Option<&NSArray<NSURLResourceKey>>,
            options: NSVolumeEnumerationOptions,
        ) -> Option<Id<NSArray<NSURL>, Shared>>;
        #[method(unmountVolumeAtURL:options:completionHandler:)]
        pub unsafe fn unmountVolumeAtURL_options_completionHandler(
            &self,
            url: &NSURL,
            mask: NSFileManagerUnmountOptions,
            completionHandler: TodoBlock,
        );
        #[method_id(contentsOfDirectoryAtURL:includingPropertiesForKeys:options:error:)]
        pub unsafe fn contentsOfDirectoryAtURL_includingPropertiesForKeys_options_error(
            &self,
            url: &NSURL,
            keys: Option<&NSArray<NSURLResourceKey>>,
            mask: NSDirectoryEnumerationOptions,
        ) -> Result<Id<NSArray<NSURL>, Shared>, Id<NSError, Shared>>;
        #[method_id(URLsForDirectory:inDomains:)]
        pub unsafe fn URLsForDirectory_inDomains(
            &self,
            directory: NSSearchPathDirectory,
            domainMask: NSSearchPathDomainMask,
        ) -> Id<NSArray<NSURL>, Shared>;
        #[method_id(URLForDirectory:inDomain:appropriateForURL:create:error:)]
        pub unsafe fn URLForDirectory_inDomain_appropriateForURL_create_error(
            &self,
            directory: NSSearchPathDirectory,
            domain: NSSearchPathDomainMask,
            url: Option<&NSURL>,
            shouldCreate: bool,
        ) -> Result<Id<NSURL, Shared>, Id<NSError, Shared>>;
        #[method(getRelationship:ofDirectoryAtURL:toItemAtURL:error:)]
        pub unsafe fn getRelationship_ofDirectoryAtURL_toItemAtURL_error(
            &self,
            outRelationship: NonNull<NSURLRelationship>,
            directoryURL: &NSURL,
            otherURL: &NSURL,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method(getRelationship:ofDirectory:inDomain:toItemAtURL:error:)]
        pub unsafe fn getRelationship_ofDirectory_inDomain_toItemAtURL_error(
            &self,
            outRelationship: NonNull<NSURLRelationship>,
            directory: NSSearchPathDirectory,
            domainMask: NSSearchPathDomainMask,
            url: &NSURL,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method(createDirectoryAtURL:withIntermediateDirectories:attributes:error:)]
        pub unsafe fn createDirectoryAtURL_withIntermediateDirectories_attributes_error(
            &self,
            url: &NSURL,
            createIntermediates: bool,
            attributes: Option<&NSDictionary<NSFileAttributeKey, Object>>,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method(createSymbolicLinkAtURL:withDestinationURL:error:)]
        pub unsafe fn createSymbolicLinkAtURL_withDestinationURL_error(
            &self,
            url: &NSURL,
            destURL: &NSURL,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSFileManagerDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSFileManagerDelegate>);
        #[method(setAttributes:ofItemAtPath:error:)]
        pub unsafe fn setAttributes_ofItemAtPath_error(
            &self,
            attributes: &NSDictionary<NSFileAttributeKey, Object>,
            path: &NSString,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method(createDirectoryAtPath:withIntermediateDirectories:attributes:error:)]
        pub unsafe fn createDirectoryAtPath_withIntermediateDirectories_attributes_error(
            &self,
            path: &NSString,
            createIntermediates: bool,
            attributes: Option<&NSDictionary<NSFileAttributeKey, Object>>,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method_id(contentsOfDirectoryAtPath:error:)]
        pub unsafe fn contentsOfDirectoryAtPath_error(
            &self,
            path: &NSString,
        ) -> Result<Id<NSArray<NSString>, Shared>, Id<NSError, Shared>>;
        #[method_id(subpathsOfDirectoryAtPath:error:)]
        pub unsafe fn subpathsOfDirectoryAtPath_error(
            &self,
            path: &NSString,
        ) -> Result<Id<NSArray<NSString>, Shared>, Id<NSError, Shared>>;
        #[method_id(attributesOfItemAtPath:error:)]
        pub unsafe fn attributesOfItemAtPath_error(
            &self,
            path: &NSString,
        ) -> Result<Id<NSDictionary<NSFileAttributeKey, Object>, Shared>, Id<NSError, Shared>>;
        #[method_id(attributesOfFileSystemForPath:error:)]
        pub unsafe fn attributesOfFileSystemForPath_error(
            &self,
            path: &NSString,
        ) -> Result<Id<NSDictionary<NSFileAttributeKey, Object>, Shared>, Id<NSError, Shared>>;
        #[method(createSymbolicLinkAtPath:withDestinationPath:error:)]
        pub unsafe fn createSymbolicLinkAtPath_withDestinationPath_error(
            &self,
            path: &NSString,
            destPath: &NSString,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method_id(destinationOfSymbolicLinkAtPath:error:)]
        pub unsafe fn destinationOfSymbolicLinkAtPath_error(
            &self,
            path: &NSString,
        ) -> Result<Id<NSString, Shared>, Id<NSError, Shared>>;
        #[method(copyItemAtPath:toPath:error:)]
        pub unsafe fn copyItemAtPath_toPath_error(
            &self,
            srcPath: &NSString,
            dstPath: &NSString,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method(moveItemAtPath:toPath:error:)]
        pub unsafe fn moveItemAtPath_toPath_error(
            &self,
            srcPath: &NSString,
            dstPath: &NSString,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method(linkItemAtPath:toPath:error:)]
        pub unsafe fn linkItemAtPath_toPath_error(
            &self,
            srcPath: &NSString,
            dstPath: &NSString,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method(removeItemAtPath:error:)]
        pub unsafe fn removeItemAtPath_error(
            &self,
            path: &NSString,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method(copyItemAtURL:toURL:error:)]
        pub unsafe fn copyItemAtURL_toURL_error(
            &self,
            srcURL: &NSURL,
            dstURL: &NSURL,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method(moveItemAtURL:toURL:error:)]
        pub unsafe fn moveItemAtURL_toURL_error(
            &self,
            srcURL: &NSURL,
            dstURL: &NSURL,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method(linkItemAtURL:toURL:error:)]
        pub unsafe fn linkItemAtURL_toURL_error(
            &self,
            srcURL: &NSURL,
            dstURL: &NSURL,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method(removeItemAtURL:error:)]
        pub unsafe fn removeItemAtURL_error(&self, URL: &NSURL) -> Result<(), Id<NSError, Shared>>;
        #[method(trashItemAtURL:resultingItemURL:error:)]
        pub unsafe fn trashItemAtURL_resultingItemURL_error(
            &self,
            url: &NSURL,
            outResultingURL: Option<&mut Option<Id<NSURL, Shared>>>,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method_id(fileAttributesAtPath:traverseLink:)]
        pub unsafe fn fileAttributesAtPath_traverseLink(
            &self,
            path: &NSString,
            yorn: bool,
        ) -> Option<Id<NSDictionary, Shared>>;
        #[method(changeFileAttributes:atPath:)]
        pub unsafe fn changeFileAttributes_atPath(
            &self,
            attributes: &NSDictionary,
            path: &NSString,
        ) -> bool;
        #[method_id(directoryContentsAtPath:)]
        pub unsafe fn directoryContentsAtPath(
            &self,
            path: &NSString,
        ) -> Option<Id<NSArray, Shared>>;
        #[method_id(fileSystemAttributesAtPath:)]
        pub unsafe fn fileSystemAttributesAtPath(
            &self,
            path: &NSString,
        ) -> Option<Id<NSDictionary, Shared>>;
        #[method_id(pathContentOfSymbolicLinkAtPath:)]
        pub unsafe fn pathContentOfSymbolicLinkAtPath(
            &self,
            path: &NSString,
        ) -> Option<Id<NSString, Shared>>;
        #[method(createSymbolicLinkAtPath:pathContent:)]
        pub unsafe fn createSymbolicLinkAtPath_pathContent(
            &self,
            path: &NSString,
            otherpath: &NSString,
        ) -> bool;
        #[method(createDirectoryAtPath:attributes:)]
        pub unsafe fn createDirectoryAtPath_attributes(
            &self,
            path: &NSString,
            attributes: &NSDictionary,
        ) -> bool;
        #[method(linkPath:toPath:handler:)]
        pub unsafe fn linkPath_toPath_handler(
            &self,
            src: &NSString,
            dest: &NSString,
            handler: Option<&Object>,
        ) -> bool;
        #[method(copyPath:toPath:handler:)]
        pub unsafe fn copyPath_toPath_handler(
            &self,
            src: &NSString,
            dest: &NSString,
            handler: Option<&Object>,
        ) -> bool;
        #[method(movePath:toPath:handler:)]
        pub unsafe fn movePath_toPath_handler(
            &self,
            src: &NSString,
            dest: &NSString,
            handler: Option<&Object>,
        ) -> bool;
        #[method(removeFileAtPath:handler:)]
        pub unsafe fn removeFileAtPath_handler(
            &self,
            path: &NSString,
            handler: Option<&Object>,
        ) -> bool;
        #[method_id(currentDirectoryPath)]
        pub unsafe fn currentDirectoryPath(&self) -> Id<NSString, Shared>;
        #[method(changeCurrentDirectoryPath:)]
        pub unsafe fn changeCurrentDirectoryPath(&self, path: &NSString) -> bool;
        #[method(fileExistsAtPath:)]
        pub unsafe fn fileExistsAtPath(&self, path: &NSString) -> bool;
        #[method(fileExistsAtPath:isDirectory:)]
        pub unsafe fn fileExistsAtPath_isDirectory(
            &self,
            path: &NSString,
            isDirectory: *mut bool,
        ) -> bool;
        #[method(isReadableFileAtPath:)]
        pub unsafe fn isReadableFileAtPath(&self, path: &NSString) -> bool;
        #[method(isWritableFileAtPath:)]
        pub unsafe fn isWritableFileAtPath(&self, path: &NSString) -> bool;
        #[method(isExecutableFileAtPath:)]
        pub unsafe fn isExecutableFileAtPath(&self, path: &NSString) -> bool;
        #[method(isDeletableFileAtPath:)]
        pub unsafe fn isDeletableFileAtPath(&self, path: &NSString) -> bool;
        #[method(contentsEqualAtPath:andPath:)]
        pub unsafe fn contentsEqualAtPath_andPath(
            &self,
            path1: &NSString,
            path2: &NSString,
        ) -> bool;
        #[method_id(displayNameAtPath:)]
        pub unsafe fn displayNameAtPath(&self, path: &NSString) -> Id<NSString, Shared>;
        #[method_id(componentsToDisplayForPath:)]
        pub unsafe fn componentsToDisplayForPath(
            &self,
            path: &NSString,
        ) -> Option<Id<NSArray<NSString>, Shared>>;
        #[method_id(enumeratorAtPath:)]
        pub unsafe fn enumeratorAtPath(
            &self,
            path: &NSString,
        ) -> Option<Id<NSDirectoryEnumerator<NSString>, Shared>>;
        #[method_id(enumeratorAtURL:includingPropertiesForKeys:options:errorHandler:)]
        pub unsafe fn enumeratorAtURL_includingPropertiesForKeys_options_errorHandler(
            &self,
            url: &NSURL,
            keys: Option<&NSArray<NSURLResourceKey>>,
            mask: NSDirectoryEnumerationOptions,
            handler: TodoBlock,
        ) -> Option<Id<NSDirectoryEnumerator<NSURL>, Shared>>;
        #[method_id(subpathsAtPath:)]
        pub unsafe fn subpathsAtPath(
            &self,
            path: &NSString,
        ) -> Option<Id<NSArray<NSString>, Shared>>;
        #[method_id(contentsAtPath:)]
        pub unsafe fn contentsAtPath(&self, path: &NSString) -> Option<Id<NSData, Shared>>;
        #[method(createFileAtPath:contents:attributes:)]
        pub unsafe fn createFileAtPath_contents_attributes(
            &self,
            path: &NSString,
            data: Option<&NSData>,
            attr: Option<&NSDictionary<NSFileAttributeKey, Object>>,
        ) -> bool;
        #[method(fileSystemRepresentationWithPath:)]
        pub unsafe fn fileSystemRepresentationWithPath(&self, path: &NSString) -> NonNull<c_char>;
        #[method_id(stringWithFileSystemRepresentation:length:)]
        pub unsafe fn stringWithFileSystemRepresentation_length(
            &self,
            str: NonNull<c_char>,
            len: NSUInteger,
        ) -> Id<NSString, Shared>;
        #[method(replaceItemAtURL:withItemAtURL:backupItemName:options:resultingItemURL:error:)]
        pub unsafe fn replaceItemAtURL_withItemAtURL_backupItemName_options_resultingItemURL_error(
            &self,
            originalItemURL: &NSURL,
            newItemURL: &NSURL,
            backupItemName: Option<&NSString>,
            options: NSFileManagerItemReplacementOptions,
            resultingURL: Option<&mut Option<Id<NSURL, Shared>>>,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method(setUbiquitous:itemAtURL:destinationURL:error:)]
        pub unsafe fn setUbiquitous_itemAtURL_destinationURL_error(
            &self,
            flag: bool,
            url: &NSURL,
            destinationURL: &NSURL,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method(isUbiquitousItemAtURL:)]
        pub unsafe fn isUbiquitousItemAtURL(&self, url: &NSURL) -> bool;
        #[method(startDownloadingUbiquitousItemAtURL:error:)]
        pub unsafe fn startDownloadingUbiquitousItemAtURL_error(
            &self,
            url: &NSURL,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method(evictUbiquitousItemAtURL:error:)]
        pub unsafe fn evictUbiquitousItemAtURL_error(
            &self,
            url: &NSURL,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method_id(URLForUbiquityContainerIdentifier:)]
        pub unsafe fn URLForUbiquityContainerIdentifier(
            &self,
            containerIdentifier: Option<&NSString>,
        ) -> Option<Id<NSURL, Shared>>;
        #[method_id(URLForPublishingUbiquitousItemAtURL:expirationDate:error:)]
        pub unsafe fn URLForPublishingUbiquitousItemAtURL_expirationDate_error(
            &self,
            url: &NSURL,
            outDate: Option<&mut Option<Id<NSDate, Shared>>>,
        ) -> Result<Id<NSURL, Shared>, Id<NSError, Shared>>;
        #[method_id(ubiquityIdentityToken)]
        pub unsafe fn ubiquityIdentityToken(&self) -> Option<Id<TodoProtocols, Shared>>;
        #[method(getFileProviderServicesForItemAtURL:completionHandler:)]
        pub unsafe fn getFileProviderServicesForItemAtURL_completionHandler(
            &self,
            url: &NSURL,
            completionHandler: TodoBlock,
        );
        #[method_id(containerURLForSecurityApplicationGroupIdentifier:)]
        pub unsafe fn containerURLForSecurityApplicationGroupIdentifier(
            &self,
            groupIdentifier: &NSString,
        ) -> Option<Id<NSURL, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSUserInformation"]
    unsafe impl NSFileManager {
        #[method_id(homeDirectoryForCurrentUser)]
        pub unsafe fn homeDirectoryForCurrentUser(&self) -> Id<NSURL, Shared>;
        #[method_id(temporaryDirectory)]
        pub unsafe fn temporaryDirectory(&self) -> Id<NSURL, Shared>;
        #[method_id(homeDirectoryForUser:)]
        pub unsafe fn homeDirectoryForUser(&self, userName: &NSString)
            -> Option<Id<NSURL, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSCopyLinkMoveHandler"]
    unsafe impl NSObject {
        #[method(fileManager:shouldProceedAfterError:)]
        pub unsafe fn fileManager_shouldProceedAfterError(
            &self,
            fm: &NSFileManager,
            errorInfo: &NSDictionary,
        ) -> bool;
        #[method(fileManager:willProcessPath:)]
        pub unsafe fn fileManager_willProcessPath(&self, fm: &NSFileManager, path: &NSString);
    }
);
pub type NSFileManagerDelegate = NSObject;
__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSDirectoryEnumerator<ObjectType: Message>;
    unsafe impl<ObjectType: Message> ClassType for NSDirectoryEnumerator<ObjectType> {
        type Super = NSEnumerator;
    }
);
extern_methods!(
    unsafe impl<ObjectType: Message> NSDirectoryEnumerator<ObjectType> {
        #[method_id(fileAttributes)]
        pub unsafe fn fileAttributes(
            &self,
        ) -> Option<Id<NSDictionary<NSFileAttributeKey, Object>, Shared>>;
        #[method_id(directoryAttributes)]
        pub unsafe fn directoryAttributes(
            &self,
        ) -> Option<Id<NSDictionary<NSFileAttributeKey, Object>, Shared>>;
        #[method(isEnumeratingDirectoryPostOrder)]
        pub unsafe fn isEnumeratingDirectoryPostOrder(&self) -> bool;
        #[method(skipDescendents)]
        pub unsafe fn skipDescendents(&self);
        #[method(level)]
        pub unsafe fn level(&self) -> NSUInteger;
        #[method(skipDescendants)]
        pub unsafe fn skipDescendants(&self);
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSFileProviderService;
    unsafe impl ClassType for NSFileProviderService {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSFileProviderService {
        #[method(getFileProviderConnectionWithCompletionHandler:)]
        pub unsafe fn getFileProviderConnectionWithCompletionHandler(
            &self,
            completionHandler: TodoBlock,
        );
        #[method_id(name)]
        pub unsafe fn name(&self) -> Id<NSFileProviderServiceName, Shared>;
    }
);
extern_methods!(
    #[doc = "NSFileAttributes"]
    unsafe impl<KeyType: Message, ObjectType: Message> NSDictionary<KeyType, ObjectType> {
        #[method(fileSize)]
        pub unsafe fn fileSize(&self) -> c_ulonglong;
        #[method_id(fileModificationDate)]
        pub unsafe fn fileModificationDate(&self) -> Option<Id<NSDate, Shared>>;
        #[method_id(fileType)]
        pub unsafe fn fileType(&self) -> Option<Id<NSString, Shared>>;
        #[method(filePosixPermissions)]
        pub unsafe fn filePosixPermissions(&self) -> NSUInteger;
        #[method_id(fileOwnerAccountName)]
        pub unsafe fn fileOwnerAccountName(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(fileGroupOwnerAccountName)]
        pub unsafe fn fileGroupOwnerAccountName(&self) -> Option<Id<NSString, Shared>>;
        #[method(fileSystemNumber)]
        pub unsafe fn fileSystemNumber(&self) -> NSInteger;
        #[method(fileSystemFileNumber)]
        pub unsafe fn fileSystemFileNumber(&self) -> NSUInteger;
        #[method(fileExtensionHidden)]
        pub unsafe fn fileExtensionHidden(&self) -> bool;
        #[method(fileHFSCreatorCode)]
        pub unsafe fn fileHFSCreatorCode(&self) -> OSType;
        #[method(fileHFSTypeCode)]
        pub unsafe fn fileHFSTypeCode(&self) -> OSType;
        #[method(fileIsImmutable)]
        pub unsafe fn fileIsImmutable(&self) -> bool;
        #[method(fileIsAppendOnly)]
        pub unsafe fn fileIsAppendOnly(&self) -> bool;
        #[method_id(fileCreationDate)]
        pub unsafe fn fileCreationDate(&self) -> Option<Id<NSDate, Shared>>;
        #[method_id(fileOwnerAccountID)]
        pub unsafe fn fileOwnerAccountID(&self) -> Option<Id<NSNumber, Shared>>;
        #[method_id(fileGroupOwnerAccountID)]
        pub unsafe fn fileGroupOwnerAccountID(&self) -> Option<Id<NSNumber, Shared>>;
    }
);
