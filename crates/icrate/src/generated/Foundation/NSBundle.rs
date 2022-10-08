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
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSBundle;
    unsafe impl ClassType for NSBundle {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSBundle {
        #[method_id(mainBundle)]
        pub unsafe fn mainBundle() -> Id<NSBundle, Shared>;
        # [method_id (bundleWithPath :)]
        pub unsafe fn bundleWithPath(path: &NSString) -> Option<Id<Self, Shared>>;
        # [method_id (initWithPath :)]
        pub unsafe fn initWithPath(&self, path: &NSString) -> Option<Id<Self, Shared>>;
        # [method_id (bundleWithURL :)]
        pub unsafe fn bundleWithURL(url: &NSURL) -> Option<Id<Self, Shared>>;
        # [method_id (initWithURL :)]
        pub unsafe fn initWithURL(&self, url: &NSURL) -> Option<Id<Self, Shared>>;
        # [method_id (bundleForClass :)]
        pub unsafe fn bundleForClass(aClass: &Class) -> Id<NSBundle, Shared>;
        # [method_id (bundleWithIdentifier :)]
        pub unsafe fn bundleWithIdentifier(identifier: &NSString) -> Option<Id<NSBundle, Shared>>;
        #[method_id(allBundles)]
        pub unsafe fn allBundles() -> Id<NSArray<NSBundle>, Shared>;
        #[method_id(allFrameworks)]
        pub unsafe fn allFrameworks() -> Id<NSArray<NSBundle>, Shared>;
        #[method(load)]
        pub unsafe fn load(&self) -> bool;
        #[method(isLoaded)]
        pub unsafe fn isLoaded(&self) -> bool;
        #[method(unload)]
        pub unsafe fn unload(&self) -> bool;
        # [method (preflightAndReturnError :)]
        pub unsafe fn preflightAndReturnError(&self) -> Result<(), Id<NSError, Shared>>;
        # [method (loadAndReturnError :)]
        pub unsafe fn loadAndReturnError(&self) -> Result<(), Id<NSError, Shared>>;
        #[method_id(bundleURL)]
        pub unsafe fn bundleURL(&self) -> Id<NSURL, Shared>;
        #[method_id(resourceURL)]
        pub unsafe fn resourceURL(&self) -> Option<Id<NSURL, Shared>>;
        #[method_id(executableURL)]
        pub unsafe fn executableURL(&self) -> Option<Id<NSURL, Shared>>;
        # [method_id (URLForAuxiliaryExecutable :)]
        pub unsafe fn URLForAuxiliaryExecutable(
            &self,
            executableName: &NSString,
        ) -> Option<Id<NSURL, Shared>>;
        #[method_id(privateFrameworksURL)]
        pub unsafe fn privateFrameworksURL(&self) -> Option<Id<NSURL, Shared>>;
        #[method_id(sharedFrameworksURL)]
        pub unsafe fn sharedFrameworksURL(&self) -> Option<Id<NSURL, Shared>>;
        #[method_id(sharedSupportURL)]
        pub unsafe fn sharedSupportURL(&self) -> Option<Id<NSURL, Shared>>;
        #[method_id(builtInPlugInsURL)]
        pub unsafe fn builtInPlugInsURL(&self) -> Option<Id<NSURL, Shared>>;
        #[method_id(appStoreReceiptURL)]
        pub unsafe fn appStoreReceiptURL(&self) -> Option<Id<NSURL, Shared>>;
        #[method_id(bundlePath)]
        pub unsafe fn bundlePath(&self) -> Id<NSString, Shared>;
        #[method_id(resourcePath)]
        pub unsafe fn resourcePath(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(executablePath)]
        pub unsafe fn executablePath(&self) -> Option<Id<NSString, Shared>>;
        # [method_id (pathForAuxiliaryExecutable :)]
        pub unsafe fn pathForAuxiliaryExecutable(
            &self,
            executableName: &NSString,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(privateFrameworksPath)]
        pub unsafe fn privateFrameworksPath(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(sharedFrameworksPath)]
        pub unsafe fn sharedFrameworksPath(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(sharedSupportPath)]
        pub unsafe fn sharedSupportPath(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(builtInPlugInsPath)]
        pub unsafe fn builtInPlugInsPath(&self) -> Option<Id<NSString, Shared>>;
        # [method_id (URLForResource : withExtension : subdirectory : inBundleWithURL :)]
        pub unsafe fn URLForResource_withExtension_subdirectory_inBundleWithURL(
            name: Option<&NSString>,
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
            bundleURL: &NSURL,
        ) -> Option<Id<NSURL, Shared>>;
        # [method_id (URLsForResourcesWithExtension : subdirectory : inBundleWithURL :)]
        pub unsafe fn URLsForResourcesWithExtension_subdirectory_inBundleWithURL(
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
            bundleURL: &NSURL,
        ) -> Option<Id<NSArray<NSURL>, Shared>>;
        # [method_id (URLForResource : withExtension :)]
        pub unsafe fn URLForResource_withExtension(
            &self,
            name: Option<&NSString>,
            ext: Option<&NSString>,
        ) -> Option<Id<NSURL, Shared>>;
        # [method_id (URLForResource : withExtension : subdirectory :)]
        pub unsafe fn URLForResource_withExtension_subdirectory(
            &self,
            name: Option<&NSString>,
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
        ) -> Option<Id<NSURL, Shared>>;
        # [method_id (URLForResource : withExtension : subdirectory : localization :)]
        pub unsafe fn URLForResource_withExtension_subdirectory_localization(
            &self,
            name: Option<&NSString>,
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
            localizationName: Option<&NSString>,
        ) -> Option<Id<NSURL, Shared>>;
        # [method_id (URLsForResourcesWithExtension : subdirectory :)]
        pub unsafe fn URLsForResourcesWithExtension_subdirectory(
            &self,
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
        ) -> Option<Id<NSArray<NSURL>, Shared>>;
        # [method_id (URLsForResourcesWithExtension : subdirectory : localization :)]
        pub unsafe fn URLsForResourcesWithExtension_subdirectory_localization(
            &self,
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
            localizationName: Option<&NSString>,
        ) -> Option<Id<NSArray<NSURL>, Shared>>;
        # [method_id (pathForResource : ofType : inDirectory :)]
        pub unsafe fn pathForResource_ofType_inDirectory(
            name: Option<&NSString>,
            ext: Option<&NSString>,
            bundlePath: &NSString,
        ) -> Option<Id<NSString, Shared>>;
        # [method_id (pathsForResourcesOfType : inDirectory :)]
        pub unsafe fn pathsForResourcesOfType_inDirectory(
            ext: Option<&NSString>,
            bundlePath: &NSString,
        ) -> Id<NSArray<NSString>, Shared>;
        # [method_id (pathForResource : ofType :)]
        pub unsafe fn pathForResource_ofType(
            &self,
            name: Option<&NSString>,
            ext: Option<&NSString>,
        ) -> Option<Id<NSString, Shared>>;
        # [method_id (pathForResource : ofType : inDirectory :)]
        pub unsafe fn pathForResource_ofType_inDirectory(
            &self,
            name: Option<&NSString>,
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
        ) -> Option<Id<NSString, Shared>>;
        # [method_id (pathForResource : ofType : inDirectory : forLocalization :)]
        pub unsafe fn pathForResource_ofType_inDirectory_forLocalization(
            &self,
            name: Option<&NSString>,
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
            localizationName: Option<&NSString>,
        ) -> Option<Id<NSString, Shared>>;
        # [method_id (pathsForResourcesOfType : inDirectory :)]
        pub unsafe fn pathsForResourcesOfType_inDirectory(
            &self,
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
        ) -> Id<NSArray<NSString>, Shared>;
        # [method_id (pathsForResourcesOfType : inDirectory : forLocalization :)]
        pub unsafe fn pathsForResourcesOfType_inDirectory_forLocalization(
            &self,
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
            localizationName: Option<&NSString>,
        ) -> Id<NSArray<NSString>, Shared>;
        # [method_id (localizedStringForKey : value : table :)]
        pub unsafe fn localizedStringForKey_value_table(
            &self,
            key: &NSString,
            value: Option<&NSString>,
            tableName: Option<&NSString>,
        ) -> Id<NSString, Shared>;
        # [method_id (localizedAttributedStringForKey : value : table :)]
        pub unsafe fn localizedAttributedStringForKey_value_table(
            &self,
            key: &NSString,
            value: Option<&NSString>,
            tableName: Option<&NSString>,
        ) -> Id<NSAttributedString, Shared>;
        #[method_id(bundleIdentifier)]
        pub unsafe fn bundleIdentifier(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(infoDictionary)]
        pub unsafe fn infoDictionary(&self) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;
        #[method_id(localizedInfoDictionary)]
        pub unsafe fn localizedInfoDictionary(
            &self,
        ) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;
        # [method_id (objectForInfoDictionaryKey :)]
        pub unsafe fn objectForInfoDictionaryKey(
            &self,
            key: &NSString,
        ) -> Option<Id<Object, Shared>>;
        # [method (classNamed :)]
        pub unsafe fn classNamed(&self, className: &NSString) -> Option<&Class>;
        #[method(principalClass)]
        pub unsafe fn principalClass(&self) -> Option<&Class>;
        #[method_id(preferredLocalizations)]
        pub unsafe fn preferredLocalizations(&self) -> Id<NSArray<NSString>, Shared>;
        #[method_id(localizations)]
        pub unsafe fn localizations(&self) -> Id<NSArray<NSString>, Shared>;
        #[method_id(developmentLocalization)]
        pub unsafe fn developmentLocalization(&self) -> Option<Id<NSString, Shared>>;
        # [method_id (preferredLocalizationsFromArray :)]
        pub unsafe fn preferredLocalizationsFromArray(
            localizationsArray: &NSArray<NSString>,
        ) -> Id<NSArray<NSString>, Shared>;
        # [method_id (preferredLocalizationsFromArray : forPreferences :)]
        pub unsafe fn preferredLocalizationsFromArray_forPreferences(
            localizationsArray: &NSArray<NSString>,
            preferencesArray: Option<&NSArray<NSString>>,
        ) -> Id<NSArray<NSString>, Shared>;
        #[method_id(executableArchitectures)]
        pub unsafe fn executableArchitectures(&self) -> Option<Id<NSArray<NSNumber>, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSBundleExtensionMethods"]
    unsafe impl NSString {
        # [method_id (variantFittingPresentationWidth :)]
        pub unsafe fn variantFittingPresentationWidth(
            &self,
            width: NSInteger,
        ) -> Id<NSString, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSBundleResourceRequest;
    unsafe impl ClassType for NSBundleResourceRequest {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSBundleResourceRequest {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        # [method_id (initWithTags :)]
        pub unsafe fn initWithTags(&self, tags: &NSSet<NSString>) -> Id<Self, Shared>;
        # [method_id (initWithTags : bundle :)]
        pub unsafe fn initWithTags_bundle(
            &self,
            tags: &NSSet<NSString>,
            bundle: &NSBundle,
        ) -> Id<Self, Shared>;
        #[method(loadingPriority)]
        pub unsafe fn loadingPriority(&self) -> c_double;
        # [method (setLoadingPriority :)]
        pub unsafe fn setLoadingPriority(&self, loadingPriority: c_double);
        #[method_id(tags)]
        pub unsafe fn tags(&self) -> Id<NSSet<NSString>, Shared>;
        #[method_id(bundle)]
        pub unsafe fn bundle(&self) -> Id<NSBundle, Shared>;
        # [method (beginAccessingResourcesWithCompletionHandler :)]
        pub unsafe fn beginAccessingResourcesWithCompletionHandler(
            &self,
            completionHandler: TodoBlock,
        );
        # [method (conditionallyBeginAccessingResourcesWithCompletionHandler :)]
        pub unsafe fn conditionallyBeginAccessingResourcesWithCompletionHandler(
            &self,
            completionHandler: TodoBlock,
        );
        #[method(endAccessingResources)]
        pub unsafe fn endAccessingResources(&self);
        #[method_id(progress)]
        pub unsafe fn progress(&self) -> Id<NSProgress, Shared>;
    }
);
extern_methods!(
    #[doc = "NSBundleResourceRequestAdditions"]
    unsafe impl NSBundle {
        # [method (setPreservationPriority : forTags :)]
        pub unsafe fn setPreservationPriority_forTags(
            &self,
            priority: c_double,
            tags: &NSSet<NSString>,
        );
        # [method (preservationPriorityForTag :)]
        pub unsafe fn preservationPriorityForTag(&self, tag: &NSString) -> c_double;
    }
);
