#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSOrthography;
    unsafe impl ClassType for NSOrthography {
        type Super = NSObject;
    }
);
impl NSOrthography {
    pub unsafe fn initWithDominantScript_languageMap(
        &self,
        script: &NSString,
        map: TodoGenerics,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithDominantScript: script, languageMap: map]
    }
    pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCoder: coder]
    }
    pub unsafe fn dominantScript(&self) -> Id<NSString, Shared> {
        msg_send_id![self, dominantScript]
    }
    pub unsafe fn languageMap(&self) -> TodoGenerics {
        msg_send![self, languageMap]
    }
}
#[doc = "NSOrthographyExtended"]
impl NSOrthography {
    pub unsafe fn languagesForScript(&self, script: &NSString) -> TodoGenerics {
        msg_send![self, languagesForScript: script]
    }
    pub unsafe fn dominantLanguageForScript(
        &self,
        script: &NSString,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, dominantLanguageForScript: script]
    }
    pub unsafe fn defaultOrthographyForLanguage(language: &NSString) -> Id<Self, Shared> {
        msg_send_id![Self::class(), defaultOrthographyForLanguage: language]
    }
    pub unsafe fn dominantLanguage(&self) -> Id<NSString, Shared> {
        msg_send_id![self, dominantLanguage]
    }
    pub unsafe fn allScripts(&self) -> TodoGenerics {
        msg_send![self, allScripts]
    }
    pub unsafe fn allLanguages(&self) -> TodoGenerics {
        msg_send![self, allLanguages]
    }
}
#[doc = "NSOrthographyCreation"]
impl NSOrthography {
    pub unsafe fn orthographyWithDominantScript_languageMap(
        script: &NSString,
        map: TodoGenerics,
    ) -> Id<Self, Shared> {
        msg_send_id![
            Self::class(),
            orthographyWithDominantScript: script,
            languageMap: map
        ]
    }
}
