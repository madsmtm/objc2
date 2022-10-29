#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSTrackingRectTag = NSInteger;
pub type NSToolTipTag = NSInteger;
extern_class!(
    #[derive(Debug)]
    pub struct NSView;
    unsafe impl ClassType for NSView {
        type Super = NSResponder;
    }
);
extern_methods!(
    unsafe impl NSView {
        #[method_id(initWithFrame:)]
        pub unsafe fn initWithFrame(&self, frameRect: NSRect) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(window)]
        pub unsafe fn window(&self) -> Option<Id<NSWindow, Shared>>;
        #[method_id(superview)]
        pub unsafe fn superview(&self) -> Option<Id<NSView, Shared>>;
        #[method_id(subviews)]
        pub unsafe fn subviews(&self) -> Id<NSArray<NSView>, Shared>;
        #[method(setSubviews:)]
        pub unsafe fn setSubviews(&self, subviews: &NSArray<NSView>);
        #[method(isDescendantOf:)]
        pub unsafe fn isDescendantOf(&self, view: &NSView) -> bool;
        #[method_id(ancestorSharedWithView:)]
        pub unsafe fn ancestorSharedWithView(&self, view: &NSView) -> Option<Id<NSView, Shared>>;
        #[method_id(opaqueAncestor)]
        pub unsafe fn opaqueAncestor(&self) -> Option<Id<NSView, Shared>>;
        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;
        #[method(setHidden:)]
        pub unsafe fn setHidden(&self, hidden: bool);
        #[method(isHiddenOrHasHiddenAncestor)]
        pub unsafe fn isHiddenOrHasHiddenAncestor(&self) -> bool;
        #[method(getRectsBeingDrawn:count:)]
        pub unsafe fn getRectsBeingDrawn_count(
            &self,
            rects: *mut *mut NSRect,
            count: *mut NSInteger,
        );
        #[method(needsToDrawRect:)]
        pub unsafe fn needsToDrawRect(&self, rect: NSRect) -> bool;
        #[method(wantsDefaultClipping)]
        pub unsafe fn wantsDefaultClipping(&self) -> bool;
        #[method(viewDidHide)]
        pub unsafe fn viewDidHide(&self);
        #[method(viewDidUnhide)]
        pub unsafe fn viewDidUnhide(&self);
        #[method(addSubview:)]
        pub unsafe fn addSubview(&self, view: &NSView);
        #[method(addSubview:positioned:relativeTo:)]
        pub unsafe fn addSubview_positioned_relativeTo(
            &self,
            view: &NSView,
            place: NSWindowOrderingMode,
            otherView: Option<&NSView>,
        );
        #[method(sortSubviewsUsingFunction:context:)]
        pub unsafe fn sortSubviewsUsingFunction_context(
            &self,
            compare: NonNull<TodoFunction>,
            context: *mut c_void,
        );
        #[method(viewWillMoveToWindow:)]
        pub unsafe fn viewWillMoveToWindow(&self, newWindow: Option<&NSWindow>);
        #[method(viewDidMoveToWindow)]
        pub unsafe fn viewDidMoveToWindow(&self);
        #[method(viewWillMoveToSuperview:)]
        pub unsafe fn viewWillMoveToSuperview(&self, newSuperview: Option<&NSView>);
        #[method(viewDidMoveToSuperview)]
        pub unsafe fn viewDidMoveToSuperview(&self);
        #[method(didAddSubview:)]
        pub unsafe fn didAddSubview(&self, subview: &NSView);
        #[method(willRemoveSubview:)]
        pub unsafe fn willRemoveSubview(&self, subview: &NSView);
        #[method(removeFromSuperview)]
        pub unsafe fn removeFromSuperview(&self);
        #[method(replaceSubview:with:)]
        pub unsafe fn replaceSubview_with(&self, oldView: &NSView, newView: &NSView);
        #[method(removeFromSuperviewWithoutNeedingDisplay)]
        pub unsafe fn removeFromSuperviewWithoutNeedingDisplay(&self);
        #[method(viewDidChangeBackingProperties)]
        pub unsafe fn viewDidChangeBackingProperties(&self);
        #[method(postsFrameChangedNotifications)]
        pub unsafe fn postsFrameChangedNotifications(&self) -> bool;
        #[method(setPostsFrameChangedNotifications:)]
        pub unsafe fn setPostsFrameChangedNotifications(
            &self,
            postsFrameChangedNotifications: bool,
        );
        #[method(resizeSubviewsWithOldSize:)]
        pub unsafe fn resizeSubviewsWithOldSize(&self, oldSize: NSSize);
        #[method(resizeWithOldSuperviewSize:)]
        pub unsafe fn resizeWithOldSuperviewSize(&self, oldSize: NSSize);
        #[method(autoresizesSubviews)]
        pub unsafe fn autoresizesSubviews(&self) -> bool;
        #[method(setAutoresizesSubviews:)]
        pub unsafe fn setAutoresizesSubviews(&self, autoresizesSubviews: bool);
        #[method(autoresizingMask)]
        pub unsafe fn autoresizingMask(&self) -> NSAutoresizingMaskOptions;
        #[method(setAutoresizingMask:)]
        pub unsafe fn setAutoresizingMask(&self, autoresizingMask: NSAutoresizingMaskOptions);
        #[method(setFrameOrigin:)]
        pub unsafe fn setFrameOrigin(&self, newOrigin: NSPoint);
        #[method(setFrameSize:)]
        pub unsafe fn setFrameSize(&self, newSize: NSSize);
        #[method(frame)]
        pub unsafe fn frame(&self) -> NSRect;
        #[method(setFrame:)]
        pub unsafe fn setFrame(&self, frame: NSRect);
        #[method(frameRotation)]
        pub unsafe fn frameRotation(&self) -> CGFloat;
        #[method(setFrameRotation:)]
        pub unsafe fn setFrameRotation(&self, frameRotation: CGFloat);
        #[method(frameCenterRotation)]
        pub unsafe fn frameCenterRotation(&self) -> CGFloat;
        #[method(setFrameCenterRotation:)]
        pub unsafe fn setFrameCenterRotation(&self, frameCenterRotation: CGFloat);
        #[method(setBoundsOrigin:)]
        pub unsafe fn setBoundsOrigin(&self, newOrigin: NSPoint);
        #[method(setBoundsSize:)]
        pub unsafe fn setBoundsSize(&self, newSize: NSSize);
        #[method(boundsRotation)]
        pub unsafe fn boundsRotation(&self) -> CGFloat;
        #[method(setBoundsRotation:)]
        pub unsafe fn setBoundsRotation(&self, boundsRotation: CGFloat);
        #[method(translateOriginToPoint:)]
        pub unsafe fn translateOriginToPoint(&self, translation: NSPoint);
        #[method(scaleUnitSquareToSize:)]
        pub unsafe fn scaleUnitSquareToSize(&self, newUnitSize: NSSize);
        #[method(rotateByAngle:)]
        pub unsafe fn rotateByAngle(&self, angle: CGFloat);
        #[method(bounds)]
        pub unsafe fn bounds(&self) -> NSRect;
        #[method(setBounds:)]
        pub unsafe fn setBounds(&self, bounds: NSRect);
        #[method(isFlipped)]
        pub unsafe fn isFlipped(&self) -> bool;
        #[method(isRotatedFromBase)]
        pub unsafe fn isRotatedFromBase(&self) -> bool;
        #[method(isRotatedOrScaledFromBase)]
        pub unsafe fn isRotatedOrScaledFromBase(&self) -> bool;
        #[method(isOpaque)]
        pub unsafe fn isOpaque(&self) -> bool;
        #[method(convertPoint:fromView:)]
        pub unsafe fn convertPoint_fromView(
            &self,
            point: NSPoint,
            view: Option<&NSView>,
        ) -> NSPoint;
        #[method(convertPoint:toView:)]
        pub unsafe fn convertPoint_toView(&self, point: NSPoint, view: Option<&NSView>) -> NSPoint;
        #[method(convertSize:fromView:)]
        pub unsafe fn convertSize_fromView(&self, size: NSSize, view: Option<&NSView>) -> NSSize;
        #[method(convertSize:toView:)]
        pub unsafe fn convertSize_toView(&self, size: NSSize, view: Option<&NSView>) -> NSSize;
        #[method(convertRect:fromView:)]
        pub unsafe fn convertRect_fromView(&self, rect: NSRect, view: Option<&NSView>) -> NSRect;
        #[method(convertRect:toView:)]
        pub unsafe fn convertRect_toView(&self, rect: NSRect, view: Option<&NSView>) -> NSRect;
        #[method(backingAlignedRect:options:)]
        pub unsafe fn backingAlignedRect_options(
            &self,
            rect: NSRect,
            options: NSAlignmentOptions,
        ) -> NSRect;
        #[method(centerScanRect:)]
        pub unsafe fn centerScanRect(&self, rect: NSRect) -> NSRect;
        #[method(convertPointToBacking:)]
        pub unsafe fn convertPointToBacking(&self, point: NSPoint) -> NSPoint;
        #[method(convertPointFromBacking:)]
        pub unsafe fn convertPointFromBacking(&self, point: NSPoint) -> NSPoint;
        #[method(convertSizeToBacking:)]
        pub unsafe fn convertSizeToBacking(&self, size: NSSize) -> NSSize;
        #[method(convertSizeFromBacking:)]
        pub unsafe fn convertSizeFromBacking(&self, size: NSSize) -> NSSize;
        #[method(convertRectToBacking:)]
        pub unsafe fn convertRectToBacking(&self, rect: NSRect) -> NSRect;
        #[method(convertRectFromBacking:)]
        pub unsafe fn convertRectFromBacking(&self, rect: NSRect) -> NSRect;
        #[method(convertPointToLayer:)]
        pub unsafe fn convertPointToLayer(&self, point: NSPoint) -> NSPoint;
        #[method(convertPointFromLayer:)]
        pub unsafe fn convertPointFromLayer(&self, point: NSPoint) -> NSPoint;
        #[method(convertSizeToLayer:)]
        pub unsafe fn convertSizeToLayer(&self, size: NSSize) -> NSSize;
        #[method(convertSizeFromLayer:)]
        pub unsafe fn convertSizeFromLayer(&self, size: NSSize) -> NSSize;
        #[method(convertRectToLayer:)]
        pub unsafe fn convertRectToLayer(&self, rect: NSRect) -> NSRect;
        #[method(convertRectFromLayer:)]
        pub unsafe fn convertRectFromLayer(&self, rect: NSRect) -> NSRect;
        #[method(canDrawConcurrently)]
        pub unsafe fn canDrawConcurrently(&self) -> bool;
        #[method(setCanDrawConcurrently:)]
        pub unsafe fn setCanDrawConcurrently(&self, canDrawConcurrently: bool);
        #[method(canDraw)]
        pub unsafe fn canDraw(&self) -> bool;
        #[method(setNeedsDisplayInRect:)]
        pub unsafe fn setNeedsDisplayInRect(&self, invalidRect: NSRect);
        #[method(needsDisplay)]
        pub unsafe fn needsDisplay(&self) -> bool;
        #[method(setNeedsDisplay:)]
        pub unsafe fn setNeedsDisplay(&self, needsDisplay: bool);
        #[method(lockFocus)]
        pub unsafe fn lockFocus(&self);
        #[method(unlockFocus)]
        pub unsafe fn unlockFocus(&self);
        #[method(lockFocusIfCanDraw)]
        pub unsafe fn lockFocusIfCanDraw(&self) -> bool;
        #[method(lockFocusIfCanDrawInContext:)]
        pub unsafe fn lockFocusIfCanDrawInContext(&self, context: &NSGraphicsContext) -> bool;
        #[method_id(focusView)]
        pub unsafe fn focusView() -> Option<Id<NSView, Shared>>;
        #[method(visibleRect)]
        pub unsafe fn visibleRect(&self) -> NSRect;
        #[method(display)]
        pub unsafe fn display(&self);
        #[method(displayIfNeeded)]
        pub unsafe fn displayIfNeeded(&self);
        #[method(displayIfNeededIgnoringOpacity)]
        pub unsafe fn displayIfNeededIgnoringOpacity(&self);
        #[method(displayRect:)]
        pub unsafe fn displayRect(&self, rect: NSRect);
        #[method(displayIfNeededInRect:)]
        pub unsafe fn displayIfNeededInRect(&self, rect: NSRect);
        #[method(displayRectIgnoringOpacity:)]
        pub unsafe fn displayRectIgnoringOpacity(&self, rect: NSRect);
        #[method(displayIfNeededInRectIgnoringOpacity:)]
        pub unsafe fn displayIfNeededInRectIgnoringOpacity(&self, rect: NSRect);
        #[method(drawRect:)]
        pub unsafe fn drawRect(&self, dirtyRect: NSRect);
        #[method(displayRectIgnoringOpacity:inContext:)]
        pub unsafe fn displayRectIgnoringOpacity_inContext(
            &self,
            rect: NSRect,
            context: &NSGraphicsContext,
        );
        #[method_id(bitmapImageRepForCachingDisplayInRect:)]
        pub unsafe fn bitmapImageRepForCachingDisplayInRect(
            &self,
            rect: NSRect,
        ) -> Option<Id<NSBitmapImageRep, Shared>>;
        #[method(cacheDisplayInRect:toBitmapImageRep:)]
        pub unsafe fn cacheDisplayInRect_toBitmapImageRep(
            &self,
            rect: NSRect,
            bitmapImageRep: &NSBitmapImageRep,
        );
        #[method(viewWillDraw)]
        pub unsafe fn viewWillDraw(&self);
        #[method(scrollPoint:)]
        pub unsafe fn scrollPoint(&self, point: NSPoint);
        #[method(scrollRectToVisible:)]
        pub unsafe fn scrollRectToVisible(&self, rect: NSRect) -> bool;
        #[method(autoscroll:)]
        pub unsafe fn autoscroll(&self, event: &NSEvent) -> bool;
        #[method(adjustScroll:)]
        pub unsafe fn adjustScroll(&self, newVisible: NSRect) -> NSRect;
        #[method(scrollRect:by:)]
        pub unsafe fn scrollRect_by(&self, rect: NSRect, delta: NSSize);
        #[method(translateRectsNeedingDisplayInRect:by:)]
        pub unsafe fn translateRectsNeedingDisplayInRect_by(&self, clipRect: NSRect, delta: NSSize);
        #[method_id(hitTest:)]
        pub unsafe fn hitTest(&self, point: NSPoint) -> Option<Id<NSView, Shared>>;
        #[method(mouse:inRect:)]
        pub unsafe fn mouse_inRect(&self, point: NSPoint, rect: NSRect) -> bool;
        #[method_id(viewWithTag:)]
        pub unsafe fn viewWithTag(&self, tag: NSInteger) -> Option<Id<NSView, Shared>>;
        #[method(tag)]
        pub unsafe fn tag(&self) -> NSInteger;
        #[method(performKeyEquivalent:)]
        pub unsafe fn performKeyEquivalent(&self, event: &NSEvent) -> bool;
        #[method(acceptsFirstMouse:)]
        pub unsafe fn acceptsFirstMouse(&self, event: Option<&NSEvent>) -> bool;
        #[method(shouldDelayWindowOrderingForEvent:)]
        pub unsafe fn shouldDelayWindowOrderingForEvent(&self, event: &NSEvent) -> bool;
        #[method(needsPanelToBecomeKey)]
        pub unsafe fn needsPanelToBecomeKey(&self) -> bool;
        #[method(mouseDownCanMoveWindow)]
        pub unsafe fn mouseDownCanMoveWindow(&self) -> bool;
        #[method(acceptsTouchEvents)]
        pub unsafe fn acceptsTouchEvents(&self) -> bool;
        #[method(setAcceptsTouchEvents:)]
        pub unsafe fn setAcceptsTouchEvents(&self, acceptsTouchEvents: bool);
        #[method(wantsRestingTouches)]
        pub unsafe fn wantsRestingTouches(&self) -> bool;
        #[method(setWantsRestingTouches:)]
        pub unsafe fn setWantsRestingTouches(&self, wantsRestingTouches: bool);
        #[method(addCursorRect:cursor:)]
        pub unsafe fn addCursorRect_cursor(&self, rect: NSRect, object: &NSCursor);
        #[method(removeCursorRect:cursor:)]
        pub unsafe fn removeCursorRect_cursor(&self, rect: NSRect, object: &NSCursor);
        #[method(discardCursorRects)]
        pub unsafe fn discardCursorRects(&self);
        #[method(resetCursorRects)]
        pub unsafe fn resetCursorRects(&self);
        #[method(addTrackingRect:owner:userData:assumeInside:)]
        pub unsafe fn addTrackingRect_owner_userData_assumeInside(
            &self,
            rect: NSRect,
            owner: &Object,
            data: *mut c_void,
            flag: bool,
        ) -> NSTrackingRectTag;
        #[method(removeTrackingRect:)]
        pub unsafe fn removeTrackingRect(&self, tag: NSTrackingRectTag);
        #[method_id(makeBackingLayer)]
        pub unsafe fn makeBackingLayer(&self) -> Id<CALayer, Shared>;
        #[method(layerContentsRedrawPolicy)]
        pub unsafe fn layerContentsRedrawPolicy(&self) -> NSViewLayerContentsRedrawPolicy;
        #[method(setLayerContentsRedrawPolicy:)]
        pub unsafe fn setLayerContentsRedrawPolicy(
            &self,
            layerContentsRedrawPolicy: NSViewLayerContentsRedrawPolicy,
        );
        #[method(layerContentsPlacement)]
        pub unsafe fn layerContentsPlacement(&self) -> NSViewLayerContentsPlacement;
        #[method(setLayerContentsPlacement:)]
        pub unsafe fn setLayerContentsPlacement(
            &self,
            layerContentsPlacement: NSViewLayerContentsPlacement,
        );
        #[method(wantsLayer)]
        pub unsafe fn wantsLayer(&self) -> bool;
        #[method(setWantsLayer:)]
        pub unsafe fn setWantsLayer(&self, wantsLayer: bool);
        #[method_id(layer)]
        pub unsafe fn layer(&self) -> Option<Id<CALayer, Shared>>;
        #[method(setLayer:)]
        pub unsafe fn setLayer(&self, layer: Option<&CALayer>);
        #[method(wantsUpdateLayer)]
        pub unsafe fn wantsUpdateLayer(&self) -> bool;
        #[method(updateLayer)]
        pub unsafe fn updateLayer(&self);
        #[method(canDrawSubviewsIntoLayer)]
        pub unsafe fn canDrawSubviewsIntoLayer(&self) -> bool;
        #[method(setCanDrawSubviewsIntoLayer:)]
        pub unsafe fn setCanDrawSubviewsIntoLayer(&self, canDrawSubviewsIntoLayer: bool);
        #[method(layoutSubtreeIfNeeded)]
        pub unsafe fn layoutSubtreeIfNeeded(&self);
        #[method(layout)]
        pub unsafe fn layout(&self);
        #[method(needsLayout)]
        pub unsafe fn needsLayout(&self) -> bool;
        #[method(setNeedsLayout:)]
        pub unsafe fn setNeedsLayout(&self, needsLayout: bool);
        #[method(alphaValue)]
        pub unsafe fn alphaValue(&self) -> CGFloat;
        #[method(setAlphaValue:)]
        pub unsafe fn setAlphaValue(&self, alphaValue: CGFloat);
        #[method(layerUsesCoreImageFilters)]
        pub unsafe fn layerUsesCoreImageFilters(&self) -> bool;
        #[method(setLayerUsesCoreImageFilters:)]
        pub unsafe fn setLayerUsesCoreImageFilters(&self, layerUsesCoreImageFilters: bool);
        #[method_id(backgroundFilters)]
        pub unsafe fn backgroundFilters(&self) -> Id<NSArray<CIFilter>, Shared>;
        #[method(setBackgroundFilters:)]
        pub unsafe fn setBackgroundFilters(&self, backgroundFilters: &NSArray<CIFilter>);
        #[method_id(compositingFilter)]
        pub unsafe fn compositingFilter(&self) -> Option<Id<CIFilter, Shared>>;
        #[method(setCompositingFilter:)]
        pub unsafe fn setCompositingFilter(&self, compositingFilter: Option<&CIFilter>);
        #[method_id(contentFilters)]
        pub unsafe fn contentFilters(&self) -> Id<NSArray<CIFilter>, Shared>;
        #[method(setContentFilters:)]
        pub unsafe fn setContentFilters(&self, contentFilters: &NSArray<CIFilter>);
        #[method_id(shadow)]
        pub unsafe fn shadow(&self) -> Option<Id<NSShadow, Shared>>;
        #[method(setShadow:)]
        pub unsafe fn setShadow(&self, shadow: Option<&NSShadow>);
        #[method(addTrackingArea:)]
        pub unsafe fn addTrackingArea(&self, trackingArea: &NSTrackingArea);
        #[method(removeTrackingArea:)]
        pub unsafe fn removeTrackingArea(&self, trackingArea: &NSTrackingArea);
        #[method_id(trackingAreas)]
        pub unsafe fn trackingAreas(&self) -> Id<NSArray<NSTrackingArea>, Shared>;
        #[method(updateTrackingAreas)]
        pub unsafe fn updateTrackingAreas(&self);
        #[method(postsBoundsChangedNotifications)]
        pub unsafe fn postsBoundsChangedNotifications(&self) -> bool;
        #[method(setPostsBoundsChangedNotifications:)]
        pub unsafe fn setPostsBoundsChangedNotifications(
            &self,
            postsBoundsChangedNotifications: bool,
        );
        #[method_id(enclosingScrollView)]
        pub unsafe fn enclosingScrollView(&self) -> Option<Id<NSScrollView, Shared>>;
        #[method_id(menuForEvent:)]
        pub unsafe fn menuForEvent(&self, event: &NSEvent) -> Option<Id<NSMenu, Shared>>;
        #[method_id(defaultMenu)]
        pub unsafe fn defaultMenu() -> Option<Id<NSMenu, Shared>>;
        #[method(willOpenMenu:withEvent:)]
        pub unsafe fn willOpenMenu_withEvent(&self, menu: &NSMenu, event: &NSEvent);
        #[method(didCloseMenu:withEvent:)]
        pub unsafe fn didCloseMenu_withEvent(&self, menu: &NSMenu, event: Option<&NSEvent>);
        #[method_id(toolTip)]
        pub unsafe fn toolTip(&self) -> Option<Id<NSString, Shared>>;
        #[method(setToolTip:)]
        pub unsafe fn setToolTip(&self, toolTip: Option<&NSString>);
        #[method(addToolTipRect:owner:userData:)]
        pub unsafe fn addToolTipRect_owner_userData(
            &self,
            rect: NSRect,
            owner: &Object,
            data: *mut c_void,
        ) -> NSToolTipTag;
        #[method(removeToolTip:)]
        pub unsafe fn removeToolTip(&self, tag: NSToolTipTag);
        #[method(removeAllToolTips)]
        pub unsafe fn removeAllToolTips(&self);
        #[method(viewWillStartLiveResize)]
        pub unsafe fn viewWillStartLiveResize(&self);
        #[method(viewDidEndLiveResize)]
        pub unsafe fn viewDidEndLiveResize(&self);
        #[method(inLiveResize)]
        pub unsafe fn inLiveResize(&self) -> bool;
        #[method(preservesContentDuringLiveResize)]
        pub unsafe fn preservesContentDuringLiveResize(&self) -> bool;
        #[method(rectPreservedDuringLiveResize)]
        pub unsafe fn rectPreservedDuringLiveResize(&self) -> NSRect;
        #[method(getRectsExposedDuringLiveResize:count:)]
        pub unsafe fn getRectsExposedDuringLiveResize_count(
            &self,
            exposedRects: [NSRect; 4usize],
            count: NonNull<NSInteger>,
        );
        #[method_id(inputContext)]
        pub unsafe fn inputContext(&self) -> Option<Id<NSTextInputContext, Shared>>;
        #[method(rectForSmartMagnificationAtPoint:inRect:)]
        pub unsafe fn rectForSmartMagnificationAtPoint_inRect(
            &self,
            location: NSPoint,
            visibleRect: NSRect,
        ) -> NSRect;
        #[method(userInterfaceLayoutDirection)]
        pub unsafe fn userInterfaceLayoutDirection(&self) -> NSUserInterfaceLayoutDirection;
        #[method(setUserInterfaceLayoutDirection:)]
        pub unsafe fn setUserInterfaceLayoutDirection(
            &self,
            userInterfaceLayoutDirection: NSUserInterfaceLayoutDirection,
        );
        #[method(prepareForReuse)]
        pub unsafe fn prepareForReuse(&self);
        #[method(isCompatibleWithResponsiveScrolling)]
        pub unsafe fn isCompatibleWithResponsiveScrolling() -> bool;
        #[method(prepareContentInRect:)]
        pub unsafe fn prepareContentInRect(&self, rect: NSRect);
        #[method(preparedContentRect)]
        pub unsafe fn preparedContentRect(&self) -> NSRect;
        #[method(setPreparedContentRect:)]
        pub unsafe fn setPreparedContentRect(&self, preparedContentRect: NSRect);
        #[method(allowsVibrancy)]
        pub unsafe fn allowsVibrancy(&self) -> bool;
        #[method(viewDidChangeEffectiveAppearance)]
        pub unsafe fn viewDidChangeEffectiveAppearance(&self);
    }
);
pub type NSViewLayerContentScaleDelegate = NSObject;
extern_methods!(
    #[doc = "NSLayerDelegateContentsScaleUpdating"]
    unsafe impl NSObject {
        #[method(layer:shouldInheritContentsScale:fromWindow:)]
        pub unsafe fn layer_shouldInheritContentsScale_fromWindow(
            &self,
            layer: &CALayer,
            newScale: CGFloat,
            window: &NSWindow,
        ) -> bool;
    }
);
pub type NSViewToolTipOwner = NSObject;
extern_methods!(
    #[doc = "NSToolTipOwner"]
    unsafe impl NSObject {
        #[method_id(view:stringForToolTip:point:userData:)]
        pub unsafe fn view_stringForToolTip_point_userData(
            &self,
            view: &NSView,
            tag: NSToolTipTag,
            point: NSPoint,
            data: *mut c_void,
        ) -> Id<NSString, Shared>;
    }
);
extern_methods!(
    #[doc = "NSKeyboardUI"]
    unsafe impl NSView {
        #[method_id(nextKeyView)]
        pub unsafe fn nextKeyView(&self) -> Option<Id<NSView, Shared>>;
        #[method(setNextKeyView:)]
        pub unsafe fn setNextKeyView(&self, nextKeyView: Option<&NSView>);
        #[method_id(previousKeyView)]
        pub unsafe fn previousKeyView(&self) -> Option<Id<NSView, Shared>>;
        #[method_id(nextValidKeyView)]
        pub unsafe fn nextValidKeyView(&self) -> Option<Id<NSView, Shared>>;
        #[method_id(previousValidKeyView)]
        pub unsafe fn previousValidKeyView(&self) -> Option<Id<NSView, Shared>>;
        #[method(canBecomeKeyView)]
        pub unsafe fn canBecomeKeyView(&self) -> bool;
        #[method(setKeyboardFocusRingNeedsDisplayInRect:)]
        pub unsafe fn setKeyboardFocusRingNeedsDisplayInRect(&self, rect: NSRect);
        #[method(focusRingType)]
        pub unsafe fn focusRingType(&self) -> NSFocusRingType;
        #[method(setFocusRingType:)]
        pub unsafe fn setFocusRingType(&self, focusRingType: NSFocusRingType);
        #[method(defaultFocusRingType)]
        pub unsafe fn defaultFocusRingType() -> NSFocusRingType;
        #[method(drawFocusRingMask)]
        pub unsafe fn drawFocusRingMask(&self);
        #[method(focusRingMaskBounds)]
        pub unsafe fn focusRingMaskBounds(&self) -> NSRect;
        #[method(noteFocusRingMaskChanged)]
        pub unsafe fn noteFocusRingMaskChanged(&self);
    }
);
extern_methods!(
    #[doc = "NSPrinting"]
    unsafe impl NSView {
        #[method(writeEPSInsideRect:toPasteboard:)]
        pub unsafe fn writeEPSInsideRect_toPasteboard(
            &self,
            rect: NSRect,
            pasteboard: &NSPasteboard,
        );
        #[method_id(dataWithEPSInsideRect:)]
        pub unsafe fn dataWithEPSInsideRect(&self, rect: NSRect) -> Id<NSData, Shared>;
        #[method(writePDFInsideRect:toPasteboard:)]
        pub unsafe fn writePDFInsideRect_toPasteboard(
            &self,
            rect: NSRect,
            pasteboard: &NSPasteboard,
        );
        #[method_id(dataWithPDFInsideRect:)]
        pub unsafe fn dataWithPDFInsideRect(&self, rect: NSRect) -> Id<NSData, Shared>;
        #[method(print:)]
        pub unsafe fn print(&self, sender: Option<&Object>);
        #[method(knowsPageRange:)]
        pub unsafe fn knowsPageRange(&self, range: NSRangePointer) -> bool;
        #[method(heightAdjustLimit)]
        pub unsafe fn heightAdjustLimit(&self) -> CGFloat;
        #[method(widthAdjustLimit)]
        pub unsafe fn widthAdjustLimit(&self) -> CGFloat;
        #[method(adjustPageWidthNew:left:right:limit:)]
        pub unsafe fn adjustPageWidthNew_left_right_limit(
            &self,
            newRight: NonNull<CGFloat>,
            oldLeft: CGFloat,
            oldRight: CGFloat,
            rightLimit: CGFloat,
        );
        #[method(adjustPageHeightNew:top:bottom:limit:)]
        pub unsafe fn adjustPageHeightNew_top_bottom_limit(
            &self,
            newBottom: NonNull<CGFloat>,
            oldTop: CGFloat,
            oldBottom: CGFloat,
            bottomLimit: CGFloat,
        );
        #[method(rectForPage:)]
        pub unsafe fn rectForPage(&self, page: NSInteger) -> NSRect;
        #[method(locationOfPrintRect:)]
        pub unsafe fn locationOfPrintRect(&self, rect: NSRect) -> NSPoint;
        #[method(drawPageBorderWithSize:)]
        pub unsafe fn drawPageBorderWithSize(&self, borderSize: NSSize);
        #[method_id(pageHeader)]
        pub unsafe fn pageHeader(&self) -> Id<NSAttributedString, Shared>;
        #[method_id(pageFooter)]
        pub unsafe fn pageFooter(&self) -> Id<NSAttributedString, Shared>;
        #[method(drawSheetBorderWithSize:)]
        pub unsafe fn drawSheetBorderWithSize(&self, borderSize: NSSize);
        #[method_id(printJobTitle)]
        pub unsafe fn printJobTitle(&self) -> Id<NSString, Shared>;
        #[method(beginDocument)]
        pub unsafe fn beginDocument(&self);
        #[method(endDocument)]
        pub unsafe fn endDocument(&self);
        #[method(beginPageInRect:atPlacement:)]
        pub unsafe fn beginPageInRect_atPlacement(&self, rect: NSRect, location: NSPoint);
        #[method(endPage)]
        pub unsafe fn endPage(&self);
    }
);
extern_methods!(
    #[doc = "NSDrag"]
    unsafe impl NSView {
        #[method_id(beginDraggingSessionWithItems:event:source:)]
        pub unsafe fn beginDraggingSessionWithItems_event_source(
            &self,
            items: &NSArray<NSDraggingItem>,
            event: &NSEvent,
            source: &NSDraggingSource,
        ) -> Id<NSDraggingSession, Shared>;
        #[method_id(registeredDraggedTypes)]
        pub unsafe fn registeredDraggedTypes(&self) -> Id<NSArray<NSPasteboardType>, Shared>;
        #[method(registerForDraggedTypes:)]
        pub unsafe fn registerForDraggedTypes(&self, newTypes: &NSArray<NSPasteboardType>);
        #[method(unregisterDraggedTypes)]
        pub unsafe fn unregisterDraggedTypes(&self);
    }
);
pub type NSViewFullScreenModeOptionKey = NSString;
extern_methods!(
    #[doc = "NSFullScreenMode"]
    unsafe impl NSView {
        #[method(enterFullScreenMode:withOptions:)]
        pub unsafe fn enterFullScreenMode_withOptions(
            &self,
            screen: &NSScreen,
            options: Option<&NSDictionary<NSViewFullScreenModeOptionKey, Object>>,
        ) -> bool;
        #[method(exitFullScreenModeWithOptions:)]
        pub unsafe fn exitFullScreenModeWithOptions(
            &self,
            options: Option<&NSDictionary<NSViewFullScreenModeOptionKey, Object>>,
        );
        #[method(isInFullScreenMode)]
        pub unsafe fn isInFullScreenMode(&self) -> bool;
    }
);
pub type NSDefinitionOptionKey = NSString;
pub type NSDefinitionPresentationType = NSString;
extern_methods!(
    #[doc = "NSDefinition"]
    unsafe impl NSView {
        #[method(showDefinitionForAttributedString:atPoint:)]
        pub unsafe fn showDefinitionForAttributedString_atPoint(
            &self,
            attrString: Option<&NSAttributedString>,
            textBaselineOrigin: NSPoint,
        );
        #[method(showDefinitionForAttributedString:range:options:baselineOriginProvider:)]
        pub unsafe fn showDefinitionForAttributedString_range_options_baselineOriginProvider(
            &self,
            attrString: Option<&NSAttributedString>,
            targetRange: NSRange,
            options: Option<&NSDictionary<NSDefinitionOptionKey, Object>>,
            originProvider: TodoBlock,
        );
    }
);
extern_methods!(
    #[doc = "NSFindIndicator"]
    unsafe impl NSView {
        #[method(isDrawingFindIndicator)]
        pub unsafe fn isDrawingFindIndicator(&self) -> bool;
    }
);
extern_methods!(
    #[doc = "NSGestureRecognizer"]
    unsafe impl NSView {
        #[method_id(gestureRecognizers)]
        pub unsafe fn gestureRecognizers(&self) -> Id<NSArray<NSGestureRecognizer>, Shared>;
        #[method(setGestureRecognizers:)]
        pub unsafe fn setGestureRecognizers(
            &self,
            gestureRecognizers: &NSArray<NSGestureRecognizer>,
        );
        #[method(addGestureRecognizer:)]
        pub unsafe fn addGestureRecognizer(&self, gestureRecognizer: &NSGestureRecognizer);
        #[method(removeGestureRecognizer:)]
        pub unsafe fn removeGestureRecognizer(&self, gestureRecognizer: &NSGestureRecognizer);
    }
);
extern_methods!(
    #[doc = "NSTouchBar"]
    unsafe impl NSView {
        #[method(allowedTouchTypes)]
        pub unsafe fn allowedTouchTypes(&self) -> NSTouchTypeMask;
        #[method(setAllowedTouchTypes:)]
        pub unsafe fn setAllowedTouchTypes(&self, allowedTouchTypes: NSTouchTypeMask);
    }
);
extern_methods!(
    #[doc = "NSSafeAreas"]
    unsafe impl NSView {
        #[method(safeAreaInsets)]
        pub unsafe fn safeAreaInsets(&self) -> NSEdgeInsets;
        #[method(additionalSafeAreaInsets)]
        pub unsafe fn additionalSafeAreaInsets(&self) -> NSEdgeInsets;
        #[method(setAdditionalSafeAreaInsets:)]
        pub unsafe fn setAdditionalSafeAreaInsets(&self, additionalSafeAreaInsets: NSEdgeInsets);
        #[method_id(safeAreaLayoutGuide)]
        pub unsafe fn safeAreaLayoutGuide(&self) -> Id<NSLayoutGuide, Shared>;
        #[method(safeAreaRect)]
        pub unsafe fn safeAreaRect(&self) -> NSRect;
        #[method_id(layoutMarginsGuide)]
        pub unsafe fn layoutMarginsGuide(&self) -> Id<NSLayoutGuide, Shared>;
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSView {
        #[method(dragImage:at:offset:event:pasteboard:source:slideBack:)]
        pub unsafe fn dragImage_at_offset_event_pasteboard_source_slideBack(
            &self,
            image: &NSImage,
            viewLocation: NSPoint,
            initialOffset: NSSize,
            event: &NSEvent,
            pboard: &NSPasteboard,
            sourceObj: &Object,
            slideFlag: bool,
        );
        #[method(dragFile:fromRect:slideBack:event:)]
        pub unsafe fn dragFile_fromRect_slideBack_event(
            &self,
            filename: &NSString,
            rect: NSRect,
            flag: bool,
            event: &NSEvent,
        ) -> bool;
        #[method(dragPromisedFilesOfTypes:fromRect:source:slideBack:event:)]
        pub unsafe fn dragPromisedFilesOfTypes_fromRect_source_slideBack_event(
            &self,
            typeArray: &NSArray<NSString>,
            rect: NSRect,
            sourceObject: &Object,
            flag: bool,
            event: &NSEvent,
        ) -> bool;
        #[method(convertPointToBase:)]
        pub unsafe fn convertPointToBase(&self, point: NSPoint) -> NSPoint;
        #[method(convertPointFromBase:)]
        pub unsafe fn convertPointFromBase(&self, point: NSPoint) -> NSPoint;
        #[method(convertSizeToBase:)]
        pub unsafe fn convertSizeToBase(&self, size: NSSize) -> NSSize;
        #[method(convertSizeFromBase:)]
        pub unsafe fn convertSizeFromBase(&self, size: NSSize) -> NSSize;
        #[method(convertRectToBase:)]
        pub unsafe fn convertRectToBase(&self, rect: NSRect) -> NSRect;
        #[method(convertRectFromBase:)]
        pub unsafe fn convertRectFromBase(&self, rect: NSRect) -> NSRect;
        #[method(performMnemonic:)]
        pub unsafe fn performMnemonic(&self, string: &NSString) -> bool;
        #[method(shouldDrawColor)]
        pub unsafe fn shouldDrawColor(&self) -> bool;
        #[method(gState)]
        pub unsafe fn gState(&self) -> NSInteger;
        #[method(allocateGState)]
        pub unsafe fn allocateGState(&self);
        #[method(setUpGState)]
        pub unsafe fn setUpGState(&self);
        #[method(renewGState)]
        pub unsafe fn renewGState(&self);
    }
);