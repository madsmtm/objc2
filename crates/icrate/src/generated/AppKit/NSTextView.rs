use super::__exported::NSLayoutManager;
use super::__exported::NSOrthography;
use super::__exported::NSParagraphStyle;
use super::__exported::NSRulerMarker;
use super::__exported::NSRulerView;
use super::__exported::NSSharingServicePicker;
use super::__exported::NSTextAttachment;
use super::__exported::NSTextAttachmentCell;
use super::__exported::NSTextContainer;
use super::__exported::NSTextContentStorage;
use super::__exported::NSTextLayoutManager;
use super::__exported::NSTextLayoutOrientationProvider;
use super::__exported::NSTextStorage;
use super::__exported::NSUndoManager;
use super::__exported::NSValue;
use super::__exported::QLPreviewItem;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSCandidateListTouchBarItem::*;
use crate::AppKit::generated::NSColorPanel::*;
use crate::AppKit::generated::NSDragging::*;
use crate::AppKit::generated::NSInputManager::*;
use crate::AppKit::generated::NSLayoutManager::*;
use crate::AppKit::generated::NSMenu::*;
use crate::AppKit::generated::NSNibDeclarations::*;
use crate::AppKit::generated::NSPasteboard::*;
use crate::AppKit::generated::NSSpellChecker::*;
use crate::AppKit::generated::NSText::*;
use crate::AppKit::generated::NSTextAttachment::*;
use crate::AppKit::generated::NSTextContent::*;
use crate::AppKit::generated::NSTextFinder::*;
use crate::AppKit::generated::NSTextInputClient::*;
use crate::AppKit::generated::NSTouchBarItem::*;
use crate::AppKit::generated::NSUserInterfaceValidation::*;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSDictionary::*;
use crate::Foundation::generated::NSTextCheckingResult::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTextView;
    unsafe impl ClassType for NSTextView {
        type Super = NSText;
    }
);
extern_methods!(
    unsafe impl NSTextView {
        #[method_id(initWithFrame:textContainer:)]
        pub unsafe fn initWithFrame_textContainer(
            &self,
            frameRect: NSRect,
            container: Option<&NSTextContainer>,
        ) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(initWithFrame:)]
        pub unsafe fn initWithFrame(&self, frameRect: NSRect) -> Id<Self, Shared>;
        #[method_id(textContainer)]
        pub unsafe fn textContainer(&self) -> Option<Id<NSTextContainer, Shared>>;
        #[method(setTextContainer:)]
        pub unsafe fn setTextContainer(&self, textContainer: Option<&NSTextContainer>);
        #[method(replaceTextContainer:)]
        pub unsafe fn replaceTextContainer(&self, newContainer: &NSTextContainer);
        #[method(textContainerInset)]
        pub unsafe fn textContainerInset(&self) -> NSSize;
        #[method(setTextContainerInset:)]
        pub unsafe fn setTextContainerInset(&self, textContainerInset: NSSize);
        #[method(textContainerOrigin)]
        pub unsafe fn textContainerOrigin(&self) -> NSPoint;
        #[method(invalidateTextContainerOrigin)]
        pub unsafe fn invalidateTextContainerOrigin(&self);
        #[method_id(layoutManager)]
        pub unsafe fn layoutManager(&self) -> Option<Id<NSLayoutManager, Shared>>;
        #[method_id(textStorage)]
        pub unsafe fn textStorage(&self) -> Option<Id<NSTextStorage, Shared>>;
        #[method_id(textLayoutManager)]
        pub unsafe fn textLayoutManager(&self) -> Option<Id<NSTextLayoutManager, Shared>>;
        #[method_id(textContentStorage)]
        pub unsafe fn textContentStorage(&self) -> Option<Id<NSTextContentStorage, Shared>>;
        #[method(insertText:)]
        pub unsafe fn insertText(&self, insertString: &Object);
        #[method(setConstrainedFrameSize:)]
        pub unsafe fn setConstrainedFrameSize(&self, desiredSize: NSSize);
        #[method(setAlignment:range:)]
        pub unsafe fn setAlignment_range(&self, alignment: NSTextAlignment, range: NSRange);
        #[method(setBaseWritingDirection:range:)]
        pub unsafe fn setBaseWritingDirection_range(
            &self,
            writingDirection: NSWritingDirection,
            range: NSRange,
        );
        #[method(turnOffKerning:)]
        pub unsafe fn turnOffKerning(&self, sender: Option<&Object>);
        #[method(tightenKerning:)]
        pub unsafe fn tightenKerning(&self, sender: Option<&Object>);
        #[method(loosenKerning:)]
        pub unsafe fn loosenKerning(&self, sender: Option<&Object>);
        #[method(useStandardKerning:)]
        pub unsafe fn useStandardKerning(&self, sender: Option<&Object>);
        #[method(turnOffLigatures:)]
        pub unsafe fn turnOffLigatures(&self, sender: Option<&Object>);
        #[method(useStandardLigatures:)]
        pub unsafe fn useStandardLigatures(&self, sender: Option<&Object>);
        #[method(useAllLigatures:)]
        pub unsafe fn useAllLigatures(&self, sender: Option<&Object>);
        #[method(raiseBaseline:)]
        pub unsafe fn raiseBaseline(&self, sender: Option<&Object>);
        #[method(lowerBaseline:)]
        pub unsafe fn lowerBaseline(&self, sender: Option<&Object>);
        #[method(toggleTraditionalCharacterShape:)]
        pub unsafe fn toggleTraditionalCharacterShape(&self, sender: Option<&Object>);
        #[method(outline:)]
        pub unsafe fn outline(&self, sender: Option<&Object>);
        #[method(performFindPanelAction:)]
        pub unsafe fn performFindPanelAction(&self, sender: Option<&Object>);
        #[method(alignJustified:)]
        pub unsafe fn alignJustified(&self, sender: Option<&Object>);
        #[method(changeColor:)]
        pub unsafe fn changeColor(&self, sender: Option<&Object>);
        #[method(changeAttributes:)]
        pub unsafe fn changeAttributes(&self, sender: Option<&Object>);
        #[method(changeDocumentBackgroundColor:)]
        pub unsafe fn changeDocumentBackgroundColor(&self, sender: Option<&Object>);
        #[method(orderFrontSpacingPanel:)]
        pub unsafe fn orderFrontSpacingPanel(&self, sender: Option<&Object>);
        #[method(orderFrontLinkPanel:)]
        pub unsafe fn orderFrontLinkPanel(&self, sender: Option<&Object>);
        #[method(orderFrontListPanel:)]
        pub unsafe fn orderFrontListPanel(&self, sender: Option<&Object>);
        #[method(orderFrontTablePanel:)]
        pub unsafe fn orderFrontTablePanel(&self, sender: Option<&Object>);
        #[method(rulerView:didMoveMarker:)]
        pub unsafe fn rulerView_didMoveMarker(&self, ruler: &NSRulerView, marker: &NSRulerMarker);
        #[method(rulerView:didRemoveMarker:)]
        pub unsafe fn rulerView_didRemoveMarker(&self, ruler: &NSRulerView, marker: &NSRulerMarker);
        #[method(rulerView:didAddMarker:)]
        pub unsafe fn rulerView_didAddMarker(&self, ruler: &NSRulerView, marker: &NSRulerMarker);
        #[method(rulerView:shouldMoveMarker:)]
        pub unsafe fn rulerView_shouldMoveMarker(
            &self,
            ruler: &NSRulerView,
            marker: &NSRulerMarker,
        ) -> bool;
        #[method(rulerView:shouldAddMarker:)]
        pub unsafe fn rulerView_shouldAddMarker(
            &self,
            ruler: &NSRulerView,
            marker: &NSRulerMarker,
        ) -> bool;
        #[method(rulerView:willMoveMarker:toLocation:)]
        pub unsafe fn rulerView_willMoveMarker_toLocation(
            &self,
            ruler: &NSRulerView,
            marker: &NSRulerMarker,
            location: CGFloat,
        ) -> CGFloat;
        #[method(rulerView:shouldRemoveMarker:)]
        pub unsafe fn rulerView_shouldRemoveMarker(
            &self,
            ruler: &NSRulerView,
            marker: &NSRulerMarker,
        ) -> bool;
        #[method(rulerView:willAddMarker:atLocation:)]
        pub unsafe fn rulerView_willAddMarker_atLocation(
            &self,
            ruler: &NSRulerView,
            marker: &NSRulerMarker,
            location: CGFloat,
        ) -> CGFloat;
        #[method(rulerView:handleMouseDown:)]
        pub unsafe fn rulerView_handleMouseDown(&self, ruler: &NSRulerView, event: &NSEvent);
        #[method(setNeedsDisplayInRect:avoidAdditionalLayout:)]
        pub unsafe fn setNeedsDisplayInRect_avoidAdditionalLayout(&self, rect: NSRect, flag: bool);
        #[method(shouldDrawInsertionPoint)]
        pub unsafe fn shouldDrawInsertionPoint(&self) -> bool;
        #[method(drawInsertionPointInRect:color:turnedOn:)]
        pub unsafe fn drawInsertionPointInRect_color_turnedOn(
            &self,
            rect: NSRect,
            color: &NSColor,
            flag: bool,
        );
        #[method(drawViewBackgroundInRect:)]
        pub unsafe fn drawViewBackgroundInRect(&self, rect: NSRect);
        #[method(updateRuler)]
        pub unsafe fn updateRuler(&self);
        #[method(updateFontPanel)]
        pub unsafe fn updateFontPanel(&self);
        #[method(updateDragTypeRegistration)]
        pub unsafe fn updateDragTypeRegistration(&self);
        #[method(selectionRangeForProposedRange:granularity:)]
        pub unsafe fn selectionRangeForProposedRange_granularity(
            &self,
            proposedCharRange: NSRange,
            granularity: NSSelectionGranularity,
        ) -> NSRange;
        #[method(clickedOnLink:atIndex:)]
        pub unsafe fn clickedOnLink_atIndex(&self, link: &Object, charIndex: NSUInteger);
        #[method(startSpeaking:)]
        pub unsafe fn startSpeaking(&self, sender: Option<&Object>);
        #[method(stopSpeaking:)]
        pub unsafe fn stopSpeaking(&self, sender: Option<&Object>);
        #[method(setLayoutOrientation:)]
        pub unsafe fn setLayoutOrientation(&self, orientation: NSTextLayoutOrientation);
        #[method(changeLayoutOrientation:)]
        pub unsafe fn changeLayoutOrientation(&self, sender: Option<&Object>);
        #[method(characterIndexForInsertionAtPoint:)]
        pub unsafe fn characterIndexForInsertionAtPoint(&self, point: NSPoint) -> NSUInteger;
        #[method(stronglyReferencesTextStorage)]
        pub unsafe fn stronglyReferencesTextStorage() -> bool;
        #[method(performValidatedReplacementInRange:withAttributedString:)]
        pub unsafe fn performValidatedReplacementInRange_withAttributedString(
            &self,
            range: NSRange,
            attributedString: &NSAttributedString,
        ) -> bool;
        #[method(usesAdaptiveColorMappingForDarkAppearance)]
        pub unsafe fn usesAdaptiveColorMappingForDarkAppearance(&self) -> bool;
        #[method(setUsesAdaptiveColorMappingForDarkAppearance:)]
        pub unsafe fn setUsesAdaptiveColorMappingForDarkAppearance(
            &self,
            usesAdaptiveColorMappingForDarkAppearance: bool,
        );
    }
);
extern_methods!(
    #[doc = "NSCompletion"]
    unsafe impl NSTextView {
        #[method(complete:)]
        pub unsafe fn complete(&self, sender: Option<&Object>);
        #[method(rangeForUserCompletion)]
        pub unsafe fn rangeForUserCompletion(&self) -> NSRange;
        #[method_id(completionsForPartialWordRange:indexOfSelectedItem:)]
        pub unsafe fn completionsForPartialWordRange_indexOfSelectedItem(
            &self,
            charRange: NSRange,
            index: NonNull<NSInteger>,
        ) -> Option<Id<NSArray<NSString>, Shared>>;
        #[method(insertCompletion:forPartialWordRange:movement:isFinal:)]
        pub unsafe fn insertCompletion_forPartialWordRange_movement_isFinal(
            &self,
            word: &NSString,
            charRange: NSRange,
            movement: NSInteger,
            flag: bool,
        );
    }
);
extern_methods!(
    #[doc = "NSPasteboard"]
    unsafe impl NSTextView {
        #[method_id(writablePasteboardTypes)]
        pub unsafe fn writablePasteboardTypes(&self) -> Id<NSArray<NSPasteboardType>, Shared>;
        #[method(writeSelectionToPasteboard:type:)]
        pub unsafe fn writeSelectionToPasteboard_type(
            &self,
            pboard: &NSPasteboard,
            type_: &NSPasteboardType,
        ) -> bool;
        #[method(writeSelectionToPasteboard:types:)]
        pub unsafe fn writeSelectionToPasteboard_types(
            &self,
            pboard: &NSPasteboard,
            types: &NSArray<NSPasteboardType>,
        ) -> bool;
        #[method_id(readablePasteboardTypes)]
        pub unsafe fn readablePasteboardTypes(&self) -> Id<NSArray<NSPasteboardType>, Shared>;
        #[method_id(preferredPasteboardTypeFromArray:restrictedToTypesFromArray:)]
        pub unsafe fn preferredPasteboardTypeFromArray_restrictedToTypesFromArray(
            &self,
            availableTypes: &NSArray<NSPasteboardType>,
            allowedTypes: Option<&NSArray<NSPasteboardType>>,
        ) -> Option<Id<NSPasteboardType, Shared>>;
        #[method(readSelectionFromPasteboard:type:)]
        pub unsafe fn readSelectionFromPasteboard_type(
            &self,
            pboard: &NSPasteboard,
            type_: &NSPasteboardType,
        ) -> bool;
        #[method(readSelectionFromPasteboard:)]
        pub unsafe fn readSelectionFromPasteboard(&self, pboard: &NSPasteboard) -> bool;
        #[method(registerForServices)]
        pub unsafe fn registerForServices();
        #[method_id(validRequestorForSendType:returnType:)]
        pub unsafe fn validRequestorForSendType_returnType(
            &self,
            sendType: Option<&NSPasteboardType>,
            returnType: Option<&NSPasteboardType>,
        ) -> Option<Id<Object, Shared>>;
        #[method(pasteAsPlainText:)]
        pub unsafe fn pasteAsPlainText(&self, sender: Option<&Object>);
        #[method(pasteAsRichText:)]
        pub unsafe fn pasteAsRichText(&self, sender: Option<&Object>);
    }
);
extern_methods!(
    #[doc = "NSDragging"]
    unsafe impl NSTextView {
        #[method(dragSelectionWithEvent:offset:slideBack:)]
        pub unsafe fn dragSelectionWithEvent_offset_slideBack(
            &self,
            event: &NSEvent,
            mouseOffset: NSSize,
            slideBack: bool,
        ) -> bool;
        #[method_id(dragImageForSelectionWithEvent:origin:)]
        pub unsafe fn dragImageForSelectionWithEvent_origin(
            &self,
            event: &NSEvent,
            origin: NSPointPointer,
        ) -> Option<Id<NSImage, Shared>>;
        #[method_id(acceptableDragTypes)]
        pub unsafe fn acceptableDragTypes(&self) -> Id<NSArray<NSPasteboardType>, Shared>;
        #[method(dragOperationForDraggingInfo:type:)]
        pub unsafe fn dragOperationForDraggingInfo_type(
            &self,
            dragInfo: &NSDraggingInfo,
            type_: &NSPasteboardType,
        ) -> NSDragOperation;
        #[method(cleanUpAfterDragOperation)]
        pub unsafe fn cleanUpAfterDragOperation(&self);
    }
);
extern_methods!(
    #[doc = "NSSharing"]
    unsafe impl NSTextView {
        #[method_id(selectedRanges)]
        pub unsafe fn selectedRanges(&self) -> Id<NSArray<NSValue>, Shared>;
        #[method(setSelectedRanges:)]
        pub unsafe fn setSelectedRanges(&self, selectedRanges: &NSArray<NSValue>);
        #[method(setSelectedRanges:affinity:stillSelecting:)]
        pub unsafe fn setSelectedRanges_affinity_stillSelecting(
            &self,
            ranges: &NSArray<NSValue>,
            affinity: NSSelectionAffinity,
            stillSelectingFlag: bool,
        );
        #[method(setSelectedRange:affinity:stillSelecting:)]
        pub unsafe fn setSelectedRange_affinity_stillSelecting(
            &self,
            charRange: NSRange,
            affinity: NSSelectionAffinity,
            stillSelectingFlag: bool,
        );
        #[method(selectionAffinity)]
        pub unsafe fn selectionAffinity(&self) -> NSSelectionAffinity;
        #[method(selectionGranularity)]
        pub unsafe fn selectionGranularity(&self) -> NSSelectionGranularity;
        #[method(setSelectionGranularity:)]
        pub unsafe fn setSelectionGranularity(&self, selectionGranularity: NSSelectionGranularity);
        #[method_id(selectedTextAttributes)]
        pub unsafe fn selectedTextAttributes(
            &self,
        ) -> Id<NSDictionary<NSAttributedStringKey, Object>, Shared>;
        #[method(setSelectedTextAttributes:)]
        pub unsafe fn setSelectedTextAttributes(
            &self,
            selectedTextAttributes: &NSDictionary<NSAttributedStringKey, Object>,
        );
        #[method_id(insertionPointColor)]
        pub unsafe fn insertionPointColor(&self) -> Id<NSColor, Shared>;
        #[method(setInsertionPointColor:)]
        pub unsafe fn setInsertionPointColor(&self, insertionPointColor: &NSColor);
        #[method(updateInsertionPointStateAndRestartTimer:)]
        pub unsafe fn updateInsertionPointStateAndRestartTimer(&self, restartFlag: bool);
        #[method_id(markedTextAttributes)]
        pub unsafe fn markedTextAttributes(
            &self,
        ) -> Option<Id<NSDictionary<NSAttributedStringKey, Object>, Shared>>;
        #[method(setMarkedTextAttributes:)]
        pub unsafe fn setMarkedTextAttributes(
            &self,
            markedTextAttributes: Option<&NSDictionary<NSAttributedStringKey, Object>>,
        );
        #[method_id(linkTextAttributes)]
        pub unsafe fn linkTextAttributes(
            &self,
        ) -> Option<Id<NSDictionary<NSAttributedStringKey, Object>, Shared>>;
        #[method(setLinkTextAttributes:)]
        pub unsafe fn setLinkTextAttributes(
            &self,
            linkTextAttributes: Option<&NSDictionary<NSAttributedStringKey, Object>>,
        );
        #[method(displaysLinkToolTips)]
        pub unsafe fn displaysLinkToolTips(&self) -> bool;
        #[method(setDisplaysLinkToolTips:)]
        pub unsafe fn setDisplaysLinkToolTips(&self, displaysLinkToolTips: bool);
        #[method(acceptsGlyphInfo)]
        pub unsafe fn acceptsGlyphInfo(&self) -> bool;
        #[method(setAcceptsGlyphInfo:)]
        pub unsafe fn setAcceptsGlyphInfo(&self, acceptsGlyphInfo: bool);
        #[method(usesRuler)]
        pub unsafe fn usesRuler(&self) -> bool;
        #[method(setUsesRuler:)]
        pub unsafe fn setUsesRuler(&self, usesRuler: bool);
        #[method(usesInspectorBar)]
        pub unsafe fn usesInspectorBar(&self) -> bool;
        #[method(setUsesInspectorBar:)]
        pub unsafe fn setUsesInspectorBar(&self, usesInspectorBar: bool);
        #[method(isContinuousSpellCheckingEnabled)]
        pub unsafe fn isContinuousSpellCheckingEnabled(&self) -> bool;
        #[method(setContinuousSpellCheckingEnabled:)]
        pub unsafe fn setContinuousSpellCheckingEnabled(
            &self,
            continuousSpellCheckingEnabled: bool,
        );
        #[method(toggleContinuousSpellChecking:)]
        pub unsafe fn toggleContinuousSpellChecking(&self, sender: Option<&Object>);
        #[method(spellCheckerDocumentTag)]
        pub unsafe fn spellCheckerDocumentTag(&self) -> NSInteger;
        #[method(isGrammarCheckingEnabled)]
        pub unsafe fn isGrammarCheckingEnabled(&self) -> bool;
        #[method(setGrammarCheckingEnabled:)]
        pub unsafe fn setGrammarCheckingEnabled(&self, grammarCheckingEnabled: bool);
        #[method(toggleGrammarChecking:)]
        pub unsafe fn toggleGrammarChecking(&self, sender: Option<&Object>);
        #[method(setSpellingState:range:)]
        pub unsafe fn setSpellingState_range(&self, value: NSInteger, charRange: NSRange);
        #[method_id(typingAttributes)]
        pub unsafe fn typingAttributes(
            &self,
        ) -> Id<NSDictionary<NSAttributedStringKey, Object>, Shared>;
        #[method(setTypingAttributes:)]
        pub unsafe fn setTypingAttributes(
            &self,
            typingAttributes: &NSDictionary<NSAttributedStringKey, Object>,
        );
        #[method(shouldChangeTextInRanges:replacementStrings:)]
        pub unsafe fn shouldChangeTextInRanges_replacementStrings(
            &self,
            affectedRanges: &NSArray<NSValue>,
            replacementStrings: Option<&NSArray<NSString>>,
        ) -> bool;
        #[method_id(rangesForUserTextChange)]
        pub unsafe fn rangesForUserTextChange(&self) -> Option<Id<NSArray<NSValue>, Shared>>;
        #[method_id(rangesForUserCharacterAttributeChange)]
        pub unsafe fn rangesForUserCharacterAttributeChange(
            &self,
        ) -> Option<Id<NSArray<NSValue>, Shared>>;
        #[method_id(rangesForUserParagraphAttributeChange)]
        pub unsafe fn rangesForUserParagraphAttributeChange(
            &self,
        ) -> Option<Id<NSArray<NSValue>, Shared>>;
        #[method(shouldChangeTextInRange:replacementString:)]
        pub unsafe fn shouldChangeTextInRange_replacementString(
            &self,
            affectedCharRange: NSRange,
            replacementString: Option<&NSString>,
        ) -> bool;
        #[method(didChangeText)]
        pub unsafe fn didChangeText(&self);
        #[method(rangeForUserTextChange)]
        pub unsafe fn rangeForUserTextChange(&self) -> NSRange;
        #[method(rangeForUserCharacterAttributeChange)]
        pub unsafe fn rangeForUserCharacterAttributeChange(&self) -> NSRange;
        #[method(rangeForUserParagraphAttributeChange)]
        pub unsafe fn rangeForUserParagraphAttributeChange(&self) -> NSRange;
        #[method(allowsDocumentBackgroundColorChange)]
        pub unsafe fn allowsDocumentBackgroundColorChange(&self) -> bool;
        #[method(setAllowsDocumentBackgroundColorChange:)]
        pub unsafe fn setAllowsDocumentBackgroundColorChange(
            &self,
            allowsDocumentBackgroundColorChange: bool,
        );
        #[method_id(defaultParagraphStyle)]
        pub unsafe fn defaultParagraphStyle(&self) -> Option<Id<NSParagraphStyle, Shared>>;
        #[method(setDefaultParagraphStyle:)]
        pub unsafe fn setDefaultParagraphStyle(
            &self,
            defaultParagraphStyle: Option<&NSParagraphStyle>,
        );
        #[method(allowsUndo)]
        pub unsafe fn allowsUndo(&self) -> bool;
        #[method(setAllowsUndo:)]
        pub unsafe fn setAllowsUndo(&self, allowsUndo: bool);
        #[method(breakUndoCoalescing)]
        pub unsafe fn breakUndoCoalescing(&self);
        #[method(isCoalescingUndo)]
        pub unsafe fn isCoalescingUndo(&self) -> bool;
        #[method(allowsImageEditing)]
        pub unsafe fn allowsImageEditing(&self) -> bool;
        #[method(setAllowsImageEditing:)]
        pub unsafe fn setAllowsImageEditing(&self, allowsImageEditing: bool);
        #[method(showFindIndicatorForRange:)]
        pub unsafe fn showFindIndicatorForRange(&self, charRange: NSRange);
        #[method(usesRolloverButtonForSelection)]
        pub unsafe fn usesRolloverButtonForSelection(&self) -> bool;
        #[method(setUsesRolloverButtonForSelection:)]
        pub unsafe fn setUsesRolloverButtonForSelection(
            &self,
            usesRolloverButtonForSelection: bool,
        );
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSTextViewDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSTextViewDelegate>);
        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;
        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);
        #[method(isSelectable)]
        pub unsafe fn isSelectable(&self) -> bool;
        #[method(setSelectable:)]
        pub unsafe fn setSelectable(&self, selectable: bool);
        #[method(isRichText)]
        pub unsafe fn isRichText(&self) -> bool;
        #[method(setRichText:)]
        pub unsafe fn setRichText(&self, richText: bool);
        #[method(importsGraphics)]
        pub unsafe fn importsGraphics(&self) -> bool;
        #[method(setImportsGraphics:)]
        pub unsafe fn setImportsGraphics(&self, importsGraphics: bool);
        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;
        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, drawsBackground: bool);
        #[method_id(backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<NSColor, Shared>;
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, backgroundColor: &NSColor);
        #[method(isFieldEditor)]
        pub unsafe fn isFieldEditor(&self) -> bool;
        #[method(setFieldEditor:)]
        pub unsafe fn setFieldEditor(&self, fieldEditor: bool);
        #[method(usesFontPanel)]
        pub unsafe fn usesFontPanel(&self) -> bool;
        #[method(setUsesFontPanel:)]
        pub unsafe fn setUsesFontPanel(&self, usesFontPanel: bool);
        #[method(isRulerVisible)]
        pub unsafe fn isRulerVisible(&self) -> bool;
        #[method(setRulerVisible:)]
        pub unsafe fn setRulerVisible(&self, rulerVisible: bool);
        #[method(setSelectedRange:)]
        pub unsafe fn setSelectedRange(&self, charRange: NSRange);
        #[method_id(allowedInputSourceLocales)]
        pub unsafe fn allowedInputSourceLocales(&self) -> Option<Id<NSArray<NSString>, Shared>>;
        #[method(setAllowedInputSourceLocales:)]
        pub unsafe fn setAllowedInputSourceLocales(
            &self,
            allowedInputSourceLocales: Option<&NSArray<NSString>>,
        );
    }
);
extern_methods!(
    #[doc = "NSTextChecking"]
    unsafe impl NSTextView {
        #[method(smartInsertDeleteEnabled)]
        pub unsafe fn smartInsertDeleteEnabled(&self) -> bool;
        #[method(setSmartInsertDeleteEnabled:)]
        pub unsafe fn setSmartInsertDeleteEnabled(&self, smartInsertDeleteEnabled: bool);
        #[method(smartDeleteRangeForProposedRange:)]
        pub unsafe fn smartDeleteRangeForProposedRange(
            &self,
            proposedCharRange: NSRange,
        ) -> NSRange;
        #[method(toggleSmartInsertDelete:)]
        pub unsafe fn toggleSmartInsertDelete(&self, sender: Option<&Object>);
        #[method(smartInsertForString:replacingRange:beforeString:afterString:)]
        pub unsafe fn smartInsertForString_replacingRange_beforeString_afterString(
            &self,
            pasteString: &NSString,
            charRangeToReplace: NSRange,
            beforeString: Option<&mut Option<Id<NSString, Shared>>>,
            afterString: Option<&mut Option<Id<NSString, Shared>>>,
        );
        #[method_id(smartInsertBeforeStringForString:replacingRange:)]
        pub unsafe fn smartInsertBeforeStringForString_replacingRange(
            &self,
            pasteString: &NSString,
            charRangeToReplace: NSRange,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(smartInsertAfterStringForString:replacingRange:)]
        pub unsafe fn smartInsertAfterStringForString_replacingRange(
            &self,
            pasteString: &NSString,
            charRangeToReplace: NSRange,
        ) -> Option<Id<NSString, Shared>>;
        #[method(isAutomaticQuoteSubstitutionEnabled)]
        pub unsafe fn isAutomaticQuoteSubstitutionEnabled(&self) -> bool;
        #[method(setAutomaticQuoteSubstitutionEnabled:)]
        pub unsafe fn setAutomaticQuoteSubstitutionEnabled(
            &self,
            automaticQuoteSubstitutionEnabled: bool,
        );
        #[method(toggleAutomaticQuoteSubstitution:)]
        pub unsafe fn toggleAutomaticQuoteSubstitution(&self, sender: Option<&Object>);
        #[method(isAutomaticLinkDetectionEnabled)]
        pub unsafe fn isAutomaticLinkDetectionEnabled(&self) -> bool;
        #[method(setAutomaticLinkDetectionEnabled:)]
        pub unsafe fn setAutomaticLinkDetectionEnabled(&self, automaticLinkDetectionEnabled: bool);
        #[method(toggleAutomaticLinkDetection:)]
        pub unsafe fn toggleAutomaticLinkDetection(&self, sender: Option<&Object>);
        #[method(isAutomaticDataDetectionEnabled)]
        pub unsafe fn isAutomaticDataDetectionEnabled(&self) -> bool;
        #[method(setAutomaticDataDetectionEnabled:)]
        pub unsafe fn setAutomaticDataDetectionEnabled(&self, automaticDataDetectionEnabled: bool);
        #[method(toggleAutomaticDataDetection:)]
        pub unsafe fn toggleAutomaticDataDetection(&self, sender: Option<&Object>);
        #[method(isAutomaticDashSubstitutionEnabled)]
        pub unsafe fn isAutomaticDashSubstitutionEnabled(&self) -> bool;
        #[method(setAutomaticDashSubstitutionEnabled:)]
        pub unsafe fn setAutomaticDashSubstitutionEnabled(
            &self,
            automaticDashSubstitutionEnabled: bool,
        );
        #[method(toggleAutomaticDashSubstitution:)]
        pub unsafe fn toggleAutomaticDashSubstitution(&self, sender: Option<&Object>);
        #[method(isAutomaticTextReplacementEnabled)]
        pub unsafe fn isAutomaticTextReplacementEnabled(&self) -> bool;
        #[method(setAutomaticTextReplacementEnabled:)]
        pub unsafe fn setAutomaticTextReplacementEnabled(
            &self,
            automaticTextReplacementEnabled: bool,
        );
        #[method(toggleAutomaticTextReplacement:)]
        pub unsafe fn toggleAutomaticTextReplacement(&self, sender: Option<&Object>);
        #[method(isAutomaticSpellingCorrectionEnabled)]
        pub unsafe fn isAutomaticSpellingCorrectionEnabled(&self) -> bool;
        #[method(setAutomaticSpellingCorrectionEnabled:)]
        pub unsafe fn setAutomaticSpellingCorrectionEnabled(
            &self,
            automaticSpellingCorrectionEnabled: bool,
        );
        #[method(toggleAutomaticSpellingCorrection:)]
        pub unsafe fn toggleAutomaticSpellingCorrection(&self, sender: Option<&Object>);
        #[method(enabledTextCheckingTypes)]
        pub unsafe fn enabledTextCheckingTypes(&self) -> NSTextCheckingTypes;
        #[method(setEnabledTextCheckingTypes:)]
        pub unsafe fn setEnabledTextCheckingTypes(
            &self,
            enabledTextCheckingTypes: NSTextCheckingTypes,
        );
        #[method(checkTextInRange:types:options:)]
        pub unsafe fn checkTextInRange_types_options(
            &self,
            range: NSRange,
            checkingTypes: NSTextCheckingTypes,
            options: &NSDictionary<NSTextCheckingOptionKey, Object>,
        );
        #[method(handleTextCheckingResults:forRange:types:options:orthography:wordCount:)]
        pub unsafe fn handleTextCheckingResults_forRange_types_options_orthography_wordCount(
            &self,
            results: &NSArray<NSTextCheckingResult>,
            range: NSRange,
            checkingTypes: NSTextCheckingTypes,
            options: &NSDictionary<NSTextCheckingOptionKey, Object>,
            orthography: &NSOrthography,
            wordCount: NSInteger,
        );
        #[method(orderFrontSubstitutionsPanel:)]
        pub unsafe fn orderFrontSubstitutionsPanel(&self, sender: Option<&Object>);
        #[method(checkTextInSelection:)]
        pub unsafe fn checkTextInSelection(&self, sender: Option<&Object>);
        #[method(checkTextInDocument:)]
        pub unsafe fn checkTextInDocument(&self, sender: Option<&Object>);
        #[method(usesFindPanel)]
        pub unsafe fn usesFindPanel(&self) -> bool;
        #[method(setUsesFindPanel:)]
        pub unsafe fn setUsesFindPanel(&self, usesFindPanel: bool);
        #[method(usesFindBar)]
        pub unsafe fn usesFindBar(&self) -> bool;
        #[method(setUsesFindBar:)]
        pub unsafe fn setUsesFindBar(&self, usesFindBar: bool);
        #[method(isIncrementalSearchingEnabled)]
        pub unsafe fn isIncrementalSearchingEnabled(&self) -> bool;
        #[method(setIncrementalSearchingEnabled:)]
        pub unsafe fn setIncrementalSearchingEnabled(&self, incrementalSearchingEnabled: bool);
    }
);
extern_methods!(
    #[doc = "NSQuickLookPreview"]
    unsafe impl NSTextView {
        #[method(toggleQuickLookPreviewPanel:)]
        pub unsafe fn toggleQuickLookPreviewPanel(&self, sender: Option<&Object>);
        #[method_id(quickLookPreviewableItemsInRanges:)]
        pub unsafe fn quickLookPreviewableItemsInRanges(
            &self,
            ranges: &NSArray<NSValue>,
        ) -> Id<NSArray<QLPreviewItem>, Shared>;
        #[method(updateQuickLookPreviewPanel)]
        pub unsafe fn updateQuickLookPreviewPanel(&self);
    }
);
extern_methods!(
    #[doc = "NSTextView_SharingService"]
    unsafe impl NSTextView {
        #[method(orderFrontSharingServicePicker:)]
        pub unsafe fn orderFrontSharingServicePicker(&self, sender: Option<&Object>);
    }
);
extern_methods!(
    #[doc = "NSTextView_TouchBar"]
    unsafe impl NSTextView {
        #[method(isAutomaticTextCompletionEnabled)]
        pub unsafe fn isAutomaticTextCompletionEnabled(&self) -> bool;
        #[method(setAutomaticTextCompletionEnabled:)]
        pub unsafe fn setAutomaticTextCompletionEnabled(
            &self,
            automaticTextCompletionEnabled: bool,
        );
        #[method(toggleAutomaticTextCompletion:)]
        pub unsafe fn toggleAutomaticTextCompletion(&self, sender: Option<&Object>);
        #[method(allowsCharacterPickerTouchBarItem)]
        pub unsafe fn allowsCharacterPickerTouchBarItem(&self) -> bool;
        #[method(setAllowsCharacterPickerTouchBarItem:)]
        pub unsafe fn setAllowsCharacterPickerTouchBarItem(
            &self,
            allowsCharacterPickerTouchBarItem: bool,
        );
        #[method(updateTouchBarItemIdentifiers)]
        pub unsafe fn updateTouchBarItemIdentifiers(&self);
        #[method(updateTextTouchBarItems)]
        pub unsafe fn updateTextTouchBarItems(&self);
        #[method(updateCandidates)]
        pub unsafe fn updateCandidates(&self);
        #[method_id(candidateListTouchBarItem)]
        pub unsafe fn candidateListTouchBarItem(
            &self,
        ) -> Option<Id<NSCandidateListTouchBarItem, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSTextView_Factory"]
    unsafe impl NSTextView {
        #[method_id(scrollableTextView)]
        pub unsafe fn scrollableTextView() -> Id<NSScrollView, Shared>;
        #[method_id(fieldEditor)]
        pub unsafe fn fieldEditor() -> Id<Self, Shared>;
        #[method_id(scrollableDocumentContentTextView)]
        pub unsafe fn scrollableDocumentContentTextView() -> Id<NSScrollView, Shared>;
        #[method_id(scrollablePlainDocumentContentTextView)]
        pub unsafe fn scrollablePlainDocumentContentTextView() -> Id<NSScrollView, Shared>;
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSTextView {
        #[method(toggleBaseWritingDirection:)]
        pub unsafe fn toggleBaseWritingDirection(&self, sender: Option<&Object>);
    }
);
pub type NSTextViewDelegate = NSObject;
pub type NSPasteboardTypeFindPanelSearchOptionKey = NSString;
