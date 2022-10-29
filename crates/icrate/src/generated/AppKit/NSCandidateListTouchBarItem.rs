#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSCandidateListTouchBarItem<CandidateType: Message>;
    unsafe impl<CandidateType: Message> ClassType for NSCandidateListTouchBarItem<CandidateType> {
        type Super = NSTouchBarItem;
    }
);
extern_methods!(
    unsafe impl<CandidateType: Message> NSCandidateListTouchBarItem<CandidateType> {
        #[method_id(client)]
        pub unsafe fn client(&self) -> Option<Id<TodoProtocols, Shared>>;
        #[method(setClient:)]
        pub unsafe fn setClient(&self, client: Option<&TodoProtocols>);
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSCandidateListTouchBarItemDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSCandidateListTouchBarItemDelegate>);
        #[method(isCollapsed)]
        pub unsafe fn isCollapsed(&self) -> bool;
        #[method(setCollapsed:)]
        pub unsafe fn setCollapsed(&self, collapsed: bool);
        #[method(allowsCollapsing)]
        pub unsafe fn allowsCollapsing(&self) -> bool;
        #[method(setAllowsCollapsing:)]
        pub unsafe fn setAllowsCollapsing(&self, allowsCollapsing: bool);
        #[method(isCandidateListVisible)]
        pub unsafe fn isCandidateListVisible(&self) -> bool;
        #[method(updateWithInsertionPointVisibility:)]
        pub unsafe fn updateWithInsertionPointVisibility(&self, isVisible: bool);
        #[method(allowsTextInputContextCandidates)]
        pub unsafe fn allowsTextInputContextCandidates(&self) -> bool;
        #[method(setAllowsTextInputContextCandidates:)]
        pub unsafe fn setAllowsTextInputContextCandidates(
            &self,
            allowsTextInputContextCandidates: bool,
        );
        #[method(attributedStringForCandidate)]
        pub unsafe fn attributedStringForCandidate(&self) -> TodoBlock;
        #[method(setAttributedStringForCandidate:)]
        pub unsafe fn setAttributedStringForCandidate(
            &self,
            attributedStringForCandidate: TodoBlock,
        );
        #[method_id(candidates)]
        pub unsafe fn candidates(&self) -> Id<NSArray<CandidateType>, Shared>;
        #[method(setCandidates:forSelectedRange:inString:)]
        pub unsafe fn setCandidates_forSelectedRange_inString(
            &self,
            candidates: &NSArray<CandidateType>,
            selectedRange: NSRange,
            originalString: Option<&NSString>,
        );
        #[method_id(customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Id<NSString, Shared>;
        #[method(setCustomizationLabel:)]
        pub unsafe fn setCustomizationLabel(&self, customizationLabel: Option<&NSString>);
    }
);
pub type NSCandidateListTouchBarItemDelegate = NSObject;
extern_methods!(
    #[doc = "NSCandidateListTouchBarItem"]
    unsafe impl NSView {
        #[method_id(candidateListTouchBarItem)]
        pub unsafe fn candidateListTouchBarItem(
            &self,
        ) -> Option<Id<NSCandidateListTouchBarItem, Shared>>;
    }
);
