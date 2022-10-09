use super::__exported::NSError;
use super::__exported::NSString;
use super::__exported::NSURL;
use crate::AppKit::generated::AppKitDefines::*;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSDictionary::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSRange::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSSpeechSynthesizerVoiceName = NSString;
pub type NSVoiceAttributeKey = NSString;
pub type NSSpeechDictionaryKey = NSString;
pub type NSVoiceGenderName = NSString;
pub type NSSpeechPropertyKey = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSSpeechSynthesizer;
    unsafe impl ClassType for NSSpeechSynthesizer {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSSpeechSynthesizer {
        #[method_id(initWithVoice:)]
        pub unsafe fn initWithVoice(
            &self,
            voice: Option<&NSSpeechSynthesizerVoiceName>,
        ) -> Option<Id<Self, Shared>>;
        #[method(startSpeakingString:)]
        pub unsafe fn startSpeakingString(&self, string: &NSString) -> bool;
        #[method(startSpeakingString:toURL:)]
        pub unsafe fn startSpeakingString_toURL(&self, string: &NSString, url: &NSURL) -> bool;
        #[method(isSpeaking)]
        pub unsafe fn isSpeaking(&self) -> bool;
        #[method(stopSpeaking)]
        pub unsafe fn stopSpeaking(&self);
        #[method(stopSpeakingAtBoundary:)]
        pub unsafe fn stopSpeakingAtBoundary(&self, boundary: NSSpeechBoundary);
        #[method(pauseSpeakingAtBoundary:)]
        pub unsafe fn pauseSpeakingAtBoundary(&self, boundary: NSSpeechBoundary);
        #[method(continueSpeaking)]
        pub unsafe fn continueSpeaking(&self);
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSSpeechSynthesizerDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSSpeechSynthesizerDelegate>);
        #[method_id(voice)]
        pub unsafe fn voice(&self) -> Option<Id<NSSpeechSynthesizerVoiceName, Shared>>;
        #[method(setVoice:)]
        pub unsafe fn setVoice(&self, voice: Option<&NSSpeechSynthesizerVoiceName>) -> bool;
        #[method(rate)]
        pub unsafe fn rate(&self) -> c_float;
        #[method(setRate:)]
        pub unsafe fn setRate(&self, rate: c_float);
        #[method(volume)]
        pub unsafe fn volume(&self) -> c_float;
        #[method(setVolume:)]
        pub unsafe fn setVolume(&self, volume: c_float);
        #[method(usesFeedbackWindow)]
        pub unsafe fn usesFeedbackWindow(&self) -> bool;
        #[method(setUsesFeedbackWindow:)]
        pub unsafe fn setUsesFeedbackWindow(&self, usesFeedbackWindow: bool);
        #[method(addSpeechDictionary:)]
        pub unsafe fn addSpeechDictionary(
            &self,
            speechDictionary: &NSDictionary<NSSpeechDictionaryKey, Object>,
        );
        #[method_id(phonemesFromText:)]
        pub unsafe fn phonemesFromText(&self, text: &NSString) -> Id<NSString, Shared>;
        #[method_id(objectForProperty:error:)]
        pub unsafe fn objectForProperty_error(
            &self,
            property: &NSSpeechPropertyKey,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;
        #[method(setObject:forProperty:error:)]
        pub unsafe fn setObject_forProperty_error(
            &self,
            object: Option<&Object>,
            property: &NSSpeechPropertyKey,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method(isAnyApplicationSpeaking)]
        pub unsafe fn isAnyApplicationSpeaking() -> bool;
        #[method_id(defaultVoice)]
        pub unsafe fn defaultVoice() -> Id<NSSpeechSynthesizerVoiceName, Shared>;
        #[method_id(availableVoices)]
        pub unsafe fn availableVoices() -> Id<NSArray<NSSpeechSynthesizerVoiceName>, Shared>;
        #[method_id(attributesForVoice:)]
        pub unsafe fn attributesForVoice(
            voice: &NSSpeechSynthesizerVoiceName,
        ) -> Id<NSDictionary<NSVoiceAttributeKey, Object>, Shared>;
    }
);
pub type NSSpeechSynthesizerDelegate = NSObject;
pub type NSSpeechMode = NSString;
pub type NSSpeechStatusKey = NSString;
pub type NSSpeechErrorKey = NSString;
pub type NSSpeechSynthesizerInfoKey = NSString;
pub type NSSpeechPhonemeInfoKey = NSString;
pub type NSSpeechCommandDelimiterKey = NSString;
