#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSWindowLevel = NSInteger;
pub type NSWindowFrameAutosaveName = NSString;
pub type NSWindowPersistableFrameDescriptor = NSString;
pub type NSWindowTabbingIdentifier = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSWindow;
    unsafe impl ClassType for NSWindow {
        type Super = NSResponder;
    }
);
extern_methods!(
    unsafe impl NSWindow {
        #[method(frameRectForContentRect:styleMask:)]
        pub unsafe fn frameRectForContentRect_styleMask(
            cRect: NSRect,
            style: NSWindowStyleMask,
        ) -> NSRect;
        #[method(contentRectForFrameRect:styleMask:)]
        pub unsafe fn contentRectForFrameRect_styleMask(
            fRect: NSRect,
            style: NSWindowStyleMask,
        ) -> NSRect;
        #[method(minFrameWidthWithTitle:styleMask:)]
        pub unsafe fn minFrameWidthWithTitle_styleMask(
            title: &NSString,
            style: NSWindowStyleMask,
        ) -> CGFloat;
        #[method(defaultDepthLimit)]
        pub unsafe fn defaultDepthLimit() -> NSWindowDepth;
        #[method(frameRectForContentRect:)]
        pub unsafe fn frameRectForContentRect(&self, contentRect: NSRect) -> NSRect;
        #[method(contentRectForFrameRect:)]
        pub unsafe fn contentRectForFrameRect(&self, frameRect: NSRect) -> NSRect;
        #[method_id(initWithContentRect:styleMask:backing:defer:)]
        pub unsafe fn initWithContentRect_styleMask_backing_defer(
            &self,
            contentRect: NSRect,
            style: NSWindowStyleMask,
            backingStoreType: NSBackingStoreType,
            flag: bool,
        ) -> Id<Self, Shared>;
        #[method_id(initWithContentRect:styleMask:backing:defer:screen:)]
        pub unsafe fn initWithContentRect_styleMask_backing_defer_screen(
            &self,
            contentRect: NSRect,
            style: NSWindowStyleMask,
            backingStoreType: NSBackingStoreType,
            flag: bool,
            screen: Option<&NSScreen>,
        ) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Id<Self, Shared>;
        #[method_id(title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);
        #[method_id(subtitle)]
        pub unsafe fn subtitle(&self) -> Id<NSString, Shared>;
        #[method(setSubtitle:)]
        pub unsafe fn setSubtitle(&self, subtitle: &NSString);
        #[method(titleVisibility)]
        pub unsafe fn titleVisibility(&self) -> NSWindowTitleVisibility;
        #[method(setTitleVisibility:)]
        pub unsafe fn setTitleVisibility(&self, titleVisibility: NSWindowTitleVisibility);
        #[method(titlebarAppearsTransparent)]
        pub unsafe fn titlebarAppearsTransparent(&self) -> bool;
        #[method(setTitlebarAppearsTransparent:)]
        pub unsafe fn setTitlebarAppearsTransparent(&self, titlebarAppearsTransparent: bool);
        #[method(toolbarStyle)]
        pub unsafe fn toolbarStyle(&self) -> NSWindowToolbarStyle;
        #[method(setToolbarStyle:)]
        pub unsafe fn setToolbarStyle(&self, toolbarStyle: NSWindowToolbarStyle);
        #[method(contentLayoutRect)]
        pub unsafe fn contentLayoutRect(&self) -> NSRect;
        #[method_id(contentLayoutGuide)]
        pub unsafe fn contentLayoutGuide(&self) -> Option<Id<Object, Shared>>;
        #[method_id(titlebarAccessoryViewControllers)]
        pub unsafe fn titlebarAccessoryViewControllers(
            &self,
        ) -> Id<NSArray<NSTitlebarAccessoryViewController>, Shared>;
        #[method(setTitlebarAccessoryViewControllers:)]
        pub unsafe fn setTitlebarAccessoryViewControllers(
            &self,
            titlebarAccessoryViewControllers: &NSArray<NSTitlebarAccessoryViewController>,
        );
        #[method(addTitlebarAccessoryViewController:)]
        pub unsafe fn addTitlebarAccessoryViewController(
            &self,
            childViewController: &NSTitlebarAccessoryViewController,
        );
        #[method(insertTitlebarAccessoryViewController:atIndex:)]
        pub unsafe fn insertTitlebarAccessoryViewController_atIndex(
            &self,
            childViewController: &NSTitlebarAccessoryViewController,
            index: NSInteger,
        );
        #[method(removeTitlebarAccessoryViewControllerAtIndex:)]
        pub unsafe fn removeTitlebarAccessoryViewControllerAtIndex(&self, index: NSInteger);
        #[method_id(representedURL)]
        pub unsafe fn representedURL(&self) -> Option<Id<NSURL, Shared>>;
        #[method(setRepresentedURL:)]
        pub unsafe fn setRepresentedURL(&self, representedURL: Option<&NSURL>);
        #[method_id(representedFilename)]
        pub unsafe fn representedFilename(&self) -> Id<NSString, Shared>;
        #[method(setRepresentedFilename:)]
        pub unsafe fn setRepresentedFilename(&self, representedFilename: &NSString);
        #[method(setTitleWithRepresentedFilename:)]
        pub unsafe fn setTitleWithRepresentedFilename(&self, filename: &NSString);
        #[method(isExcludedFromWindowsMenu)]
        pub unsafe fn isExcludedFromWindowsMenu(&self) -> bool;
        #[method(setExcludedFromWindowsMenu:)]
        pub unsafe fn setExcludedFromWindowsMenu(&self, excludedFromWindowsMenu: bool);
        #[method_id(contentView)]
        pub unsafe fn contentView(&self) -> Option<Id<NSView, Shared>>;
        #[method(setContentView:)]
        pub unsafe fn setContentView(&self, contentView: Option<&NSView>);
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSWindowDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSWindowDelegate>);
        #[method(windowNumber)]
        pub unsafe fn windowNumber(&self) -> NSInteger;
        #[method(styleMask)]
        pub unsafe fn styleMask(&self) -> NSWindowStyleMask;
        #[method(setStyleMask:)]
        pub unsafe fn setStyleMask(&self, styleMask: NSWindowStyleMask);
        #[method_id(fieldEditor:forObject:)]
        pub unsafe fn fieldEditor_forObject(
            &self,
            createFlag: bool,
            object: Option<&Object>,
        ) -> Option<Id<NSText, Shared>>;
        #[method(endEditingFor:)]
        pub unsafe fn endEditingFor(&self, object: Option<&Object>);
        #[method(constrainFrameRect:toScreen:)]
        pub unsafe fn constrainFrameRect_toScreen(
            &self,
            frameRect: NSRect,
            screen: Option<&NSScreen>,
        ) -> NSRect;
        #[method(setFrame:display:)]
        pub unsafe fn setFrame_display(&self, frameRect: NSRect, flag: bool);
        #[method(setContentSize:)]
        pub unsafe fn setContentSize(&self, size: NSSize);
        #[method(setFrameOrigin:)]
        pub unsafe fn setFrameOrigin(&self, point: NSPoint);
        #[method(setFrameTopLeftPoint:)]
        pub unsafe fn setFrameTopLeftPoint(&self, point: NSPoint);
        #[method(cascadeTopLeftFromPoint:)]
        pub unsafe fn cascadeTopLeftFromPoint(&self, topLeftPoint: NSPoint) -> NSPoint;
        #[method(frame)]
        pub unsafe fn frame(&self) -> NSRect;
        #[method(animationResizeTime:)]
        pub unsafe fn animationResizeTime(&self, newFrame: NSRect) -> NSTimeInterval;
        #[method(setFrame:display:animate:)]
        pub unsafe fn setFrame_display_animate(
            &self,
            frameRect: NSRect,
            displayFlag: bool,
            animateFlag: bool,
        );
        #[method(inLiveResize)]
        pub unsafe fn inLiveResize(&self) -> bool;
        #[method(resizeIncrements)]
        pub unsafe fn resizeIncrements(&self) -> NSSize;
        #[method(setResizeIncrements:)]
        pub unsafe fn setResizeIncrements(&self, resizeIncrements: NSSize);
        #[method(aspectRatio)]
        pub unsafe fn aspectRatio(&self) -> NSSize;
        #[method(setAspectRatio:)]
        pub unsafe fn setAspectRatio(&self, aspectRatio: NSSize);
        #[method(contentResizeIncrements)]
        pub unsafe fn contentResizeIncrements(&self) -> NSSize;
        #[method(setContentResizeIncrements:)]
        pub unsafe fn setContentResizeIncrements(&self, contentResizeIncrements: NSSize);
        #[method(contentAspectRatio)]
        pub unsafe fn contentAspectRatio(&self) -> NSSize;
        #[method(setContentAspectRatio:)]
        pub unsafe fn setContentAspectRatio(&self, contentAspectRatio: NSSize);
        #[method(viewsNeedDisplay)]
        pub unsafe fn viewsNeedDisplay(&self) -> bool;
        #[method(setViewsNeedDisplay:)]
        pub unsafe fn setViewsNeedDisplay(&self, viewsNeedDisplay: bool);
        #[method(displayIfNeeded)]
        pub unsafe fn displayIfNeeded(&self);
        #[method(display)]
        pub unsafe fn display(&self);
        #[method(preservesContentDuringLiveResize)]
        pub unsafe fn preservesContentDuringLiveResize(&self) -> bool;
        #[method(setPreservesContentDuringLiveResize:)]
        pub unsafe fn setPreservesContentDuringLiveResize(
            &self,
            preservesContentDuringLiveResize: bool,
        );
        #[method(update)]
        pub unsafe fn update(&self);
        #[method(makeFirstResponder:)]
        pub unsafe fn makeFirstResponder(&self, responder: Option<&NSResponder>) -> bool;
        #[method_id(firstResponder)]
        pub unsafe fn firstResponder(&self) -> Option<Id<NSResponder, Shared>>;
        #[method(resizeFlags)]
        pub unsafe fn resizeFlags(&self) -> NSEventModifierFlags;
        #[method(close)]
        pub unsafe fn close(&self);
        #[method(isReleasedWhenClosed)]
        pub unsafe fn isReleasedWhenClosed(&self) -> bool;
        #[method(setReleasedWhenClosed:)]
        pub unsafe fn setReleasedWhenClosed(&self, releasedWhenClosed: bool);
        #[method(miniaturize:)]
        pub unsafe fn miniaturize(&self, sender: Option<&Object>);
        #[method(deminiaturize:)]
        pub unsafe fn deminiaturize(&self, sender: Option<&Object>);
        #[method(isZoomed)]
        pub unsafe fn isZoomed(&self) -> bool;
        #[method(zoom:)]
        pub unsafe fn zoom(&self, sender: Option<&Object>);
        #[method(isMiniaturized)]
        pub unsafe fn isMiniaturized(&self) -> bool;
        #[method(tryToPerform:with:)]
        pub unsafe fn tryToPerform_with(&self, action: Sel, object: Option<&Object>) -> bool;
        #[method_id(validRequestorForSendType:returnType:)]
        pub unsafe fn validRequestorForSendType_returnType(
            &self,
            sendType: Option<&NSPasteboardType>,
            returnType: Option<&NSPasteboardType>,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<NSColor, Shared>;
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, backgroundColor: Option<&NSColor>);
        #[method(setContentBorderThickness:forEdge:)]
        pub unsafe fn setContentBorderThickness_forEdge(
            &self,
            thickness: CGFloat,
            edge: NSRectEdge,
        );
        #[method(contentBorderThicknessForEdge:)]
        pub unsafe fn contentBorderThicknessForEdge(&self, edge: NSRectEdge) -> CGFloat;
        #[method(setAutorecalculatesContentBorderThickness:forEdge:)]
        pub unsafe fn setAutorecalculatesContentBorderThickness_forEdge(
            &self,
            flag: bool,
            edge: NSRectEdge,
        );
        #[method(autorecalculatesContentBorderThicknessForEdge:)]
        pub unsafe fn autorecalculatesContentBorderThicknessForEdge(
            &self,
            edge: NSRectEdge,
        ) -> bool;
        #[method(isMovable)]
        pub unsafe fn isMovable(&self) -> bool;
        #[method(setMovable:)]
        pub unsafe fn setMovable(&self, movable: bool);
        #[method(isMovableByWindowBackground)]
        pub unsafe fn isMovableByWindowBackground(&self) -> bool;
        #[method(setMovableByWindowBackground:)]
        pub unsafe fn setMovableByWindowBackground(&self, movableByWindowBackground: bool);
        #[method(hidesOnDeactivate)]
        pub unsafe fn hidesOnDeactivate(&self) -> bool;
        #[method(setHidesOnDeactivate:)]
        pub unsafe fn setHidesOnDeactivate(&self, hidesOnDeactivate: bool);
        #[method(canHide)]
        pub unsafe fn canHide(&self) -> bool;
        #[method(setCanHide:)]
        pub unsafe fn setCanHide(&self, canHide: bool);
        #[method(center)]
        pub unsafe fn center(&self);
        #[method(makeKeyAndOrderFront:)]
        pub unsafe fn makeKeyAndOrderFront(&self, sender: Option<&Object>);
        #[method(orderFront:)]
        pub unsafe fn orderFront(&self, sender: Option<&Object>);
        #[method(orderBack:)]
        pub unsafe fn orderBack(&self, sender: Option<&Object>);
        #[method(orderOut:)]
        pub unsafe fn orderOut(&self, sender: Option<&Object>);
        #[method(orderWindow:relativeTo:)]
        pub unsafe fn orderWindow_relativeTo(
            &self,
            place: NSWindowOrderingMode,
            otherWin: NSInteger,
        );
        #[method(orderFrontRegardless)]
        pub unsafe fn orderFrontRegardless(&self);
        #[method_id(miniwindowImage)]
        pub unsafe fn miniwindowImage(&self) -> Option<Id<NSImage, Shared>>;
        #[method(setMiniwindowImage:)]
        pub unsafe fn setMiniwindowImage(&self, miniwindowImage: Option<&NSImage>);
        #[method_id(miniwindowTitle)]
        pub unsafe fn miniwindowTitle(&self) -> Id<NSString, Shared>;
        #[method(setMiniwindowTitle:)]
        pub unsafe fn setMiniwindowTitle(&self, miniwindowTitle: Option<&NSString>);
        #[method_id(dockTile)]
        pub unsafe fn dockTile(&self) -> Id<NSDockTile, Shared>;
        #[method(isDocumentEdited)]
        pub unsafe fn isDocumentEdited(&self) -> bool;
        #[method(setDocumentEdited:)]
        pub unsafe fn setDocumentEdited(&self, documentEdited: bool);
        #[method(isVisible)]
        pub unsafe fn isVisible(&self) -> bool;
        #[method(isKeyWindow)]
        pub unsafe fn isKeyWindow(&self) -> bool;
        #[method(isMainWindow)]
        pub unsafe fn isMainWindow(&self) -> bool;
        #[method(canBecomeKeyWindow)]
        pub unsafe fn canBecomeKeyWindow(&self) -> bool;
        #[method(canBecomeMainWindow)]
        pub unsafe fn canBecomeMainWindow(&self) -> bool;
        #[method(makeKeyWindow)]
        pub unsafe fn makeKeyWindow(&self);
        #[method(makeMainWindow)]
        pub unsafe fn makeMainWindow(&self);
        #[method(becomeKeyWindow)]
        pub unsafe fn becomeKeyWindow(&self);
        #[method(resignKeyWindow)]
        pub unsafe fn resignKeyWindow(&self);
        #[method(becomeMainWindow)]
        pub unsafe fn becomeMainWindow(&self);
        #[method(resignMainWindow)]
        pub unsafe fn resignMainWindow(&self);
        #[method(worksWhenModal)]
        pub unsafe fn worksWhenModal(&self) -> bool;
        #[method(preventsApplicationTerminationWhenModal)]
        pub unsafe fn preventsApplicationTerminationWhenModal(&self) -> bool;
        #[method(setPreventsApplicationTerminationWhenModal:)]
        pub unsafe fn setPreventsApplicationTerminationWhenModal(
            &self,
            preventsApplicationTerminationWhenModal: bool,
        );
        #[method(convertRectToScreen:)]
        pub unsafe fn convertRectToScreen(&self, rect: NSRect) -> NSRect;
        #[method(convertRectFromScreen:)]
        pub unsafe fn convertRectFromScreen(&self, rect: NSRect) -> NSRect;
        #[method(convertPointToScreen:)]
        pub unsafe fn convertPointToScreen(&self, point: NSPoint) -> NSPoint;
        #[method(convertPointFromScreen:)]
        pub unsafe fn convertPointFromScreen(&self, point: NSPoint) -> NSPoint;
        #[method(convertRectToBacking:)]
        pub unsafe fn convertRectToBacking(&self, rect: NSRect) -> NSRect;
        #[method(convertRectFromBacking:)]
        pub unsafe fn convertRectFromBacking(&self, rect: NSRect) -> NSRect;
        #[method(convertPointToBacking:)]
        pub unsafe fn convertPointToBacking(&self, point: NSPoint) -> NSPoint;
        #[method(convertPointFromBacking:)]
        pub unsafe fn convertPointFromBacking(&self, point: NSPoint) -> NSPoint;
        #[method(backingAlignedRect:options:)]
        pub unsafe fn backingAlignedRect_options(
            &self,
            rect: NSRect,
            options: NSAlignmentOptions,
        ) -> NSRect;
        #[method(backingScaleFactor)]
        pub unsafe fn backingScaleFactor(&self) -> CGFloat;
        #[method(performClose:)]
        pub unsafe fn performClose(&self, sender: Option<&Object>);
        #[method(performMiniaturize:)]
        pub unsafe fn performMiniaturize(&self, sender: Option<&Object>);
        #[method(performZoom:)]
        pub unsafe fn performZoom(&self, sender: Option<&Object>);
        #[method_id(dataWithEPSInsideRect:)]
        pub unsafe fn dataWithEPSInsideRect(&self, rect: NSRect) -> Id<NSData, Shared>;
        #[method_id(dataWithPDFInsideRect:)]
        pub unsafe fn dataWithPDFInsideRect(&self, rect: NSRect) -> Id<NSData, Shared>;
        #[method(print:)]
        pub unsafe fn print(&self, sender: Option<&Object>);
        #[method(allowsToolTipsWhenApplicationIsInactive)]
        pub unsafe fn allowsToolTipsWhenApplicationIsInactive(&self) -> bool;
        #[method(setAllowsToolTipsWhenApplicationIsInactive:)]
        pub unsafe fn setAllowsToolTipsWhenApplicationIsInactive(
            &self,
            allowsToolTipsWhenApplicationIsInactive: bool,
        );
        #[method(backingType)]
        pub unsafe fn backingType(&self) -> NSBackingStoreType;
        #[method(setBackingType:)]
        pub unsafe fn setBackingType(&self, backingType: NSBackingStoreType);
        #[method(level)]
        pub unsafe fn level(&self) -> NSWindowLevel;
        #[method(setLevel:)]
        pub unsafe fn setLevel(&self, level: NSWindowLevel);
        #[method(depthLimit)]
        pub unsafe fn depthLimit(&self) -> NSWindowDepth;
        #[method(setDepthLimit:)]
        pub unsafe fn setDepthLimit(&self, depthLimit: NSWindowDepth);
        #[method(setDynamicDepthLimit:)]
        pub unsafe fn setDynamicDepthLimit(&self, flag: bool);
        #[method(hasDynamicDepthLimit)]
        pub unsafe fn hasDynamicDepthLimit(&self) -> bool;
        #[method_id(screen)]
        pub unsafe fn screen(&self) -> Option<Id<NSScreen, Shared>>;
        #[method_id(deepestScreen)]
        pub unsafe fn deepestScreen(&self) -> Option<Id<NSScreen, Shared>>;
        #[method(hasShadow)]
        pub unsafe fn hasShadow(&self) -> bool;
        #[method(setHasShadow:)]
        pub unsafe fn setHasShadow(&self, hasShadow: bool);
        #[method(invalidateShadow)]
        pub unsafe fn invalidateShadow(&self);
        #[method(alphaValue)]
        pub unsafe fn alphaValue(&self) -> CGFloat;
        #[method(setAlphaValue:)]
        pub unsafe fn setAlphaValue(&self, alphaValue: CGFloat);
        #[method(isOpaque)]
        pub unsafe fn isOpaque(&self) -> bool;
        #[method(setOpaque:)]
        pub unsafe fn setOpaque(&self, opaque: bool);
        #[method(sharingType)]
        pub unsafe fn sharingType(&self) -> NSWindowSharingType;
        #[method(setSharingType:)]
        pub unsafe fn setSharingType(&self, sharingType: NSWindowSharingType);
        #[method(allowsConcurrentViewDrawing)]
        pub unsafe fn allowsConcurrentViewDrawing(&self) -> bool;
        #[method(setAllowsConcurrentViewDrawing:)]
        pub unsafe fn setAllowsConcurrentViewDrawing(&self, allowsConcurrentViewDrawing: bool);
        #[method(displaysWhenScreenProfileChanges)]
        pub unsafe fn displaysWhenScreenProfileChanges(&self) -> bool;
        #[method(setDisplaysWhenScreenProfileChanges:)]
        pub unsafe fn setDisplaysWhenScreenProfileChanges(
            &self,
            displaysWhenScreenProfileChanges: bool,
        );
        #[method(disableScreenUpdatesUntilFlush)]
        pub unsafe fn disableScreenUpdatesUntilFlush(&self);
        #[method(canBecomeVisibleWithoutLogin)]
        pub unsafe fn canBecomeVisibleWithoutLogin(&self) -> bool;
        #[method(setCanBecomeVisibleWithoutLogin:)]
        pub unsafe fn setCanBecomeVisibleWithoutLogin(&self, canBecomeVisibleWithoutLogin: bool);
        #[method(collectionBehavior)]
        pub unsafe fn collectionBehavior(&self) -> NSWindowCollectionBehavior;
        #[method(setCollectionBehavior:)]
        pub unsafe fn setCollectionBehavior(&self, collectionBehavior: NSWindowCollectionBehavior);
        #[method(animationBehavior)]
        pub unsafe fn animationBehavior(&self) -> NSWindowAnimationBehavior;
        #[method(setAnimationBehavior:)]
        pub unsafe fn setAnimationBehavior(&self, animationBehavior: NSWindowAnimationBehavior);
        #[method(isOnActiveSpace)]
        pub unsafe fn isOnActiveSpace(&self) -> bool;
        #[method(toggleFullScreen:)]
        pub unsafe fn toggleFullScreen(&self, sender: Option<&Object>);
        #[method_id(stringWithSavedFrame)]
        pub unsafe fn stringWithSavedFrame(&self)
            -> Id<NSWindowPersistableFrameDescriptor, Shared>;
        #[method(setFrameFromString:)]
        pub unsafe fn setFrameFromString(&self, string: &NSWindowPersistableFrameDescriptor);
        #[method(saveFrameUsingName:)]
        pub unsafe fn saveFrameUsingName(&self, name: &NSWindowFrameAutosaveName);
        #[method(setFrameUsingName:force:)]
        pub unsafe fn setFrameUsingName_force(
            &self,
            name: &NSWindowFrameAutosaveName,
            force: bool,
        ) -> bool;
        #[method(setFrameUsingName:)]
        pub unsafe fn setFrameUsingName(&self, name: &NSWindowFrameAutosaveName) -> bool;
        #[method(setFrameAutosaveName:)]
        pub unsafe fn setFrameAutosaveName(&self, name: &NSWindowFrameAutosaveName) -> bool;
        #[method_id(frameAutosaveName)]
        pub unsafe fn frameAutosaveName(&self) -> Id<NSWindowFrameAutosaveName, Shared>;
        #[method(removeFrameUsingName:)]
        pub unsafe fn removeFrameUsingName(name: &NSWindowFrameAutosaveName);
        #[method(minSize)]
        pub unsafe fn minSize(&self) -> NSSize;
        #[method(setMinSize:)]
        pub unsafe fn setMinSize(&self, minSize: NSSize);
        #[method(maxSize)]
        pub unsafe fn maxSize(&self) -> NSSize;
        #[method(setMaxSize:)]
        pub unsafe fn setMaxSize(&self, maxSize: NSSize);
        #[method(contentMinSize)]
        pub unsafe fn contentMinSize(&self) -> NSSize;
        #[method(setContentMinSize:)]
        pub unsafe fn setContentMinSize(&self, contentMinSize: NSSize);
        #[method(contentMaxSize)]
        pub unsafe fn contentMaxSize(&self) -> NSSize;
        #[method(setContentMaxSize:)]
        pub unsafe fn setContentMaxSize(&self, contentMaxSize: NSSize);
        #[method(minFullScreenContentSize)]
        pub unsafe fn minFullScreenContentSize(&self) -> NSSize;
        #[method(setMinFullScreenContentSize:)]
        pub unsafe fn setMinFullScreenContentSize(&self, minFullScreenContentSize: NSSize);
        #[method(maxFullScreenContentSize)]
        pub unsafe fn maxFullScreenContentSize(&self) -> NSSize;
        #[method(setMaxFullScreenContentSize:)]
        pub unsafe fn setMaxFullScreenContentSize(&self, maxFullScreenContentSize: NSSize);
        #[method_id(deviceDescription)]
        pub unsafe fn deviceDescription(
            &self,
        ) -> Id<NSDictionary<NSDeviceDescriptionKey, Object>, Shared>;
        #[method_id(windowController)]
        pub unsafe fn windowController(&self) -> Option<Id<NSWindowController, Shared>>;
        #[method(setWindowController:)]
        pub unsafe fn setWindowController(&self, windowController: Option<&NSWindowController>);
        #[method(beginSheet:completionHandler:)]
        pub unsafe fn beginSheet_completionHandler(
            &self,
            sheetWindow: &NSWindow,
            handler: TodoBlock,
        );
        #[method(beginCriticalSheet:completionHandler:)]
        pub unsafe fn beginCriticalSheet_completionHandler(
            &self,
            sheetWindow: &NSWindow,
            handler: TodoBlock,
        );
        #[method(endSheet:)]
        pub unsafe fn endSheet(&self, sheetWindow: &NSWindow);
        #[method(endSheet:returnCode:)]
        pub unsafe fn endSheet_returnCode(
            &self,
            sheetWindow: &NSWindow,
            returnCode: NSModalResponse,
        );
        #[method_id(sheets)]
        pub unsafe fn sheets(&self) -> Id<NSArray<NSWindow>, Shared>;
        #[method_id(attachedSheet)]
        pub unsafe fn attachedSheet(&self) -> Option<Id<NSWindow, Shared>>;
        #[method(isSheet)]
        pub unsafe fn isSheet(&self) -> bool;
        #[method_id(sheetParent)]
        pub unsafe fn sheetParent(&self) -> Option<Id<NSWindow, Shared>>;
        #[method_id(standardWindowButton:forStyleMask:)]
        pub unsafe fn standardWindowButton_forStyleMask(
            b: NSWindowButton,
            styleMask: NSWindowStyleMask,
        ) -> Option<Id<NSButton, Shared>>;
        #[method_id(standardWindowButton:)]
        pub unsafe fn standardWindowButton(
            &self,
            b: NSWindowButton,
        ) -> Option<Id<NSButton, Shared>>;
        #[method(addChildWindow:ordered:)]
        pub unsafe fn addChildWindow_ordered(
            &self,
            childWin: &NSWindow,
            place: NSWindowOrderingMode,
        );
        #[method(removeChildWindow:)]
        pub unsafe fn removeChildWindow(&self, childWin: &NSWindow);
        #[method_id(childWindows)]
        pub unsafe fn childWindows(&self) -> Option<Id<NSArray<NSWindow>, Shared>>;
        #[method_id(parentWindow)]
        pub unsafe fn parentWindow(&self) -> Option<Id<NSWindow, Shared>>;
        #[method(setParentWindow:)]
        pub unsafe fn setParentWindow(&self, parentWindow: Option<&NSWindow>);
        #[method_id(appearanceSource)]
        pub unsafe fn appearanceSource(&self) -> Option<Id<TodoProtocols, Shared>>;
        #[method(setAppearanceSource:)]
        pub unsafe fn setAppearanceSource(&self, appearanceSource: Option<&TodoProtocols>);
        #[method_id(colorSpace)]
        pub unsafe fn colorSpace(&self) -> Option<Id<NSColorSpace, Shared>>;
        #[method(setColorSpace:)]
        pub unsafe fn setColorSpace(&self, colorSpace: Option<&NSColorSpace>);
        #[method(canRepresentDisplayGamut:)]
        pub unsafe fn canRepresentDisplayGamut(&self, displayGamut: NSDisplayGamut) -> bool;
        #[method_id(windowNumbersWithOptions:)]
        pub unsafe fn windowNumbersWithOptions(
            options: NSWindowNumberListOptions,
        ) -> Option<Id<NSArray<NSNumber>, Shared>>;
        #[method(windowNumberAtPoint:belowWindowWithWindowNumber:)]
        pub unsafe fn windowNumberAtPoint_belowWindowWithWindowNumber(
            point: NSPoint,
            windowNumber: NSInteger,
        ) -> NSInteger;
        #[method(occlusionState)]
        pub unsafe fn occlusionState(&self) -> NSWindowOcclusionState;
        #[method(titlebarSeparatorStyle)]
        pub unsafe fn titlebarSeparatorStyle(&self) -> NSTitlebarSeparatorStyle;
        #[method(setTitlebarSeparatorStyle:)]
        pub unsafe fn setTitlebarSeparatorStyle(
            &self,
            titlebarSeparatorStyle: NSTitlebarSeparatorStyle,
        );
        #[method_id(contentViewController)]
        pub unsafe fn contentViewController(&self) -> Option<Id<NSViewController, Shared>>;
        #[method(setContentViewController:)]
        pub unsafe fn setContentViewController(
            &self,
            contentViewController: Option<&NSViewController>,
        );
        #[method_id(windowWithContentViewController:)]
        pub unsafe fn windowWithContentViewController(
            contentViewController: &NSViewController,
        ) -> Id<Self, Shared>;
        #[method(performWindowDragWithEvent:)]
        pub unsafe fn performWindowDragWithEvent(&self, event: &NSEvent);
        #[method_id(initialFirstResponder)]
        pub unsafe fn initialFirstResponder(&self) -> Option<Id<NSView, Shared>>;
        #[method(setInitialFirstResponder:)]
        pub unsafe fn setInitialFirstResponder(&self, initialFirstResponder: Option<&NSView>);
        #[method(selectNextKeyView:)]
        pub unsafe fn selectNextKeyView(&self, sender: Option<&Object>);
        #[method(selectPreviousKeyView:)]
        pub unsafe fn selectPreviousKeyView(&self, sender: Option<&Object>);
        #[method(selectKeyViewFollowingView:)]
        pub unsafe fn selectKeyViewFollowingView(&self, view: &NSView);
        #[method(selectKeyViewPrecedingView:)]
        pub unsafe fn selectKeyViewPrecedingView(&self, view: &NSView);
        #[method(keyViewSelectionDirection)]
        pub unsafe fn keyViewSelectionDirection(&self) -> NSSelectionDirection;
        #[method_id(defaultButtonCell)]
        pub unsafe fn defaultButtonCell(&self) -> Option<Id<NSButtonCell, Shared>>;
        #[method(setDefaultButtonCell:)]
        pub unsafe fn setDefaultButtonCell(&self, defaultButtonCell: Option<&NSButtonCell>);
        #[method(disableKeyEquivalentForDefaultButtonCell)]
        pub unsafe fn disableKeyEquivalentForDefaultButtonCell(&self);
        #[method(enableKeyEquivalentForDefaultButtonCell)]
        pub unsafe fn enableKeyEquivalentForDefaultButtonCell(&self);
        #[method(autorecalculatesKeyViewLoop)]
        pub unsafe fn autorecalculatesKeyViewLoop(&self) -> bool;
        #[method(setAutorecalculatesKeyViewLoop:)]
        pub unsafe fn setAutorecalculatesKeyViewLoop(&self, autorecalculatesKeyViewLoop: bool);
        #[method(recalculateKeyViewLoop)]
        pub unsafe fn recalculateKeyViewLoop(&self);
        #[method_id(toolbar)]
        pub unsafe fn toolbar(&self) -> Option<Id<NSToolbar, Shared>>;
        #[method(setToolbar:)]
        pub unsafe fn setToolbar(&self, toolbar: Option<&NSToolbar>);
        #[method(toggleToolbarShown:)]
        pub unsafe fn toggleToolbarShown(&self, sender: Option<&Object>);
        #[method(runToolbarCustomizationPalette:)]
        pub unsafe fn runToolbarCustomizationPalette(&self, sender: Option<&Object>);
        #[method(showsToolbarButton)]
        pub unsafe fn showsToolbarButton(&self) -> bool;
        #[method(setShowsToolbarButton:)]
        pub unsafe fn setShowsToolbarButton(&self, showsToolbarButton: bool);
        #[method(allowsAutomaticWindowTabbing)]
        pub unsafe fn allowsAutomaticWindowTabbing() -> bool;
        #[method(setAllowsAutomaticWindowTabbing:)]
        pub unsafe fn setAllowsAutomaticWindowTabbing(allowsAutomaticWindowTabbing: bool);
        #[method(userTabbingPreference)]
        pub unsafe fn userTabbingPreference() -> NSWindowUserTabbingPreference;
        #[method(tabbingMode)]
        pub unsafe fn tabbingMode(&self) -> NSWindowTabbingMode;
        #[method(setTabbingMode:)]
        pub unsafe fn setTabbingMode(&self, tabbingMode: NSWindowTabbingMode);
        #[method_id(tabbingIdentifier)]
        pub unsafe fn tabbingIdentifier(&self) -> Id<NSWindowTabbingIdentifier, Shared>;
        #[method(setTabbingIdentifier:)]
        pub unsafe fn setTabbingIdentifier(&self, tabbingIdentifier: &NSWindowTabbingIdentifier);
        #[method(selectNextTab:)]
        pub unsafe fn selectNextTab(&self, sender: Option<&Object>);
        #[method(selectPreviousTab:)]
        pub unsafe fn selectPreviousTab(&self, sender: Option<&Object>);
        #[method(moveTabToNewWindow:)]
        pub unsafe fn moveTabToNewWindow(&self, sender: Option<&Object>);
        #[method(mergeAllWindows:)]
        pub unsafe fn mergeAllWindows(&self, sender: Option<&Object>);
        #[method(toggleTabBar:)]
        pub unsafe fn toggleTabBar(&self, sender: Option<&Object>);
        #[method(toggleTabOverview:)]
        pub unsafe fn toggleTabOverview(&self, sender: Option<&Object>);
        #[method_id(tabbedWindows)]
        pub unsafe fn tabbedWindows(&self) -> Option<Id<NSArray<NSWindow>, Shared>>;
        #[method(addTabbedWindow:ordered:)]
        pub unsafe fn addTabbedWindow_ordered(
            &self,
            window: &NSWindow,
            ordered: NSWindowOrderingMode,
        );
        #[method_id(tab)]
        pub unsafe fn tab(&self) -> Id<NSWindowTab, Shared>;
        #[method_id(tabGroup)]
        pub unsafe fn tabGroup(&self) -> Option<Id<NSWindowTabGroup, Shared>>;
        #[method(windowTitlebarLayoutDirection)]
        pub unsafe fn windowTitlebarLayoutDirection(&self) -> NSUserInterfaceLayoutDirection;
    }
);
extern_methods!(
    #[doc = "NSEvent"]
    unsafe impl NSWindow {
        #[method(trackEventsMatchingMask:timeout:mode:handler:)]
        pub unsafe fn trackEventsMatchingMask_timeout_mode_handler(
            &self,
            mask: NSEventMask,
            timeout: NSTimeInterval,
            mode: &NSRunLoopMode,
            trackingHandler: TodoBlock,
        );
        #[method_id(nextEventMatchingMask:)]
        pub unsafe fn nextEventMatchingMask(
            &self,
            mask: NSEventMask,
        ) -> Option<Id<NSEvent, Shared>>;
        #[method_id(nextEventMatchingMask:untilDate:inMode:dequeue:)]
        pub unsafe fn nextEventMatchingMask_untilDate_inMode_dequeue(
            &self,
            mask: NSEventMask,
            expiration: Option<&NSDate>,
            mode: &NSRunLoopMode,
            deqFlag: bool,
        ) -> Option<Id<NSEvent, Shared>>;
        #[method(discardEventsMatchingMask:beforeEvent:)]
        pub unsafe fn discardEventsMatchingMask_beforeEvent(
            &self,
            mask: NSEventMask,
            lastEvent: Option<&NSEvent>,
        );
        #[method(postEvent:atStart:)]
        pub unsafe fn postEvent_atStart(&self, event: &NSEvent, flag: bool);
        #[method(sendEvent:)]
        pub unsafe fn sendEvent(&self, event: &NSEvent);
        #[method_id(currentEvent)]
        pub unsafe fn currentEvent(&self) -> Option<Id<NSEvent, Shared>>;
        #[method(acceptsMouseMovedEvents)]
        pub unsafe fn acceptsMouseMovedEvents(&self) -> bool;
        #[method(setAcceptsMouseMovedEvents:)]
        pub unsafe fn setAcceptsMouseMovedEvents(&self, acceptsMouseMovedEvents: bool);
        #[method(ignoresMouseEvents)]
        pub unsafe fn ignoresMouseEvents(&self) -> bool;
        #[method(setIgnoresMouseEvents:)]
        pub unsafe fn setIgnoresMouseEvents(&self, ignoresMouseEvents: bool);
        #[method(mouseLocationOutsideOfEventStream)]
        pub unsafe fn mouseLocationOutsideOfEventStream(&self) -> NSPoint;
    }
);
extern_methods!(
    #[doc = "NSCursorRect"]
    unsafe impl NSWindow {
        #[method(disableCursorRects)]
        pub unsafe fn disableCursorRects(&self);
        #[method(enableCursorRects)]
        pub unsafe fn enableCursorRects(&self);
        #[method(discardCursorRects)]
        pub unsafe fn discardCursorRects(&self);
        #[method(areCursorRectsEnabled)]
        pub unsafe fn areCursorRectsEnabled(&self) -> bool;
        #[method(invalidateCursorRectsForView:)]
        pub unsafe fn invalidateCursorRectsForView(&self, view: &NSView);
        #[method(resetCursorRects)]
        pub unsafe fn resetCursorRects(&self);
    }
);
extern_methods!(
    #[doc = "NSDrag"]
    unsafe impl NSWindow {
        #[method(dragImage:at:offset:event:pasteboard:source:slideBack:)]
        pub unsafe fn dragImage_at_offset_event_pasteboard_source_slideBack(
            &self,
            image: &NSImage,
            baseLocation: NSPoint,
            initialOffset: NSSize,
            event: &NSEvent,
            pboard: &NSPasteboard,
            sourceObj: &Object,
            slideFlag: bool,
        );
        #[method(registerForDraggedTypes:)]
        pub unsafe fn registerForDraggedTypes(&self, newTypes: &NSArray<NSPasteboardType>);
        #[method(unregisterDraggedTypes)]
        pub unsafe fn unregisterDraggedTypes(&self);
    }
);
extern_methods!(
    #[doc = "NSCarbonExtensions"]
    unsafe impl NSWindow {
        #[method_id(initWithWindowRef:)]
        pub unsafe fn initWithWindowRef(
            &self,
            windowRef: NonNull<c_void>,
        ) -> Option<Id<NSWindow, Shared>>;
        #[method(windowRef)]
        pub unsafe fn windowRef(&self) -> NonNull<c_void>;
    }
);
pub type NSWindowDelegate = NSObject;
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSWindow {
        #[method(cacheImageInRect:)]
        pub unsafe fn cacheImageInRect(&self, rect: NSRect);
        #[method(restoreCachedImage)]
        pub unsafe fn restoreCachedImage(&self);
        #[method(discardCachedImage)]
        pub unsafe fn discardCachedImage(&self);
        #[method(menuChanged:)]
        pub unsafe fn menuChanged(menu: &NSMenu);
        #[method(gState)]
        pub unsafe fn gState(&self) -> NSInteger;
        #[method(convertBaseToScreen:)]
        pub unsafe fn convertBaseToScreen(&self, point: NSPoint) -> NSPoint;
        #[method(convertScreenToBase:)]
        pub unsafe fn convertScreenToBase(&self, point: NSPoint) -> NSPoint;
        #[method(userSpaceScaleFactor)]
        pub unsafe fn userSpaceScaleFactor(&self) -> CGFloat;
        #[method(useOptimizedDrawing:)]
        pub unsafe fn useOptimizedDrawing(&self, flag: bool);
        #[method(canStoreColor)]
        pub unsafe fn canStoreColor(&self) -> bool;
        #[method(disableFlushWindow)]
        pub unsafe fn disableFlushWindow(&self);
        #[method(enableFlushWindow)]
        pub unsafe fn enableFlushWindow(&self);
        #[method(isFlushWindowDisabled)]
        pub unsafe fn isFlushWindowDisabled(&self) -> bool;
        #[method(flushWindow)]
        pub unsafe fn flushWindow(&self);
        #[method(flushWindowIfNeeded)]
        pub unsafe fn flushWindowIfNeeded(&self);
        #[method(isAutodisplay)]
        pub unsafe fn isAutodisplay(&self) -> bool;
        #[method(setAutodisplay:)]
        pub unsafe fn setAutodisplay(&self, autodisplay: bool);
        #[method_id(graphicsContext)]
        pub unsafe fn graphicsContext(&self) -> Option<Id<NSGraphicsContext, Shared>>;
        #[method(isOneShot)]
        pub unsafe fn isOneShot(&self) -> bool;
        #[method(setOneShot:)]
        pub unsafe fn setOneShot(&self, oneShot: bool);
        #[method(preferredBackingLocation)]
        pub unsafe fn preferredBackingLocation(&self) -> NSWindowBackingLocation;
        #[method(setPreferredBackingLocation:)]
        pub unsafe fn setPreferredBackingLocation(
            &self,
            preferredBackingLocation: NSWindowBackingLocation,
        );
        #[method(backingLocation)]
        pub unsafe fn backingLocation(&self) -> NSWindowBackingLocation;
        #[method(showsResizeIndicator)]
        pub unsafe fn showsResizeIndicator(&self) -> bool;
        #[method(setShowsResizeIndicator:)]
        pub unsafe fn setShowsResizeIndicator(&self, showsResizeIndicator: bool);
    }
);
