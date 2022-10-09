use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSError::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSString::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSMorphology;
    unsafe impl ClassType for NSMorphology {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSMorphology {
        #[method(grammaticalGender)]
        pub unsafe fn grammaticalGender(&self) -> NSGrammaticalGender;
        #[method(setGrammaticalGender:)]
        pub unsafe fn setGrammaticalGender(&self, grammaticalGender: NSGrammaticalGender);
        #[method(partOfSpeech)]
        pub unsafe fn partOfSpeech(&self) -> NSGrammaticalPartOfSpeech;
        #[method(setPartOfSpeech:)]
        pub unsafe fn setPartOfSpeech(&self, partOfSpeech: NSGrammaticalPartOfSpeech);
        #[method(number)]
        pub unsafe fn number(&self) -> NSGrammaticalNumber;
        #[method(setNumber:)]
        pub unsafe fn setNumber(&self, number: NSGrammaticalNumber);
    }
);
extern_methods!(
    #[doc = "NSCustomPronouns"]
    unsafe impl NSMorphology {
        #[method_id(customPronounForLanguage:)]
        pub unsafe fn customPronounForLanguage(
            &self,
            language: &NSString,
        ) -> Option<Id<NSMorphologyCustomPronoun, Shared>>;
        #[method(setCustomPronoun:forLanguage:error:)]
        pub unsafe fn setCustomPronoun_forLanguage_error(
            &self,
            features: Option<&NSMorphologyCustomPronoun>,
            language: &NSString,
        ) -> Result<(), Id<NSError, Shared>>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSMorphologyCustomPronoun;
    unsafe impl ClassType for NSMorphologyCustomPronoun {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSMorphologyCustomPronoun {
        #[method(isSupportedForLanguage:)]
        pub unsafe fn isSupportedForLanguage(language: &NSString) -> bool;
        #[method_id(requiredKeysForLanguage:)]
        pub unsafe fn requiredKeysForLanguage(language: &NSString)
            -> Id<NSArray<NSString>, Shared>;
        #[method_id(subjectForm)]
        pub unsafe fn subjectForm(&self) -> Option<Id<NSString, Shared>>;
        #[method(setSubjectForm:)]
        pub unsafe fn setSubjectForm(&self, subjectForm: Option<&NSString>);
        #[method_id(objectForm)]
        pub unsafe fn objectForm(&self) -> Option<Id<NSString, Shared>>;
        #[method(setObjectForm:)]
        pub unsafe fn setObjectForm(&self, objectForm: Option<&NSString>);
        #[method_id(possessiveForm)]
        pub unsafe fn possessiveForm(&self) -> Option<Id<NSString, Shared>>;
        #[method(setPossessiveForm:)]
        pub unsafe fn setPossessiveForm(&self, possessiveForm: Option<&NSString>);
        #[method_id(possessiveAdjectiveForm)]
        pub unsafe fn possessiveAdjectiveForm(&self) -> Option<Id<NSString, Shared>>;
        #[method(setPossessiveAdjectiveForm:)]
        pub unsafe fn setPossessiveAdjectiveForm(&self, possessiveAdjectiveForm: Option<&NSString>);
        #[method_id(reflexiveForm)]
        pub unsafe fn reflexiveForm(&self) -> Option<Id<NSString, Shared>>;
        #[method(setReflexiveForm:)]
        pub unsafe fn setReflexiveForm(&self, reflexiveForm: Option<&NSString>);
    }
);
extern_methods!(
    #[doc = "NSMorphologyUserSettings"]
    unsafe impl NSMorphology {
        #[method(isUnspecified)]
        pub unsafe fn isUnspecified(&self) -> bool;
        #[method_id(userMorphology)]
        pub unsafe fn userMorphology() -> Id<NSMorphology, Shared>;
    }
);
