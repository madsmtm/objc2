#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSTextLayoutOrientationProvider = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSLayoutManager;
    unsafe impl ClassType for NSLayoutManager {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSLayoutManager {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(textStorage)]
        pub unsafe fn textStorage(&self) -> Option<Id<NSTextStorage, Shared>>;
        #[method(setTextStorage:)]
        pub unsafe fn setTextStorage(&self, textStorage: Option<&NSTextStorage>);
        #[method(replaceTextStorage:)]
        pub unsafe fn replaceTextStorage(&self, newTextStorage: &NSTextStorage);
        #[method_id(textContainers)]
        pub unsafe fn textContainers(&self) -> Id<NSArray<NSTextContainer>, Shared>;
        #[method(addTextContainer:)]
        pub unsafe fn addTextContainer(&self, container: &NSTextContainer);
        #[method(insertTextContainer:atIndex:)]
        pub unsafe fn insertTextContainer_atIndex(
            &self,
            container: &NSTextContainer,
            index: NSUInteger,
        );
        #[method(removeTextContainerAtIndex:)]
        pub unsafe fn removeTextContainerAtIndex(&self, index: NSUInteger);
        #[method(textContainerChangedGeometry:)]
        pub unsafe fn textContainerChangedGeometry(&self, container: &NSTextContainer);
        #[method(textContainerChangedTextView:)]
        pub unsafe fn textContainerChangedTextView(&self, container: &NSTextContainer);
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSLayoutManagerDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSLayoutManagerDelegate>);
        #[method(showsInvisibleCharacters)]
        pub unsafe fn showsInvisibleCharacters(&self) -> bool;
        #[method(setShowsInvisibleCharacters:)]
        pub unsafe fn setShowsInvisibleCharacters(&self, showsInvisibleCharacters: bool);
        #[method(showsControlCharacters)]
        pub unsafe fn showsControlCharacters(&self) -> bool;
        #[method(setShowsControlCharacters:)]
        pub unsafe fn setShowsControlCharacters(&self, showsControlCharacters: bool);
        #[method(usesDefaultHyphenation)]
        pub unsafe fn usesDefaultHyphenation(&self) -> bool;
        #[method(setUsesDefaultHyphenation:)]
        pub unsafe fn setUsesDefaultHyphenation(&self, usesDefaultHyphenation: bool);
        #[method(usesFontLeading)]
        pub unsafe fn usesFontLeading(&self) -> bool;
        #[method(setUsesFontLeading:)]
        pub unsafe fn setUsesFontLeading(&self, usesFontLeading: bool);
        #[method(allowsNonContiguousLayout)]
        pub unsafe fn allowsNonContiguousLayout(&self) -> bool;
        #[method(setAllowsNonContiguousLayout:)]
        pub unsafe fn setAllowsNonContiguousLayout(&self, allowsNonContiguousLayout: bool);
        #[method(hasNonContiguousLayout)]
        pub unsafe fn hasNonContiguousLayout(&self) -> bool;
        #[method(limitsLayoutForSuspiciousContents)]
        pub unsafe fn limitsLayoutForSuspiciousContents(&self) -> bool;
        #[method(setLimitsLayoutForSuspiciousContents:)]
        pub unsafe fn setLimitsLayoutForSuspiciousContents(
            &self,
            limitsLayoutForSuspiciousContents: bool,
        );
        #[method(backgroundLayoutEnabled)]
        pub unsafe fn backgroundLayoutEnabled(&self) -> bool;
        #[method(setBackgroundLayoutEnabled:)]
        pub unsafe fn setBackgroundLayoutEnabled(&self, backgroundLayoutEnabled: bool);
        #[method(defaultAttachmentScaling)]
        pub unsafe fn defaultAttachmentScaling(&self) -> NSImageScaling;
        #[method(setDefaultAttachmentScaling:)]
        pub unsafe fn setDefaultAttachmentScaling(&self, defaultAttachmentScaling: NSImageScaling);
        #[method_id(typesetter)]
        pub unsafe fn typesetter(&self) -> Id<NSTypesetter, Shared>;
        #[method(setTypesetter:)]
        pub unsafe fn setTypesetter(&self, typesetter: &NSTypesetter);
        #[method(typesetterBehavior)]
        pub unsafe fn typesetterBehavior(&self) -> NSTypesetterBehavior;
        #[method(setTypesetterBehavior:)]
        pub unsafe fn setTypesetterBehavior(&self, typesetterBehavior: NSTypesetterBehavior);
        #[method(invalidateGlyphsForCharacterRange:changeInLength:actualCharacterRange:)]
        pub unsafe fn invalidateGlyphsForCharacterRange_changeInLength_actualCharacterRange(
            &self,
            charRange: NSRange,
            delta: NSInteger,
            actualCharRange: NSRangePointer,
        );
        #[method(invalidateLayoutForCharacterRange:actualCharacterRange:)]
        pub unsafe fn invalidateLayoutForCharacterRange_actualCharacterRange(
            &self,
            charRange: NSRange,
            actualCharRange: NSRangePointer,
        );
        #[method(invalidateDisplayForCharacterRange:)]
        pub unsafe fn invalidateDisplayForCharacterRange(&self, charRange: NSRange);
        #[method(invalidateDisplayForGlyphRange:)]
        pub unsafe fn invalidateDisplayForGlyphRange(&self, glyphRange: NSRange);
        #[method(processEditingForTextStorage:edited:range:changeInLength:invalidatedRange:)]
        pub unsafe fn processEditingForTextStorage_edited_range_changeInLength_invalidatedRange(
            &self,
            textStorage: &NSTextStorage,
            editMask: NSTextStorageEditActions,
            newCharRange: NSRange,
            delta: NSInteger,
            invalidatedCharRange: NSRange,
        );
        #[method(ensureGlyphsForCharacterRange:)]
        pub unsafe fn ensureGlyphsForCharacterRange(&self, charRange: NSRange);
        #[method(ensureGlyphsForGlyphRange:)]
        pub unsafe fn ensureGlyphsForGlyphRange(&self, glyphRange: NSRange);
        #[method(ensureLayoutForCharacterRange:)]
        pub unsafe fn ensureLayoutForCharacterRange(&self, charRange: NSRange);
        #[method(ensureLayoutForGlyphRange:)]
        pub unsafe fn ensureLayoutForGlyphRange(&self, glyphRange: NSRange);
        #[method(ensureLayoutForTextContainer:)]
        pub unsafe fn ensureLayoutForTextContainer(&self, container: &NSTextContainer);
        #[method(ensureLayoutForBoundingRect:inTextContainer:)]
        pub unsafe fn ensureLayoutForBoundingRect_inTextContainer(
            &self,
            bounds: NSRect,
            container: &NSTextContainer,
        );
        #[method(setGlyphs:properties:characterIndexes:font:forGlyphRange:)]
        pub unsafe fn setGlyphs_properties_characterIndexes_font_forGlyphRange(
            &self,
            glyphs: NonNull<CGGlyph>,
            props: NonNull<NSGlyphProperty>,
            charIndexes: NonNull<NSUInteger>,
            aFont: &NSFont,
            glyphRange: NSRange,
        );
        #[method(numberOfGlyphs)]
        pub unsafe fn numberOfGlyphs(&self) -> NSUInteger;
        #[method(CGGlyphAtIndex:isValidIndex:)]
        pub unsafe fn CGGlyphAtIndex_isValidIndex(
            &self,
            glyphIndex: NSUInteger,
            isValidIndex: *mut bool,
        ) -> CGGlyph;
        #[method(CGGlyphAtIndex:)]
        pub unsafe fn CGGlyphAtIndex(&self, glyphIndex: NSUInteger) -> CGGlyph;
        #[method(isValidGlyphIndex:)]
        pub unsafe fn isValidGlyphIndex(&self, glyphIndex: NSUInteger) -> bool;
        #[method(propertyForGlyphAtIndex:)]
        pub unsafe fn propertyForGlyphAtIndex(&self, glyphIndex: NSUInteger) -> NSGlyphProperty;
        #[method(characterIndexForGlyphAtIndex:)]
        pub unsafe fn characterIndexForGlyphAtIndex(&self, glyphIndex: NSUInteger) -> NSUInteger;
        #[method(glyphIndexForCharacterAtIndex:)]
        pub unsafe fn glyphIndexForCharacterAtIndex(&self, charIndex: NSUInteger) -> NSUInteger;
        #[method(getGlyphsInRange:glyphs:properties:characterIndexes:bidiLevels:)]
        pub unsafe fn getGlyphsInRange_glyphs_properties_characterIndexes_bidiLevels(
            &self,
            glyphRange: NSRange,
            glyphBuffer: *mut CGGlyph,
            props: *mut NSGlyphProperty,
            charIndexBuffer: *mut NSUInteger,
            bidiLevelBuffer: *mut c_uchar,
        ) -> NSUInteger;
        #[method(setTextContainer:forGlyphRange:)]
        pub unsafe fn setTextContainer_forGlyphRange(
            &self,
            container: &NSTextContainer,
            glyphRange: NSRange,
        );
        #[method(setLineFragmentRect:forGlyphRange:usedRect:)]
        pub unsafe fn setLineFragmentRect_forGlyphRange_usedRect(
            &self,
            fragmentRect: NSRect,
            glyphRange: NSRange,
            usedRect: NSRect,
        );
        #[method(setExtraLineFragmentRect:usedRect:textContainer:)]
        pub unsafe fn setExtraLineFragmentRect_usedRect_textContainer(
            &self,
            fragmentRect: NSRect,
            usedRect: NSRect,
            container: &NSTextContainer,
        );
        #[method(setLocation:forStartOfGlyphRange:)]
        pub unsafe fn setLocation_forStartOfGlyphRange(
            &self,
            location: NSPoint,
            glyphRange: NSRange,
        );
        #[method(setNotShownAttribute:forGlyphAtIndex:)]
        pub unsafe fn setNotShownAttribute_forGlyphAtIndex(
            &self,
            flag: bool,
            glyphIndex: NSUInteger,
        );
        #[method(setDrawsOutsideLineFragment:forGlyphAtIndex:)]
        pub unsafe fn setDrawsOutsideLineFragment_forGlyphAtIndex(
            &self,
            flag: bool,
            glyphIndex: NSUInteger,
        );
        #[method(setAttachmentSize:forGlyphRange:)]
        pub unsafe fn setAttachmentSize_forGlyphRange(
            &self,
            attachmentSize: NSSize,
            glyphRange: NSRange,
        );
        #[method(getFirstUnlaidCharacterIndex:glyphIndex:)]
        pub unsafe fn getFirstUnlaidCharacterIndex_glyphIndex(
            &self,
            charIndex: *mut NSUInteger,
            glyphIndex: *mut NSUInteger,
        );
        #[method(firstUnlaidCharacterIndex)]
        pub unsafe fn firstUnlaidCharacterIndex(&self) -> NSUInteger;
        #[method(firstUnlaidGlyphIndex)]
        pub unsafe fn firstUnlaidGlyphIndex(&self) -> NSUInteger;
        #[method_id(textContainerForGlyphAtIndex:effectiveRange:)]
        pub unsafe fn textContainerForGlyphAtIndex_effectiveRange(
            &self,
            glyphIndex: NSUInteger,
            effectiveGlyphRange: NSRangePointer,
        ) -> Option<Id<NSTextContainer, Shared>>;
        #[method_id(textContainerForGlyphAtIndex:effectiveRange:withoutAdditionalLayout:)]
        pub unsafe fn textContainerForGlyphAtIndex_effectiveRange_withoutAdditionalLayout(
            &self,
            glyphIndex: NSUInteger,
            effectiveGlyphRange: NSRangePointer,
            flag: bool,
        ) -> Option<Id<NSTextContainer, Shared>>;
        #[method(usedRectForTextContainer:)]
        pub unsafe fn usedRectForTextContainer(&self, container: &NSTextContainer) -> NSRect;
        #[method(lineFragmentRectForGlyphAtIndex:effectiveRange:)]
        pub unsafe fn lineFragmentRectForGlyphAtIndex_effectiveRange(
            &self,
            glyphIndex: NSUInteger,
            effectiveGlyphRange: NSRangePointer,
        ) -> NSRect;
        #[method(lineFragmentRectForGlyphAtIndex:effectiveRange:withoutAdditionalLayout:)]
        pub unsafe fn lineFragmentRectForGlyphAtIndex_effectiveRange_withoutAdditionalLayout(
            &self,
            glyphIndex: NSUInteger,
            effectiveGlyphRange: NSRangePointer,
            flag: bool,
        ) -> NSRect;
        #[method(lineFragmentUsedRectForGlyphAtIndex:effectiveRange:)]
        pub unsafe fn lineFragmentUsedRectForGlyphAtIndex_effectiveRange(
            &self,
            glyphIndex: NSUInteger,
            effectiveGlyphRange: NSRangePointer,
        ) -> NSRect;
        #[method(lineFragmentUsedRectForGlyphAtIndex:effectiveRange:withoutAdditionalLayout:)]
        pub unsafe fn lineFragmentUsedRectForGlyphAtIndex_effectiveRange_withoutAdditionalLayout(
            &self,
            glyphIndex: NSUInteger,
            effectiveGlyphRange: NSRangePointer,
            flag: bool,
        ) -> NSRect;
        #[method(extraLineFragmentRect)]
        pub unsafe fn extraLineFragmentRect(&self) -> NSRect;
        #[method(extraLineFragmentUsedRect)]
        pub unsafe fn extraLineFragmentUsedRect(&self) -> NSRect;
        #[method_id(extraLineFragmentTextContainer)]
        pub unsafe fn extraLineFragmentTextContainer(&self) -> Option<Id<NSTextContainer, Shared>>;
        #[method(locationForGlyphAtIndex:)]
        pub unsafe fn locationForGlyphAtIndex(&self, glyphIndex: NSUInteger) -> NSPoint;
        #[method(notShownAttributeForGlyphAtIndex:)]
        pub unsafe fn notShownAttributeForGlyphAtIndex(&self, glyphIndex: NSUInteger) -> bool;
        #[method(drawsOutsideLineFragmentForGlyphAtIndex:)]
        pub unsafe fn drawsOutsideLineFragmentForGlyphAtIndex(
            &self,
            glyphIndex: NSUInteger,
        ) -> bool;
        #[method(attachmentSizeForGlyphAtIndex:)]
        pub unsafe fn attachmentSizeForGlyphAtIndex(&self, glyphIndex: NSUInteger) -> NSSize;
        #[method(truncatedGlyphRangeInLineFragmentForGlyphAtIndex:)]
        pub unsafe fn truncatedGlyphRangeInLineFragmentForGlyphAtIndex(
            &self,
            glyphIndex: NSUInteger,
        ) -> NSRange;
        #[method(glyphRangeForCharacterRange:actualCharacterRange:)]
        pub unsafe fn glyphRangeForCharacterRange_actualCharacterRange(
            &self,
            charRange: NSRange,
            actualCharRange: NSRangePointer,
        ) -> NSRange;
        #[method(characterRangeForGlyphRange:actualGlyphRange:)]
        pub unsafe fn characterRangeForGlyphRange_actualGlyphRange(
            &self,
            glyphRange: NSRange,
            actualGlyphRange: NSRangePointer,
        ) -> NSRange;
        #[method(glyphRangeForTextContainer:)]
        pub unsafe fn glyphRangeForTextContainer(&self, container: &NSTextContainer) -> NSRange;
        #[method(rangeOfNominallySpacedGlyphsContainingIndex:)]
        pub unsafe fn rangeOfNominallySpacedGlyphsContainingIndex(
            &self,
            glyphIndex: NSUInteger,
        ) -> NSRange;
        #[method(boundingRectForGlyphRange:inTextContainer:)]
        pub unsafe fn boundingRectForGlyphRange_inTextContainer(
            &self,
            glyphRange: NSRange,
            container: &NSTextContainer,
        ) -> NSRect;
        #[method(glyphRangeForBoundingRect:inTextContainer:)]
        pub unsafe fn glyphRangeForBoundingRect_inTextContainer(
            &self,
            bounds: NSRect,
            container: &NSTextContainer,
        ) -> NSRange;
        #[method(glyphRangeForBoundingRectWithoutAdditionalLayout:inTextContainer:)]
        pub unsafe fn glyphRangeForBoundingRectWithoutAdditionalLayout_inTextContainer(
            &self,
            bounds: NSRect,
            container: &NSTextContainer,
        ) -> NSRange;
        #[method(glyphIndexForPoint:inTextContainer:fractionOfDistanceThroughGlyph:)]
        pub unsafe fn glyphIndexForPoint_inTextContainer_fractionOfDistanceThroughGlyph(
            &self,
            point: NSPoint,
            container: &NSTextContainer,
            partialFraction: *mut CGFloat,
        ) -> NSUInteger;
        #[method(glyphIndexForPoint:inTextContainer:)]
        pub unsafe fn glyphIndexForPoint_inTextContainer(
            &self,
            point: NSPoint,
            container: &NSTextContainer,
        ) -> NSUInteger;
        #[method(fractionOfDistanceThroughGlyphForPoint:inTextContainer:)]
        pub unsafe fn fractionOfDistanceThroughGlyphForPoint_inTextContainer(
            &self,
            point: NSPoint,
            container: &NSTextContainer,
        ) -> CGFloat;
        #[method(characterIndexForPoint:inTextContainer:fractionOfDistanceBetweenInsertionPoints:)]
        pub unsafe fn characterIndexForPoint_inTextContainer_fractionOfDistanceBetweenInsertionPoints(
            &self,
            point: NSPoint,
            container: &NSTextContainer,
            partialFraction: *mut CGFloat,
        ) -> NSUInteger;
        #[method(getLineFragmentInsertionPointsForCharacterAtIndex:alternatePositions:inDisplayOrder:positions:characterIndexes:)]
        pub unsafe fn getLineFragmentInsertionPointsForCharacterAtIndex_alternatePositions_inDisplayOrder_positions_characterIndexes(
            &self,
            charIndex: NSUInteger,
            aFlag: bool,
            dFlag: bool,
            positions: *mut CGFloat,
            charIndexes: *mut NSUInteger,
        ) -> NSUInteger;
        #[method(enumerateLineFragmentsForGlyphRange:usingBlock:)]
        pub unsafe fn enumerateLineFragmentsForGlyphRange_usingBlock(
            &self,
            glyphRange: NSRange,
            block: TodoBlock,
        );
        #[method(enumerateEnclosingRectsForGlyphRange:withinSelectedGlyphRange:inTextContainer:usingBlock:)]
        pub unsafe fn enumerateEnclosingRectsForGlyphRange_withinSelectedGlyphRange_inTextContainer_usingBlock(
            &self,
            glyphRange: NSRange,
            selectedRange: NSRange,
            textContainer: &NSTextContainer,
            block: TodoBlock,
        );
        #[method(drawBackgroundForGlyphRange:atPoint:)]
        pub unsafe fn drawBackgroundForGlyphRange_atPoint(
            &self,
            glyphsToShow: NSRange,
            origin: NSPoint,
        );
        #[method(drawGlyphsForGlyphRange:atPoint:)]
        pub unsafe fn drawGlyphsForGlyphRange_atPoint(
            &self,
            glyphsToShow: NSRange,
            origin: NSPoint,
        );
        #[method(showCGGlyphs:positions:count:font:textMatrix:attributes:inContext:)]
        pub unsafe fn showCGGlyphs_positions_count_font_textMatrix_attributes_inContext(
            &self,
            glyphs: NonNull<CGGlyph>,
            positions: NonNull<CGPoint>,
            glyphCount: NSInteger,
            font: &NSFont,
            textMatrix: CGAffineTransform,
            attributes: &NSDictionary<NSAttributedStringKey, Object>,
            CGContext: CGContextRef,
        );
        #[method(fillBackgroundRectArray:count:forCharacterRange:color:)]
        pub unsafe fn fillBackgroundRectArray_count_forCharacterRange_color(
            &self,
            rectArray: NonNull<NSRect>,
            rectCount: NSUInteger,
            charRange: NSRange,
            color: &NSColor,
        );
        #[method(drawUnderlineForGlyphRange:underlineType:baselineOffset:lineFragmentRect:lineFragmentGlyphRange:containerOrigin:)]
        pub unsafe fn drawUnderlineForGlyphRange_underlineType_baselineOffset_lineFragmentRect_lineFragmentGlyphRange_containerOrigin(
            &self,
            glyphRange: NSRange,
            underlineVal: NSUnderlineStyle,
            baselineOffset: CGFloat,
            lineRect: NSRect,
            lineGlyphRange: NSRange,
            containerOrigin: NSPoint,
        );
        #[method(underlineGlyphRange:underlineType:lineFragmentRect:lineFragmentGlyphRange:containerOrigin:)]
        pub unsafe fn underlineGlyphRange_underlineType_lineFragmentRect_lineFragmentGlyphRange_containerOrigin(
            &self,
            glyphRange: NSRange,
            underlineVal: NSUnderlineStyle,
            lineRect: NSRect,
            lineGlyphRange: NSRange,
            containerOrigin: NSPoint,
        );
        #[method(drawStrikethroughForGlyphRange:strikethroughType:baselineOffset:lineFragmentRect:lineFragmentGlyphRange:containerOrigin:)]
        pub unsafe fn drawStrikethroughForGlyphRange_strikethroughType_baselineOffset_lineFragmentRect_lineFragmentGlyphRange_containerOrigin(
            &self,
            glyphRange: NSRange,
            strikethroughVal: NSUnderlineStyle,
            baselineOffset: CGFloat,
            lineRect: NSRect,
            lineGlyphRange: NSRange,
            containerOrigin: NSPoint,
        );
        #[method(strikethroughGlyphRange:strikethroughType:lineFragmentRect:lineFragmentGlyphRange:containerOrigin:)]
        pub unsafe fn strikethroughGlyphRange_strikethroughType_lineFragmentRect_lineFragmentGlyphRange_containerOrigin(
            &self,
            glyphRange: NSRange,
            strikethroughVal: NSUnderlineStyle,
            lineRect: NSRect,
            lineGlyphRange: NSRange,
            containerOrigin: NSPoint,
        );
        #[method(showAttachmentCell:inRect:characterIndex:)]
        pub unsafe fn showAttachmentCell_inRect_characterIndex(
            &self,
            cell: &NSCell,
            rect: NSRect,
            attachmentIndex: NSUInteger,
        );
        #[method(setLayoutRect:forTextBlock:glyphRange:)]
        pub unsafe fn setLayoutRect_forTextBlock_glyphRange(
            &self,
            rect: NSRect,
            block: &NSTextBlock,
            glyphRange: NSRange,
        );
        #[method(setBoundsRect:forTextBlock:glyphRange:)]
        pub unsafe fn setBoundsRect_forTextBlock_glyphRange(
            &self,
            rect: NSRect,
            block: &NSTextBlock,
            glyphRange: NSRange,
        );
        #[method(layoutRectForTextBlock:glyphRange:)]
        pub unsafe fn layoutRectForTextBlock_glyphRange(
            &self,
            block: &NSTextBlock,
            glyphRange: NSRange,
        ) -> NSRect;
        #[method(boundsRectForTextBlock:glyphRange:)]
        pub unsafe fn boundsRectForTextBlock_glyphRange(
            &self,
            block: &NSTextBlock,
            glyphRange: NSRange,
        ) -> NSRect;
        #[method(layoutRectForTextBlock:atIndex:effectiveRange:)]
        pub unsafe fn layoutRectForTextBlock_atIndex_effectiveRange(
            &self,
            block: &NSTextBlock,
            glyphIndex: NSUInteger,
            effectiveGlyphRange: NSRangePointer,
        ) -> NSRect;
        #[method(boundsRectForTextBlock:atIndex:effectiveRange:)]
        pub unsafe fn boundsRectForTextBlock_atIndex_effectiveRange(
            &self,
            block: &NSTextBlock,
            glyphIndex: NSUInteger,
            effectiveGlyphRange: NSRangePointer,
        ) -> NSRect;
        #[method_id(temporaryAttributesAtCharacterIndex:effectiveRange:)]
        pub unsafe fn temporaryAttributesAtCharacterIndex_effectiveRange(
            &self,
            charIndex: NSUInteger,
            effectiveCharRange: NSRangePointer,
        ) -> Id<NSDictionary<NSAttributedStringKey, Object>, Shared>;
        #[method(setTemporaryAttributes:forCharacterRange:)]
        pub unsafe fn setTemporaryAttributes_forCharacterRange(
            &self,
            attrs: &NSDictionary<NSAttributedStringKey, Object>,
            charRange: NSRange,
        );
        #[method(addTemporaryAttributes:forCharacterRange:)]
        pub unsafe fn addTemporaryAttributes_forCharacterRange(
            &self,
            attrs: &NSDictionary<NSAttributedStringKey, Object>,
            charRange: NSRange,
        );
        #[method(removeTemporaryAttribute:forCharacterRange:)]
        pub unsafe fn removeTemporaryAttribute_forCharacterRange(
            &self,
            attrName: &NSAttributedStringKey,
            charRange: NSRange,
        );
        #[method_id(temporaryAttribute:atCharacterIndex:effectiveRange:)]
        pub unsafe fn temporaryAttribute_atCharacterIndex_effectiveRange(
            &self,
            attrName: &NSAttributedStringKey,
            location: NSUInteger,
            range: NSRangePointer,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(temporaryAttribute:atCharacterIndex:longestEffectiveRange:inRange:)]
        pub unsafe fn temporaryAttribute_atCharacterIndex_longestEffectiveRange_inRange(
            &self,
            attrName: &NSAttributedStringKey,
            location: NSUInteger,
            range: NSRangePointer,
            rangeLimit: NSRange,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(temporaryAttributesAtCharacterIndex:longestEffectiveRange:inRange:)]
        pub unsafe fn temporaryAttributesAtCharacterIndex_longestEffectiveRange_inRange(
            &self,
            location: NSUInteger,
            range: NSRangePointer,
            rangeLimit: NSRange,
        ) -> Id<NSDictionary<NSAttributedStringKey, Object>, Shared>;
        #[method(addTemporaryAttribute:value:forCharacterRange:)]
        pub unsafe fn addTemporaryAttribute_value_forCharacterRange(
            &self,
            attrName: &NSAttributedStringKey,
            value: &Object,
            charRange: NSRange,
        );
        #[method(defaultLineHeightForFont:)]
        pub unsafe fn defaultLineHeightForFont(&self, theFont: &NSFont) -> CGFloat;
        #[method(defaultBaselineOffsetForFont:)]
        pub unsafe fn defaultBaselineOffsetForFont(&self, theFont: &NSFont) -> CGFloat;
    }
);
extern_methods!(
    #[doc = "NSTextViewSupport"]
    unsafe impl NSLayoutManager {
        #[method_id(rulerMarkersForTextView:paragraphStyle:ruler:)]
        pub unsafe fn rulerMarkersForTextView_paragraphStyle_ruler(
            &self,
            view: &NSTextView,
            style: &NSParagraphStyle,
            ruler: &NSRulerView,
        ) -> Id<NSArray<NSRulerMarker>, Shared>;
        #[method_id(rulerAccessoryViewForTextView:paragraphStyle:ruler:enabled:)]
        pub unsafe fn rulerAccessoryViewForTextView_paragraphStyle_ruler_enabled(
            &self,
            view: &NSTextView,
            style: &NSParagraphStyle,
            ruler: &NSRulerView,
            isEnabled: bool,
        ) -> Option<Id<NSView, Shared>>;
        #[method(layoutManagerOwnsFirstResponderInWindow:)]
        pub unsafe fn layoutManagerOwnsFirstResponderInWindow(&self, window: &NSWindow) -> bool;
        #[method_id(firstTextView)]
        pub unsafe fn firstTextView(&self) -> Option<Id<NSTextView, Shared>>;
        #[method_id(textViewForBeginningOfSelection)]
        pub unsafe fn textViewForBeginningOfSelection(&self) -> Option<Id<NSTextView, Shared>>;
    }
);
pub type NSLayoutManagerDelegate = NSObject;
extern_methods!(
    #[doc = "NSLayoutManagerDeprecated"]
    unsafe impl NSLayoutManager {
        #[method(glyphAtIndex:isValidIndex:)]
        pub unsafe fn glyphAtIndex_isValidIndex(
            &self,
            glyphIndex: NSUInteger,
            isValidIndex: *mut bool,
        ) -> NSGlyph;
        #[method(glyphAtIndex:)]
        pub unsafe fn glyphAtIndex(&self, glyphIndex: NSUInteger) -> NSGlyph;
        #[method(rectArrayForCharacterRange:withinSelectedCharacterRange:inTextContainer:rectCount:)]
        pub unsafe fn rectArrayForCharacterRange_withinSelectedCharacterRange_inTextContainer_rectCount(
            &self,
            charRange: NSRange,
            selCharRange: NSRange,
            container: &NSTextContainer,
            rectCount: NonNull<NSUInteger>,
        ) -> NSRectArray;
        #[method(rectArrayForGlyphRange:withinSelectedGlyphRange:inTextContainer:rectCount:)]
        pub unsafe fn rectArrayForGlyphRange_withinSelectedGlyphRange_inTextContainer_rectCount(
            &self,
            glyphRange: NSRange,
            selGlyphRange: NSRange,
            container: &NSTextContainer,
            rectCount: NonNull<NSUInteger>,
        ) -> NSRectArray;
        #[method(usesScreenFonts)]
        pub unsafe fn usesScreenFonts(&self) -> bool;
        #[method(setUsesScreenFonts:)]
        pub unsafe fn setUsesScreenFonts(&self, usesScreenFonts: bool);
        #[method_id(substituteFontForFont:)]
        pub unsafe fn substituteFontForFont(&self, originalFont: &NSFont) -> Id<NSFont, Shared>;
        #[method(insertGlyphs:length:forStartingGlyphAtIndex:characterIndex:)]
        pub unsafe fn insertGlyphs_length_forStartingGlyphAtIndex_characterIndex(
            &self,
            glyphs: NonNull<NSGlyph>,
            length: NSUInteger,
            glyphIndex: NSUInteger,
            charIndex: NSUInteger,
        );
        #[method(insertGlyph:atGlyphIndex:characterIndex:)]
        pub unsafe fn insertGlyph_atGlyphIndex_characterIndex(
            &self,
            glyph: NSGlyph,
            glyphIndex: NSUInteger,
            charIndex: NSUInteger,
        );
        #[method(replaceGlyphAtIndex:withGlyph:)]
        pub unsafe fn replaceGlyphAtIndex_withGlyph(
            &self,
            glyphIndex: NSUInteger,
            newGlyph: NSGlyph,
        );
        #[method(deleteGlyphsInRange:)]
        pub unsafe fn deleteGlyphsInRange(&self, glyphRange: NSRange);
        #[method(setCharacterIndex:forGlyphAtIndex:)]
        pub unsafe fn setCharacterIndex_forGlyphAtIndex(
            &self,
            charIndex: NSUInteger,
            glyphIndex: NSUInteger,
        );
        #[method(setIntAttribute:value:forGlyphAtIndex:)]
        pub unsafe fn setIntAttribute_value_forGlyphAtIndex(
            &self,
            attributeTag: NSInteger,
            val: NSInteger,
            glyphIndex: NSUInteger,
        );
        #[method(invalidateGlyphsOnLayoutInvalidationForGlyphRange:)]
        pub unsafe fn invalidateGlyphsOnLayoutInvalidationForGlyphRange(&self, glyphRange: NSRange);
        #[method(intAttribute:forGlyphAtIndex:)]
        pub unsafe fn intAttribute_forGlyphAtIndex(
            &self,
            attributeTag: NSInteger,
            glyphIndex: NSUInteger,
        ) -> NSInteger;
        #[method(getGlyphsInRange:glyphs:characterIndexes:glyphInscriptions:elasticBits:)]
        pub unsafe fn getGlyphsInRange_glyphs_characterIndexes_glyphInscriptions_elasticBits(
            &self,
            glyphRange: NSRange,
            glyphBuffer: *mut NSGlyph,
            charIndexBuffer: *mut NSUInteger,
            inscribeBuffer: *mut NSGlyphInscription,
            elasticBuffer: *mut bool,
        ) -> NSUInteger;
        #[method(getGlyphsInRange:glyphs:characterIndexes:glyphInscriptions:elasticBits:bidiLevels:)]
        pub unsafe fn getGlyphsInRange_glyphs_characterIndexes_glyphInscriptions_elasticBits_bidiLevels(
            &self,
            glyphRange: NSRange,
            glyphBuffer: *mut NSGlyph,
            charIndexBuffer: *mut NSUInteger,
            inscribeBuffer: *mut NSGlyphInscription,
            elasticBuffer: *mut bool,
            bidiLevelBuffer: *mut c_uchar,
        ) -> NSUInteger;
        #[method(getGlyphs:range:)]
        pub unsafe fn getGlyphs_range(
            &self,
            glyphArray: *mut NSGlyph,
            glyphRange: NSRange,
        ) -> NSUInteger;
        #[method(invalidateLayoutForCharacterRange:isSoft:actualCharacterRange:)]
        pub unsafe fn invalidateLayoutForCharacterRange_isSoft_actualCharacterRange(
            &self,
            charRange: NSRange,
            flag: bool,
            actualCharRange: NSRangePointer,
        );
        #[method(textStorage:edited:range:changeInLength:invalidatedRange:)]
        pub unsafe fn textStorage_edited_range_changeInLength_invalidatedRange(
            &self,
            str: &NSTextStorage,
            editedMask: NSTextStorageEditedOptions,
            newCharRange: NSRange,
            delta: NSInteger,
            invalidatedCharRange: NSRange,
        );
        #[method(setLocations:startingGlyphIndexes:count:forGlyphRange:)]
        pub unsafe fn setLocations_startingGlyphIndexes_count_forGlyphRange(
            &self,
            locations: NSPointArray,
            glyphIndexes: NonNull<NSUInteger>,
            count: NSUInteger,
            glyphRange: NSRange,
        );
        #[method(showPackedGlyphs:length:glyphRange:atPoint:font:color:printingAdjustment:)]
        pub unsafe fn showPackedGlyphs_length_glyphRange_atPoint_font_color_printingAdjustment(
            &self,
            glyphs: NonNull<c_char>,
            glyphLen: NSUInteger,
            glyphRange: NSRange,
            point: NSPoint,
            font: &NSFont,
            color: &NSColor,
            printingAdjustment: NSSize,
        );
        #[method(showCGGlyphs:positions:count:font:matrix:attributes:inContext:)]
        pub unsafe fn showCGGlyphs_positions_count_font_matrix_attributes_inContext(
            &self,
            glyphs: NonNull<CGGlyph>,
            positions: NonNull<NSPoint>,
            glyphCount: NSUInteger,
            font: &NSFont,
            textMatrix: &NSAffineTransform,
            attributes: &NSDictionary<NSAttributedStringKey, Object>,
            graphicsContext: &NSGraphicsContext,
        );
        #[method(hyphenationFactor)]
        pub unsafe fn hyphenationFactor(&self) -> c_float;
        #[method(setHyphenationFactor:)]
        pub unsafe fn setHyphenationFactor(&self, hyphenationFactor: c_float);
    }
);
extern_methods!(
    #[doc = "NSGlyphGeneration"]
    unsafe impl NSLayoutManager {
        #[method_id(glyphGenerator)]
        pub unsafe fn glyphGenerator(&self) -> Id<NSGlyphGenerator, Shared>;
        #[method(setGlyphGenerator:)]
        pub unsafe fn setGlyphGenerator(&self, glyphGenerator: &NSGlyphGenerator);
    }
);