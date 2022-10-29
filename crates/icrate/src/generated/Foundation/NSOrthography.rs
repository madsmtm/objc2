#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSOrthography;
    unsafe impl ClassType for NSOrthography {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSOrthography {
        #[method_id(dominantScript)]
        pub unsafe fn dominantScript(&self) -> Id<NSString, Shared>;
        #[method_id(languageMap)]
        pub unsafe fn languageMap(&self) -> Id<NSDictionary<NSString, NSArray<NSString>>, Shared>;
        #[method_id(initWithDominantScript:languageMap:)]
        pub unsafe fn initWithDominantScript_languageMap(
            &self,
            script: &NSString,
            map: &NSDictionary<NSString, NSArray<NSString>>,
        ) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSOrthographyExtended"]
    unsafe impl NSOrthography {
        #[method_id(languagesForScript:)]
        pub unsafe fn languagesForScript(
            &self,
            script: &NSString,
        ) -> Option<Id<NSArray<NSString>, Shared>>;
        #[method_id(dominantLanguageForScript:)]
        pub unsafe fn dominantLanguageForScript(
            &self,
            script: &NSString,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(dominantLanguage)]
        pub unsafe fn dominantLanguage(&self) -> Id<NSString, Shared>;
        #[method_id(allScripts)]
        pub unsafe fn allScripts(&self) -> Id<NSArray<NSString>, Shared>;
        #[method_id(allLanguages)]
        pub unsafe fn allLanguages(&self) -> Id<NSArray<NSString>, Shared>;
        #[method_id(defaultOrthographyForLanguage:)]
        pub unsafe fn defaultOrthographyForLanguage(language: &NSString) -> Id<Self, Shared>;
    }
);
extern_methods!(
    #[doc = "NSOrthographyCreation"]
    unsafe impl NSOrthography {
        #[method_id(orthographyWithDominantScript:languageMap:)]
        pub unsafe fn orthographyWithDominantScript_languageMap(
            script: &NSString,
            map: &NSDictionary<NSString, NSArray<NSString>>,
        ) -> Id<Self, Shared>;
    }
);
