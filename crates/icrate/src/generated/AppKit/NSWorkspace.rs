#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSWorkspace;
    unsafe impl ClassType for NSWorkspace {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSWorkspace {
        #[method_id(sharedWorkspace)]
        pub unsafe fn sharedWorkspace() -> Id<NSWorkspace, Shared>;
        #[method_id(notificationCenter)]
        pub unsafe fn notificationCenter(&self) -> Id<NSNotificationCenter, Shared>;
        #[method(openURL:)]
        pub unsafe fn openURL(&self, url: &NSURL) -> bool;
        #[method(openURL:configuration:completionHandler:)]
        pub unsafe fn openURL_configuration_completionHandler(
            &self,
            url: &NSURL,
            configuration: &NSWorkspaceOpenConfiguration,
            completionHandler: TodoBlock,
        );
        #[method(openURLs:withApplicationAtURL:configuration:completionHandler:)]
        pub unsafe fn openURLs_withApplicationAtURL_configuration_completionHandler(
            &self,
            urls: &NSArray<NSURL>,
            applicationURL: &NSURL,
            configuration: &NSWorkspaceOpenConfiguration,
            completionHandler: TodoBlock,
        );
        #[method(openApplicationAtURL:configuration:completionHandler:)]
        pub unsafe fn openApplicationAtURL_configuration_completionHandler(
            &self,
            applicationURL: &NSURL,
            configuration: &NSWorkspaceOpenConfiguration,
            completionHandler: TodoBlock,
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
        #[method(noteFileSystemChanged:)]
        pub unsafe fn noteFileSystemChanged(&self, path: &NSString);
        #[method(isFilePackageAtPath:)]
        pub unsafe fn isFilePackageAtPath(&self, fullPath: &NSString) -> bool;
        #[method_id(iconForFile:)]
        pub unsafe fn iconForFile(&self, fullPath: &NSString) -> Id<NSImage, Shared>;
        #[method_id(iconForFiles:)]
        pub unsafe fn iconForFiles(
            &self,
            fullPaths: &NSArray<NSString>,
        ) -> Option<Id<NSImage, Shared>>;
        #[method_id(iconForContentType:)]
        pub unsafe fn iconForContentType(&self, contentType: &UTType) -> Id<NSImage, Shared>;
        #[method(setIcon:forFile:options:)]
        pub unsafe fn setIcon_forFile_options(
            &self,
            image: Option<&NSImage>,
            fullPath: &NSString,
            options: NSWorkspaceIconCreationOptions,
        ) -> bool;
        #[method_id(fileLabels)]
        pub unsafe fn fileLabels(&self) -> Id<NSArray<NSString>, Shared>;
        #[method_id(fileLabelColors)]
        pub unsafe fn fileLabelColors(&self) -> Id<NSArray<NSColor>, Shared>;
        #[method(recycleURLs:completionHandler:)]
        pub unsafe fn recycleURLs_completionHandler(
            &self,
            URLs: &NSArray<NSURL>,
            handler: TodoBlock,
        );
        #[method(duplicateURLs:completionHandler:)]
        pub unsafe fn duplicateURLs_completionHandler(
            &self,
            URLs: &NSArray<NSURL>,
            handler: TodoBlock,
        );
        #[method(getFileSystemInfoForPath:isRemovable:isWritable:isUnmountable:description:type:)]
        pub unsafe fn getFileSystemInfoForPath_isRemovable_isWritable_isUnmountable_description_type(
            &self,
            fullPath: &NSString,
            removableFlag: *mut bool,
            writableFlag: *mut bool,
            unmountableFlag: *mut bool,
            description: Option<&mut Option<Id<NSString, Shared>>>,
            fileSystemType: Option<&mut Option<Id<NSString, Shared>>>,
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
        #[method_id(URLForApplicationWithBundleIdentifier:)]
        pub unsafe fn URLForApplicationWithBundleIdentifier(
            &self,
            bundleIdentifier: &NSString,
        ) -> Option<Id<NSURL, Shared>>;
        #[method_id(URLsForApplicationsWithBundleIdentifier:)]
        pub unsafe fn URLsForApplicationsWithBundleIdentifier(
            &self,
            bundleIdentifier: &NSString,
        ) -> Id<NSArray<NSURL>, Shared>;
        #[method_id(URLForApplicationToOpenURL:)]
        pub unsafe fn URLForApplicationToOpenURL(&self, url: &NSURL) -> Option<Id<NSURL, Shared>>;
        #[method_id(URLsForApplicationsToOpenURL:)]
        pub unsafe fn URLsForApplicationsToOpenURL(
            &self,
            url: &NSURL,
        ) -> Id<NSArray<NSURL>, Shared>;
        #[method(setDefaultApplicationAtURL:toOpenContentTypeOfFileAtURL:completionHandler:)]
        pub unsafe fn setDefaultApplicationAtURL_toOpenContentTypeOfFileAtURL_completionHandler(
            &self,
            applicationURL: &NSURL,
            url: &NSURL,
            completionHandler: TodoBlock,
        );
        #[method(setDefaultApplicationAtURL:toOpenURLsWithScheme:completionHandler:)]
        pub unsafe fn setDefaultApplicationAtURL_toOpenURLsWithScheme_completionHandler(
            &self,
            applicationURL: &NSURL,
            urlScheme: &NSString,
            completionHandler: TodoBlock,
        );
        #[method(setDefaultApplicationAtURL:toOpenFileAtURL:completionHandler:)]
        pub unsafe fn setDefaultApplicationAtURL_toOpenFileAtURL_completionHandler(
            &self,
            applicationURL: &NSURL,
            url: &NSURL,
            completionHandler: TodoBlock,
        );
        #[method_id(URLForApplicationToOpenContentType:)]
        pub unsafe fn URLForApplicationToOpenContentType(
            &self,
            contentType: &UTType,
        ) -> Option<Id<NSURL, Shared>>;
        #[method_id(URLsForApplicationsToOpenContentType:)]
        pub unsafe fn URLsForApplicationsToOpenContentType(
            &self,
            contentType: &UTType,
        ) -> Id<NSArray<NSURL>, Shared>;
        #[method(setDefaultApplicationAtURL:toOpenContentType:completionHandler:)]
        pub unsafe fn setDefaultApplicationAtURL_toOpenContentType_completionHandler(
            &self,
            applicationURL: &NSURL,
            contentType: &UTType,
            completionHandler: TodoBlock,
        );
        #[method_id(frontmostApplication)]
        pub unsafe fn frontmostApplication(&self) -> Option<Id<NSRunningApplication, Shared>>;
        #[method_id(menuBarOwningApplication)]
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
        #[method_id(configuration)]
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
        #[method_id(arguments)]
        pub unsafe fn arguments(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(setArguments:)]
        pub unsafe fn setArguments(&self, arguments: &NSArray<NSString>);
        #[method_id(environment)]
        pub unsafe fn environment(&self) -> Id<NSDictionary<NSString, NSString>, Shared>;
        #[method(setEnvironment:)]
        pub unsafe fn setEnvironment(&self, environment: &NSDictionary<NSString, NSString>);
        #[method_id(appleEvent)]
        pub unsafe fn appleEvent(&self) -> Option<Id<NSAppleEventDescriptor, Shared>>;
        #[method(setAppleEvent:)]
        pub unsafe fn setAppleEvent(&self, appleEvent: Option<&NSAppleEventDescriptor>);
        #[method(architecture)]
        pub unsafe fn architecture(&self) -> cpu_type_t;
        #[method(setArchitecture:)]
        pub unsafe fn setArchitecture(&self, architecture: cpu_type_t);
        #[method(requiresUniversalLinks)]
        pub unsafe fn requiresUniversalLinks(&self) -> bool;
        #[method(setRequiresUniversalLinks:)]
        pub unsafe fn setRequiresUniversalLinks(&self, requiresUniversalLinks: bool);
    }
);
pub type NSWorkspaceDesktopImageOptionKey = NSString;
extern_methods!(
    #[doc = "NSDesktopImages"]
    unsafe impl NSWorkspace {
        #[method(setDesktopImageURL:forScreen:options:error:)]
        pub unsafe fn setDesktopImageURL_forScreen_options_error(
            &self,
            url: &NSURL,
            screen: &NSScreen,
            options: &NSDictionary<NSWorkspaceDesktopImageOptionKey, Object>,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method_id(desktopImageURLForScreen:)]
        pub unsafe fn desktopImageURLForScreen(
            &self,
            screen: &NSScreen,
        ) -> Option<Id<NSURL, Shared>>;
        #[method_id(desktopImageOptionsForScreen:)]
        pub unsafe fn desktopImageOptionsForScreen(
            &self,
            screen: &NSScreen,
        ) -> Option<Id<NSDictionary<NSWorkspaceDesktopImageOptionKey, Object>, Shared>>;
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
    #[doc = "NSWorkspaceAuthorization"]
    unsafe impl NSWorkspace {
        #[method(requestAuthorizationOfType:completionHandler:)]
        pub unsafe fn requestAuthorizationOfType_completionHandler(
            &self,
            type_: NSWorkspaceAuthorizationType,
            completionHandler: TodoBlock,
        );
    }
);
extern_methods!(
    #[doc = "NSWorkspaceAuthorization"]
    unsafe impl NSFileManager {
        #[method_id(fileManagerWithAuthorization:)]
        pub unsafe fn fileManagerWithAuthorization(
            authorization: &NSWorkspaceAuthorization,
        ) -> Id<Self, Shared>;
    }
);
pub type NSWorkspaceFileOperationName = NSString;
pub type NSWorkspaceLaunchConfigurationKey = NSString;
extern_methods!(
    #[doc = "NSDeprecated"]
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
        #[method_id(launchApplicationAtURL:options:configuration:error:)]
        pub unsafe fn launchApplicationAtURL_options_configuration_error(
            &self,
            url: &NSURL,
            options: NSWorkspaceLaunchOptions,
            configuration: &NSDictionary<NSWorkspaceLaunchConfigurationKey, Object>,
        ) -> Result<Id<NSRunningApplication, Shared>, Id<NSError, Shared>>;
        #[method_id(openURL:options:configuration:error:)]
        pub unsafe fn openURL_options_configuration_error(
            &self,
            url: &NSURL,
            options: NSWorkspaceLaunchOptions,
            configuration: &NSDictionary<NSWorkspaceLaunchConfigurationKey, Object>,
        ) -> Result<Id<NSRunningApplication, Shared>, Id<NSError, Shared>>;
        #[method_id(openURLs:withApplicationAtURL:options:configuration:error:)]
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
        #[method_id(fullPathForApplication:)]
        pub unsafe fn fullPathForApplication(
            &self,
            appName: &NSString,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(absolutePathForAppBundleWithIdentifier:)]
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
            identifier: Option<&mut Option<Id<NSNumber, Shared>>>,
        ) -> bool;
        #[method(openURLs:withAppBundleIdentifier:options:additionalEventParamDescriptor:launchIdentifiers:)]
        pub unsafe fn openURLs_withAppBundleIdentifier_options_additionalEventParamDescriptor_launchIdentifiers(
            &self,
            urls: &NSArray<NSURL>,
            bundleIdentifier: Option<&NSString>,
            options: NSWorkspaceLaunchOptions,
            descriptor: Option<&NSAppleEventDescriptor>,
            identifiers: Option<&mut Option<Id<NSArray<NSNumber>, Shared>>>,
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
        #[method(noteFileSystemChanged)]
        pub unsafe fn noteFileSystemChanged(&self);
        #[method(fileSystemChanged)]
        pub unsafe fn fileSystemChanged(&self) -> bool;
        #[method(userDefaultsChanged)]
        pub unsafe fn userDefaultsChanged(&self) -> bool;
        #[method_id(mountNewRemovableMedia)]
        pub unsafe fn mountNewRemovableMedia(&self) -> Option<Id<NSArray, Shared>>;
        #[method_id(activeApplication)]
        pub unsafe fn activeApplication(&self) -> Option<Id<NSDictionary, Shared>>;
        #[method_id(mountedLocalVolumePaths)]
        pub unsafe fn mountedLocalVolumePaths(&self) -> Option<Id<NSArray, Shared>>;
        #[method_id(mountedRemovableMedia)]
        pub unsafe fn mountedRemovableMedia(&self) -> Option<Id<NSArray, Shared>>;
        #[method_id(launchedApplications)]
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
            appName: Option<&mut Option<Id<NSString, Shared>>>,
            type_: Option<&mut Option<Id<NSString, Shared>>>,
        ) -> bool;
        #[method_id(iconForFileType:)]
        pub unsafe fn iconForFileType(&self, fileType: &NSString) -> Id<NSImage, Shared>;
        #[method_id(typeOfFile:error:)]
        pub unsafe fn typeOfFile_error(
            &self,
            absoluteFilePath: &NSString,
        ) -> Result<Id<NSString, Shared>, Id<NSError, Shared>>;
        #[method_id(localizedDescriptionForType:)]
        pub unsafe fn localizedDescriptionForType(
            &self,
            typeName: &NSString,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(preferredFilenameExtensionForType:)]
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
