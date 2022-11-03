//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTextLayoutManagerSegmentType {
        NSTextLayoutManagerSegmentTypeStandard = 0,
        NSTextLayoutManagerSegmentTypeSelection = 1,
        NSTextLayoutManagerSegmentTypeHighlight = 2,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSTextLayoutManagerSegmentOptions {
        NSTextLayoutManagerSegmentOptionsNone = 0,
        NSTextLayoutManagerSegmentOptionsRangeNotRequired = 1 << 0,
        NSTextLayoutManagerSegmentOptionsMiddleFragmentsExcluded = 1 << 1,
        NSTextLayoutManagerSegmentOptionsHeadSegmentExtended = 1 << 2,
        NSTextLayoutManagerSegmentOptionsTailSegmentExtended = 1 << 3,
        NSTextLayoutManagerSegmentOptionsUpstreamAffinity = 1 << 4,
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSTextLayoutManager;

    unsafe impl ClassType for NSTextLayoutManager {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSTextLayoutManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other delegate)]
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

        #[method_id(@__retain_semantics Other textContentManager)]
        pub unsafe fn textContentManager(&self) -> Option<Id<NSTextContentManager, Shared>>;

        #[method(replaceTextContentManager:)]
        pub unsafe fn replaceTextContentManager(&self, textContentManager: &NSTextContentManager);

        #[method_id(@__retain_semantics Other textContainer)]
        pub unsafe fn textContainer(&self) -> Option<Id<NSTextContainer, Shared>>;

        #[method(setTextContainer:)]
        pub unsafe fn setTextContainer(&self, textContainer: Option<&NSTextContainer>);

        #[method(usageBoundsForTextContainer)]
        pub unsafe fn usageBoundsForTextContainer(&self) -> CGRect;

        #[method_id(@__retain_semantics Other textViewportLayoutController)]
        pub unsafe fn textViewportLayoutController(
            &self,
        ) -> Id<NSTextViewportLayoutController, Shared>;

        #[method_id(@__retain_semantics Other layoutQueue)]
        pub unsafe fn layoutQueue(&self) -> Option<Id<NSOperationQueue, Shared>>;

        #[method(setLayoutQueue:)]
        pub unsafe fn setLayoutQueue(&self, layoutQueue: Option<&NSOperationQueue>);

        #[method(ensureLayoutForRange:)]
        pub unsafe fn ensureLayoutForRange(&self, range: &NSTextRange);

        #[method(ensureLayoutForBounds:)]
        pub unsafe fn ensureLayoutForBounds(&self, bounds: CGRect);

        #[method(invalidateLayoutForRange:)]
        pub unsafe fn invalidateLayoutForRange(&self, range: &NSTextRange);

        #[method_id(@__retain_semantics Other textLayoutFragmentForPosition:)]
        pub unsafe fn textLayoutFragmentForPosition(
            &self,
            position: CGPoint,
        ) -> Option<Id<NSTextLayoutFragment, Shared>>;

        #[method_id(@__retain_semantics Other textLayoutFragmentForLocation:)]
        pub unsafe fn textLayoutFragmentForLocation(
            &self,
            location: &NSTextLocation,
        ) -> Option<Id<NSTextLayoutFragment, Shared>>;

        #[method_id(@__retain_semantics Other enumerateTextLayoutFragmentsFromLocation:options:usingBlock:)]
        pub unsafe fn enumerateTextLayoutFragmentsFromLocation_options_usingBlock(
            &self,
            location: Option<&NSTextLocation>,
            options: NSTextLayoutFragmentEnumerationOptions,
            block: &Block<(NonNull<NSTextLayoutFragment>,), Bool>,
        ) -> Option<Id<NSTextLocation, Shared>>;

        #[method_id(@__retain_semantics Other textSelections)]
        pub unsafe fn textSelections(&self) -> Id<NSArray<NSTextSelection>, Shared>;

        #[method(setTextSelections:)]
        pub unsafe fn setTextSelections(&self, textSelections: &NSArray<NSTextSelection>);

        #[method_id(@__retain_semantics Other textSelectionNavigation)]
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
            block: &Block<
                (
                    NonNull<NSTextLayoutManager>,
                    NonNull<NSDictionary<NSAttributedStringKey, Object>>,
                    NonNull<NSTextRange>,
                ),
                Bool,
            >,
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
        pub unsafe fn renderingAttributesValidator(
            &self,
        ) -> *mut Block<(NonNull<NSTextLayoutManager>, NonNull<NSTextLayoutFragment>), ()>;

        #[method(setRenderingAttributesValidator:)]
        pub unsafe fn setRenderingAttributesValidator(
            &self,
            renderingAttributesValidator: Option<
                &Block<(NonNull<NSTextLayoutManager>, NonNull<NSTextLayoutFragment>), ()>,
            >,
        );

        #[method_id(@__retain_semantics Other linkRenderingAttributes)]
        pub unsafe fn linkRenderingAttributes(
        ) -> Id<NSDictionary<NSAttributedStringKey, Object>, Shared>;

        #[method_id(@__retain_semantics Other renderingAttributesForLink:atLocation:)]
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
            block: &Block<(*mut NSTextRange, CGRect, CGFloat, NonNull<NSTextContainer>), Bool>,
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

extern_protocol!(
    pub struct NSTextLayoutManagerDelegate;

    unsafe impl NSTextLayoutManagerDelegate {
        #[optional]
        #[method_id(@__retain_semantics Other textLayoutManager:textLayoutFragmentForLocation:inTextElement:)]
        pub unsafe fn textLayoutManager_textLayoutFragmentForLocation_inTextElement(
            &self,
            textLayoutManager: &NSTextLayoutManager,
            location: &NSTextLocation,
            textElement: &NSTextElement,
        ) -> Id<NSTextLayoutFragment, Shared>;

        #[optional]
        #[method(textLayoutManager:shouldBreakLineBeforeLocation:hyphenating:)]
        pub unsafe fn textLayoutManager_shouldBreakLineBeforeLocation_hyphenating(
            &self,
            textLayoutManager: &NSTextLayoutManager,
            location: &NSTextLocation,
            hyphenating: bool,
        ) -> bool;

        #[optional]
        #[method_id(@__retain_semantics Other textLayoutManager:renderingAttributesForLink:atLocation:defaultAttributes:)]
        pub unsafe fn textLayoutManager_renderingAttributesForLink_atLocation_defaultAttributes(
            &self,
            textLayoutManager: &NSTextLayoutManager,
            link: &Object,
            location: &NSTextLocation,
            renderingAttributes: &NSDictionary<NSAttributedStringKey, Object>,
        ) -> Option<Id<NSDictionary<NSAttributedStringKey, Object>, Shared>>;
    }
);
