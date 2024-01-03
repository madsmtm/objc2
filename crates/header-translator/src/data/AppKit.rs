// Most of these are marked as MainThreadOnly automatically
data! {
    // TODO: This should be one of MainThreadOnly or Immutable (+Send/Sync)
    class NSAppearance {
        unsafe -appearanceNamed;
        unsafe -bestMatchFromAppearancesWithNames;
    }

    class NSApplication {
        unsafe +sharedApplication;

        unsafe -currentEvent;
        unsafe -postEvent_atStart;
        unsafe -presentationOptions;
        unsafe -windows;
        unsafe -keyWindow;
        unsafe -setDelegate;
        unsafe -setPresentationOptions;
        unsafe -hide;
        unsafe -orderFrontCharacterPalette;
        unsafe -hideOtherApplications;
        unsafe -stop;
        unsafe -activateIgnoringOtherApps;
        unsafe -requestUserAttention;
        unsafe -setActivationPolicy;
        unsafe -setMainMenu;
        unsafe -effectiveAppearance;
        unsafe -setAppearance;

        // `run` cannot be safe, the user must ensure there is no re-entrancy.
    }

    class NSController: MainThreadOnly {}
    class NSObjectController: MainThreadOnly {}
    class NSArrayController: MainThreadOnly {}
    class NSDictionaryController: MainThreadOnly {}
    class NSTreeController: MainThreadOnly {}
    class NSUserDefaultsController: MainThreadOnly {}

    // Documentation says:
    // > Color objects are immutable and thread-safe
    //
    // TODO: Send + Sync
    class NSColor: Immutable {
        unsafe -clear;
    }

    class NSColorPicker: MainThreadOnly {}

    class NSControl {
        unsafe -isEnabled;
        unsafe -setEnabled;
    }

    // NSCursor is immutable, stated here:
    // https://developer.apple.com/documentation/appkit/nscursor/1527062-image?language=objc
    //
    // TODO: Send + Sync
    class NSCursor: Immutable {
        unsafe -initWithImage_hotSpot;

        unsafe -arrowCursor;
        unsafe -IBeamCursor;
        unsafe -pointingHandCursor;
        unsafe -closedHandCursor;
        unsafe -openHandCursor;
        unsafe -resizeLeftCursor;
        unsafe -resizeRightCursor;
        unsafe -resizeLeftRightCursor;
        unsafe -resizeUpCursor;
        unsafe -resizeDownCursor;
        unsafe -resizeUpDownCursor;
        unsafe -crosshairCursor;
        unsafe -disappearingItemCursor;
        unsafe -operationNotAllowedCursor;
        unsafe -dragLinkCursor;
        unsafe -dragCopyCursor;
        unsafe -contextualMenuCursor;
        unsafe -IBeamCursorForVerticalLayout;
    }

    // Since this is immutable, it _may_ be possible to make Send+Sync, but
    // let's refrain from doing so, because of:
    // > Safely handled only on the same thread, whether that be the main
    // > thread or a secondary thread; otherwise you run the risk of having
    // > events get out of sequence.
    // <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/CocoaFundamentals/AddingBehaviortoaCocoaProgram/AddingBehaviorCocoa.html#//apple_ref/doc/uid/TP40002974-CH5-SW47>
    // <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Multithreading/ThreadSafetySummary/ThreadSafetySummary.html#//apple_ref/doc/uid/10000057i-CH12-123383>
    class NSEvent: Immutable {

    }

    class NSFontManager: MainThreadOnly {}

    // Documented Thread-Unsafe, but:
    // > One thread can create an NSImage object, draw to the image buffer,
    // > and pass it off to the main thread for drawing. The underlying image
    // > cache is shared among all threads.
    // <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Multithreading/ThreadSafetySummary/ThreadSafetySummary.html#//apple_ref/doc/uid/10000057i-CH12-126728>
    //
    // So really only unsafe to mutate on several threads.
    //
    // Unsure yet if it would be beneficial to mark this as `Mutable`, or if
    // we should just keep it as interiormutable?
    class NSImage {
        unsafe -initWithData;
        unsafe -initByReferencingFile;
    }

    class NSMenu: MainThreadOnly {
        unsafe -init;
        unsafe -addItem;
    }

    // Any modification of the target or the action has to remain `unsafe`
    class NSMenuItem: MainThreadOnly {
        unsafe -init;
        unsafe +separatorItem;
        unsafe -setKeyEquivalentModifierMask;
        unsafe -setSubmenu;
    }

    class NSPasteboard {
        unsafe -propertyListForType;
    }

    // Documented as "Thread-Unsafe"
    class NSResponder {}

    // Accesses the shared application, and hence is main thread only (even
    // though not marked so in Swift).
    class NSScreen: MainThreadOnly {
        unsafe +mainScreen;
        unsafe +screens;
        unsafe -frame;
        unsafe -visibleFrame;
        unsafe -deviceDescription;
        unsafe -backingScaleFactor;
    }

    class NSWindowTabGroup: MainThreadOnly {
        unsafe -windows;
        unsafe -setSelectedWindow;
    }

    class NSTextInputContext: MainThreadOnly {
        unsafe -invalidateCharacterCoordinates;
        unsafe -discardMarkedText;
        unsafe -selectedKeyboardInputSource;
    }

    // Subclasses `NSMutableAttributedString`, though I think this should
    // actually be `InteriorMutable`?
    class NSTextStorage: Mutable {}

    // Documented as "Main Thread Only".
    // > generally thread safe, although operations on views such as creating,
    // > resizing, and moving should happen on the main thread.
    // <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/CocoaFundamentals/AddingBehaviortoaCocoaProgram/AddingBehaviorCocoa.html#//apple_ref/doc/uid/TP40002974-CH5-SW47>
    //
    // > If you want to use a thread to draw to a view, bracket all drawing code
    // > between the lockFocusIfCanDraw and unlockFocus methods of NSView.
    // <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Multithreading/ThreadSafetySummary/ThreadSafetySummary.html#//apple_ref/doc/uid/10000057i-CH12-123351-BBCFIIEB>
    class NSView {
        unsafe -frame;
        unsafe -bounds;
        unsafe -inputContext;
        unsafe -visibleRect;
        unsafe -hasMarkedText;
        unsafe -convertPoint_fromView;
        unsafe -window;

        unsafe -setWantsBestResolutionOpenGLSurface;
        unsafe -setWantsLayer;
        unsafe -setPostsFrameChangedNotifications;
        unsafe -removeTrackingRect;
        unsafe -addCursorRect_cursor;
        unsafe -setHidden;

        unsafe -convertRect_toView;
        unsafe -isFlipped;
    }

    // Documented as "Main Thread Only", but:
    // > Thread safe in that you can create and manage them on a secondary
    // > thread.
    // <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/CocoaFundamentals/AddingBehaviortoaCocoaProgram/AddingBehaviorCocoa.html#//apple_ref/doc/uid/TP40002974-CH5-SW47>
    // <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Multithreading/ThreadSafetySummary/ThreadSafetySummary.html#//apple_ref/doc/uid/10000057i-CH12-123364>
    //
    // So could in theory be `Send`, and perhaps also `Sync` - but we would
    // like interior mutability on windows, since that's just much easier, and
    // in that case, they can't be!
    class NSWindow {
        // Initializers are not safe, since it is critical to memory safety
        // that `window.setReleasedWhenClosed(false)` is called.

        unsafe -frame;
        unsafe -backingScaleFactor;
        unsafe -contentView;
        unsafe -setContentView;
        unsafe -setInitialFirstResponder;
        unsafe -makeFirstResponder;
        unsafe -contentRectForFrameRect;
        unsafe -screen;
        unsafe -setContentSize;
        unsafe -setFrameTopLeftPoint;
        unsafe -setMinSize;
        unsafe -setMaxSize;
        unsafe -setResizeIncrements;
        unsafe -contentResizeIncrements;
        unsafe -setContentResizeIncrements;
        unsafe -setFrame_display;
        unsafe -setMovable;
        unsafe -setSharingType;
        unsafe -setTabbingMode;
        unsafe -setOpaque;
        unsafe -hasShadow;
        unsafe -setHasShadow;
        unsafe -setIgnoresMouseEvents;
        unsafe -setBackgroundColor;
        unsafe -styleMask;
        unsafe -setStyleMask;
        unsafe -registerForDraggedTypes;
        unsafe -makeKeyAndOrderFront;
        unsafe -orderFront;
        unsafe -miniaturize;
        unsafe -sender;
        unsafe -toggleFullScreen;
        unsafe -orderOut;
        unsafe -zoom;
        unsafe -selectNextKeyView;
        unsafe -selectPreviousKeyView;
        unsafe -firstResponder;
        unsafe -standardWindowButton;
        unsafe -setTitle;
        unsafe -title;
        unsafe -setAcceptsMouseMovedEvents;
        unsafe -setTitlebarAppearsTransparent;
        unsafe -setTitleVisibility;
        unsafe -setMovableByWindowBackground;
        unsafe -setLevel;
        unsafe -setAllowsAutomaticWindowTabbing;
        unsafe -setTabbingIdentifier;
        unsafe -setDocumentEdited;
        unsafe -occlusionState;
        unsafe -center;
        unsafe -isResizable;
        unsafe -isMiniaturizable;
        unsafe -hasCloseBox;
        unsafe -isMiniaturized;
        unsafe -isVisible;
        unsafe -isKeyWindow;
        unsafe -isZoomed;
        unsafe -allowsAutomaticWindowTabbing;
        unsafe -selectNextTab;
        unsafe -tabbingIdentifier;
        unsafe -tabGroup;
        unsafe -isDocumentEdited;
        unsafe -close;
        unsafe -performWindowDragWithEvent;
        unsafe -invalidateCursorRectsForView;
        unsafe -setDelegate;
        unsafe -sendEvent;
        unsafe -convertPointFromScreen;
        unsafe -convertRectToScreen;

        // `addChildWindow:ordered:` is not safe, as cycles must be prevented
    }

    class NSTouch: Immutable {}

    class NSUserInterfaceCompressionOptions: Immutable {}
}
