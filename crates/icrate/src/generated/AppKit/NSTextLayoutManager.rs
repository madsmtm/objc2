use super::__exported::NSTextContainer;
use super::__exported::NSTextContentManager;
use super::__exported::NSTextElement;
use super::__exported::NSTextLocation;
use super::__exported::NSTextRange;
use super::__exported::NSTextSelection;
use super::__exported::NSTextSelectionDataSource;
use super::__exported::NSTextSelectionNavigation;
use super::__exported::NSTextViewportLayoutController;
use crate::AppKit::generated::NSTextLayoutFragment::*;
use crate::CoreGraphics::generated::CGGeometry::*;
use crate::Foundation::generated::NSAttributedString::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTextLayoutManager;
    unsafe impl ClassType for NSTextLayoutManager {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSTextLayoutManager {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSTextLayoutManagerDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSTextLayoutManagerDelegate>);
        #[method(usesFontLeading)]
        pub unsafe fn usesFontLeading(&self) -> bool;
        #[method(setUsesFontLeading:)]
        pub unsafe fn setUsesFontLeading(&self, usesFontLeading: bool);
        #[method(limitsLayoutForSuspiciousContents)]
        pub unsafe fn limitsLayoutForSuspiciousContents(&self) -> bool;
        #[method(setLimitsLayoutForSuspiciousContents:)]
        pub unsafe fn setLimitsLayoutForSuspiciousContents(
            &self,
            limitsLayoutForSuspiciousContents: bool,
        );
        #[method(usesHyphenation)]
        pub unsafe fn usesHyphenation(&self) -> bool;
        #[method(setUsesHyphenation:)]
        pub unsafe fn setUsesHyphenation(&self, usesHyphenation: bool);
        #[method_id(textContentManager)]
        pub unsafe fn textContentManager(&self) -> Option<Id<NSTextContentManager, Shared>>;
        #[method(replaceTextContentManager:)]
        pub unsafe fn replaceTextContentManager(&self, textContentManager: &NSTextContentManager);
        #[method_id(textContainer)]
        pub unsafe fn textContainer(&self) -> Option<Id<NSTextContainer, Shared>>;
        #[method(setTextContainer:)]
        pub unsafe fn setTextContainer(&self, textContainer: Option<&NSTextContainer>);
        #[method(usageBoundsForTextContainer)]
        pub unsafe fn usageBoundsForTextContainer(&self) -> CGRect;
        #[method_id(textViewportLayoutController)]
        pub unsafe fn textViewportLayoutController(
            &self,
        ) -> Id<NSTextViewportLayoutController, Shared>;
        #[method_id(layoutQueue)]
        pub unsafe fn layoutQueue(&self) -> Option<Id<NSOperationQueue, Shared>>;
        #[method(setLayoutQueue:)]
        pub unsafe fn setLayoutQueue(&self, layoutQueue: Option<&NSOperationQueue>);
        #[method(ensureLayoutForRange:)]
        pub unsafe fn ensureLayoutForRange(&self, range: &NSTextRange);
        #[method(ensureLayoutForBounds:)]
        pub unsafe fn ensureLayoutForBounds(&self, bounds: CGRect);
        #[method(invalidateLayoutForRange:)]
        pub unsafe fn invalidateLayoutForRange(&self, range: &NSTextRange);
        #[method_id(textLayoutFragmentForPosition:)]
        pub unsafe fn textLayoutFragmentForPosition(
            &self,
            position: CGPoint,
        ) -> Option<Id<NSTextLayoutFragment, Shared>>;
        #[method_id(textLayoutFragmentForLocation:)]
        pub unsafe fn textLayoutFragmentForLocation(
            &self,
            location: &NSTextLocation,
        ) -> Option<Id<NSTextLayoutFragment, Shared>>;
        #[method_id(enumerateTextLayoutFragmentsFromLocation:options:usingBlock:)]
        pub unsafe fn enumerateTextLayoutFragmentsFromLocation_options_usingBlock(
            &self,
            location: Option<&NSTextLocation>,
            options: NSTextLayoutFragmentEnumerationOptions,
            block: TodoBlock,
        ) -> Option<Id<NSTextLocation, Shared>>;
        #[method_id(textSelections)]
        pub unsafe fn textSelections(&self) -> Id<NSArray<NSTextSelection>, Shared>;
        #[method(setTextSelections:)]
        pub unsafe fn setTextSelections(&self, textSelections: &NSArray<NSTextSelection>);
        #[method_id(textSelectionNavigation)]
        pub unsafe fn textSelectionNavigation(&self) -> Id<NSTextSelectionNavigation, Shared>;
        #[method(setTextSelectionNavigation:)]
        pub unsafe fn setTextSelectionNavigation(
            &self,
            textSelectionNavigation: &NSTextSelectionNavigation,
        );
        #[method(enumerateRenderingAttributesFromLocation:reverse:usingBlock:)]
        pub unsafe fn enumerateRenderingAttributesFromLocation_reverse_usingBlock(
            &self,
            location: &NSTextLocation,
            reverse: bool,
            block: TodoBlock,
        );
        #[method(setRenderingAttributes:forTextRange:)]
        pub unsafe fn setRenderingAttributes_forTextRange(
            &self,
            renderingAttributes: &NSDictionary<NSAttributedStringKey, Object>,
            textRange: &NSTextRange,
        );
        #[method(addRenderingAttribute:value:forTextRange:)]
        pub unsafe fn addRenderingAttribute_value_forTextRange(
            &self,
            renderingAttribute: &NSAttributedStringKey,
            value: Option<&Object>,
            textRange: &NSTextRange,
        );
        #[method(removeRenderingAttribute:forTextRange:)]
        pub unsafe fn removeRenderingAttribute_forTextRange(
            &self,
            renderingAttribute: &NSAttributedStringKey,
            textRange: &NSTextRange,
        );
        #[method(invalidateRenderingAttributesForTextRange:)]
        pub unsafe fn invalidateRenderingAttributesForTextRange(&self, textRange: &NSTextRange);
        #[method(renderingAttributesValidator)]
        pub unsafe fn renderingAttributesValidator(&self) -> TodoBlock;
        #[method(setRenderingAttributesValidator:)]
        pub unsafe fn setRenderingAttributesValidator(
            &self,
            renderingAttributesValidator: TodoBlock,
        );
        #[method_id(linkRenderingAttributes)]
        pub unsafe fn linkRenderingAttributes(
        ) -> Id<NSDictionary<NSAttributedStringKey, Object>, Shared>;
        #[method_id(renderingAttributesForLink:atLocation:)]
        pub unsafe fn renderingAttributesForLink_atLocation(
            &self,
            link: &Object,
            location: &NSTextLocation,
        ) -> Id<NSDictionary<NSAttributedStringKey, Object>, Shared>;
        #[method(enumerateTextSegmentsInRange:type:options:usingBlock:)]
        pub unsafe fn enumerateTextSegmentsInRange_type_options_usingBlock(
            &self,
            textRange: &NSTextRange,
            type_: NSTextLayoutManagerSegmentType,
            options: NSTextLayoutManagerSegmentOptions,
            block: TodoBlock,
        );
        #[method(replaceContentsInRange:withTextElements:)]
        pub unsafe fn replaceContentsInRange_withTextElements(
            &self,
            range: &NSTextRange,
            textElements: &NSArray<NSTextElement>,
        );
        #[method(replaceContentsInRange:withAttributedString:)]
        pub unsafe fn replaceContentsInRange_withAttributedString(
            &self,
            range: &NSTextRange,
            attributedString: &NSAttributedString,
        );
    }
);
pub type NSTextLayoutManagerDelegate = NSObject;
