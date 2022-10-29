#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSATSTypesetter;
    unsafe impl ClassType for NSATSTypesetter {
        type Super = NSTypesetter;
    }
);
extern_methods!(
    unsafe impl NSATSTypesetter {
        #[method_id(sharedTypesetter)]
        pub unsafe fn sharedTypesetter() -> Id<NSATSTypesetter, Shared>;
    }
);
extern_methods!(
    #[doc = "NSPantherCompatibility"]
    unsafe impl NSATSTypesetter {
        #[method(lineFragmentRectForProposedRect:remainingRect:)]
        pub unsafe fn lineFragmentRectForProposedRect_remainingRect(
            &self,
            proposedRect: NSRect,
            remainingRect: NSRectPointer,
        ) -> NSRect;
    }
);
extern_methods!(
    #[doc = "NSPrimitiveInterface"]
    unsafe impl NSATSTypesetter {
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
        #[method(layoutParagraphAtPoint:)]
        pub unsafe fn layoutParagraphAtPoint(
            &self,
            lineFragmentOrigin: NonNull<NSPoint>,
        ) -> NSUInteger;
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
        #[method_id(layoutManager)]
        pub unsafe fn layoutManager(&self) -> Option<Id<NSLayoutManager, Shared>>;
        #[method_id(currentTextContainer)]
        pub unsafe fn currentTextContainer(&self) -> Option<Id<NSTextContainer, Shared>>;
        #[method(setHardInvalidation:forGlyphRange:)]
        pub unsafe fn setHardInvalidation_forGlyphRange(&self, flag: bool, glyphRange: NSRange);
        #[method(getLineFragmentRect:usedRect:forParagraphSeparatorGlyphRange:atProposedOrigin:)]
        pub unsafe fn getLineFragmentRect_usedRect_forParagraphSeparatorGlyphRange_atProposedOrigin(
            &self,
            lineFragmentRect: NonNull<NSRect>,
            lineFragmentUsedRect: NonNull<NSRect>,
            paragraphSeparatorGlyphRange: NSRange,
            lineOrigin: NSPoint,
        );
    }
);
extern_methods!(
    #[doc = "NSLayoutPhaseInterface"]
    unsafe impl NSATSTypesetter {
        #[method(willSetLineFragmentRect:forGlyphRange:usedRect:baselineOffset:)]
        pub unsafe fn willSetLineFragmentRect_forGlyphRange_usedRect_baselineOffset(
            &self,
            lineRect: NonNull<NSRect>,
            glyphRange: NSRange,
            usedRect: NonNull<NSRect>,
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
    unsafe impl NSATSTypesetter {
        #[method(getGlyphsInRange:glyphs:characterIndexes:glyphInscriptions:elasticBits:)]
        pub unsafe fn getGlyphsInRange_glyphs_characterIndexes_glyphInscriptions_elasticBits(
            &self,
            glyphsRange: NSRange,
            glyphBuffer: *mut NSGlyph,
            charIndexBuffer: *mut NSUInteger,
            inscribeBuffer: *mut NSGlyphInscription,
            elasticBuffer: *mut bool,
        ) -> NSUInteger;
    }
);
