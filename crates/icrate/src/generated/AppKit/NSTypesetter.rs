#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTypesetter;
    unsafe impl ClassType for NSTypesetter {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSTypesetter {
        #[method(usesFontLeading)]
        pub unsafe fn usesFontLeading(&self) -> bool;
        #[method(setUsesFontLeading:)]
        pub unsafe fn setUsesFontLeading(&self, usesFontLeading: bool);
        #[method(typesetterBehavior)]
        pub unsafe fn typesetterBehavior(&self) -> NSTypesetterBehavior;
        #[method(setTypesetterBehavior:)]
        pub unsafe fn setTypesetterBehavior(&self, typesetterBehavior: NSTypesetterBehavior);
        #[method(hyphenationFactor)]
        pub unsafe fn hyphenationFactor(&self) -> c_float;
        #[method(setHyphenationFactor:)]
        pub unsafe fn setHyphenationFactor(&self, hyphenationFactor: c_float);
        #[method(lineFragmentPadding)]
        pub unsafe fn lineFragmentPadding(&self) -> CGFloat;
        #[method(setLineFragmentPadding:)]
        pub unsafe fn setLineFragmentPadding(&self, lineFragmentPadding: CGFloat);
        #[method_id(substituteFontForFont:)]
        pub unsafe fn substituteFontForFont(&self, originalFont: &NSFont) -> Id<NSFont, Shared>;
        #[method_id(textTabForGlyphLocation:writingDirection:maxLocation:)]
        pub unsafe fn textTabForGlyphLocation_writingDirection_maxLocation(
            &self,
            glyphLocation: CGFloat,
            direction: NSWritingDirection,
            maxLocation: CGFloat,
        ) -> Option<Id<NSTextTab, Shared>>;
        #[method(bidiProcessingEnabled)]
        pub unsafe fn bidiProcessingEnabled(&self) -> bool;
        #[method(setBidiProcessingEnabled:)]
        pub unsafe fn setBidiProcessingEnabled(&self, bidiProcessingEnabled: bool);
        #[method_id(attributedString)]
        pub unsafe fn attributedString(&self) -> Option<Id<NSAttributedString, Shared>>;
        #[method(setAttributedString:)]
        pub unsafe fn setAttributedString(&self, attributedString: Option<&NSAttributedString>);
        #[method(setParagraphGlyphRange:separatorGlyphRange:)]
        pub unsafe fn setParagraphGlyphRange_separatorGlyphRange(
            &self,
            paragraphRange: NSRange,
            paragraphSeparatorRange: NSRange,
        );
        #[method(paragraphGlyphRange)]
        pub unsafe fn paragraphGlyphRange(&self) -> NSRange;
        #[method(paragraphSeparatorGlyphRange)]
        pub unsafe fn paragraphSeparatorGlyphRange(&self) -> NSRange;
        #[method(paragraphCharacterRange)]
        pub unsafe fn paragraphCharacterRange(&self) -> NSRange;
        #[method(paragraphSeparatorCharacterRange)]
        pub unsafe fn paragraphSeparatorCharacterRange(&self) -> NSRange;
        #[method(layoutParagraphAtPoint:)]
        pub unsafe fn layoutParagraphAtPoint(
            &self,
            lineFragmentOrigin: NSPointPointer,
        ) -> NSUInteger;
        #[method(beginParagraph)]
        pub unsafe fn beginParagraph(&self);
        #[method(endParagraph)]
        pub unsafe fn endParagraph(&self);
        #[method(beginLineWithGlyphAtIndex:)]
        pub unsafe fn beginLineWithGlyphAtIndex(&self, glyphIndex: NSUInteger);
        #[method(endLineWithGlyphRange:)]
        pub unsafe fn endLineWithGlyphRange(&self, lineGlyphRange: NSRange);
        #[method(lineSpacingAfterGlyphAtIndex:withProposedLineFragmentRect:)]
        pub unsafe fn lineSpacingAfterGlyphAtIndex_withProposedLineFragmentRect(
            &self,
            glyphIndex: NSUInteger,
            rect: NSRect,
        ) -> CGFloat;
        #[method(paragraphSpacingBeforeGlyphAtIndex:withProposedLineFragmentRect:)]
        pub unsafe fn paragraphSpacingBeforeGlyphAtIndex_withProposedLineFragmentRect(
            &self,
            glyphIndex: NSUInteger,
            rect: NSRect,
        ) -> CGFloat;
        #[method(paragraphSpacingAfterGlyphAtIndex:withProposedLineFragmentRect:)]
        pub unsafe fn paragraphSpacingAfterGlyphAtIndex_withProposedLineFragmentRect(
            &self,
            glyphIndex: NSUInteger,
            rect: NSRect,
        ) -> CGFloat;
        #[method(getLineFragmentRect:usedRect:forParagraphSeparatorGlyphRange:atProposedOrigin:)]
        pub unsafe fn getLineFragmentRect_usedRect_forParagraphSeparatorGlyphRange_atProposedOrigin(
            &self,
            lineFragmentRect: NSRectPointer,
            lineFragmentUsedRect: NSRectPointer,
            paragraphSeparatorGlyphRange: NSRange,
            lineOrigin: NSPoint,
        );
        #[method_id(attributesForExtraLineFragment)]
        pub unsafe fn attributesForExtraLineFragment(
            &self,
        ) -> Id<NSDictionary<NSAttributedStringKey, Object>, Shared>;
        #[method_id(layoutManager)]
        pub unsafe fn layoutManager(&self) -> Option<Id<NSLayoutManager, Shared>>;
        #[method_id(textContainers)]
        pub unsafe fn textContainers(&self) -> Option<Id<NSArray<NSTextContainer>, Shared>>;
        #[method_id(currentTextContainer)]
        pub unsafe fn currentTextContainer(&self) -> Option<Id<NSTextContainer, Shared>>;
        #[method_id(currentParagraphStyle)]
        pub unsafe fn currentParagraphStyle(&self) -> Option<Id<NSParagraphStyle, Shared>>;
        #[method(setHardInvalidation:forGlyphRange:)]
        pub unsafe fn setHardInvalidation_forGlyphRange(&self, flag: bool, glyphRange: NSRange);
        #[method(layoutGlyphsInLayoutManager:startingAtGlyphIndex:maxNumberOfLineFragments:nextGlyphIndex:)]
        pub unsafe fn layoutGlyphsInLayoutManager_startingAtGlyphIndex_maxNumberOfLineFragments_nextGlyphIndex(
            &self,
            layoutManager: &NSLayoutManager,
            startGlyphIndex: NSUInteger,
            maxNumLines: NSUInteger,
            nextGlyph: NonNull<NSUInteger>,
        );
        #[method(layoutCharactersInRange:forLayoutManager:maximumNumberOfLineFragments:)]
        pub unsafe fn layoutCharactersInRange_forLayoutManager_maximumNumberOfLineFragments(
            &self,
            characterRange: NSRange,
            layoutManager: &NSLayoutManager,
            maxNumLines: NSUInteger,
        ) -> NSRange;
        #[method(printingAdjustmentInLayoutManager:forNominallySpacedGlyphRange:packedGlyphs:count:)]
        pub unsafe fn printingAdjustmentInLayoutManager_forNominallySpacedGlyphRange_packedGlyphs_count(
            layoutMgr: &NSLayoutManager,
            nominallySpacedGlyphsRange: NSRange,
            packedGlyphs: NonNull<c_uchar>,
            packedGlyphsCount: NSUInteger,
        ) -> NSSize;
        #[method(baselineOffsetInLayoutManager:glyphIndex:)]
        pub unsafe fn baselineOffsetInLayoutManager_glyphIndex(
            &self,
            layoutMgr: &NSLayoutManager,
            glyphIndex: NSUInteger,
        ) -> CGFloat;
        #[method_id(sharedSystemTypesetter)]
        pub unsafe fn sharedSystemTypesetter() -> Id<NSTypesetter, Shared>;
        #[method_id(sharedSystemTypesetterForBehavior:)]
        pub unsafe fn sharedSystemTypesetterForBehavior(
            behavior: NSTypesetterBehavior,
        ) -> Id<Object, Shared>;
        #[method(defaultTypesetterBehavior)]
        pub unsafe fn defaultTypesetterBehavior() -> NSTypesetterBehavior;
    }
);
extern_methods!(
    #[doc = "NSLayoutPhaseInterface"]
    unsafe impl NSTypesetter {
        #[method(willSetLineFragmentRect:forGlyphRange:usedRect:baselineOffset:)]
        pub unsafe fn willSetLineFragmentRect_forGlyphRange_usedRect_baselineOffset(
            &self,
            lineRect: NSRectPointer,
            glyphRange: NSRange,
            usedRect: NSRectPointer,
            baselineOffset: NonNull<CGFloat>,
        );
        #[method(shouldBreakLineByWordBeforeCharacterAtIndex:)]
        pub unsafe fn shouldBreakLineByWordBeforeCharacterAtIndex(
            &self,
            charIndex: NSUInteger,
        ) -> bool;
        #[method(shouldBreakLineByHyphenatingBeforeCharacterAtIndex:)]
        pub unsafe fn shouldBreakLineByHyphenatingBeforeCharacterAtIndex(
            &self,
            charIndex: NSUInteger,
        ) -> bool;
        #[method(hyphenationFactorForGlyphAtIndex:)]
        pub unsafe fn hyphenationFactorForGlyphAtIndex(&self, glyphIndex: NSUInteger) -> c_float;
        #[method(hyphenCharacterForGlyphAtIndex:)]
        pub unsafe fn hyphenCharacterForGlyphAtIndex(&self, glyphIndex: NSUInteger) -> UTF32Char;
        #[method(boundingBoxForControlGlyphAtIndex:forTextContainer:proposedLineFragment:glyphPosition:characterIndex:)]
        pub unsafe fn boundingBoxForControlGlyphAtIndex_forTextContainer_proposedLineFragment_glyphPosition_characterIndex(
            &self,
            glyphIndex: NSUInteger,
            textContainer: &NSTextContainer,
            proposedRect: NSRect,
            glyphPosition: NSPoint,
            charIndex: NSUInteger,
        ) -> NSRect;
    }
);
extern_methods!(
    #[doc = "NSGlyphStorageInterface"]
    unsafe impl NSTypesetter {
        #[method(characterRangeForGlyphRange:actualGlyphRange:)]
        pub unsafe fn characterRangeForGlyphRange_actualGlyphRange(
            &self,
            glyphRange: NSRange,
            actualGlyphRange: NSRangePointer,
        ) -> NSRange;
        #[method(glyphRangeForCharacterRange:actualCharacterRange:)]
        pub unsafe fn glyphRangeForCharacterRange_actualCharacterRange(
            &self,
            charRange: NSRange,
            actualCharRange: NSRangePointer,
        ) -> NSRange;
        #[method(getLineFragmentRect:usedRect:remainingRect:forStartingGlyphAtIndex:proposedRect:lineSpacing:paragraphSpacingBefore:paragraphSpacingAfter:)]
        pub unsafe fn getLineFragmentRect_usedRect_remainingRect_forStartingGlyphAtIndex_proposedRect_lineSpacing_paragraphSpacingBefore_paragraphSpacingAfter(
            &self,
            lineFragmentRect: NSRectPointer,
            lineFragmentUsedRect: NSRectPointer,
            remainingRect: NSRectPointer,
            startingGlyphIndex: NSUInteger,
            proposedRect: NSRect,
            lineSpacing: CGFloat,
            paragraphSpacingBefore: CGFloat,
            paragraphSpacingAfter: CGFloat,
        );
        #[method(setLineFragmentRect:forGlyphRange:usedRect:baselineOffset:)]
        pub unsafe fn setLineFragmentRect_forGlyphRange_usedRect_baselineOffset(
            &self,
            fragmentRect: NSRect,
            glyphRange: NSRange,
            usedRect: NSRect,
            baselineOffset: CGFloat,
        );
        #[method(setNotShownAttribute:forGlyphRange:)]
        pub unsafe fn setNotShownAttribute_forGlyphRange(&self, flag: bool, glyphRange: NSRange);
        #[method(setDrawsOutsideLineFragment:forGlyphRange:)]
        pub unsafe fn setDrawsOutsideLineFragment_forGlyphRange(
            &self,
            flag: bool,
            glyphRange: NSRange,
        );
        #[method(setLocation:withAdvancements:forStartOfGlyphRange:)]
        pub unsafe fn setLocation_withAdvancements_forStartOfGlyphRange(
            &self,
            location: NSPoint,
            advancements: *mut CGFloat,
            glyphRange: NSRange,
        );
        #[method(setAttachmentSize:forGlyphRange:)]
        pub unsafe fn setAttachmentSize_forGlyphRange(
            &self,
            attachmentSize: NSSize,
            glyphRange: NSRange,
        );
        #[method(setBidiLevels:forGlyphRange:)]
        pub unsafe fn setBidiLevels_forGlyphRange(&self, levels: *mut u8, glyphRange: NSRange);
    }
);
extern_methods!(
    #[doc = "NSTypesetter_Deprecated"]
    unsafe impl NSTypesetter {
        #[method(actionForControlCharacterAtIndex:)]
        pub unsafe fn actionForControlCharacterAtIndex(
            &self,
            charIndex: NSUInteger,
        ) -> NSTypesetterControlCharacterAction;
        #[method(getGlyphsInRange:glyphs:characterIndexes:glyphInscriptions:elasticBits:bidiLevels:)]
        pub unsafe fn getGlyphsInRange_glyphs_characterIndexes_glyphInscriptions_elasticBits_bidiLevels(
            &self,
            glyphsRange: NSRange,
            glyphBuffer: *mut NSGlyph,
            charIndexBuffer: *mut NSUInteger,
            inscribeBuffer: *mut NSGlyphInscription,
            elasticBuffer: *mut bool,
            bidiLevelBuffer: *mut c_uchar,
        ) -> NSUInteger;
        #[method(substituteGlyphsInRange:withGlyphs:)]
        pub unsafe fn substituteGlyphsInRange_withGlyphs(
            &self,
            glyphRange: NSRange,
            glyphs: *mut NSGlyph,
        );
        #[method(insertGlyph:atGlyphIndex:characterIndex:)]
        pub unsafe fn insertGlyph_atGlyphIndex_characterIndex(
            &self,
            glyph: NSGlyph,
            glyphIndex: NSUInteger,
            characterIndex: NSUInteger,
        );
        #[method(deleteGlyphsInRange:)]
        pub unsafe fn deleteGlyphsInRange(&self, glyphRange: NSRange);
    }
);
