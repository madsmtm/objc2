#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    struct NSMorphology;
    unsafe impl ClassType for NSMorphology {
        type Super = NSObject;
    }
);
impl NSMorphology {
    pub unsafe fn grammaticalGender(&self) -> NSGrammaticalGender {
        msg_send![self, grammaticalGender]
    }
    pub unsafe fn setGrammaticalGender(&self, grammaticalGender: NSGrammaticalGender) {
        msg_send![self, setGrammaticalGender: grammaticalGender]
    }
    pub unsafe fn partOfSpeech(&self) -> NSGrammaticalPartOfSpeech {
        msg_send![self, partOfSpeech]
    }
    pub unsafe fn setPartOfSpeech(&self, partOfSpeech: NSGrammaticalPartOfSpeech) {
        msg_send![self, setPartOfSpeech: partOfSpeech]
    }
    pub unsafe fn number(&self) -> NSGrammaticalNumber {
        msg_send![self, number]
    }
    pub unsafe fn setNumber(&self, number: NSGrammaticalNumber) {
        msg_send![self, setNumber: number]
    }
}
#[doc = "NSCustomPronouns"]
impl NSMorphology {
    pub unsafe fn customPronounForLanguage(
        &self,
        language: &NSString,
    ) -> Option<Id<NSMorphologyCustomPronoun, Shared>> {
        msg_send_id![self, customPronounForLanguage: language]
    }
    pub unsafe fn setCustomPronoun_forLanguage_error(
        &self,
        features: Option<&NSMorphologyCustomPronoun>,
        language: &NSString,
        error: *mut Option<&NSError>,
    ) -> bool {
        msg_send![
            self,
            setCustomPronoun: features,
            forLanguage: language,
            error: error
        ]
    }
}
extern_class!(
    #[derive(Debug)]
    struct NSMorphologyCustomPronoun;
    unsafe impl ClassType for NSMorphologyCustomPronoun {
        type Super = NSObject;
    }
);
impl NSMorphologyCustomPronoun {
    pub unsafe fn isSupportedForLanguage(language: &NSString) -> bool {
        msg_send![Self::class(), isSupportedForLanguage: language]
    }
    pub unsafe fn requiredKeysForLanguage(language: &NSString) -> TodoGenerics {
        msg_send![Self::class(), requiredKeysForLanguage: language]
    }
    pub unsafe fn subjectForm(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, subjectForm]
    }
    pub unsafe fn setSubjectForm(&self, subjectForm: Option<&NSString>) {
        msg_send![self, setSubjectForm: subjectForm]
    }
    pub unsafe fn objectForm(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, objectForm]
    }
    pub unsafe fn setObjectForm(&self, objectForm: Option<&NSString>) {
        msg_send![self, setObjectForm: objectForm]
    }
    pub unsafe fn possessiveForm(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, possessiveForm]
    }
    pub unsafe fn setPossessiveForm(&self, possessiveForm: Option<&NSString>) {
        msg_send![self, setPossessiveForm: possessiveForm]
    }
    pub unsafe fn possessiveAdjectiveForm(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, possessiveAdjectiveForm]
    }
    pub unsafe fn setPossessiveAdjectiveForm(&self, possessiveAdjectiveForm: Option<&NSString>) {
        msg_send![self, setPossessiveAdjectiveForm: possessiveAdjectiveForm]
    }
    pub unsafe fn reflexiveForm(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, reflexiveForm]
    }
    pub unsafe fn setReflexiveForm(&self, reflexiveForm: Option<&NSString>) {
        msg_send![self, setReflexiveForm: reflexiveForm]
    }
}
#[doc = "NSMorphologyUserSettings"]
impl NSMorphology {
    pub unsafe fn isUnspecified(&self) -> bool {
        msg_send![self, isUnspecified]
    }
    pub unsafe fn userMorphology() -> Id<NSMorphology, Shared> {
        msg_send_id![Self::class(), userMorphology]
    }
}
