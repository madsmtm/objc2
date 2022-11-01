//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug)]
    pub struct NSResponder;

    unsafe impl ClassType for NSResponder {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSResponder {
        #[method_id(init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(nextResponder)]
        pub unsafe fn nextResponder(&self) -> Option<Id<NSResponder, Shared>>;

        #[method(setNextResponder:)]
        pub unsafe fn setNextResponder(&self, nextResponder: Option<&NSResponder>);

        #[method(tryToPerform:with:)]
        pub unsafe fn tryToPerform_with(&self, action: Sel, object: Option<&Object>) -> bool;

        #[method(performKeyEquivalent:)]
        pub unsafe fn performKeyEquivalent(&self, event: &NSEvent) -> bool;

        #[method_id(validRequestorForSendType:returnType:)]
        pub unsafe fn validRequestorForSendType_returnType(
            &self,
            sendType: Option<&NSPasteboardType>,
            returnType: Option<&NSPasteboardType>,
        ) -> Option<Id<Object, Shared>>;

        #[method(mouseDown:)]
        pub unsafe fn mouseDown(&self, event: &NSEvent);

        #[method(rightMouseDown:)]
        pub unsafe fn rightMouseDown(&self, event: &NSEvent);

        #[method(otherMouseDown:)]
        pub unsafe fn otherMouseDown(&self, event: &NSEvent);

        #[method(mouseUp:)]
        pub unsafe fn mouseUp(&self, event: &NSEvent);

        #[method(rightMouseUp:)]
        pub unsafe fn rightMouseUp(&self, event: &NSEvent);

        #[method(otherMouseUp:)]
        pub unsafe fn otherMouseUp(&self, event: &NSEvent);

        #[method(mouseMoved:)]
        pub unsafe fn mouseMoved(&self, event: &NSEvent);

        #[method(mouseDragged:)]
        pub unsafe fn mouseDragged(&self, event: &NSEvent);

        #[method(scrollWheel:)]
        pub unsafe fn scrollWheel(&self, event: &NSEvent);

        #[method(rightMouseDragged:)]
        pub unsafe fn rightMouseDragged(&self, event: &NSEvent);

        #[method(otherMouseDragged:)]
        pub unsafe fn otherMouseDragged(&self, event: &NSEvent);

        #[method(mouseEntered:)]
        pub unsafe fn mouseEntered(&self, event: &NSEvent);

        #[method(mouseExited:)]
        pub unsafe fn mouseExited(&self, event: &NSEvent);

        #[method(keyDown:)]
        pub unsafe fn keyDown(&self, event: &NSEvent);

        #[method(keyUp:)]
        pub unsafe fn keyUp(&self, event: &NSEvent);

        #[method(flagsChanged:)]
        pub unsafe fn flagsChanged(&self, event: &NSEvent);

        #[method(tabletPoint:)]
        pub unsafe fn tabletPoint(&self, event: &NSEvent);

        #[method(tabletProximity:)]
        pub unsafe fn tabletProximity(&self, event: &NSEvent);

        #[method(cursorUpdate:)]
        pub unsafe fn cursorUpdate(&self, event: &NSEvent);

        #[method(magnifyWithEvent:)]
        pub unsafe fn magnifyWithEvent(&self, event: &NSEvent);

        #[method(rotateWithEvent:)]
        pub unsafe fn rotateWithEvent(&self, event: &NSEvent);

        #[method(swipeWithEvent:)]
        pub unsafe fn swipeWithEvent(&self, event: &NSEvent);

        #[method(beginGestureWithEvent:)]
        pub unsafe fn beginGestureWithEvent(&self, event: &NSEvent);

        #[method(endGestureWithEvent:)]
        pub unsafe fn endGestureWithEvent(&self, event: &NSEvent);

        #[method(smartMagnifyWithEvent:)]
        pub unsafe fn smartMagnifyWithEvent(&self, event: &NSEvent);

        #[method(changeModeWithEvent:)]
        pub unsafe fn changeModeWithEvent(&self, event: &NSEvent);

        #[method(touchesBeganWithEvent:)]
        pub unsafe fn touchesBeganWithEvent(&self, event: &NSEvent);

        #[method(touchesMovedWithEvent:)]
        pub unsafe fn touchesMovedWithEvent(&self, event: &NSEvent);

        #[method(touchesEndedWithEvent:)]
        pub unsafe fn touchesEndedWithEvent(&self, event: &NSEvent);

        #[method(touchesCancelledWithEvent:)]
        pub unsafe fn touchesCancelledWithEvent(&self, event: &NSEvent);

        #[method(quickLookWithEvent:)]
        pub unsafe fn quickLookWithEvent(&self, event: &NSEvent);

        #[method(pressureChangeWithEvent:)]
        pub unsafe fn pressureChangeWithEvent(&self, event: &NSEvent);

        #[method(noResponderFor:)]
        pub unsafe fn noResponderFor(&self, eventSelector: Sel);

        #[method(acceptsFirstResponder)]
        pub unsafe fn acceptsFirstResponder(&self) -> bool;

        #[method(becomeFirstResponder)]
        pub unsafe fn becomeFirstResponder(&self) -> bool;

        #[method(resignFirstResponder)]
        pub unsafe fn resignFirstResponder(&self) -> bool;

        #[method(interpretKeyEvents:)]
        pub unsafe fn interpretKeyEvents(&self, eventArray: &NSArray<NSEvent>);

        #[method(flushBufferedKeyEvents)]
        pub unsafe fn flushBufferedKeyEvents(&self);

        #[method_id(menu)]
        pub unsafe fn menu(&self) -> Option<Id<NSMenu, Shared>>;

        #[method(setMenu:)]
        pub unsafe fn setMenu(&self, menu: Option<&NSMenu>);

        #[method(showContextHelp:)]
        pub unsafe fn showContextHelp(&self, sender: Option<&Object>);

        #[method(helpRequested:)]
        pub unsafe fn helpRequested(&self, eventPtr: &NSEvent);

        #[method(shouldBeTreatedAsInkEvent:)]
        pub unsafe fn shouldBeTreatedAsInkEvent(&self, event: &NSEvent) -> bool;

        #[method(wantsScrollEventsForSwipeTrackingOnAxis:)]
        pub unsafe fn wantsScrollEventsForSwipeTrackingOnAxis(
            &self,
            axis: NSEventGestureAxis,
        ) -> bool;

        #[method(wantsForwardedScrollEventsForAxis:)]
        pub unsafe fn wantsForwardedScrollEventsForAxis(&self, axis: NSEventGestureAxis) -> bool;

        #[method_id(supplementalTargetForAction:sender:)]
        pub unsafe fn supplementalTargetForAction_sender(
            &self,
            action: Sel,
            sender: Option<&Object>,
        ) -> Option<Id<Object, Shared>>;
    }
);

