use super::__exported::NSError;
use super::__exported::NSLock;
use super::__exported::NSNumber;
use super::__exported::NSString;
use super::__exported::NSURL;
use super::__exported::NSUUID;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSDictionary::*;
use crate::Foundation::generated::NSNotification::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSProgress::*;
use crate::Foundation::generated::NSSet::*;
use crate::Foundation::generated::NSString::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSBundle;
    unsafe impl ClassType for NSBundle {
        type Super = NSObject;
    }
);
impl NSBundle {
    pub unsafe fn bundleWithPath(path: &NSString) -> Option<Id<Self, Shared>> {
        msg_send_id![Self::class(), bundleWithPath: path]
    }
    pub unsafe fn initWithPath(&self, path: &NSString) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithPath: path]
    }
    pub unsafe fn bundleWithURL(url: &NSURL) -> Option<Id<Self, Shared>> {
        msg_send_id![Self::class(), bundleWithURL: url]
    }
    pub unsafe fn initWithURL(&self, url: &NSURL) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithURL: url]
    }
    pub unsafe fn bundleForClass(aClass: &Class) -> Id<NSBundle, Shared> {
        msg_send_id![Self::class(), bundleForClass: aClass]
    }
    pub unsafe fn bundleWithIdentifier(identifier: &NSString) -> Option<Id<NSBundle, Shared>> {
        msg_send_id![Self::class(), bundleWithIdentifier: identifier]
    }
    pub unsafe fn load(&self) -> bool {
        msg_send![self, load]
    }
    pub unsafe fn unload(&self) -> bool {
        msg_send![self, unload]
    }
    pub unsafe fn preflightAndReturnError(&self, error: *mut *mut NSError) -> bool {
        msg_send![self, preflightAndReturnError: error]
    }
    pub unsafe fn loadAndReturnError(&self, error: *mut *mut NSError) -> bool {
        msg_send![self, loadAndReturnError: error]
    }
    pub unsafe fn URLForAuxiliaryExecutable(
        &self,
        executableName: &NSString,
    ) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, URLForAuxiliaryExecutable: executableName]
    }
    pub unsafe fn pathForAuxiliaryExecutable(
        &self,
        executableName: &NSString,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, pathForAuxiliaryExecutable: executableName]
    }
    pub unsafe fn URLForResource_withExtension_subdirectory_inBundleWithURL(
        name: Option<&NSString>,
        ext: Option<&NSString>,
        subpath: Option<&NSString>,
        bundleURL: &NSURL,
    ) -> Option<Id<NSURL, Shared>> {
        msg_send_id![
            Self::class(),
            URLForResource: name,
            withExtension: ext,
            subdirectory: subpath,
            inBundleWithURL: bundleURL
        ]
    }
    pub unsafe fn URLsForResourcesWithExtension_subdirectory_inBundleWithURL(
        ext: Option<&NSString>,
        subpath: Option<&NSString>,
        bundleURL: &NSURL,
    ) -> TodoGenerics {
        msg_send![
            Self::class(),
            URLsForResourcesWithExtension: ext,
            subdirectory: subpath,
            inBundleWithURL: bundleURL
        ]
    }
    pub unsafe fn URLForResource_withExtension(
        &self,
        name: Option<&NSString>,
        ext: Option<&NSString>,
    ) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, URLForResource: name, withExtension: ext]
    }
    pub unsafe fn URLForResource_withExtension_subdirectory(
        &self,
        name: Option<&NSString>,
        ext: Option<&NSString>,
        subpath: Option<&NSString>,
    ) -> Option<Id<NSURL, Shared>> {
        msg_send_id![
            self,
            URLForResource: name,
            withExtension: ext,
            subdirectory: subpath
        ]
    }
    pub unsafe fn URLForResource_withExtension_subdirectory_localization(
        &self,
        name: Option<&NSString>,
        ext: Option<&NSString>,
        subpath: Option<&NSString>,
        localizationName: Option<&NSString>,
    ) -> Option<Id<NSURL, Shared>> {
        msg_send_id![
            self,
            URLForResource: name,
            withExtension: ext,
            subdirectory: subpath,
            localization: localizationName
        ]
    }
    pub unsafe fn URLsForResourcesWithExtension_subdirectory(
        &self,
        ext: Option<&NSString>,
        subpath: Option<&NSString>,
    ) -> TodoGenerics {
        msg_send![
            self,
            URLsForResourcesWithExtension: ext,
            subdirectory: subpath
        ]
    }
    pub unsafe fn URLsForResourcesWithExtension_subdirectory_localization(
        &self,
        ext: Option<&NSString>,
        subpath: Option<&NSString>,
        localizationName: Option<&NSString>,
    ) -> TodoGenerics {
        msg_send![
            self,
            URLsForResourcesWithExtension: ext,
            subdirectory: subpath,
            localization: localizationName
        ]
    }
    pub unsafe fn pathForResource_ofType_inDirectory(
        name: Option<&NSString>,
        ext: Option<&NSString>,
        bundlePath: &NSString,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![
            Self::class(),
            pathForResource: name,
            ofType: ext,
            inDirectory: bundlePath
        ]
    }
    pub unsafe fn pathsForResourcesOfType_inDirectory(
        ext: Option<&NSString>,
        bundlePath: &NSString,
    ) -> TodoGenerics {
        msg_send![
            Self::class(),
            pathsForResourcesOfType: ext,
            inDirectory: bundlePath
        ]
    }
    pub unsafe fn pathForResource_ofType(
        &self,
        name: Option<&NSString>,
        ext: Option<&NSString>,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, pathForResource: name, ofType: ext]
    }
    pub unsafe fn pathForResource_ofType_inDirectory(
        &self,
        name: Option<&NSString>,
        ext: Option<&NSString>,
        subpath: Option<&NSString>,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![
            self,
            pathForResource: name,
            ofType: ext,
            inDirectory: subpath
        ]
    }
    pub unsafe fn pathForResource_ofType_inDirectory_forLocalization(
        &self,
        name: Option<&NSString>,
        ext: Option<&NSString>,
        subpath: Option<&NSString>,
        localizationName: Option<&NSString>,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![
            self,
            pathForResource: name,
            ofType: ext,
            inDirectory: subpath,
            forLocalization: localizationName
        ]
    }
    pub unsafe fn pathsForResourcesOfType_inDirectory(
        &self,
        ext: Option<&NSString>,
        subpath: Option<&NSString>,
    ) -> TodoGenerics {
        msg_send![self, pathsForResourcesOfType: ext, inDirectory: subpath]
    }
    pub unsafe fn pathsForResourcesOfType_inDirectory_forLocalization(
        &self,
        ext: Option<&NSString>,
        subpath: Option<&NSString>,
        localizationName: Option<&NSString>,
    ) -> TodoGenerics {
        msg_send![
            self,
            pathsForResourcesOfType: ext,
            inDirectory: subpath,
            forLocalization: localizationName
        ]
    }
    pub unsafe fn localizedStringForKey_value_table(
        &self,
        key: &NSString,
        value: Option<&NSString>,
        tableName: Option<&NSString>,
    ) -> Id<NSString, Shared> {
        msg_send_id![
            self,
            localizedStringForKey: key,
            value: value,
            table: tableName
        ]
    }
    pub unsafe fn localizedAttributedStringForKey_value_table(
        &self,
        key: &NSString,
        value: Option<&NSString>,
        tableName: Option<&NSString>,
    ) -> Id<NSAttributedString, Shared> {
        msg_send_id![
            self,
            localizedAttributedStringForKey: key,
            value: value,
            table: tableName
        ]
    }
    pub unsafe fn objectForInfoDictionaryKey(&self, key: &NSString) -> Option<Id<Object, Shared>> {
        msg_send_id![self, objectForInfoDictionaryKey: key]
    }
    pub unsafe fn classNamed(&self, className: &NSString) -> Option<&Class> {
        msg_send![self, classNamed: className]
    }
    pub unsafe fn preferredLocalizationsFromArray(
        localizationsArray: TodoGenerics,
    ) -> TodoGenerics {
        msg_send![
            Self::class(),
            preferredLocalizationsFromArray: localizationsArray
        ]
    }
    pub unsafe fn preferredLocalizationsFromArray_forPreferences(
        localizationsArray: TodoGenerics,
        preferencesArray: TodoGenerics,
    ) -> TodoGenerics {
        msg_send![
            Self::class(),
            preferredLocalizationsFromArray: localizationsArray,
            forPreferences: preferencesArray
        ]
    }
    pub unsafe fn mainBundle() -> Id<NSBundle, Shared> {
        msg_send_id![Self::class(), mainBundle]
    }
    pub unsafe fn allBundles() -> TodoGenerics {
        msg_send![Self::class(), allBundles]
    }
    pub unsafe fn allFrameworks() -> TodoGenerics {
        msg_send![Self::class(), allFrameworks]
    }
    pub unsafe fn isLoaded(&self) -> bool {
        msg_send![self, isLoaded]
    }
    pub unsafe fn bundleURL(&self) -> Id<NSURL, Shared> {
        msg_send_id![self, bundleURL]
    }
    pub unsafe fn resourceURL(&self) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, resourceURL]
    }
    pub unsafe fn executableURL(&self) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, executableURL]
    }
    pub unsafe fn privateFrameworksURL(&self) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, privateFrameworksURL]
    }
    pub unsafe fn sharedFrameworksURL(&self) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, sharedFrameworksURL]
    }
    pub unsafe fn sharedSupportURL(&self) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, sharedSupportURL]
    }
    pub unsafe fn builtInPlugInsURL(&self) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, builtInPlugInsURL]
    }
    pub unsafe fn appStoreReceiptURL(&self) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, appStoreReceiptURL]
    }
    pub unsafe fn bundlePath(&self) -> Id<NSString, Shared> {
        msg_send_id![self, bundlePath]
    }
    pub unsafe fn resourcePath(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, resourcePath]
    }
    pub unsafe fn executablePath(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, executablePath]
    }
    pub unsafe fn privateFrameworksPath(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, privateFrameworksPath]
    }
    pub unsafe fn sharedFrameworksPath(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, sharedFrameworksPath]
    }
    pub unsafe fn sharedSupportPath(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, sharedSupportPath]
    }
    pub unsafe fn builtInPlugInsPath(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, builtInPlugInsPath]
    }
    pub unsafe fn bundleIdentifier(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, bundleIdentifier]
    }
    pub unsafe fn infoDictionary(&self) -> TodoGenerics {
        msg_send![self, infoDictionary]
    }
    pub unsafe fn localizedInfoDictionary(&self) -> TodoGenerics {
        msg_send![self, localizedInfoDictionary]
    }
    pub unsafe fn principalClass(&self) -> Option<&Class> {
        msg_send![self, principalClass]
    }
    pub unsafe fn preferredLocalizations(&self) -> TodoGenerics {
        msg_send![self, preferredLocalizations]
    }
    pub unsafe fn localizations(&self) -> TodoGenerics {
        msg_send![self, localizations]
    }
    pub unsafe fn developmentLocalization(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, developmentLocalization]
    }
    pub unsafe fn executableArchitectures(&self) -> TodoGenerics {
        msg_send![self, executableArchitectures]
    }
}
#[doc = "NSBundleExtensionMethods"]
impl NSString {
    pub unsafe fn variantFittingPresentationWidth(&self, width: NSInteger) -> Id<NSString, Shared> {
        msg_send_id![self, variantFittingPresentationWidth: width]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSBundleResourceRequest;
    unsafe impl ClassType for NSBundleResourceRequest {
        type Super = NSObject;
    }
);
impl NSBundleResourceRequest {
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn initWithTags(&self, tags: TodoGenerics) -> Id<Self, Shared> {
        msg_send_id![self, initWithTags: tags]
    }
    pub unsafe fn initWithTags_bundle(
        &self,
        tags: TodoGenerics,
        bundle: &NSBundle,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithTags: tags, bundle: bundle]
    }
    pub unsafe fn beginAccessingResourcesWithCompletionHandler(
        &self,
        completionHandler: TodoBlock,
    ) {
        msg_send![
            self,
            beginAccessingResourcesWithCompletionHandler: completionHandler
        ]
    }
    pub unsafe fn conditionallyBeginAccessingResourcesWithCompletionHandler(
        &self,
        completionHandler: TodoBlock,
    ) {
        msg_send![
            self,
            conditionallyBeginAccessingResourcesWithCompletionHandler: completionHandler
        ]
    }
    pub unsafe fn endAccessingResources(&self) {
        msg_send![self, endAccessingResources]
    }
    pub unsafe fn loadingPriority(&self) -> c_double {
        msg_send![self, loadingPriority]
    }
    pub unsafe fn setLoadingPriority(&self, loadingPriority: c_double) {
        msg_send![self, setLoadingPriority: loadingPriority]
    }
    pub unsafe fn tags(&self) -> TodoGenerics {
        msg_send![self, tags]
    }
    pub unsafe fn bundle(&self) -> Id<NSBundle, Shared> {
        msg_send_id![self, bundle]
    }
    pub unsafe fn progress(&self) -> Id<NSProgress, Shared> {
        msg_send_id![self, progress]
    }
}
#[doc = "NSBundleResourceRequestAdditions"]
impl NSBundle {
    pub unsafe fn setPreservationPriority_forTags(&self, priority: c_double, tags: TodoGenerics) {
        msg_send![self, setPreservationPriority: priority, forTags: tags]
    }
    pub unsafe fn preservationPriorityForTag(&self, tag: &NSString) -> c_double {
        msg_send![self, preservationPriorityForTag: tag]
    }
}
