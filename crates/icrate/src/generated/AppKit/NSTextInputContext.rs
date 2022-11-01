//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

pub type NSTextInputSourceIdentifier = NSString;

extern_class!(
    #[derive(Debug)]
    pub struct NSTextInputContext;

    unsafe impl ClassType for NSTextInputContext {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSTextInputContext {
        #[method_id(@__retain_semantics Other currentInputContext)]
        pub unsafe fn currentInputContext() -> Option<Id<NSTextInputContext, Shared>>;

        #[method_id(@__retain_semantics Init initWithClient:)]
        pub unsafe fn initWithClient(
            this: Option<Allocated<Self>>,
            client: &NSTextInputClient,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other client)]
        pub unsafe fn client(&self) -> Id<NSTextInputClient, Shared>;

        #[method(acceptsGlyphInfo)]
        pub unsafe fn acceptsGlyphInfo(&self) -> bool;

        #[method(setAcceptsGlyphInfo:)]
        pub unsafe fn setAcceptsGlyphInfo(&self, acceptsGlyphInfo: bool);

        #[method_id(@__retain_semantics Other allowedInputSourceLocales)]
        pub unsafe fn allowedInputSourceLocales(&self) -> Option<Id<NSArray<NSString>, Shared>>;

        #[method(setAllowedInputSourceLocales:)]
        pub unsafe fn setAllowedInputSourceLocales(
            &self,
            allowedInputSourceLocales: Option<&NSArray<NSString>>,
        );

        #[method(activate)]
        pub unsafe fn activate(&self);

        #[method(deactivate)]
        pub unsafe fn deactivate(&self);

        #[method(handleEvent:)]
        pub unsafe fn handleEvent(&self, event: &NSEvent) -> bool;

        #[method(discardMarkedText)]
        pub unsafe fn discardMarkedText(&self);

        #[method(invalidateCharacterCoordinates)]
        pub unsafe fn invalidateCharacterCoordinates(&self);

        #[method_id(@__retain_semantics Other keyboardInputSources)]
        pub unsafe fn keyboardInputSources(
            &self,
        ) -> Option<Id<NSArray<NSTextInputSourceIdentifier>, Shared>>;

        #[method_id(@__retain_semantics Other selectedKeyboardInputSource)]
        pub unsafe fn selectedKeyboardInputSource(
            &self,
        ) -> Option<Id<NSTextInputSourceIdentifier, Shared>>;

        #[method(setSelectedKeyboardInputSource:)]
        pub unsafe fn setSelectedKeyboardInputSource(
            &self,
            selectedKeyboardInputSource: Option<&NSTextInputSourceIdentifier>,
        );

        #[method_id(@__retain_semantics Other localizedNameForInputSource:)]
        pub unsafe fn localizedNameForInputSource(
            inputSourceIdentifier: &NSTextInputSourceIdentifier,
        ) -> Option<Id<NSString, Shared>>;
    }
);

extern "C" {
    pub static NSTextInputContextKeyboardSelectionDidChangeNotification:
        &'static NSNotificationName;
}
