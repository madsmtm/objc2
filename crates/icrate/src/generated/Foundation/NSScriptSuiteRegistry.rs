use super::__exported::NSArray;
use super::__exported::NSBundle;
use super::__exported::NSData;
use super::__exported::NSDictionary;
use super::__exported::NSMutableArray;
use super::__exported::NSMutableDictionary;
use super::__exported::NSMutableSet;
use super::__exported::NSScriptClassDescription;
use super::__exported::NSScriptCommandDescription;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSScriptSuiteRegistry;
    unsafe impl ClassType for NSScriptSuiteRegistry {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSScriptSuiteRegistry {
        #[method_id(sharedScriptSuiteRegistry)]
        pub unsafe fn sharedScriptSuiteRegistry() -> Id<NSScriptSuiteRegistry, Shared>;
        #[method(setSharedScriptSuiteRegistry:)]
        pub unsafe fn setSharedScriptSuiteRegistry(registry: &NSScriptSuiteRegistry);
        #[method(loadSuitesFromBundle:)]
        pub unsafe fn loadSuitesFromBundle(&self, bundle: &NSBundle);
        #[method(loadSuiteWithDictionary:fromBundle:)]
        pub unsafe fn loadSuiteWithDictionary_fromBundle(
            &self,
            suiteDeclaration: &NSDictionary,
            bundle: &NSBundle,
        );
        #[method(registerClassDescription:)]
        pub unsafe fn registerClassDescription(&self, classDescription: &NSScriptClassDescription);
        #[method(registerCommandDescription:)]
        pub unsafe fn registerCommandDescription(
            &self,
            commandDescription: &NSScriptCommandDescription,
        );
        #[method_id(suiteNames)]
        pub unsafe fn suiteNames(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(appleEventCodeForSuite:)]
        pub unsafe fn appleEventCodeForSuite(&self, suiteName: &NSString) -> FourCharCode;
        #[method_id(bundleForSuite:)]
        pub unsafe fn bundleForSuite(&self, suiteName: &NSString) -> Option<Id<NSBundle, Shared>>;
        #[method_id(classDescriptionsInSuite:)]
        pub unsafe fn classDescriptionsInSuite(
            &self,
            suiteName: &NSString,
        ) -> Option<Id<NSDictionary<NSString, NSScriptClassDescription>, Shared>>;
        #[method_id(commandDescriptionsInSuite:)]
        pub unsafe fn commandDescriptionsInSuite(
            &self,
            suiteName: &NSString,
        ) -> Option<Id<NSDictionary<NSString, NSScriptCommandDescription>, Shared>>;
        #[method_id(suiteForAppleEventCode:)]
        pub unsafe fn suiteForAppleEventCode(
            &self,
            appleEventCode: FourCharCode,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(classDescriptionWithAppleEventCode:)]
        pub unsafe fn classDescriptionWithAppleEventCode(
            &self,
            appleEventCode: FourCharCode,
        ) -> Option<Id<NSScriptClassDescription, Shared>>;
        #[method_id(commandDescriptionWithAppleEventClass:andAppleEventCode:)]
        pub unsafe fn commandDescriptionWithAppleEventClass_andAppleEventCode(
            &self,
            appleEventClassCode: FourCharCode,
            appleEventIDCode: FourCharCode,
        ) -> Option<Id<NSScriptCommandDescription, Shared>>;
        #[method_id(aeteResource:)]
        pub unsafe fn aeteResource(&self, languageName: &NSString) -> Option<Id<NSData, Shared>>;
    }
);
