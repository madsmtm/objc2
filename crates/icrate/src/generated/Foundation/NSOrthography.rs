use super::__exported::NSArray;
use super::__exported::NSDictionary;
use super::__exported::NSString;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSOrthography;
    unsafe impl ClassType for NSOrthography {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSOrthography {
        pub unsafe fn dominantScript(&self) -> Id<NSString, Shared> {
            msg_send_id![self, dominantScript]
        }
        pub unsafe fn languageMap(&self) -> Id<NSDictionary<NSString, NSArray<NSString>>, Shared> {
            msg_send_id![self, languageMap]
        }
        pub unsafe fn initWithDominantScript_languageMap(
            &self,
            script: &NSString,
            map: &NSDictionary<NSString, NSArray<NSString>>,
        ) -> Id<Self, Shared> {
            msg_send_id![self, initWithDominantScript: script, languageMap: map]
        }
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>> {
            msg_send_id![self, initWithCoder: coder]
        }
    }
);
extern_methods!(
    #[doc = "NSOrthographyExtended"]
    unsafe impl NSOrthography {
        pub unsafe fn languagesForScript(
            &self,
            script: &NSString,
        ) -> Option<Id<NSArray<NSString>, Shared>> {
            msg_send_id![self, languagesForScript: script]
        }
        pub unsafe fn dominantLanguageForScript(
            &self,
            script: &NSString,
        ) -> Option<Id<NSString, Shared>> {
            msg_send_id![self, dominantLanguageForScript: script]
        }
        pub unsafe fn dominantLanguage(&self) -> Id<NSString, Shared> {
            msg_send_id![self, dominantLanguage]
        }
        pub unsafe fn allScripts(&self) -> Id<NSArray<NSString>, Shared> {
            msg_send_id![self, allScripts]
        }
        pub unsafe fn allLanguages(&self) -> Id<NSArray<NSString>, Shared> {
            msg_send_id![self, allLanguages]
        }
        pub unsafe fn defaultOrthographyForLanguage(language: &NSString) -> Id<Self, Shared> {
            msg_send_id![Self::class(), defaultOrthographyForLanguage: language]
        }
    }
);
extern_methods!(
    #[doc = "NSOrthographyCreation"]
    unsafe impl NSOrthography {
        pub unsafe fn orthographyWithDominantScript_languageMap(
            script: &NSString,
            map: &NSDictionary<NSString, NSArray<NSString>>,
        ) -> Id<Self, Shared> {
            msg_send_id![
                Self::class(),
                orthographyWithDominantScript: script,
                languageMap: map
            ]
        }
    }
);
