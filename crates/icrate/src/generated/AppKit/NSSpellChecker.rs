use super::__exported::NSMenu;
use super::__exported::NSOrthography;
use super::__exported::NSPanel;
use super::__exported::NSString;
use super::__exported::NSView;
use super::__exported::NSViewController;
use crate::AppKit::generated::AppKitDefines::*;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSDictionary::*;
use crate::Foundation::generated::NSGeometry::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSRange::*;
use crate::Foundation::generated::NSTextCheckingResult::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSTextCheckingOptionKey = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSSpellChecker;
    unsafe impl ClassType for NSSpellChecker {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSSpellChecker {
        #[method_id(sharedSpellChecker)]
        pub unsafe fn sharedSpellChecker() -> Id<NSSpellChecker, Shared>;
        #[method(sharedSpellCheckerExists)]
        pub unsafe fn sharedSpellCheckerExists() -> bool;
        #[method(uniqueSpellDocumentTag)]
        pub unsafe fn uniqueSpellDocumentTag() -> NSInteger;
        #[method(checkSpellingOfString:startingAt:language:wrap:inSpellDocumentWithTag:wordCount:)]
        pub unsafe fn checkSpellingOfString_startingAt_language_wrap_inSpellDocumentWithTag_wordCount(
            &self,
            stringToCheck: &NSString,
            startingOffset: NSInteger,
            language: Option<&NSString>,
            wrapFlag: bool,
            tag: NSInteger,
            wordCount: *mut NSInteger,
        ) -> NSRange;
        #[method(checkSpellingOfString:startingAt:)]
        pub unsafe fn checkSpellingOfString_startingAt(
            &self,
            stringToCheck: &NSString,
            startingOffset: NSInteger,
        ) -> NSRange;
        #[method(countWordsInString:language:)]
        pub unsafe fn countWordsInString_language(
            &self,
            stringToCount: &NSString,
            language: Option<&NSString>,
        ) -> NSInteger;
        #[method(checkGrammarOfString:startingAt:language:wrap:inSpellDocumentWithTag:details:)]
        pub unsafe fn checkGrammarOfString_startingAt_language_wrap_inSpellDocumentWithTag_details(
            &self,
            stringToCheck: &NSString,
            startingOffset: NSInteger,
            language: Option<&NSString>,
            wrapFlag: bool,
            tag: NSInteger,
            details: Option<&mut Option<Id<NSArray<NSDictionary<NSString, Object>>, Shared>>>,
        ) -> NSRange;
        #[method_id(checkString:range:types:options:inSpellDocumentWithTag:orthography:wordCount:)]
        pub unsafe fn checkString_range_types_options_inSpellDocumentWithTag_orthography_wordCount(
            &self,
            stringToCheck: &NSString,
            range: NSRange,
            checkingTypes: NSTextCheckingTypes,
            options: Option<&NSDictionary<NSTextCheckingOptionKey, Object>>,
            tag: NSInteger,
            orthography: Option<&mut Option<Id<NSOrthography, Shared>>>,
            wordCount: *mut NSInteger,
        ) -> Id<NSArray<NSTextCheckingResult>, Shared>;
        #[method(requestCheckingOfString:range:types:options:inSpellDocumentWithTag:completionHandler:)]
        pub unsafe fn requestCheckingOfString_range_types_options_inSpellDocumentWithTag_completionHandler(
            &self,
            stringToCheck: &NSString,
            range: NSRange,
            checkingTypes: NSTextCheckingTypes,
            options: Option<&NSDictionary<NSTextCheckingOptionKey, Object>>,
            tag: NSInteger,
            completionHandler: TodoBlock,
        ) -> NSInteger;
        #[method(requestCandidatesForSelectedRange:inString:types:options:inSpellDocumentWithTag:completionHandler:)]
        pub unsafe fn requestCandidatesForSelectedRange_inString_types_options_inSpellDocumentWithTag_completionHandler(
            &self,
            selectedRange: NSRange,
            stringToCheck: &NSString,
            checkingTypes: NSTextCheckingTypes,
            options: Option<&NSDictionary<NSTextCheckingOptionKey, Object>>,
            tag: NSInteger,
            completionHandler: TodoBlock,
        ) -> NSInteger;
        #[method_id(menuForResult:string:options:atLocation:inView:)]
        pub unsafe fn menuForResult_string_options_atLocation_inView(
            &self,
            result: &NSTextCheckingResult,
            checkedString: &NSString,
            options: Option<&NSDictionary<NSTextCheckingOptionKey, Object>>,
            location: NSPoint,
            view: &NSView,
        ) -> Option<Id<NSMenu, Shared>>;
        #[method_id(userQuotesArrayForLanguage:)]
        pub unsafe fn userQuotesArrayForLanguage(
            &self,
            language: &NSString,
        ) -> Id<NSArray<NSString>, Shared>;
        #[method_id(userReplacementsDictionary)]
        pub unsafe fn userReplacementsDictionary(
            &self,
        ) -> Id<NSDictionary<NSString, NSString>, Shared>;
        #[method(updateSpellingPanelWithMisspelledWord:)]
        pub unsafe fn updateSpellingPanelWithMisspelledWord(&self, word: &NSString);
        #[method(updateSpellingPanelWithGrammarString:detail:)]
        pub unsafe fn updateSpellingPanelWithGrammarString_detail(
            &self,
            string: &NSString,
            detail: &NSDictionary<NSString, Object>,
        );
        #[method_id(spellingPanel)]
        pub unsafe fn spellingPanel(&self) -> Id<NSPanel, Shared>;
        #[method_id(accessoryView)]
        pub unsafe fn accessoryView(&self) -> Option<Id<NSView, Shared>>;
        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, accessoryView: Option<&NSView>);
        #[method_id(substitutionsPanel)]
        pub unsafe fn substitutionsPanel(&self) -> Id<NSPanel, Shared>;
        #[method_id(substitutionsPanelAccessoryViewController)]
        pub unsafe fn substitutionsPanelAccessoryViewController(
            &self,
        ) -> Option<Id<NSViewController, Shared>>;
        #[method(setSubstitutionsPanelAccessoryViewController:)]
        pub unsafe fn setSubstitutionsPanelAccessoryViewController(
            &self,
            substitutionsPanelAccessoryViewController: Option<&NSViewController>,
        );
        #[method(updatePanels)]
        pub unsafe fn updatePanels(&self);
        #[method(ignoreWord:inSpellDocumentWithTag:)]
        pub unsafe fn ignoreWord_inSpellDocumentWithTag(
            &self,
            wordToIgnore: &NSString,
            tag: NSInteger,
        );
        #[method_id(ignoredWordsInSpellDocumentWithTag:)]
        pub unsafe fn ignoredWordsInSpellDocumentWithTag(
            &self,
            tag: NSInteger,
        ) -> Option<Id<NSArray<NSString>, Shared>>;
        #[method(setIgnoredWords:inSpellDocumentWithTag:)]
        pub unsafe fn setIgnoredWords_inSpellDocumentWithTag(
            &self,
            words: &NSArray<NSString>,
            tag: NSInteger,
        );
        #[method_id(guessesForWordRange:inString:language:inSpellDocumentWithTag:)]
        pub unsafe fn guessesForWordRange_inString_language_inSpellDocumentWithTag(
            &self,
            range: NSRange,
            string: &NSString,
            language: Option<&NSString>,
            tag: NSInteger,
        ) -> Option<Id<NSArray<NSString>, Shared>>;
        #[method_id(correctionForWordRange:inString:language:inSpellDocumentWithTag:)]
        pub unsafe fn correctionForWordRange_inString_language_inSpellDocumentWithTag(
            &self,
            range: NSRange,
            string: &NSString,
            language: &NSString,
            tag: NSInteger,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(completionsForPartialWordRange:inString:language:inSpellDocumentWithTag:)]
        pub unsafe fn completionsForPartialWordRange_inString_language_inSpellDocumentWithTag(
            &self,
            range: NSRange,
            string: &NSString,
            language: Option<&NSString>,
            tag: NSInteger,
        ) -> Option<Id<NSArray<NSString>, Shared>>;
        #[method_id(languageForWordRange:inString:orthography:)]
        pub unsafe fn languageForWordRange_inString_orthography(
            &self,
            range: NSRange,
            string: &NSString,
            orthography: Option<&NSOrthography>,
        ) -> Option<Id<NSString, Shared>>;
        #[method(closeSpellDocumentWithTag:)]
        pub unsafe fn closeSpellDocumentWithTag(&self, tag: NSInteger);
        #[method(recordResponse:toCorrection:forWord:language:inSpellDocumentWithTag:)]
        pub unsafe fn recordResponse_toCorrection_forWord_language_inSpellDocumentWithTag(
            &self,
            response: NSCorrectionResponse,
            correction: &NSString,
            word: &NSString,
            language: Option<&NSString>,
            tag: NSInteger,
        );
        #[method(showCorrectionIndicatorOfType:primaryString:alternativeStrings:forStringInRect:view:completionHandler:)]
        pub unsafe fn showCorrectionIndicatorOfType_primaryString_alternativeStrings_forStringInRect_view_completionHandler(
            &self,
            type_: NSCorrectionIndicatorType,
            primaryString: &NSString,
            alternativeStrings: &NSArray<NSString>,
            rectOfTypedString: NSRect,
            view: &NSView,
            completionBlock: TodoBlock,
        );
        #[method(dismissCorrectionIndicatorForView:)]
        pub unsafe fn dismissCorrectionIndicatorForView(&self, view: &NSView);
        #[method(preventsAutocorrectionBeforeString:language:)]
        pub unsafe fn preventsAutocorrectionBeforeString_language(
            &self,
            string: &NSString,
            language: Option<&NSString>,
        ) -> bool;
        #[method(deletesAutospaceBetweenString:andString:language:)]
        pub unsafe fn deletesAutospaceBetweenString_andString_language(
            &self,
            precedingString: &NSString,
            followingString: &NSString,
            language: Option<&NSString>,
        ) -> bool;
        #[method_id(availableLanguages)]
        pub unsafe fn availableLanguages(&self) -> Id<NSArray<NSString>, Shared>;
        #[method_id(userPreferredLanguages)]
        pub unsafe fn userPreferredLanguages(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(automaticallyIdentifiesLanguages)]
        pub unsafe fn automaticallyIdentifiesLanguages(&self) -> bool;
        #[method(setAutomaticallyIdentifiesLanguages:)]
        pub unsafe fn setAutomaticallyIdentifiesLanguages(
            &self,
            automaticallyIdentifiesLanguages: bool,
        );
        #[method(setWordFieldStringValue:)]
        pub unsafe fn setWordFieldStringValue(&self, string: &NSString);
        #[method(learnWord:)]
        pub unsafe fn learnWord(&self, word: &NSString);
        #[method(hasLearnedWord:)]
        pub unsafe fn hasLearnedWord(&self, word: &NSString) -> bool;
        #[method(unlearnWord:)]
        pub unsafe fn unlearnWord(&self, word: &NSString);
        #[method(isAutomaticTextReplacementEnabled)]
        pub unsafe fn isAutomaticTextReplacementEnabled() -> bool;
        #[method(isAutomaticSpellingCorrectionEnabled)]
        pub unsafe fn isAutomaticSpellingCorrectionEnabled() -> bool;
        #[method(isAutomaticQuoteSubstitutionEnabled)]
        pub unsafe fn isAutomaticQuoteSubstitutionEnabled() -> bool;
        #[method(isAutomaticDashSubstitutionEnabled)]
        pub unsafe fn isAutomaticDashSubstitutionEnabled() -> bool;
        #[method(isAutomaticCapitalizationEnabled)]
        pub unsafe fn isAutomaticCapitalizationEnabled() -> bool;
        #[method(isAutomaticPeriodSubstitutionEnabled)]
        pub unsafe fn isAutomaticPeriodSubstitutionEnabled() -> bool;
        #[method(isAutomaticTextCompletionEnabled)]
        pub unsafe fn isAutomaticTextCompletionEnabled() -> bool;
        #[method_id(language)]
        pub unsafe fn language(&self) -> Id<NSString, Shared>;
        #[method(setLanguage:)]
        pub unsafe fn setLanguage(&self, language: &NSString) -> bool;
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSSpellChecker {
        #[method_id(guessesForWord:)]
        pub unsafe fn guessesForWord(&self, word: Option<&NSString>)
            -> Option<Id<NSArray, Shared>>;
        #[method(forgetWord:)]
        pub unsafe fn forgetWord(&self, word: Option<&NSString>);
    }
);
