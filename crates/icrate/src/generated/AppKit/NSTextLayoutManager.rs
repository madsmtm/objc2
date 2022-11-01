//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

pub type NSTextLayoutManagerSegmentType = NSInteger;
pub const NSTextLayoutManagerSegmentTypeStandard: NSTextLayoutManagerSegmentType = 0;
pub const NSTextLayoutManagerSegmentTypeSelection: NSTextLayoutManagerSegmentType = 1;
pub const NSTextLayoutManagerSegmentTypeHighlight: NSTextLayoutManagerSegmentType = 2;

pub type NSTextLayoutManagerSegmentOptions = NSUInteger;
pub const NSTextLayoutManagerSegmentOptionsNone: NSTextLayoutManagerSegmentOptions = 0;
pub const NSTextLayoutManagerSegmentOptionsRangeNotRequired: NSTextLayoutManagerSegmentOptions =
    1 << 0;
pub const NSTextLayoutManagerSegmentOptionsMiddleFragmentsExcluded:
    NSTextLayoutManagerSegmentOptions = 1 << 1;
pub const NSTextLayoutManagerSegmentOptionsHeadSegmentExtended: NSTextLayoutManagerSegmentOptions =
    1 << 2;
pub const NSTextLayoutManagerSegmentOptionsTailSegmentExtended: NSTextLayoutManagerSegmentOptions =
    1 << 3;
pub const NSTextLayoutManagerSegmentOptionsUpstreamAffinity: NSTextLayoutManagerSegmentOptions =
    1 << 4;

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
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

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
