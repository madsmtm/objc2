use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSSpellChecker::*;
use crate::AppKit::generated::NSTextCheckingClient::*;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSAttributedString::*;
use crate::Foundation::generated::NSDictionary::*;
use crate::Foundation::generated::NSGeometry::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSRange::*;
use crate::Foundation::generated::NSString::*;
use crate::Foundation::generated::NSTextCheckingResult::*;
use crate::Foundation::generated::NSValue::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTextCheckingController;
    unsafe impl ClassType for NSTextCheckingController {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSTextCheckingController {
        #[method_id(initWithClient:)]
        pub unsafe fn initWithClient(&self, client: &NSTextCheckingClient) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(client)]
        pub unsafe fn client(&self) -> Id<NSTextCheckingClient, Shared>;
        #[method(invalidate)]
        pub unsafe fn invalidate(&self);
        #[method(didChangeTextInRange:)]
        pub unsafe fn didChangeTextInRange(&self, range: NSRange);
        #[method(insertedTextInRange:)]
        pub unsafe fn insertedTextInRange(&self, range: NSRange);
        #[method(didChangeSelectedRange)]
        pub unsafe fn didChangeSelectedRange(&self);
        #[method(considerTextCheckingForRange:)]
        pub unsafe fn considerTextCheckingForRange(&self, range: NSRange);
        #[method(checkTextInRange:types:options:)]
        pub unsafe fn checkTextInRange_types_options(
            &self,
            range: NSRange,
            checkingTypes: NSTextCheckingTypes,
            options: &NSDictionary<NSTextCheckingOptionKey, Object>,
        );
        #[method(checkTextInSelection:)]
        pub unsafe fn checkTextInSelection(&self, sender: Option<&Object>);
        #[method(checkTextInDocument:)]
        pub unsafe fn checkTextInDocument(&self, sender: Option<&Object>);
        #[method(orderFrontSubstitutionsPanel:)]
        pub unsafe fn orderFrontSubstitutionsPanel(&self, sender: Option<&Object>);
        #[method(checkSpelling:)]
        pub unsafe fn checkSpelling(&self, sender: Option<&Object>);
        #[method(showGuessPanel:)]
        pub unsafe fn showGuessPanel(&self, sender: Option<&Object>);
        #[method(changeSpelling:)]
        pub unsafe fn changeSpelling(&self, sender: Option<&Object>);
        #[method(ignoreSpelling:)]
        pub unsafe fn ignoreSpelling(&self, sender: Option<&Object>);
        #[method(updateCandidates)]
        pub unsafe fn updateCandidates(&self);
        #[method_id(validAnnotations)]
        pub unsafe fn validAnnotations(&self) -> Id<NSArray<NSAttributedStringKey>, Shared>;
        #[method_id(menuAtIndex:clickedOnSelection:effectiveRange:)]
        pub unsafe fn menuAtIndex_clickedOnSelection_effectiveRange(
            &self,
            location: NSUInteger,
            clickedOnSelection: bool,
            effectiveRange: NSRangePointer,
        ) -> Option<Id<NSMenu, Shared>>;
        #[method(spellCheckerDocumentTag)]
        pub unsafe fn spellCheckerDocumentTag(&self) -> NSInteger;
        #[method(setSpellCheckerDocumentTag:)]
        pub unsafe fn setSpellCheckerDocumentTag(&self, spellCheckerDocumentTag: NSInteger);
    }
);