pub type NSStandardKeyBindingResponding = NSObject;

extern_methods!(
    /// NSStandardKeyBindingMethods
    unsafe impl NSResponder {}
);

extern_methods!(
    /// NSUndoSupport
    unsafe impl NSResponder {
        #[method_id(undoManager)]
        pub unsafe fn undoManager(&self) -> Option<Id<NSUndoManager, Shared>>;
    }
);

extern_methods!(
    /// NSControlEditingSupport
    unsafe impl NSResponder {
        #[method(validateProposedFirstResponder:forEvent:)]
        pub unsafe fn validateProposedFirstResponder_forEvent(
            &self,
            responder: &NSResponder,
            event: Option<&NSEvent>,
        ) -> bool;
    }
);

extern_methods!(
    /// NSErrorPresentation
    unsafe impl NSResponder {
        #[method(presentError:modalForWindow:delegate:didPresentSelector:contextInfo:)]
        pub unsafe fn presentError_modalForWindow_delegate_didPresentSelector_contextInfo(
            &self,
            error: &NSError,
            window: &NSWindow,
            delegate: Option<&Object>,
            didPresentSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[method(presentError:)]
        pub unsafe fn presentError(&self, error: &NSError) -> bool;

        #[method_id(willPresentError:)]
        pub unsafe fn willPresentError(&self, error: &NSError) -> Id<NSError, Shared>;
    }
);

extern_methods!(
    /// NSTextFinderSupport
    unsafe impl NSResponder {
        #[method(performTextFinderAction:)]
        pub unsafe fn performTextFinderAction(&self, sender: Option<&Object>);
    }
);

extern_methods!(
    /// NSWindowTabbing
    unsafe impl NSResponder {
        #[method(newWindowForTab:)]
        pub unsafe fn newWindowForTab(&self, sender: Option<&Object>);
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSResponder {
        #[method(performMnemonic:)]
        pub unsafe fn performMnemonic(&self, string: &NSString) -> bool;
    }
);
