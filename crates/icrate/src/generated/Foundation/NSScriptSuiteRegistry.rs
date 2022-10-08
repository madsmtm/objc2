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
use objc2::{extern_class, extern_methods, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSScriptSuiteRegistry;
    unsafe impl ClassType for NSScriptSuiteRegistry {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSScriptSuiteRegistry {
        pub unsafe fn sharedScriptSuiteRegistry() -> Id<NSScriptSuiteRegistry, Shared> {
            msg_send_id![Self::class(), sharedScriptSuiteRegistry]
        }
        pub unsafe fn setSharedScriptSuiteRegistry(registry: &NSScriptSuiteRegistry) {
            msg_send![Self::class(), setSharedScriptSuiteRegistry: registry]
        }
        pub unsafe fn loadSuitesFromBundle(&self, bundle: &NSBundle) {
            msg_send![self, loadSuitesFromBundle: bundle]
        }
        pub unsafe fn loadSuiteWithDictionary_fromBundle(
            &self,
            suiteDeclaration: &NSDictionary,
            bundle: &NSBundle,
        ) {
            msg_send![
                self,
                loadSuiteWithDictionary: suiteDeclaration,
                fromBundle: bundle
            ]
        }
        pub unsafe fn registerClassDescription(&self, classDescription: &NSScriptClassDescription) {
            msg_send![self, registerClassDescription: classDescription]
        }
        pub unsafe fn registerCommandDescription(
            &self,
            commandDescription: &NSScriptCommandDescription,
        ) {
            msg_send![self, registerCommandDescription: commandDescription]
        }
        pub unsafe fn suiteNames(&self) -> Id<NSArray<NSString>, Shared> {
            msg_send_id![self, suiteNames]
        }
        pub unsafe fn appleEventCodeForSuite(&self, suiteName: &NSString) -> FourCharCode {
            msg_send![self, appleEventCodeForSuite: suiteName]
        }
        pub unsafe fn bundleForSuite(&self, suiteName: &NSString) -> Option<Id<NSBundle, Shared>> {
            msg_send_id![self, bundleForSuite: suiteName]
        }
        pub unsafe fn classDescriptionsInSuite(
            &self,
            suiteName: &NSString,
        ) -> Option<Id<NSDictionary<NSString, NSScriptClassDescription>, Shared>> {
            msg_send_id![self, classDescriptionsInSuite: suiteName]
        }
        pub unsafe fn commandDescriptionsInSuite(
            &self,
            suiteName: &NSString,
        ) -> Option<Id<NSDictionary<NSString, NSScriptCommandDescription>, Shared>> {
            msg_send_id![self, commandDescriptionsInSuite: suiteName]
        }
        pub unsafe fn suiteForAppleEventCode(
            &self,
            appleEventCode: FourCharCode,
        ) -> Option<Id<NSString, Shared>> {
            msg_send_id![self, suiteForAppleEventCode: appleEventCode]
        }
        pub unsafe fn classDescriptionWithAppleEventCode(
            &self,
            appleEventCode: FourCharCode,
        ) -> Option<Id<NSScriptClassDescription, Shared>> {
            msg_send_id![self, classDescriptionWithAppleEventCode: appleEventCode]
        }
        pub unsafe fn commandDescriptionWithAppleEventClass_andAppleEventCode(
            &self,
            appleEventClassCode: FourCharCode,
            appleEventIDCode: FourCharCode,
        ) -> Option<Id<NSScriptCommandDescription, Shared>> {
            msg_send_id![
                self,
                commandDescriptionWithAppleEventClass: appleEventClassCode,
                andAppleEventCode: appleEventIDCode
            ]
        }
        pub unsafe fn aeteResource(&self, languageName: &NSString) -> Option<Id<NSData, Shared>> {
            msg_send_id![self, aeteResource: languageName]
        }
    }
);
