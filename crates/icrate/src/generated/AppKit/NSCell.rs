#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSControlStateValue = NSInteger;
extern_class!(
    #[derive(Debug)]
    pub struct NSCell;
    unsafe impl ClassType for NSCell {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSCell {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(initTextCell:)]
        pub unsafe fn initTextCell(&self, string: &NSString) -> Id<Self, Shared>;
        #[method_id(initImageCell:)]
        pub unsafe fn initImageCell(&self, image: Option<&NSImage>) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Id<Self, Shared>;
        #[method(prefersTrackingUntilMouseUp)]
        pub unsafe fn prefersTrackingUntilMouseUp() -> bool;
        #[method_id(controlView)]
        pub unsafe fn controlView(&self) -> Option<Id<NSView, Shared>>;
        #[method(setControlView:)]
        pub unsafe fn setControlView(&self, controlView: Option<&NSView>);
        #[method(type)]
        pub unsafe fn type_(&self) -> NSCellType;
        #[method(setType:)]
        pub unsafe fn setType(&self, type_: NSCellType);
        #[method(state)]
        pub unsafe fn state(&self) -> NSControlStateValue;
        #[method(setState:)]
        pub unsafe fn setState(&self, state: NSControlStateValue);
        #[method_id(target)]
        pub unsafe fn target(&self) -> Option<Id<Object, Shared>>;
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&Object>);
        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;
        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);
        #[method(tag)]
        pub unsafe fn tag(&self) -> NSInteger;
        #[method(setTag:)]
        pub unsafe fn setTag(&self, tag: NSInteger);
        #[method_id(title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);
        #[method(isOpaque)]
        pub unsafe fn isOpaque(&self) -> bool;
        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;
        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);
        #[method(sendActionOn:)]
        pub unsafe fn sendActionOn(&self, mask: NSEventMask) -> NSInteger;
        #[method(isContinuous)]
        pub unsafe fn isContinuous(&self) -> bool;
        #[method(setContinuous:)]
        pub unsafe fn setContinuous(&self, continuous: bool);
        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;
        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);
        #[method(isSelectable)]
        pub unsafe fn isSelectable(&self) -> bool;
        #[method(setSelectable:)]
        pub unsafe fn setSelectable(&self, selectable: bool);
        #[method(isBordered)]
        pub unsafe fn isBordered(&self) -> bool;
        #[method(setBordered:)]
        pub unsafe fn setBordered(&self, bordered: bool);
        #[method(isBezeled)]
        pub unsafe fn isBezeled(&self) -> bool;
        #[method(setBezeled:)]
        pub unsafe fn setBezeled(&self, bezeled: bool);
        #[method(isScrollable)]
        pub unsafe fn isScrollable(&self) -> bool;
        #[method(setScrollable:)]
        pub unsafe fn setScrollable(&self, scrollable: bool);
        #[method(isHighlighted)]
        pub unsafe fn isHighlighted(&self) -> bool;
        #[method(setHighlighted:)]
        pub unsafe fn setHighlighted(&self, highlighted: bool);
        #[method(alignment)]
        pub unsafe fn alignment(&self) -> NSTextAlignment;
        #[method(setAlignment:)]
        pub unsafe fn setAlignment(&self, alignment: NSTextAlignment);
        #[method(wraps)]
        pub unsafe fn wraps(&self) -> bool;
        #[method(setWraps:)]
        pub unsafe fn setWraps(&self, wraps: bool);
        #[method_id(font)]
        pub unsafe fn font(&self) -> Option<Id<NSFont, Shared>>;
        #[method(setFont:)]
        pub unsafe fn setFont(&self, font: Option<&NSFont>);
        #[method_id(keyEquivalent)]
        pub unsafe fn keyEquivalent(&self) -> Id<NSString, Shared>;
        #[method_id(formatter)]
        pub unsafe fn formatter(&self) -> Option<Id<NSFormatter, Shared>>;
        #[method(setFormatter:)]
        pub unsafe fn setFormatter(&self, formatter: Option<&NSFormatter>);
        #[method_id(objectValue)]
        pub unsafe fn objectValue(&self) -> Option<Id<Object, Shared>>;
        #[method(setObjectValue:)]
        pub unsafe fn setObjectValue(&self, objectValue: Option<&Object>);
        #[method(hasValidObjectValue)]
        pub unsafe fn hasValidObjectValue(&self) -> bool;
        #[method_id(stringValue)]
        pub unsafe fn stringValue(&self) -> Id<NSString, Shared>;
        #[method(setStringValue:)]
        pub unsafe fn setStringValue(&self, stringValue: &NSString);
        #[method(compare:)]
        pub unsafe fn compare(&self, otherCell: &Object) -> NSComparisonResult;
        #[method(intValue)]
        pub unsafe fn intValue(&self) -> c_int;
        #[method(setIntValue:)]
        pub unsafe fn setIntValue(&self, intValue: c_int);
        #[method(floatValue)]
        pub unsafe fn floatValue(&self) -> c_float;
        #[method(setFloatValue:)]
        pub unsafe fn setFloatValue(&self, floatValue: c_float);
        #[method(doubleValue)]
        pub unsafe fn doubleValue(&self) -> c_double;
        #[method(setDoubleValue:)]
        pub unsafe fn setDoubleValue(&self, doubleValue: c_double);
        #[method(integerValue)]
        pub unsafe fn integerValue(&self) -> NSInteger;
        #[method(setIntegerValue:)]
        pub unsafe fn setIntegerValue(&self, integerValue: NSInteger);
        #[method(takeIntValueFrom:)]
        pub unsafe fn takeIntValueFrom(&self, sender: Option<&Object>);
        #[method(takeFloatValueFrom:)]
        pub unsafe fn takeFloatValueFrom(&self, sender: Option<&Object>);
        #[method(takeDoubleValueFrom:)]
        pub unsafe fn takeDoubleValueFrom(&self, sender: Option<&Object>);
        #[method(takeStringValueFrom:)]
        pub unsafe fn takeStringValueFrom(&self, sender: Option<&Object>);
        #[method(takeObjectValueFrom:)]
        pub unsafe fn takeObjectValueFrom(&self, sender: Option<&Object>);
        #[method(takeIntegerValueFrom:)]
        pub unsafe fn takeIntegerValueFrom(&self, sender: Option<&Object>);
        #[method_id(image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage, Shared>>;
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);
        #[method(controlSize)]
        pub unsafe fn controlSize(&self) -> NSControlSize;
        #[method(setControlSize:)]
        pub unsafe fn setControlSize(&self, controlSize: NSControlSize);
        #[method_id(representedObject)]
        pub unsafe fn representedObject(&self) -> Option<Id<Object, Shared>>;
        #[method(setRepresentedObject:)]
        pub unsafe fn setRepresentedObject(&self, representedObject: Option<&Object>);
        #[method(cellAttribute:)]
        pub unsafe fn cellAttribute(&self, parameter: NSCellAttribute) -> NSInteger;
        #[method(setCellAttribute:to:)]
        pub unsafe fn setCellAttribute_to(&self, parameter: NSCellAttribute, value: NSInteger);
        #[method(imageRectForBounds:)]
        pub unsafe fn imageRectForBounds(&self, rect: NSRect) -> NSRect;
        #[method(titleRectForBounds:)]
        pub unsafe fn titleRectForBounds(&self, rect: NSRect) -> NSRect;
        #[method(drawingRectForBounds:)]
        pub unsafe fn drawingRectForBounds(&self, rect: NSRect) -> NSRect;
        #[method(cellSize)]
        pub unsafe fn cellSize(&self) -> NSSize;
        #[method(cellSizeForBounds:)]
        pub unsafe fn cellSizeForBounds(&self, rect: NSRect) -> NSSize;
        #[method_id(highlightColorWithFrame:inView:)]
        pub unsafe fn highlightColorWithFrame_inView(
            &self,
            cellFrame: NSRect,
            controlView: &NSView,
        ) -> Option<Id<NSColor, Shared>>;
        #[method(calcDrawInfo:)]
        pub unsafe fn calcDrawInfo(&self, rect: NSRect);
        #[method_id(setUpFieldEditorAttributes:)]
        pub unsafe fn setUpFieldEditorAttributes(&self, textObj: &NSText) -> Id<NSText, Shared>;
        #[method(drawInteriorWithFrame:inView:)]
        pub unsafe fn drawInteriorWithFrame_inView(&self, cellFrame: NSRect, controlView: &NSView);
        #[method(drawWithFrame:inView:)]
        pub unsafe fn drawWithFrame_inView(&self, cellFrame: NSRect, controlView: &NSView);
        #[method(highlight:withFrame:inView:)]
        pub unsafe fn highlight_withFrame_inView(
            &self,
            flag: bool,
            cellFrame: NSRect,
            controlView: &NSView,
        );
        #[method(mouseDownFlags)]
        pub unsafe fn mouseDownFlags(&self) -> NSInteger;
        #[method(getPeriodicDelay:interval:)]
        pub unsafe fn getPeriodicDelay_interval(
            &self,
            delay: NonNull<c_float>,
            interval: NonNull<c_float>,
        );
        #[method(startTrackingAt:inView:)]
        pub unsafe fn startTrackingAt_inView(
            &self,
            startPoint: NSPoint,
            controlView: &NSView,
        ) -> bool;
        #[method(continueTracking:at:inView:)]
        pub unsafe fn continueTracking_at_inView(
            &self,
            lastPoint: NSPoint,
            currentPoint: NSPoint,
            controlView: &NSView,
        ) -> bool;
        #[method(stopTracking:at:inView:mouseIsUp:)]
        pub unsafe fn stopTracking_at_inView_mouseIsUp(
            &self,
            lastPoint: NSPoint,
            stopPoint: NSPoint,
            controlView: &NSView,
            flag: bool,
        );
        #[method(trackMouse:inRect:ofView:untilMouseUp:)]
        pub unsafe fn trackMouse_inRect_ofView_untilMouseUp(
            &self,
            event: &NSEvent,
            cellFrame: NSRect,
            controlView: &NSView,
            flag: bool,
        ) -> bool;
        #[method(editWithFrame:inView:editor:delegate:event:)]
        pub unsafe fn editWithFrame_inView_editor_delegate_event(
            &self,
            rect: NSRect,
            controlView: &NSView,
            textObj: &NSText,
            delegate: Option<&Object>,
            event: Option<&NSEvent>,
        );
        #[method(selectWithFrame:inView:editor:delegate:start:length:)]
        pub unsafe fn selectWithFrame_inView_editor_delegate_start_length(
            &self,
            rect: NSRect,
            controlView: &NSView,
            textObj: &NSText,
            delegate: Option<&Object>,
            selStart: NSInteger,
            selLength: NSInteger,
        );
        #[method(endEditing:)]
        pub unsafe fn endEditing(&self, textObj: &NSText);
        #[method(resetCursorRect:inView:)]
        pub unsafe fn resetCursorRect_inView(&self, cellFrame: NSRect, controlView: &NSView);
        #[method_id(menu)]
        pub unsafe fn menu(&self) -> Option<Id<NSMenu, Shared>>;
        #[method(setMenu:)]
        pub unsafe fn setMenu(&self, menu: Option<&NSMenu>);
        #[method_id(menuForEvent:inRect:ofView:)]
        pub unsafe fn menuForEvent_inRect_ofView(
            &self,
            event: &NSEvent,
            cellFrame: NSRect,
            view: &NSView,
        ) -> Option<Id<NSMenu, Shared>>;
        #[method_id(defaultMenu)]
        pub unsafe fn defaultMenu() -> Option<Id<NSMenu, Shared>>;
        #[method(sendsActionOnEndEditing)]
        pub unsafe fn sendsActionOnEndEditing(&self) -> bool;
        #[method(setSendsActionOnEndEditing:)]
        pub unsafe fn setSendsActionOnEndEditing(&self, sendsActionOnEndEditing: bool);
        #[method(baseWritingDirection)]
        pub unsafe fn baseWritingDirection(&self) -> NSWritingDirection;
        #[method(setBaseWritingDirection:)]
        pub unsafe fn setBaseWritingDirection(&self, baseWritingDirection: NSWritingDirection);
        #[method(lineBreakMode)]
        pub unsafe fn lineBreakMode(&self) -> NSLineBreakMode;
        #[method(setLineBreakMode:)]
        pub unsafe fn setLineBreakMode(&self, lineBreakMode: NSLineBreakMode);
        #[method(allowsUndo)]
        pub unsafe fn allowsUndo(&self) -> bool;
        #[method(setAllowsUndo:)]
        pub unsafe fn setAllowsUndo(&self, allowsUndo: bool);
        #[method(truncatesLastVisibleLine)]
        pub unsafe fn truncatesLastVisibleLine(&self) -> bool;
        #[method(setTruncatesLastVisibleLine:)]
        pub unsafe fn setTruncatesLastVisibleLine(&self, truncatesLastVisibleLine: bool);
        #[method(userInterfaceLayoutDirection)]
        pub unsafe fn userInterfaceLayoutDirection(&self) -> NSUserInterfaceLayoutDirection;
        #[method(setUserInterfaceLayoutDirection:)]
        pub unsafe fn setUserInterfaceLayoutDirection(
            &self,
            userInterfaceLayoutDirection: NSUserInterfaceLayoutDirection,
        );
        #[method_id(fieldEditorForView:)]
        pub unsafe fn fieldEditorForView(
            &self,
            controlView: &NSView,
        ) -> Option<Id<NSTextView, Shared>>;
        #[method(usesSingleLineMode)]
        pub unsafe fn usesSingleLineMode(&self) -> bool;
        #[method(setUsesSingleLineMode:)]
        pub unsafe fn setUsesSingleLineMode(&self, usesSingleLineMode: bool);
        #[method_id(draggingImageComponentsWithFrame:inView:)]
        pub unsafe fn draggingImageComponentsWithFrame_inView(
            &self,
            frame: NSRect,
            view: &NSView,
        ) -> Id<NSArray<NSDraggingImageComponent>, Shared>;
    }
);
extern_methods!(
    #[doc = "NSKeyboardUI"]
    unsafe impl NSCell {
        #[method(refusesFirstResponder)]
        pub unsafe fn refusesFirstResponder(&self) -> bool;
        #[method(setRefusesFirstResponder:)]
        pub unsafe fn setRefusesFirstResponder(&self, refusesFirstResponder: bool);
        #[method(acceptsFirstResponder)]
        pub unsafe fn acceptsFirstResponder(&self) -> bool;
        #[method(showsFirstResponder)]
        pub unsafe fn showsFirstResponder(&self) -> bool;
        #[method(setShowsFirstResponder:)]
        pub unsafe fn setShowsFirstResponder(&self, showsFirstResponder: bool);
        #[method(performClick:)]
        pub unsafe fn performClick(&self, sender: Option<&Object>);
        #[method(focusRingType)]
        pub unsafe fn focusRingType(&self) -> NSFocusRingType;
        #[method(setFocusRingType:)]
        pub unsafe fn setFocusRingType(&self, focusRingType: NSFocusRingType);
        #[method(defaultFocusRingType)]
        pub unsafe fn defaultFocusRingType() -> NSFocusRingType;
        #[method(drawFocusRingMaskWithFrame:inView:)]
        pub unsafe fn drawFocusRingMaskWithFrame_inView(
            &self,
            cellFrame: NSRect,
            controlView: &NSView,
        );
        #[method(focusRingMaskBoundsForFrame:inView:)]
        pub unsafe fn focusRingMaskBoundsForFrame_inView(
            &self,
            cellFrame: NSRect,
            controlView: &NSView,
        ) -> NSRect;
        #[method(wantsNotificationForMarkedText)]
        pub unsafe fn wantsNotificationForMarkedText(&self) -> bool;
    }
);
extern_methods!(
    #[doc = "NSCellAttributedStringMethods"]
    unsafe impl NSCell {
        #[method_id(attributedStringValue)]
        pub unsafe fn attributedStringValue(&self) -> Id<NSAttributedString, Shared>;
        #[method(setAttributedStringValue:)]
        pub unsafe fn setAttributedStringValue(&self, attributedStringValue: &NSAttributedString);
        #[method(allowsEditingTextAttributes)]
        pub unsafe fn allowsEditingTextAttributes(&self) -> bool;
        #[method(setAllowsEditingTextAttributes:)]
        pub unsafe fn setAllowsEditingTextAttributes(&self, allowsEditingTextAttributes: bool);
        #[method(importsGraphics)]
        pub unsafe fn importsGraphics(&self) -> bool;
        #[method(setImportsGraphics:)]
        pub unsafe fn setImportsGraphics(&self, importsGraphics: bool);
    }
);
extern_methods!(
    #[doc = "NSCellMixedState"]
    unsafe impl NSCell {
        #[method(allowsMixedState)]
        pub unsafe fn allowsMixedState(&self) -> bool;
        #[method(setAllowsMixedState:)]
        pub unsafe fn setAllowsMixedState(&self, allowsMixedState: bool);
        #[method(nextState)]
        pub unsafe fn nextState(&self) -> NSInteger;
        #[method(setNextState)]
        pub unsafe fn setNextState(&self);
    }
);
extern_methods!(
    #[doc = "NSCellHitTest"]
    unsafe impl NSCell {
        #[method(hitTestForEvent:inRect:ofView:)]
        pub unsafe fn hitTestForEvent_inRect_ofView(
            &self,
            event: &NSEvent,
            cellFrame: NSRect,
            controlView: &NSView,
        ) -> NSCellHitResult;
    }
);
extern_methods!(
    #[doc = "NSCellExpansion"]
    unsafe impl NSCell {
        #[method(expansionFrameWithFrame:inView:)]
        pub unsafe fn expansionFrameWithFrame_inView(
            &self,
            cellFrame: NSRect,
            view: &NSView,
        ) -> NSRect;
        #[method(drawWithExpansionFrame:inView:)]
        pub unsafe fn drawWithExpansionFrame_inView(&self, cellFrame: NSRect, view: &NSView);
    }
);
extern_methods!(
    #[doc = "NSCellBackgroundStyle"]
    unsafe impl NSCell {
        #[method(backgroundStyle)]
        pub unsafe fn backgroundStyle(&self) -> NSBackgroundStyle;
        #[method(setBackgroundStyle:)]
        pub unsafe fn setBackgroundStyle(&self, backgroundStyle: NSBackgroundStyle);
        #[method(interiorBackgroundStyle)]
        pub unsafe fn interiorBackgroundStyle(&self) -> NSBackgroundStyle;
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSCell {
        #[method(controlTint)]
        pub unsafe fn controlTint(&self) -> NSControlTint;
        #[method(setControlTint:)]
        pub unsafe fn setControlTint(&self, controlTint: NSControlTint);
        #[method(entryType)]
        pub unsafe fn entryType(&self) -> NSInteger;
        #[method(setEntryType:)]
        pub unsafe fn setEntryType(&self, type_: NSInteger);
        #[method(isEntryAcceptable:)]
        pub unsafe fn isEntryAcceptable(&self, string: &NSString) -> bool;
        #[method(setFloatingPointFormat:left:right:)]
        pub unsafe fn setFloatingPointFormat_left_right(
            &self,
            autoRange: bool,
            leftDigits: NSUInteger,
            rightDigits: NSUInteger,
        );
        #[method(setMnemonicLocation:)]
        pub unsafe fn setMnemonicLocation(&self, location: NSUInteger);
        #[method(mnemonicLocation)]
        pub unsafe fn mnemonicLocation(&self) -> NSUInteger;
        #[method_id(mnemonic)]
        pub unsafe fn mnemonic(&self) -> Id<NSString, Shared>;
        #[method(setTitleWithMnemonic:)]
        pub unsafe fn setTitleWithMnemonic(&self, stringWithAmpersand: &NSString);
    }
);
pub type NSCellStateValue = NSControlStateValue;
