//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSWorkspaceIconCreationOptions {
        NSExcludeQuickDrawElementsIconCreationOption = 1 << 1,
        NSExclude10_4ElementsIconCreationOption = 1 << 2,
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSWorkspace;

    unsafe impl ClassType for NSWorkspace {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSWorkspace {
        #[method_id(@__retain_semantics Other sharedWorkspace)]
        pub unsafe fn sharedWorkspace() -> Id<NSWorkspace, Shared>;

        #[method_id(@__retain_semantics Other notificationCenter)]
        pub unsafe fn notificationCenter(&self) -> Id<NSNotificationCenter, Shared>;

        #[method(openURL:)]
        pub unsafe fn openURL(&self, url: &NSURL) -> bool;

        #[method(openURL:configuration:completionHandler:)]
        pub unsafe fn openURL_configuration_completionHandler(
            &self,
            url: &NSURL,
            configuration: &NSWorkspaceOpenConfiguration,
            completionHandler: Option<&Block<(*mut NSRunningApplication, *mut NSError), ()>>,
        );

        #[method(openURLs:withApplicationAtURL:configuration:completionHandler:)]
        pub unsafe fn openURLs_withApplicationAtURL_configuration_completionHandler(
            &self,
            urls: &NSArray<NSURL>,
            applicationURL: &NSURL,
            configuration: &NSWorkspaceOpenConfiguration,
            completionHandler: Option<&Block<(*mut NSRunningApplication, *mut NSError), ()>>,
        );

        #[method(openApplicationAtURL:configuration:completionHandler:)]
        pub unsafe fn openApplicationAtURL_configuration_completionHandler(
            &self,
            applicationURL: &NSURL,
            configuration: &NSWorkspaceOpenConfiguration,
            completionHandler: Option<&Block<(*mut NSRunningApplication, *mut NSError), ()>>,
        );

        #[method(selectFile:inFileViewerRootedAtPath:)]
        pub unsafe fn selectFile_inFileViewerRootedAtPath(
            &self,
            fullPath: Option<&NSString>,
            rootFullPath: &NSString,
        ) -> bool;

        #[method(activateFileViewerSelectingURLs:)]
        pub unsafe fn activateFileViewerSelectingURLs(&self, fileURLs: &NSArray<NSURL>);

        #[method(showSearchResultsForQueryString:)]
        pub unsafe fn showSearchResultsForQueryString(&self, queryString: &NSString) -> bool;

        #[method(isFilePackageAtPath:)]
        pub unsafe fn isFilePackageAtPath(&self, fullPath: &NSString) -> bool;

        #[method_id(@__retain_semantics Other iconForFile:)]
        pub unsafe fn iconForFile(&self, fullPath: &NSString) -> Id<NSImage, Shared>;

        #[method_id(@__retain_semantics Other iconForFiles:)]
        pub unsafe fn iconForFiles(
            &self,
            fullPaths: &NSArray<NSString>,
        ) -> Option<Id<NSImage, Shared>>;

        #[method(setIcon:forFile:options:)]
        pub unsafe fn setIcon_forFile_options(
            &self,
            image: Option<&NSImage>,
            fullPath: &NSString,
            options: NSWorkspaceIconCreationOptions,
        ) -> bool;

        #[method_id(@__retain_semantics Other fileLabels)]
        pub unsafe fn fileLabels(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other fileLabelColors)]
        pub unsafe fn fileLabelColors(&self) -> Id<NSArray<NSColor>, Shared>;

        #[method(recycleURLs:completionHandler:)]
        pub unsafe fn recycleURLs_completionHandler(
            &self,
            URLs: &NSArray<NSURL>,
            handler: Option<&Block<(NonNull<NSDictionary<NSURL, NSURL>>, *mut NSError), ()>>,
        );

        #[method(duplicateURLs:completionHandler:)]
        pub unsafe fn duplicateURLs_completionHandler(
            &self,
            URLs: &NSArray<NSURL>,
            handler: Option<&Block<(NonNull<NSDictionary<NSURL, NSURL>>, *mut NSError), ()>>,
        );

        #[method(getFileSystemInfoForPath:isRemovable:isWritable:isUnmountable:description:type:)]
        pub unsafe fn getFileSystemInfoForPath_isRemovable_isWritable_isUnmountable_description_type(
            &self,
            fullPath: &NSString,
            removableFlag: *mut Bool,
            writableFlag: *mut Bool,
            unmountableFlag: *mut Bool,
            description: *mut *mut NSString,
            fileSystemType: *mut *mut NSString,
        ) -> bool;

        #[method(unmountAndEjectDeviceAtPath:)]
        pub unsafe fn unmountAndEjectDeviceAtPath(&self, path: &NSString) -> bool;

        #[method(unmountAndEjectDeviceAtURL:error:)]
        pub unsafe fn unmountAndEjectDeviceAtURL_error(
            &self,
            url: &NSURL,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(extendPowerOffBy:)]
        pub unsafe fn extendPowerOffBy(&self, requested: NSInteger) -> NSInteger;

        #[method(hideOtherApplications)]
        pub unsafe fn hideOtherApplications(&self);

        #[method_id(@__retain_semantics Other URLForApplicationWithBundleIdentifier:)]
        pub unsafe fn URLForApplicationWithBundleIdentifier(
            &self,
            bundleIdentifier: &NSString,
        ) -> Option<Id<NSURL, Shared>>;

        #[method_id(@__retain_semantics Other URLsForApplicationsWithBundleIdentifier:)]
        pub unsafe fn URLsForApplicationsWithBundleIdentifier(
            &self,
            bundleIdentifier: &NSString,
        ) -> Id<NSArray<NSURL>, Shared>;

        #[method_id(@__retain_semantics Other URLForApplicationToOpenURL:)]
        pub unsafe fn URLForApplicationToOpenURL(&self, url: &NSURL) -> Option<Id<NSURL, Shared>>;

        #[method_id(@__retain_semantics Other URLsForApplicationsToOpenURL:)]
        pub unsafe fn URLsForApplicationsToOpenURL(
            &self,
            url: &NSURL,
        ) -> Id<NSArray<NSURL>, Shared>;

        #[method(setDefaultApplicationAtURL:toOpenContentTypeOfFileAtURL:completionHandler:)]
        pub unsafe fn setDefaultApplicationAtURL_toOpenContentTypeOfFileAtURL_completionHandler(
            &self,
            applicationURL: &NSURL,
            url: &NSURL,
            completionHandler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[method(setDefaultApplicationAtURL:toOpenURLsWithScheme:completionHandler:)]
        pub unsafe fn setDefaultApplicationAtURL_toOpenURLsWithScheme_completionHandler(
            &self,
            applicationURL: &NSURL,
            urlScheme: &NSString,
            completionHandler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[method(setDefaultApplicationAtURL:toOpenFileAtURL:completionHandler:)]
        pub unsafe fn setDefaultApplicationAtURL_toOpenFileAtURL_completionHandler(
            &self,
            applicationURL: &NSURL,
            url: &NSURL,
            completionHandler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[method_id(@__retain_semantics Other frontmostApplication)]
        pub unsafe fn frontmostApplication(&self) -> Option<Id<NSRunningApplication, Shared>>;

        #[method_id(@__retain_semantics Other menuBarOwningApplication)]
        pub unsafe fn menuBarOwningApplication(&self) -> Option<Id<NSRunningApplication, Shared>>;
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSWorkspaceOpenConfiguration;

    unsafe impl ClassType for NSWorkspaceOpenConfiguration {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSWorkspaceOpenConfiguration {
        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration() -> Id<Self, Shared>;

        #[method(promptsUserIfNeeded)]
        pub unsafe fn promptsUserIfNeeded(&self) -> bool;

        #[method(setPromptsUserIfNeeded:)]
        pub unsafe fn setPromptsUserIfNeeded(&self, promptsUserIfNeeded: bool);

        #[method(addsToRecentItems)]
        pub unsafe fn addsToRecentItems(&self) -> bool;

        #[method(setAddsToRecentItems:)]
        pub unsafe fn setAddsToRecentItems(&self, addsToRecentItems: bool);

        #[method(activates)]
        pub unsafe fn activates(&self) -> bool;

        #[method(setActivates:)]
        pub unsafe fn setActivates(&self, activates: bool);

        #[method(hides)]
        pub unsafe fn hides(&self) -> bool;

        #[method(setHides:)]
        pub unsafe fn setHides(&self, hides: bool);

        #[method(hidesOthers)]
        pub unsafe fn hidesOthers(&self) -> bool;

        #[method(setHidesOthers:)]
        pub unsafe fn setHidesOthers(&self, hidesOthers: bool);

        #[method(isForPrinting)]
        pub unsafe fn isForPrinting(&self) -> bool;

        #[method(setForPrinting:)]
        pub unsafe fn setForPrinting(&self, forPrinting: bool);

        #[method(createsNewApplicationInstance)]
        pub unsafe fn createsNewApplicationInstance(&self) -> bool;

        #[method(setCreatesNewApplicationInstance:)]
        pub unsafe fn setCreatesNewApplicationInstance(&self, createsNewApplicationInstance: bool);

        #[method(allowsRunningApplicationSubstitution)]
        pub unsafe fn allowsRunningApplicationSubstitution(&self) -> bool;

        #[method(setAllowsRunningApplicationSubstitution:)]
        pub unsafe fn setAllowsRunningApplicationSubstitution(
            &self,
            allowsRunningApplicationSubstitution: bool,
        );

        #[method_id(@__retain_semantics Other arguments)]
        pub unsafe fn arguments(&self) -> Id<NSArray<NSString>, Shared>;

        #[method(setArguments:)]
        pub unsafe fn setArguments(&self, arguments: &NSArray<NSString>);

        #[method_id(@__retain_semantics Other environment)]
        pub unsafe fn environment(&self) -> Id<NSDictionary<NSString, NSString>, Shared>;

        #[method(setEnvironment:)]
        pub unsafe fn setEnvironment(&self, environment: &NSDictionary<NSString, NSString>);

        #[method_id(@__retain_semantics Other appleEvent)]
        pub unsafe fn appleEvent(&self) -> Option<Id<NSAppleEventDescriptor, Shared>>;

        #[method(setAppleEvent:)]
        pub unsafe fn setAppleEvent(&self, appleEvent: Option<&NSAppleEventDescriptor>);

        #[method(requiresUniversalLinks)]
        pub unsafe fn requiresUniversalLinks(&self) -> bool;

        #[method(setRequiresUniversalLinks:)]
        pub unsafe fn setRequiresUniversalLinks(&self, requiresUniversalLinks: bool);
    }
);

pub type NSWorkspaceDesktopImageOptionKey = NSString;

extern_static!(NSWorkspaceDesktopImageScalingKey: &'static NSWorkspaceDesktopImageOptionKey);

extern_static!(NSWorkspaceDesktopImageAllowClippingKey: &'static NSWorkspaceDesktopImageOptionKey);

extern_static!(NSWorkspaceDesktopImageFillColorKey: &'static NSWorkspaceDesktopImageOptionKey);

extern_methods!(
    /// NSDesktopImages
    unsafe impl NSWorkspace {
        #[method(setDesktopImageURL:forScreen:options:error:)]
        pub unsafe fn setDesktopImageURL_forScreen_options_error(
            &self,
            url: &NSURL,
            screen: &NSScreen,
            options: &NSDictionary<NSWorkspaceDesktopImageOptionKey, Object>,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other desktopImageURLForScreen:)]
        pub unsafe fn desktopImageURLForScreen(
            &self,
            screen: &NSScreen,
        ) -> Option<Id<NSURL, Shared>>;

        #[method_id(@__retain_semantics Other desktopImageOptionsForScreen:)]
        pub unsafe fn desktopImageOptionsForScreen(
            &self,
            screen: &NSScreen,
        ) -> Option<Id<NSDictionary<NSWorkspaceDesktopImageOptionKey, Object>, Shared>>;
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSWorkspaceAuthorizationType {
        NSWorkspaceAuthorizationTypeCreateSymbolicLink = 0,
        NSWorkspaceAuthorizationTypeSetAttributes = 1,
        NSWorkspaceAuthorizationTypeReplaceFile = 2,
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSWorkspaceAuthorization;

    unsafe impl ClassType for NSWorkspaceAuthorization {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSWorkspaceAuthorization {}
);

extern_methods!(
    /// NSWorkspaceAuthorization
    unsafe impl NSWorkspace {
        #[method(requestAuthorizationOfType:completionHandler:)]
        pub unsafe fn requestAuthorizationOfType_completionHandler(
            &self,
            type_: NSWorkspaceAuthorizationType,
            completionHandler: &Block<(*mut NSWorkspaceAuthorization, *mut NSError), ()>,
        );
    }
);

extern_methods!(
    /// NSWorkspaceAuthorization
    unsafe impl NSFileManager {
        #[method_id(@__retain_semantics Other fileManagerWithAuthorization:)]
        pub unsafe fn fileManagerWithAuthorization(
            authorization: &NSWorkspaceAuthorization,
        ) -> Id<Self, Shared>;
    }
);

extern_static!(NSWorkspaceApplicationKey: &'static NSString);

extern_static!(NSWorkspaceWillLaunchApplicationNotification: &'static NSNotificationName);

extern_static!(NSWorkspaceDidLaunchApplicationNotification: &'static NSNotificationName);

extern_static!(NSWorkspaceDidTerminateApplicationNotification: &'static NSNotificationName);

extern_static!(NSWorkspaceDidHideApplicationNotification: &'static NSNotificationName);

extern_static!(NSWorkspaceDidUnhideApplicationNotification: &'static NSNotificationName);

extern_static!(NSWorkspaceDidActivateApplicationNotification: &'static NSNotificationName);

extern_static!(NSWorkspaceDidDeactivateApplicationNotification: &'static NSNotificationName);

extern_static!(NSWorkspaceVolumeLocalizedNameKey: &'static NSString);

extern_static!(NSWorkspaceVolumeURLKey: &'static NSString);

extern_static!(NSWorkspaceVolumeOldLocalizedNameKey: &'static NSString);

extern_static!(NSWorkspaceVolumeOldURLKey: &'static NSString);

extern_static!(NSWorkspaceDidMountNotification: &'static NSNotificationName);

extern_static!(NSWorkspaceDidUnmountNotification: &'static NSNotificationName);

extern_static!(NSWorkspaceWillUnmountNotification: &'static NSNotificationName);

extern_static!(NSWorkspaceDidRenameVolumeNotification: &'static NSNotificationName);

extern_static!(NSWorkspaceWillPowerOffNotification: &'static NSNotificationName);

extern_static!(NSWorkspaceWillSleepNotification: &'static NSNotificationName);

extern_static!(NSWorkspaceDidWakeNotification: &'static NSNotificationName);

extern_static!(NSWorkspaceScreensDidSleepNotification: &'static NSNotificationName);

extern_static!(NSWorkspaceScreensDidWakeNotification: &'static NSNotificationName);

extern_static!(NSWorkspaceSessionDidBecomeActiveNotification: &'static NSNotificationName);

extern_static!(NSWorkspaceSessionDidResignActiveNotification: &'static NSNotificationName);

extern_static!(NSWorkspaceDidChangeFileLabelsNotification: &'static NSNotificationName);

extern_static!(NSWorkspaceActiveSpaceDidChangeNotification: &'static NSNotificationName);

pub type NSWorkspaceFileOperationName = NSString;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSWorkspaceLaunchOptions {
        NSWorkspaceLaunchAndPrint = 0x00000002,
        NSWorkspaceLaunchWithErrorPresentation = 0x00000040,
        NSWorkspaceLaunchInhibitingBackgroundOnly = 0x00000080,
        NSWorkspaceLaunchWithoutAddingToRecents = 0x00000100,
        NSWorkspaceLaunchWithoutActivation = 0x00000200,
        NSWorkspaceLaunchAsync = 0x00010000,
        NSWorkspaceLaunchNewInstance = 0x00080000,
        NSWorkspaceLaunchAndHide = 0x00100000,
        NSWorkspaceLaunchAndHideOthers = 0x00200000,
        NSWorkspaceLaunchDefault = NSWorkspaceLaunchAsync,
        NSWorkspaceLaunchAllowingClassicStartup = 0x00020000,
        NSWorkspaceLaunchPreferringClassic = 0x00040000,
    }
);

pub type NSWorkspaceLaunchConfigurationKey = NSString;

extern_static!(
    NSWorkspaceLaunchConfigurationAppleEvent: &'static NSWorkspaceLaunchConfigurationKey
);

extern_static!(NSWorkspaceLaunchConfigurationArguments: &'static NSWorkspaceLaunchConfigurationKey);

extern_static!(
    NSWorkspaceLaunchConfigurationEnvironment: &'static NSWorkspaceLaunchConfigurationKey
);

extern_static!(
    NSWorkspaceLaunchConfigurationArchitecture: &'static NSWorkspaceLaunchConfigurationKey
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSWorkspace {
        #[method(openFile:)]
        pub unsafe fn openFile(&self, fullPath: &NSString) -> bool;

        #[method(openFile:withApplication:)]
        pub unsafe fn openFile_withApplication(
            &self,
            fullPath: &NSString,
            appName: Option<&NSString>,
        ) -> bool;

        #[method(openFile:withApplication:andDeactivate:)]
        pub unsafe fn openFile_withApplication_andDeactivate(
            &self,
            fullPath: &NSString,
            appName: Option<&NSString>,
            flag: bool,
        ) -> bool;

        #[method(launchApplication:)]
        pub unsafe fn launchApplication(&self, appName: &NSString) -> bool;

        #[method_id(@__retain_semantics Other launchApplicationAtURL:options:configuration:error:)]
        pub unsafe fn launchApplicationAtURL_options_configuration_error(
            &self,
            url: &NSURL,
            options: NSWorkspaceLaunchOptions,
            configuration: &NSDictionary<NSWorkspaceLaunchConfigurationKey, Object>,
        ) -> Result<Id<NSRunningApplication, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other openURL:options:configuration:error:)]
        pub unsafe fn openURL_options_configuration_error(
            &self,
            url: &NSURL,
            options: NSWorkspaceLaunchOptions,
            configuration: &NSDictionary<NSWorkspaceLaunchConfigurationKey, Object>,
        ) -> Result<Id<NSRunningApplication, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other openURLs:withApplicationAtURL:options:configuration:error:)]
        pub unsafe fn openURLs_withApplicationAtURL_options_configuration_error(
            &self,
            urls: &NSArray<NSURL>,
            applicationURL: &NSURL,
            options: NSWorkspaceLaunchOptions,
            configuration: &NSDictionary<NSWorkspaceLaunchConfigurationKey, Object>,
        ) -> Result<Id<NSRunningApplication, Shared>, Id<NSError, Shared>>;

        #[method(launchApplication:showIcon:autolaunch:)]
        pub unsafe fn launchApplication_showIcon_autolaunch(
            &self,
            appName: &NSString,
            showIcon: bool,
            autolaunch: bool,
        ) -> bool;

        #[method_id(@__retain_semantics Other fullPathForApplication:)]
        pub unsafe fn fullPathForApplication(
            &self,
            appName: &NSString,
        ) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other absolutePathForAppBundleWithIdentifier:)]
        pub unsafe fn absolutePathForAppBundleWithIdentifier(
            &self,
            bundleIdentifier: &NSString,
        ) -> Option<Id<NSString, Shared>>;

        #[method(launchAppWithBundleIdentifier:options:additionalEventParamDescriptor:launchIdentifier:)]
        pub unsafe fn launchAppWithBundleIdentifier_options_additionalEventParamDescriptor_launchIdentifier(
            &self,
            bundleIdentifier: &NSString,
            options: NSWorkspaceLaunchOptions,
            descriptor: Option<&NSAppleEventDescriptor>,
            identifier: *mut *mut NSNumber,
        ) -> bool;

        #[method(openURLs:withAppBundleIdentifier:options:additionalEventParamDescriptor:launchIdentifiers:)]
        pub unsafe fn openURLs_withAppBundleIdentifier_options_additionalEventParamDescriptor_launchIdentifiers(
            &self,
            urls: &NSArray<NSURL>,
            bundleIdentifier: Option<&NSString>,
            options: NSWorkspaceLaunchOptions,
            descriptor: Option<&NSAppleEventDescriptor>,
            identifiers: *mut *mut NSArray<NSNumber>,
        ) -> bool;

        #[method(openTempFile:)]
        pub unsafe fn openTempFile(&self, fullPath: &NSString) -> bool;

        #[method(findApplications)]
        pub unsafe fn findApplications(&self);

        #[method(noteUserDefaultsChanged)]
        pub unsafe fn noteUserDefaultsChanged(&self);

        #[method(slideImage:from:to:)]
        pub unsafe fn slideImage_from_to(
            &self,
            image: &NSImage,
            fromPoint: NSPoint,
            toPoint: NSPoint,
        );

        #[method(checkForRemovableMedia)]
        pub unsafe fn checkForRemovableMedia(&self);

        #[method(fileSystemChanged)]
        pub unsafe fn fileSystemChanged(&self) -> bool;

        #[method(userDefaultsChanged)]
        pub unsafe fn userDefaultsChanged(&self) -> bool;

        #[method_id(@__retain_semantics Other mountNewRemovableMedia)]
        pub unsafe fn mountNewRemovableMedia(&self) -> Option<Id<NSArray, Shared>>;

        #[method_id(@__retain_semantics Other activeApplication)]
        pub unsafe fn activeApplication(&self) -> Option<Id<NSDictionary, Shared>>;

        #[method_id(@__retain_semantics Other mountedLocalVolumePaths)]
        pub unsafe fn mountedLocalVolumePaths(&self) -> Option<Id<NSArray, Shared>>;

        #[method_id(@__retain_semantics Other mountedRemovableMedia)]
        pub unsafe fn mountedRemovableMedia(&self) -> Option<Id<NSArray, Shared>>;

        #[method_id(@__retain_semantics Other launchedApplications)]
        pub unsafe fn launchedApplications(&self) -> Option<Id<NSArray, Shared>>;

        #[method(openFile:fromImage:at:inView:)]
        pub unsafe fn openFile_fromImage_at_inView(
            &self,
            fullPath: &NSString,
            image: Option<&NSImage>,
            point: NSPoint,
            view: Option<&NSView>,
        ) -> bool;

        #[method(performFileOperation:source:destination:files:tag:)]
        pub unsafe fn performFileOperation_source_destination_files_tag(
            &self,
            operation: &NSWorkspaceFileOperationName,
            source: &NSString,
            destination: &NSString,
            files: &NSArray,
            tag: *mut NSInteger,
        ) -> bool;

        #[method(getInfoForFile:application:type:)]
        pub unsafe fn getInfoForFile_application_type(
            &self,
            fullPath: &NSString,
            appName: *mut *mut NSString,
            type_: *mut *mut NSString,
        ) -> bool;

        #[method_id(@__retain_semantics Other iconForFileType:)]
        pub unsafe fn iconForFileType(&self, fileType: &NSString) -> Id<NSImage, Shared>;

        #[method_id(@__retain_semantics Other typeOfFile:error:)]
        pub unsafe fn typeOfFile_error(
            &self,
            absoluteFilePath: &NSString,
        ) -> Result<Id<NSString, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other localizedDescriptionForType:)]
        pub unsafe fn localizedDescriptionForType(
            &self,
            typeName: &NSString,
        ) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other preferredFilenameExtensionForType:)]
        pub unsafe fn preferredFilenameExtensionForType(
            &self,
            typeName: &NSString,
        ) -> Option<Id<NSString, Shared>>;

        #[method(filenameExtension:isValidForType:)]
        pub unsafe fn filenameExtension_isValidForType(
            &self,
            filenameExtension: &NSString,
            typeName: &NSString,
        ) -> bool;

        #[method(type:conformsToType:)]
        pub unsafe fn type_conformsToType(
            &self,
            firstTypeName: &NSString,
            secondTypeName: &NSString,
        ) -> bool;
    }
);

extern_static!(NSWorkspaceMoveOperation: &'static NSWorkspaceFileOperationName);

extern_static!(NSWorkspaceCopyOperation: &'static NSWorkspaceFileOperationName);

extern_static!(NSWorkspaceLinkOperation: &'static NSWorkspaceFileOperationName);

extern_static!(NSWorkspaceCompressOperation: &'static NSWorkspaceFileOperationName);

extern_static!(NSWorkspaceDecompressOperation: &'static NSWorkspaceFileOperationName);

extern_static!(NSWorkspaceEncryptOperation: &'static NSWorkspaceFileOperationName);

extern_static!(NSWorkspaceDecryptOperation: &'static NSWorkspaceFileOperationName);

extern_static!(NSWorkspaceDestroyOperation: &'static NSWorkspaceFileOperationName);

extern_static!(NSWorkspaceRecycleOperation: &'static NSWorkspaceFileOperationName);

extern_static!(NSWorkspaceDuplicateOperation: &'static NSWorkspaceFileOperationName);

extern_static!(NSWorkspaceDidPerformFileOperationNotification: &'static NSNotificationName);

extern_static!(NSPlainFileType: &'static NSString);

extern_static!(NSDirectoryFileType: &'static NSString);

extern_static!(NSApplicationFileType: &'static NSString);

extern_static!(NSFilesystemFileType: &'static NSString);

extern_static!(NSShellCommandFileType: &'static NSString);
